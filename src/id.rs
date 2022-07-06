use crate::context::ImGuiContext;
use crate::defines::{ImGuiHoveredFlags, ImGuiID};
use crate::window::ImGuiWindow;

// void ImGui::SetActiveID(ImGuiID id, ImGuiWindow* window)
pub fn SetActiveID(g: &mut ImGuiContext, id: ImGuiID, window: &mut ImGuiWindow)
{
    // ImGuiContext& g = *GImGui;

    // While most behaved code would make an effort to not steal active id during window move/drag operations,
    // we at least need to be resilient to it. Cancelling the move is rather aggressive and users of 'master' branch
    // may prefer the weird ill-defined half working situation ('docking' did assert), so may need to rework that.
    if g.MovingWindow != NULL && g.ActiveId == g.MovingWindow.MoveId
    {
        IMGUI_DEBUG_LOG_ACTIVEID("SetActiveID() cancel MovingWindow\n");
        g.MovingWindow = NULL;
    }

    // Set active id
    g.ActiveIdIsJustActivated = (g.ActiveId != id);
    if g.ActiveIdIsJustActivated
    {
        // IMGUI_DEBUG_LOG_ACTIVEID("SetActiveID() old:0x%08X (window \"%s\") -> new:0x%08X (window \"%s\")\n", g.ActiveId, g.ActiveIdWindow ? g.ActiveIdWindow->Name : "", id, window ? window.Name : "");
        g.ActiveIdTimer = 0.0;
        g.ActiveIdHasBeenPressedBefore = false;
        g.ActiveIdHasBeenEditedBefore = false;
        g.ActiveIdMouseButton = -1;
        if id != 0
        {
            g.LastActiveId = id;
            g.LastActiveIdTimer = 0.0;
        }
    }
    g.ActiveId = id;
    g.ActiveIdAllowOverlap = false;
    g.ActiveIdNoClearOnFocusLoss = false;
    g.ActiveIdWindow = window;
    g.ActiveIdHasBeenEditedThisFrame = false;
    if id
    {
        g.ActiveIdIsAlive = id;
        g.ActiveIdSource = (g.NavActivateId == id || g.NavActivateInputId == id || g.NavJustMovedToId == id) ? (ImGuiInputSource)ImGuiInputSource_Nav : ImGuiInputSource_Mouse;
    }

    // Clear declaration of inputs claimed by the widget
    // (Please note that this is WIP and not all keys/inputs are thoroughly declared by all widgets yet)
    g.ActiveIdUsingMouseWheel = false;
    g.ActiveIdUsingNavDirMask = 0x00;
    g.ActiveIdUsingNavInputMask = 0x00;
    g.ActiveIdUsingKeyInputMask.ClearAllBits();
}


// void ImGui::MarkItemEdited(ImGuiID id)
pub fn MarkItemEdited(g: &mut ImGuiContext, id: ImGuiID)
{
    // This marking is solely to be able to provide info for IsItemDeactivatedAfterEdit().
    // ActiveId might have been released by the time we call this (as in the typical press/release button behavior) but still need need to fill the data.
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.ActiveId == id || g.ActiveId == 0 || g.DragDropActive);
    // IM_UNUSED(id); // Avoid unused variable warnings when asserts are compiled out.
    //IM_ASSERT(g.CurrentWindow->DC.LastItemId == id);
    g.ActiveIdHasBeenEditedThisFrame = true;
    g.ActiveIdHasBeenEditedBefore = true;
    g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_Edited;
}





void ImGui::ClearActiveID()
{
    SetActiveID(0, NULL); // g.ActiveId = 0;
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
// Code not using ItemAdd() may need to call this manually otherwise ActiveId will be cleared. In IMGUI_VERSION_NUM < 18717 this was called by GetID().
void ImGui::KeepAliveID(ImGuiID id)
{
    ImGuiContext& g = *GImGui;
    if (g.ActiveId == id)
        g.ActiveIdIsAlive = id;
    if (g.ActiveIdPreviousFrame == id)
        g.ActiveIdPreviousFrameIsAlive = true;
}
