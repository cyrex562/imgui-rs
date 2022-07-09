use crate::context::DimgContext;
use crate::window::DimgHoveredFlags;
use crate::types::DimgId;
use crate::window::DimgWindow;

// void ImGui::SetActiveID(ImGuiID id, ImGuiWindow* window)
pub fn SetActiveID(g: &mut DimgContext, id: DimgId, window: &mut DimgWindow)
{
    // ImGuiContext& g = *GImGui;

    // While most behaved code would make an effort to not steal active id during window move/drag operations,
    // we at least need to be resilient to it. Cancelling the move is rather aggressive and users of 'master' branch
    // may prefer the weird ill-defined half working situation ('docking' did assert), so may need to rework that.
    if g.moving_window != NULL && g.active_id == g.moving_window.MoveId
    {
        IMGUI_DEBUG_LOG_ACTIVEID("SetActiveID() cancel moving_window\n");
        g.moving_window = NULL;
    }

    // Set active id
    g.active_id_is_just_activated = (g.active_id != id);
    if g.active_id_is_just_activated
    {
        // IMGUI_DEBUG_LOG_ACTIVEID("SetActiveID() old:0x%08X (window \"%s\") -> new:0x%08X (window \"%s\")\n", g.active_id, g.active_id_window ? g.active_id_window->name : "", id, window ? window.name : "");
        g.active_id_timer = 0.0;
        g.active_id_has_been_pressed_before = false;
        g.ActiveIdHasBeenEditedBefore = false;
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
    g.active_id_window = window;
    g.active_id_has_been_edited_this_frame = false;
    if id
    {
        g.active_id_is_alive = id;
        g.active_id_source = (g.nav_activate_id == id || g.nav_activate_input_id == id || g.nav_just_moved_to_id == id) ? (ImGuiInputSource)ImGuiInputSource_Nav : ImGuiInputSource_Mouse;
    }

    // clear declaration of inputs claimed by the widget
    // (Please note that this is WIP and not all keys/inputs are thoroughly declared by all widgets yet)
    g.active_id_using_mouse_wheel = false;
    g.active_id_using_nav_dir_mask = 0x00;
    g.active_id_using_nav_input_mask = 0x00;
    g.active_id_using_key_input_mask.ClearAllBits();
}


// void ImGui::MarkItemEdited(ImGuiID id)
pub fn MarkItemEdited(g: &mut DimgContext, id: DimgId)
{
    // This marking is solely to be able to provide info for IsItemDeactivatedAfterEdit().
    // active_id might have been released by the time we call this (as in the typical press/release button behavior) but still need need to fill the data.
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.active_id == id || g.active_id == 0 || g.drag_drop_active);
    // IM_UNUSED(id); // Avoid unused variable warnings when asserts are compiled out.
    //IM_ASSERT(g.current_window->dc.LastItemId == id);
    g.active_id_has_been_edited_this_frame = true;
    g.ActiveIdHasBeenEditedBefore = true;
    g.last_item_data.StatusFlags |= ImGuiItemStatusFlags_Edited;
}





void ImGui::ClearActiveID()
{
    SetActiveID(0, NULL); // g.active_id = 0;
}

void ImGui::SetHoveredID(ImGuiID id)
{
    ImGuiContext& g = *GImGui;
    g.HoveredId = id;
    g.HoveredIdAllowOverlap = false;
    g.HoveredIdUsingMouseWheel = false;
    if (id != 0 && g.HoveredIdPreviousFrame != id)
        g.HoveredIdTimer = g.HoveredIdNotActiveTimer = 0.0;
}

ImGuiID ImGui::GetHoveredID()
{
    ImGuiContext& g = *GImGui;
    return g.HoveredId ? g.HoveredId : g.HoveredIdPreviousFrame;
}

// This is called by ItemAdd().
// Code not using ItemAdd() may need to call this manually otherwise active_id will be cleared. In IMGUI_VERSION_NUM < 18717 this was called by GetID().
void ImGui::KeepAliveID(ImGuiID id)
{
    ImGuiContext& g = *GImGui;
    if (g.ActiveId == id)
        g.ActiveIdIsAlive = id;
    if (g.ActiveIdPreviousFrame == id)
        g.ActiveIdPreviousFrameIsAlive = true;
}
