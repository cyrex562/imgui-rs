use std::ptr::null_mut;
use libc::{c_char, c_float};
use crate::font::ImFont;
use crate::GImGui;
use crate::render_ops::FindRenderedTextEnd;
use crate::vec2::ImVec2;

// Calculate text size. Text can be multi-line. Optionally ignore text after a ## marker.
// CalcTextSize("") should return ImVec2::new(0.0, g.FontSize)
// CalcTextSize: ImVec2(text: *const c_char, text_end: *const c_char, hide_text_after_double_hash: bool, c_float wrap_width)
pub unsafe fn CalcTextSize(text: *const c_char, text_end: *const c_char, hid_text_after_double_hash: bool, wrap_width: c_float) -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let text_display_end: *const c_char;
    if hide_text_after_double_hash {
        text_display_end = FindRenderedTextEnd(text, text_end);
    }     // Hide anything after a '##' string
    else {
        text_display_end = text_end;
    }

    font: *mut ImFont = g.Font;
    let font_size: c_float = g.FontSize;
    if text == text_display_end {
        return ImVec2::new2(0.0, font_size);
    }
    let mut text_size: ImVec2 = font.CalcTextSizeA(font_size, f32::MAX, wrap_width, text, text_display_end, null_mut());

    // Round
    // FIXME: This has been here since Dec 2015 (7b0bf230) but down the line we want this out.
    // FIXME: Investigate using ceilf or e.g.
    // - https://git.musl-libc.org/cgit/musl/tree/src/math/ceilf.c
    // - https://embarkstudios.github.io/rust-gpu/api/src/libm/math/ceilf.rs.html
    text_size.x = IM_FLOOR(text_size.x + 0.999990f32);

    return text_size;
}

// GetTextLineHeight: c_float()
pub unsafe fn GetTextLineHeight() -> c_float
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize;
}


pub unsafe fn GetTextLineHeightWithSpacing() -> c_float
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize + g.Style.ItemSpacing.y;
}

