use libc::c_float;
use std::ptr::null;
use crate::color::ImGuiCol_Text;
use crate::GImGui;
use crate::item_ops::{ItemAdd, ItemSize};
use crate::layout_ops::SameLine;
use crate::rect::ImRect;
use crate::render_ops::RenderBullet;
use crate::style_ops::GetColorU32;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;
use crate::window::ops::GetCurrentWindow;

pub unsafe fn Bullet()
{
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    if window.SkipItems { return ; }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let setyle = &mut g.Style;
    let line_height: c_float =  window.DC.CurrLineSize.y.min( g.FontSize + style.FramePadding.y * 2).max( g.FontSize);
    let mut bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + ImVec2::from_floats(g.FontSize, line_height));
    ItemSize(&bb.GetSize(), 0.0);
    if !ItemAdd(&mut bb, 0, null(), 0)
    {
        SameLine(0, style.FramePadding.x * 2);
        return;
    }

    // Render and stay on same line
    text_col: u32 = GetColorU32(ImGuiCol_Text, 0.0);
    RenderBullet(window.DrawList, bb.Min + ImVec2::from_floats(style.FramePadding.x + g.FontSize * 0.5, line_height * 0.5), text_col);
    SameLine(0, style.FramePadding.x * 2.0);
}
