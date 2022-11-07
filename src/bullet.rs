use crate::color::ImGuiCol_Text;
use crate::item_ops::{ItemAdd, ItemSize};
use crate::layout_ops::same_line;
use crate::rect::ImRect;
use crate::render_ops::RenderBullet;
use crate::style_ops::GetColorU32;
use crate::vec2::ImVec2;
use crate::window::ops::GetCurrentWindow;
use crate::window::ImguiWindow;
use crate::GImGui;
use libc::c_float;
use std::ptr::null;

pub unsafe fn Bullet() {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let setyle = &mut g.style;
    let line_height: c_float = window
        .DC
        .CurrLineSize
        .y
        .min(g.FontSize + style.FramePadding.y * 2)
        .max(g.FontSize);
    let mut bb: ImRect = ImRect::new(
        window.dc.cursor_pos,
        window.dc.cursor_pos + ImVec2::from_floats(g.FontSize, line_height),
    );
    ItemSize(g, &bb.GetSize(), 0.0);
    if !ItemAdd(g, &mut bb, 0, None, 0) {
        same_line(g, 0.0, style.FramePadding.x * 2);
        return;
    }

    // Render and stay on same line
    text_col: u32 = GetColorU32(ImGuiCol_Text, 0.0);
    RenderBullet(
        &window.DrawList,
        bb.min + ImVec2::from_floats(style.FramePadding.x + g.FontSize * 0.5, line_height * 0.5),
        text_col,
    );
    same_line(g, 0.0, style.FramePadding.x * 2.0);
}
