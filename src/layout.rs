use crate::Context;
use crate::globals::GImGui;

// FIXME: this is in development, not exposed/functional as a generic feature yet.
// Horizontal/Vertical enums are fixed to 0/1 so they may be used to index Vector2D
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum LayoutType
{
    Horizontal,
    Vertical
}

// Gets back to previous line and continue with horizontal layout
//      offset_from_start_x == 0 : follow right after previous item
//      offset_from_start_x != 0 : align to specified x position (relative to window/group left)
//      spacing_w < 0            : use default spacing if pos_x == 0, no spacing if pos_x != 0
//      spacing_w >= 0           : enforce spacing amount
// void SameLine(float offset_from_start_x, float spacing_w)
pub fn same_line(g: &mut Context, offset_from_start_x: f32, spacing_w: f32)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    if (window.skip_items)
        return;

    if (offset_from_start_x != 0.0)
    {
        if (spacing_w < 0.0)
            spacing_w = 0.0;
        window.dc.cursor_pos.x = window.pos.x - window.scroll.x + offset_from_start_x + spacing_w + window.dc.GroupOffset.x + window.dc.ColumnsOffset.x;
        window.dc.cursor_pos.y = window.dc.CursorPosPrevLine.y;
    }
    else
    {
        if (spacing_w < 0.0)
            spacing_w = g.style.ItemSpacing.x;
        window.dc.cursor_pos.x = window.dc.CursorPosPrevLine.x + spacing_w;
        window.dc.cursor_pos.y = window.dc.CursorPosPrevLine.y;
    }
    window.dc.CurrLineSize = window.dc.PrevLineSize;
    window.dc.CurrLineTextBaseOffset = window.dc.PrevLineTextBaseOffset;
    window.dc.IsSameLine = true;
}
