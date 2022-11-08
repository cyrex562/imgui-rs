use libc::{c_char, c_float, c_int, memcmp, memcpy, size_t};
use std::ptr::null_mut;
use std::env::args;
use crate::child_ops::{BeginChildEx, BeginChildFrame, EndChild, EndChildFrame};
use crate::color::{color_u32_from_rgba, IM_COL32_A_MASK, ImGuiCol_ChildBg, ImGuiCol_FrameBg, ImGuiCol_Header, ImGuiCol_HeaderActive, ImGuiCol_HeaderHovered, ImGuiCol_Text, ImGuiCol_TextDisabled, ImGuiCol_TextSelectedBg};
use crate::{button_ops, drag, GImGui, input_num_ops, layout_ops, radio_button, scrolling_ops, separator, stb, text_ops};
use crate::activate_flags::IM_GUI_ACTIVATE_FLAGS_TRY_TO_PRESERVE_STATE;
use crate::core::axis::IM_GUI_AXIS_Y;
use crate::backend_flags::IM_GUI_BACKEND_FLAGS_HAS_GAMEPAD;
use crate::widgets::button_flags::{ImGuiButtonFlags, ImGuiButtonFlags_AllowItemOverlap, ImGuiButtonFlags_NoHoldingActiveId, ImGuiButtonFlags_NoKeyModifiers, ImGuiButtonFlags_PressedOnClick, ImGuiButtonFlags_PressedOnClickRelease, ImGuiButtonFlags_PressedOnDoubleClick, ImGuiButtonFlags_PressedOnDragDropHold, ImGuiButtonFlags_PressedOnRelease};
use crate::clipboard_ops::{GetClipboardText, SetClipboardText};
use crate::color::color_edit_flags::{ImGuiColorEditFlags, ImGuiColorEditFlags_AlphaBar, ImGuiColorEditFlags_AlphaPreview, ImGuiColorEditFlags_AlphaPreviewHalf, ImGuiColorEditFlags_DataTypeMask_, ImGuiColorEditFlags_DefaultOptions_, ImGuiColorEditFlags_DisplayHex, ImGuiColorEditFlags_DisplayHSV, ImGuiColorEditFlags_DisplayMask_, ImGuiColorEditFlags_DisplayRGB, ImGuiColorEditFlags_Float, ImGuiColorEditFlags_HDR, ImGuiColorEditFlags_InputHSV, ImGuiColorEditFlags_InputMask_, ImGuiColorEditFlags_InputRGB, ImGuiColorEditFlags_NoAlpha, ImGuiColorEditFlags_NoBorder, ImGuiColorEditFlags_NoDragDrop, ImGuiColorEditFlags_NoInputs, ImGuiColorEditFlags_NoLabel, ImGuiColorEditFlags_NoOptions, ImGuiColorEditFlags_NoPicker, ImGuiColorEditFlags_NoSidePreview, ImGuiColorEditFlags_NoSmallPreview, ImGuiColorEditFlags_NoTooltip, ImGuiColorEditFlags_PickerHueBar, ImGuiColorEditFlags_PickerHueWheel, ImGuiColorEditFlags_PickerMask_, ImGuiColorEditFlags_Uint8};
use crate::color::color_ops::{ColorConvertFloat4ToU32, ColorConvertHSVtoRGB, ColorConvertRGBtoHSV};
use crate::core::condition::{ImGuiCond, ImGuiCond_Always, ImGuiCond_Once};
use crate::core::config_flags::ImGuiConfigFlags_NavEnableGamepad;
use crate::cursor_ops::{cursor_screen_pos, indent, set_cursor_screen_pos, unindent};
use crate::direction::{ImGuiDir_Down, ImGuiDir_Left, ImGuiDir_Right, ImGuiDir_Up};
use crate::drag_drop_ops::{AcceptDragDropPayload, BeginDragDropSource, BeginDragDropTarget, EndDragDropSource, EndDragDropTarget, SetDragDropPayload};
use crate::draw_flags::{ImDrawFlags_RoundCornersLeft, ImDrawFlags_RoundCornersRight};
use crate::draw_list::ImDrawList;
use crate::font_glyph::ImFontGlyph;
use crate::font_ops::{PopFont, PushFont};
use crate::frame_ops::GetFrameHeight;
use crate::geometry_ops::{ImTriangleBarycentricCoords, ImTriangleClosestPoint, ImTriangleContainsPoint};
use crate::group_ops::{BeginGroup, EndGroup};
use crate::id_ops::{ClearActiveID, GetIDWithSeed, pop_win_id_from_stack, PushOverrideID, SetActiveID};
use crate::input_ops::IsKeyPressed;
use crate::input_source::{ImGuiInputSource, ImGuiInputSource_Clipboard, ImGuiInputSource_Keyboard};
use crate::input_text_callback_data::ImGuiInputTextCallbackData;
use crate::input_text_flags::{ImGuiInputTextFlags, ImGuiInputTextFlags_AllowTabInput, ImGuiInputTextFlags_AlwaysOverwrite, ImGuiInputTextFlags_AutoSelectAll, ImGuiInputTextFlags_CallbackAlways, ImGuiInputTextFlags_CallbackCompletion, ImGuiInputTextFlags_CallbackEdit, ImGuiInputTextFlags_CallbackHistory, ImGuiInputTextFlags_CallbackResize, ImGuiInputTextFlags_CharsHexadecimal, ImGuiInputTextFlags_CharsUppercase, ImGuiInputTextFlags_CtrlEnterForNewLine, ImGuiInputTextFlags_EnterReturnsTrue, ImGuiInputTextFlags_MergedItem, ImGuiInputTextFlags_Multiline, ImGuiInputTextFlags_NoHorizontalScroll, ImGuiInputTextFlags_NoMarkEdited, ImGuiInputTextFlags_None, ImGuiInputTextFlags_NoUndoRedo, ImGuiInputTextFlags_Password, ImGuiInputTextFlags_ReadOnly};
use crate::input_text_state::ImGuiInputTextState;
use crate::item_flags::{ImGuiItemFlags, ImGuiItemFlags_Disabled, ImGuiItemFlags_Inputable, ImGuiItemFlags_NoNav, ImGuiItemFlags_NoNavDefaultFocus, ImGuiItemFlags_None, ImGuiItemFlags_NoTabStop, ImGuiItemFlags_SelectableDontClosePopup};
use crate::item_ops::{CalcItemSize, CalcItemWidth, IsItemActive, ItemAdd, ItemHoverable, ItemSize, MarkItemEdited, PopItemFlag, PopItemWidth, PushItemFlag, PushItemWidth, SetNextItemWidth};
use crate::item_status_flags::{ImGuiItemStatusFlags, ImGuiItemStatusFlags_FocusedByTabbing, ImGuiItemStatusFlags_HasDisplayRect, ImGuiItemStatusFlags_HoveredRect, ImGuiItemStatusFlags_HoveredWindow, ImGuiItemStatusFlags_Openable, ImGuiItemStatusFlags_Opened, ImGuiItemStatusFlags_ToggledOpen, ImGuiItemStatusFlags_ToggledSelection};
use crate::key::{ImGuiKey, ImGuiKey_A, ImGuiKey_Backspace, ImGuiKey_C, ImGuiKey_Delete, ImGuiKey_DownArrow, ImGuiKey_End, ImGuiKey_Enter, ImGuiKey_Escape, ImGuiKey_Home, ImGuiKey_Insert, ImGuiKey_KeypadEnter, ImGuiKey_LeftArrow, ImGuiKey_NavGamepadActivate, ImGuiKey_NavGamepadCancel, ImGuiKey_NavGamepadInput, ImGuiKey_None, ImGuiKey_PageDown, ImGuiKey_PageUp, ImGuiKey_RightArrow, ImGuiKey_Tab, ImGuiKey_UpArrow, ImGuiKey_V, ImGuiKey_X, ImGuiKey_Y, ImGuiKey_Z};
use crate::last_item_data::ImGuiLastItemData;
use crate::layout_ops::same_line;
use crate::logging_ops::LogSetNextTextDecoration;
use crate::math_ops::{char_is_blank, ImAtan2, ImClamp, ImCos, ImFmod, ImLerp, ImMax, ImMin, ImRotate, ImSin, ImSwap};
use crate::mod_flags::{ImGuiModFlags_Ctrl, ImGuiModFlags_Shift, ImGuiModFlags_Super};
use crate::mouse_cursor::ImGuiMouseCursor_TextInput;
use crate::nav_highlight_flags::{ImGuiNavHighlightFlags_NoRounding, ImGuiNavHighlightFlags_TypeThin};
use crate::nav_ops::{NavMoveRequestButNoResultYet, NavMoveRequestCancel, SetFocusID, SetNavID};
use crate::next_item_data_flags::ImGuiNextItemDataFlags_HasOpen;
use crate::popup_flags::ImGuiPopupFlags_MouseButtonRight;
use crate::popup_ops::{BeginPopup, CloseCurrentPopup, EndPopup, OpenPopup, OpenPopupOnItemClick};
use crate::rect::{ImRect, IsRectVisible};
use crate::render_ops::{FindRenderedTextEnd, RenderArrow, RenderArrowPointingAt, RenderBullet, RenderColorRectWithAlphaCheckerboard, RenderFrame, RenderFrameBorder, RenderNavHighlight, RenderText, RenderTextClipped};
use crate::scrolling_ops::{GetScrollMaxY, SetScrollY};
use crate::selectable_flags::{ImGuiSelectableFlags, ImGuiSelectableFlags_AllowDoubleClick, ImGuiSelectableFlags_AllowItemOverlap, ImGuiSelectableFlags_Disabled, ImGuiSelectableFlags_DontClosePopups, ImGuiSelectableFlags_DrawHoveredWhenHeld, ImGuiSelectableFlags_NoHoldingActiveID, ImGuiSelectableFlags_NoPadWithHalfSpacing, ImGuiSelectableFlags_SelectOnClick, ImGuiSelectableFlags_SelectOnNav, ImGuiSelectableFlags_SelectOnRelease, ImGuiSelectableFlags_SetNavIdOnHover, ImGuiSelectableFlags_SpanAllColumns, ImGuiSelectableFlags_SpanAvailWidth};
use crate::shade_verts_ops::ShadeVertsLinearColorGradientKeepAlpha;
use crate::stb::stb_textedit::{stb_text_createundo, stb_textedit_click, stb_textedit_cut, stb_textedit_drag, stb_textedit_initialize_state, stb_textedit_paste};
use crate::string_ops::{ImFormatString, ImFormatStringToTempBufferV, ImTextCharFromUtf8, ImTextCountCharsFromUtf8, ImTextCountUtf8BytesFromStr, ImTextStrFromUtf8, ImTextStrToUtf8};
use crate::style_ops::{GetColorU32, PopStyleColor, PushStyleColor};
use crate::style_var::{ImGuiStyleVar_ChildBorderSize, ImGuiStyleVar_ChildRounding, ImGuiStyleVar_WindowPadding};
use crate::tables::{PopColumnsBackground, PushColumnsBackground, TablePopBackgroundChannel, TablePushBackgroundChannel};
use crate::text_ops::{CalcTextSize, GetTextLineHeightWithSpacing};
use crate::tooltip_flags::ImGuiTooltipFlags_OverridePreviousTooltip;
use crate::tooltip_ops::{BeginTooltipEx, EndTooltip};
use crate::tree_node_flags::{ImGuiTreeNodeFlags, ImGuiTreeNodeFlags_AllowItemOverlap, ImGuiTreeNodeFlags_Bullet, ImGuiTreeNodeFlags_ClipLabelForTrailingButton, ImGuiTreeNodeFlags_CollapsingHeader, ImGuiTreeNodeFlags_DefaultOpen, ImGuiTreeNodeFlags_Framed, ImGuiTreeNodeFlags_FramePadding, ImGuiTreeNodeFlags_NavLeftJumpsBackHere, ImGuiTreeNodeFlags_NoAutoOpenOnLog, ImGuiTreeNodeFlags_None, ImGuiTreeNodeFlags_NoTreePushOnOpen, ImGuiTreeNodeFlags_OpenOnArrow, ImGuiTreeNodeFlags_OpenOnDoubleClick, ImGuiTreeNodeFlags_Selected, ImGuiTreeNodeFlags_SpanAvailWidth, ImGuiTreeNodeFlags_SpanFullWidth};
use crate::type_defs::{ImguiHandle, ImGuiInputTextCallback, ImWchar};
use crate::utils::{flag_clear, flag_set};
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::widgets::checkbox_ops;
use crate::window::focus::FocusWindow;
use crate::window::ImguiWindow;
use crate::window::ops::{BeginDisabled, EndDisabled, GetCurrentWindow};
use crate::window::props::{GetFontTexUvWhitePixel, SetNextWindowPos};
use crate::window::rect::window_rect_abs_to_rel;
use crate::window::window_flags::{ImGuiWindowFlags_NoMove, ImGuiWindowFlags_None, ImGuiWindowFlags_Popup};

// Create text input in place of another active widget (e.g. used when doing a CTRL+Click on drag/slider widgets)
// FIXME: Facilitate using this in variety of other situations.
pub unsafe fn TempInputText(bb: &mut ImRect,
                            id: ImguiHandle,
                            label: String,
                            buf: &mut String,
                            buf_size: usize,
                            flags: ImGuiInputTextFlags) -> bool
{
    // On the first frame, g.TempInputTextId == 0, then on subsequent frames it becomes == id.
    // We clear ActiveID on the first frame to allow the InputText() taking it back.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let init: bool = (g.TempInputId != id);
    if init {
        ClearActiveID(g); }

    g.Currentwindow.dc.cursor_pos = bb.min;
    let mut value_changed: bool =  InputTextEx(label, "", buf, buf_size, &mut bb.GetSize(), flags | ImGuiInputTextFlags_MergedItem, None, None);
    if init
    {
        // First frame we started displaying the InputText widget, we expect it to take the active id.
        // IM_ASSERT(g.ActiveId == id);
        g.TempInputId = g.ActiveId;
    }
    return value_changed;
}

pub unsafe fn InputTextMultiline(label: String, buf: &mut String, buf_size: size_t, size: &mut ImVec2, flags: ImGuiInputTextFlags, callback: Option<ImGuiInputTextCallback>, user_data: Option<&Vec<u8>>) -> bool
{
    return InputTextEx(label, "", buf, buf_size, size, flags | ImGuiInputTextFlags_Multiline, Some(callback), Some(user_data));
}

pub unsafe fn InputTextWithHint(label: String, hint: &str, buf: &mut String, buf_size: size_t, flags: ImGuiInputTextFlags, callback: ImGuiInputTextCallback, user_data: &Vec<u8>) -> bool
{
    // IM_ASSERT(flag_clear(flags, ImGuiInputTextFlags_Multiline)); // call InputTextMultiline() or  InputTextEx() manually if you need multi-line + hint.
    return InputTextEx(label, hint, buf, buf_size, ImVec2::new(0, 0), flags, Some(callback), Some(user_data));
}

pub fn InputTextCalcTextLenAndLineCount(text_begin: &String, out_text_end: &mut usize) -> usize
{
    // let line_count: c_int = 0;
    // let mut  s: &str = text_begin;
    // while ( c: c_char = *s++) // We are only matching for \n so we can ignore UTF-8 decoding
    //     if (c == '\n')
    //         line_count+= 1;
    // s-= 1;
    // if (s[0] != '\n' && s[0] != '\r') {
    //     line_count += 1;
    // }
    // *out_text_end = s;
    // return line_count;
    let mut line_count: usize = 0;
    let mut start: usize = 0;
    while start <= text_begin.len() {
        let next_line_end = text_begin[start..].find('\n');
        if next_line_end.is_none() {
            break;
        }
        start = next_line_end.unwrap();
    }

    return line_count;
}

pub unsafe fn InputTextCalcTextSizeW(
    text_begin: &String,
    remaining: &mut usize,
    mut out_offset: Option<&mut ImVec2>,
    stop_on_new_line: bool) -> ImVec2
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let font = g.Font;
    let line_height =  g.FontSize;
    let scale =  line_height / font.FontSize;

    let mut text_size = ImVec2::new(0, 0);
    let mut line_width: c_float =  0.0;
    let mut text_end = text_begin.len();

    let mut s= 0usize;
    while s < text_end
    {
        let mut c = text_begin[s];
        if c == '\n'
        {
            text_size.x = ImMax(text_size.x, line_width);
            text_size.y += line_height;
            line_width = 0.0;
            if stop_on_new_line {
                break(); }
            continue;
        }
        if c == '\r' {
            continue;
        }

        let char_width: c_float =  font.GetCharAdvance(c) * scale;
        line_width += char_width;
        s += 1;
    }

    if text_size.x < line_width{
        text_size.x = line_width;}

    if out_offset.is_some() {
        // offset allow for the possibility of sitting after a trailing
        // *out_offset. = ImVec2::new(line_width, text_size.y + line_height);
        let _ = out_offset.replace(&mut ImVec2::from_floats(line_width, text_size.y + line_height));
    }

    if line_width > 0.0 as c_float || text_size.y == 0.0 {
        // whereas size.y will ignore the trailing \n
        text_size.y += line_height;
    }

    if remaining {
        *remaining = s;
    }

    return text_size;
}

pub const STB_TEXTEDIT_NEWLINE: char = '\n';

// When ImGuiInputTextFlags_Password is set, we don't want actions such as CTRL+Arrow to leak the fact that underlying data are blanks or separators.
pub unsafe fn is_separator(c: char) -> bool {
    return char_is_blank(c) || c == ',' || c == ';' || c == '(' || c == ')' || c == '{' || c == '}' || c == '[' || c == ']' || c == '|' || c == '\n' || c == '\r';
}

pub unsafe fn is_word_boundary_from_right(obj: &mut ImGuiInputTextState, idx: usize) -> bool     {
    if obj.Flags & ImGuiInputTextFlags_Password { return  false; }
    return if idx > 0 { (is_separator(obj.TextW[idx - 1]) & & ! is_separator(obj.TextW[idx]) )} else {true};
}

pub unsafe fn is_word_boundary_from_left(obj: &mut ImGuiInputTextState, idx: usize) -> bool {
    if flag_set(obj.Flags , ImGuiInputTextFlags_Password) { return false; }
    return if idx > 0 {
        (!is_separator(obj.TextW[idx - 1]) && is_separator(obj.TextW[idx])) }
    else { true };
}


// Return false to discard a character.
pub unsafe fn InputTextFilterCharacter(p_char: char, flags: ImGuiInputTextFlags, callback: Option<ImGuiInputTextCallback>, user_data: Option<&Vec<u8>>, input_source: ImGuiInputSource) -> bool
{
    // IM_ASSERT(input_source == ImGuiInputSource_Keyboard || input_source == ImGuiInputSource_Clipboard);
    // let mut c = *p_char;
    //
    // // Filter non-printable (NB: isprint is unreliable! see #2467)
    // let mut apply_named_filters: bool =  true;
    // if (c < 0x20)
    // {
    //     let mut pass: bool =  false;
    //     pass |= (c == '\n' && (flags & ImGuiInputTextFlags_Multiline)); // Note that an Enter KEY will emit \r and be ignored (we poll for KEY in InputText() code)
    //     pass |= (c == '\t' && (flags & ImGuiInputTextFlags_AllowTabInput));
    //     if !pass { return  false; }
    //     apply_named_filters = false; // Override named filters below so newline and tabs can still be inserted.
    // }
    //
    // if (input_source != ImGuiInputSource_Clipboard)
    // {
    //     // We ignore Ascii representation of delete (emitted from Backspace on OSX, see #2578, #2817)
    //     if c == 127 { return  false; }
    //
    //     // Filter private Unicode range. GLFW on OSX seems to send private characters for special keys like arrow keys (FIXME)
    //     if c >= 0xE000 && c <= 0xF8F0f32 { return  false; }
    // }
    //
    // // Filter Unicode ranges we are not handling in this build
    // if c > IM_UNICODE_CODEPOINT_MAX { return  false; }
    //
    // // Generic named filters
    // if (apply_named_filters && (flags & (ImGuiInputTextFlags_CharsDecimal | ImGuiInputTextFlags_CharsHexadecimal | ImGuiInputTextFlags_CharsUppercase | ImGuiInputTextFlags_CharsNoBlank | ImGuiInputTextFlags_CharsScientific)))
    // {
    //     // The libc allows overriding locale, with e.g. 'setlocale(LC_NUMERIC, "de_DE.UTF-8");' which affect the output/input of printf/scanf to use e.g. ',' instead of '.'.
    //     // The standard mandate that programs starts in the "C" locale where the decimal point is '.'.
    //     // We don't really intend to provide widespread support for it, but out of empathy for people stuck with using odd API, we support the bare minimum aka overriding the decimal point.
    //     // Change the default decimal_point with:
    //     //   GetCurrentContext()->PlatformLocaleDecimalPoint = *localeconv()->decimal_point;
    //     // Users of non-default decimal point (in particular ',') may be affected by word-selection logic (is_word_boundary_from_right/is_word_boundary_from_left) functions.
    //     let g = GImGui; // ImGuiContext& g = *GImGui;
    //     const unsigned c_decimal_point = g.PlatformLocaleDecimalPoint;
    //
    //     // Full-width -> half-width conversion for numeric fields (https://en.wikipedia.org/wiki/Halfwidth_and_Fullwidth_Forms_(Unicode_block)
    //     // While this is mostly convenient, this has the side-effect for uninformed users accidentally inputting full-width characters that they may
    //     // scratch their head as to why it works in numerical fields vs in generic text fields it would require support in the font.
    //     if (flags & (ImGuiInputTextFlags_CharsDecimal | ImGuiInputTextFlags_CharsScientific | ImGuiInputTextFlags_CharsHexadecimal))
    //         if (c >= 0xFF01 && c <= 0xFF5E)
    //             c = c - 0xFF01 + 0x21;
    //
    //     // Allow 0-9 . - + * /
    //     if (flags & ImGuiInputTextFlags_CharsDecimal)
    //         if !(c >= '0' && c <= '9') && (c != c_decimal_point) && (c != '-') && (c != '+') && (c != '*') && (c != '/') { return  false; }
    //
    //     // Allow 0-9 . - + * / e E
    //     if (flags & ImGuiInputTextFlags_CharsScientific)
    //         if !(c >= '0' && c <= '9') && (c != c_decimal_point) && (c != '-') && (c != '+') && (c != '*') && (c != '/') && (c != 'e') && (c != 'E') { return  false; }
    //
    //     // Allow 0-9 a-F A-F
    //     if (flags & ImGuiInputTextFlags_CharsHexadecimal)
    //         if !(c >= '0' && c <= '9') && !(c >= 'a' && c <= 'f') && !(c >= 'A' && c <= 'F') { return  false; }
    //
    //     // Turn a-z into A-Z
    //     if (flags & ImGuiInputTextFlags_CharsUppercase)
    //         if (c >= 'a' && c <= 'z')
    //             c += ('A' - 'a');
    //
    //     if (flags & ImGuiInputTextFlags_CharsNoBlank)
    //         if ImCharIsBlankW(c) { return  false; }
    //
    //     *p_char = c;
    // }
    //
    // // Custom callback filter
    // if (flags & ImGuiInputTextFlags_CallbackCharFilter)
    // {
    //     callback_data: ImGuiInputTextCallbackData;
    //     memset(&callback_data, 0, sizeof(ImGuiInputTextCallbackData));
    //     callback_data.EventFlag = ImGuiInputTextFlags_CallbackCharFilter;
    //     callback_data.EventChar = c;
    //     callback_data.Flags = flags;
    //     callback_data.UserData = user_data;
    //     if callback(&callback_data) != 0 { return  false; }
    //     *p_char = callback_data.EventChar;
    //     if !callback_data.EventChar { return  false; }
    // }
    //
    // return true;
    // TODO
    todo!()
}

// Find the shortest single replacement we can make to get the new text from the old text.
// Important: needs to be run before TextW is rewritten with the new characters because calling STB_TEXTEDIT_GETCHAR() at the end.
// FIXME: Ideally we should transition toward (1) making InsertChars()/DeleteChars() update undo-stack (2) discourage (and keep reconcile) or obsolete (and remove reconcile) accessing buffer directly.
pub unsafe fn InputTextReconcileUndoStateAfterUserCallback(state: &mut ImGuiInputTextState, new_buf_a: &String, new_length_a: usize)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let old_buf: *const ImWchar = state.TextW.Data;
    let old_length: usize = state.CurLenW;
    let new_length: usize = ImTextCountCharsFromUtf8(new_buf_a);
    g.TempBuffer.reserve_discard((new_length + 1) * sizeof);
    let mut new_buf: String = String::from(g.TempBuffer.clone());
    ImTextStrFromUtf8(&mut new_buf, new_length + 1, new_buf_a);

    let shorter_length: usize = old_length.min(new_length);
    let mut first_diff: usize = 0;
    // for (first_diff = 0; first_diff < shorter_length; first_diff++)
    for first_diff in 0 .. shorter_length
    {
        if old_buf[first_diff] != new_buf[first_diff] {
            break;
        }
    }
    if first_diff == old_length && first_diff == new_length { return ; }

    let mut old_last_diff: usize = old_length - 1;
    let mut new_last_diff: usize = new_length - 1;
    // for (; old_last_diff >= first_diff && new_last_diff >= first_diff; old_last_diff--, new_last_diff--)
    while old_last_diff >= first_diff && new_last_diff >= first_diff
    {
        if old_buf[old_last_diff] != new_buf[new_last_diff] {
            break;
        }
        old_last_diff -= 1;
        new_last_diff -= 1;
    }

    let insert_len: usize = new_last_diff - first_diff + 1;
    let delete_len: usize = old_last_diff - first_diff + 1;
    if insert_len > 0 || delete_len > 0 {
        let p = stb_text_createundo(&mut state.Stb.undostate, first_diff, delete_len, insert_len);
        if p.is_null() == false {
            // for (let i: c_int = 0; i < delete_len; i+ +)
            for i in 0 .. delete_len
            {
                p[i] = ImStb::STB_TEXTEDIT_GETCHAR(state, first_diff + i);
            }
        }
}
}

// Edit a string of text
// - buf_size account for the zero-terminator, so a buf_size of 6 can hold "Hello" but not "Hello!".
//   This is so we can easily call InputText() on static arrays using ARRAYSIZE() and to match
//   Note that in std::string world, capacity() would omit 1 byte used by the zero-terminator.
// - When active, hold on a privately held copy of the text (and apply back to 'buf'). So changing 'buf' while the InputText is active has no effect.
// - If you want to use InputText() with std::string, see misc/cpp/imgui_stdlib.h
// (FIXME: Rather confusing and messy function, among the worse part of our codebase, expecting to rewrite a V2 at some point.. Partly because we are
//  doing UTF8 > U16 > UTF8 conversions on the go to easily interface with stb_textedit. Ideally should stay in UTF-8 all the time. See https://github.com/nothings/stb/issues/188)
pub unsafe fn InputTextEx(label: String,
                          hint: &str,
                          buf: &mut String,
                          mut buf_size: usize,
                          size_arg: &mut ImVec2,
                          flags: ImGuiInputTextFlags,
                          callback: Option<ImGuiInputTextCallback>,
                          callback_user_data: Option<&Vec<u8>>) -> bool
{
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items { return  false; }

    // IM_ASSERT(buf != NULL && buf_size >= 0);
    // IM_ASSERT(!(flag_set(flags, ImGuiInputTextFlags_CallbackHistory) && (flags & ImGuiInputTextFlags_Multiline)));        // Can't use both together (they both use up/down keys)
    // IM_ASSERT(!(flag_set(flags, ImGuiInputTextFlags_CallbackCompletion) && (flags & ImGuiInputTextFlags_AllowTabInput))); // Can't use both together (they both use tab key)

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let io = &mut g.IO;
    let setyle = &mut g.style;

    let RENDER_SELECTION_WHEN_INACTIVE: bool = false;
    let is_multiline: bool = flag_set(flags, ImGuiInputTextFlags_Multiline);
    let is_readonly: bool = flag_set(flags, ImGuiInputTextFlags_ReadOnly);
    let is_password: bool = flag_set(flags, ImGuiInputTextFlags_Password);
    let is_undoable: bool = flag_clear(flags, ImGuiInputTextFlags_NoUndoRedo);
    let is_resizable: bool = flag_set(flags, ImGuiInputTextFlags_CallbackResize);
    if is_resizable {}
        // IM_ASSERT(callback != NULL); // Must provide a callback if you set the ImGuiInputTextFlags_CallbackResize flag!

    if is_multiline { // Open group before calling GetID() because groups tracks id created within their scope (including the scrollbar)
        BeginGroup();
    }
    let mut id: ImguiHandle =  window.GetID(label);
    let label_size: ImVec2 = CalcTextSize(, label, true, 0.0);
    let frame_size: ImVec2 = CalcItemSize(g, size_arg, CalcItemWidth(g), (if is_multiline { g.FontSize * 8.0} else {label_size.y}) + style.FramePadding.y * 2.0); // Arbitrary default of 8 lines high for multi-line
    let total_size: ImVec2 = ImVec2::new(frame_size.x + (if label_size.x > 0.0 { style.ItemInnerSpacing.x + label_size.x} else {0.0}), frame_size.y);

    let mut frame_bb: ImRect = ImRect::new(window.dc.cursor_pos, window.dc.cursor_pos + frame_size);
    let mut total_bb: ImRect = ImRect::new(frame_bb.min, frame_bb.min + total_size);

    draw_window: &mut ImguiWindow = window;
    let mut inner_size: ImVec2 = frame_size;
    let mut item_status_flags: ImGuiItemStatusFlags =  0;
    let mut item_data_backup = ImGuiLastItemData::default();
    if is_multiline
    {
        let backup_pos: ImVec2 = window.dc.cursor_pos;
        ItemSize(g, &total_bb.GetSize(), style.FramePadding.y);
        if !ItemAdd(g, &mut total_bb, id, &frame_bb, ImGuiItemFlags_Inputable)
        {
            EndGroup();
            return false;
        }
        item_status_flags = g.last_item_data.StatusFlags;
        item_data_backup = g.last_item_data;
        window.dc.cursor_pos = backup_pos;

        // We reproduce the contents of BeginChildFrame() in order to provide 'label' so our window internal data are easier to read/debug.
        // FIXME-NAV: Pressing NavActivate will trigger general child activation right before triggering our own below. Harmless but bizarre.
        PushStyleColor(ImGuiCol_ChildBg, style.Colors[ImGuiCol_FrameBg]);
        PushStyleVar(ImGuiStyleVar_ChildRounding, style.FrameRounding);
        PushStyleVar(ImGuiStyleVar_ChildBorderSize, style.FrameBorderSize);
        PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2::new(0, 0)); // Ensure no clip rect so mouse hover can reach FramePadding edges
        let mut child_visible: bool =  BeginChildEx(label, id, &frame_bb.GetSize(), true, ImGuiWindowFlags_NoMove);
        PopStyleVar(3);
        PopStyleColor(0);
        if !child_visible
        {
            EndChild();
            EndGroup();
            return false;
        }
        draw_window = g.CurrentWindow; // Child window
        draw_window.DC.NavLayersActiveMaskNext |= (1 << draw_window.DC.NavLayerCurrent); // This is to ensure that EndChild() will display a navigation highlight so we can "enter" into it.
        draw_window.dc.cursor_pos += style.FramePadding;
        inner_size.x -= draw_window.scrollbarSizes.x;
    }
    else
    {
        // Support for internal ImGuiInputTextFlags_MergedItem flag, which could be redesigned as an ItemFlags if needed (with test performed in ItemAdd)
        ItemSize(g, &total_bb.GetSize(), style.FramePadding.y);
        if flag_clear(flags, ImGuiInputTextFlags_MergedItem) {
            if !ItemAdd(g, &mut total_bb, id, &frame_bb, ImGuiItemFlags_Inputable) { return false; }
        }
        item_status_flags = g.last_item_data.StatusFlags;
    }
    let hovered: bool = ItemHoverable(&frame_bb, id);
    if hovered{
        g.MouseCursor = ImGuiMouseCursor_TextInput;}

    // We are only allowed to access the state if we are already the active widget.
    state: &mut ImGuiInputTextState = GetInputTextState(id);

    let input_requested_by_tabbing: bool = (item_status_flags & ImGuiItemStatusFlags_FocusedByTabbing) != 0;
    let input_requested_by_nav: bool = (g.ActiveId != id) && ((g.NavActivateInputId == id) || (g.NavActivateId == id && g.NavInputSource == ImGuiInputSource_Keyboard));

    let user_clicked: bool = hovered && io.MouseClicked[0];
    let user_scroll_finish: bool = is_multiline && state != None && g.ActiveId == 0 && g.ActiveIdPreviousFrame == scrolling_ops::GetWindowScrollbarID(draw_window, IM_GUI_AXIS_Y);
    let user_scroll_active: bool = is_multiline && state != None && g.ActiveId == scrolling_ops::GetWindowScrollbarID(draw_window, IM_GUI_AXIS_Y);
    let mut clear_active_id: bool =  false;
    let mut select_all: bool =  false;

    let mut scroll_y: c_float =  if is_multiline { draw_window.scroll.y} else {f32::MAX};

    let init_changed_specs: bool = (state != None && state.Stb.single_line != !is_multiline);
    let init_make_active: bool = (user_clicked || user_scroll_finish || input_requested_by_nav || input_requested_by_tabbing);
    let init_state: bool = (init_make_active || user_scroll_active);
    if ((init_state && g.ActiveId != id) || init_changed_specs)
    {
        // Access state even if we don't own it yet.
        state = &g.InputTextState;
        state.CursorAnimReset();

        // Take a copy of the initial buffer value (both in original UTF-8 format and converted to wchar)
        // From the moment we focused we are ignoring the content of 'buf' (unless we are in read-only mode)
        let buf_len = buf.len();
        state.InitialTextA.resize(buf_len + 1);    // UTF-8. we use +1 to make sure that .Data is always pointing to at least an empty string.
        // TODO
        // memcpy(state.InitialTextA.Data, buf, buf_len + 1);

        // Preserve cursor position and undo/redo stack if we come back to same widget
        // FIXME: Since we reworked this on 2022/06, may want to differenciate recycle_cursor vs recycle_undostate?
        let mut recycle_state: bool =  (state.ID == id && !init_changed_specs);
        if recycle_state && (state.CurLenA != buf_len || (state.TextAIsValid && state.TextA != buf)) {
            recycle_state = false;}

        // Start edition
        let mut  buf_end = 0usize;
        state.ID = id;
        state.TextW.resize(buf_size + 1);          // wchar count <= UTF-8 count. we use +1 to make sure that .Data is always pointing to at least an empty string.
        state.TextA.clear();
        state.TextAIsValid = false;                // TextA is not valid yet (we will display buf until then)
        state.CurLenW = ImTextStrFromUtf8(state.TextW.Data, buf_size, buf);
        state.CurLenA = (buf_end - buf);      // We can't get the result from ImStrncpy() above because it is not UTF-8 aware. Here we'll cut off malformed UTF-8.

        if recycle_state
        {
            // Recycle existing cursor/selection/undo stack but clamp position
            // Note a single mouse click will override the cursor/position immediately by calling stb_textedit_click handler.
            state.CursorClamp();
        }
        else
        {
            state.ScrollX = 0.0;
            stb_textedit_initialize_state(&mut state.Stb, !is_multiline);
        }

        if !is_multiline
        {
            if flags & ImGuiInputTextFlags_AutoSelectAll {
                select_all = true;}
            if input_requested_by_nav && (!recycle_state || flag_clear(g.NavActivateFlags, IM_GUI_ACTIVATE_FLAGS_TRY_TO_PRESERVE_STATE)) {
                select_all = true;}
            if input_requested_by_tabbing || (user_clicked && io.KeyCtrl) {
                select_all = true;}
        }

        if flags & ImGuiInputTextFlags_AlwaysOverwrite{
            state.Stb.insert_mode = 1;} // stb field name is indeed incorrect (see #2863)
    }

    if (g.ActiveId != id && init_make_active)
    {
        // IM_ASSERT(state && state.ID == id);
        SetActiveID(g, id, window);
        SetFocusID(id, window);
        FocusWindow(window);

        // Declare our inputs
        g.ActiveIdUsingNavDirMask |= (1 << ImGuiDir_Left) | (1 << ImGuiDir_Right);
        if is_multiline || flag_set(flags, ImGuiInputTextFlags_CallbackHistory) {
            g.ActiveIdUsingNavDirMask |= (1 << ImGuiDir_Up) | (1 << ImGuiDir_Down);
        }
        SetActiveIdUsingKey(ImGuiKey_Escape);
        SetActiveIdUsingKey(ImGuiKey_NavGamepadCancel);
        SetActiveIdUsingKey(ImGuiKey_Home);
        SetActiveIdUsingKey(ImGuiKey_End);
        if is_multiline
        {
            SetActiveIdUsingKey(ImGuiKey_PageUp);
            SetActiveIdUsingKey(ImGuiKey_PageDown);
        }
        if flag_set(flags , (ImGuiInputTextFlags_CallbackCompletion | ImGuiInputTextFlags_AllowTabInput)) // Disable keyboard tabbing out as we will use the \t character.
        {
            SetActiveIdUsingKey(ImGuiKey_Tab);
        }
    }

    // We have an edge case if ActiveId was set through another widget (e.g. widget being swapped), clear id immediately (don't wait until the end of the function)
    if g.ActiveId == id && state == None{
        ClearActiveID(g);}

    // Release focus when we click outside
    if g.ActiveId == id && io.MouseClicked[0] && !init_state && !init_make_active { //-V560
        clear_active_id = true;
    }

    // Lock the decision of whether we are going to take the path displaying the cursor or selection
    let render_cursor: bool = (g.ActiveId == id) || (state && user_scroll_active);
    let mut render_selection: bool =  state && (state.HasSelection() || select_all) && (RENDER_SELECTION_WHEN_INACTIVE || render_cursor);
    let mut value_changed: bool =  false;
    let mut validated: bool =  false;

    // When read-only we always use the live data passed to the function
    // FIXME-OPT: Because our selection/cursor code currently needs the wide text we need to convert it when active, which is not ideal :(
    if is_readonly && state != None && (render_cursor || render_selection)
    {
        let mut  buf_end = 0usize;
        state.TextW.resize(buf_size + 1);
        state.CurLenW = ImTextStrFromUtf8(state.TextW, state.TextW.len(), buf);
        state.CurLenA = (buf_end - buf);
        state.CursorClamp();
        render_selection &= state.HasSelection();
    }

    // Select the buffer to render.
    let buf_display_from_state: bool = (render_cursor || render_selection || g.ActiveId == id) && !is_readonly && state && state.TextAIsValid;
    let is_displaying_hint: bool = (hint != None && (if buf_display_from_state { state.TextA.Data} else {buf})[0] == 0);

    // Password pushes a temporary font with only a fallback glyph
    if is_password && !is_displaying_hint
    {
        let glyph: *const ImFontGlyph = g.Font.FindGlyph('*');
        let mut password_font = &mut g.InputTextPasswordFont;
        password_font.FontSize = g.Font.FontSize;
        password_font.Scale = g.Font.Scale;
        password_font.Ascent = g.Font.Ascent;
        password_font.Descent = g.Font.Descent;
        password_font.ContainerAtlas = g.Font.ContainerAtlas;
        password_font.FallbackGlyph = glyph;
        password_font.FallbackAdvanceX = glyph.AdvanceX;
        // IM_ASSERT(password_font.Glyphs.empty() && password_font.IndexAdvanceX.empty() && password_font.IndexLookup.empty());
        PushFont(password_font);
    }

    // Process mouse inputs and character inputs
    let mut backup_current_text_length: usize = 0;
    if g.ActiveId == id
    {
        // IM_ASSERT(state != NULL);
        backup_current_text_length = state.CurLenA;
        state.Edited = false;
        state.BufCapacityA = buf_size;
        state.Flags = flags;

        // Although we are active we don't prevent mouse from hovering other elements unless we are interacting right now with the widget.
        // Down the line we should have a cleaner library-wide concept of Selected vs Active.
        g.ActiveIdAllowOverlap = !io.MouseDown[0];
        g.WantTextInputNextFrame = 1;

        // Edit in progress
        let mouse_x: c_float =  (io.MousePos.x - frame_bb.min.x - style.FramePadding.x) + state.ScrollX;
        let mouse_y: c_float =  (if is_multiline { (io.MousePos.y - draw_window.dc.cursor_pos.y) }else {g.FontSize * 0.5});

        let is_osx: bool = io.ConfigMacOSXBehaviors;
        if (select_all)
        {
            state.SelectAll();
            state.SelectedAllMouseLock = true;
        }
        else if hovered && io.MouseClickedCount[0] >= 2 && !io.KeyShift
        {
            stb_textedit_click(state, &mut state.Stb, mouse_x, mouse_y);
            let multiclick_count: usize = (io.MouseClickedCount[0] - 2);
            if (multiclick_count % 2) == 0
            {
                // Double-click: Select word
                // We always use the "Mac" word advance for double-click select vs CTRL+Right which use the platform dependent variant:
                // FIXME: There are likely many ways to improve this behavior, but there's no "right" behavior (depends on use-case, software, OS)
                let is_bol: bool = (state.Stb.cursor == 0) || ImStb::STB_TEXTEDIT_GETCHAR(state, state.Stb.cursor - 1) == '\n';
                if STB_TEXT_HAS_SELECTION(&state.Stb) || !is_bol {
                    state.OnKeyPressed(STB_TEXTEDIT_K_WORDLEFT);
                }
                //state->OnKeyPressed(STB_TEXTEDIT_K_WORDRIGHT | STB_TEXTEDIT_K_SHIFT);
                if !STB_TEXT_HAS_SELECTION(&state.Stb) {
                    ImStb::stb_textedit_prep_selection_at_cursor(&state.Stb);
                }
                state.Stb.cursor = ImStb::STB_TEXTEDIT_MOVEWORDRIGHT_MAC(state, state.Stb.cursor);
                state.Stb.select_end = state.Stb.cursor;
                ImStb::stb_textedit_clamp(state, &state.Stb);
            }
            else
            {
                // Triple-click: Select line
                let is_eol: bool = ImStb::STB_TEXTEDIT_GETCHAR(state, state.Stb.cursor) == '\n';
                state.OnKeyPressed(STB_TEXTEDIT_K_LINESTART);
                state.OnKeyPressed(STB_TEXTEDIT_K_LINEEND | STB_TEXTEDIT_K_SHIFT);
                state.OnKeyPressed(STB_TEXTEDIT_K_RIGHT | STB_TEXTEDIT_K_SHIFT);
                if (!is_eol && is_multiline)
                {
                    ImSwap(state.Stb.select_start, state.Stb.select_end);
                    state.Stb.cursor = state.Stb.select_end;
                }
                state.CursorFollow = false;
            }
            state.CursorAnimReset();
        }
        else if (io.MouseClicked[0] && !state.SelectedAllMouseLock)
        {
            if (hovered)
            {
                if (io.KeyShift) {
                    stb_textedit_drag(state, &mut state.Stb, mouse_x, mouse_y);
                }
                else {
                    stb_textedit_click(state, &mut state.Stb, mouse_x, mouse_y);
                }
                state.CursorAnimReset();
            }
        }
        else if io.MouseDown[0] && !state.SelectedAllMouseLock && (io.MouseDelta.x != 0.0 || io.MouseDelta.y != 0.0)
        {
            stb_textedit_drag(state, &mut state.Stb, mouse_x, mouse_y);
            state.CursorAnimReset();
            state.CursorFollow = true;
        }
        if state.SelectedAllMouseLock && !io.MouseDown[0] {
            state.SelectedAllMouseLock = false;
        }

        // We expect backends to emit a Tab key but some also emit a Tab character which we ignore (#2467, #1336)
        // (For Tab and Enter: Win32/SFML/Allegro are sending both keys and chars, GLFW and SDL are only sending keys. For Space they all send all threes)
        let ignore_char_inputs: bool = (io.KeyCtrl && !io.KeyAlt) || (is_osx && io.KeySuper);
        if flag_set(flags, ImGuiInputTextFlags_AllowTabInput) && IsKeyPressed(ImGuiKey_Tab, false) && !ignore_char_inputs && !io.KeyShift && !is_readonly
        {
            let mut c =  '\t'; // Insert TAB
            if InputTextFilterCharacter(c, flags, callback, callback_user_data, ImGuiInputSource_Keyboard) {
                state.OnKeyPressed(c);
            }
        }

        // Process regular text input (before we check for Return because using some IME will effectively send a Return?)
        // We ignore CTRL inputs, but need to allow ALT+CTRL as some keyboards (e.g. German) use AltGR (which _is_ Alt+Ctrl) to input certain characters.
        if io.InputQueueCharacters.Size > 0
        {
            if !ignore_char_inputs && !is_readonly && !input_requested_by_nav {
                // for (let n: c_int = 0; n < io.InputQueueCharacters.Size; n+ +)
                for n in 0 .. io.InputQueueCharacters.len()
                {
                    // Insert character if they pass filtering
                    let mut c = io.InputQueueCharacters[n];
                    if c == '\t' { // Skip Tab, see above.
                        continue;
                    }
                    if InputTextFilterCharacter(c, flags, callback, callback_user_data, ImGuiInputSource_Keyboard) {
                        state.OnKeyPressed(c);
                    }
                }
            }

            // Consume characters
            io.InputQueueCharacters.clear();
        }
    }

    // Process other shortcuts/key-presses
    let mut cancel_edit: bool =  false;
    if g.ActiveId == id && !g.ActiveIdIsJustActivated && !clear_active_id
    {
        // IM_ASSERT(state != NULL);

        let row_count_per_page: c_int = ImMax(((inner_size.y - style.FramePadding.y) / g.FontSize), 1);
        state.Stb.row_count_per_page = row_count_per_page;

        let k_mask: c_int = (if io.KeyShift { STB_TEXTEDIT_K_SHIFT }else {0});
        let is_osx: bool = io.ConfigMacOSXBehaviors;
        let is_osx_shift_shortcut: bool = is_osx && (io.KeyMods == (ImGuiModFlags_Super | ImGuiModFlags_Shift));
        let is_wordmove_key_down: bool = if is_osx { io.KeyAlt }else {io.KeyCtrl};                     // OS X style: Text editing cursor movement using Alt instead of Ctrl
        let is_startend_key_down: bool = is_osx && io.KeySuper && !io.KeyCtrl && !io.KeyAlt;  // OS X style: Line/Text Start and End using Cmd+Arrows instead of Home/End
        let is_ctrl_key_only: bool = (io.KeyMods == ImGuiModFlags_Ctrl);
        let is_shift_key_only: bool = (io.KeyMods == ImGuiModFlags_Shift);
        let is_shortcut_key: bool = if g.IO.ConfigMacOSXBehaviors { (io.KeyMods == ImGuiModFlags_Super) }else{ (io.KeyMods == ImGuiModFlags_Ctrl)};

        let is_cut: bool = ((is_shortcut_key && IsKeyPressed(ImGuiKey_X, false)) || (is_shift_key_only && IsKeyPressed(ImGuiKey_Delete, false))) && !is_readonly && !is_password && (!is_multiline || state.HasSelection());
        let is_copy: bool = ((is_shortcut_key && IsKeyPressed(ImGuiKey_C, false)) || (is_ctrl_key_only  && IsKeyPressed(ImGuiKey_Insert, false))) && !is_password && (!is_multiline || state.HasSelection());
        let is_paste: bool = ((is_shortcut_key && IsKeyPressed(ImGuiKey_V, false)) || (is_shift_key_only && IsKeyPressed(ImGuiKey_Insert, false))) && !is_readonly;
        let is_undo: bool = ((is_shortcut_key && IsKeyPressed(ImGuiKey_Z, false)) && !is_readonly && is_undoable);
        let is_redo: bool = ((is_shortcut_key && IsKeyPressed(ImGuiKey_Y, false)) || (is_osx_shift_shortcut && IsKeyPressed(ImGuiKey_Z, false))) && !is_readonly && is_undoable;
        let is_select_all: bool = is_shortcut_key && IsKeyPressed(ImGuiKey_A, false);

        // We allow validate/cancel with Nav source (gamepad) to makes it easier to undo an accidental NavInput press with no keyboard wired, but otherwise it isn't very useful.
        let nav_gamepad_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (io.BackendFlags & IM_GUI_BACKEND_FLAGS_HAS_GAMEPAD) != 0;
        let is_enter_pressed: bool = IsKeyPressed(ImGuiKey_Enter, true) || IsKeyPressed(ImGuiKey_KeypadEnter, true);
        let is_gamepad_validate: bool = nav_gamepad_active && (IsKeyPressed(ImGuiKey_NavGamepadActivate, false) || IsKeyPressed(ImGuiKey_NavGamepadInput, false));
        let is_cancel: bool = IsKeyPressed(ImGuiKey_Escape, false) || (nav_gamepad_active && IsKeyPressed(ImGuiKey_NavGamepadCancel, false));

        if IsKeyPressed(ImGuiKey_LeftArrow, false) { state.OnKeyPressed((if is_startend_key_down { STB_TEXTEDIT_K_LINESTART} else { if is_wordmove_key_down { STB_TEXTEDIT_K_WORDLEFT}else {STB_TEXTEDIT_K_LEFT}}) | k_mask); }
        else if IsKeyPressed(ImGuiKey_RightArrow, false) { state.OnKeyPressed((if is_startend_key_down { STB_TEXTEDIT_K_LINEEND} else { if is_wordmove_key_down { STB_TEXTEDIT_K_WORDRIGHT}else {STB_TEXTEDIT_K_RIGHT}}) | k_mask); }
        else if IsKeyPressed(ImGuiKey_UpArrow, false) && is_multiline { if io.KeyCtrl {
            SetScrollY(draw_window, ImMax(draw_window.scroll.y - g.FontSize, 0.0));
        } else {state.OnKeyPressed((if is_startend_key_down {STB_TEXTEDIT_K_TEXTSTART} else { STB_TEXTEDIT_K_UP }) | k_mask); }
         if IsKeyPressed(ImGuiKey_DownArrow, false) && is_multiline { if io.KeyCtrl { SetScrollY(draw_window, ImMin(draw_window.scroll.y + g.FontSize, GetScrollMaxY() as c_int)); } else { state.OnKeyPressed((if is_startend_key_down { STB_TEXTEDIT_K_TEXTEND } else { STB_TEXTEDIT_K_DOWN }) | k_mask); }}
        else if IsKeyPressed(ImGuiKey_PageUp, false) && is_multiline { state.OnKeyPressed(STB_TEXTEDIT_K_PGUP | k_mask); scroll_y -= row_count_per_page * g.FontSize; }
        else if IsKeyPressed(ImGuiKey_PageDown, false) && is_multiline { state.OnKeyPressed(STB_TEXTEDIT_K_PGDOWN | k_mask); scroll_y += row_count_per_page * g.FontSize; }
        else if IsKeyPressed(ImGuiKey_Home, false) { state.OnKeyPressed(if io.KeyCtrl { STB_TEXTEDIT_K_TEXTSTART | k_mask} else {STB_TEXTEDIT_K_LINESTART | k_mask}); }
        else if IsKeyPressed(ImGuiKey_End, false) { state.OnKeyPressed(if io.KeyCtrl { STB_TEXTEDIT_K_TEXTEND | k_mask} else {STB_TEXTEDIT_K_LINEEND | k_mask}); }
        else if IsKeyPressed(ImGuiKey_Delete, false) && !is_readonly && !is_cut { state.OnKeyPressed(STB_TEXTEDIT_K_DELETE | k_mask); }
        else if IsKeyPressed(ImGuiKey_Backspace, false) && !is_readonly
        {
            if !state.HasSelection()
            {
                if is_wordmove_key_down {
                    state.OnKeyPressed(STB_TEXTEDIT_K_WORDLEFT | STB_TEXTEDIT_K_SHIFT);
                }
                else if is_osx && io.KeySuper && !io.KeyAlt && !io.KeyCtrl {
                    state.OnKeyPressed(STB_TEXTEDIT_K_LINESTART | STB_TEXTEDIT_K_SHIFT);
                }
            }
            state.OnKeyPressed(STB_TEXTEDIT_K_BACKSPACE | k_mask);
        }
        else if is_enter_pressed || is_gamepad_validate
        {
            // Determine if we turn Enter into a \n character
            let mut ctrl_enter_for_new_line: bool =  flag_set(flags, ImGuiInputTextFlags_CtrlEnterForNewLine);
            if !is_multiline || is_gamepad_validate || (ctrl_enter_for_new_line && !io.KeyCtrl) || (!ctrl_enter_for_new_line && io.KeyCtrl)
            {
                validated = true;
                if io.ConfigInputTextEnterKeepActive && !is_multiline {
                    state.SelectAll();
                } // No need to scroll
                else {
                    clear_active_id = true;
                }
            }
            else if !is_readonly
            {
                let mut c =  '\n'; // Insert new line
                if InputTextFilterCharacter(c, flags, callback, callback_user_data, ImGuiInputSource_Keyboard) {
                    state.OnKeyPressed(c);
                }
            }
        }
        else if is_cancel
        {
            clear_active_id = true;
            cancel_edit = true;
        }
        else if is_undo || is_redo
        {
            state.OnKeyPressed(if is_undo {STB_TEXTEDIT_K_UNDO} else { STB_TEXTEDIT_K_REDO });
            state.ClearSelection();
        }
        else if is_select_all
        {
            state.SelectAll();
            state.CursorFollow = true;
        }
        else if is_cut || is_copy
        {
            // Cut, Copy
            if io.SetClipboardTextFn
            {
                let ib: c_int = if state.HasSelection() { ImMin(state.Stb.select_start, state.Stb.select_end)} else {0};
                let ie: c_int = if state.HasSelection() { ImMax(state.Stb.select_start, state.Stb.select_end)} else{ state.CurLenW};
                let clipboard_data_len: usize = (ImTextCountUtf8BytesFromStr(state.TextW.Data + ib) + 1);
                let mut clipboard_data = String::new();
                ImTextStrToUtf8(&mut clipboard_data, clipboard_data_len, state.TextW.Data + ib);
                SetClipboardText(&clipboard_data);
                MemFree(clipboard_data);
            }
            if is_cut
            {
                if !state.HasSelection() {
                    state.SelectAll();
                }
                state.CursorFollow = true;
                stb_textedit_cut(state, &mut state.Stb);
            }
        }
        else if (is_paste)
        {
            let clipboard = GetClipboardText();
            if clipboard.is_empty() == false
            {
                // Filter pasted buffer
                let clipboard_len = clipboard.len();
                let mut clipboard_filtered = String::with_capacity(clipboard_len);
                let mut clipboard_filtered_len: usize = 0;
                // for (s: &str = clipboard; *s; )
                for s in clipboard
                {
                    let mut c = '\0';
                    s += ImTextCharFromUtf8(&mut c, s);
                    if c == 0 {
                        break(); }
                    if !InputTextFilterCharacter(c, flags, callback, callback_user_data, ImGuiInputSource_Clipboard) {
                        continue;
                    }
                    clipboard_filtered[clipboard_filtered_len] = c;
                    clipboard_filtered_len += 1;
                }
                clipboard_filtered[clipboard_filtered_len] = 0;
                if clipboard_filtered_len > 0 // If everything was filtered, ignore the pasting operation
                {
                    stb_textedit_paste(state, &mut state.Stb, &mut clipboard_filtered, clipboard_filtered_len);
                    state.CursorFollow = true;
                }
                MemFree(clipboard_filtered);
            }
        }

        // Update render selection flag after events have been handled, so selection highlight can be displayed during the same frame.
        render_selection |= state.HasSelection() && (RENDER_SELECTION_WHEN_INACTIVE || render_cursor);
    }

    // Process callbacks and apply result back to user's buffer.
    let mut  apply_new_text = String::default();
    let mut apply_new_text_length: usize = 0;
    if g.ActiveId == id
    {
        // IM_ASSERT(state != NULL);
        if cancel_edit
        {
            // Restore initial value. Only return true if restoring to the initial value changes the current buffer contents.
            if !is_readonly && buf != state.InitialTextA
            {
                // Push records into the undo stack so we can CTRL+Z the revert operation itself
                apply_new_text = state.InitialTextA.Data;
                apply_new_text_length = state.InitialTextA.Size - 1;
                let mut w_text: Vec<char> = vec![];
                if apply_new_text_length > 0
                {
                    w_text.resize(ImTextCountCharsFromUtf8(apply_new_text.as_str()) + 1, '\0');
                    ImTextStrFromUtf8(w_text.Data, w_text.Size, &apply_new_text);
                }
                stb::stb_textedit_replace(state, &mut state.Stb, w_text.Data, if (apply_new_text_length > 0) { (w_text.Size - 1)} else{ 0});
            }
        }

        // Apply ASCII value
        if (!is_readonly)
        {
            state.TextAIsValid = true;
            state.TextA.resize(state.TextW.Size * 4 + 1);
            ImTextStrToUtf8(state.TextA.Data, state.TextA.Size, state.TextW.Data);
        }

        // When using 'ImGuiInputTextFlags_EnterReturnsTrue' as a special case we reapply the live buffer back to the input buffer before clearing ActiveId, even though strictly speaking it wasn't modified on this frame.
        // If we didn't do that, code like InputInt() with ImGuiInputTextFlags_EnterReturnsTrue would fail.
        // This also allows the user to use InputText() with ImGuiInputTextFlags_EnterReturnsTrue without maintaining any user-side storage (please note that if you use this property along ImGuiInputTextFlags_CallbackResize you can end up with your temporary string object unnecessarily allocating once a frame, either store your string data, either if you don't then don't use ImGuiInputTextFlags_CallbackResize).
        let apply_edit_back_to_user_buffer: bool = !cancel_edit || (validated && flag_set(flags, ImGuiInputTextFlags_EnterReturnsTrue));
        if apply_edit_back_to_user_buffer
        {
            // Apply new value immediately - copy modified buffer back
            // Note that as soon as the input box is active, the in-widget value gets priority over any underlying modification of the input buffer
            // FIXME: We actually always render 'buf' when calling DrawList.AddText, making the comment above incorrect.
            // FIXME-OPT: CPU waste to do this every time the widget is active, should mark dirty state from the stb_textedit callbacks.

            // User callback
            if flag_set(flags , (ImGuiInputTextFlags_CallbackCompletion | ImGuiInputTextFlags_CallbackHistory | ImGuiInputTextFlags_CallbackEdit | ImGuiInputTextFlags_CallbackAlways))
            {
                // IM_ASSERT(callback != NULL);

                // The reason we specify the usage semantic (Completion/History) is that Completion needs to disable keyboard TABBING at the moment.
                event_flag: ImGuiInputTextFlags = 0;
                let mut event_key: ImGuiKey =  ImGuiKey_None;
                if flag_set(flags, ImGuiInputTextFlags_CallbackCompletion) && IsKeyPressed(ImGuiKey_Tab, false)
                {
                    event_flag = ImGuiInputTextFlags_CallbackCompletion;
                    event_key = ImGuiKey_Tab;
                }
                else if flag_set(flags, ImGuiInputTextFlags_CallbackHistory) && IsKeyPressed(ImGuiKey_UpArrow, false)
                {
                    event_flag = ImGuiInputTextFlags_CallbackHistory;
                    event_key = ImGuiKey_UpArrow;
                }
                else if flag_set(flags, ImGuiInputTextFlags_CallbackHistory) && IsKeyPressed(ImGuiKey_DownArrow, false)
                {
                    event_flag = ImGuiInputTextFlags_CallbackHistory;
                    event_key = ImGuiKey_DownArrow;
                }
                else if flag_set(flags, ImGuiInputTextFlags_CallbackEdit) && state.Edited
                {
                    event_flag = ImGuiInputTextFlags_CallbackEdit;
                }
                else if flag_set(flags , ImGuiInputTextFlags_CallbackAlways)
                {
                    event_flag = ImGuiInputTextFlags_CallbackAlways;
                }

                if event_flag
                {
                    let mut callback_data: ImGuiInputTextCallbackData = ImGuiInputTextCallbackData::default();
                    // memset(&callback_data, 0, sizeof(ImGuiInputTextCallbackData));
                    callback_data.EventFlag = event_flag;
                    callback_data.Flags = flags;
                    callback_data.UserData = callback_user_data.unwrap_or(&vec![]).clone();

                    let mut callback_buf: String = if is_readonly { buf.clone()} else {state.TextA};
                    callback_data.EventKey = event_key;
                    callback_data.Buf = callback_buf;
                    callback_data.BufTextLen = state.CurLenA;
                    callback_data.BufSize = state.BufCapacityA;
                    callback_data.BufDirty = false;

                    // We have to convert from wchar-positions to UTF-8-positions, which can be pretty slow (an incentive to ditch the ImWchar buffer, see https://github.com/nothings/stb/issues/188)
                    let mut text = state.TextW.clone();
                    callback_data.CursorPos = ImTextCountUtf8BytesFromStr(text);
                    let utf8_cursor_pos = callback_data.CursorPos;
                    callback_data.SelectionStart = ImTextCountUtf8BytesFromStr(text);
                    let utf8_selection_start = callback_data.SelectionStart;
                    callback_data.SelectionEnd = ImTextCountUtf8BytesFromStr(text);
                    let utf8_selection_end = callback_data.SelectionEnd;

                    // Call user code
                    callback.unwrap()(&mut callback_data);

                    // Read back what user may have modified
                    callback_buf = if is_readonly { buf.clone() }else {state.TextA}; // Pointer may have been invalidated by a resize callback
                    // IM_ASSERT(callback_data.Buf == callback_bu0f32);         // Invalid to modify those fields
                    // IM_ASSERT(callback_data.BufSize == state->BufCapacityA);
                    // IM_ASSERT(callback_data.Flags == flags);
                    let buf_dirty: bool = callback_data.BufDirty;
                    if callback_data.CursorPos != utf8_cursor_pos || buf_dirty { state.Stb.cursor = ImTextCountCharsFromUtf8(callback_data.Buf.as_str()); state.CursorFollow = true; }
                    if callback_data.SelectionStart != utf8_selection_start || buf_dirty {
                        state.Stb.select_start = if callback_data.SelectionStart == callback_data.CursorPos { state.Stb.cursor} else { ImTextCountCharsFromUtf8(callback_data.Buf.as_str())}; }
                    if callback_data.SelectionEnd != utf8_selection_end || buf_dirty { state.Stb.select_end = if callback_data.SelectionEnd == callback_data.SelectionStart { state.Stb.select_start} else { ImTextCountCharsFromUtf8(callback_data.Buf.as_str())}; }
                    if buf_dirty
                    {
                        // IM_ASSERT(flag_set(flags, ImGuiInputTextFlags_ReadOnly) == 0);
                        // IM_ASSERT(callback_data.BufTextLen == strlen(callback_data.Bu0f32)); // You need to maintain BufTextLen if you change the text!
                        InputTextReconcileUndoStateAfterUserCallback(state, &callback_data.Buf, callback_data.BufTextLen); // FIXME: Move the rest of this block inside function and rename to InputTextReconcileStateAfterUserCallback() ?
                        if callback_data.BufTextLen > backup_current_text_length && is_resizable {
                            state.TextW.resize(state.TextW.Size + (callback_data.BufTextLen - backup_current_text_length));
                        } // Worse case scenario resize
                        state.CurLenW = ImTextStrFromUtf8(state.TextW.Data, state.TextW.Size, &callback_data.Buf);
                        state.CurLenA = callback_data.BufTextLen;  // Assume correct length and valid UTF-8 from user, saves us an extra strlen()
                        state.CursorAnimReset();
                    }
                }
            }

            // Will copy result string if modified
            if !is_readonly && state.TextA != buf
            {
                apply_new_text = state.TextA.Data;
                apply_new_text_length = state.CurLenA;
            }
        }

        // Clear temporary user storage
        state.Flags = ImGuiInputTextFlags_None;
    }

    // Copy result to user buffer. This can currently only happen when (g.ActiveId == id)
    if apply_new_text != None
    {
        // We cannot test for 'backup_current_text_length != apply_new_text_length' here because we have no guarantee that the size
        // of our owned buffer matches the size of the string object held by the user, and by design we allow InputText() to be used
        // without any storage on user's side.
        // IM_ASSERT(apply_new_text_length >= 0);
        if is_resizable
        {
            callback_data: ImGuiInputTextCallbackData;
            callback_data.EventFlag = ImGuiInputTextFlags_CallbackResize;
            callback_data.Flags = flags;
            callback_data.Buf = buf;
            callback_data.BufTextLen = apply_new_text_length;
            callback_data.BufSize = ImMax(buf_size, apply_new_text_length + 1);
            callback_data.UserData = callback_user_data;
            callback.unwrap()(&mut callback_data);
            *buf = callback_data.Buf;
            buf_size = callback_data.BufSize;
            apply_new_text_length = callback_data.BufTextLen.min(buf_size - 1);
            // IM_ASSERT(apply_new_text_length <= buf_size);
        }
        //IMGUI_DEBUG_PRINT("InputText(\"{}\"): apply_new_text length {}\n", label, apply_new_text_length);

        // If the underlying buffer resize was denied or not carried to the next frame, apply_new_text_length+1 may be >= buf_size.
        // ImStrncpy(buf, apply_new_text, ImMin(apply_new_text_length + 1, buf_size));
        *buf = apply_new_text;
        value_changed = true;
    }

    // Release active ID at the end of the function (so e.g. pressing Return still does a final application of the value)
    if clear_active_id && g.ActiveId == id{
        ClearActiveID(g);}

    // Render frame
    if !is_multiline
    {
        RenderNavHighlight(, &frame_bb, id, 0);
        RenderFrame(frame_bb.min, frame_bb.max, GetColorU32(ImGuiCol_FrameBg, 0.0), true, style.FrameRounding);
    }

    let mut clip_rect = ImVec4(frame_bb.min.x, frame_bb.min.y, frame_bb.min.x + inner_size.x, frame_bb.min.y + inner_size.y); // Not using frame_bb.Max because we have adjusted size
    let mut draw_pos: ImVec2 = if is_multiline { draw_window.dc.cursor_pos} else {frame_bb.min + style.FramePadding};
    let mut text_size = ImVec2::from_floats(0.0, 0.0);

    // Set upper limit of single-line InputTextEx() at 2 million characters strings. The current pathological worst case is a long line
    // without any carriage return, which would makes ImFont::RenderText() reserve too many vertices and probably crash. Avoid it altogether.
    // Note that we only use this limit on single-line InputText(), so a pathologically large line on a InputTextMultiline() would still crash.
    let buf_display_max_length: c_int = 2 * 1024 * 1024;
    let mut  buf_display: String = if buf_display_from_state { state.TextA.clone()} else {buf.clone()}; //-V595
    let mut  buf_display_end: usize = 0; // We have specialized paths below for setting the length
    if is_displaying_hint
    {
        buf_display = String::from(hint);
        buf_display_end = buf_display.len();
    }

    // Render text. We currently only render selection when the widget is active or while scrolling.
    // FIXME: We could remove the '&& render_cursor' to keep rendering selection when inactive.
    if render_cursor || render_selection
    {
        // IM_ASSERT(state != NULL);
        if !is_displaying_hint {
            // buf_display_end = buf_display + state.CurLenA;
        }

        // Render text (with cursor and selection)
        // This is going to be messy. We need to:
        // - Display the text (this alone can be more easily clipped)
        // - Handle scrolling, highlight selection, display cursor (those all requires some form of 1d->2d cursor position calculation)
        // - Measure text height (for scrollbar)
        // We are attempting to do most of that in **one main pass** to minimize the computation cost (non-negligible for large amount of text) + 2nd pass for selection rendering (we could merge them by an extra refactoring effort)
        // FIXME: This should occur on buf_display but we'd need to maintain cursor/select_start/select_end for UTF-8.
        let text_begin: String = state.TextW.clone();
        // cursor_offset: ImVec2, select_start_offset;
        let mut cursor_offset = ImVec2::default();
        let mut select_start_offset = ImVec2::default();

        {
            // Find lines numbers straddling 'cursor' (slot 0) and 'select_start' (slot 1) positions.
            let mut searches_input_ptr: [char;2] = [ '\0', '\0' ];
            let mut searches_result_line_no: [usize;2] = [ usize::MIN, usize::MIN ];
            let mut searches_remaining: usize = 0;
            if render_sdcursor
            {
                searches_input_ptr[0] = text_begin[state.Stb.cursor];
                searches_result_line_no[0] = -1;
                searches_remaining+= 1;
            }
            if render_selection
            {
                searches_input_ptr[1] = text_begin[state.Stb.select_start.min(state.Stb.select_start, state.Stb.select_end)];
                searches_result_line_no[1] = -1;
                searches_remaining+= 1;
            }

            // Iterate all lines to find our line numbers
            // In multi-line mode, we never exit the loop until all lines are counted, so add one extra to the searches_remaining counter.
            searches_remaining += if is_multiline {1} else { 0 };
            let mut line_count: usize = 0;
            //for (const s: *mut ImWchar = text_begin; (s = (const ImWchar*)wcschr((const wchar_t*)s, (wchar_t)'\n')) != None; s++)  // FIXME-OPT: Could use this when wchar_t are 16-bit
            // for (*let s: ImWchar = text_begin; *s != 0; s++)
            for s in text_begin {
                if s == '\n'
                {
                    line_count+= 1;
                    if searches_result_line_no[0] == -1 && s >= searches_input_ptr[0] {
                        searches_result_line_no[0] = line_count;
                        if --searches_remaining <= 0 { break; } }
                    if searches_result_line_no[1] == -1 && s >= searches_input_ptr[1] { searches_result_line_no[1] = line_count; if --searches_remaining <= 0 { break; } }
                }
            line_count+= 1;
            if searches_result_line_no[0] == -1 {
                searches_result_line_no[0] = line_count;
            }
            if searches_result_line_no[1] == -1 {
                searches_result_line_no[1] = line_count;
            }

            // Calculate 2d position by finding the beginning of the line and measuring distance
            // TODO:
                // cursor_offset.x = InputTextCalcTextSizeW(ImStrbolW(searches_input_ptr[0], &text_begin), searches_input_ptr[0], None, false).x;
            cursor_offset.y = searches_result_line_no[0] * g.FontSize;
            if searches_result_line_no[1] >= 0
            {
             // TODO
                // select_start_offset.x = InputTextCalcTextSizeW(ImStrbolW(searches_input_ptr[1], &text_begin), searches_input_ptr[1], None, false).x;
                select_start_offset.y = searches_result_line_no[1] * g.FontSize;
            }

            // Store text height (note that we haven't calculated text width at all, see GitHub issues #383, #1224)
            if is_multiline {
                text_size = ImVec2::new(inner_size.x, line_count * g.FontSize);
            }
        }

        // Scroll
        if render_cursor && state.CursorFollow
        {
            // Horizontal scroll in chunks of quarter width
            if flag_clear(flags, ImGuiInputTextFlags_NoHorizontalScroll)
            {
                let scroll_increment_x: c_float =  inner_size.x * 0.25f32;
                let visible_width: c_float =  inner_size.x - style.FramePadding.x;
                if cursor_offset.x < state.ScrollX {
                    state.ScrollX = IM_FLOOR(ImMax(0.0, cursor_offset.x - scroll_increment_x));
                }
                else if cursor_offset.x - visible_width >= state.ScrollX {
                    state.ScrollX = IM_FLOOR(cursor_offset.x - visible_width + scroll_increment_x);
                }
            }
            else
            {
                state.ScrollX = 0.0;
            }

            // Vertical scroll
            if is_multiline
            {
                // Test if cursor is vertically visible
                if cursor_offset.y - g.FontSize < scroll_y {
                    scroll_y = ImMax(0.0, cursor_offset.y - g.FontSize);
                }
                else if cursor_offset.y - (inner_size.y - style.FramePadding.y * 2.0) >= scroll_y {
                    scroll_y = cursor_offset.y - inner_size.y + style.FramePadding.y * 2.0;
                }
                let scroll_max_y: c_float =  ImMax((text_size.y + style.FramePadding.y * 2.0) - inner_size.y, 0.0);
                scroll_y = ImClamp(scroll_y, 0.0, scroll_max_y);
                draw_pos.y += (draw_window.scroll.y - scroll_y);   // Manipulate cursor pos immediately avoid a frame of lag
                draw_window.scroll.y = scroll_y;
            }

            state.CursorFollow = false;
        }

        // Draw selection
        let draw_scroll: ImVec2 = ImVec2::new(state.ScrollX, 0.0);
        if render_selection
        {
            let text_selected_begin = ImMin(state.Stb.select_start, state.Stb.select_end);
            let text_selected_end = ImMax(state.Stb.select_start, state.Stb.select_end);

            bg_color: u32 = GetColorU32(ImGuiCol_TextSelectedBg, if render_cursor { 1.0} else {0.60}); // FIXME: current code flow mandate that render_cursor is always true here, we are leaving the transparent one for tests.
            let bg_offy_up: c_float =  if is_multiline { 0.0 }else {- 1.0};    // FIXME: those offsets should be part of the style? they don't play so well with multi-line selection.
            let bg_offy_dn: c_float = if is_multiline { 0.0} else {2.0};
            let mut rect_pos: ImVec2 = draw_pos + select_start_offset - draw_scroll;
            // for (*let p: ImWchar = text_selected_begin; p < text_selected_end; )
            for p in text_selected_begin .. text_selected_end
            {
                if rect_pos.y > clip_rect.w + g.FontSize{
                    break;}
                if rect_pos.y < clip_rect.y
                {
                    //p = (const ImWchar*)wmemchr((const wchar_t*)p, '\n', text_selected_end - p);  // FIXME-OPT: Could use this when wchar_t are 16-bit
                    //p = p ? p + 1 : text_selected_end;
                    while p < text_selected_end {
                        // TODO
                        // if (*p + + == '\n') {
                        //     break;
                        // }
                    }
                }
                else
                {
                    // let rect_size: ImVec2 = InputTextCalcTextSizeW(p, text_selected_end, &mut p,  true);
                    // if rect_size.x <= 0.0 { rect_size.x = IM_FLOOR(g.Font.GetCharAdvance(' ') * 0.5); } // So we can see selected empty lines
                    // let mut rect: ImRect = ImRect::new(rect_pos + ImVec2::new(0.0, bg_offy_up - g.FontSize), rect_pos + ImVec2::new(rect_size.x, bg_offy_dn));
                    rect.ClipWith(clip_rect);
                    if rect.Overlaps(clip_rect) {
                        draw_window.DrawList.AddRectFilled(rect.Min, rect.Max, bg_color);
                    }
                }
                rect_pos.x = draw_pos.x - draw_scroll.x;
                rect_pos.y += g.FontSize;
            }
        }

        // We test for 'buf_display_max_length' as a way to avoid some pathological cases (e.g. single-line 1 MB string) which would make ImDrawList crash.
        if is_multiline || (buf_display_end - buf_display) < buf_display_max_length
        {
            col: u32 = GetColorU32(if is_displaying_hint { ImGuiCol_TextDisabled } else { ImGuiCol_Text }, 0.0);
            draw_window.DrawList.AddText(g.Font, g.FontSize, draw_pos - draw_scroll, col, & buf_display, buf_display_end, 0.0, if is_multiline { None} else {& clip_rect});
        }

        // Draw blinking cursor
        if render_cursor
        {
            state.CursorAnim += io.DeltaTime;
            let mut cursor_is_visible: bool =  (!g.IO.ConfigInputTextCursorBlink) || (state.CursorAnim <= 0.0) || ImFmod(state.CursorAnim, 1.200) <= 0.80;
            let cursor_screen_pos: ImVec2 = ImFloor(draw_pos + cursor_offset - draw_scroll);
            let mut cursor_screen_rect: ImRect = ImRect::new(cursor_screen_pos.x, cursor_screen_pos.y - g.FontSize + 0.5, cursor_screen_pos.x + 1.0, cursor_screen_pos.y - 1.5);
            if cursor_is_visible && cursor_screen_rect.Overlaps(clip_rect) {
                draw_window.DrawList.AddLine(cursor_screen_rect.min, cursor_screen_rect.GetBL(), GetColorU32(ImGuiCol_Text, 0.0));
            }

            // Notify OS of text input position for advanced IME (-1 x offset so that Windows IME can cover our cursor. Bit of an extra nicety.)
            if !is_readonly
            {
                g.PlatformImeData.WantVisible = true;
                g.PlatformImeData.InputPos = ImVec2::new(cursor_screen_pos.x - 1.0, cursor_screen_pos.y - g.FontSize);
                g.PlatformImeData.InputLineHeight = g.FontSize;
                g.PlatformImeViewport = window.Viewport.ID;
            }
        }
    }
    else
    {
        // Render text only (no selection, no cursor)
        if is_multiline {
            // text_size = ImVec2::new(inner_size.x, InputTextCalcTextLenAndLineCount(buf_display, &buf_display_end) * g.FontSize);
        } // We don't need width
        else if !is_displaying_hint && g.ActiveId == id {
            buf_display_end = &buf_display + state.CurLenA;
        }
        else if !is_displaying_hint {
            // buf_display_end = buf_display + strlen(buf_display);
        }

        // if is_multiline || (buf_display_end - buf_display) < buf_display_max_length
        // {
        //     col: u32 = GetColorU32(if is_displaying_hint { ImGuiCol_TextDisabled } else { ImGuiCol_Text }, 0.0);
        //     draw_window.DrawList.AddText(g.Font, g.FontSize, draw_pos, col, &buf_display, buf_display_end, 0.0, if is_multiline { None} else {& clip_rect});
        // }
    }

    if is_password && !is_displaying_hint {
        PopFont();
    }

    if is_multiline
    {
        // For focus requests to work on our multiline we need to ensure our child ItemAdd() call specifies the ImGuiItemFlags_Inputable (ref issue #4761)...
        layout_ops::Dummy(g, ImVec2::new(text_size.x, text_size.y + style.FramePadding.y));
        let mut backup_item_flags: ImGuiItemFlags =  g.CurrentItemFlags;
        g.CurrentItemFlags |= ImGuiItemFlags_Inputable | ImGuiItemFlags_NoTabStop;
        EndChild();
        item_data_backup.status_flags |= (g.last_item_data.StatusFlags & ImGuiItemStatusFlags_HoveredWindow);
        g.CurrentItemFlags = backup_item_flags;

        // ...and then we need to undo the group overriding last item data, which gets a bit messy as EndGroup() tries to forward scrollbar being active...
        // FIXME: This quite messy/tricky, should attempt to get rid of the child window.
        EndGroup();
        if g.last_item_data.ID == 0
        {
            g.last_item_data.ID = id;
            g.last_item_data.in_flags = item_data_backup.in_flags;
            g.last_item_data.StatusFlags = item_data_backup.status_flags;
        }
    }

    // Log as text
    if g.LogEnabled && (!is_password || is_displaying_hint)
    {
        LogSetNextTextDecoration("{", "}");
        // LogRenderedText(&draw_pos, buf_display, buf_display_end);
    }

    if label_size.x > 0.0 {
        RenderText(ImVec2::new(frame_bb.max.x + style.ItemInnerSpacing.x, frame_bb.min.y + style.FramePadding.y), label, false, g);
    }

    if value_changed && flag_clear(flags, ImGuiInputTextFlags_NoMarkEdited) {
        MarkItemEdited(g, id);
    }

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.last_item_data.StatusFlags);
    if flag_set(flags, ImGuiInputTextFlags_EnterReturnsTrue) { return  validated; }
    else {
        return value_changed;
    }
}

pub unsafe fn DebugNodeInputTextState(state: &mut ImGuiInputTextState)
{
// #ifndef IMGUI_DISABLE_DEBUG_TOOLS
//     let g = GImGui; // ImGuiContext& g = *GImGui;
//     ImStb::stb_state: &mut STB_TexteditState = &state.Stb;
//     ImStb::*mut StbUndoState undo_state = &stb_state.undostate;
//     text_ops::Text("ID: 0x{}, ActiveID: 0x{}", state.ID, g.ActiveId);
//     text_ops::Text("CurLenW: {}, CurLenA: {}, Cursor: {}, Selection: {}..{}", state.CurLenA, state.CurLenW, stb_state.cursor, stb_state.select_start, stb_state.select_end);
//     text_ops::Text("undo_point: {}, redo_point: {}, undo_char_point: {}, redo_char_point: {}", undo_state.undo_point, undo_state.redo_point, undo_state.undo_char_point, undo_state.redo_char_point);
//     if (BeginChild("undopoints", ImVec2::new(0.0, GetTextLineHeight() * 15), true)) // Visualize undo state
//     {
//         PushStyleVar(ImGuiStyleVar_ItemSpacing, ImVec2::new(0, 0));
//         for (let n: c_int = 0; n < STB_TEXTEDIT_UNDOSTATECOUNT; n++)
//         {
//             ImStb::*mut StbUndoRecord undo_rec = &undo_state.undo_rec[n];
//             const  undo_rec_type: c_char = if n < undo_state.undo_point) ? 'u' : (n >= undo_state.redo_point { 'r'} else { ' '};
//             if (undo_rec_type == ' ')
//                 BeginDisabled();
//             buf: [c_char;64] = "";
//             if (undo_rec_type != ' ' && undo_rec->char_storage != -1)
//                 ImTextStrToUtf8(buf, buf.len(), undo_state.undo_char + undo_rec->char_storage, undo_state.undo_char + undo_rec->char_storage + undo_rec->insert_length);
//             text_ops::Text("%c [{}] where %03d, insert %03d, delete %03d, char_storage %03d \"{}\"",
//                            undo_rec_type, n, undo_rec-> where, undo_rec->insert_length, undo_rec->delete_length, undo_rec->char_storage, buf);
//             if (undo_rec_type == ' ')
//                 EndDisabled();
//         }
//         PopStyleVar();
//     }
//     EndChild();
// // #else
//     IM_UNUSED(state);
// // #endif
}
