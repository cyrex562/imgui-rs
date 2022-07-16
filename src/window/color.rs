use crate::window::{Window, WindowFlags};

// static ImGuiColor GetWindowBgColorIdx(ImGuiWindow* window)
pub fn get_window_bg_color_idx(window: &mut Window)
{
    // if (window.flags & (WindowFlags::Tooltip | WindowFlags::Popup))
    if window.flags.contains(&WindowFlags::Tooltip) && window.flags.contains(&WindowFlags::Popup)
    {
        return Color::PopupBg;
    }
    // if ((window.flags & WindowFlags::ChildWindow) && !window.dock_is_active)
   if window.flags.contains(WindowFlags::ChildWindow) && window.dock_is_active == false
    {
        return Color::ChildBg;
    }
    return Color::WindowBg;
}
