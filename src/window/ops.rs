use std::collections::HashSet;
use crate::{Context, Viewport, window};
use crate::condition::Condition;
use crate::vectors::two_d::Vector2D;
use crate::window::{settings, Window, WindowFlags};
use crate::window::settings::WindowSettings;

// static ImGuiWindow* CreateNewWindow(const char* name, ImGuiWindowFlags flags)
pub fn create_new_window(g: &mut Context, name: &str, flags: &mut HashSet<WindowFlags>) -> &mut Window
{
    // ImGuiContext& g = *GImGui;
    //IMGUI_DEBUG_LOG("CreateNewWindow '%s', flags = 0x%08X\n", name, flags);

    // Create window the first time
    // ImGuiWindow* window = IM_NEW(ImGuiWindow)(&g, name);
    let mut window = Window::new(g, name);
    window.flags = flags.clone();
    // TODO: add window to context?
    // g.windows_by_id.SetVoidPtr(window.id, window);


    // Default/arbitrary window position. Use SetNextWindowPos() with the appropriate condition flag to change the initial position of a window.
    // const ImGuiViewport* main_viewport = ImGui::GetMainViewport();
   let main_viewport: &mut Viewport = get_main_viewport(g).unwrap();
    window.pos = &main_viewport.pos + Vector2D::new(60.0, 60.0);
    window.viewport_pos = main_viewport.pos.clone();

    // User can disable loading and saving of settings. Tooltip and child windows also don't store settings.
    if !(flags.contains(&WindowFlags::NoSavedSettings)) {
        let settings: Option<&mut WindowSettings> = find_window_settings(g, window.id);
        if settings.is_some(){
            // Retrieve settings from .ini file
            window.settings_offset = g.settings_windows.offset_from_ptr(settings);
            window::set_window_condition_allow_flags(&mut window, &mut HashSet::from([Condition::FirstUseEver]), false);
            settings::apply_window_settings(g, &mut window, &mut(settings.some()));
        }
    }
    window.dc.ideal_max_pos = window.pos.clone();
    window.dc.cursor_max_pos = window.pos.clone();
    window.dc.cursor_start_pos = window.pos.clone(); // So first call to CalcWindowContentSizes() doesn't return crazy values

    // if ((flags & WindowFlags::AlwaysAutoResize) != 0)
    if flags.contains(&WindowFlags::AlwaysAutoResize)
    {
        window.auto_fit_frames_y = 2;
        window.auto_fit_frames_x = 2;
        window.auto_fit_only_grows = false;
    }
    else
    {
        if window.size.x <= 0.0 {
            window.auto_fit_frames_x = 2;
        }
        if window.size.y <= 0.0 {
            window.auto_fit_frames_y = 2;
        }
        window.auto_fit_only_grows = (window.auto_fit_frames_x > 0) || (window.auto_fit_frames_y > 0);
    }

    // if (flags & WindowFlags::NoBringToFrontOnFocus) {
    if flags.contains(&WindowFlags::NoBringToFrontOnFocus) {
        g.windows.push_front(window);
    }// Quite slow but rare and only once
    else {
        g.windows.push_back(window);
    }
    // UpdateWindowInFocusOrderList(window, true, window.flags);
    window::update_window_focus_order_list(g, &mut window, true, &mut window.flags);

    return &mut window;
}

/// static void ScaleWindow(ImGuiWindow* window, float scale)
pub fn scale_window(window: &mut Window, scale: f32)
{
    // Vector2D origin = window.viewport.pos;
    let origin = window.viewport_id.pos;
    window.pos = Vector2D::floor((&window.pos - origin) * scale + origin);
    window.size = Vector2D::floor(&window.size * scale);
    window.size_full = Vector2D::floor(&window.size_full * scale);
    window.content_size = Vector2D::floor(window.ContentSize * scale);
}

/// This is called during NewFrame()->UpdateViewportsNewFrame() only.
/// Need to keep in sync with set_window_pos()
/// static void TranslateWindow(ImGuiWindow* window, const Vector2D& delta)
pub fn translate_window(window: &mut Window, delta: &Vector2D)
{
    window.pos += delta;
    window.clip_rect.Translate(delta);
    window.OuterRectClipped.Translate(delta);
    window.inner_rect.Translate(delta);
    window.dc.cursor_pos += delta;
    window.dc.cursor_start_pos += delta;
    window.dc.cursor_max_pos += delta;
    window.dc.ideal_max_pos += delta;
}
