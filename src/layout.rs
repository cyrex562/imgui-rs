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
// void same_line(float offset_from_start_x, float spacing_w)
pub fn same_line(g: &mut Context, offset_from_start_x: f32, mut spacing_w: f32)
{
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();
    if window.skip_items {
        return;
    }

    if offset_from_start_x != 0.0
    {
        if (spacing_w < 0.0) {
            spacing_w = 0.0;
        }
        window.dc.cursor_pos.x = window.pos.x - window.scroll.x + offset_from_start_x + spacing_w + window.dc.GroupOffset.x + window.dc.columns_offset.x;
        window.dc.cursor_pos.y = window.dc.cursor_pos_prev_line.y;
    }
    else
    {
        if spacing_w < 0.0 {
            spacing_w = g.style.item_spacing.x;
        }
        window.dc.cursor_pos.x = window.dc.cursor_pos_prev_line.x + spacing_w;
        window.dc.cursor_pos.y = window.dc.cursor_pos_prev_line.y;
    }
    window.dc.curr_line_size = window.dc.PrevLineSize;
    window.dc.curr_line_text_base_offset = window.dc.PrevLineTextBaseOffset;
    window.dc.Issame_line = true;
}
