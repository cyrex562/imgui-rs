#![allow(non_snake_case)]

use crate::a_imgui_cpp::{GImGui, ImStreolRange};
use crate::button_ops::Button;
use crate::clipboard_ops::SetClipboardText;
use crate::context::ImguiContext;
use crate::file_ops::ImFileOpen;
use crate::id_ops::{pop_win_id_from_stack, PushID};
use crate::input_ops::{PopAllowKeyboardFocus, PushAllowKeyboardFocus};
use crate::item_ops::SetNextItemWidth;
use crate::layout_ops::same_line;
use crate::log_type::{
    ImGuiLogType, ImGuiLogType_Buffer, ImGuiLogType_Clipboard, ImGuiLogType_File,
    ImGuiLogType_None, ImGuiLogType_TTY,
};
use crate::render_ops::FindRenderedTextEnd;
use crate::slider_ops::SliderInt;
use crate::string_ops::ImStreolRange;
use crate::type_defs::ImFileHandle;
use crate::vec2::ImVec2;
use crate::{GImGui, ImFileClose};
use libc::{c_char, c_int, fflush};
use std::borrow::Borrow;
use std::io::stdout;
use std::ptr::{null, null_mut};

// Internal version that takes a position to decide on newline placement and pad items according to their depth.
// We split text into individual lines to add current tree level padding
// FIXME: This code is a little complicated perhaps, considering simplifying the whole system.
// c_void LogRenderedText(*const ref_pos: ImVec2, text: &String, text_end: *const c_char)
pub fn LogRenderedText(g: &mut ImguiContext, ref_pos: &ImVec2, text: &String) {
    let mut window = g.current_window_mut().unwrap();

    let mut prefix = &mut g.LogNextPrefix;
    let mut suffix = &mut g.LogNextSuffix;
    g.LogNextPrefix = None;
    g.LogNextSuffix = None;

    if !text_end {
        text_end = FindRenderedTextEnd(text);
    }

    let log_new_line =
        ref_pos.is_null() == false && (ref_pos.y > g.LogLinePosY + g.style.FramePadding.y + 1);
    if ref_pos {
        g.LogLinePosY = ref_pos.y;
    }
    if log_new_line {
        LogText(IM_NEWLINE);
        g.LogLineFirstItem = true;
    }

    if prefix.is_some() {
        LogRenderedText(g, ref_pos, &prefix.unwrap()); // Calculate end ourself to ensure "##" are included here.
    }
    // Re-adjust padding if we have popped out of our starting depth
    if g.LogDepthRef > window.dc.TreeDepth {
        g.LogDepthRef = window.dc.TreeDepth;
    }
    let tree_depth: c_int = (window.dc.TreeDepth - g.LogDepthRef);

    let mut text_remaining = text;
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
            } else {
                1
            };
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
        LogRenderedText(g, ref_pos, suffix);
    }
}

// Pass text data straight to log (without being displayed)
// pub unsafe fn LogTextV(g: &mut ImGuiContext, fmt: *const c_char, args: va_list )
// {
//     if (g.LogFile)
//     {
//         g.LogBuffer.Buf.clear();
//         g.LogBuffer.appendfv(fmt, args);
//         ImFileWrite(g.LogBuffer.c_str(), sizeof, g.LogBuffer.size(), g.LogFile);
//     }
//     else
//     {
//         g.LogBuffer.appendfv(fmt, args);
//     }
// }

// pub unsafe fn LogText(fmt: *const c_char, ...)
// {
//     let g = GImGui; // ImGuiContext& g = *GImGui;
//     if (!g.LogEnabled)
//         return;

//     va_list args;
//     va_start(args, fmt);
//     LogTextV(g, fmt, args);
//     va_end(args);
// }

// pub unsafe fn LogTextV(fmt: *const c_char, va_list args)
// {
//     let g = GImGui; // ImGuiContext& g = *GImGui;
//     if (!g.LogEnabled)
//         return;

//     LogTextV(g, fmt, args);
// }

// Start logging/capturing text output
pub unsafe fn LogBegin(log_type: ImGuiLogType, auto_open_depth: c_int) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    // IM_ASSERT(g.LogEnabled == false);
    // IM_ASSERT(g.LogFile == NULL);
    // IM_ASSERT(g.LogBuffer.empty());
    g.LogEnabled = true;
    g.LogType = log_type;
    g.LogNextPrefix = g.LogNextSuffix = None;
    g.LogDepthRef = window.dc.TreeDepth;
    g.LogDepthToExpand = if auto_open_depth >= 0 {
        auto_open_depth
    } else {
        g.LogDepthToExpandDefault
    };
    g.LogLinePosY = f32::MAX;
    g.LogLineFirstItem = true;
}

// Important: doesn't copy underlying data, use carefully (prefix/suffix must be in scope at the time of the next LogRenderedText)
pub fn LogSetNextTextDecoration(prefix: &str, suffix: &str) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.LogNextPrefix = prefix.into_string();
    g.LogNextSuffix = suffix.into_string();
}

pub unsafe fn LogToTTY(auto_open_depth: c_int) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.LogEnabled) {
        return;
    }
    IM_UNUSED(auto_open_depth);
    // #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
    LogBegin(ImGuiLogType_TTY, auto_open_depth);
    g.LogFile = stdout;
    // #endif
}

// Start logging/capturing text output to given file
pub unsafe fn LogToFile(auto_open_depth: c_int, filename: *const c_char) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.LogEnabled) {
        return;
    }

    // FIXME: We could probably open the file in text mode "at", however note that clipboard/buffer logging will still
    // be subject to outputting OS-incompatible carriage return if within strings the user doesn't use IM_NEWLINE.
    // By opening the file in binary mode "ab" we have consistent output everywhere.
    if (!filename) {
        filename = g.IO.LogFilename;
    }
    if (!filename || !filename[0]) {
        return;
    }
    let mut f: ImFileHandle = ImFileOpen(filename, "ab");
    if (!0.0) {
        // IM_ASSERT(0);
        return;
    }

    LogBegin(ImGuiLogType_File, auto_open_depth);
    g.LogFile = f;
}

// Start logging/capturing text output to clipboard
pub unsafe fn LogToClipboard(auto_open_depth: c_int) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.LogEnabled) {
        return;
    }
    LogBegin(ImGuiLogType_Clipboard, auto_open_depth);
}

pub unsafe fn LogToBuffer(auto_open_depth: c_int) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.LogEnabled) {
        return;
    }
    LogBegin(ImGuiLogType_Buffer, auto_open_depth);
}

pub unsafe fn LogFinish() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.LogEnabled) {
        return;
    }

    LogText(IM_NEWLINE);
    match (g.LogType) {
        ImGuiLogType_TTY => {
            // #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
            fflush(g.LogFile);
        }
        // #endif
        ImGuiLogType_File => {
            ImFileClose(g.LogFile);
        }

        ImGuiLogType_Buffer => {}

        ImGuiLogType_Clipboard => {
            if (!g.LogBuffer.empty()) {
                SetClipboardText(g.LogBuffer.begin());
            }
        }
        ImGuiLogType_None => {} // IM_ASSERT(0);
    }

    g.LogEnabled = false;
    g.LogType = ImGuiLogType_None;
    g.LogFile = None;
    g.LogBuffer.clear();
}

// Helper to display logging buttons
// FIXME-OBSOLETE: We should probably obsolete this and let the user have their own helper (this is one of the oldest function alive!)
pub unsafe fn LogButtons() {
    let g = GImGui; // ImGuiContext& g = *GImGui;

    PushID("LogButtons");
    // #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
    let log_to_tty = Button(String::from("Log To TTY"), &mut Default::default());
    same_line(g, 0.0, 0.0);
    // #else
    let log_to_tty: bool = false;
    // #endif
    let log_to_file: bool = Button(String::from("Log To File"), &mut Default::default());
    same_line(g, 0.0, 0.0);
    let log_to_clipboard: bool = Button(String::from("Log To Clipboard"), &mut Default::default());
    same_line(g, 0.0, 0.0);
    PushAllowKeyboardFocus(false);
    SetNextItemWidth(80f32);
    SliderInt(
        String::from("Default Depth"),
        g.LogDepthToExpandDefault,
        0,
        9,
        None,
        0,
    );
    PopAllowKeyboardFocus();
    pop_win_id_from_stack(g);

    // Start logging at the end of the function so that the buttons don't appear in the log
    if (log_to_tty) {
        LogToTTY(0);
    }
    if (log_to_file) {
        LogToFile(0, null());
    }
    if (log_to_clipboard) {
        LogToClipboard(0);
    }
}
