use crate::context::Context;
use crate::globals::GImGui;
use crate::orig_imgui_single_file::ImGuiID;
use crate::window::HoveredFlags;
use crate::types::Id32;
use crate::window::Window;

// void ImGui::set_active_id(ImGuiID id, ImGuiWindow* window)
pub fn set_active_id(g: &mut Context, id: Id32, window: &mut Window)
{
    // ImGuiContext& g = *GImGui;

    // While most behaved code would make an effort to not steal active id during window move/drag operations,
    // we at least need to be resilient to it. Cancelling the move is rather aggressive and users of 'master' branch
    // may prefer the weird ill-defined half working situation ('docking' did assert), so may need to rework that.
    if g.moving_window_id != None && g.active_id == g.moving_window_id.move_id
    {
        debug!("set_active_id() cancel moving_window\n");
        g.moving_window_id = None;
    }

    // Set active id
    g.active_id_is_just_activated = (g.active_id != id);
    if g.active_id_is_just_activated
    {
        // IMGUI_DEBUG_LOG_ACTIVEID("set_active_id() old:0x%08X (window \"%s\") -> new:0x%08X (window \"%s\")\n", g.active_id, g.active_id_window ? g.active_id_window->name : "", id, window ? window.name : "");
        g.active_id_timer = 0.0;
        g.active_id_has_been_pressed_before = false;
        g.active_id_has_been_edited_before = false;
        g.active_id_mouse_button = -1;
        if id != 0
        {
            g.last_active_id = id;
            g.last_active_id_timer = 0.0;
        }
    }
    g.active_id = id;
    g.active_id_allow_overlap = false;
    g.active_id_no_clear_on_focus_loss = false;
    g.active_id_window_id = window;
    g.active_id_has_been_edited_this_frame = false;
    if id
    {
        g.active_id_is_alive = id;
        g.active_id_source = if(g.nav_activate_id == id || g.nav_activate_input_id == id || g.nav_just_moved_to_id == id) { (ImGuiInputSource)InputSource::Nav : InputSource:}else{Mouse};
    }

    // clear declaration of inputs claimed by the widget
    // (Please note that this is WIP and not all keys/inputs are thoroughly declared by all widgets yet)
    g.active_id_using_mouse_wheel = false;
    g.active_id_using_nav_dir_mask = 0x00;
    g.active_id_using_nav_input_mask = 0x00;
    g.active_id_using_key_input_mask.ClearAllBits();
}


// void ImGui::MarkItemEdited(ImGuiID id)
pub fn MarkItemEdited(g: &mut Context, id: Id32)
{
    // This marking is solely to be able to provide info for IsItemDeactivatedAfterEdit().
    // active_id might have been released by the time we call this (as in the typical press/release button behavior) but still need need to fill the data.
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.active_id == id || g.active_id == 0 || g.drag_drop_active);
    // IM_UNUSED(id); // Avoid unused variable warnings when asserts are compiled out.
    //IM_ASSERT(g.current_window->dc.LastItemId == id);
    g.active_id_has_been_edited_this_frame = true;
    g.active_id_has_been_edited_before = true;
    g.last_item_data.status_flags |= ImGuiItemStatusFlags_Edited;
}





void ImGui::clear_active_id()
{
    set_active_id(0, None); // g.active_id = 0;
}

void ImGui::SetHoveredID(ImGuiID id)
{
    // ImGuiContext& g = *GImGui;
    g.hovered_id = id;
    g.hovered_id_allow_overlap = false;
    g.hovered_id_using_mouse_wheel = false;
    if (id != 0 && g.hovered_id_previous_frame != id)
        g.hovered_id_timer = g.hovered_id_not_active_timer = 0.0;
}

ImGuiID ImGui::GetHoveredID()
{
    // ImGuiContext& g = *GImGui;
    return g.hovered_id ? g.hovered_id : g.hovered_id_previous_frame;
}

// This is called by ItemAdd().
// Code not using ItemAdd() may need to call this manually otherwise active_id will be cleared. In IMGUI_VERSION_NUM < 18717 this was called by GetID().
void ImGui::keep_alive_id(ImGuiID id)
{
    // ImGuiContext& g = *GImGui;
    if (g.active_id == id)
        g.active_id_is_alive = id;
    if (g.active_id_previous_frame == id)
        g.active_id_previous_frame_is_alive = true;
}

// Helper to avoid a common series of PushOverrideID -> GetID() -> PopID() call
// (note that when using this pattern, test_engine's "Stack Tool" will tend to not display the intermediate stack level.
//  for that to work we would need to do PushOverrideID() -> ItemAdd() -> PopID() which would alter widget code a little more)
// ImGuiID GetIDWithSeed(const char* str, const char* str_end, ImGuiID seed)
pub fn get_id_with_seed(g: &mut Context, in_str: &str, seed: Id32) -> Id32
{
    ImGuiID id = ifImHashStr(str, str_end { (str_end - str) }else{ 0, seed)};
    // ImGuiContext& g = *GImGui;
    if (g.debug_hook_id_info == id)
        debug_hook_id_info(id, DataType::String, str, str_end);
    return id;
}
