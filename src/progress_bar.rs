use libc::{c_char, c_float};
use std::ptr::null;
use crate::color::{ImGuiCol_FrameBg, ImGuiCol_PlotHistogram};
use crate::GImGui;
use crate::item_ops::{CalcItemSize, CalcItemWidth, ItemAdd, ItemSize};
use crate::math_ops::{ImClamp, ImLerp, ImSaturateFloat};
use crate::rect::ImRect;
use crate::render_ops::{RenderFrame, RenderRectFilledRangeH, RenderTextClipped};
use crate::style_ops::GetColorU32;
use crate::text_ops::CalcTextSize;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;
use crate::window::ops::GetCurrentWindow;

// size_arg (for each axis) < 0.0: align to end, 0.0: auto, > 0.0: specified size
pub unsafe fn ProgressBar(mut fraction: c_float, size_arg: &mut ImVec2, overlay: &mut str)
{
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    if window.SkipItems { return ; }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let setyle = &mut g.Style;

    let pos: ImVec2 = window.DC.CursorPos;
    let size: ImVec2 = CalcItemSize(size_arg, CalcItemWidth(), g.FontSize + style.FramePadding.y * 2.0);
    let mut bb: ImRect = ImRect::new(pos, pos + size);
    ItemSize(&size, style.FramePadding.y);
    if !ItemAdd(&mut bb, 0, null(), 0) { return ; }

    // Render
    fraction = ImSaturateFloat(fraction);
    RenderFrame(bb.Min, bb.Max, GetColorU32(ImGuiCol_FrameBg, 0.0), true, style.FrameRounding);
    bb.expand_from_vec(&ImVec2::from_floats(-style.FrameBorderSize, -style.FrameBorderSize));
    let fill_br: ImVec2 = ImVec2::from_floats(ImLerp(bb.Min.x, bb.Max.x, fraction), bb.Max.y);
    RenderRectFilledRangeH(window.DrawList, &bb, GetColorU32(ImGuiCol_PlotHistogram, 0.0), 0.0, fraction, style.FrameRounding);

    // Default displaying the fraction as percentage string, but user can override it
    overlay_buf: [c_char;32];
    if !overlay
    {
        // ImFormatString(overlay_buf, overlay_buf.len(), "%.0f%%", fraction * 100 + 0.010f32);
        // overlay = overlay_buf;
    }

    let overlay_size: ImVec2 = CalcTextSize(overlay, false, 0.0);
    if overlay_size.x > 0.0 {
        RenderTextClipped(&ImVec2::from_floats(ImClamp(fill_br.x + style.ItemSpacing.x, bb.Min.x, bb.Max.x - overlay_size.x - style.ItemInnerSpacing.x), bb.Min.y), &bb.Max, overlay,  &overlay_size, Some(&ImVec2::from_floats(0.0, 0.5)), &bb);
    }
}
