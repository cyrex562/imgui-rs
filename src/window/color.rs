use crate::window::{Window, WindowFlags};

// static ImGuiColor get_window_bg_color_idx(ImGuiWindow* window)
pub fn get_window_bg_color_idx(window: &mut Window)
{
    // if (window.flags & (WindowFlags::Tooltip | WindowFlags::Popup))
    if window.flags.contains(&WindowFlags::Tooltip) && window.flags.contains(&WindowFlags::Popup)
    {
        return StyleColor::PopupBg;
    }
    // if ((window.flags & WindowFlags::ChildWindow) && !window.dock_is_active)
   if window.flags.contains(WindowFlags::ChildWindow) && window.dock_is_active == false
    {
        return StyleColor::ChildBg;
    }
    return StyleColor::WindowBg;
}
