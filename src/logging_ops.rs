#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::{c_char, c_int};
use crate::imgui_cpp::{GImGui, ImStreolRange};
use crate::vec2::ImVec2;

// Internal version that takes a position to decide on newline placement and pad items according to their depth.
// We split text into individual lines to add current tree level padding
// FIXME: This code is a little complicated perhaps, considering simplifying the whole system.
// c_void LogRenderedText(*const ImVec2 ref_pos, *const char text, *const char text_end)
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




//-----------------------------------------------------------------------------
// [SECTION] LOGGING/CAPTURING
//-----------------------------------------------------------------------------
// All text output from the interface can be captured into tty/file/clipboard.
// By default, tree nodes are automatically opened during logging.
//-----------------------------------------------------------------------------

// Pass text data straight to log (without being displayed)
static inline c_void LogTextV(ImGuiContext& g, *const char fmt, va_list args)
{
    if (g.LogFile)
    {
        g.LogBuffer.Buf.clear();
        g.LogBuffer.appendfv(fmt, args);
        ImFileWrite(g.LogBuffer.c_str(), sizeof, (u64)g.LogBuffer.size(), g.LogFile);
    }
    else
    {
        g.LogBuffer.appendfv(fmt, args);
    }
}

c_void LogText(*const char fmt, ...)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.LogEnabled)
        return;

    va_list args;
    va_start(args, fmt);
    LogTextV(g, fmt, args);
    va_end(args);
}

c_void LogTextV(*const char fmt, va_list args)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.LogEnabled)
        return;

    LogTextV(g, fmt, args);
}

// Start logging/capturing text output
c_void LogBegin(ImGuiLogType type, c_int auto_open_depth)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT(g.LogEnabled == false);
    // IM_ASSERT(g.LogFile == NULL);
    // IM_ASSERT(g.LogBuffer.empty());
    g.LogEnabled = true;
    g.LogType = type;
    g.LogNextPrefix = g.LogNextSuffix= null_mut();
    g.LogDepthRef = window.DC.TreeDepth;
    g.LogDepthToExpand = ((auto_open_depth >= 0) ? auto_open_depth : g.LogDepthToExpandDefault);
    g.LogLinePosY = f32::MAX;
    g.LogLineFirstItem = true;
}

// Important: doesn't copy underlying data, use carefully (prefix/suffix must be in scope at the time of the next LogRenderedText)
c_void LogSetNextTextDecoration(*const char prefix, *const char suffix)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.LogNextPrefix = prefix;
    g.LogNextSuffix = suffix;
}

c_void LogToTTY(c_int auto_open_depth)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.LogEnabled)
        return;
    IM_UNUSED(auto_open_depth);
// #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
    LogBegin(ImGuiLogType_TTY, auto_open_depth);
    g.LogFile = stdout;
// #endif
}

// Start logging/capturing text output to given file
c_void LogToFile(c_int auto_open_depth, *const char filename)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.LogEnabled)
        return;

    // FIXME: We could probably open the file in text mode "at", however note that clipboard/buffer logging will still
    // be subject to outputting OS-incompatible carriage return if within strings the user doesn't use IM_NEWLINE.
    // By opening the file in binary mode "ab" we have consistent output everywhere.
    if (!filename)
        filename = g.IO.LogFilename;
    if (!filename || !filename[0])
        return;
    ImFileHandle f = ImFileOpen(filename, "ab");
    if (!0f32)
    {
        // IM_ASSERT(0);
        return;
    }

    LogBegin(ImGuiLogType_File, auto_open_depth);
    g.LogFile = f;
}

// Start logging/capturing text output to clipboard
c_void LogToClipboard(c_int auto_open_depth)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.LogEnabled)
        return;
    LogBegin(ImGuiLogType_Clipboard, auto_open_depth);
}

c_void LogToBuffer(c_int auto_open_depth)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.LogEnabled)
        return;
    LogBegin(ImGuiLogType_Buffer, auto_open_depth);
}

c_void LogFinish()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.LogEnabled)
        return;

    LogText(IM_NEWLINE);
    switch (g.LogType)
    {
    case ImGuiLogType_TTY:
// #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
        fflush(g.LogFile);
// #endif
        break;
    case ImGuiLogType_File:
        ImFileClose(g.LogFile);
        break;
    case ImGuiLogType_Buffer:
        break;
    case ImGuiLogType_Clipboard:
        if (!g.LogBuffer.empty())
            SetClipboardText(g.LogBuffer.begin());
        break;
    case ImGuiLogType_None:
        // IM_ASSERT(0);
        break;
    }

    g.LogEnabled = false;
    g.LogType = ImGuiLogType_None;
    g.LogFile= null_mut();
    g.LogBuffer.clear();
}

// Helper to display logging buttons
// FIXME-OBSOLETE: We should probably obsolete this and let the user have their own helper (this is one of the oldest function alive!)
c_void LogButtons()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    PushID("LogButtons");
// #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
    let log_to_tty: bool = Button("Log To TTY"); SameLine();
// #else
    let log_to_tty: bool = false;
// #endif
    let log_to_file: bool = Button("Log To File"); SameLine();
    let log_to_clipboard: bool = Button("Log To Clipboard"); SameLine();
    PushAllowKeyboardFocus(false);
    SetNextItemWidth(80f32);
    SliderInt("Default Depth", &g.LogDepthToExpandDefault, 0, 9, null_mut());
    PopAllowKeyboardFocus();
    PopID();

    // Start logging at the end of the function so that the buttons don't appear in the log
    if (log_to_tty)
        LogToTTY();
    if (log_to_file)
        LogToFile();
    if (log_to_clipboard)
        LogToClipboard();
}

