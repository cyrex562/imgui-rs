use crate::{Context, Viewport};
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;
use crate::window::Window;

/// windows data saved in imgui.ini file
/// Because we never destroy or rename ImGuiWindowSettings, we can store the names in a separate buffer easily.
/// (this is designed to be stored in a ImChunkStream buffer, with the variable-length name following our structure)
#[derive(Default,Debug,Clone)]
pub struct WindowSettings
{
    //ImGuiID     id;
    pub id: Id32,
    // Vector2Dih    pos;            // NB: Settings position are stored RELATIVE to the viewport! Whereas runtime ones are absolute positions.
    pub pos: Vector2D,
    // Vector2Dih    size;
    pub size: Vector2D,
    // Vector2Dih    ViewportPos;
    pub viewport_pos: Vector2D,
    // ImGuiID     ViewportId;
    pub viewport_id: Id32,
    // ImGuiID     DockId;         // id of last known dock_node (even if the dock_node is invisible because it has only 1 active window), or 0 if none.
    pub dock_id: Id32,
    // ImGuiID     ClassId;        // id of window class if specified
    pub class_id: Id32,
    // short       DockOrder;      // Order of the last time the window was visible within its dock_node. This is used to reorder windows that are reappearing on the same frame. Same value between windows that were active and windows that were none are possible.
    pub dock_order: i16,
    // bool        Collapsed;
    pub collapsed: bool,
    // bool        WantApply;      // Set when loaded from .ini data (to enable merging/loading .ini data into an already running context)
    pub want_apply: bool,
    // ImGuiWindowSettings()       { memset(this, 0, sizeof(*this)); DockOrder = -1; }
    // char* GetName()             { return (char*)(this + 1); }
}

// static void ApplyWindowSettings(ImGuiWindow* window, ImGuiWindowSettings* settings)
pub fn apply_window_settings(g: &mut Context, window: &mut Window, settings: &mut WindowSettings)
{
    // const ImGuiViewport* main_viewport = ImGui::GetMainViewport();
    let main_viewport: &mut Viewport = get_main_viewport(g).unwrap();
    window.viewport_pos = main_viewport.pos.clone();
    if settings.viewport_id
    {
        window.viewport_id = settings.viewport_id;
        window.viewport_pos = Vector2D::new(settings.viewport_pos.x, settings.viewport_pos.y);
    }
    window.pos = Vector2D::floor(Vector2D::new(settings.pos.x + window.viewport_pos.x, settings.pos.y + window.viewport_pos.y));
    if settings.size.x > 0.0 && settings.size.y > 0.0 {
        window.size_full = Vector2D::floor(Vector2D::new(settings.size.x, settings.size.y));
        window.size = window.size_full.clone();
    }
    window.collapsed = settings.collapsed;
    window.dock_id = settings.dock_id;
    window.dock_order = settings.dock_order;
}
