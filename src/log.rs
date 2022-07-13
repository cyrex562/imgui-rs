use std::os::raw::c_char;
use std::ptr::{null, null_mut};
use crate::imgui::SetNextItemWidth;
use crate::imgui_clipboard::SetClipboardText;
use crate::imgui_context::ImGuiContext;
use crate::imgui_file::{ImFileClose, ImFileOpen, ImFileWrite};
use crate::imgui_globals::GImGui;
use crate::imgui_h::ImGuiColor::Button;
use crate::imgui_render::FindRenderedTextEnd;
use crate::imguI_string::ImStreolRange;
use crate::imgui_vec::Vector2D;

pub enum ImGuiLogType
{
    None = 0,
    TTY,
    File,
    Buffer,
    Clipboard
}

pub enum ImGuiDebugLogFlags
{
    // Event types
    None             = 0,
    EventActiveId    = 1 << 0,
    EventFocus       = 1 << 1,
    EventPopup       = 1 << 2,
    EventNav         = 1 << 3,
    EventIO          = 1 << 4,
    EventDocking     = 1 << 5,
    EventViewport    = 1 << 6,
    OutputToTTY      = 1 << 10   // Also send output to TTY
}

pub const ImGuiDebugLogFlags_EventMask: u32 = ImGuiDebugLogFlags::EventActiveId | ImGuiDebugLogFlags::EventFocus | ImGuiDebugLogFlags::EventPopup | ImGuiDebugLogFlags::EventNav | ImGuiDebugLogFlags::EventIO | ImGuiDebugLogFlags::EventDocking | ImGuiDebugLogFlags::EventViewport;



// Internal version that takes a position to decide on newline placement and pad items according to their depth.
// We split text into individual lines to add current tree level padding
// FIXME: This code is a little complicated perhaps, considering simplifying the whole system.
// void ImGui::LogRenderedText(const Vector2D* ref_pos, const char* text, const char* text_end)
pub unsafe fn LogRenderedText(ref_pos: &Vector2D, text: *const c_char, mut text_end: *const c_char)
{
    // ImGuiContext& g = *GImGui;
    let g = GImGui;
    // ImGuiWindow* window = g.current_window;
    let window = g.current_window;

    // const char* prefix = g.LogNextPrefix;
    let prefix = g.LogNextPrefix;
    // const char* suffix = g.LogNextSuffix;
    let suffix = g.LogNextSuffix;
    g.LogNextPrefix =  null();
    g.LogNextSuffix = null();

    if (!text_end) {
        text_end = FindRenderedTextEnd(text, text_end);
    }

    let log_new_line = ref_pos != Vector2D::new(0.0,0.0) && (ref_pos.y > g.LogLinePosY + g.style.FramePadding.y + 1);
    if (ref_pos) {
        g.LogLinePosY = ref_pos.y;
    }
    if (log_new_line)
    {
        LogText(IM_NEWLINE);
        g.LogLineFirstItem = true;
    }

    if (prefix) {
        LogRenderedText(ref_pos, prefix, prefix + (prefix.len())); // Calculate end ourself to ensure "##" are included here.
        }

    // Re-adjust padding if we have popped out of our starting depth
    if (g.LogDepthRef > window.DC.TreeDepth) {
        g.LogDepthRef = window.DC.TreeDepth;
    }
    let tree_depth = (window.DC.TreeDepth - g.LogDepthRef);

    let mut text_remaining = text;
    loop
    {
        // split the string. Each new line (after a '\n') is followed by indentation corresponding to the current depth of our log entry.
        // We don't add a trailing \n yet to allow a subsequent item on the same line to be captured.
        let line_start = text_remaining;
        let line_end = ImStreolRange(line_start, text_end);
        let is_last_line = (line_end == text_end);
        if line_start != line_end || !is_last_line
        {
            let line_length = (line_end - line_start);
            let indentation = if(g.LogLineFirstItem) { tree_depth * 4 } else { 1 };
            LogText("%*s%.*s", indentation, "", line_length, line_start);
            g.LogLineFirstItem = false;
            if (*line_end == '\n')
            {
                LogText(IM_NEWLINE);
                g.LogLineFirstItem = true;
            }
        }
        if (is_last_line) {
            break;
        }
        text_remaining = line_end + 1;
    }

    // Pass text data straight to log (without being displayed)
// static inline void LogTextV(ImGuiContext& g, const char* fmt, va_list args)
pub fn LogTextV(g: &mut ImGuiContext, msg: &String)
    {
    if (g.log_file)
    {
        g.LogBuffer.Buf.resize(0);
        // g.LogBuffer.appendfv(fmt, args);
        g.LogBuffer.append(msg.as_ptr(), null());
        ImFileWrite(g.LogBuffer.Buf.as_vec(), g.LogBuffer.size(), g.LogBuffer.size(), g.log_file);
    }
    else
    {
        g.LogBuffer.appendfv(msg);
    }
}



// void ImGui::LogText(const char* fmt, ...)
pub fn LogText(msg: &String)
    {
    // ImGuiContext& g = *GImGui;
    let g = GImGui;
        if (!g.LogEnabled) {
            return;
        }

    // va_list args;
    // va_start(args, fmt);
    // LogTextV(g, fmt, args);
    // va_end(args);
}

// void ImGui::LogTextV(const char* fmt, va_list args)
// {
//     ImGuiContext& g = *GImGui;
//     if (!g.log_enabled)
//         return;
//
//     LogTextV(g, fmt, args);
// }

// Start logging/capturing text output
// void ImGui::LogBegin(ImGuiLogType type, int auto_open_depth)
pub fn LogBegin(log_type: ImGuiLogType, auto_open_depth: i32)
    {
    // ImGuiContext& g = *GImGui;
    let g = GImGui;
        // ImGuiWindow* window = g.current_window;
    let window = g.current_window;
        // IM_ASSERT(g.log_enabled == false);
    // IM_ASSERT(g.log_file == NULL);
    // IM_ASSERT(g.LogBuffer.empty());
    g.LogEnabled = true;
    g.LogType = log_type;
    g.LogNextPrefix = null();
        g.LogNextSuffix = null();
    g.LogDepthRef = window.DC.TreeDepth;
    g.LogDepthToExpand = if auto_open_depth >= 0 { auto_open_depth } else { g.LogDepthToExpandDefault };
    g.LogLinePosY = f32::MAX;
    g.LogLineFirstItem = true;
}

// Important: doesn't copy underlying data, use carefully (prefix/suffix must be in scope at the time of the next LogRenderedText)
// void ImGui::LogSetNextTextDecoration(const char* prefix, const char* suffix)
pub fn LogSetNextTextDecoration(prefix: *const c_char, suffix: *const c_char)
    {
    // ImGuiContext& g = *GImGui;
    let g = GImGui;
        g.LogNextPrefix = prefix;
    g.LogNextSuffix = suffix;
}

// void ImGui::LogToTTY(int auto_open_depth)
pub fn LogToTTY(auto_open_depth: i32)
    {
    // ImGuiContext& g = *GImGui;
    let g = GImGui;
        if (g.LogEnabled) {
            return;
        }
    // IM_UNUSED(auto_open_depth);
// #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
//     LogBegin(ImGuiLogType_TTY, auto_open_depth);
//     g.log_file = stdout;
// #endif
}

// Start logging/capturing text output to given file
// void ImGui::LogToFile(int auto_open_depth, const char* filename)
pub fn LogToFile(auto_open_depth: i32, mut filename: *const c_char)
    {
    let g = GImGui;
    if (g.LogEnabled) {
        return;
    }

    // FIXME: We could probably open the file in text mode "at", however note that clipboard/buffer logging will still
    // be subject to outputting OS-incompatible carriage return if within strings the user doesn't use IM_NEWLINE.
    // By opening the file in binary mode "ab" we have consistent output everywhere.
    if !filename {
        filename = g.io.LogFilename.as_ptr() as *const c_char;
    }
    if !filename || !filename[0] {
        return;
    }
    let f = ImFileOpen(&String::from(filename), &String::from("ab"));
    if (!f)
    {
        // IM_ASSERT(0);
        return;
    }

    LogBegin(ImGuiLogType::File, auto_open_depth);
    g.log_file = f;
}

// Start logging/capturing text output to clipboard
// void ImGui::LogToClipboard(int auto_open_depth)
pub fn LogToClipboard(auto_open_depth: i32)
    {
    // ImGuiContext& g = *GImGui;
    let g = GImGui;
        if (g.LogEnabled) {
            return;
        }
    LogBegin(ImGuiLogType::Clipboard, auto_open_depth);
}

// void ImGui::LogToBuffer(int auto_open_depth)
pub fn LogToBuffer(auto_open_depth: i32)
    {
    // ImGuiContext& g = *GImGui;
    let g = GImGui;
        if (g.LogEnabled) {
            return;
        }
    LogBegin(ImGuiLogType::Buffer, auto_open_depth);
}

// void ImGui::LogFinish()
pub fn LogFinish()
    {
    // ImGuiContext& g = *GImGui;
    let g = GImGui;
        if (!g.LogEnabled) {
            return;
        }

    LogText(&String::from("\n"));
    // switch (g.log_type)
    match g.LogType
        {
//     case ImGuiLogType_TTY:
// #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
//         fflush(g.log_file);
// #endif
//         break;
            ImGuiLogType::TTY => {

            },
    // case ImGuiLogType_File:
    ImGuiLogType::File => {
        ImFileClose(g.log_file); }

    // case ImGuiLogType_Buffer:
    //     break;
    ImGuiLogType::Buffer => { },
            // case ImGuiLogType_Clipboard:
            ImGuiLogType::Clipboard => {
                if (!g.LogBuffer.empty()) {
                    SetClipboardText(g, g.LogBuffer.begin() as *const c_char);
                }
            }
        // break;
    // case ImGuiLogType_None:
    //     IM_ASSERT(0);
    //     break;
            ImGuiLogType::None => {}
    }

    g.LogEnabled = false;
    g.LogType = ImGuiLogType::None;
    g.log_file = null_mut();
    g.LogBuffer.clear();
}

// Helper to display logging buttons
// FIXME-OBSOLETE: We should probably obsolete this and let the user have their own helper (this is one of the oldest function alive!)
// void ImGui::LogButtons()
pub fn LogButtons(g: *mut ImGuiContext)
    {
    // ImGuiContext& g = *GImGui;

    PushID("LogButtons");
// #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
//     const bool log_to_tty = Button("Log To TTY"); SameLine();
// #else
//     const bool log_to_tty = false;
// #endif
//     const bool log_to_file = Button("Log To File"); SameLine();
    let log_to_file = Button("Log To File");
        SameLine();
// const bool log_to_clipboard = Button("Log To Clipboard"); SameLine();
    let log_to_clipboard = Button("Log To Clipboard");
        SameLine();
    PushAllowKeyboardFocus(false);
    SetNextItemWidth(80.0);
    SliderInt("Default Depth", &g.LogDepthToExpandDefault, 0, 9, null());
    PopAllowKeyboardFocus();
    PopID();

    // Start logging at the end of the function so that the buttons don't appear in the log
    if (log_to_tty) {
        LogToTTY(0);
    }
    if (log_to_file) {
        LogToFile(0);
    }
    if (log_to_clipboard) {
        LogToClipboard(0);
    }
}


    if (suffix) {
        LogRenderedText(ref_pos, suffix, suffix + String::from(suffix).len());
    }
}
