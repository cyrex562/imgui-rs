use crate::color::{ImGuiCol_Text, ImGuiCol_TextDisabled};
use crate::font::ImFont;
use crate::item::item_ops::{CalcItemWidth, calc_width_for_pos, IsClippedEx, ItemAdd, ItemSize};
use crate::core::math_ops::ImMax;
use crate::rect::ImRect;
use crate::drawing::render_ops::{
    FindRenderedTextEnd, RenderBullet, RenderText, RenderTextClipped, RenderTextWrapped,
};
use crate::core::string_ops::ImFormatStringToTempBufferV;
use crate::style_ops::{GetColorU32, PopStyleColor, PushStyleColor, PushStyleColor2};
use crate::text_flags::{ImGuiTextFlags, ImGuiTextFlags_NoWidthForLargeClippedText};
use crate::core::utils::flag_clear;
use crate::core::vec2::Vector2;
use crate::core::vec4::ImVec4;
use crate::viewport::widget_ops::{PopTextWrapPos, PushTextWrapPos};
use crate::window::ops::GetCurrentWindow;
use crate::window::ImguiWindow;
use crate::GImGui;
use libc::{c_char, c_float, c_int};
use std::ptr::{null, null_mut};
use crate::core::context::AppContext;

// Calculate text size. Text can be multi-line. Optionally ignore text after a ## marker.
// CalcTextSize("") should return ImVec2::new(0.0, g.FontSize)
// CalcTextSize: ImVec2(text: &String, text_end: *const c_char, hide_text_after_double_hash: bool, c_float wrap_width)
pub fn CalcTextSize(
    g: &mut AppContext,
    text: &String,
    hide_text_after_double_hash: bool,
    wrap_width: c_float,
) -> Vector2 {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
                    // let text_display_end: *const c_char;
                    // if hide_text_after_double_hash {
                    //     text_display_end = FindRenderedTextEnd(text);
                    // }     // Hide anything after a '##' string
                    // else {
                    //     text_display_end = text_end;
                    // }

    let mut font = g.Font;
    let font_size: c_float = g.FontSize;
    // if text == text_display_end {
    //     return ImVec2::new2(0.0, font_size);
    // }
    let mut text_size: Vector2 =
        font.CalcTextSizeA(font_size, f32::MAX, wrap_width, text, None);

    // Round
    // FIXME: This has been here since Dec 2015 (7b0bf230) but down the line we want this out.
    // FIXME: Investigate using ceilf or e.g.
    // - https://git.musl-libc.org/cgit/musl/tree/src/math/ceilf.c
    // - https://embarkstudios.github.io/rust-gpu/api/src/libm/math/ceilf.rs.html
    text_size.x = (text_size.x + 0.999990f32).floor();

    return text_size;
}

// GetTextLineHeight: c_float()
pub unsafe fn GetTextLineHeight() -> f32 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize;
}

pub unsafe fn GetTextLineHeightWithSpacing() -> f32 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize + g.style.ItemSpacing.y;
}

pub fn TextEx(g: &mut AppContext, mut text: &String, flags: ImGuiTextFlags) {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return;
    }
    // Calculate length
    let mut text_begin = 0usize;

    let mut text_pos = Vector2::from_floats(
        window.dc.cursor_pos.x,
        window.dc.cursor_pos.y + window.dc.CurrLineTextBaseOffset,
    );
    let wrap_pos_x = window.dc.TextWrapPos;
    let wrap_enabled = (wrap_pos_x >= 0.0);
    if text.len() <= 2000 || wrap_enabled {
        // Common case
        let wrap_width = if wrap_enabled {
            calc_width_for_pos(g, &window.dc.cursor_pos, wrap_pos_x)
        } else {
            0.0
        };
        let text_size: Vector2 = CalcTextSize(, text, false, wrap_width);

        let mut bb: ImRect = ImRect::new(text_pos, text_pos + text_size);
        ItemSize(g, &text_size, 0.0);
        if !ItemAdd(g, &mut bb, 0, None, 0) {
            return;
        }

        // Render (we don't hide text after ## in this end-user function)
        RenderTextWrapped(g, bb.min, text);
    } else {
        // Long text!
        // Perform manual coarse clipping to optimize for long multi-line text
        // - From this point we will only compute the width of lines that are visible. Optimization only available when word-wrapping is disabled.
        // - We also don't vertically center the text within the line full height, which is unlikely to matter because we are likely the biggest and only item on the line.
        // - We use memchr(), pay attention that well optimized versions of those str/mem functions are much faster than a casually written loop.
        let mut line: String = text.to_string();
        let line_height: c_float = GetTextLineHeight();
        let mut text_size = Vector2::from_floats(0.0, 0.0);

        // Lines to skip (can't skip when logging text)
        let mut pos: Vector2 = text_pos;
        if !g.LogEnabled {
            let lines_skippable: c_int = ((window.ClipRect.Min.y - text_pos.y) / line_height);
            if lines_skippable > 0 {
                let mut lines_skipped: c_int = 0;
                while line < text_end && lines_skipped < lines_skippable {
                    // let mut  line_end: &str = libc::memchr(line, '\n' as c_int, text_end - line);
                    let line_end = line.find('\n').unwrap_or(line.len() - 1);
                    // if !line_end {
                    //     line_end = text_end;}
                    if flag_clear(flags, ImGuiTextFlags_NoWidthForLargeClippedText) {
                        text_size.x = ImMax(text_size.x, CalcTextSize(, line.as_str(), false, 0.0).x);
                    }
                    line = line[line_end + 1..].into_string();
                    lines_skipped += 1;
                }
                pos.y += lines_skipped * line_height;
            }
        }

        // Lines to render
        if line < text_end {
            let mut line_rect: ImRect =
                ImRect::new(pos, pos + Vector2::from_floats(f32::MAX, line_height));
            while line < text_end {
                if IsClippedEx(&mut line_rect, 0) {
                    break;
                }

                // let mut  line_end: &str = libc::memchr(line, '\n' as c_int, text_end - line);
                let mut line_end = line.find('\n').unwrap_or(usize::MAX);
                if line_end == usize::MAX {
                    line_end = text_end;
                }
                text_size.x = ImMax(text_size.x, CalcTextSize(, line.as_str(), false, 0.0).x);
                RenderText(pos, line.as_str(), false, g);
                line = line[line_end + 1..].to_string();
                line_rect.min.y += line_height;
                line_rect.max.y += line_height;
                pos.y += line_height;
            }

            // Count remaining lines
            let mut lines_skipped: c_int = 0;
            while (line < text_end) {
                // let mut  line_end: &str = libc::memchr(line, '\n' as c_int, text_end - line);
                let mut line_end = line.find('\n').unwrap_or(usize::MAX);
                if line_end == usize::MAX {
                    line_end = text_end;
                }
                if flag_clear(flags, ImGuiTextFlags_NoWidthForLargeClippedText) {
                    text_size.x = ImMax(text_size.x, CalcTextSize(, line.as_str(), false, 0.0).x);
                }
                ine = line[line_end + 1..].to_string();
                lines_skipped += 1;
            }
            pos.y += lines_skipped * line_height;
        }
        text_size.y = (pos - text_pos).y;

        let mut bb: ImRect = ImRect::new(text_pos, text_pos + text_size);
        ItemSize(g, &text_size, 0.0);
        ItemAdd(g, &mut bb, 0, None, 0);
    }
}

pub unsafe fn TextUnformatted(text: String) {
    TextEx(g, text, ImGuiTextFlags_NoWidthForLargeClippedText);
}

pub fn Text(fmt: String) {
    // va_list args;
    // va_start(args, fmt);
    TextV(g, fmt);
    // va_end(args);
}

pub fn TextV(g: &mut AppContext, fmt: &String) {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return;
    }
    // let mut text = ImFormatStringToTempBufferV(fmt);
    TextEx(g, fmt, ImGuiTextFlags_NoWidthForLargeClippedText);
}

pub unsafe fn TextColored(mut col: &ImVec4, fmt: String) {
    // va_list args;
    // va_start(args, fmt);
    TextColoredV(col, fmt);
    // va_end(args);
}

pub unsafe fn TextColoredV(mut col: &ImVec4, fmt: String) {
    PushStyleColor2(ImGuiCol_Text, col);
    if fmt[0] == '%' && fmt[1] == 's' && fmt[2] == 0 {
        TextEx(g, fmt, ImGuiTextFlags_NoWidthForLargeClippedText);
    }
    // Skip formatting
    else {
        TextV(g, fmt);
    }
    PopStyleColor(0);
}

pub unsafe fn TextDisabled(fmt: String) {
    // va_list args;
    // va_start(args, fmt);
    TextDisabledV(fmt);
    // va_end(args);
}

pub unsafe fn TextDisabledV(fmt: String) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    PushStyleColor(ImGuiCol_Text, g.style.Colors[ImGuiCol_TextDisabled]);
    if fmt[0] == '%' && fmt[1] == 's' && fmt[2] == 0 {
        TextEx(g, fmt, ImGuiTextFlags_NoWidthForLargeClippedText);
    }
    // Skip formatting
    else {
        TextV(g, fmt);
    }
    PopStyleColor(0);
}

pub unsafe fn TextWrapped(fmt: String) {
    // va_list args;
    // va_start(args, fmt);
    TextWrappedV(fmt);
    // va_end(args);
}

pub unsafe fn TextWrappedV(fmt: String) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut need_backup: bool = (g.Currentwindow.DC.TextWrapPos < 0.0); // Keep existing wrap position if one is already set
    if need_backup {
        PushTextWrapPos(0.0);
    }
    if fmt[0] == '%' && fmt[1] == 's' && fmt[2] == 0 {
        TextEx(g, fmt, ImGuiTextFlags_NoWidthForLargeClippedText);
    }
    // Skip formatting
    else {
        TextV(g, fmt);
    }
    if need_backup {
        PopTextWrapPos();
    }
}

pub unsafe fn LabelText(label: String, fmt: String) {
    // va_list args;
    // va_start(args, fmt);
    LabelTextV(label, fmt);
    // va_end(args);
}

// Add a label+text combo aligned to other label+value widgets
pub unsafe fn LabelTextV(label: String, fmt: String) {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let setyle = &mut g.style;
    let w: c_float = CalcItemWidth(g);

    // let mut value_text_begin: *mut c_char;
    // let mut value_text_end: *mut c_char;
    let value_text_begin = ImFormatStringToTempBufferV(fmt);
    let value_size: Vector2 = CalcTextSize(, &value_text_begin, false, 0.0);
    let label_size: Vector2 = CalcTextSize(, label, true, 0.0);

    let pos: Vector2 = window.dc.cursor_pos;
    let mut value_bb: ImRect = ImRect::new(
        pos,
        pos + Vector2::from_floats(w, value_size.y + style.FramePadding.y * 2),
    );
    let mut total_bb: ImRect = ImRect::new(
        pos,
        pos + Vector2::from_floats(
            w + (if label_size.x > 0.0 {
                style.ItemInnerSpacing.x + label_size.x
            } else {
                0.0
            }),
            ImMax(value_size.y, label_size.y) + style.FramePadding.y * 2,
        ),
    );
    ItemSize(g, &otal_bb.GetSize(), style.FramePadding.y);
    if !ItemAdd(g, &mut total_bb, 0, None, 0) {
        return;
    }

    // Render
    RenderTextClipped(
        value_bb.min + style.FramePadding,
        &value_bb.max,
        value_text_begin.as_str(),
        &value_size,
        Some(&Vector2::from_floats(0.0, 0.0)),
        None,
    );
    if (label_size.x > 0.0) {
        RenderText(
            Vector2::from_floats(
                value_bb.max.x + style.ItemInnerSpacing.x,
                value_bb.min.y + style.FramePadding.y,
            ),
            label,
            false,
            g,
        );
    }
}

pub unsafe fn BulletText(fmt: String) {
    // va_list args;
    // va_start(args, fmt);
    BulletTextV(fmt);
    // va_end(args);
}

// Text with a little bullet aligned to the typical tree node.
pub unsafe fn BulletTextV(fmt: String) {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let setyle = &mut g.style;

    // text_begin: &str, *text_end;
    // let mut text_begin: *mut c_char = None;
    // let mut text_end: *mut c_char = None;
    let text_begin = ImFormatStringToTempBufferV(fmt);
    let label_size: Vector2 = CalcTextSize(, text_begin, false, 0.0);
    let total_size: Vector2 = Vector2::from_floats(
        g.FontSize
            + (if label_size.x > 0.0 {
                (label_size.x + style.FramePadding.x * 2)
            } else {
                0.0
            }),
        label_size.y,
    ); // Empty text doesn't add padding
    let mut pos: Vector2 = window.dc.cursor_pos;
    pos.y += window.dc.CurrLineTextBaseOffset;
    ItemSize(g, &total_size, 0.0);
    let mut bb: ImRect = ImRect::new(pos, pos + total_size);
    if !ItemAdd(g, &mut bb, 0, None, 0) {
        return;
    }

    // Render
    text_col: u32 = GetColorU32(ImGuiCol_Text, 0.0);
    RenderBullet(
        window.DrawList,
        bb.min + Vector2::from_floats(style.FramePadding.x + g.FontSize * 0.5, g.FontSize * 0.5),
        text_col,
    );
    RenderText(
        bb.min + Vector2::from_floats(g.FontSize + style.FramePadding.x * 2, 0.0),
        text_begin.as_str(),
        false,
        g,
    );
}
