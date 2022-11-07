#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiInputTextFlags;    // -> enum ImGuiInputTextFlags_  // Flags: for InputText(); InputTextMultiline()
pub type ImGuiInputTextFlags = c_int;

pub const ImGuiInputTextFlags_None: ImGuiInputTextFlags = 0;
pub const ImGuiInputTextFlags_CharsDecimal: ImGuiInputTextFlags = 1 << 0; // Allow 0123456789.+-*/
pub const ImGuiInputTextFlags_CharsHexadecimal: ImGuiInputTextFlags = 1 << 1; // Allow 0123456789ABCDEFabcdef
pub const ImGuiInputTextFlags_CharsUppercase: ImGuiInputTextFlags = 1 << 2; // Turn a..z into A..Z
pub const ImGuiInputTextFlags_CharsNoBlank: ImGuiInputTextFlags = 1 << 3; // Filter out spaces; tabs
pub const ImGuiInputTextFlags_AutoSelectAll: ImGuiInputTextFlags = 1 << 4; // Select entire text when first taking mouse focus
pub const ImGuiInputTextFlags_EnterReturnsTrue: ImGuiInputTextFlags = 1 << 5; // Return 'true' when Enter is pressed (as opposed to every time the value was modified). Consider looking at the IsItemDeactivatedAfterEdit() function.
pub const ImGuiInputTextFlags_CallbackCompletion: ImGuiInputTextFlags = 1 << 6; // Callback on pressing TAB (for completion handling)
pub const ImGuiInputTextFlags_CallbackHistory: ImGuiInputTextFlags = 1 << 7; // Callback on pressing Up/Down arrows (for history handling)
pub const ImGuiInputTextFlags_CallbackAlways: ImGuiInputTextFlags = 1 << 8; // Callback on each iteration. User code may query cursor position; modify text buffer.
pub const ImGuiInputTextFlags_CallbackCharFilter: ImGuiInputTextFlags = 1 << 9; // Callback on character inputs to replace or discard them. Modify 'EventChar' to replace or discard; or return 1 in callback to discard.
pub const ImGuiInputTextFlags_AllowTabInput: ImGuiInputTextFlags = 1 << 10; // Pressing TAB input a '\t' character into the text field
pub const ImGuiInputTextFlags_CtrlEnterForNewLine: ImGuiInputTextFlags = 1 << 11; // In multi-line mode; unfocus with Enter; add new line with Ctrl+Enter (default is opposite: unfocus with Ctrl+Enter; add line with Enter).
pub const ImGuiInputTextFlags_NoHorizontalScroll: ImGuiInputTextFlags = 1 << 12; // Disable following the cursor horizontally
pub const ImGuiInputTextFlags_AlwaysOverwrite: ImGuiInputTextFlags = 1 << 13; // Overwrite mode
pub const ImGuiInputTextFlags_ReadOnly: ImGuiInputTextFlags = 1 << 14; // Read-only mode
pub const ImGuiInputTextFlags_Password: ImGuiInputTextFlags = 1 << 15; // Password mode; display all characters as '*'
pub const ImGuiInputTextFlags_NoUndoRedo: ImGuiInputTextFlags = 1 << 16; // Disable undo/redo. Note that input text owns the text data while active; if you want to provide your own undo/redo stack you need e.g. to call ClearActiveID().
pub const ImGuiInputTextFlags_CharsScientific: ImGuiInputTextFlags = 1 << 17; // Allow 0123456789.+-*/eE (Scientific notation input)
pub const ImGuiInputTextFlags_CallbackResize: ImGuiInputTextFlags = 1 << 18; // Callback on buffer capacity changes request (beyond 'buf_size' parameter value); allowing the string to grow. Notify when the string wants to be resized (for string types which hold a cache of their Size). You will be provided a new BufSize in the callback and NEED to honor it. (see misc/cpp/imgui_stdlib.h for an example of using this)
pub const ImGuiInputTextFlags_CallbackEdit: ImGuiInputTextFlags = 1 << 19; // Callback on any edit (note that InputText() already returns true on edit; the callback is useful mainly to manipulate the underlying buffer while focus is active)

// Obsolete names (will be removed soon)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
pub const ImGuiInputTextFlags_AlwaysInsertMode: ImGuiInputTextFlags =
    ImGuiInputTextFlags_AlwaysOverwrite; // [renamed in 1.82] name was not matching behavior
                                         // #endif

// Extend ImGuiInputTextFlags_
// enum ImGuiInputTextFlagsPrivate_
// {
// [Internal]
pub const ImGuiInputTextFlags_Multiline: ImGuiInputTextFlags = 1 << 26; // For internal use by InputTextMultiline()
pub const ImGuiInputTextFlags_NoMarkEdited: ImGuiInputTextFlags = 1 << 27; // For internal use by functions using InputText() before reformatting data
pub const ImGuiInputTextFlags_MergedItem: ImGuiInputTextFlags = 1 << 28; // For internal use by TempInputText(), will skip calling ItemAdd(). Require bounding-box to strictly match.
                                                                         // };
