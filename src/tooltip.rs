use std::collections::HashSet;
use crate::color::StyleColor;
use crate::condition::Condition;
use crate::Context;
use crate::vectors::vector_2d::Vector2D;
use crate::window::get::find_window_by_name;
use crate::window::lifecycle::{begin, end};
use crate::window::next_window::set_next_window_pos;
use crate::window::WindowFlags;

pub enum TooltipFlags
{
    None = 0,
    OverridePreviousTooltip = 1 << 0      // Override will clear/ignore previously submitted tooltip (defaults to append)
}

// void BeginTooltip()
pub fn begin_tooltip(g: &mut Context)
{

    let mut tooltip_flags: HashSet<TooltipFlags> = HashSet::new();
    let extra_window_flags: HashSet<WindowFlags> = HashSet::new();
    begin_tooltip_ex(g, &mut tooltip_flags, &extra_window_flags);
}

// void BeginTooltipEx(ImGuiTooltipFlags tooltip_flags, WindowFlags extra_window_flags)
pub fn begin_tooltip_ex(g: &mut Context, tooltip_flags: &mut HashSet<TooltipFlags>, extra_window_flags: &HashSet<WindowFlags>)
{
    // ImGuiContext& g = *GImGui;

    if g.drag_drop_within_source || g.drag_drop_within_target
    {
        // The default tooltip position is a little offset to give space to see the context menu (it's also clamped within the current viewport/monitor)
        // In the context of a dragging tooltip we try to reduce that offset and we enforce following the cursor.
        // Whatever we do we want to call set_next_window_pos() to enforce a tooltip position and disable clipping the tooltip without our display area, like regular tooltip do.
        //Vector2D tooltip_pos = g.io.mouse_pos - g.active_id_click_offset - g.style.window_padding;
        let tooltip_pos = g.io.mouse_pos + Vector2D::new(16 * g.style.mouse_cursor_scale, 8 * g.style.mouse_cursor_scale);
        set_next_window_pos(g, &tooltip_pos, Condition::None, None);
        set_netxt_window_bg_alpha(g.style.colors[StyleColor::PopupBg].w * 0.60);
        //PushStyleVar(ImGuiStyleVar_Alpha, g.style.alpha * 0.60); // This would be nice but e.g ColorButton with checkboard has issue with transparent colors :(
        // tooltip_flags |= TooltipFlags::OverridePreviousTooltip;
        tooltip_flags.insert(TooltipFlags::OverridePreviousTooltip);
    }

    // char window_name[16];
    let mut window_name = String::new();
    // ImFormatString(window_name, IM_ARRAYSIZE(window_name), "##Tooltip_%02d", g.tool_tip_override_count);
    window_name = format!("##Tooltip_{:02}", g.tooltip_override_count);

    if tooltip_flags.contains(&TooltipFlags::OverridePreviousTooltip) {
        // if (Window * window = find_window_by_name(window_name))
        let window = find_window_by_name(g, &window_name);
        if window.is_some()
        {
            if window.unwrap().active {
                // Hide previous tooltip from being displayed. We can't easily "reset" the content of a window so we create a new one.
                window.unwrap().hidden = true;
                window.unwrap().hidden_frames_can_skip_items = 1; // FIXME: This may not be necessary?
                // ImFormatString(window_name, IM_ARRAYSIZE(window_name), "##Tooltip_%02d", g.tool_tip_override_count += 1);
                window_name = format!("##Tooltip_{:02}", g.tooltip_override_count);
                g.tooltip_override_count += 1;
            }
        }
    }
    // WindowFlags flags = WindowFlags::Tooltip | WindowFlags::NoInputs | WindowFlags::NoTitleBar | WindowFlags::NoMove | WindowFlags::NoResize | WindowFlags::NoSavedSettings | WindowFlags::AlwaysAutoResize | WindowFlags::NoDocking;
    let flags: HashSet<WindowFlags> = HashSet::from([
        WindowFlags::Tooltip, WindowFlags::NoInputs, WindowFlags::NoTitleBar, WindowFlags::NoMove, WindowFlags::NoResize, WindowFlags::NoSavedSettings, WindowFlags::AlwaysAutoResize, WindowFlags::NoDocking
    ]);
    let begin_flags = flags | extra_window_flags;
    begin(g, window_name.as_str(), None, begin_flags);
}

// void EndTooltip()
pub fn end_tooltip(g: &mut Context)
{
    // IM_ASSERT(GetCurrentWindowRead().flags & WindowFlags::Tooltip);   // Mismatched BeginTooltip()/EndTooltip() calls
    end(g);
}

// void SetTooltipV(const char* fmt, va_list args)
pub fn set_tooltip_v(g: &mut Context, fmt: &str)
{
//     BeginTooltipEx(TooltipFlags::OverridePreviousTooltip, WindowFlags::None);
//     TextV(fmt, args);
//     EndTooltip();
//
    todo!()
}

// void SetTooltip(const char* fmt, ...)
pub fn set_tooltip(g: &mut Context, in_str: &str)
{
    // va_list args;
    // va_start(args, fmt);
    // SetTooltipV(fmt, args);
    // va_end(args);
    todo!()
}
