use std::collections::HashSet;
use crate::color::StyleColor;
use crate::Context;
use crate::vectors::vector_2d::Vector2D;
use crate::window::WindowFlags;

pub enum TooltipFlags
{
    None = 0,
    OverridePreviousTooltip = 1 << 0      // Override will clear/ignore previously submitted tooltip (defaults to append)
}

// void BeginTooltip()
pub fn begin_tolltip(g: &mut Context)
{
    BeginTooltipEx(TooltipFlags::None, WindowFlags::None);
}

// void BeginTooltipEx(ImGuiTooltipFlags tooltip_flags, ImGuiWindowFlags extra_window_flags)
pub fn begin_tooltip_ex(g: &mut Context, tooltip_flags: &HashSet<TooltipFlags>, extra_window_flags: &HashSet<WindowFlags>)
{
    // ImGuiContext& g = *GImGui;

    if (g.drag_drop_within_source || g.drag_drop_within_target)
    {
        // The default tooltip position is a little offset to give space to see the context menu (it's also clamped within the current viewport/monitor)
        // In the context of a dragging tooltip we try to reduce that offset and we enforce following the cursor.
        // Whatever we do we want to call set_next_window_pos() to enforce a tooltip position and disable clipping the tooltip without our display area, like regular tooltip do.
        //Vector2D tooltip_pos = g.io.mouse_pos - g.active_id_click_offset - g.style.window_padding;
        Vector2D tooltip_pos = g.io.mouse_pos + Vector2D::new(16 * g.style.MouseCursorScale, 8 * g.style.MouseCursorScale);
        set_next_window_pos(tooltip_pos);
        set_netxt_window_bg_alpha(g.style.colors[StyleColor::PopupBg].w * 0.60);
        //PushStyleVar(ImGuiStyleVar_Alpha, g.style.Alpha * 0.60); // This would be nice but e.g ColorButton with checkboard has issue with transparent colors :(
        tooltip_flags |= TooltipFlags::OverridePreviousTooltip;
    }

    char window_name[16];
    ImFormatString(window_name, IM_ARRAYSIZE(window_name), "##Tooltip_%02d", g.tool_tip_override_count);
    if (tooltip_flags & TooltipFlags::OverridePreviousTooltip)
        if (ImGuiWindow* window = find_window_by_name(window_name))
            if (window.active)
            {
                // Hide previous tooltip from being displayed. We can't easily "reset" the content of a window so we create a new one.
                window.hidden = true;
                window..hidden_frames_can_skip_items = 1; // FIXME: This may not be necessary?
                ImFormatString(window_name, IM_ARRAYSIZE(window_name), "##Tooltip_%02d", g.tool_tip_override_count += 1);
            }
    ImGuiWindowFlags flags = WindowFlags::Tooltip | WindowFlags::NoInputs | WindowFlags::NoTitleBar | WindowFlags::NoMove | WindowFlags::NoResize | WindowFlags::NoSavedSettings | WindowFlags::AlwaysAutoResize | WindowFlags::NoDocking;
    begin(window_name, None, flags | extra_window_flags);
}

// void EndTooltip()
pub fn end_tooltip(g: &mut Context)
{
    // IM_ASSERT(GetCurrentWindowRead().flags & WindowFlags::Tooltip);   // Mismatched BeginTooltip()/EndTooltip() calls
    end();
}

// void SetTooltipV(const char* fmt, va_list args)
pub fn set_tooltip_v(g: &mut Context, fmt: &str)
{
    BeginTooltipEx(TooltipFlags::OverridePreviousTooltip, WindowFlags::None);
    TextV(fmt, args);
    EndTooltip();
}

// void SetTooltip(const char* fmt, ...)
pub fn set_tooltip(g: &mut Context, in_str: &str)
{
    va_list args;
    va_start(args, fmt);
    SetTooltipV(fmt, args);
    va_end(args);
}
