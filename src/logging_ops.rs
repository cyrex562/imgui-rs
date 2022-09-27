#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::{c_char, c_int};
use crate::imgui_cpp::{GImGui, ImStreolRange};
use crate::vec2::ImVec2;

// Internal version that takes a position to decide on newline placement and pad items according to their depth.
// We split text into individual lines to add current tree level padding
// FIXME: This code is a little complicated perhaps, considering simplifying the whole system.
// c_void ImGui::LogRenderedText(*const ImVec2 ref_pos, *const char text, *const char text_end)
pub unsafe fn LogRenderedText(ref_pos: *const ImVec2, text: *const c_char, mut text_end: *const c_char) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    let mut prefix: *const c_char = g.LogNextPrefix;
    let mut suffix: *const c_char = g.LogNextSuffix;
    g.LogNextPrefix = null_mut();
    g.LogNextSuffix = null_mut();

    if !text_end {
        text_end = FindRenderedTextEnd(text, text_end);
    }

    let log_new_line = ref_pos.is_null() == false && (ref_pos.y > g.LogLinePosY + g.Style.FramePadding.y + 1);
    if ref_pos {
        g.LogLinePosY = ref_pos.y;
    }
    if log_new_line {
        LogText(IM_NEWLINE);
        g.LogLineFirstItem = true;
    }

    if prefix {
        LogRenderedText(ref_pos, prefix, prefix + libc::strlen(prefix)); // Calculate end ourself to ensure "##" are included here.
    }
// Re-adjust padding if we have popped out of our starting depth
    if g.LogDepthRef > window.DC.TreeDepth {
        g.LogDepthRef = window.DC.TreeDepth;
    }
    let tree_depth: c_int = (window.DC.TreeDepth - g.LogDepthRef);

    let mut text_remaining: *const c_char = text;
    loop {
// Split the string. Each new line (after a '\n') is followed by indentation corresponding to the current depth of our log entry.
// We don't add a trailing \n yet to allow a subsequent item on the same line to be captured.
        let mut line_start: *const c_char = text_remaining;
        let mut line_end: *const c_char = ImStreolRange(line_start, text_end);
        let is_last_line: bool = (line_end == text_end);
        if line_start != line_end || !is_last_line {
            let line_length: c_int = (line_end - line_start);
            let indentation: c_int = if g.LogLineFirstItem {
                tree_depth * 4
            } else { 1 };
            LogText("%*s%.*s", indentation, "", line_length, line_start);
            g.LogLineFirstItem = false;
            if *line_end == '\n' as c_char {
                LogText(IM_NEWLINE);
                g.LogLineFirstItem = true;
            }
        }
        if is_last_line {
            break;
        }
        text_remaining = line_end + 1;
    }

    if suffix {
        LogRenderedText(ref_pos, suffix, suffix + libc::strlen(suffix));
    }
}
