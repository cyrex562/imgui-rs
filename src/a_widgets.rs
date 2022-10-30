// dear imgui, v1.89 WIP
#![allow(non_upper_case_globals)]
// (widgets code)

/*

Index of this file:

// [SECTION] Forward Declarations
// [SECTION] Widgets: Text, etc.
// [SECTION] Widgets: Main (Button, Image, Checkbox, RadioButton, ProgressBar, Bullet, etc.)
// [SECTION] Widgets: Low-level Layout helpers (Spacing, Dummy, NewLine, Separator, etc.)
// [SECTION] Widgets: ComboBox
// [SECTION] Data Type and Data Formatting Helpers
// [SECTION] Widgets: DragScalar, DragFloat, DragInt, etc.
// [SECTION] Widgets: SliderScalar, SliderFloat, SliderInt, etc.
// [SECTION] Widgets: InputScalar, InputFloat, InputInt, etc.
// [SECTION] Widgets: InputText, InputTextMultiline
// [SECTION] Widgets: ColorEdit, ColorPicker, ColorButton, etc.
// [SECTION] Widgets: TreeNode, CollapsingHeader, etc.
// [SECTION] Widgets: Selectable
// [SECTION] Widgets: ListBox
// [SECTION] Widgets: PlotLines, PlotHistogram
// [SECTION] Widgets: Value helpers
// [SECTION] Widgets: MenuItem, BeginMenu, EndMenu, etc.
// [SECTION] Widgets: BeginTabBar, EndTabBar, etc.
// [SECTION] Widgets: BeginTabItem, EndTabItem, etc.
// [SECTION] Widgets: Columns, BeginColumns, EndColumns, etc.

*/

// #if defined(_MSC_VER) && !defined(_CRT_SECURE_NO_WARNINGS)
// #define _CRT_SECURE_NO_WARNINGS
// #endif

// #include "imgui.h"
// #ifndef IMGUI_DISABLE

// #ifndef IMGUI_DEFINE_MATH_OPERATORS
// #define IMGUI_DEFINE_MATH_OPERATORS
// #endif
// #include "imgui_internal.h"

// System includes
// #include <ctype.h>      // toupper
// #if defined(_MSC_VER) && _MSC_VER <= 1500 // MSVC 2008 or earlier
// #include <stddef.h>     // intptr_t
// #else
// #include <stdint.h>     // intptr_t
// #endif

//-------------------------------------------------------------------------
// Warnings
//-------------------------------------------------------------------------

// Visual Studio warnings
// #ifdef _MSC_VER
// #pragma warning (disable: 4127)     // condition expression is constant
// #pragma warning (disable: 4996)     // 'This function or variable may be unsafe': strcpy, strdup, sprintf, vsnprintf, sscanf, fopen
// #if defined(_MSC_VER) && _MSC_VER >= 1922 // MSVC 2019 16.2 or later
// #pragma warning (disable: 5054)     // operator '|': deprecated between enumerations of different types
// #endif
// #pragma warning (disable: 26451)    // [Static Analyzer] Arithmetic overflow : Using operator 'xxx' on a 4 byte value and then casting the result to a 8 byte value. Cast the value to the wider type before calling operator 'xxx' to avoid overflow(io.2).
// #pragma warning (disable: 26812)    // [Static Analyzer] The enum type 'xxx' is unscoped. Prefer 'enum class' over 'enum' (Enum.3).
// #endif

// Clang/GCC warnings with -Weverything
// #if defined(__clang__)
// #if __has_warning("-Wunknown-warning-option")
// #pragma clang diagnostic ignored "-Wunknown-warning-option"         // warning: unknown warning group 'xxx'                      // not all warnings are known by all Clang versions and they tend to be rename-happy.. so ignoring warnings triggers new warnings on some configuration. Great!
// #endif
// #pragma clang diagnostic ignored "-Wunknown-pragmas"                // warning: unknown warning group 'xxx'
// #pragma clang diagnostic ignored "-Wold-style-cast"                 // warning: use of old-style cast                            // yes, they are more terse.
// #pragma clang diagnostic ignored "-Wfloat-equal"                    // warning: comparing floating point with == or != is unsafe // storing and comparing against same constants (typically 0.0) is ok.
// #pragma clang diagnostic ignored "-Wformat-nonliteral"              // warning: format string is not a string literal            // passing non-literal to vsnformat(). yes, user passing incorrect format strings can crash the code.
// #pragma clang diagnostic ignored "-Wsign-conversion"                // warning: implicit conversion changes signedness
// #pragma clang diagnostic ignored "-Wzero-as-null-pointer-constant"  // warning: zero as null pointer constant                    // some standard header variations use #define NULL 0
// #pragma clang diagnostic ignored "-Wdouble-promotion"               // warning: implicit conversion from 'float' to 'double' when passing argument to function  // using printf() is a misery with this as C++ va_arg ellipsis changes float to double.
// #pragma clang diagnostic ignored "-Wenum-enum-conversion"           // warning: bitwise operation between different enumeration types ('XXXFlags_' and 'XXXFlagsPrivate_')
// #pragma clang diagnostic ignored "-Wdeprecated-enum-enum-conversion"// warning: bitwise operation between different enumeration types ('XXXFlags_' and 'XXXFlagsPrivate_') is deprecated
// #pragma clang diagnostic ignored "-Wimplicit-int-float-conversion"  // warning: implicit conversion from 'xxx' to 'float' may lose precision
// #elif defined(__GNUC__)
// #pragma GCC diagnostic ignored "-Wpragmas"                          // warning: unknown option after '#pragma GCC diagnostic' kind
// #pragma GCC diagnostic ignored "-Wformat-nonliteral"                // warning: format not a string literal, format string not checked
// #pragma GCC diagnostic ignored "-Wclass-memaccess"                  // [__GNUC__ >= 8] warning: 'memset/memcpy' clearing/writing an object of type 'xxxx' with no trivial copy-assignment; use assignment or value-initialization instead
// #pragma GCC diagnostic ignored "-Wdeprecated-enum-enum-conversion"  // warning: bitwise operation between different enumeration types ('XXXFlags_' and 'XXXFlagsPrivate_') is deprecated
// #endif

//-------------------------------------------------------------------------
// Data
//-------------------------------------------------------------------------

use std::borrow::Borrow;
use std::env::args;
use std::ops::Index;
use std::ptr::{null, null_mut};
use libc::{c_char, c_double, c_float, c_int, c_uint, c_void, INT_MAX, INT_MIN, memcmp, memcpy, memmove, memset, size_t, strcmp, strlen, strncmp};
use crate::activate_flags::{ImGuiActivateFlags_PreferInput, ImGuiActivateFlags_TryToPreserveState};
use crate::color::{IM_COL32, IM_COL32_A_MASK, ImGuiCol_Border, ImGuiCol_BorderShadow, ImGuiCol_Button, ImGuiCol_ButtonActive, ImGuiCol_ButtonHovered, ImGuiCol_CheckMark, ImGuiCol_ChildBg, ImGuiCol_FrameBg, ImGuiCol_FrameBgActive, ImGuiCol_FrameBgHovered, ImGuiCol_Header, ImGuiCol_HeaderActive, ImGuiCol_HeaderHovered, ImGuiCol_PlotHistogram, ImGuiCol_ScrollbarBg, ImGuiCol_ScrollbarGrab, ImGuiCol_ScrollbarGrabActive, ImGuiCol_ScrollbarGrabHovered, ImGuiCol_Separator, ImGuiCol_SeparatorActive, ImGuiCol_SeparatorHovered, ImGuiCol_SliderGrab, ImGuiCol_SliderGrabActive, ImGuiCol_Text, ImGuiCol_TextDisabled, ImGuiCol_TextSelectedBg};
use crate::combo_flags::{ImGuiComboFlags, ImGuiComboFlags_CustomPreview, ImGuiComboFlags_HeightLarge, ImGuiComboFlags_HeightMask_, ImGuiComboFlags_HeightRegular, ImGuiComboFlags_HeightSmall, ImGuiComboFlags_NoArrowButton, ImGuiComboFlags_None, ImGuiComboFlags_NoPreview, ImGuiComboFlags_PopupAlignLeft};
use crate::combo_preview_data::ImGuiComboPreviewData;
use crate::data_type::{ImGuiDataType, ImGuiDataType_COUNT, ImGuiDataType_Double, ImGuiDataType_Float, ImGuiDataType_S16, ImGuiDataType_S32, ImGuiDataType_S64, ImGuiDataType_S8, ImGuiDataType_U16, ImGuiDataType_U32, ImGuiDataType_U64, ImGuiDataType_U8};
use crate::data_type_info::{GDataTypeInfo, ImGuiDataTypeInfo};
use crate::data_type_temp_storage::ImGuiDataTypeTempStorage;
use crate::direction::{ImGuiDir, ImGuiDir_Down, ImGuiDir_Left, ImGuiDir_Right, ImGuiDir_Up};
use crate::draw_flags::{ImDrawFlags, ImDrawFlags_RoundCornersAll, ImDrawFlags_RoundCornersBottomLeft, ImDrawFlags_RoundCornersBottomRight, ImDrawFlags_RoundCornersLeft, ImDrawFlags_RoundCornersNone, ImDrawFlags_RoundCornersRight, ImDrawFlags_RoundCornersTopRight};
use crate::draw_list::ImDrawList;
use crate::{button_ops, checkbox_ops, data_type_ops, drag, GImGui, ImGuiViewport, ImHashStr, input_num_ops, layout_ops, popup_ops, radio_button, scrolling_ops, separator, stb, text_ops};
use crate::axis::{ImGuiAxis, ImGuiAxis_X, ImGuiAxis_Y};
use crate::backend_flags::ImGuiBackendFlags_HasGamepad;
use crate::button_flags::{ImGuiButtonFlags, ImGuiButtonFlags_AlignTextBaseLine, ImGuiButtonFlags_AllowItemOverlap, ImGuiButtonFlags_DontClosePopups, ImGuiButtonFlags_FlattenChildren, ImGuiButtonFlags_MouseButtonDefault_, ImGuiButtonFlags_MouseButtonLeft, ImGuiButtonFlags_MouseButtonMask_, ImGuiButtonFlags_MouseButtonMiddle, ImGuiButtonFlags_MouseButtonRight, ImGuiButtonFlags_NoHoldingActiveId, ImGuiButtonFlags_NoHoveredOnFocus, ImGuiButtonFlags_NoKeyModifiers, ImGuiButtonFlags_NoNavFocus, ImGuiButtonFlags_None, ImGuiButtonFlags_PressedOnClick, ImGuiButtonFlags_PressedOnClickRelease, ImGuiButtonFlags_PressedOnClickReleaseAnywhere, ImGuiButtonFlags_PressedOnDefault_, ImGuiButtonFlags_PressedOnDoubleClick, ImGuiButtonFlags_PressedOnDragDropHold, ImGuiButtonFlags_PressedOnMask_, ImGuiButtonFlags_PressedOnRelease, ImGuiButtonFlags_Repeat};
use crate::button_ops::{ButtonBehavior, ButtonEx, InvisibleButton};
use crate::checkbox_ops::CheckboxFlags;
use crate::child_ops::{BeginChild, BeginChildEx, BeginChildFrame, EndChild, EndChildFrame};
use crate::clipboard_ops::{GetClipboardText, SetClipboardText};
use crate::color_edit_flags::{ImGuiColorEditFlags, ImGuiColorEditFlags_AlphaBar, ImGuiColorEditFlags_AlphaPreview, ImGuiColorEditFlags_AlphaPreviewHalf, ImGuiColorEditFlags_DataTypeMask_, ImGuiColorEditFlags_DefaultOptions_, ImGuiColorEditFlags_DisplayHex, ImGuiColorEditFlags_DisplayHSV, ImGuiColorEditFlags_DisplayMask_, ImGuiColorEditFlags_DisplayRGB, ImGuiColorEditFlags_Float, ImGuiColorEditFlags_HDR, ImGuiColorEditFlags_InputHSV, ImGuiColorEditFlags_InputMask_, ImGuiColorEditFlags_InputRGB, ImGuiColorEditFlags_NoAlpha, ImGuiColorEditFlags_NoBorder, ImGuiColorEditFlags_NoDragDrop, ImGuiColorEditFlags_NoInputs, ImGuiColorEditFlags_NoLabel, ImGuiColorEditFlags_NoOptions, ImGuiColorEditFlags_NoPicker, ImGuiColorEditFlags_NoSidePreview, ImGuiColorEditFlags_NoSmallPreview, ImGuiColorEditFlags_NoTooltip, ImGuiColorEditFlags_PickerHueBar, ImGuiColorEditFlags_PickerHueWheel, ImGuiColorEditFlags_PickerMask_, ImGuiColorEditFlags_Uint8};
use crate::color_ops::{ColorConvertFloat4ToU32, ColorConvertHSVtoRGB, ColorConvertRGBtoHSV};
use crate::condition::{ImGuiCond, ImGuiCond_Always, ImGuiCond_Once};
use crate::config_flags::ImGuiConfigFlags_NavEnableGamepad;
use crate::content_ops::GetContentRegionAvail;
use crate::cursor_ops::{GetCursorScreenPos, Indent, SetCursorScreenPos, Unindent};
use crate::data_type_ops::{DataTypeApplyFromText, DataTypeApplyOp, DataTypeFormatString, DataTypeOperationAdd, DataTypeOperationSub};
use crate::dock_node::ImGuiDockNode;
use crate::drag_drop_flags::{ImGuiDragDropFlags_SourceNoDisableHover, ImGuiDragDropFlags_SourceNoHoldToOpenOthers};
use crate::drag_drop_ops::{AcceptDragDropPayload, BeginDragDropSource, BeginDragDropTarget, EndDragDropSource, EndDragDropTarget, SetDragDropPayload};
use crate::font::ImFont;
use crate::font_glyph::ImFontGlyph;
use crate::font_ops::{PopFont, PushFont};
use crate::frame_ops::GetFrameHeight;
use crate::geometry_ops::{ImTriangleBarycentricCoords, ImTriangleClosestPoint, ImTriangleContainsPoint};
use crate::group_ops::{BeginGroup, EndGroup};
use crate::hovered_flags::ImGuiHoveredFlags_AllowWhenBlockedByActiveItem;
use crate::id_ops::{ClearActiveID, GetIDWithSeed, KeepAliveID, PopID, push_int_id, push_str_id, PushID, PushOverrideID, SetActiveID, SetHoveredID};
use crate::input_num_ops::InputText;
use crate::input_ops::{CalcTypematicRepeatAmount, GetKeyData, IsKeyDown, IsKeyPressed, IsMouseClicked, IsMouseDragging, IsMouseDragPastThreshold, IsMousePosValid, SetMouseCursor};
use crate::input_source::{ImGuiInputSource, ImGuiInputSource_Clipboard, ImGuiInputSource_Gamepad, ImGuiInputSource_Keyboard, ImGuiInputSource_Mouse, ImGuiInputSource_Nav};
use crate::input_text_callback_data::ImGuiInputTextCallbackData;
use crate::input_text_flags::{ImGuiInputTextFlags, ImGuiInputTextFlags_AllowTabInput, ImGuiInputTextFlags_AlwaysOverwrite, ImGuiInputTextFlags_AutoSelectAll, ImGuiInputTextFlags_CallbackAlways, ImGuiInputTextFlags_CallbackCharFilter, ImGuiInputTextFlags_CallbackCompletion, ImGuiInputTextFlags_CallbackEdit, ImGuiInputTextFlags_CallbackHistory, ImGuiInputTextFlags_CallbackResize, ImGuiInputTextFlags_CharsDecimal, ImGuiInputTextFlags_CharsHexadecimal, ImGuiInputTextFlags_CharsNoBlank, ImGuiInputTextFlags_CharsScientific, ImGuiInputTextFlags_CharsUppercase, ImGuiInputTextFlags_CtrlEnterForNewLine, ImGuiInputTextFlags_EnterReturnsTrue, ImGuiInputTextFlags_MergedItem, ImGuiInputTextFlags_Multiline, ImGuiInputTextFlags_NoHorizontalScroll, ImGuiInputTextFlags_NoMarkEdited, ImGuiInputTextFlags_None, ImGuiInputTextFlags_NoUndoRedo, ImGuiInputTextFlags_Password, ImGuiInputTextFlags_ReadOnly};
use crate::input_text_state::ImGuiInputTextState;
use crate::io::ImGuiIO;
use crate::io_ops::GetIO;
use crate::item_flags::{ImGuiItemFlags, ImGuiItemFlags_ButtonRepeat, ImGuiItemFlags_Disabled, ImGuiItemFlags_Inputable, ImGuiItemFlags_MixedValue, ImGuiItemFlags_NoNav, ImGuiItemFlags_NoNavDefaultFocus, ImGuiItemFlags_None, ImGuiItemFlags_NoTabStop, ImGuiItemFlags_ReadOnly, ImGuiItemFlags_SelectableDontClosePopup};
use crate::item_ops::{CalcItemSize, CalcItemWidth, CalcWrapWidthForPos, IsClippedEx, IsItemActive, IsItemHovered, ItemAdd, ItemHoverable, ItemSize, MarkItemEdited, PopItemFlag, PopItemWidth, PushItemFlag, PushItemWidth, PushMultiItemsWidths, SetNextItemWidth};
use crate::item_status_flags::{ImGuiItemStatusFlags, ImGuiItemStatusFlags_Checkable, ImGuiItemStatusFlags_Checked, ImGuiItemStatusFlags_FocusedByTabbing, ImGuiItemStatusFlags_HasDisplayRect, ImGuiItemStatusFlags_HoveredRect, ImGuiItemStatusFlags_HoveredWindow, ImGuiItemStatusFlags_Openable, ImGuiItemStatusFlags_Opened, ImGuiItemStatusFlags_ToggledOpen, ImGuiItemStatusFlags_ToggledSelection};
use crate::key::{ImGuiKey, ImGuiKey_A, ImGuiKey_Backspace, ImGuiKey_C, ImGuiKey_Delete, ImGuiKey_DownArrow, ImGuiKey_End, ImGuiKey_Enter, ImGuiKey_Escape, ImGuiKey_Home, ImGuiKey_Insert, ImGuiKey_KeypadEnter, ImGuiKey_LeftArrow, ImGuiKey_NavGamepadActivate, ImGuiKey_NavGamepadCancel, ImGuiKey_NavGamepadInput, ImGuiKey_NavGamepadTweakFast, ImGuiKey_NavGamepadTweakSlow, ImGuiKey_NavKeyboardTweakFast, ImGuiKey_NavKeyboardTweakSlow, ImGuiKey_None, ImGuiKey_PageDown, ImGuiKey_PageUp, ImGuiKey_RightArrow, ImGuiKey_Space, ImGuiKey_Tab, ImGuiKey_UpArrow, ImGuiKey_V, ImGuiKey_X, ImGuiKey_Y, ImGuiKey_Z};
use crate::last_item_data::ImGuiLastItemData;
use crate::layout_ops::SameLine;
use crate::layout_type::{ImGuiLayoutType, ImGuiLayoutType_Horizontal, ImGuiLayoutType_Vertical};
use crate::logging_ops::{LogRenderedText, LogSetNextTextDecoration};
use crate::math_ops::{ImAddClampOverflow, ImAtan2, ImCharIsBlankA, ImCharIsBlankW, ImClamp, ImCos, ImFabs, ImFmod, ImLerp, ImLinearSweep, ImMax, ImMin, ImRotate, ImSaturateFloat, ImSin, ImSubClampOverflow, ImSwap};
use crate::mod_flags::{ImGuiModFlags_Ctrl, ImGuiModFlags_Shift, ImGuiModFlags_Super};
use crate::mouse_cursor::{ImGuiMouseCursor_ResizeEW, ImGuiMouseCursor_ResizeNS, ImGuiMouseCursor_TextInput};
use crate::mouse_ops::StartMouseMovingWindowOrNode;
use crate::nav_highlight_flags::{ImGuiNavHighlightFlags, ImGuiNavHighlightFlags_NoRounding, ImGuiNavHighlightFlags_TypeThin};
use crate::nav_layer::{ImGuiNavLayer, ImGuiNavLayer_Main, ImGuiNavLayer_Menu};
use crate::nav_move_flags::ImGuiNavMoveFlags_Forwarded;
use crate::nav_ops::{GetNavTweakPressedAmount, NavMoveRequestButNoResultYet, NavMoveRequestCancel, NavMoveRequestForward, SetFocusID, SetNavID};
use crate::next_item_data_flags::ImGuiNextItemDataFlags_HasOpen;
use crate::next_window_data_flags::{ImGuiNextWindowDataFlags, ImGuiNextWindowDataFlags_HasSizeConstraint};
use crate::old_columns::ImGuiOldColumns;
use crate::plot_type::{ImGuiPlotType_Histogram, ImGuiPlotType_Lines};
use crate::popup_data::ImGuiPopupData;
use crate::popup_flags::{ImGuiPopupFlags_MouseButtonRight, ImGuiPopupFlags_None};
use crate::popup_ops::{BeginPopup, BeginPopupEx, CloseCurrentPopup, ClosePopupToLevel, EndPopup, FindBestWindowPosForPopupEx, GetPopupAllowedExtentRect, IsPopupOpen, OpenPopup, OpenPopupEx, OpenPopupOnItemClick};
use crate::popup_position_policy::ImGuiPopupPositionPolicy_ComboBox;
use crate::rect::{ImRect, IsRectVisible};
use crate::render_ops::{FindRenderedTextEnd, RenderArrow, RenderArrowDockMenu, RenderArrowPointingAt, RenderBullet, RenderCheckMark, RenderColorRectWithAlphaCheckerboard, RenderFrame, RenderFrameBorder, RenderNavHighlight, RenderRectFilledRangeH, RenderText, RenderTextClipped, RenderTextEllipsis, RenderTextWrapped};
use crate::scrolling_ops::{GetScrollMaxY, SetScrollY};
use crate::selectable_flags::{ImGuiSelectableFlags, ImGuiSelectableFlags_AllowDoubleClick, ImGuiSelectableFlags_AllowItemOverlap, ImGuiSelectableFlags_Disabled, ImGuiSelectableFlags_DontClosePopups, ImGuiSelectableFlags_DrawHoveredWhenHeld, ImGuiSelectableFlags_NoHoldingActiveID, ImGuiSelectableFlags_NoPadWithHalfSpacing, ImGuiSelectableFlags_SelectOnClick, ImGuiSelectableFlags_SelectOnNav, ImGuiSelectableFlags_SelectOnRelease, ImGuiSelectableFlags_SetNavIdOnHover, ImGuiSelectableFlags_SpanAllColumns, ImGuiSelectableFlags_SpanAvailWidth};
use crate::separator_flags::{ImGuiSeparatorFlags, ImGuiSeparatorFlags_Horizontal, ImGuiSeparatorFlags_SpanAllColumns, ImGuiSeparatorFlags_Vertical};
use crate::settings_ops::MarkIniSettingsDirty;
use crate::shade_verts_ops::ShadeVertsLinearColorGradientKeepAlpha;
use crate::shrink_width_item::ImGuiShrinkWidthItem;
use crate::slider_flags::{ImGuiSliderFlags, ImGuiSliderFlags_AlwaysClamp, ImGuiSliderFlags_Logarithmic, ImGuiSliderFlags_NoInput, ImGuiSliderFlags_NoRoundToFormat, ImGuiSliderFlags_ReadOnly, ImGuiSliderFlags_Vertical};
use crate::stb::stb_text_edit_row::StbTexteditRow;
use crate::stb::stb_text_edit_state::STB_TexteditState;
use crate::stb::stb_textedit::{stb_text_createundo, stb_text_makeundo_replace, STB_TEXTEDIT_CHARTYPE, stb_textedit_click, stb_textedit_cut, stb_textedit_drag, stb_textedit_initialize_state, stb_textedit_paste, STB_TEXTEDIT_STRING};
use crate::stb::stb_undo_record::StbUndoRecord;
use crate::stb::stb_undo_state::StbUndoState;
use crate::storage::ImGuiStorage;
use crate::string_ops::{ImFormatString, ImFormatStringToTempBufferV, ImStrbolW, ImStrncpy, ImStrTrimBlanks, ImTextCharFromUtf8, ImTextCountCharsFromUtf8, ImTextCountUtf8BytesFromStr, ImTextStrFromUtf8, ImTextStrToUtf8, str_to_const_c_char_ptr};
use crate::style::ImGuiStyle;
use crate::style_ops::{GetColorU32, GetColorU32FromImVec4, PopStyleColor, PushStyleColor, PushStyleColor2};
use crate::style_var::{ImGuiStyleVar_ChildBorderSize, ImGuiStyleVar_ChildRounding, ImGuiStyleVar_FramePadding, ImGuiStyleVar_ItemSpacing, ImGuiStyleVar_WindowMinSize, ImGuiStyleVar_WindowPadding, ImGuiStyleVar_WindowRounding};
use crate::tab_bar::ImGuiTabBar;
use crate::tab_bar_flags::{ImGuiTabBarFlags_FittingPolicyResizeDown, ImGuiTabBarFlags_FittingPolicyScroll, ImGuiTabBarFlags_NoTabListScrollingButtons, ImGuiTabBarFlags_SaveSettings, ImGuiTabBarFlags_TabListPopupButton};
use crate::tab_item::ImGuiTabItem;
use crate::tab_item_flags::{ImGuiTabItemFlags, ImGuiTabItemFlags_Button, ImGuiTabItemFlags_NoCloseButton, ImGuiTabItemFlags_NoCloseWithMiddleMouseButton, ImGuiTabItemFlags_NoReorder, ImGuiTabItemFlags_Preview, ImGuiTabItemFlags_SectionMask_, ImGuiTabItemFlags_UnsavedDocument};
use crate::table::ImGuiTable;
use crate::tables::{PopColumnsBackground, PushColumnsBackground, TablePopBackgroundChannel, TablePushBackgroundChannel};
use crate::text_flags::{ImGuiTextFlags, ImGuiTextFlags_None, ImGuiTextFlags_NoWidthForLargeClippedText};
use crate::text_ops::{CalcTextSize, GetTextLineHeight, GetTextLineHeightWithSpacing};
use crate::tooltip_flags::ImGuiTooltipFlags_OverridePreviousTooltip;
use crate::tooltip_ops::{BeginTooltipEx, EndTooltip};
use crate::tree_node_flags::{ImGuiTreeNodeFlags, ImGuiTreeNodeFlags_AllowItemOverlap, ImGuiTreeNodeFlags_Bullet, ImGuiTreeNodeFlags_ClipLabelForTrailingButton, ImGuiTreeNodeFlags_CollapsingHeader, ImGuiTreeNodeFlags_DefaultOpen, ImGuiTreeNodeFlags_Framed, ImGuiTreeNodeFlags_FramePadding, ImGuiTreeNodeFlags_NavLeftJumpsBackHere, ImGuiTreeNodeFlags_NoAutoOpenOnLog, ImGuiTreeNodeFlags_None, ImGuiTreeNodeFlags_NoTreePushOnOpen, ImGuiTreeNodeFlags_OpenOnArrow, ImGuiTreeNodeFlags_OpenOnDoubleClick, ImGuiTreeNodeFlags_Selected, ImGuiTreeNodeFlags_SpanAvailWidth, ImGuiTreeNodeFlags_SpanFullWidth};
use crate::type_defs::{ImGuiID, ImGuiInputTextCallback, ImTextureID, ImWchar};
use crate::utils::{flag_clear, flag_set, ImQsort};
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::viewport_ops::{GetMainViewport, SetCurrentViewport};
use crate::widget_ops::{PopTextWrapPos, PushTextWrapPos};
use crate::window::find::FindWindowByName;
use crate::window::focus::{FocusTopMostWindowUnderOne, FocusWindow, SetItemDefaultFocus};
use crate::window::ImGuiWindow;
use crate::window::ops::{Begin, BeginDisabled, CalcWindowNextAutoFitSize, End, EndDisabled, GetCurrentWindow, SetNextWindowSize};
use crate::window::props::{GetFontTexUvWhitePixel, SetNextWindowPos, SetNextWindowSizeConstraints, SetNextWindowViewport};
use crate::window::rect::{PopClipRect, PushClipRect, WindowRectAbsToRel};
use crate::window::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_ChildMenu, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_MenuBar, ImGuiWindowFlags_NoDocking, ImGuiWindowFlags_NoMove, ImGuiWindowFlags_NoNavFocus, ImGuiWindowFlags_None, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoScrollbar, ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_Popup};    // Time for drag-hold to activate items accepting the ImGuiButtonFlags_PressedOnDragDropHold button behavior.

//-------------------------------------------------------------------------
// [SECTION] Widgets: InputText, InputTextMultiline, InputTextWithHint
//-------------------------------------------------------------------------
// - InputText()
// - InputTextWithHint()
// - InputTextMultiline()
// - InputTextGetCharInfo() [Internal]
// - InputTextReindexLines() [Internal]
// - InputTextReindexLinesRange() [Internal]
// - InputTextEx() [Internal]
// - DebugNodeInputTextState() [Internal]
//-------------------------------------------------------------------------

// Wrapper for stb_textedit.h to edit text (our wrapper is for: statically sized buffer, single-line, wchar characters. InputText converts between UTF-8 and wchar)
// namespace ImStb
// {



//-------------------------------------------------------------------------
// [SECTION] Widgets: ColorEdit, ColorPicker, ColorButton, etc.
//-------------------------------------------------------------------------
// - ColorEdit3()
// - ColorEdit4()
// - ColorPicker3()
// - RenderColorRectWithAlphaCheckerboard() [Internal]
// - ColorPicker4()
// - ColorButton()
// - SetColorEditOptions()
// - ColorTooltip() [Internal]
// - ColorEditOptionsPopup() [Internal]
// - ColorPickerOptionsPopup() [Internal]
//-------------------------------------------------------------------------

pub unsafe fn ColorEdit3(label: &str,col: [c_float;3], flags: ImGuiColorEditFlags) -> bool
{
    let mut color_b: [c_float;4] = [col[0], col[1], col[2], 0.0];

    return ColorEdit4(label, &mut color_b, flags | ImGuiColorEditFlags_NoAlpha);
}

// ColorEdit supports RGB and HSV inputs. In case of RGB input resulting color may have undefined hue and/or saturation.
// Since widget displays both RGB and HSV values we must preserve hue and saturation to prevent these values resetting.
pub unsafe fn ColorEditRestoreHS(col: &[c_float], H: &mut c_float, S: &mut c_float, V: &mut c_float)
{
    // This check is optional. Suppose we have two color widgets side by side, both widgets display different colors, but both colors have hue and/or saturation undefined.
    // With color check: hue/saturation is preserved in one widget. Editing color in one widget would reset hue/saturation in another one.
    // Without color check: common hue/saturation would be displayed in all widgets that have hue/saturation undefined.
    // g.ColorEditLastColor is stored as ImU32 RGB value: this essentially gives us color equality check with reduced precision.
    // Tiny external color changes would not be detected and this check would still pass. This is OK, since we only restore hue/saturation _only_ if they are undefined,
    // therefore this change flipping hue/saturation from undefined to a very tiny value would still be represented in color picker.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.ColorEditLastColor != ColorConvertFloat4ToU32(ImVec4(col[0], col[1], col[2], 0)) { return ; }

    // When S == 0, H is undefined.
    // When H == 1 it wraps around to 0.
    if *S == 0.0 || (*H == 0.0 && g.ColorEditLastHue == 1.0) {
        *H = g.ColorEditLastHue;
    }

    // When V == 0, S is undefined.
    if *V == 0.0 {
        *S = g.ColorEditLastSat;
    }
}

// Edit colors components (each component in 0.0..1.0 range).
// See enum ImGuiColorEditFlags_ for available options. e.g. Only access 3 floats if ImGuiColorEditFlags_NoAlpha flag is set.
// With typical options: Left-click on color square to open color picker. Right-click to open option menu. CTRL-Click over input fields to edit them and TAB to go to next item.
pub unsafe fn ColorEdit4(label: &str, col: &mut [c_float;4], mut flags: ImGuiColorEditFlags) -> bool
{
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    if window.SkipItems { return  false; }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let setyle = &mut g.Style;
    let square_sz: c_float =  GetFrameHeight();
    let w_full: c_float =  CalcItemWidth();
    let w_button: c_float =  if flag_set(flags, ImGuiColorEditFlags_NoSmallPreview) { 0.0} else{ (square_sz + style.ItemInnerSpacing.x)};
    let w_inputs: c_float =  w_full - w_button;
    let mut  label_display_end = FindRenderedTextEnd(label);
    g.NextItemData.ClearFlags();

    BeginGroup();
    PushID(label);

    // If we're not showing any slider there's no point in doing any HSV conversions
    const flags_untouched: ImGuiColorEditFlags = flags;
    if flag_set(flags , ImGuiColorEditFlags_NoInputs) {
        flags = (flags & (!ImGuiColorEditFlags_DisplayMask_)) | ImGuiColorEditFlags_DisplayRGB | ImGuiColorEditFlags_NoOptions;
    }

    // Context menu: display and modify options (before defaults are applied)
    if flag_clear(flags, ImGuiColorEditFlags_NoOptions) {
        ColorEditOptionsPopup(&col, flags);
    }

    // Read stored options
    if flag_clear(flags, ImGuiColorEditFlags_DisplayMask_) {
        flags |= (g.ColorEditOptions & ImGuiColorEditFlags_DisplayMask_);
    }
    if flag_clear(flags, ImGuiColorEditFlags_DataTypeMask_) {
        flags |= (g.ColorEditOptions & ImGuiColorEditFlags_DataTypeMask_);
    }
    if flag_clear(flags, ImGuiColorEditFlags_PickerMask_) {
        flags |= (g.ColorEditOptions & ImGuiColorEditFlags_PickerMask_);
    }
    if flag_clear(flags, ImGuiColorEditFlags_InputMask_) {
        flags |= (g.ColorEditOptions & ImGuiColorEditFlags_InputMask_);
    }
    flags |= (g.ColorEditOptions & !(ImGuiColorEditFlags_DisplayMask_ | ImGuiColorEditFlags_DataTypeMask_ | ImGuiColorEditFlags_PickerMask_ | ImGuiColorEditFlags_InputMask_));
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_DisplayMask_)); // Check that only 1 is selected
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_InputMask_));   // Check that only 1 is selected

    let alpha: bool = flag_clear(flags, ImGuiColorEditFlags_NoAlpha);
    let hdr: bool = flag_set(flags, ImGuiColorEditFlags_HDR);
    let components: c_int = if alpha {4} else { 3 };

    // Convert to the formats we needf: [c_float;4] = { col[0], col[1], col[2], alpha ? col[3] : 1.0 };
    if flag_set(flags, ImGuiColorEditFlags_InputHSV) && flag_set(flags, ImGuiColorEditFlags_DisplayRGB) {
        ColorConvertHSVtoRGB(f[0], f[1], f[2], f[0], f[1], f[2]);
    }
    else if flag_set(flags, ImGuiColorEditFlags_InputRGB) && flag_set(flags, ImGuiColorEditFlags_DisplayHSV)
    {
        // Hue is lost when converting from greyscale rgb (saturation=0). Restore it.
        ColorConvertRGBtoHSV(f[0], f[1], f[2], f[0], f[1], f[2]);
        ColorEditRestoreHS(&col, &mut f[0], &mut f[1], &mut f[2]);
    }
    let mut i: [c_int;4] = [ IM_F32_TO_INT8_UNBOUND(f[0]), IM_F32_TO_INT8_UNBOUND(f[1]), IM_F32_TO_INT8_UNBOUND(f[2]), IM_F32_TO_INT8_UNBOUND(f[3]) ];

    let mut value_changed: bool =  false;
    let mut value_changed_as_float: bool =  false;

    let pos: ImVec2 = window.DC.CursorPos;
    let inputs_offset_x: c_float =  if style.ColorButtonPosition == ImGuiDir_Left { w_button }else {0.0};
    window.DC.CursorPos.x = pos.x + inputs_offset_x;

    if (flags & (ImGuiColorEditFlags_DisplayRGB | ImGuiColorEditFlags_DisplayHSV)) != 0 && flag_clear(flags, ImGuiColorEditFlags_NoInputs)
    {
        // RGB/HSV 0..255 Sliders
        w_item_one: c_float  = ImMax(1.0, IM_FLOOR((w_inputs - (style.ItemInnerSpacing.x) * (components - 1)) / components));
        let w_item_last: c_float =  ImMax(1.0, IM_FLOOR(w_inputs - (w_item_one + style.ItemInnerSpacing.x) * (components - 1)));

        // let hide_prefix: bool = if w_item_one <= CalcTextSize((flags & ImGuiColorEditFlags_Float { "M:0.0"} else { "M:000").x)};
        let ids: [String;4] = [ String::from("##X"),
            String::from("##Y"),
            String::from("##Z"),
            String::from("##W") ];
        let fmt_table_int: [[String;4];3] =
            [
                [   String::from("%3d"),   String::from("%3d"),   String::from("%3d"),   String::from("%3d") ], // Short display
                [ String::from("R:%3d"), String::from("G:%3d"), String::from("B:%3d"), String::from("A:%3d") ], // Long display for RGBA
                [ String::from("H:%3d"), String::from("S:%3d"), String::from("V:%3d"), String::from("A:%3d") ] // Long display for HSVA
            ];
        let fmt_table_float: [[String;4];3] =
        [
            [   String::from("%0.3f"),   String::from("%0.3f"),   String::from("%0.3f"),   String::from("%0.3f") ], // Short display
            [ String::from("R:%0.3f"), String::from("G:%0.3f"), String::from("B:%0.3f"), "A:%0.3f" ], // Long display for RGBA
            [ "H:%0.3f", "S:%0.3f", "V:%0.3f", "A:%0.3f" ]  // Long display for HSVA
        ];
        let fmt_idx: c_int = if hide_prefix { 0} else{ if flag_set(flags, ImGuiColorEditFlags_DisplayHSV) { 2}else {1}};

        // for (let n: c_int = 0; n < components; n++)
        for n in 0 .. components
        {
            if (n > 0) {
                SameLine(0.0, style.ItemInnerSpacing.x);
            }
            SetNextItemWidth(if n + 1 < components { w_item_one }else {w_item_last});

            // FIXME: When ImGuiColorEditFlags_HDR flag is passed HS values snap in weird ways when SV values go below 0.
            if flag_set(flags , ImGuiColorEditFlags_Float)
            {
                value_changed |= drag::DragFloat(ids[n], &mut f[n], 1.0 / 255f32, 0.0, if hdr { 0.0 } else { 1.0 }, fmt_table_float[fmt_idx][n], 0);
                value_changed_as_float |= value_changed;
            }
            else
            {
                value_changed |= drag::DragInt(ids[n], &mut i[n], 1.0, 0, if hdr { 0 } else { 255 }, fmt_table_int[fmt_idx][n], 0);
            }
            if flag_clear(flags, ImGuiColorEditFlags_NoOptions) {
                OpenPopupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);
            }
        }
    }
    else if flag_set(flags, ImGuiColorEditFlags_DisplayHex) && flag_clear(flags, ImGuiColorEditFlags_NoInputs)
    {
        // RGB Hexadecimal Input
        buf: [c_char;64];
        if (alpha) {}
            // ImFormatString(buf, buf.len(), "#%02X%02X%02X%02X", ImClamp(i[0], 0, 255), ImClamp(i[1], 0, 255), ImClamp(i[2], 0, 255), ImClamp(i[3], 0, 255));
        else {
            // ImFormatString(buf, buf.len(), "#%02X%02X%02X", ImClamp(i[0], 0, 255), ImClamp(i[1], 0, 255), ImClamp(i[2], 0, 255));
            SetNextItemWidth(w_inputs);
        }
        if InputText("##Text",
                     buf,
                     buf.len(),
                     ImGuiInputTextFlags_CharsHexadecimal | ImGuiInputTextFlags_CharsUppercase,
                     None,
                     None)
        {
            value_changed = true;
            p: *mut c_char = buf;
            while *p == '#' || ImCharIsBlankA(*p) {
                p += 1;
            }
            i[2] = 0;
            i[1] = 0;
            i[0] = i[1];
            i[3] = 0xFF; // alpha default to 255 is not parsed by scanf (e.g. inputting #FFFFFF omitting alpha)
            let mut r: c_int = 0;
            if alpha {
                // r = sscanf(p, "%02X%02X%02X%02X", &i[0], &i[1], &i[2], &i[3]);
            } // Treat at unsigned (%X is unsigned)
            else {
                // r = sscanf(p, "%02X%02X%02X", &i[0], &i[1], &i[2]);
            }
            IM_UNUSED(r); // Fixes C6031: Return value ignored: 'sscanf'.
        }
        if flag_clear(flags, ImGuiColorEditFlags_NoOptions) {
            OpenPopupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);
        }
    }

    picker_active_window: *mut ImGuiWindow= null_mut();
    if flag_clear(flags, ImGuiColorEditFlags_NoSmallPreview)
    {
        let button_offset_x: c_float = if flag_set(flags, ImGuiColorEditFlags_NoInputs) || (style.ColorButtonPosition == ImGuiDir_Left) {
            0.0
        } else {
            w_inputs + style.ItemInnerSpacing.x};
        window.DC.CursorPos = ImVec2::new(pos.x + button_offset_x, pos.y);

        let mut col_v4 = ImVec4::from_floats(col[0], col[1], col[2], if alpha { col[3]} else {1.0});
        if ColorButton("##ColorButton", &col_v4, flags, &Default::default())
        {
            if flag_clear(flags, ImGuiColorEditFlags_NoPicker)
            {
                // Store current color and open a picker
                g.ColorPickerRef = col_v4;
                OpenPopup("picker", 0);
                SetNextWindowPos(g.LastItemData.Rect.GetBL() + ImVec2::from_floats(0.0, style.ItemSpacing.y), 0, &Default::default());
            }
        }
        if flag_clear(flags, ImGuiColorEditFlags_NoOptions) {
            OpenPopupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);
        }

        if BeginPopup("picker", 0)
        {
            picker_active_window = g.CurrentWindow;
            if label != label_display_end
            {
                text_ops::TextEx(label, 0);
                layout_ops::Spacing();
            }
            picker_flags_to_forward: ImGuiColorEditFlags = ImGuiColorEditFlags_DataTypeMask_ | ImGuiColorEditFlags_PickerMask_ | ImGuiColorEditFlags_InputMask_ | ImGuiColorEditFlags_HDR | ImGuiColorEditFlags_NoAlpha | ImGuiColorEditFlags_AlphaBar;
            picker_flags: ImGuiColorEditFlags = (flags_untouched & picker_flags_to_forward) | ImGuiColorEditFlags_DisplayMask_ | ImGuiColorEditFlags_NoLabel | ImGuiColorEditFlags_AlphaPreviewHalf;
            SetNextItemWidth(square_sz * 12.0); // Use 256 + bar sizes?
            value_changed |= ColorPicker4("##picker", col, picker_flags, g.ColorPickerRef.x);
            EndPopup();
        }
    }

    if label != label_display_end && flag_clear(flags, ImGuiColorEditFlags_NoLabel)
    {
        SameLine(0.0, style.ItemInnerSpacing.x);
        text_ops::TextEx(label, 0);
    }

    // Convert back
    if value_changed && picker_active_window == null_mut()
    {
        if !value_changed_as_float {
            // for (let n: c_int = 0; n < 4; n++)
            for n in 0..4 {
                f[n] = i[n] / 255f32;
            }
        }
        if flag_set(flags, ImGuiColorEditFlags_DisplayHSV) && flag_set(flags, ImGuiColorEditFlags_InputRGB)
        {
            g.ColorEditLastHue = f[0];
            g.ColorEditLastSat = f[1];
            ColorConvertHSVtoRGB(f[0], f[1], f[2], f[0], f[1], f[2]);
            g.ColorEditLastColor = ColorConvertFloat4ToU32(ImVec4(f[0], f[1], f[2], 0));
        }
        if flag_set(flags, ImGuiColorEditFlags_DisplayRGB) && flag_set(flags, ImGuiColorEditFlags_InputHSV) {
            ColorConvertRGBtoHSV(f[0], f[1], f[2], f[0], f[1], f[2]);
        }

        col[0] = f[0];
        col[1] = f[1];
        col[2] = f[2];
        if alpha {
            col[3] = f[3];
        }
    }

    PopID();
    EndGroup();

    // Drag and Drop Target
    // NB: The flag test is merely an optional micro-optimization, BeginDragDropTarget() does the same test.
    if flag_set(g.LastItemData.StatusFlags, ImGuiItemStatusFlags_HoveredRect) && flag_clear(flags, ImGuiColorEditFlags_NoDragDrop) && BeginDragDropTarget()
    {
        let mut accepted_drag_drop: bool =  false;
        let payload = AcceptDragDropPayload(IMGUI_PAYLOAD_TYPE_COLOR_30, 0);
        if payload.is_null() == false
        {
            // memcpy((&mut c_float)col, payload.Data, sizeof * 3); // Preserve alpha if any //-V512 //-V1086
            value_changed = true;
            accepted_drag_drop = true;
        }
        let payload = AcceptDragDropPayload(IMGUI_PAYLOAD_TYPE_COLOR_40, 0);
        if payload.is_null() == false
        {
            // memcpy(col, payload.Data, sizeof * components);
            value_changed = true;
            accepted_drag_drop = true;
        }

        // Drag-drop payloads are always RGB
        if accepted_drag_drop && flag_set(flags, ImGuiColorEditFlags_InputHSV) {
            ColorConvertRGBtoHSV(col[0], col[1], col[2], &mut col[0], &mut col[1], &mut col[2]);
        }
        EndDragDropTarget();
    }

    // When picker is being actively used, use its active id so IsItemActive() will function on ColorEdit4().
    if picker_active_window && g.ActiveId != 0 && g.ActiveIdWindow == picker_active_window {
        g.LastItemData.ID = g.ActiveId;
    }

    if value_changed{
        MarkItemEdited(g.LastItemData.ID);}

    return value_changed;
}

pub unsafe fn ColorPicker3(label: &str, col: &mut [c_float; 3], flags: ImGuiColorEditFlags) -> bool {
    let mut col4: [c_float; 4] = [col[0], col[1], col[2], 1.0];
    if !ColorPicker4(label, &mut col4, flags | ImGuiColorEditFlags_NoAlpha, 0.0) { return false; }
    col[0] = col4[0];
    col[1] = col4[1];
    col[2] = col4[2];
    return true;
}

// Helper for ColorPicker4()
pub unsafe fn RenderArrowsForVerticalBar(draw_list: *mut ImDrawList, pos: ImVec2, half_sz: ImVec2,bar_w: c_float,alpha: c_float)
{
    alpha8: u32 = IM_F32_TO_INT8_SAT(alpha);
    RenderArrowPointingAt(draw_list, ImVec2::new(pos.x + half_sz.x + 1,         pos.y), ImVec2::new(half_sz.x + 2, half_sz.y + 1), ImGuiDir_Right, IM_COL32(0,0,0,alpha8));
    RenderArrowPointingAt(draw_list, ImVec2::new(pos.x + half_sz.x,             pos.y), half_sz,                              ImGuiDir_Right, IM_COL32(255,255,255,alpha8));
    RenderArrowPointingAt(draw_list, ImVec2::new(pos.x + bar_w - half_sz.x - 1, pos.y), ImVec2::new(half_sz.x + 2, half_sz.y + 1), ImGuiDir_Left,  IM_COL32(0,0,0,alpha8));
    RenderArrowPointingAt(draw_list, ImVec2::new(pos.x + bar_w - half_sz.x,     pos.y), half_sz,                              ImGuiDir_Left,  IM_COL32(255,255,255,alpha8));
}

// Note: ColorPicker4() only accesses 3 floats if ImGuiColorEditFlags_NoAlpha flag is set.
// (In C++ the 'float col[4]' notation for a function argument is equivalent to 'float* col', we only specify a size to facilitate understanding of the code.)
// FIXME: we adjust the big color square height based on item width, which may cause a flickering feedback loop (if automatic height makes a vertical scrollbar appears, affecting automatic width..)
// FIXME: this is trying to be aware of style.Alpha but not fully correct. Also, the color wheel will have overlapping glitches with (style.Alpha < 1.0)
pub unsafe fn ColorPicker4(label: &str, col: &mut [c_float; 4], mut flags: ImGuiColorEditFlags, ref_col: c_float) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    if window.SkipItems { return  false; }

    draw_list: *mut ImDrawList = window.DrawList;
   let style = &mut g.Style;
    let io = &mut g.IO;

    let width: c_float =  CalcItemWidth();
    g.NextItemData.ClearFlags();

    PushID(label);
    BeginGroup();

    if flag_clear(flags, ImGuiColorEditFlags_NoSidePreview) {
        flags |= ImGuiColorEditFlags_NoSmallPreview;
    }

    // Context menu: display and store options.
    if flag_clear(flags, ImGuiColorEditFlags_NoOptions) {
        ColorPickerOptionsPopup(&col, flags);
    }

    // Read stored options
    if flag_clear(flags, ImGuiColorEditFlags_PickerMask_) {
        flags |= if g.ColorEditOptions & ImGuiColorEditFlags_PickerMask_ { g.ColorEditOptions } else { ImGuiColorEditFlags_DefaultOptions_ &ImGuiColorEditFlags_PickerMask_ };
    }
    if (flag_clear(flags, ImGuiColorEditFlags_InputMask_)) {
        flags |= if g.ColorEditOptions & ImGuiColorEditFlags_InputMask_ { g.ColorEditOptions } else { ImGuiColorEditFlags_DefaultOptions_ &ImGuiColorEditFlags_InputMask_ };
    }
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_PickerMask_)); // Check that only 1 is selected
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_InputMask_));  // Check that only 1 is selected
    if flag_clear(flags, ImGuiColorEditFlags_NoOptions) {
        flags |= (g.ColorEditOptions & ImGuiColorEditFlags_AlphaBar);
    }

    // Setup
    let components: c_int = if flags & ImGuiColorEditFlags_NoAlpha { 3} else { 4};
    let mut alpha_bar: bool =  flag_set(flags, ImGuiColorEditFlags_AlphaBar) && flag_clear(flags, ImGuiColorEditFlags_NoAlpha);
    let picker_pos: ImVec2 = window.DC.CursorPos;
    let square_sz: c_float =  GetFrameHeight();
    let bars_width: c_float =  square_sz; // Arbitrary smallish width of Hue/Alpha picking bars
    let sv_picker_size: c_float =  ImMax(bars_width * 1, width - (if alpha_bar {2} else { 1 }) * (bars_width + style.ItemInnerSpacing.x)); // Saturation/Value picking box
    let bar0_pos_x: c_float =  picker_pos.x + sv_picker_size + style.ItemInnerSpacing.x;
    let bar1_pos_x: c_float =  bar0_pos_x + bars_width + style.ItemInnerSpacing.x;
    let bars_triangles_half_sz: c_float =  IM_FLOOR(bars_width * 0.200);backup_initial_col: [c_float;4];
    memcpy(backup_initial_col, col, components * sizeof);

    let wheel_thickness: c_float =  sv_picker_size * 0.08;
    let wheel_r_outer: c_float =  sv_picker_size * 0.50;
    let wheel_r_inner: c_float =  wheel_r_outer - wheel_thickness;
    let wheel_center = ImVec2::from_floats(picker_pos.x + (sv_picker_size + bars_width)*0.5, picker_pos.y + sv_picker_size * 0.5);

    // Note: the triangle is displayed rotated with triangle_pa pointing to Hue, but most coordinates stays unrotated for logic.
    let triangle_r: c_float =  wheel_r_inner - (sv_picker_size * 0.0270f32);
    let triangle_pa: ImVec2 = ImVec2::from_floats(triangle_r, 0.0); // Hue point.
    let triangle_pb: ImVec2 = ImVec2::from_floats(triangle_r * -0.5, triangle_r * -0.8660250f32); // Black point.
    let triangle_pc: ImVec2 = ImVec2::from_floats(triangle_r * -0.5, triangle_r * 0.8660250f32); // White point.

    let mut H: c_float =  col[0]; let mut S = col[1]; let mut V = col[2];
    let mut R: c_float =  col[0]; let mut G = col[1]; let mut B = col[2];
    if flag_set(flags , ImGuiColorEditFlags_InputRGB)
    {
        // Hue is lost when converting from greyscale rgb (saturation=0). Restore it.
        ColorConvertRGBtoHSV(R, G, B, &mut H, &mut S, &mut V);
        ColorEditRestoreHS(col, &mut H, &mut S, &mut V);
    }
    else if flag_set(flags , ImGuiColorEditFlags_InputHSV)
    {
        ColorConvertHSVtoRGB(H, S, V, &mut R, &mut G, &mut B);
    }

    let mut value_changed: bool =  false;
    let mut value_changed_h = false;
    let mut value_changed_sv = false;

    PushItemFlag(ImGuiItemFlags_NoNav, true);
    if flag_set(flags , ImGuiColorEditFlags_PickerHueWheel)
    {
        // Hue wheel + SV triangle logic
        button_ops::InvisibleButton("hsv", ImVec2::new(sv_picker_size + style.ItemInnerSpacing.x + bars_width, sv_picker_size), 0);
        if IsItemActive()
        {
            let initial_off: ImVec2 = g.IO.MouseClickedPos[0] - wheel_center;
            let current_off: ImVec2 = g.IO.MousePos - wheel_center;
            let initial_dist2: c_float =  ImLengthSqr(initial_off);
            if initial_dist2 >= (wheel_r_inner - 1) * (wheel_r_inner - 1) && initial_dist2 <= (wheel_r_outer + 1) * (wheel_r_outer + 1)
            {
                // Interactive with Hue wheel
                H = ImAtan2(current_off.y, current_off.x) / IM_PI * 0.5;
                if H < 0.0 {
                    H += 1.0;
                }
                value_changed = true;
                value_changed_h = true;
            }
            let cos_hue_angle: c_float =  ImCos(-H * 2.0 * IM_PI);
            let sin_hue_angle: c_float =  ImSin(-H * 2.0 * IM_PI);
            if ImTriangleContainsPoint(&triangle_pa, &triangle_pb, &triangle_pc, &ImRotate(&initial_off, cos_hue_angle, sin_hue_angle))
            {
                // Interacting with SV triangle
                let mut current_off_unrotated: ImVec2 = ImRotate(&current_off, cos_hue_angle, sin_hue_angle);
                if !ImTriangleContainsPoint(&triangle_pa, &triangle_pb, &triangle_pc, &current_off_unrotated) {
                    current_off_unrotated = ImTriangleClosestPoint(&triangle_pa, &triangle_pb, &triangle_pc, &current_off_unrotated);
                    // uu: c_float, vv, ww;
                    let mut uu = 0f32;
                    let mut vv = 0f32;
                    let mut ww = 0f32;
                }
                ImTriangleBarycentricCoords(&triangle_pa, &triangle_pb, &triangle_pc, &current_off_unrotated, uu, vv, ww);
                V = ImClamp(1.0 - vv, 0.01, 1.0);
                S = ImClamp(uu / V, 0.01, 1.0);
                value_changed = true;value_changed_sv = true;
            }
        }
        if flag_clear(flags, ImGuiColorEditFlags_NoOptions) {
            OpenPopupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);
        }
    }
    else if flag_set(flags , ImGuiColorEditFlags_PickerHueBar)
    {
        // SV rectangle logic
        InvisibleButton("sv",
                        &mut ImVec2::from_floats(sv_picker_size, sv_picker_size),
                        0);
        if IsItemActive()
        {
            S = ImSaturate((io.MousePos.x - picker_pos.x) / (sv_picker_size - 1));
            V = 1.0 - ImSaturate((io.MousePos.y - picker_pos.y) / (sv_picker_size - 1));

            // Greatly reduces hue jitter and reset to 0 when hue == 255 and color is rapidly modified using SV square.
            if g.ColorEditLastColor == ColorConvertFloat4ToU32(ImVec4(col[0], col[1], col[2], 0)) {
                H = g.ColorEditLastHue;
            }
            value_changed = true;
            value_changed_sv = true;
        }
        if (flag_clear(flags, ImGuiColorEditFlags_NoOptions)) {
            OpenPopupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);
        }

        // Hue bar logic
        SetCursorScreenPos(ImVec2::new(bar0_pos_x, picker_pos.y));
        button_ops::InvisibleButton("hue", ImVec2::new(bars_width, sv_picker_size), 0);
        if IsItemActive()
        {
            H = ImSaturate((io.MousePos.y - picker_pos.y) / (sv_picker_size - 1));
            value_changed = true;
            value_changed_h = true;
        }
    }

    // Alpha bar logic
    if alpha_bar
    {
        SetCursorScreenPos(ImVec2::new(bar1_pos_x, picker_pos.y));
        button_ops::InvisibleButton("alpha", ImVec2::new(bars_width, sv_picker_size), 0);
        if IsItemActive()
        {
            col[3] = 1.0 - ImSaturate((io.MousePos.y - picker_pos.y) / (sv_picker_size - 1));
            value_changed = true;
        }
    }
    PopItemFlag(); // ImGuiItemFlags_NoNav

    if flag_clear(flags, ImGuiColorEditFlags_NoSidePreview)
    {
        SameLine(0.0, style.ItemInnerSpacing.x);
        BeginGroup();
    }

    if flag_clear(flags, ImGuiColorEditFlags_NoLabel)
    {
        let mut  label_display_end= FindRenderedTextEnd(label);
        if label != label_display_end
        {
            if flag_set(flags, ImGuiColorEditFlags_NoSidePreview) {
                SameLine(0.0, style.ItemInnerSpacing.x);
            }
            text_ops::TextEx(label, 0);
        }
    }

    if flag_clear(flags, ImGuiColorEditFlags_NoSidePreview)
    {
        PushItemFlag(ImGuiItemFlags_NoNavDefaultFocus, true);
        let mut col_v4 = ImVec4::from_floats(col[0], col[1], col[2], if flag_set(flags, ImGuiColorEditFlags_NoAlpha) { 1.0} else {col[3]});
        if flags & ImGuiColorEditFlags_NoLabel {
            text_ops::Text("Current");}

        sub_flags_to_forward: ImGuiColorEditFlags = ImGuiColorEditFlags_InputMask_ | ImGuiColorEditFlags_HDR | ImGuiColorEditFlags_AlphaPreview | ImGuiColorEditFlags_AlphaPreviewHalf | ImGuiColorEditFlags_NoTooltip;
        ColorButton("##current", col_v4, (flags & sub_flags_to_forward), ImVec2::new(square_sz * 3, square_sz * 2));
        if ref_col != c_float::MIN
        {
            text_ops::Text("Original");
            let mut ref_col_v4 = ImVec4::from_floats(ref_col[0], ref_col[1], ref_col[2], if flag_set(flags, ImGuiColorEditFlags_NoAlpha) { 1.0} else {ref_col[3]});
            if ColorButton("##original", ref_col_v4, (flags & sub_flags_to_forward), ImVec2::new(square_sz * 3, square_sz * 2))
            {
                // memcpy(col, ref_col, components * sizeof);
                value_changed = true;
            }
        }
        PopItemFlag();
        EndGroup();
    }

    // Convert back color to RGB
    if value_changed_h || value_changed_sv
    {
        if flag_set(flags , ImGuiColorEditFlags_InputRGB)
        {
            ColorConvertHSVtoRGB(H, S, V, &mut col[0], &mut col[1], &mut col[2]);
            g.ColorEditLastHue = H;
            g.ColorEditLastSat = S;
            g.ColorEditLastColor = ColorConvertFloat4ToU32(ImVec4(col[0], col[1], col[2], 0));
        }
        else if flag_set(flags , ImGuiColorEditFlags_InputHSV)
        {
            col[0] = H;
            col[1] = S;
            col[2] = V;
        }
    }

    // R,G,B and H,S,V slider color editor
    let mut value_changed_fix_hue_wrap: bool =  false;
    if flag_clear(flags, ImGuiColorEditFlags_NoInputs)
    {
        PushItemWidth((if alpha_bar {bar1_pos_x} else { bar0_pos_x }) + bars_width - picker_pos.x);
        sub_flags_to_forward: ImGuiColorEditFlags = ImGuiColorEditFlags_DataTypeMask_ | ImGuiColorEditFlags_InputMask_ | ImGuiColorEditFlags_HDR | ImGuiColorEditFlags_NoAlpha | ImGuiColorEditFlags_NoOptions | ImGuiColorEditFlags_NoSmallPreview | ImGuiColorEditFlags_AlphaPreview | ImGuiColorEditFlags_AlphaPreviewHalf;
        sub_flags: ImGuiColorEditFlags = flag_set(flags, sub_flags_to_forward) | ImGuiColorEditFlags_NoPicker;
        if flag_set(flags, ImGuiColorEditFlags_DisplayRGB) || flag_clear(flags, ImGuiColorEditFlags_DisplayMask_) {
            if ColorEdit4("##rgb", col, sub_flags | ImGuiColorEditFlags_DisplayRGB) {
                // FIXME: Hackily differentiating using the DragInt (ActiveId != 0 && !ActiveIdAllowOverlap) vs. using the InputText or DropTarget.
                // For the later we don't want to run the hue-wrap canceling code. If you are well versed in HSV picker please provide your input! (See #2050)
                value_changed_fix_hue_wrap = (g.ActiveId != 0 && !g.ActiveIdAllowOverlap);
                value_changed = true;
            }
        }
        if flag_set(flags , ImGuiColorEditFlags_DisplayHSV) || flag_clear(flags, ImGuiColorEditFlags_DisplayMask_) {
            value_changed |= ColorEdit4("##hsv", col, sub_flags | ImGuiColorEditFlags_DisplayHSV);
        }
        if flag_set( flags , ImGuiColorEditFlags_DisplayHex) || flag_clear(flags, ImGuiColorEditFlags_DisplayMask_) {
            value_changed |= ColorEdit4("##hex", col, sub_flags | ImGuiColorEditFlags_DisplayHex);
        }
        PopItemWidth();
    }

    // Try to cancel hue wrap (after ColorEdit4 call), if any
    if value_changed_fix_hue_wrap && flag_set(flags, ImGuiColorEditFlags_InputRGB) {
    // {new_H: c_float, new_S, new_V;
        let mut new_H:c_float = 0.0;
        let mut new_S:c_float = 0.0;
        let mut new_V:c_float = 0.0;
        ColorConvertRGBtoHSV(col[0], col[1], col[2], &mut new_H, &mut new_S, &mut new_V);
        if new_H <= 0.0 && H > 0.0
        {
            if new_V <= 0.0 && V != new_V {
                ColorConvertHSVtoRGB(H, S, if new_V <= 0.0 { V * 0.5 } else { new_V }, &mut col[0], &mut col[1], &mut col[2]);
            }
            else if new_S <= 0.0 {
                ColorConvertHSVtoRGB(H, if new_S <= 0.0 { S * 0.5 } else { new_S }, new_V, &mut col[0], &mut col[1], &mut col[2]);
            }
        }
    }

    if (value_changed)
    {
        if flag_set(flags , ImGuiColorEditFlags_InputRGB)
        {
            R = col[0];
            G = col[1];
            B = col[2];
            ColorConvertRGBtoHSV(R, G, B, &mut H, &mut S, &mut V);
            ColorEditRestoreHS(col, &mut H, &mut S, &mut V);   // Fix local Hue as display below will use it immediately.
        }
        else if flag_set(flags , ImGuiColorEditFlags_InputHSV)
        {
            H = col[0];
            S = col[1];
            V = col[2];
            ColorConvertHSVtoRGB(H, S, V, &mut R, &mut G, &mut B);
        }
    }

    let style_alpha8 = IM_F32_TO_INT8_SAT(style.Alpha);
    let col_black: u32 = IM_COL32(0,0,0,style_alpha8);
    let col_white: u32 = IM_COL32(255,255,255,style_alpha8);
    let col_midgrey: u32 = IM_COL32(128,128,128,style_alpha8);
    let col_hues: [u32;6 + 1] = [ IM_COL32(255,0,0,style_alpha8), IM_COL32(255,255,0,style_alpha8), IM_COL32(0,255,0,style_alpha8), IM_COL32(0,255,255,style_alpha8), IM_COL32(0,0,255,style_alpha8), IM_COL32(255,0,255,style_alpha8), IM_COL32(255,0,0,style_alpha8) ];

    let mut hue_color_f = ImVec4::from_floats(1.0, 1.0, 1.0, style.Alpha); ColorConvertHSVtoRGB(H, 1.0, 1.0, hue_color_f.x, hue_color_f.y, hue_color_f.z);
    hue_color32: u32 = ColorConvertFloat4ToU32(hue_color_0f32);
    user_col32_striped_of_alpha: u32 = ColorConvertFloat4ToU32(ImVec4(R, G, B, style.Alpha)); // Important: this is still including the main rendering/style alpha!!

    sv_cursor_pos: ImVec2;

    if flag_set(flags , ImGuiColorEditFlags_PickerHueWheel)
    {
        // Render Hue Wheel
        let aeps: c_float =  0.5 / wheel_r_outer; // Half a pixel arc length in radians (2pi cancels out).
        let segment_per_arc: c_int = ImMax(4, wheel_r_outer / 12);
        // for (let n: c_int = 0; n < 6; n++)
        for n in 0 .. 6
        {
            let a0: c_float =  (n)     /6f32 * 2.0 * IM_PI - aeps;
            let a1: c_float =  (n1f32)/6f32 * 2.0 * IM_PI + aeps;
            let vert_start_idx: c_int = draw_list.VtxBuffer.len();
            draw_list.PathArcTo(wheel_center, (wheel_r_inner + wheel_r_outer)*0.5, a0, a1, segment_per_arc);
            draw_list.PathStroke(col_white, 0, wheel_thickness);
            let vert_end_idx: c_int = draw_list.VtxBuffer.len();

            // Paint colors over existing vertices
            let mut gradient_p0 = ImVec2::from_floats(wheel_center.x + ImCos(a0) * wheel_r_inner, wheel_center.y + ImSin(a0) * wheel_r_inner);
            let mut gradient_p1 = ImVec2::from_floats(wheel_center.x + ImCos(a1) * wheel_r_inner, wheel_center.y + ImSin(a1) * wheel_r_inner);
            ShadeVertsLinearColorGradientKeepAlpha(draw_list, vert_start_idx, vert_end_idx, gradient_p0, gradient_p1, col_hues[n], col_hues[n + 1]);
        }

        // Render Cursor + preview on Hue Wheel
        let cos_hue_angle: c_float =  ImCos(H * 2.0 * IM_PI);
        let sin_hue_angle: c_float =  ImSin(H * 2.0 * IM_PI);
        let mut hue_cursor_pos = ImVec2::from_floats(wheel_center.x + cos_hue_angle * (wheel_r_inner + wheel_r_outer) * 0.5, wheel_center.y + sin_hue_angle * (wheel_r_inner + wheel_r_outer) * 0.5);
        let hue_cursor_rad: c_float = if value_changed_h { wheel_thickness * 0.65 }else {wheel_thickness * 0.55f32};
        let hue_cursor_segments: c_int = (hue_cursor_rad / 1.40).clamp(9.0, 32.0) as c_int;
        draw_list.AddCircleFilled(hue_cursor_pos, hue_cursor_rad, hue_color32, hue_cursor_segments);
        draw_list.AddCircle(hue_cursor_pos, hue_cursor_rad + 1, col_midgrey, hue_cursor_segments);
        draw_list.AddCircle(hue_cursor_pos, hue_cursor_rad, col_white, hue_cursor_segments);

        // Render SV triangle (rotated according to hue)
        let tra: ImVec2 = wheel_center + ImRotate(&triangle_pa, cos_hue_angle, sin_hue_angle);
        let trb: ImVec2 = wheel_center + ImRotate(&triangle_pb, cos_hue_angle, sin_hue_angle);
        let trc: ImVec2 = wheel_center + ImRotate(&triangle_pc, cos_hue_angle, sin_hue_angle);
        let uv_white: ImVec2 = GetFontTexUvWhitePixel();
        draw_list.PrimReserve(6, 6);
        draw_list.PrimVtx(tra, uv_white, hue_color32);
        draw_list.PrimVtx(trb, uv_white, hue_color32);
        draw_list.PrimVtx(trc, uv_white, col_white);
        draw_list.PrimVtx(tra, uv_white, 0);
        draw_list.PrimVtx(trb, uv_white, col_black);
        draw_list.PrimVtx(trc, uv_white, 0);
        draw_list.AddTriangle(tra, trb, trc, col_midgrey, 1.5);
        sv_cursor_pos = ImLerp(ImLerp(trc, tra, ImSaturate(S)), trb, ImSaturate(1 - V));
    }
    else if flag_set(flags , ImGuiColorEditFlags_PickerHueBar)
    {
        // Render SV Square
        draw_list.AddRectFilledMultiColor(picker_pos, picker_pos + ImVec2::new(sv_picker_size, sv_picker_size), col_white, hue_color32, hue_color32, col_white);
        draw_list.AddRectFilledMultiColor(picker_pos, picker_pos + ImVec2::new(sv_picker_size, sv_picker_size), 0, 0, col_black, col_black);
        RenderFrameBorder(picker_pos, picker_pos + ImVec2::new(sv_picker_size, sv_picker_size), 0.0);
        sv_cursor_pos.x = ImClamp(IM_ROUND(picker_pos.x + ImSaturate(S)     * sv_picker_size), picker_pos.x + 2, picker_pos.x + sv_picker_size - 2); // Sneakily prevent the circle to stick out too much
        sv_cursor_pos.y = ImClamp(IM_ROUND(picker_pos.y + ImSaturate(1 - V) * sv_picker_size), picker_pos.y + 2, picker_pos.y + sv_picker_size - 2);

        // Render Hue Bar
        // for (let i: c_int = 0; i < 6; ++i)
        for i in 0 .. 6
        {
            draw_list.AddRectFilledMultiColor(ImVec2::new(bar0_pos_x, picker_pos.y + i * (sv_picker_size / 6)), ImVec2::new(bar0_pos_x + bars_width, picker_pos.y + (i + 1) * (sv_picker_size / 6)), col_hues[i], col_hues[i], col_hues[i + 1], col_hues[i + 1]);
        }
        let bar0_line_y: c_float =  IM_ROUND(picker_pos.y + H * sv_picker_size);
        RenderFrameBorder(ImVec2::new(bar0_pos_x, picker_pos.y), ImVec2::new(bar0_pos_x + bars_width, picker_pos.y + sv_picker_size), 0.0);
        RenderArrowsForVerticalBar(draw_list, ImVec2::new(bar0_pos_x - 1, bar0_line_y), ImVec2::new(bars_triangles_half_sz + 1, bars_triangles_half_sz), bars_width + 2.0, style.Alpha);
    }

    // Render cursor/preview circle (clamp S/V within 0..1 range because floating points colors may lead HSV values to be out of range)
    let sv_cursor_rad: c_float =  if value_changed_sv {10.0} else { 6f32 };
    draw_list.AddCircleFilled(sv_cursor_pos, sv_cursor_rad, user_col32_striped_of_alpha, 12);
    draw_list.AddCircle(sv_cursor_pos, sv_cursor_rad + 1, col_midgrey, 12);
    draw_list.AddCircle(sv_cursor_pos, sv_cursor_rad, col_white, 12);

    // Render alpha bar
    if alpha_bar
    {
        let alpha: c_float =  ImSaturate(col[3]);
        let mut bar1_bb: ImRect = ImRect::new(bar1_pos_x, picker_pos.y, bar1_pos_x + bars_width, picker_pos.y + sv_picker_size);
        RenderColorRectWithAlphaCheckerboard(draw_list, bar1_bb.Min, bar1_bb.Max, 0, bar1_bb.GetWidth() / 2.0, ImVec2::new(0.0, 0.0), 0.0, 0);
        draw_list.AddRectFilledMultiColor(bar1_bb.Min, bar1_bb.Max, user_col32_striped_of_alpha, user_col32_striped_of_alpha, user_col32_striped_of_alpha & !IM_COL32_A_MASK, user_col32_striped_of_alpha & !IM_COL32_A_MASK);
        let bar1_line_y: c_float =  IM_ROUND(picker_pos.y + (1.0 - alpha) * sv_picker_size);
        RenderFrameBorder(bar1_bb.Min, bar1_bb.Max, 0.0);
        RenderArrowsForVerticalBar(draw_list, ImVec2::new(bar1_pos_x - 1, bar1_line_y), ImVec2::new(bars_triangles_half_sz + 1, bars_triangles_half_sz), bars_width + 2.0, style.Alpha);
    }

    EndGroup();

    if value_changed && memcmp(backup_initial_col, col, components * sizeof) == 0 {
        value_changed = false;}
    if value_changed{
        MarkItemEdited(g.LastItemData.ID);}

    PopID();

    return value_changed;
}

// A little color square. Return true when clicked.
// FIXME: May want to display/ignore the alpha component in the color display? Yet show it in the tooltip.
// 'desc_id' is not called 'label' because we don't display it next to the button, but only in the tooltip.
// Note that 'col' may be encoded in HSV if ImGuiColorEditFlags_InputHSV is set.
pub unsafe fn ColorButton(desc_id: &str, col: &ImVec4, mut flags: ImGuiColorEditFlags, size_arg: &ImVec2) -> bool
{
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    if window.SkipItems { return  false; }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut id: ImGuiID =  window.GetID(desc_id);
    let default_size: c_float =  GetFrameHeight();
    let mut size = ImVec2::from_floats(if size_arg.x == 0.0 { default_size} else {size_arg.x}, if size_arg.y == 0.0 { default_size }else{ size_arg.y});
    let mut bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + size);
    ItemSize(&bb.GetSize(), if size.y >= default_size { g.Style.FramePadding.y} else {0.0});
    if !ItemAdd(&mut bb, id, null(), 0) { return  false; }

    let mut hovered = false; let mut held = false;
    let mut pressed: bool =  ButtonBehavior(&bb, id, &mut hovered, &mut held, 0);

    if flag_set(flags , ImGuiColorEditFlags_NoAlpha) {
        flags &= !(ImGuiColorEditFlags_AlphaPreview | ImGuiColorEditFlags_AlphaPreviewHal0f32);
    }

    let mut col_rgb: ImVec4 = col.clone();
    if flag_set(flags , ImGuiColorEditFlags_InputHSV) {
        ColorConvertHSVtoRGB(col_rgb.x, col_rgb.y, col_rgb.z, &mut col_rgb.x, &mut col_rgb.y, &mut col_rgb.z);
    }

    let mut col_rgb_without_alpha = ImVec4::from_floats(col_rgb.x, col_rgb.y, col_rgb.z, 1.0);
    let grid_step: c_float = size.x.min(size.y) / 2.99;
    let rounding: c_float =  g.Style.FrameRounding.min(grid_step *0.5);
    let mut bb_inner: ImRect =  bb;
    let mut off: c_float =  0.0;
    if flag_clear(flags, ImGuiColorEditFlags_NoBorder)
    {
        off = -0.75; // The border (using Col_FrameBg) tends to look off when color is near-opaque and rounding is enabled. This offset seemed like a good middle ground to reduce those artifacts.
        bb_inner.Expand(off);
    }
    if flag_set(flags, ImGuiColorEditFlags_AlphaPreviewHal0f32) && col_rgb.w < 1.0
    {
        let mid_x: c_float =  IM_ROUND((bb_inner.Min.x + bb_inner.Max.x) * 0.5);
        RenderColorRectWithAlphaCheckerboard(window.DrawList, ImVec2::new(bb_inner.Min.x + grid_step, bb_inner.Min.y), bb_inner.Max, GetColorU32FromImVec4(&col_rgb), grid_step, ImVec2::new(-grid_step + off, off), rounding, ImDrawFlags_RoundCornersRight);
        window.DrawList.AddRectFilled(&bb_inner.Min, ImVec2::new(mid_x, bb_inner.Max.y), GetColorU32(col_rgb_without_alpha, 0.0), rounding, ImDrawFlags_RoundCornersLeft);
    }
    else
    {
        // Because GetColorU32() multiplies by the global style Alpha and we don't want to display a checkerboard if the source code had no alpha
        col_source: ImVec4 = if flags & ImGuiColorEditFlags_AlphaPreview { col_rgb} else { col_rgb_without_alpha};
        if (col_source.w < 1.0) {
            RenderColorRectWithAlphaCheckerboard(window.DrawList, bb_inner.Min, bb_inner.Max, GetColorU32(col_source, 0.0), grid_step, ImVec2::new(off, off), rounding, 0);
        }
        else {
            window.DrawList.AddRectFilled(&bb_inner.Min, &bb_inner.Max, GetColorU32(col_source, 0.0), rounding, 0);
        }
    }
    RenderNavHighlight(&bb, id, 0);
    if flag_clear(flags, ImGuiColorEditFlags_NoBorder)
    {
        if g.Style.FrameBorderSize > 0.0 {
            RenderFrameBorder(bb.Min, bb.Max, rounding);
        }
        else {
            window.DrawList.AddRect(&bb.Min, &bb.Max, GetColorU32(ImGuiCol_FrameBg, 0.0), rounding);
        }// Color button are often in need of some sort of border
    }

    // Drag and Drop Source
    // NB: The ActiveId test is merely an optional micro-optimization, BeginDragDropSource() does the same test.
    if g.ActiveId == id && flag_clear(flags, ImGuiColorEditFlags_NoDragDrop) && BeginDragDropSource(0)
    {
        if flag_set(flags , ImGuiColorEditFlags_NoAlpha) {
            SetDragDropPayload(IMGUI_PAYLOAD_TYPE_COLOR_3F, &col_rgb, sizeof * 3, ImGuiCond_Once);
        }
        else {
            SetDragDropPayload(IMGUI_PAYLOAD_TYPE_COLOR_4F, &col_rgb, sizeof * 4, ImGuiCond_Once);
        }
        ColorButton(desc_id, col, flags);
        SameLine(0.0, 0.0);
        text_ops::TextEx("Color", 0);
        EndDragDropSource();
    }

    // Tooltip
    if flag_clear(flags, ImGuiColorEditFlags_NoTooltip) && hovered
        ColorTooltip(desc_id, col.x, flags & (ImGuiColorEditFlags_InputMask_ | ImGuiColorEditFlags_NoAlpha | ImGuiColorEditFlags_AlphaPreview | ImGuiColorEditFlags_AlphaPreviewHal0f32));

    return pressed;
}

// Initialize/override default color options
pub unsafe fn SetColorEditOptions(flags: ImGuiColorEditFlags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (flag_clear(flags, ImGuiColorEditFlags_DisplayMask_))
        flags |= ImGuiColorEditFlags_DefaultOptions_ & ImGuiColorEditFlags_DisplayMask_;
    if (flag_clear(flags, ImGuiColorEditFlags_DataTypeMask_))
        flags |= ImGuiColorEditFlags_DefaultOptions_ & ImGuiColorEditFlags_DataTypeMask_;
    if (flag_clear(flags, ImGuiColorEditFlags_PickerMask_))
        flags |= ImGuiColorEditFlags_DefaultOptions_ & ImGuiColorEditFlags_PickerMask_;
    if (flag_clear(flags, ImGuiColorEditFlags_InputMask_))
        flags |= ImGuiColorEditFlags_DefaultOptions_ & ImGuiColorEditFlags_InputMask_;
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_DisplayMask_));    // Check only 1 option is selected
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_DataTypeMask_));   // Check only 1 option is selected
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_PickerMask_));     // Check only 1 option is selected
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_InputMask_));      // Check only 1 option is selected
    g.ColorEditOptions = flags;
}

// Note: only access 3 floats if ImGuiColorEditFlags_NoAlpha flag is set.
pub unsafe fn ColorTooltip(text: &str, *col: c_float, flags: ImGuiColorEditFlags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    BeginTooltipEx(ImGuiTooltipFlags_OverridePreviousTooltip, ImGuiWindowFlags_None);
    let mut  text_end: &str = if text { FindRenderedTextEnd(text, null_mut()) }else {text};
    if (text_end > text)
    {
        text_ops::TextEx(text, text_end);
        separator::Separator();
    }

    sz: ImVec2(g.FontSize * 3 + g.Style.FramePadding.y * 2, g.FontSize * 3 + g.Style.FramePadding.y * 2);
    let mut cf = ImVec4::new(col[0], col[1], col[2], if flag_set(flags, ImGuiColorEditFlags_NoAlpha) { 1.0} else {col[3]});
    let cr: c_int = IM_F32_TO_INT8_SAT(col[0]), cg = IM_F32_TO_INT8_SAT(col[1]), cb = IM_F32_TO_INT8_SAT(col[2]), ca = if flags & ImGuiColorEditFlags_NoAlpha { 255} else { IM_F32_TO_INT8_SAT(col[3])};
    ColorButton("##preview", cf, (flags & (ImGuiColorEditFlags_InputMask_ | ImGuiColorEditFlags_NoAlpha | ImGuiColorEditFlags_AlphaPreview | ImGuiColorEditFlags_AlphaPreviewHal0f32)) | ImGuiColorEditFlags_NoTooltip, sz);
    SameLine();
    if (flag_set(flags, ImGuiColorEditFlags_InputRGB) || flag_clear(flags, ImGuiColorEditFlags_InputMask_))
    {
        if (flags & ImGuiColorEditFlags_NoAlpha)
            text_ops::Text("#%02X%02X%02X\nR: %d, G: %d, B: %d\n(%.3f, %.3f, %.30f32)", cr, cg, cb, cr, cg, cb, col[0], col[1], col[2]);
        else
            text_ops::Text("#%02X%02X%02X%02X\nR:%d, G:%d, B:%d, A:%d\n(%.3f, %.3f, %.3f, %.30f32)", cr, cg, cb, ca, cr, cg, cb, ca, col[0], col[1], col[2], col[3]);
    }
    else if (flags & ImGuiColorEditFlags_InputHSV)
    {
        if (flags & ImGuiColorEditFlags_NoAlpha)
            text_ops::Text("H: %.3f, S: %.3f, V: %.3f", col[0], col[1], col[2]);
        else
            text_ops::Text("H: %.3f, S: %.3f, V: %.3f, A: %.3f", col[0], col[1], col[2], col[3]);
    }
    EndTooltip();
}

pub unsafe fn ColorEditOptionsPopup(col: &[c_float], flags: ImGuiColorEditFlags)
{
    let mut allow_opt_inputs: bool =  flag_clear(flags, ImGuiColorEditFlags_DisplayMask_);
    let mut allow_opt_datatype: bool =  flag_clear(flags, ImGuiColorEditFlags_DataTypeMask_);
    if (!allow_opt_inputs && !allow_opt_datatype) || !BeginPopup("context") { return ; }
    let g = GImGui; // ImGuiContext& g = *GImGui;
    opts: ImGuiColorEditFlags = g.ColorEditOptions;
    if (allow_opt_inputs)
    {
        if (radio_button::RadioButton("RGB", (opts & ImGuiColorEditFlags_DisplayRGB) != 0)) opts = (opts & !ImGuiColorEditFlags_DisplayMask_) | ImGuiColorEditFlags_DisplayRGB;
        if (radio_button::RadioButton("HSV", (opts & ImGuiColorEditFlags_DisplayHSV) != 0)) opts = (opts & !ImGuiColorEditFlags_DisplayMask_) | ImGuiColorEditFlags_DisplayHSV;
        if (radio_button::RadioButton("Hex", (opts & ImGuiColorEditFlags_DisplayHex) != 0)) opts = (opts & !ImGuiColorEditFlags_DisplayMask_) | ImGuiColorEditFlags_DisplayHex;
    }
    if (allow_opt_datatype)
    {
        if allow_opt_inputs {  separator::Separator(); }
        if (radio_button::RadioButton("0..255", (opts & ImGuiColorEditFlags_Uint8) != 0)) opts = (opts & !ImGuiColorEditFlags_DataTypeMask_) | ImGuiColorEditFlags_Uint8;
        if (radio_button::RadioButton("0.00..1.00", (opts & ImGuiColorEditFlags_Float) != 0)) opts = (opts & !ImGuiColorEditFlags_DataTypeMask_) | ImGuiColorEditFlags_Float;
    }

    if allow_opt_inputs || allow_opt_datatype {
        separator::Separator(); }
    if (button_ops::Button("Copy as..", ImVec2::new(-1, 0)))
        OpenPopup("Copy");
    if (BeginPopup("Copy"))
    {
        let cr: c_int = IM_F32_TO_INT8_SAT(col[0]), cg = IM_F32_TO_INT8_SAT(col[1]), cb = IM_F32_TO_INT8_SAT(col[2]), ca = if flags & ImGuiColorEditFlags_NoAlpha { 255} else { IM_F32_TO_INT8_SAT(col[3])};
        buf: [c_char;64];
        ImFormatString(buf, buf.len(), "(%.3ff, %.3ff, %.3ff, %.3f0f32)", col[0], col[1], col[2], if flag_set(flags, ImGuiColorEditFlags_NoAlpha) { 1.0} else {col[3]});
        if Selectable(buf) {
            SetClipboardText(buf)(); }
        ImFormatString(buf, buf.len(), "(%d,%d,%d,%d)", cr, cg, cb, ca);
        if Selectable(buf) {
            SetClipboardText(buf)(); }
        ImFormatString(buf, buf.len(), "#%02X%02X%02X", cr, cg, cb);
        if Selectable(buf) {
            SetClipboardText(buf)(); }
        if (flag_clear(flags, ImGuiColorEditFlags_NoAlpha))
        {
            ImFormatString(buf, buf.len(), "#%02X%02X%02X%02X", cr, cg, cb, ca);
            if Selectable(buf) {
                SetClipboardText(buf)(); }
        }
        EndPopup();
    }

    g.ColorEditOptions = opts;
    EndPopup();
}

pub unsafe fn ColorPickerOptionsPopup(ref_col: &[c_float], flags: ImGuiColorEditFlags)
{
    let mut allow_opt_picker: bool =  flag_clear(flags, ImGuiColorEditFlags_PickerMask_);
    let mut allow_opt_alpha_bar: bool =  flag_clear(flags, ImGuiColorEditFlags_NoAlpha) && flag_clear(flags, ImGuiColorEditFlags_AlphaBar);
    if (!allow_opt_picker && !allow_opt_alpha_bar) || !BeginPopup("context") { return ; }
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (allow_opt_picker)
    {
        let picker_size= ImVec2::from_floats(g.FontSize * 8, ImMax(g.FontSize * 8 - (GetFrameHeight() + g.Style.ItemInnerSpacing.x), 1.0)); // FIXME: Picker size copied from main picker function
        PushItemWidth(picker_size.x);
        // for (let picker_type: c_int = 0; picker_type < 2; picker_type++)
        for picker_type in 0 .. 2
        {
            // Draw small/thumbnail version of each picker type (over an invisible button for selection)
            if picker_type > 0 {  separator::Separator(); }
            PushID(picker_type);
            picker_flags: ImGuiColorEditFlags = ImGuiColorEditFlags_NoInputs | ImGuiColorEditFlags_NoOptions | ImGuiColorEditFlags_NoLabel | ImGuiColorEditFlags_NoSidePreview | (flags & ImGuiColorEditFlags_NoAlpha);
            if (picker_type == 0) { picker_flags |= ImGuiColorEditFlags_PickerHueBar; }
            if (picker_type == 1) { picker_flags |= ImGuiColorEditFlags_PickerHueWheel; }
            let backup_pos: ImVec2 = GetCursorScreenPos();
            if (Selectable("##selectable", false, 0, picker_size)) { // By default, Selectable() is closing popup
                g.ColorEditOptions = (g.ColorEditOptions & !ImGuiColorEditFlags_PickerMask_) | (picker_flags & ImGuiColorEditFlags_PickerMask_);
            }
            SetCursorScreenPos(&backup_pos);
            previewing_ref_col: ImVec4;
            // memcpy(&previewing_ref_col, ref_col, sizeof * (if (picker_flags & ImGuiColorEditFlags_NoAlpha) { 3 }else {4}));
            ColorPicker4("##previewing_picker", &previewing_ref_col.x, picker_flags, 0.0);
            PopID();
        }
        PopItemWidth();
    }
    if allow_opt_alpha_bar
    {
        if allow_opt_picker {  separator::Separator(); }
        CheckboxFlags("Alpha Bar", &mut g.ColorEditOptions, ImGuiColorEditFlags_AlphaBar);
    }
    EndPopup();
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: TreeNode, CollapsingHeader, etc.
//-------------------------------------------------------------------------
// - TreeNode()
// - TreeNodeV()
// - TreeNodeEx()
// - TreeNodeExV()
// - TreeNodeBehavior() [Internal]
// - TreePush()
// - TreePop()
// - GetTreeNodeToLabelSpacing()
// - SetNextItemOpen()
// - CollapsingHeader()
//-------------------------------------------------------------------------

pub unsafe fn TreeNode(str_id: &str, fmt: &str, ...) -> bool
{
    va_list args;
    va_start(args, fmt);
    let mut is_open: bool =  TreeNodeExV(str_id, 0, fmt, args);
    va_end(args);
    return is_open;
}

pub unsafe fn TreeNode(ptr_id: *const c_void, fmt: &str, ...) -> bool
{
    va_list args;
    va_start(args, fmt);
    let mut is_open: bool =  TreeNodeExV(ptr_id, 0, fmt, args);
    va_end(args);
    return is_open;
}

pub unsafe fn TreeNode(label: &str) -> bool
{
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    if window.SkipItems { return  false; }
    return TreeNodeBehavior(window.GetID(label), 0, label, null_mut());
}

pub unsafe fn TreeNodeV(str_id: &str, fmt: &str, va_list args) -> bool
{
    return TreeNodeExV(str_id, 0, fmt, args);
}

pub unsafe fn TreeNodeV(ptr_id: *const c_void, fmt: &str, va_list args) -> bool
{
    return TreeNodeExV(ptr_id, 0, fmt, args);
}

pub unsafe fn TreeNodeEx(label: &str, ImGuiTreeNodeFlags flags) -> bool
{
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    if window.SkipItems { return  false; }

    return TreeNodeBehavior(window.GetID(label), flags, label, null_mut());
}

pub unsafe fn TreeNodeEx(str_id: &str, ImGuiTreeNodeFlags flags, fmt: &str, ...) -> bool
{
    va_list args;
    va_start(args, fmt);
    let mut is_open: bool =  TreeNodeExV(str_id, flags, fmt, args);
    va_end(args);
    return is_open;
}

pub unsafe fn TreeNodeEx(ptr_id: *const c_void, ImGuiTreeNodeFlags flags, fmt: &str, ...) -> bool
{
    va_list args;
    va_start(args, fmt);
    let mut is_open: bool =  TreeNodeExV(ptr_id, flags, fmt, args);
    va_end(args);
    return is_open;
}

pub unsafe fn TreeNodeExV(str_id: &str, ImGuiTreeNodeFlags flags, fmt: &str, va_list args) -> bool
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if window.SkipItems { return  false; }

    label: &str, *label_end;
    ImFormatStringToTempBufferV(&label, &label_end, fmt, args);
    return TreeNodeBehavior(window.GetID(str_id), flags, label, label_end);
}

pub unsafe fn TreeNodeExV(ptr_id: *const c_void, ImGuiTreeNodeFlags flags, fmt: &str, va_list args) -> bool
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if window.SkipItems { return  false; }

    label: &str, *label_end;
    ImFormatStringToTempBufferV(&label, &label_end, fmt, args);
    return TreeNodeBehavior(window.GetID(ptr_id), flags, label, label_end);
}

pub unsafe fn TreeNodeSetOpen(id: ImGuiID, open: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiStorage* storage = g.Currentwindow.DC.StateStorage;
    storage.SetInt(id, if open {1} else { 0 });
}

pub unsafe fn TreeNodeUpdateNextOpen(id: ImGuiID, ImGuiTreeNodeFlags flags) -> bool
{
    if flags & ImGuiTreeNodeFlags_Lea0f32 { return  true; }

    // We only write to the tree storage if the user clicks (or explicitly use the SetNextItemOpen function)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    ImGuiStorage* storage = window.DC.StateStorage;

    is_open: bool;
    if (g.NextItemData.Flags & ImGuiNextItemDataFlags_HasOpen)
    {
        if (g.NextItemData.OpenCond & ImGuiCond_Always)
        {
            is_open = g.NextItemData.OpenVal;
            TreeNodeSetOpen(id, is_open);
        }
        else
        {
            // We treat ImGuiCond_Once and ImGuiCond_FirstUseEver the same because tree node state are not saved persistently.
            let stored_value: c_int = storage.GetInt(id, -1);
            if (stored_value == -1)
            {
                is_open = g.NextItemData.OpenVal;
                TreeNodeSetOpen(id, is_open);
            }
            else
            {
                is_open = stored_value != 0;
            }
        }
    }
    else
    {
        is_open = storage.GetInt(id, if flag_set(flags, ImGuiTreeNodeFlags_DefaultOpen) { 1} else {0}) != 0;
    }

    // When logging is enabled, we automatically expand tree nodes (but *NOT* collapsing headers.. seems like sensible behavior).
    // NB- If we are above max depth we still allow manually opened nodes to be logged.
    if g.LogEnabled && flag_clear(flags, ImGuiTreeNodeFlags_NoAutoOpenOnLog) && (window.DC.TreeDepth - g.LogDepthRe0f32) < g.LogDepthToExpand {
        is_open = true;}

    return is_open;
}

pub unsafe fn TreeNodeBehavior(id: ImGuiID, ImGuiTreeNodeFlags flags, label: &str, label_end: &str) -> bool
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if window.SkipItems { return  false; }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let setyle = &mut g.Style;
    let display_frame: bool = flag_set(flags, ImGuiTreeNodeFlags_Framed);
    let padding: ImVec2 = if display_frame || flag_set(flags, ImGuiTreeNodeFlags_FramePadding) { style.FramePadding} else { ImVec2::new(style.FramePadding.x, ImMin(window.DC.CurrLineTextBaseOffset, style.FramePadding.y))};

    if (!label_end)
        label_end = FindRenderedTextEnd(label);
    let label_size: ImVec2 = CalcTextSize(label, label_end, false);

    // We vertically grow up to current line height up the typical widget height.
    let frame_height: c_float =  ImMax(ImMin(window.DC.CurrLineSize.y, g.FontSize + style.FramePadding.y * 2), label_size.y + padding.y * 2);
    let mut frame_bb: ImRect = ImRect::default();
    frame_bb.Min.x = if flags & ImGuiTreeNodeFlags_SpanFullWidth { window.WorkRect.Min.x} else { window.DC.CursorPos.x};
    frame_bb.Min.y = window.DC.CursorPos.y;
    frame_bb.Max.x = window.WorkRect.Max.x;
    frame_bb.Max.y = window.DC.CursorPos.y + frame_height;
    if (display_frame)
    {
        // Framed header expand a little outside the default padding, to the edge of InnerClipRect
        // (FIXME: May remove this at some point and make InnerClipRect align with WindowPadding.x instead of WindowPadding.x*0.5)
        frame_bb.Min.x -= IM_FLOOR(window.WindowPadding.x * 0.5 - 1.0);
        frame_bb.Max.x += IM_FLOOR(window.WindowPadding.x * 0.5);
    }

    let text_offset_x: c_float =  g.FontSize + (if display_frame { padding.x * 3 }else {padding.x * 2});           // Collapser arrow width + Spacing
    let text_offset_y: c_float =  ImMax(padding.y, window.DC.CurrLineTextBaseOffset);                    // Latch before ItemSize changes it
    let text_width: c_float =  g.FontSize + (if label_size.x > 0.0 { label_size.x + padding.x * 2} else {0.0});  // Include collapser
    text_pos: ImVec2(window.DC.CursorPos.x + text_offset_x, window.DC.CursorPos.y + text_offset_y);
    ItemSize(ImVec2::new(text_width, frame_height), padding.y);

    // For regular tree nodes, we arbitrary allow to click past 2 worth of ItemSpacing
    let interact_bb: ImRect =  frame_bb;
    if (!display_frame && (flags & (ImGuiTreeNodeFlags_SpanAvailWidth | ImGuiTreeNodeFlags_SpanFullWidth)) == 0)
        interact_bb.Max.x = frame_bb.Min.x + text_width + style.ItemSpacing.x * 2.0;

    // Store a flag for the current depth to tell if we will allow closing this node when navigating one of its child.
    // For this purpose we essentially compare if g.NavIdIsAlive went from 0 to 1 between TreeNode() and TreePop().
    // This is currently only support 32 level deep and we are fine with (1 << Depth) overflowing into a zero.
    let is_leaf: bool = flag_set(flags, ImGuiTreeNodeFlags_Lea0f32);
    let mut is_open: bool =  TreeNodeUpdateNextOpen(id, flags);
    if (is_open && !g.NavIdIsAlive && flag_set(flags, ImGuiTreeNodeFlags_NavLeftJumpsBackHere) && flag_clear(flags, ImGuiTreeNodeFlags_NoTreePushOnOpen))
        window.DC.TreeJumpToParentOnPopMask |= (1 << window.DC.TreeDepth);

    let mut item_add: bool =  ItemAdd(interact_bb, id);
    g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_HasDisplayRect;
    g.LastItemData.DisplayRect = frame_bb;

    if (!item_add)
    {
        if (is_open && flag_clear(flags, ImGuiTreeNodeFlags_NoTreePushOnOpen))
            TreePushOverrideID(id);
        IMGUI_TEST_ENGINE_ITEM_INFO(g.LastItemData.ID, label, g.LastItemData.StatusFlags | (if is_leaf {0} else { ImGuiItemStatusFlags_Openable }) | (if is_open {ImGuiItemStatusFlags_Opened} else { 0 }));
        return is_open;
    }

    button_flags: ImGuiButtonFlags = ImGuiTreeNodeFlags_None;
    if (flags & ImGuiTreeNodeFlags_AllowItemOverlap)
        button_flags |= ImGuiButtonFlags_AllowItemOverlap;
    if (!is_lea0f32)
        button_flags |= ImGuiButtonFlags_PressedOnDragDropHold;

    // We allow clicking on the arrow section with keyboard modifiers held, in order to easily
    // allow browsing a tree while preserving selection with code implementing multi-selection patterns.
    // When clicking on the rest of the tree node we always disallow keyboard modifiers.
    let arrow_hit_x1: c_float =  (text_pos.x - text_offset_x) - style.TouchExtraPadding.x;
    let arrow_hit_x2: c_float =  (text_pos.x - text_offset_x) + (g.FontSize + padding.x * 2.0) + style.TouchExtraPadding.x;
    let is_mouse_x_over_arrow: bool = (g.IO.MousePos.x >= arrow_hit_x1 && g.IO.MousePos.x < arrow_hit_x2);
    if (window != g.HoveredWindow || !is_mouse_x_over_arrow)
        button_flags |= ImGuiButtonFlags_NoKeyModifiers;

    // Open behaviors can be altered with the _OpenOnArrow and _OnOnDoubleClick flags.
    // Some alteration have subtle effects (e.g. toggle on MouseUp vs MouseDown events) due to requirements for multi-selection and drag and drop support.
    // - Single-click on label = Toggle on MouseUp (default, when _OpenOnArrow=0)
    // - Single-click on arrow = Toggle on MouseDown (when _OpenOnArrow=0)
    // - Single-click on arrow = Toggle on MouseDown (when _OpenOnArrow=1)
    // - Double-click on label = Toggle on MouseDoubleClick (when _OpenOnDoubleClick=1)
    // - Double-click on arrow = Toggle on MouseDoubleClick (when _OpenOnDoubleClick=1 and _OpenOnArrow=0)
    // It is rather standard that arrow click react on Down rather than Up.
    // We set ImGuiButtonFlags_PressedOnClickRelease on OpenOnDoubleClick because we want the item to be active on the initial MouseDown in order for drag and drop to work.
    if (is_mouse_x_over_arrow)
        button_flags |= ImGuiButtonFlags_PressedOnClick;
    else if (flags & ImGuiTreeNodeFlags_OpenOnDoubleClick)
        button_flags |= ImGuiButtonFlags_PressedOnClickRelease | ImGuiButtonFlags_PressedOnDoubleClick;
    else
        button_flags |= ImGuiButtonFlags_PressedOnClickRelease;

    let mut selected: bool =  flag_set(flags, ImGuiTreeNodeFlags_Selected);
    let was_selected: bool = selected;

    let mut hovered = false; let mut held = false;
    let mut pressed: bool =  button_ops::ButtonBehavior(interact_bb, id, &hovered, &held, button_flags);
    let mut toggled: bool =  false;
    if (!is_lea0f32)
    {
        if (pressed && g.DragDropHoldJustPressedId != id)
        {
            if (flags & (ImGuiTreeNodeFlags_OpenOnArrow | ImGuiTreeNodeFlags_OpenOnDoubleClick)) == 0 || (g.NavActivateId == id) {
                toggled = true;}
            if (flags & ImGuiTreeNodeFlags_OpenOnArrow)
                toggled |= is_mouse_x_over_arrow && !g.NavDisableMouseHover; // Lightweight equivalent of IsMouseHoveringRect() since ButtonBehavior() already did the job
            if flag_set(flags, ImGuiTreeNodeFlags_OpenOnDoubleClick) && g.IO.MouseClickedCount[0] == 2 {
                toggled = true;}
        }
        else if (pressed && g.DragDropHoldJustPressedId == id)
        {
            // IM_ASSERT(button_flags & ImGuiButtonFlags_PressedOnDragDropHold);
            if (!is_open) // When using Drag and Drop "hold to open" we keep the node highlighted after opening, but never close it again.
                toggled = true;
        }

        if (g.NavId == id && g.NavMoveDir == ImGuiDir_Left && is_open)
        {
            toggled = true;
            NavMoveRequestCancel();
        }
        if (g.NavId == id && g.NavMoveDir == ImGuiDir_Right && !is_open) // If there's something upcoming on the line we may want to give it the priority?
        {
            toggled = true;
            NavMoveRequestCancel();
        }

        if (toggled)
        {
            is_open = !is_open;
            window.DC.StateStorage.SetInt(id, is_open);
            g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_ToggledOpen;
        }
    }
    if flags & ImGuiTreeNodeFlags_AllowItemOverlap {
        SetItemAllowOverlap(); }

    // In this branch, TreeNodeBehavior() cannot toggle the selection so this will never trigger.
    if (selected != was_selected) //-V547
        g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_ToggledSelection;

    // Render
    text_col: u32 = GetColorU32(ImGuiCol_Text, 0.0);
    ImGuiNavHighlightFlags nav_highlight_flags = ImGuiNavHighlightFlags_TypeThin;
    if (display_frame)
    {
        // Framed type
        bg_col: u32 = GetColorU32(if (held && hovered) { ImGuiCol_HeaderActive} else {if hovered {ImGuiCol_HeaderHovered} else { ImGuiCol_Header }});
        RenderFrame(frame_bb.Min, frame_bb.Max, bg_col, true, style.FrameRounding);
        RenderNavHighlight(frame_bb, id, nav_highlight_flags);
        if (flags & ImGuiTreeNodeFlags_Bullet)
            RenderBullet(window.DrawList, ImVec2::new(text_pos.x - text_offset_x * 0.60, text_pos.y + g.FontSize * 0.5), text_col);
        else if (!is_lea0f32)
            RenderArrow(window.DrawList, ImVec2::new(text_pos.x - text_offset_x + padding.x, text_pos.y), text_col, if is_open {ImGuiDir_Down} else { ImGuiDir_Right }, 1.0);
        else // Leaf without bullet, left-adjusted text
            text_pos.x -= text_offset_x;
        if (flags & ImGuiTreeNodeFlags_ClipLabelForTrailingButton)
            frame_bb.Max.x -= g.FontSize + style.FramePadding.x;

        if (g.LogEnabled)
            LogSetNextTextDecoration("###", "###");
        RenderTextClipped(text_pos, frame_bb.Max, label, label_end, &label_size);
    }
    else
    {
        // Unframed typed for tree nodes
        if (hovered || selected)
        {
            bg_col: u32 = GetColorU32(if (held && hovered) { ImGuiCol_HeaderActive} else { if hovered {ImGuiCol_HeaderHovered} else { ImGuiCol_Header }});
            RenderFrame(frame_bb.Min, frame_bb.Max, bg_col, false);
        }
        RenderNavHighlight(frame_bb, id, nav_highlight_flags);
        if (flags & ImGuiTreeNodeFlags_Bullet)
            RenderBullet(window.DrawList, ImVec2::new(text_pos.x - text_offset_x * 0.5, text_pos.y + g.FontSize * 0.5), text_col);
        else if (!is_lea0f32)
            RenderArrow(window.DrawList, ImVec2::new(text_pos.x - text_offset_x + padding.x, text_pos.y + g.FontSize * 0.150f32), text_col, if is_open {ImGuiDir_Down} else { ImGuiDir_Right }, 0.70);
        if (g.LogEnabled)
            LogSetNextTextDecoration(">", null_mut());
        RenderText(text_pos, label, label_end, false);
    }

    if (is_open && flag_clear(flags, ImGuiTreeNodeFlags_NoTreePushOnOpen))
        TreePushOverrideID(id);
    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags | (if is_leaf {0} else { ImGuiItemStatusFlags_Openable }) | (if is_open {ImGuiItemStatusFlags_Opened} else { 0 }));
    return is_open;
}

pub unsafe fn TreePush(str_id: &str)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    Indent();
    window.DC.TreeDepth+= 1;
    PushID(str_id);
}

pub unsafe fn TreePush(ptr_id: *const c_void)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    Indent();
    window.DC.TreeDepth+= 1;
    PushID(ptr_id);
}

pub unsafe fn TreePushOverrideID(id: ImGuiID)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    Indent();
    window.DC.TreeDepth+= 1;
    PushOverrideID(id);
}

pub unsafe fn TreePop()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    Unindent();

    window.DC.TreeDepth-= 1;
    tree_depth_mask: u32 = (1 << window.DC.TreeDepth);

    // Handle Left arrow to move to parent tree node (when ImGuiTreeNodeFlags_NavLeftJumpsBackHere is enabled)
    if (g.NavMoveDir == ImGuiDir_Left && g.NavWindow == window && NavMoveRequestButNoResultYet())
        if (g.NavIdIsAlive && (window.DC.TreeJumpToParentOnPopMask & tree_depth_mask))
        {
            SetNavID(window.IDStack.last().unwrap(), g.NavLayer, 0, ImRect());
            NavMoveRequestCancel();
        }
    window.DC.TreeJumpToParentOnPopMask &= tree_depth_mask - 1;

    // IM_ASSERT(window.IDStack.Size > 1); // There should always be 1 element in the IDStack (pushed during window creation). If this triggers you called TreePop/PopID too much.
    PopID();
}

// Horizontal distance preceding label when using TreeNode() or Bullet()GetTreeNodeToLabelSpacing: c_float()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize + (g.Style.FramePadding.x * 2.0);
}

// Set next TreeNode/CollapsingHeader open state.
pub unsafe fn SetNextItemOpen(is_open: bool, cond: ImGuiCond)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.Currentwindow.SkipItems { return ; }
    g.NextItemData.Flags |= ImGuiNextItemDataFlags_HasOpen;
    g.NextItemData.OpenVal = is_open;
    g.NextItemData.OpenCond = if cond {cond} else { ImGuiCond_Always };
}

// CollapsingHeader returns true when opened but do not indent nor push into the ID stack (because of the ImGuiTreeNodeFlags_NoTreePushOnOpen flag).
// This is basically the same as calling TreeNodeEx(label, ImGuiTreeNodeFlags_CollapsingHeader). You can remove the _NoTreePushOnOpen flag if you want behavior closer to normal TreeNode().
pub unsafe fn CollapsingHeader(label: &str, ImGuiTreeNodeFlags flags) -> bool
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if window.SkipItems { return  false; }

    return TreeNodeBehavior(window.GetID(label), flags | ImGuiTreeNodeFlags_CollapsingHeader, label);
}

// p_visible == NULL                        : regular collapsing header
// p_visible != NULL && *p_visible == true  : show a small close button on the corner of the header, clicking the button will set *p_visible = false
// p_visible != NULL && *p_visible == false : do not show the header at all
// Do not mistake this with the Open state of the header itself, which you can adjust with SetNextItemOpen() or ImGuiTreeNodeFlags_DefaultOpen.
pub unsafe fn CollapsingHeader(label: &str,p_visible: *mut bool, ImGuiTreeNodeFlags flags) -> bool
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if window.SkipItems { return  false; }

    if p_visible && !*p_visible { return  false; }

    let mut id: ImGuiID =  window.GetID(label);
    flags |= ImGuiTreeNodeFlags_CollapsingHeader;
    if (p_visible)
        flags |= ImGuiTreeNodeFlags_AllowItemOverlap | ImGuiTreeNodeFlags_ClipLabelForTrailingButton;
    let mut is_open: bool =  TreeNodeBehavior(id, flags, label);
    if (p_visible != null_mut())
    {
        // Create a small overlapping close button
        // FIXME: We can evolve this into user accessible helpers to add extra buttons on title bars, headers, etc.
        // FIXME: CloseButton can overlap into text, need find a way to clip the text somehow.
        let g = GImGui; // ImGuiContext& g = *GImGui;
        last_item_backup: ImGuiLastItemData = g.LastItemData;
        let button_size: c_float =  g.FontSize;
        let button_x: c_float =  ImMax(g.LastItemData.Rect.Min.x, g.LastItemData.Rect.Max.x - g.Style.FramePadding.x * 2.0 - button_size);
        let button_y: c_float =  g.LastItemData.Rect.Min.y;
        let mut close_button_id: ImGuiID =  GetIDWithSeed("#CLOSE", null_mut(), id);
        if (button_ops::CloseButton(close_button_id, ImVec2::new(button_x, button_y)))
            *p_visible = false;
        g.LastItemData = last_item_backup;
    }

    return is_open;
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: Selectable
//-------------------------------------------------------------------------
// - Selectable()
//-------------------------------------------------------------------------

// Tip: pass a non-visible label (e.g. "##hello") then you can use the space to draw other text or image.
// But you need to make sure the ID is unique, e.g. enclose calls in PushID/PopID or use ##unique_id.
// With this scheme, ImGuiSelectableFlags_SpanAllColumns and ImGuiSelectableFlags_AllowItemOverlap are also frequently used flags.
// FIXME: Selectable() with (size.x == 0.0) and (SelectableTextAlign.x > 0.0) followed by SameLine() is currently not supported.
pub unsafe fn Selectable(label: &str, selected: bool, ImGuiSelectableFlags flags, size_arg: &ImVec2) -> bool
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if window.SkipItems { return  false; }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let setyle = &mut g.Style;

    // Submit label or explicit size to ItemSize(), whereas ItemAdd() will submit a larger/spanning rectangle.
    let mut id: ImGuiID =  window.GetID(label);
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);
    size: ImVec2(if size_arg.x != 0.0 { size_arg.x} else {label_size.x}, if size_arg.y != 0.0 { size_arg.y} else {label_size.y});
    let pos: ImVec2 = window.DC.CursorPos;
    pos.y += window.DC.CurrLineTextBaseOffset;
    ItemSize(size, 0.0);

    // Fill horizontal space
    // We don't support (size < 0.0) in Selectable() because the ItemSpacing extension would make explicitly right-aligned sizes not visibly match other widgets.
    let span_all_columns: bool = flag_set(flags, ImGuiSelectableFlags_SpanAllColumns);
    let min_x: c_float =  if span_all_columns { window.ParentWorkRect.Min.x} else {pos.x};
    let max_x: c_float =  if span_all_columns { window.ParentWorkRect.Max.x} else {window.WorkRect.Max.x};
    if (size_arg.x == 0.0 || (flags & ImGuiSelectableFlags_SpanAvailWidth))
        size.x = ImMax(label_size.x, max_x - min_x);

    // Text stays at the submission position, but bounding box may be extended on both sides
    let text_min: ImVec2 = pos;
    const text_max: ImVec2(min_x + size.x, pos.y + size.y);

    // Selectables are meant to be tightly packed together with no click-gap, so we extend their box to cover spacing between selectable.
    let mut bb: ImRect = ImRect::new(min_x, pos.y, text_max.x, text_max.y);
    if (flag_clear(flags, ImGuiSelectableFlags_NoPadWithHalfSpacing))
    {
        let spacing_x: c_float =  if span_all_columns { 0.0} else {style.ItemSpacing.x};
        let spacing_y: c_float =  style.ItemSpacing.y;
        let spacing_L: c_float =  IM_FLOOR(spacing_x * 0.5);
        let spacing_U: c_float =  IM_FLOOR(spacing_y * 0.5);
        bb.Min.x -= spacing_L;
        bb.Min.y -= spacing_U;
        bb.Max.x += (spacing_x - spacing_L);
        bb.Max.y += (spacing_y - spacing_U);
    }
    //if (g.IO.KeyCtrl) { GetForegroundDrawList().AddRect(bb.Min, bb.Max, IM_COL32(0, 255, 0, 255)); }

    // Modify ClipRect for the ItemAdd(), faster than doing a PushColumnsBackground/PushTableBackground for every Selectable..
    let backup_clip_rect_min_x: c_float =  window.ClipRect.Min.x;
    let backup_clip_rect_max_x: c_float =  window.ClipRect.Max.x;
    if (span_all_columns)
    {
        window.ClipRect.Min.x = window.ParentWorkRect.Min.x;
        window.ClipRect.Max.x = window.ParentWorkRect.Max.x;
    }

    let disabled_item: bool = flag_set(flags, ImGuiSelectableFlags_Disabled);
    let item_add: bool = ItemAdd(bb, id, null_mut(), if disabled_item {ImGuiItemFlags_Disabled} else { ImGuiItemFlags_None });
    if (span_all_columns)
    {
        window.ClipRect.Min.x = backup_clip_rect_min_x;
        window.ClipRect.Max.x = backup_clip_rect_max_x;
    }

    if !item_add { return  false; }

    let disabled_global: bool = (g.CurrentItemFlags & ImGuiItemFlags_Disabled) != 0;
    if (disabled_item && !disabled_global) // Only testing this as an optimization
        BeginDisabled();

    // FIXME: We can standardize the behavior of those two, we could also keep the fast path of override ClipRect + full push on render only,
    // which would be advantageous since most selectable are not selected.
    if span_all_columns && window.DC.CurrentColumns{
        PushColumnsBackground();}
    else if span_all_columns && g.CurrentTable{
        TablePushBackgroundChannel();}

    // We use NoHoldingActiveID on menus so user can click and _hold_ on a menu then drag to browse child entries
    button_flags: ImGuiButtonFlags = 0;
    if flag_set(flags, ImGuiSelectableFlags_NoHoldingActiveID) { button_flags |= ImGuiButtonFlags_NoHoldingActiveId; }
    if flag_set(flags, ImGuiSelectableFlags_SelectOnClick)     { button_flags |= ImGuiButtonFlags_PressedOnClick; }
    if flag_set(flags, ImGuiSelectableFlags_SelectOnRelease)   { button_flags |= ImGuiButtonFlags_PressedOnRelease; }
    if flag_set(flags, ImGuiSelectableFlags_AllowDoubleClick)  { button_flags |= ImGuiButtonFlags_PressedOnClickRelease | ImGuiButtonFlags_PressedOnDoubleClick; }
    if flag_set(flags, ImGuiSelectableFlags_AllowItemOverlap)  { button_flags |= ImGuiButtonFlags_AllowItemOverlap; }

    let was_selected: bool = selected;
    let mut hovered = false; let mut held = false;
    let mut pressed: bool =  button_ops::ButtonBehavior(bb, id, &hovered, &held, button_flags);

    // Auto-select when moved into
    // - This will be more fully fleshed in the range-select branch
    // - This is not exposed as it won't nicely work with some user side handling of shift/control
    // - We cannot do 'if (g.NavJustMovedToId != id) { selected = false; pressed = was_selected; }' for two reasons
    //   - (1) it would require focus scope to be set, need exposing PushFocusScope() or equivalent (e.g. BeginSelection() calling PushFocusScope())
    //   - (2) usage will fail with clipped items
    //   The multi-select API aim to fix those issues, e.g. may be replaced with a BeginSelection() API.
    if (flag_set(flags, ImGuiSelectableFlags_SelectOnNav) && g.NavJustMovedToId != 0 && g.NavJustMovedToFocusScopeId == window.DC.NavFocusScopeIdCurrent)
        if g.NavJustMovedToId == id{
            selected = pressed = true;}

    // Update NavId when clicking or when Hovering (this doesn't happen on most widgets), so navigation can be resumed with gamepad/keyboard
    if (pressed || (hovered && (flags & ImGuiSelectableFlags_SetNavIdOnHover)))
    {
        if (!g.NavDisableMouseHover && g.NavWindow == window && g.NavLayer == window.DC.NavLayerCurrent)
        {
            SetNavID(id, window.DC.NavLayerCurrent, window.DC.NavFocusScopeIdCurrent, WindowRectAbsToRel(window, bb)); // (bb == NavRect)
            g.NavDisableHighlight = true;
        }
    }
    if pressed {
        MarkItemEdited(id); }

    if flags & ImGuiSelectableFlags_AllowItemOverlap {
        SetItemAllowOverlap(); }

    // In this branch, Selectable() cannot toggle the selection so this will never trigger.
    if (selected != was_selected) //-V547
        g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_ToggledSelection;

    // Render
    if held && flag_set(flags, ImGuiSelectableFlags_DrawHoveredWhenHeld) {
        hovered = true;}
    if (hovered || selected)
    {
        col: u32 = GetColorU32(if (held && hovered) { ImGuiCol_HeaderActive } else{ if hovered {ImGuiCol_HeaderHovered} else { ImGuiCol_Header }});
        RenderFrame(bb.Min, bb.Max, col, false, 0.0);
    }
    RenderNavHighlight(bb, id, ImGuiNavHighlightFlags_TypeThin | ImGuiNavHighlightFlags_NoRounding);

    if span_all_columns && window.DC.CurrentColumns{
        PopColumnsBackground();}
    else if span_all_columns && g.CurrentTable{
        TablePopBackgroundChannel();}

    RenderTextClipped(text_min, text_max, label, null_mut(), &label_size, style.SelectableTextAlign, &bb);

    // Automatically close popups
    if (pressed && flag_set(window.Flags, ImGuiWindowFlags_Popup) && flag_clear(flags, ImGuiSelectableFlags_DontClosePopups) && !(g.LastItemData.InFlags & ImGuiItemFlags_SelectableDontClosePopup))
        CloseCurrentPopup();

    if (disabled_item && !disabled_global)
        EndDisabled();

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags);
    return pressed; //-V1020
}

pub unsafe fn Selectable(label: &str,p_selected: *mut bool, ImGuiSelectableFlags flags, size_arg: &ImVec2) -> bool
{
    if (Selectable(label, *p_selected, flags, size_arg))
    {
        *p_selected = !*p_selected;
        return true;
    }
    return false;
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: ListBox
//-------------------------------------------------------------------------
// - BeginListBox()
// - EndListBox()
// - ListBox()
//-------------------------------------------------------------------------

// Tip: To have a list filling the entire window width, use size.x = -FLT_MIN and pass an non-visible label e.g. "##empty"
// Tip: If your vertical size is calculated from an item count (e.g. 10 * item_height) consider adding a fractional part to facilitate seeing scrolling boundaries (e.g. 10.25 * item_height).
pub unsafe fn BeginListBox(label: &str, size_arg: &ImVec2) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if window.SkipItems { return  false; }

    let setyle = &mut g.Style;
    let mut id: ImGuiID =  GetID(label);
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);

    // Size default to hold ~7.25 items.
    // Fractional number of items helps seeing that we can scroll down/up without looking at scrollbar.
    let size: ImVec2 = ImFloor(CalcItemSize(size_arg, CalcItemWidth(), GetTextLineHeightWithSpacing() * 7.25f + style.FramePadding.y * 2.0));
    let frame_size: ImVec2 = ImVec2::new(size.x, ImMax(size.y, label_size.y));
    let mut frame_bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + frame_size);
    let mut bb: ImRect = ImRect::new(frame_bb.Min, frame_bb.Max + ImVec2::new(if label_size.x > 0.0 { style.ItemInnerSpacing.x + label_size.x} else {0.0}, 0.0));
    g.NextItemData.ClearFlags();

    if (!IsRectVisible(bb.Min, bb.Max))
    {
        ItemSize(bb.GetSize(), style.FramePadding.y);
        ItemAdd(bb, 0, &frame_bb);
        return false;
    }

    // FIXME-OPT: We could omit the BeginGroup() if label_size.x but would need to omit the EndGroup() as well.
    BeginGroup();
    if (label_size.x > 0.0)
    {
        let label_pos: ImVec2 = ImVec2::new(frame_bb.Max.x + style.ItemInnerSpacing.x, frame_bb.Min.y + style.FramePadding.y);
        RenderText(label_pos, label);
        window.DC.CursorMaxPos = ImMax(window.DC.CursorMaxPos, label_pos + label_size);
    }

    BeginChildFrame(id, frame_bb.GetSize());
    return true;
}

// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
// OBSOLETED in 1.81 (from February 2021)
pub unsafe fn ListBoxHeader(label: &str, items_count: c_int, height_in_items: c_int) -> bool
{
    // If height_in_items == -1, default height is maximum 7.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let height_in_items_f: c_float =  (if height_in_items < 0 { ImMin(items_count, 7)} else {height_in_items}) + 0.25f32;
    size: ImVec2;
    size.x = 0.0;
    size.y = GetTextLineHeightWithSpacing() * height_in_items_f + g.Style.FramePadding.y * 2.0;
    return BeginListBox(label, size);
}
// #endif

pub unsafe fn EndListBox()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT((window.Flags & ImGuiWindowFlags_ChildWindow) && "Mismatched BeginListBox/EndListBox calls. Did you test the return value of BeginListBox?");
    IM_UNUSED(window);

    EndChildFrame();
    EndGroup(); // This is only required to be able to do IsItemXXX query on the whole ListBox including label
}

pub unsafe fn ListBox(label: &str, current_item:  *mut c_int, const: &str items[], items_count: c_int, height_items: c_int) -> bool
{
    let value_changed: bool = ListBox(label, current_item, Items_ArrayGetter, items, items_count, height_items);
    return value_changed;
}

// This is merely a helper around BeginListBox(), EndListBox().
// Considering using those directly to submit custom data or store selection differently.
pub unsafe fn ListBox(label: &str, current_item:  *mut c_int, bool (*items_getter)(*mut c_void, c_int, *const char*), data: *mut c_void, items_count: c_int, height_in_items: c_int) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Calculate size from "height_in_items"
    if (height_in_items < 0)
        height_in_items = ImMin(items_count, 7);
    let height_in_items_f: c_float =  height_in_items + 0.25f32;
    size: ImVec2(0.0, ImFloor(GetTextLineHeightWithSpacing() * height_in_items_f + g.Style.FramePadding.y * 2.0));

    if !BeginListBox(label, size) { return  false; }

    // Assume all items have even height (= 1 line of text). If you need items of different height,
    // you can create a custom version of ListBox() in your code without using the clipper.
    let mut value_changed: bool =  false;
    ImGuiListClipper clipper;
    clipper.Begin(items_count, GetTextLineHeightWithSpacing()); // We know exactly our line height here so we pass it as a minor optimization, but generally you don't need to.
    while (clipper.Step())
        for (let i: c_int = clipper.DisplayStart; i < clipper.DisplayEnd; i++)
        {
let item_text: &str;
            if (!items_getter(data, i, &item_text))
                item_text = "*Unknown item*";

            PushID(i);
            let item_selected: bool = (i == *current_item);
            if (Selectable(item_text, item_selected))
            {
                *current_item = i;
                value_changed = true;
            }
            if item_selected {
                SetItemDefaultFocus(); }
            PopID();
        }
    EndListBox();

    if value_changed{
        MarkItemEdited(g.LastItemData.ID);}

    return value_changed;
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: PlotLines, PlotHistogram
//-------------------------------------------------------------------------
// - PlotEx() [Internal]
// - PlotLines()
// - PlotHistogram()
//-------------------------------------------------------------------------
// Plot/Graph widgets are not very good.
// Consider writing your own, or using a third-party one, see:
// - ImPlot https://github.com/epezent/implot
// - others https://github.com/ocornut/imgui/wiki/Useful-Extensions
//-------------------------------------------------------------------------

PlotEx: c_int(ImGuiPlotType plot_type, label: &str, c_float (*values_getter)(data: *mut c_void, idx: c_int), data: *mut c_void, values_count: c_int, values_offset: c_int, overlay_text: &str,scale_min: c_float,scale_max: c_float, frame_size: ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if (window.SkipItems)
        return -1;

    let setyle = &mut g.Style;
    let mut id: ImGuiID =  window.GetID(label);

    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);
    if frame_size.x == 0.0{
        frame_size.x = CalcItemWidth();}
    if (frame_size.y == 0.0)
        frame_size.y = label_size.y + (style.FramePadding.y * 2);

    let mut frame_bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + frame_size);
    let mut inner_bb: ImRect = ImRect::new(frame_bb.Min + style.FramePadding, frame_bb.Max - style.FramePadding);
    let mut total_bb: ImRect = ImRect::new(frame_bb.Min, frame_bb.Max + ImVec2::new(if label_size.x > 0.0 { style.ItemInnerSpacing.x + label_size.x} else {0.0}, 0));
    ItemSize(total_bb, style.FramePadding.y);
    if (!ItemAdd(total_bb, 0, &frame_bb))
        return -1;
    let hovered: bool = ItemHoverable(frame_bb, id);

    // Determine scale from values if not specified
    if (scale_min == f32::MAX || scale_max == f32::MAX)
    {
        let v_min: c_float =  f32::MAX;
        let v_max: c_float =  -f32::MAX;
        for (let i: c_int = 0; i < values_count; i++)
        {
            let v: c_float =  values_getter(data, i);
            if (v != v) // Ignore NaN values
                continue;
            v_min = ImMin(v_min, v);
            v_max = ImMax(v_max, v);
        }
        if scale_min == f32::MAX {
            scale_min = v_min;}
        if scale_max == f32::MAX {
            scale_max = v_max;}
    }

    RenderFrame(frame_bb.Min, frame_bb.Max, GetColorU32(ImGuiCol_FrameBg, 0.0), true, style.FrameRounding);

    let values_count_min: c_int = if plot_type == ImGuiPlotType_Lines { 2} else { 1};
    let idx_hovered: c_int = -1;
    if (values_count >= values_count_min)
    {
        let res_w: c_int = ImMin(frame_size.x, values_count) + (if (plot_type == ImGuiPlotType_Lines) { - 1} else {0});
        let item_count: c_int = values_count + (if (plot_type == ImGuiPlotType_Lines) { - 1} else {0});

        // Tooltip on hover
        if (hovered && inner_bb.Contains(g.IO.MousePos))
        {
            let t: c_float =  ImClamp((g.IO.MousePos.x - inner_bb.Min.x) / (inner_bb.Max.x - inner_bb.Min.x), 0.0, 0.99990f32);
            let v_idx: c_int = (t * item_count);
            // IM_ASSERT(v_idx >= 0 && v_idx < values_count);

            let v0: c_float =  values_getter(data, (v_idx + values_offset) % values_count);
            let v1: c_float =  values_getter(data, (v_idx + 1 + values_offset) % values_count);
            if (plot_type == ImGuiPlotType_Lines)
                SetTooltip("%d: %8.4g\n%d: %8.4g", v_idx, v0, v_idx + 1, v1);
            else if (plot_type == ImGuiPlotType_Histogram)
                SetTooltip("%d: %8.4g", v_idx, v0);
            idx_hovered = v_idx;
        }

        let t_step: c_float =  1.0 / res_w;
        let inv_scale: c_float =  if (scale_min == scale_max) { 0.0} else {1.0 / scale_max - scale_min};

        let v0: c_float =  values_getter(data, (0 + values_offset) % values_count);
        let t0: c_float =  0.0;
        let tp0: ImVec2 = ImVec2::new( t0, 1.0 - ImSaturate((v0 - scale_min) * inv_scale) );                       // Point in the normalized space of our target rectangle
        let histogram_zero_line_t: c_float = if  (scale_min * scale_max < 0.0) { (1 + scale_min * inv_scale)} else{ (if scale_min < 0.0 { 0.0} else {1.0})};   // Where does the zero line stands

        col_base: u32 = GetColorU32(if (plot_type == ImGuiPlotType_Lines) { ImGuiCol_PlotLines } else {ImGuiCol_PlotHistogram});
        col_hovered: u32 = GetColorU32(if (plot_type == ImGuiPlotType_Lines) { ImGuiCol_PlotLinesHovered }else{ ImGuiCol_PlotHistogramHovered});

        for (let n: c_int = 0; n < res_w; n++)
        {
            let t1: c_float =  t0 + t_step;
            let v1_idx: c_int = (t0 * item_count + 0.5);
            // IM_ASSERT(v1_idx >= 0 && v1_idx < values_count);
            let v1: c_float =  values_getter(data, (v1_idx + values_offset + 1) % values_count);
            let tp1: ImVec2 = ImVec2::new( t1, 1.0 - ImSaturate((v1 - scale_min) * inv_scale) );

            // NB: Draw calls are merged together by the DrawList system. Still, we should render our batch are lower level to save a bit of CPU.
            let pos0: ImVec2 = ImLerp(inner_bb.Min, inner_bb.Max, tp0);
            let pos1: ImVec2 = ImLerp(inner_bb.Min, inner_bb.Max, if (plot_type == ImGuiPlotType_Lines) { tp1} else{ ImVec2::new(tp1.x, histogram_zero_line_t)});
            if (plot_type == ImGuiPlotType_Lines)
            {
                window.DrawList.AddLine(pos0, pos1, idx_hovered == if v1_idx {col_hovered} else { col_base });
            }
            else if (plot_type == ImGuiPlotType_Histogram)
            {
                if (pos1.x >= pos0.x + 2.0)
                    pos1.x -= 1.0;
                window.DrawList.AddRectFilled(pos0, pos1, idx_hovered == if v1_idx {col_hovered} else { col_base });
            }

            t0 = t1;
            tp0 = tp1;
        }
    }

    // Text overlay
    if (overlay_text)
        RenderTextClipped(ImVec2::new(frame_bb.Min.x, frame_bb.Min.y + style.FramePadding.y), frame_bb.Max, overlay_text, null_mut(), null_mut(), ImVec2::new(0.5, 0.0));

    if (label_size.x > 0.0)
        RenderText(ImVec2::new(frame_bb.Max.x + style.ItemInnerSpacing.x, inner_bb.Min.y), label);

    // Return hovered index or -1 if none are hovered.
    // This is currently not exposed in the public API because we need a larger redesign of the whole thing, but in the short-term we are making it available in PlotEx().
    return idx_hovered;
}

struct ImGuiPlotArrayGetterData
{
    *let mut Values: c_float = 0.0;
    let mut Stride: c_int = 0;

    ImGuiPlotArrayGetterData(*values: c_float, stride: c_int) { Values = values; Stride = stride; }
};

staticPlot_ArrayGetter: c_float(data: *mut c_void, idx: c_int)
{
    ImGuiPlotArrayGetterData* plot_data = (ImGuiPlotArrayGetterData*)data;
    let v: c_float =  (*const c_void)(plot_Data.Values + idx * plot_Data.Stride);
    return v;
}

pub unsafe fn PlotLines(label: &str, *values: c_float, values_count: c_int, values_offset: c_int, overlay_text: &str,scale_min: c_float,scale_max: c_float, graph_size: ImVec2, stride: c_int)
{
    ImGuiPlotArrayGetterData data(values, stride);
    PlotEx(ImGuiPlotType_Lines, label, &Plot_ArrayGetter, &data, values_count, values_offset, overlay_text, scale_min, scale_max, graph_size);
}

pub unsafe fn PlotLines(label: &str, c_float (*values_getter)(data: *mut c_void, idx: c_int), data: *mut c_void, values_count: c_int, values_offset: c_int, overlay_text: &str,scale_min: c_float,scale_max: c_float, graph_size: ImVec2)
{
    PlotEx(ImGuiPlotType_Lines, label, values_getter, data, values_count, values_offset, overlay_text, scale_min, scale_max, graph_size);
}

pub unsafe fn PlotHistogram(label: &str, *values: c_float, values_count: c_int, values_offset: c_int, overlay_text: &str,scale_min: c_float,scale_max: c_float, graph_size: ImVec2, stride: c_int)
{
    ImGuiPlotArrayGetterData data(values, stride);
    PlotEx(ImGuiPlotType_Histogram, label, &Plot_ArrayGetter, &data, values_count, values_offset, overlay_text, scale_min, scale_max, graph_size);
}

pub unsafe fn PlotHistogram(label: &str, c_float (*values_getter)(data: *mut c_void, idx: c_int), data: *mut c_void, values_count: c_int, values_offset: c_int, overlay_text: &str,scale_min: c_float,scale_max: c_float, graph_size: ImVec2)
{
    PlotEx(ImGuiPlotType_Histogram, label, values_getter, data, values_count, values_offset, overlay_text, scale_min, scale_max, graph_size);
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: Value helpers
// Those is not very useful, legacy API.
//-------------------------------------------------------------------------
// - Value()
//-------------------------------------------------------------------------

pub unsafe fn Value(prefix: &str, b: bool)
{
    text_ops::Text("%s: %s", prefix, (b ? "true": "false"));
}

pub unsafe fn Value(prefix: &str, v: c_int)
{
    text_ops::Text("%s: %d", prefix, v);
}

pub unsafe fn Value(prefix: &str, v: c_uint)
{
    text_ops::Text("%s: %d", prefix, v);
}

pub unsafe fn Value(prefix: &str,v: c_float, float_format: &str)
{
    if (float_format)
    {
        fmt: [c_char;64];
        ImFormatString(fmt, fmt.len(), "%%s: %s", float_format);
        text_ops::Text(fmt, prefix, v);
    }
    else
    {
        text_ops::Text("%s: %.3f", prefix, v);
    }
}

//-------------------------------------------------------------------------
// [SECTION] MenuItem, BeginMenu, EndMenu, etc.
//-------------------------------------------------------------------------
// - ImGuiMenuColumns [Internal]
// - BeginMenuBar()
// - EndMenuBar()
// - BeginMainMenuBar()
// - EndMainMenuBar()
// - BeginMenu()
// - EndMenu()
// - MenuItemEx() [Internal]
// - MenuItem()
//-------------------------------------------------------------------------

// Helpers for internal use
c_void ImGuiMenuColumns::Update(spacing: c_float, window_reappearing: bool)
{
    if (window_reappearing)
        memset(Widths, 0, sizeof(Widths));
    Spacing = (ImU16)spacing;
    CalcNextTotalWidth(true);
    memset(Widths, 0, sizeof(Widths));
    TotalWidth = NextTotalWidth;
    NextTotalWidth = 0;
}

c_void ImGuiMenuColumns::CalcNextTotalWidth(update_offsets: bool)
{
    ImU16 offset = 0;
    let mut want_spacing: bool =  false;
    for (let i: c_int = 0; i < Widths.len(); i++)
    {
        ImU16 width = Widths[i];
        if (want_spacing && width > 0)
            offset += Spacing;
        want_spacing |= (width > 0);
        if (update_offsets)
        {
            if (i == 1) { OffsetLabel = offset; }
            if (i == 2) { OffsetShortcut = offset; }
            if (i == 3) { OffsetMark = offset; }
        }
        offset += width;
    }
    NextTotalWidth = offset;
}ImGuiMenuColumns: c_float::DeclColumns(w_icon: c_float,w_label: c_float,w_shortcut: c_float,w_mark: c_float)
{
    Widths[0] = ImMax(Widths[0], (ImU16)w_icon);
    Widths[1] = ImMax(Widths[1], (ImU16)w_label);
    Widths[2] = ImMax(Widths[2], (ImU16)w_shortcut);
    Widths[3] = ImMax(Widths[3], (ImU16)w_mark);
    CalcNextTotalWidth(false);
    return ImMax(TotalWidth, NextTotalWidth);
}

// FIXME: Provided a rectangle perhaps e.g. a BeginMenuBarEx() could be used anywhere..
// Currently the main responsibility of this function being to setup clip-rect + horizontal layout + menu navigation layer.
// Ideally we also want this to be responsible for claiming space out of the main window scrolling rectangle, in which case ImGuiWindowFlags_MenuBar will become unnecessary.
// Then later the same system could be used for multiple menu-bars, scrollbars, side-bars.
pub unsafe fn BeginMenuBar() -> bool
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if window.SkipItems { return  false; }
    if flag_clear(window.Flags, ImGuiWindowFlags_MenuBar) { return  false; }

    // IM_ASSERT(!window.DC.MenuBarAppending);
    BeginGroup(); // Backup position on layer 0 // FIXME: Misleading to use a group for that backup/restore
    PushID("##menubar");

    // We don't clip with current window clipping rectangle as it is already set to the area below. However we clip with window full rect.
    // We remove 1 worth of rounding to Max.x to that text in long menus and small windows don't tend to display over the lower-right rounded area, which looks particularly glitchy.
    let bar_rect: ImRect =  window.MenuBarRect();
    let mut clip_rect: ImRect = ImRect::new(IM_ROUND(bar_rect.Min.x + window.WindowBorderSize), IM_ROUND(bar_rect.Min.y + window.WindowBorderSize), IM_ROUND(ImMax(bar_rect.Min.x, bar_rect.Max.x - ImMax(window.WindowRounding, window.WindowBorderSize))), IM_ROUND(bar_rect.Max.y));
    clip_rect.ClipWith(window.OuterRectClipped);
    PushClipRect(clip_rect.Min, clip_rect.Max, false);

    // We overwrite CursorMaxPos because BeginGroup sets it to CursorPos (essentially the .EmitItem hack in EndMenuBar() would need something analogous here, maybe a BeginGroupEx() with flags).
    window.DC.CursorPos = window.DC.CursorMaxPos = ImVec2::new(bar_rect.Min.x + window.DC.MenuBarOffset.x, bar_rect.Min.y + window.DC.MenuBarOffset.y);
    window.DC.LayoutType = ImGuiLayoutType_Horizontal;
    window.DC.IsSameLine = false;
    window.DC.NavLayerCurrent = ImGuiNavLayer_Menu;
    window.DC.MenuBarAppending = true;
    layout_ops::AlignTextToFramePadding();
    return true;
}

pub unsafe fn EndMenuBar()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if window.SkipItems { return ; }
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Nav: When a move request within one of our child menu failed, capture the request to navigate among our siblings.
    if (NavMoveRequestButNoResultYet() && (g.NavMoveDir == ImGuiDir_Left || g.NavMoveDir == ImGuiDir_Right) && (g.NavWindow.Flags & ImGuiWindowFlags_ChildMenu))
    {
        // Try to find out if the request is for one of our child menu
        let mut nav_earliest_child: *mut ImGuiWindow =  g.NavWindow;
        while (nav_earliest_child->ParentWindow && (nav_earliest_child->Parentwindow.Flags & ImGuiWindowFlags_ChildMenu))
            nav_earliest_child = nav_earliest_child->ParentWindow;
        if (nav_earliest_child->ParentWindow == window && nav_earliest_child->DC.ParentLayoutType == ImGuiLayoutType_Horizontal && (g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded) == 0)
        {
            // To do so we claim focus back, restore NavId and then process the movement request for yet another frame.
            // This involve a one-frame delay which isn't very problematic in this situation. We could remove it by scoring in advance for multiple window (probably not worth bothering)
            const layer: ImGuiNavLayer = ImGuiNavLayer_Menu;
            // IM_ASSERT(window.DC.NavLayersActiveMaskNext & (1 << layer)); // Sanity check
            FocusWindow(window);
            SetNavID(window.NavLastIds[layer], layer, 0, window.NavRectRel[layer]);
            g.NavDisableHighlight = true; // Hide highlight for the current frame so we don't see the intermediary selection.
            g.NavDisableMouseHover = g.NavMousePosDirty = true;
            NavMoveRequestForward(g.NavMoveDir, g.NavMoveClipDir, g.NavMoveFlags, g.NavMoveScrollFlags); // Repeat
        }
    }

    IM_MSVC_WARNING_SUPPRESS(6011); // Static Analysis false positive "warning C6011: Dereferencing NULL pointer 'window'"
    // IM_ASSERT(window.Flags & ImGuiWindowFlags_MenuBar);
    // IM_ASSERT(window.DC.MenuBarAppending);
    PopClipRect();
    PopID();
    window.DC.MenuBarOffset.x = window.DC.CursorPos.x - window.Pos.x; // Save horizontal position so next append can reuse it. This is kinda equivalent to a per-layer CursorPos.
    g.GroupStack.last().unwrap().EmitItem = false;
    EndGroup(); // Restore position on layer 0
    window.DC.LayoutType = ImGuiLayoutType_Vertical;
    window.DC.IsSameLine = false;
    window.DC.NavLayerCurrent = ImGuiNavLayer_Main;
    window.DC.MenuBarAppending = false;
}

// Important: calling order matters!
// FIXME: Somehow overlapping with docking tech.
// FIXME: The "rect-cut" aspect of this could be formalized into a lower-level helper (rect-cut: https://halt.software/dead-simple-layouts)
pub unsafe fn BeginViewportSideBar(name: &str, viewport_p: *mut ImGuiViewport, dir: ImGuiDir,axis_size: c_float, window_flags: ImGuiWindowFlags) -> bool
{
    // IM_ASSERT(dir != ImGuiDir_None);

    let mut bar_window: *mut ImGuiWindow =  FindWindowByName(name);
    let mut viewport: *mut ImGuiViewport =  (if viewport_p {viewport_p} else { GetMainViewport }());
    if (bar_window == null_mut() || bar_window.BeginCount == 0)
    {
        // Calculate and set window size/position
        let avail_rect: ImRect =  viewport.GetBuildWorkRect();
        axis: ImGuiAxis = if dir == ImGuiDir_Up || dir == ImGuiDir_Down { ImGuiAxis_Y} else { ImGuiAxis_X};
        let pos: ImVec2 = avail_rect.Min;
        if (dir == ImGuiDir_Right || dir == ImGuiDir_Down)
            pos[axis] = avail_rect.Max[axis] - axis_size;
        let size: ImVec2 = avail_rect.GetSize();
        size[axis] = axis_size;
        SetNextWindowPos(pos);
        SetNextWindowSize(size);

        // Report our size into work area (for next frame) using actual window size
        if (dir == ImGuiDir_Up || dir == ImGuiDir_Left)
            viewport.BuildWorkOffsetMin[axis] += axis_size;
        else if (dir == ImGuiDir_Down || dir == ImGuiDir_Right)
            viewport.BuildWorkOffsetMax[axis] -= axis_size;
    }

    window_flags |= ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoDocking;
    SetNextWindowViewport(viewport.ID); // Enforce viewport so we don't create our own viewport when ImGuiConfigFlags_ViewportsNoMerge is set.
    PushStyleVar(ImGuiStyleVar_WindowRounding, 0.0);
    PushStyleVar(ImGuiStyleVar_WindowMinSize, ImVec2::new(0, 0)); // Lift normal size constraint
    let mut is_open: bool =  Begin(name, null_mut(), window_flags);
    PopStyleVar(2);

    return is_open;
}

pub unsafe fn BeginMainMenuBar() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut viewport: *mut ImGuiViewport =  GetMainViewport();

    // Notify of viewport change so GetFrameHeight() can be accurate in case of DPI change
    SetCurrentViewport(null_mut(), viewport);

    // For the main menu bar, which cannot be moved, we honor g.Style.DisplaySafeAreaPadding to ensure text can be visible on a TV set.
    // FIXME: This could be generalized as an opt-in way to clamp window.DC.CursorStartPos to avoid SafeArea?
    // FIXME: Consider removing support for safe area down the line... it's messy. Nowadays consoles have support for TV calibration in OS settings.
    g.NextWindowData.MenuBarOffsetMinVal = ImVec2::new(g.Style.DisplaySafeAreaPadding.x, ImMax(g.Style.DisplaySafeAreaPadding.y - g.Style.FramePadding.y, 0.0));
    window_flags: ImGuiWindowFlags = ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_MenuBar;
    let height: c_float =  GetFrameHeight();
    let mut is_open: bool =  BeginViewportSideBar("##MainMenuBar", viewport, ImGuiDir_Up, height, window_flags);
    g.NextWindowData.MenuBarOffsetMinVal = ImVec2::new(0.0, 0.0);

    if is_open {
        BeginMenuBar(); }
    else
        End();
    return is_open;
}

pub unsafe fn EndMainMenuBar()
{
    EndMenuBar();

    // When the user has left the menu layer (typically: closed menus through activation of an item), we restore focus to the previous window
    // FIXME: With this strategy we won't be able to restore a NULL focus.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.CurrentWindow == g.NavWindow && g.NavLayer == ImGuiNavLayer_Main && !g.NavAnyRequest)
        FocusTopMostWindowUnderOne(g.NavWindow, null_mut());

    End();
}

pub unsafe fn IsRootOfOpenMenuSet() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (g.OpenPopupStack.len() <= g.BeginPopupStack.len()) || flag_set(window.Flags, ImGuiWindowFlags_ChildMenu) { return  false; }

    // Initially we used 'upper_popup->OpenParentId == window.IDStack.back()' to differentiate multiple menu sets from each others
    // (e.g. inside menu bar vs loose menu items) based on parent ID.
    // This would however prevent the use of e.g. PuhsID() user code submitting menus.
    // Previously this worked between popup and a first child menu because the first child menu always had the _ChildWindow flag,
    // making  hovering on parent popup possible while first child menu was focused - but this was generally a bug with other side effects.
    // Instead we don't treat Popup specifically (in order to consistently support menu features in them), maybe the first child menu of a Popup
    // doesn't have the _ChildWindow flag, and we rely on this IsRootOfOpenMenuSet() check to allow hovering between root window/popup and first child menu.
    // In the end, lack of ID check made it so we could no longer differentiate between separate menu sets. To compensate for that, we at least check parent window nav layer.
    // This fixes the most common case of menu opening on hover when moving between window content and menu bar. Multiple different menu sets in same nav layer would still
    // open on hover, but that should be a lesser problem, because if such menus are close in proximity in window content then it won't feel weird and if they are far apart
    // it likely won't be a problem anyone runs into.
    let upper_popup: *const ImGuiPopupData = &g.OpenPopupStack[g.BeginPopupStack.len()];
    return (window.DC.NavLayerCurrent == upper_popup->ParentNavLayer && upper_popup.Window && (upper_popup->window.Flags & ImGuiWindowFlags_ChildMenu));
}

pub unsafe fn BeginMenuEx(label: &str, icon: &str, enabled: bool) -> bool
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if window.SkipItems { return  false; }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let setyle = &mut g.Style;
    let mut id: ImGuiID =  window.GetID(label);
    let mut menu_is_open: bool =  IsPopupOpen(id, ImGuiPopupFlags_None);

    // Sub-menus are ChildWindow so that mouse can be hovering across them (otherwise top-most popup menu would steal focus and not allow hovering on parent menu)
    // The first menu in a hierarchy isn't so hovering doesn't get across (otherwise e.g. resizing borders with ImGuiButtonFlags_FlattenChildren would react), but top-most BeginMenu() will bypass that limitation.
    flags: ImGuiWindowFlags = ImGuiWindowFlags_ChildMenu | ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_NoNavFocus;
    if flag_set(window.Flags, ImGuiWindowFlags_ChildMenu)
        flags |= ImGuiWindowFlags_ChildWindow;

    // If a menu with same the ID was already submitted, we will append to it, matching the behavior of Begin().
    // We are relying on a O(N) search - so O(N log N) over the frame - which seems like the most efficient for the expected small amount of BeginMenu() calls per frame.
    // If somehow this is ever becoming a problem we can switch to use e.g. ImGuiStorage mapping key to last frame used.
    if (g.MenusIdSubmittedThisFrame.contains(id))
    {
        if (menu_is_open)
            menu_is_open = BeginPopupEx(id, flags); // menu_is_open can be 'false' when the popup is completely clipped (e.g. zero size display)
        else
            g.NextWindowData.ClearFlags();          // we behave like Begin() and need to consume those values
        return menu_is_open;
    }

    // Tag menu as used. Next time BeginMenu() with same ID is called it will append to existing menu
    g.MenusIdSubmittedThisFrame.push(id);

    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);

    // Odd hack to allow hovering across menus of a same menu-set (otherwise we wouldn't be able to hover parent without always being a Child window)
    let menuset_is_open: bool = IsRootOfOpenMenuSet();
    let mut backed_nav_window: *mut ImGuiWindow =  g.NavWindow;
    if menuset_is_open{
        g.NavWindow = window;}

    // The reference position stored in popup_pos will be used by Begin() to find a suitable position for the child menu,
    // However the final position is going to be different! It is chosen by FindBestWindowPosForPopup().
    // e.g. Menus tend to overlap each other horizontally to amplify relative Z-ordering.
    popup_pos: ImVec2, pos = window.DC.CursorPos;
    PushID(label);
    if (!enabled)
        BeginDisabled();
    let offsets: *const ImGuiMenuColumns = &window.DC.MenuColumns;
    pressed: bool;
    const ImGuiSelectableFlags selectable_flags = ImGuiSelectableFlags_NoHoldingActiveID | ImGuiSelectableFlags_SelectOnClick | ImGuiSelectableFlags_DontClosePopups;
    if (window.DC.LayoutType == ImGuiLayoutType_Horizontal)
    {
        // Menu inside an horizontal menu bar
        // Selectable extend their highlight by half ItemSpacing in each direction.
        // For ChildMenu, the popup position will be overwritten by the call to FindBestWindowPosForPopup() in Begin()
        popup_pos = ImVec2::new(pos.x - 1.0 - IM_FLOOR(style.ItemSpacing.x * 0.5), pos.y - style.FramePadding.y + window.MenuBarHeight());
        window.DC.CursorPos.x += IM_FLOOR(style.ItemSpacing.x * 0.5);
        PushStyleVar(ImGuiStyleVar_ItemSpacing, ImVec2::new(style.ItemSpacing.x * 2.0, style.ItemSpacing.y));
        let w: c_float =  label_size.x;
        text_pos: ImVec2(window.DC.CursorPos.x + offsets->OffsetLabel, window.DC.CursorPos.y + window.DC.CurrLineTextBaseOffset);
        pressed = Selectable("", menu_is_open, selectable_flags, ImVec2::new(w, 0.0));
        RenderText(text_pos, label);
        PopStyleVar();
        window.DC.CursorPos.x += IM_FLOOR(style.ItemSpacing.x * (-1.0 + 0.5)); // -1 spacing to compensate the spacing added when Selectable() did a SameLine(). It would also work to call SameLine() ourselves after the PopStyleVar().
    }
    else
    {
        // Menu inside a regular/vertical menu
        // (In a typical menu window where all items are BeginMenu() or MenuItem() calls, extra_w will always be 0.0.
        //  Only when they are other items sticking out we're going to add spacing, yet only register minimum width into the layout system.
        popup_pos = ImVec2::new(pos.x, pos.y - style.WindowPadding.y);
        let icon_w: c_float =  if (icon && icon[0]) { CalcTextSize(icon, null_mut()).x} else {0.0};
        let checkmark_w: c_float =  IM_FLOOR(g.FontSize * 1.200);
        let min_w: c_float =  window.DC.MenuColumns.DeclColumns(icon_w, label_size.x, 0.0, checkmark_w); // Feedback to next frame
        let extra_w: c_float =  ImMax(0.0, GetContentRegionAvail().x - min_w);
        text_pos: ImVec2(window.DC.CursorPos.x + offsets->OffsetLabel, window.DC.CursorPos.y + window.DC.CurrLineTextBaseOffset);
        pressed = Selectable("", menu_is_open, selectable_flags | ImGuiSelectableFlags_SpanAvailWidth, ImVec2::new(min_w, 0.0));
        RenderText(text_pos, label);
        if (icon_w > 0.0)
            RenderText(pos + ImVec2::new(offsets->OffsetIcon, 0.0), icon);
        RenderArrow(window.DrawList, pos + ImVec2::new(offsets->OffsetMark + extra_w + g.FontSize * 0.3f32, 0.0), GetColorU32(ImGuiCol_Text, 0.0), ImGuiDir_Right);
    }
    if (!enabled)
        EndDisabled();

    let hovered: bool = (g.HoveredId == id) && enabled && !g.NavDisableMouseHover;
    if menuset_is_open{
        g.NavWindow = backed_nav_window;}

    let mut want_open: bool =  false;
    let mut want_close: bool =  false;
    if (window.DC.LayoutType == ImGuiLayoutType_Vertical) // (window.Flags & (ImGuiWindowFlags_Popup|ImGuiWindowFlags_ChildMenu))
    {
        // Close menu when not hovering it anymore unless we are moving roughly in the direction of the menu
        // Implement http://bjk5.com/post/44698559168/breaking-down-amazons-mega-dropdown to avoid using timers, so menus feels more reactive.
        let mut moving_toward_child_menu: bool =  false;
        ImGuiPopupData* child_popup = if g.BeginPopupStack.len() < g.OpenPopupStack.len() { &g.OpenPopupStack[g.BeginPopupStack.len()]} else { null_mut()}; // Popup candidate (testing below)
        let mut child_menu_window: *mut ImGuiWindow = if (child_popup && child_popup.Window && child_popup->window.ParentWindow == window) { child_popup.Window }else {null_mut()};
        if (g.HoveredWindow == window && child_menu_window != null_mut())
        {
            let ref_unit: c_float =  g.FontSize; // FIXME-DPI
            let child_dir: c_float =  if(window.Pos.x < child_menu_window.Pos.x) { 1.0} else {- 1.0};
            let next_window_rect: ImRect =  child_menu_window.Rect();
            let ta: ImVec2 = (g.IO.MousePos - g.IO.MouseDelta);
            let tb: ImVec2 = if child_dir > 0.0 { next_window_rect.GetTL()} else { next_window_rect.GetTR()};
            let tc: ImVec2 = if child_dir > 0.0 { next_window_rect.GetBL()} else { next_window_rect.GetBR()};
            let extra: c_float =  ImClamp(ImFabs(ta.x - tb.x) * 0.3f32, ref_unit * 0.5, ref_unit * 2.5);   // add a bit of extra slack.
            ta.x += child_dir * -0.5;
            tb.x += child_dir * ref_unit;
            tc.x += child_dir * ref_unit;
            tb.y = ta.y + ImMax((tb.y - extra) - ta.y, -ref_unit * 8.0);                           // triangle has maximum height to limit the slope and the bias toward large sub-menus
            tc.y = ta.y + ImMin((tc.y + extra) - ta.y, +ref_unit * 8.0);
            moving_toward_child_menu = ImTriangleContainsPoint(ta, tb, tc, g.IO.MousePos);
            //GetForegroundDrawList()->AddTriangleFilled(ta, tb, tc, moving_toward_child_menu ? IM_COL32(0,128,0,128) : IM_COL32(128,0,0,128)); // [DEBUG]
        }

        // The 'HovereWindow == window' check creates an inconsistency (e.g. moving away from menu slowly tends to hit same window, whereas moving away fast does not)
        // But we also need to not close the top-menu menu when moving over void. Perhaps we should extend the triangle check to a larger polygon.
        // (Remember to test this on BeginPopup("A")->BeginMenu("B") sequence which behaves slightly differently as B isn't a Child of A and hovering isn't shared.)
        if menu_is_open && !hovered && g.HoveredWindow == window && !moving_toward_child_menu && !g.NavDisableMouseHover {
            want_close = true;}

        // Open
        if (!menu_is_open && pressed) // Click/activate to open
            want_open = true;
        else if (!menu_is_open && hovered && !moving_toward_child_menu) // Hover to open
            want_open = true;
        if (g.NavId == id && g.NavMoveDir == ImGuiDir_Right) // Nav-Right to open
        {
            want_open = true;
            NavMoveRequestCancel();
        }
    }
    else
    {
        // Menu bar
        if (menu_is_open && pressed && menuset_is_open) // Click an open menu again to close it
        {
            want_close = true;
            want_open = menu_is_open = false;
        }
        else if (pressed || (hovered && menuset_is_open && !menu_is_open)) // First click to open, then hover to open others
        {
            want_open = true;
        }
        else if (g.NavId == id && g.NavMoveDir == ImGuiDir_Down) // Nav-Down to open
        {
            want_open = true;
            NavMoveRequestCancel();
        }
    }

    if (!enabled) // explicitly close if an open menu becomes disabled, facilitate users code a lot in pattern such as 'if (BeginMenu("options", has_object)) { ..use object.. }'
        want_close = true;
    if (want_close && IsPopupOpen(id, ImGuiPopupFlags_None))
        ClosePopupToLevel(g.BeginPopupStack.len(), true);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags | ImGuiItemStatusFlags_Openable | (if menu_is_open {ImGuiItemStatusFlags_Opened} else { 0 }));
    PopID();

    if (!menu_is_open && want_open && g.OpenPopupStack.len() > g.BeginPopupStack.len())
    {
        // Don't recycle same menu level in the same frame, first close the other menu and yield for a frame.
        OpenPopup(label);
        return false;
    }

    menu_is_open |= want_open;
    if want_open {
        OpenPopup(label)(); }

    if (menu_is_open)
    {
        SetNextWindowPos(popup_pos, ImGuiCond_Always); // Note: this is super misleading! The value will serve as reference for FindBestWindowPosForPopup(), not actual pos.
        PushStyleVar(ImGuiStyleVar_ChildRounding, style.PopupRounding); // First level will use _PopupRounding, subsequent will use _ChildRounding
        menu_is_open = BeginPopupEx(id, flags); // menu_is_open can be 'false' when the popup is completely clipped (e.g. zero size display)
        PopStyleVar();
    }
    else
    {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
    }

    return menu_is_open;
}

pub unsafe fn BeginMenu(label: &str, enabled: bool) -> bool
{
    return BeginMenuEx(label, null_mut(), enabled);
}

pub unsafe fn EndMenu()
{
    // Nav: When a left move request _within our child menu_ failed, close ourselves (the _parent_ menu).
    // A menu doesn't close itself because EndMenuBar() wants the catch the last Left<>Right inputs.
    // However, it means that with the current code, a BeginMenu() from outside another menu or a menu-bar won't be closable with the Left direction.
    // FIXME: This doesn't work if the parent BeginMenu() is not on a menu.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (g.NavMoveDir == ImGuiDir_Left && NavMoveRequestButNoResultYet() && window.DC.LayoutType == ImGuiLayoutType_Vertical)
        if (g.NavWindow && (g.NavWindow.RootWindowForNav.Flags & ImGuiWindowFlags_Popup) && g.NavWindow.RootWindowForNav->ParentWindow == window)
        {
            ClosePopupToLevel(g.BeginPopupStack.len(), true);
            NavMoveRequestCancel();
        }

    EndPopup();
}

pub unsafe fn MenuItemEx(label: &str, icon: &str, shortcut: &str, selected: bool, enabled: bool) -> bool
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if window.SkipItems { return  false; }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;
    let pos: ImVec2 = window.DC.CursorPos;
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);

    let menuset_is_open: bool = IsRootOfOpenMenuSet();
    let mut backed_nav_window: *mut ImGuiWindow =  g.NavWindow;
    if menuset_is_open{
        g.NavWindow = window;}

    // We've been using the equivalent of ImGuiSelectableFlags_SetNavIdOnHover on all Selectable() since early Nav system days (commit 43ee5d73),
    // but I am unsure whether this should be kept at all. For now moved it to be an opt-in feature used by menus only.
    pressed: bool;
    PushID(label);
    if (!enabled)
        BeginDisabled();

    const ImGuiSelectableFlags selectable_flags = ImGuiSelectableFlags_SelectOnRelease | ImGuiSelectableFlags_SetNavIdOnHover;
    let offsets: *const ImGuiMenuColumns = &window.DC.MenuColumns;
    if (window.DC.LayoutType == ImGuiLayoutType_Horizontal)
    {
        // Mimic the exact layout spacing of BeginMenu() to allow MenuItem() inside a menu bar, which is a little misleading but may be useful
        // Note that in this situation: we don't render the shortcut, we render a highlight instead of the selected tick mark.
        let w: c_float =  label_size.x;
        window.DC.CursorPos.x += IM_FLOOR(style.ItemSpacing.x * 0.5);
        text_pos: ImVec2(window.DC.CursorPos.x + offsets->OffsetLabel, window.DC.CursorPos.y + window.DC.CurrLineTextBaseOffset);
        PushStyleVar(ImGuiStyleVar_ItemSpacing, ImVec2::new(style.ItemSpacing.x * 2.0, style.ItemSpacing.y));
        pressed = Selectable("", selected, selectable_flags, ImVec2::new(w, 0.0));
        PopStyleVar();
        RenderText(text_pos, label);
        window.DC.CursorPos.x += IM_FLOOR(style.ItemSpacing.x * (-1.0 + 0.5)); // -1 spacing to compensate the spacing added when Selectable() did a SameLine(). It would also work to call SameLine() ourselves after the PopStyleVar().
    }
    else
    {
        // Menu item inside a vertical menu
        // (In a typical menu window where all items are BeginMenu() or MenuItem() calls, extra_w will always be 0.0.
        //  Only when they are other items sticking out we're going to add spacing, yet only register minimum width into the layout system.
        let icon_w: c_float = if  (icon && icon[0]) { CalcTextSize(icon, null_mut()).x} else {0.0};
        let shortcut_w: c_float =  if (shortcut && shortcut[0]) { CalcTextSize(shortcut, null_mut()).x} else {0.0};
        let checkmark_w: c_float =  IM_FLOOR(g.FontSize * 1.200);
        let min_w: c_float =  window.DC.MenuColumns.DeclColumns(icon_w, label_size.x, shortcut_w, checkmark_w); // Feedback for next frame
        let stretch_w: c_float =  ImMax(0.0, GetContentRegionAvail().x - min_w);
        pressed = Selectable("", false, selectable_flags | ImGuiSelectableFlags_SpanAvailWidth, ImVec2::new(min_w, 0.0));
        RenderText(pos + ImVec2::new(offsets->OffsetLabel, 0.0), label);
        if (icon_w > 0.0)
            RenderText(pos + ImVec2::new(offsets->OffsetIcon, 0.0), icon);
        if (shortcut_w > 0.0)
        {
            PushStyleColor(ImGuiCol_Text, style.Colors[ImGuiCol_TextDisabled]);
            RenderText(pos + ImVec2::new(offsets->OffsetShortcut + stretch_w, 0.0), shortcut, null_mut(), false);
            PopStyleColor();
        }
        if (selected)
            RenderCheckMark(window.DrawList, pos + ImVec2::new(offsets->OffsetMark + stretch_w + g.FontSize * 0.40f32, g.FontSize * 0.134f * 0.5), GetColorU32(ImGuiCol_Text, 0.0), g.FontSize  * 0.8660f32);
    }
    IMGUI_TEST_ENGINE_ITEM_INFO(g.LastItemData.ID, label, g.LastItemData.StatusFlags | ImGuiItemStatusFlags_Checkable | (if selected {ImGuiItemStatusFlags_Checked} else { 0 }));
    if (!enabled)
        EndDisabled();
    PopID();
    if menuset_is_open{
        g.NavWindow = backed_nav_window;}

    return pressed;
}

pub unsafe fn MenuItem(label: &str, shortcut: &str, selected: bool, enabled: bool) -> bool
{
    return MenuItemEx(label, null_mut(), shortcut, selected, enabled);
}

pub unsafe fn MenuItem(label: &str, shortcut: &str,p_selected: *mut bool, enabled: bool) -> bool
{
    if (MenuItemEx(label, null_mut(), shortcut, if p_selected { * p_selected }else{ false}, enabled))
    {
        if (p_selected)
            *p_selected = !*p_selected;
        return true;
    }
    return false;
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: BeginTabBar, EndTabBar, etc.
//-------------------------------------------------------------------------
// - BeginTabBar()
// - BeginTabBarEx() [Internal]
// - EndTabBar()
// - TabBarLayout() [Internal]
// - TabBarCalcTabID() [Internal]
// - TabBarCalcMaxTabWidth() [Internal]
// - TabBarFindTabById() [Internal]
// - TabBarAddTab() [Internal]
// - TabBarRemoveTab() [Internal]
// - TabBarCloseTab() [Internal]
// - TabBarScrollClamp() [Internal]
// - TabBarScrollToTab() [Internal]
// - TabBarQueueChangeTabOrder() [Internal]
// - TabBarScrollingButtons() [Internal]
// - TabBarTabListPopupButton() [Internal]
//-------------------------------------------------------------------------

struct ImGuiTabBarSection
{
    c_int                 TabCount;               // Number of tabs in this section.Width: c_float;                  // Sum of width of tabs in this section (after shrinking down)Spacing: c_float;                // Horizontal spacing at the end of the section.

    ImGuiTabBarSection() { memset(this, 0, sizeof(*this)); }
};

namespace ImGui
{
    static c_void             TabBarLayout(ImGuiTabBar* tab_bar);
    static u32            TabBarCalcTabID(ImGuiTabBar* tab_bar, label: &str, docked_window: *mut ImGuiWindow);
    staticTabBarCalcMaxTabWidth: c_float();
    staticTabBarScrollClamp: c_float(ImGuiTabBar* tab_bar,scrolling: c_float);
    static c_void             TabBarScrollToTab(ImGuiTabBar* tab_bar, tab_id: ImGuiID, ImGuiTabBarSection* sections);
    static ImGuiTabItem*    TabBarScrollingButtons(ImGuiTabBar* tab_bar);
    static ImGuiTabItem*    TabBarTabListPopupButton(ImGuiTabBar* tab_bar);
}

ImGuiTabBar::ImGuiTabBar()
{
    memset(this, 0, sizeof(*this));
    CurrFrameVisible = PrevFrameVisible = -1;
    LastTabItemIdx = -1;
}

static inline TabItemGetSectionIdx: c_int(*const ImGuiTabItem tab)
{
    return if flag_set(tab.Flags, ImGuiTabItemFlags_Leading) { 0 } else{ if flag_set(tab.Flags, ImGuiTabItemFlags_Trailing) { 2}else {1}};
}

: c_int TabItemComparerBySection(lhs: *const c_void, rhs: *const c_void)
{
    let a: *const ImGuiTabItem = (*const ImGuiTabItem)lhs;
    let b: *const ImGuiTabItem = (*const ImGuiTabItem)rhs;
    let a_section: c_int = TabItemGetSectionIdx(a);
    let b_section: c_int = TabItemGetSectionIdx(b);
    if (a_section != b_section)
        return a_section - b_section;
    return (a->IndexDuringLayout - b->IndexDuringLayout);
}

: c_int TabItemComparerByBeginOrder(lhs: *const c_void, rhs: *const c_void)
{
    let a: *const ImGuiTabItem = (*const ImGuiTabItem)lhs;
    let b: *const ImGuiTabItem = (*const ImGuiTabItem)rhs;
    return (a->BeginOrder - b->BeginOrder);
}

static ImGuiTabBar* GetTabBarFromTabBarRef(const ImGuiPtrOrIndex& re0f32)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return if ref.Ptr { (ImGuiTabBar * ) ref.Ptr }else {g.TabBars.GetByIndex( ref.Index)};
}

static ImGuiPtrOrIndex GetTabBarRefFromTabBar(ImGuiTabBar* tab_bar)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.TabBars.Contains(tab_bar){
        return ImGuiPtrOrIndex(g.TabBars.GetIndex(tab_bar));}
    return ImGuiPtrOrIndex(tab_bar);
}

bool    BeginTabBar(str_id: &str, ImGuiTabBarFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if window.SkipItems { return  false; }

    let mut id: ImGuiID =  window.GetID(str_id);
    ImGuiTabBar* tab_bar = g.TabBars.GetOrAddByKey(id);
    let tab_bar_bb: ImRect =  ImRect(window.DC.CursorPos.x, window.DC.CursorPos.y, window.WorkRect.Max.x, window.DC.CursorPos.y + g.FontSize + g.Style.FramePadding.y * 2);
    tab_bar.ID = id;
    return BeginTabBarEx(tab_bar, tab_bar_bb, flags | ImGuiTabBarFlags_IsFocused, null_mut());
}

bool    BeginTabBarEx(ImGuiTabBar* tab_bar, tab_bar_bb: &ImRect, ImGuiTabBarFlags flags, dock_node:*mut ImGuiDockNode)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if window.SkipItems { return  false; }

    if (flag_clear(flags, ImGuiTabBarFlags_DockNode))
        PushOverrideID(tab_bar.ID);

    // Add to stack
    g.CurrentTabBarStack.push(GetTabBarRefFromTabBar(tab_bar));
    g.CurrentTabBar = tab_bar;

    // Append with multiple BeginTabBar()/EndTabBar() pairs.
    tab_bar->BackupCursorPos = window.DC.CursorPos;
    if (tab_bar->CurrFrameVisible == g.FrameCount)
    {
        window.DC.CursorPos = ImVec2::new(tab_bar.BarRect.Min.x, tab_bar.BarRect.Max.y + tab_bar->ItemSpacingY);
        tab_bar->BeginCount+= 1;
        return true;
    }

    // Ensure correct ordering when toggling ImGuiTabBarFlags_Reorderable flag, or when a new tab was added while being not reorderable
    if (flag_set(flags, ImGuiTabBarFlags_Reorderable) != flag_set(tab_bar.Flags, ImGuiTabBarFlags_Reorderable) || (tab_bar.TabsAddedNew && flag_clear(flags, ImGuiTabBarFlags_Reorderable)))
        if (flag_clear(flags, ImGuiTabBarFlags_DockNode)) // FIXME: TabBar with DockNode can now be hybrid
            ImQsort(tab_bar.Tabs.Data, tab_bar.Tabs.Size, sizeof(ImGuiTabItem), TabItemComparerByBeginOrder);
    tab_bar.TabsAddedNew = false;

    // Flags
    if (flag_clear(flags, ImGuiTabBarFlags_FittingPolicyMask_))
        flags |= ImGuiTabBarFlags_FittingPolicyDefault_;

    tab_bar.Flags = flags;
    tab_bar.BarRect = tab_bar_bb;
    tab_bar->WantLayout = true; // Layout will be done on the first call to ItemTab()
    tab_bar->PrevFrameVisible = tab_bar->CurrFrameVisible;
    tab_bar->CurrFrameVisible = g.FrameCount;
    tab_bar->PrevTabsContentsHeight = tab_bar->CurrTabsContentsHeight;
    tab_bar->CurrTabsContentsHeight = 0.0;
    tab_bar->ItemSpacingY = g.Style.ItemSpacing.y;
    tab_bar->FramePadding = g.Style.FramePadding;
    tab_bar.TabsActiveCount = 0;
    tab_bar->BeginCount = 1;

    // Set cursor pos in a way which only be used in the off-chance the user erroneously submits item before BeginTabItem(): items will overlap
    window.DC.CursorPos = ImVec2::new(tab_bar.BarRect.Min.x, tab_bar.BarRect.Max.y + tab_bar->ItemSpacingY);

    // Draw separator
    col: u32 = GetColorU32(if flag_set(flags, ImGuiTabBarFlags_IsFocused) { ImGuiCol_TabActive }else {ImGuiCol_TabUnfocusedActive});
    let y: c_float =  tab_bar.BarRect.Max.y - 1.0;
    if (dock_node != null_mut())
    {
        let separator_min_x: c_float =  dock_node.Pos.x + window.WindowBorderSize;
        let separator_max_x: c_float =  dock_node.Pos.x + dock_node.Size.x - window.WindowBorderSize;
        window.DrawList.AddLine(ImVec2::new(separator_min_x, y), ImVec2::new(separator_max_x, y), col, 1.0);
    }
    else
    {
        let separator_min_x: c_float =  tab_bar.BarRect.Min.x - IM_FLOOR(window.WindowPadding.x * 0.5);
        let separator_max_x: c_float =  tab_bar.BarRect.Max.x + IM_FLOOR(window.WindowPadding.x * 0.5);
        window.DrawList.AddLine(ImVec2::new(separator_min_x, y), ImVec2::new(separator_max_x, y), col, 1.0);
    }
    return true;
}

c_void    EndTabBar()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if window.SkipItems { return ; }

    ImGuiTabBar* tab_bar = g.CurrentTabBar;
    if (tab_bar == null_mut())
    {
        // IM_ASSERT_USER_ERROR(tab_bar != NULL, "Mismatched BeginTabBar()/EndTabBar()!");
        return;
    }

    // Fallback in case no TabItem have been submitted
    if (tab_bar->WantLayout)
        TabBarLayout(tab_bar);

    // Restore the last visible height if no tab is visible, this reduce vertical flicker/movement when a tabs gets removed without calling SetTabItemClosed().
    let tab_bar_appearing: bool = (tab_bar->PrevFrameVisible + 1 < g.FrameCount);
    if (tab_bar->VisibleTabWasSubmitted || tab_bar.VisibleTabId == 0 || tab_bar_appearing)
    {
        tab_bar->CurrTabsContentsHeight = ImMax(window.DC.CursorPos.y - tab_bar.BarRect.Max.y, tab_bar->CurrTabsContentsHeight);
        window.DC.CursorPos.y = tab_bar.BarRect.Max.y + tab_bar->CurrTabsContentsHeight;
    }
    else
    {
        window.DC.CursorPos.y = tab_bar.BarRect.Max.y + tab_bar->PrevTabsContentsHeight;
    }
    if (tab_bar->BeginCount > 1)
        window.DC.CursorPos = tab_bar->BackupCursorPos;

    if ((tab_bar.Flags & ImGuiTabBarFlags_DockNode) == 0)
        PopID();

    g.CurrentTabBarStack.pop_back();
    g.CurrentTabBar = if g.CurrentTabBarStack.empty() { null_mut()} else {GetTabBarFromTabBarRef(g.CurrentTabBarStack.last().unwrap()});
}

// This is called only once a frame before by the first call to ItemTab()
// The reason we're not calling it in BeginTabBar() is to leave a chance to the user to call the SetTabItemClosed() functions.
pub unsafe fn TabBarLayout(ImGuiTabBar* tab_bar)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    tab_bar->WantLayout = false;

    // Garbage collect by compacting list
    // Detect if we need to sort out tab list (e.g. in rare case where a tab changed section)
    let tab_dst_n: c_int = 0;
    let mut need_sort_by_section: bool =  false;
    ImGuiTabBarSection sections[3]; // Layout sections: Leading, Central, Trailing
    for (let tab_src_n: c_int = 0; tab_src_n < tab_bar.Tabs.Size; tab_src_n++)
    {
        ImGuiTabItem* tab = &tab_bar.Tabs[tab_src_n];
        if (tab->LastFrameVisible < tab_bar->PrevFrameVisible || tab->WantClose)
        {
            // Remove tab
            if (tab_bar.VisibleTabId == tab.ID) { tab_bar.VisibleTabId = 0; }
            if (tab_bar.SelectedTabId == tab.ID) { tab_bar.SelectedTabId = 0; }
            if (tab_bar.NextSelectedTabId == tab.ID) { tab_bar.NextSelectedTabId = 0; }
            continue;
        }
        if (tab_dst_n != tab_src_n)
            tab_bar.Tabs[tab_dst_n] = tab_bar.Tabs[tab_src_n];

        tab = &tab_bar.Tabs[tab_dst_n];
        tab->IndexDuringLayout = tab_dst_n;

        // We will need sorting if tabs have changed section (e.g. moved from one of Leading/Central/Trailing to another)
        let curr_tab_section_n: c_int = TabItemGetSectionIdx(tab);
        if (tab_dst_n > 0)
        {
            ImGuiTabItem* prev_tab = &tab_bar.Tabs[tab_dst_n - 1];
            let prev_tab_section_n: c_int = TabItemGetSectionIdx(prev_tab);
            if curr_tab_section_n == 0 && prev_tab_section_n != 0 {
                need_sort_by_section = true;}
            if prev_tab_section_n == 2 && curr_tab_section_n != 2 {
                need_sort_by_section = true;}
        }

        sections[curr_tab_section_n].TabCount+= 1;
        tab_dst_n+= 1;
    }
    if (tab_bar.Tabs.Size != tab_dst_n)
        tab_bar.Tabs.resize(tab_dst_n);

    if (need_sort_by_section)
        ImQsort(tab_bar.Tabs.Data, tab_bar.Tabs.Size, sizeof(ImGuiTabItem), TabItemComparerBySection);

    // Calculate spacing between sections
    sections[0].Spacing = if sections[0].TabCount > 0 && (sections[1].TabCount + sections[2].TabCount) > 0 { g.Style.ItemInnerSpacing.x} else {0.0};
    sections[1].Spacing = if sections[1].TabCount > 0 && sections[2].TabCount > 0 { g.Style.ItemInnerSpacing.x} else {0.0};

    // Setup next selected tab
    let mut scroll_to_tab_id: ImGuiID =  0;
    if (tab_bar.NextSelectedTabId)
    {
        tab_bar.SelectedTabId = tab_bar.NextSelectedTabId;
        tab_bar.NextSelectedTabId = 0;
        scroll_to_tab_id = tab_bar.SelectedTabId;
    }

    // Process order change request (we could probably process it when requested but it's just saner to do it in a single spot).
    if (tab_bar->ReorderRequestTabId != 0)
    {
        if (TabBarProcessReorder(tab_bar))
            if (tab_bar->ReorderRequestTabId == tab_bar.SelectedTabId)
                scroll_to_tab_id = tab_bar->ReorderRequestTabId;
        tab_bar->ReorderRequestTabId = 0;
    }

    // Tab List Popup (will alter tab_bar.BarRect and therefore the available width!)
    let tab_list_popup_button: bool = flag_set(tab_bar.Flags, ImGuiTabBarFlags_TabListPopupButton) != 0;
    if (tab_list_popup_button)
        if (ImGuiTabItem* tab_to_select = TabBarTabListPopupButton(tab_bar)) // NB: Will alter BarRect.Min.x!
            scroll_to_tab_id = tab_bar.SelectedTabId = tab_to_select.ID;

    // Leading/Trailing tabs will be shrink only if central one aren't visible anymore, so layout the shrink data as: leading, trailing, central
    // (whereas our tabs are stored as: leading, central, trailing)
    shrink_buffer_indexes: [c_int;3] = { 0, sections[0].TabCount + sections[2].TabCount, sections[0].TabCount };
    g.ShrinkWidthBuffer.resize(tab_bar.Tabs.Size);

    // Compute ideal tabs widths + store them into shrink buffer
    ImGuiTabItem* most_recently_selected_tab= null_mut();
    let curr_section_n: c_int = -1;
    let mut found_selected_tab_id: bool =  false;
    for (let tab_n: c_int = 0; tab_n < tab_bar.Tabs.Size; tab_n++)
    {
        ImGuiTabItem* tab = &tab_bar.Tabs[tab_n];
        // IM_ASSERT(tab->LastFrameVisible >= tab_bar->PrevFrameVisible);

        if (most_recently_selected_tab == null_mut() || most_recently_selected_tab->LastFrameSelected < tab->LastFrameSelected) && flag_clear(tab.Flags, ImGuiTabItemFlags_Button) {
            most_recently_selected_tab = tab;}
        if tab.ID == tab_bar.SelectedTabId {
            found_selected_tab_id = true;}
        if (scroll_to_tab_id == 0 && g.NavJustMovedToId == tab.ID)
            scroll_to_tab_id = tab.ID;

        // Refresh tab width immediately, otherwise changes of style e.g. style.FramePadding.x would noticeably lag in the tab bar.
        // Additionally, when using TabBarAddTab() to manipulate tab bar order we occasionally insert new tabs that don't have a width yet,
        // and we cannot wait for the next BeginTabItem() call. We cannot compute this width within TabBarAddTab() because font size depends on the active window.
        let mut  tab_name: &str = tab_bar.GetTabNametab);
        let has_close_button: bool = if tab.Flags & ImGuiTabItemFlags_NoCloseButton { false} else { true};
        tab->ContentWidth = if tab->RequestedWidth >= 0.0 { tab->RequestedWidth} else { TabItemCalcSize(tab_name, has_close_button).x};

        let section_n: c_int = TabItemGetSectionIdx(tab);
        ImGuiTabBarSection* section = &sections[section_n];
        section->Width += tab->ContentWidth + (if section_n == curr_section_n { g.Style.ItemInnerSpacing.x} else {0.0});
        curr_section_n = section_n;

        // Store data so we can build an array sorted by width if we need to shrink tabs down
        IM_MSVC_WARNING_SUPPRESS(6385);
        ImGuiShrinkWidthItem* shrink_width_item = &g.ShrinkWidthBuffer[shrink_buffer_indexes[section_n]++];
        shrink_width_item->Index = tab_n;
        shrink_width_item->Width = shrink_width_item->InitialWidth = tab->ContentWidth;
        tab->Width = ImMax(tab->ContentWidth, 1.0);
    }

    // Compute total ideal width (used for e.g. auto-resizing a window)
    tab_bar.WidthAllTabsIdeal = 0.0;
    for (let section_n: c_int = 0; section_n < 3; section_n++)
        tab_bar.WidthAllTabsIdeal += sections[section_n].Width + sections[section_n].Spacing;

    // Horizontal scrolling buttons
    // (note that TabBarScrollButtons() will alter BarRect.Max.x)
    if ((tab_bar.WidthAllTabsIdeal > tab_bar.BarRect.GetWidth() && tab_bar.Tabs.Size > 1) && flag_clear(tab_bar.Flags, ImGuiTabBarFlags_NoTabListScrollingButtons) && (tab_bar.Flags & ImGuiTabBarFlags_FittingPolicyScroll))
        if (ImGuiTabItem* scroll_and_select_tab = TabBarScrollingButtons(tab_bar))
        {
            scroll_to_tab_id = scroll_and_select_tab.ID;
            if ((scroll_and_select_tab.Flags & ImGuiTabItemFlags_Button) == 0)
                tab_bar.SelectedTabId = scroll_to_tab_id;
        }

    // Shrink widths if full tabs don't fit in their allocated space
    let section_0_w: c_float =  sections[0].Width + sections[0].Spacing;
    let section_1_w: c_float =  sections[1].Width + sections[1].Spacing;
    let section_2_w: c_float =  sections[2].Width + sections[2].Spacing;
    let mut central_section_is_visible: bool =  (section_0_w + section_2_w) < tab_bar.BarRect.GetWidth();
    let mut width_excess: c_float = 0.0;
    if (central_section_is_visible)
        width_excess = ImMax(section_1_w - (tab_bar.BarRect.GetWidth() - section_0_w - section_2_w), 0.0); // Excess used to shrink central section
    else
        width_excess = (section_0_w + section_2_w) - tab_bar.BarRect.GetWidth(); // Excess used to shrink leading/trailing section

    // With ImGuiTabBarFlags_FittingPolicyScroll policy, we will only shrink leading/trailing if the central section is not visible anymore
    if (width_excess >= 1.0 && ((tab_bar.Flags & ImGuiTabBarFlags_FittingPolicyResizeDown) || !central_section_is_visible))
    {
        let shrink_data_count: c_int = (if central_section_is_visible { sections[1].TabCount} else{ sections[0].TabCount + sections[2].TabCount});
        let shrink_data_offset: c_int = (if central_section_is_visible { sections[0].TabCount + sections[2].TabCount} else{ 0});
        layout_ops::ShrinkWidths(g.ShrinkWidthBuffer.Data + shrink_data_offset, shrink_data_count, width_excess);

        // Apply shrunk values into tabs and sections
        for (let tab_n: c_int = shrink_data_offset; tab_n < shrink_data_offset + shrink_data_count; tab_n++)
        {
            ImGuiTabItem* tab = &tab_bar.Tabs[g.ShrinkWidthBuffer[tab_n].Index];
            let shrinked_width: c_float =  IM_FLOOR(g.ShrinkWidthBuffer[tab_n].Width);
            if (shrinked_width < 0.0)
                continue;

            shrinked_width = ImMax(1.0, shrinked_width);
            let section_n: c_int = TabItemGetSectionIdx(tab);
            sections[section_n].Width -= (tab->Width - shrinked_width);
            tab->Width = shrinked_width;
        }
    }

    // Layout all active tabs
    let section_tab_index: c_int = 0;
    let tab_offset: c_float =  0.0;
    tab_bar.WidthAllTabs = 0.0;
    for (let section_n: c_int = 0; section_n < 3; section_n++)
    {
        ImGuiTabBarSection* section = &sections[section_n];
        if (section_n == 2)
            tab_offset = ImMin(ImMax(0.0, tab_bar.BarRect.GetWidth() - section->Width), tab_offset);

        for (let tab_n: c_int = 0; tab_n < section->TabCount; tab_n++)
        {
            ImGuiTabItem* tab = &tab_bar.Tabs[section_tab_index + tab_n];
            tab->Offset = tab_offset;
            tab.NameOffset = -1;
            tab_offset += tab->Width + (if tab_n < section->TabCount - 1 { g.Style.ItemInnerSpacing.x} else {0.0});
        }
        tab_bar.WidthAllTabs += ImMax(section->Width + section->layout_ops::Spacing, 0.0);
        tab_offset += section-> layout_ops::Spacing;
        section_tab_index += section->TabCount;
    }

    // Clear name buffers
    tab_bar.TabsNames.Buf.clear();

    // If we have lost the selected tab, select the next most recently active one
    if (found_selected_tab_id == false)
        tab_bar.SelectedTabId = 0;
    if (tab_bar.SelectedTabId == 0 && tab_bar.NextSelectedTabId == 0 && most_recently_selected_tab != null_mut())
        scroll_to_tab_id = tab_bar.SelectedTabId = most_recently_selected_tab.ID;

    // Lock in visible tab
    tab_bar.VisibleTabId = tab_bar.SelectedTabId;
    tab_bar->VisibleTabWasSubmitted = false;

    // CTRL+TAB can override visible tab temporarily
    if (g.NavWindowingTarget != null_mut() && g.NavWindowingTarget.DockNode && g.NavWindowingTarget.DockNode.TabBar == tab_bar)
        tab_bar.VisibleTabId = scroll_to_tab_id = g.NavWindowingTarget.TabId;

    // Update scrolling
    if (scroll_to_tab_id != 0)
        TabBarScrollToTab(tab_bar, scroll_to_tab_id, sections);
    tab_bar->ScrollingAnim = TabBarScrollClamp(tab_bar, tab_bar->ScrollingAnim);
    tab_bar->ScrollingTarget = TabBarScrollClamp(tab_bar, tab_bar->ScrollingTarget);
    if (tab_bar->ScrollingAnim != tab_bar->ScrollingTarget)
    {
        // Scrolling speed adjust itself so we can always reach our target in 1/3 seconds.
        // Teleport if we are aiming far off the visible line
        tab_bar->ScrollingSpeed = ImMax(tab_bar->ScrollingSpeed, 70f32 * g.FontSize);
        tab_bar->ScrollingSpeed = ImMax(tab_bar->ScrollingSpeed, ImFabs(tab_bar->ScrollingTarget - tab_bar->ScrollingAnim) / 0.3f32);
        let teleport: bool = (tab_bar->PrevFrameVisible + 1 < g.FrameCount) || (tab_bar->ScrollingTargetDistToVisibility > 10.0 * g.FontSize);
        tab_bar->ScrollingAnim = if teleport { tab_bar -> ScrollingTarget }else{ ImLinearSweep(tab_bar -> ScrollingAnim, tab_bar -> ScrollingTarget, g.IO.DeltaTime * tab_bar -> ScrollingSpeed)};
    }
    else
    {
        tab_bar->ScrollingSpeed = 0.0;
    }
    tab_bar->ScrollingRectMinX = tab_bar.BarRect.Min.x + sections[0].Width + sections[0].Spacing;
    tab_bar->ScrollingRectMaxX = tab_bar.BarRect.Max.x - sections[2].Width - sections[1].Spacing;

    // Actual layout in host window (we don't do it in BeginTabBar() so as not to waste an extra frame)
    let mut window = g.CurrentWindow;
    window.DC.CursorPos = tab_bar.BarRect.Min;
    ItemSize(ImVec2::new(tab_bar.WidthAllTabs, tab_bar.BarRect.GetHeight()), tab_bar->FramePadding.y);
    window.DC.IdealMaxPos.x = ImMax(window.DC.IdealMaxPos.x, tab_bar.BarRect.Min.x + tab_bar.WidthAllTabsIdeal);
}

// Dockable uses Name/ID in the global namespace. Non-dockable items use the ID stack.
static u32   TabBarCalcTabID(ImGuiTabBar* tab_bar, label: &str, docked_window: *mut ImGuiWindow)
{
    if (docked_window != null_mut())
    {
        IM_UNUSED(tab_bar);
        // IM_ASSERT(tab_bar->Flags & ImGuiTabBarFlags_DockNode);
        let mut id: ImGuiID =  docked_window.TabId;
        KeepAliveID(id);
        return id;
    }
    else
    {
        let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
        return window.GetID(label);
    }
}

staticTabBarCalcMaxTabWidth: c_float()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize * 20f32;
}

ImGuiTabItem* TabBarFindTabByID(ImGuiTabBar* tab_bar, tab_id: ImGuiID)
{
    if (tab_id != 0)
        for (let n: c_int = 0; n < tab_bar.Tabs.Size; n++)
            if (tab_bar.Tabs[n].ID == tab_id)
                return &tab_bar.Tabs[n];
    return null_mut();
}

// FIXME: See references to #2304 in TODO.txt
ImGuiTabItem* TabBarFindMostRecentlySelectedTabForActiveWindow(ImGuiTabBar* tab_bar)
{
    ImGuiTabItem* most_recently_selected_tab= null_mut();
    for (let tab_n: c_int = 0; tab_n < tab_bar.Tabs.Size; tab_n++)
    {
        ImGuiTabItem* tab = &tab_bar.Tabs[tab_n];
        if (most_recently_selected_tab == null_mut() || most_recently_selected_tab->LastFrameSelected < tab->LastFrameSelected)
            if tab.Window && tab->window.WasActive {
                most_recently_selected_tab = tab;}
    }
    return most_recently_selected_tab;
}

// The purpose of this call is to register tab in advance so we can control their order at the time they appear.
// Otherwise calling this is unnecessary as tabs are appending as needed by the BeginTabItem() function.
pub unsafe fn TabBarAddTab(ImGuiTabBar* tab_bar, ImGuiTabItemFlags tab_flags, window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(TabBarFindTabByID(tab_bar, window.TabId) == NULL);
    // IM_ASSERT(g.CurrentTabBar != tab_bar);  // Can't work while the tab bar is active as our tab doesn't have an X offset yet, in theory we could/should test something like (tab_bar->CurrFrameVisible < g.FrameCount) but we'd need to solve why triggers the commented early-out assert in BeginTabBarEx() (probably dock node going from implicit to explicit in same frame)

    if (!window.HasCloseButton)
        tab_flags |= ImGuiTabItemFlags_NoCloseButton;       // Set _NoCloseButton immediately because it will be used for first-frame width calculation.

    ImGuiTabItem new_tab;
    new_tab.ID = window.TabId;
    new_tab.Flags = tab_flags;
    new_tab.LastFrameVisible = tab_bar->CurrFrameVisible;   // Required so BeginTabBar() doesn't ditch the tab
    if (new_tab.LastFrameVisible == -1)
        new_tab.LastFrameVisible = g.FrameCount - 1;
    new_tab.Window = window;                                // Required so tab bar layout can compute the tab width before tab submission
    tab_bar.Tabs.push(new_tab);
}

// The *TabId fields be already set by the docking system _before_ the actual TabItem was created, so we clear them regardless.
pub unsafe fn TabBarRemoveTab(ImGuiTabBar* tab_bar, tab_id: ImGuiID)
{
    if (ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, tab_id))
        tab_bar.Tabs.erase(tab);
    if (tab_bar.VisibleTabId == tab_id)      { tab_bar.VisibleTabId = 0; }
    if (tab_bar.SelectedTabId == tab_id)     { tab_bar.SelectedTabId = 0; }
    if (tab_bar.NextSelectedTabId == tab_id) { tab_bar.NextSelectedTabId = 0; }
}

// Called on manual closure attempt
pub unsafe fn TabBarCloseTab(ImGuiTabBar* tab_bar, ImGuiTabItem* tab)
{
    if tab.Flags & ImGuiTabItemFlags_Button { return ; } // A button appended with TabItemButton().

    if (flag_clear(tab.Flags, ImGuiTabItemFlags_UnsavedDocument))
    {
        // This will remove a frame of lag for selecting another tab on closure.
        // However we don't run it in the case where the 'Unsaved' flag is set, so user gets a chance to fully undo the closure
        tab->WantClose = true;
        if (tab_bar.VisibleTabId == tab.ID)
        {
            tab->LastFrameVisible = -1;
            tab_bar.SelectedTabId = tab_bar.NextSelectedTabId = 0;
        }
    }
    else
    {
        // Actually select before expecting closure attempt (on an UnsavedDocument tab user is expect to e.g. show a popup)
        if (tab_bar.VisibleTabId != tab.ID)
            tab_bar.NextSelectedTabId = tab.ID;
    }
}

staticTabBarScrollClamp: c_float(ImGuiTabBar* tab_bar,scrolling: c_float)
{
    scrolling = ImMin(scrolling, tab_bar.WidthAllTabs - tab_bar.BarRect.GetWidth());
    return ImMax(scrolling, 0.0);
}

// Note: we may scroll to tab that are not selected! e.g. using keyboard arrow keys
pub unsafe fn TabBarScrollToTab(ImGuiTabBar* tab_bar, tab_id: ImGuiID, ImGuiTabBarSection* sections)
{
    ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, tab_id);
    if tab == null_mut() { return ; }
    if tab.Flags & ImGuiTabItemFlags_SectionMask_ { return ; }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let margin: c_float =  g.FontSize * 1.0; // When to scroll to make Tab N+1 visible always make a bit of N visible to suggest more scrolling area (since we don't have a scrollbar)
    let order: c_int = tab_bar->GetTabOrder(tab);

    // Scrolling happens only in the central section (leading/trailing sections are not scrolling)
    // FIXME: This is all confusing.
    let scrollable_width: c_float =  tab_bar.BarRect.GetWidth() - sections[0].Width - sections[2].Width - sections[1].Spacing;

    // We make all tabs positions all relative Sections[0].Width to make code simpler
    let tab_x1: c_float =  tab->Offset - sections[0].Width + (if order > sections[0].TabCount - 1 { - margin} else {0.0});
    let tab_x2: c_float =  tab->Offset - sections[0].Width + tab->Width + (if order + 1 < tab_bar.Tabs.Size - sections[2].TabCount { margin} else {1.0});
    tab_bar->ScrollingTargetDistToVisibility = 0.0;
    if (tab_bar->ScrollingTarget > tab_x1 || (tab_x2 - tab_x1 >= scrollable_width))
    {
        // Scroll to the left
        tab_bar->ScrollingTargetDistToVisibility = ImMax(tab_bar->ScrollingAnim - tab_x2, 0.0);
        tab_bar->ScrollingTarget = tab_x1;
    }
    else if (tab_bar->ScrollingTarget < tab_x2 - scrollable_width)
    {
        // Scroll to the right
        tab_bar->ScrollingTargetDistToVisibility = ImMax((tab_x1 - scrollable_width) - tab_bar->ScrollingAnim, 0.0);
        tab_bar->ScrollingTarget = tab_x2 - scrollable_width;
    }
}

pub unsafe fn TabBarQueueReorder(ImGuiTabBar* tab_bar, *const ImGuiTabItem tab, offset: c_int)
{
    // IM_ASSERT(offset != 0);
    // IM_ASSERT(tab_bar->ReorderRequestTabId == 0);
    tab_bar->ReorderRequestTabId = tab.ID;
    tab_bar->ReorderRequestOffset = offset;
}

pub unsafe fn TabBarQueueReorderFromMousePos(ImGuiTabBar* tab_bar, *const ImGuiTabItem src_tab, mouse_pos: ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(tab_bar->ReorderRequestTabId == 0);
    if flag_set(tab_bar.Flags, ImGuiTabBarFlags_Reorderable) == 0 { return ; }

    let is_central_section: bool = flag_set(src_tab.Flags, ImGuiTabItemFlags_SectionMask_) == 0;
    let bar_offset: c_float =  tab_bar.BarRect.Min.x - (if is_central_section { tab_bar -> ScrollingTarget }else {0});

    // Count number of contiguous tabs we are crossing over
    let dir: c_int = if (bar_offset + src_tab->Offset) > mouse_pos.x { - 1 }else {1};
    let src_idx: c_int = tab_bar.Tabs.index_from_ptr(src_tab);
    let dst_idx: c_int = src_idx;
    for (let i: c_int = src_idx; i >= 0 && i < tab_bar.Tabs.Size; i += dir)
    {
        // Reordered tabs must share the same section
        let dst_tab: *const ImGuiTabItem = &tab_bar.Tabs[i];
        if flag_set(dst_tab.Flags, ImGuiTabItemFlags_NoReorder)
            break;
        if ((dst_tab.Flags & ImGuiTabItemFlags_SectionMask_) != (src_tab.Flags & ImGuiTabItemFlags_SectionMask_))
            break;
        dst_idx = i;

        // Include spacing after tab, so when mouse cursor is between tabs we would not continue checking further tabs that are not hovered.
        let x1: c_float =  bar_offset + dst_tab->Offset - g.Style.ItemInnerSpacing.x;
        let x2: c_float =  bar_offset + dst_tab->Offset + dst_tab->Width + g.Style.ItemInnerSpacing.x;
        //GetForegroundDrawList().AddRect(ImVec2::new(x1, tab_bar.BarRect.Min.y), ImVec2::new(x2, tab_bar.BarRect.Max.y), IM_COL32(255, 0, 0, 255));
        if ((dir < 0 && mouse_pos.x > x1) || (dir > 0 && mouse_pos.x < x2))
            break;
    }

    if (dst_idx != src_idx)
        TabBarQueueReorder(tab_bar, src_tab, dst_idx - src_idx);
}

pub unsafe fn TabBarProcessReorder(ImGuiTabBar* tab_bar) -> bool
{
    ImGuiTabItem* tab1 = TabBarFindTabByID(tab_bar, tab_bar->ReorderRequestTabId);
    if tab1 == null_mut() || flag_set(tab1.Flags, ImGuiTabItemFlags_NoReorder) { return  false; }

    //IM_ASSERT(tab_bar->Flags & ImGuiTabBarFlags_Reorderable); // <- this may happen when using debug tools
    let tab2_order: c_int = tab_bar->GetTabOrder(tab1) + tab_bar->ReorderRequestOffset;
    if tab2_order < 0 || tab2_order >= tab_bar.Tabs.Size { return  false; }

    // Reordered tabs must share the same section
    // (Note: TabBarQueueReorderFromMousePos() also has a similar test but since we allow direct calls to TabBarQueueReorder() we do it here too)
    ImGuiTabItem* tab2 = &tab_bar.Tabs[tab2_order];
    if tab2.Flags & ImGuiTabItemFlags_NoReorder { return  false; }
    if flag_set(tab1.Flags, ImGuiTabItemFlags_SectionMask_) != flag_set(tab2.Flags, ImGuiTabItemFlags_SectionMask_) { return  false; }

    ImGuiTabItem item_tmp = *tab1;
    ImGuiTabItem* src_tab = if tab_bar->ReorderRequestOffset > 0 { tab1 + 1} else { tab2};
    ImGuiTabItem* dst_tab = if tab_bar->ReorderRequestOffset > 0 { tab1} else { tab2 + 1};
    let move_count: c_int = if tab_bar->ReorderRequestOffset > 0 { tab_bar->ReorderRequestOffset} else { -tab_bar->ReorderRequestOffset};
    memmove(dst_tab, src_tab, move_count * sizeof(ImGuiTabItem));
    *tab2 = item_tmp;

    if flag_set(tab_bar.Flags, ImGuiTabBarFlags_SaveSettings)
        MarkIniSettingsDirty();
    return true;
}

static ImGuiTabItem* TabBarScrollingButtons(ImGuiTabBar* tab_bar)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    const arrow_button_size: ImVec2(g.FontSize - 2.0, g.FontSize + g.Style.FramePadding.y * 2.0);
    let scrolling_buttons_width: c_float =  arrow_button_size.x * 2.0;

    let backup_cursor_pos: ImVec2 = window.DC.CursorPos;
    //window.DrawList.AddRect(ImVec2::new(tab_bar.BarRect.Max.x - scrolling_buttons_width, tab_bar.BarRect.Min.y), ImVec2::new(tab_bar.BarRect.Max.x, tab_bar.BarRect.Max.y), IM_COL32(255,0,0,255));

    let select_dir: c_int = 0;
    arrow_col: ImVec4 = g.Style.Colors[ImGuiCol_Text];
    arrow_col.w *= 0.5;

    PushStyleColor(ImGuiCol_Text, arrow_col);
    PushStyleColor(ImGuiCol_Button, ImVec4(0, 0, 0, 0));
    let backup_repeat_delay: c_float =  g.IO.KeyRepeatDelay;
    let backup_repeat_rate: c_float =  g.IO.KeyRepeatRate;
    g.IO.KeyRepeatDelay = 0.250f32;
    g.IO.KeyRepeatRate = 0.200;
    let x: c_float =  ImMax(tab_bar.BarRect.Min.x, tab_bar.BarRect.Max.x - scrolling_buttons_width);
    window.DC.CursorPos = ImVec2::new(x, tab_bar.BarRect.Min.y);
    if (ArrowButtonEx("##<", ImGuiDir_Left, arrow_button_size, ImGuiButtonFlags_PressedOnClick | ImGuiButtonFlags_Repeat))
        select_dir = -1;
    window.DC.CursorPos = ImVec2::new(x + arrow_button_size.x, tab_bar.BarRect.Min.y);
    if ArrowButtonEx("##>", ImGuiDir_Right, arrow_button_size, ImGuiButtonFlags_PressedOnClick | ImGuiButtonFlags_Repeat) {
        select_dir = 1;}
    PopStyleColor(2);
    g.IO.KeyRepeatRate = backup_repeat_rate;
    g.IO.KeyRepeatDelay = backup_repeat_delay;

    ImGuiTabItem* tab_to_scroll_to= null_mut();
    if (select_dir != 0)
        if (ImGuiTabItem* tab_item = TabBarFindTabByID(tab_bar, tab_bar.SelectedTabId))
        {
            let selected_order: c_int = tab_bar->GetTabOrder(tab_item);
            let target_order: c_int = selected_order + select_dir;

            // Skip tab item buttons until another tab item is found or end is reached
            while (tab_to_scroll_to == null_mut())
            {
                // If we are at the end of the list, still scroll to make our tab visible
                tab_to_scroll_to = &tab_bar.Tabs[if (target_order >= 0 && target_order < tab_bar.Tabs.Size) { target_order}else { selected_order}];

                // Cross through buttons
                // (even if first/last item is a button, return it so we can update the scroll)
                if flag_set(tab_to_scroll_to.Flags, ImGuiTabItemFlags_Button)
                {
                    target_order += select_dir;
                    selected_order += select_dir;
                    tab_to_scroll_to = if target_order < 0 || target_order >= tab_bar.Tabs.Size { tab_to_scroll_to} else { null_mut()};
                }
            }
        }
    window.DC.CursorPos = backup_cursor_pos;
    tab_bar.BarRect.Max.x -= scrolling_buttons_width + 1.0;

    return tab_to_scroll_to;
}

static ImGuiTabItem* TabBarTabListPopupButton(ImGuiTabBar* tab_bar)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // We use g.Style.FramePadding.y to match the square ArrowButton size
    let tab_list_popup_button_width: c_float =  g.FontSize + g.Style.FramePadding.y;
    let backup_cursor_pos: ImVec2 = window.DC.CursorPos;
    window.DC.CursorPos = ImVec2::new(tab_bar.BarRect.Min.x - g.Style.FramePadding.y, tab_bar.BarRect.Min.y);
    tab_bar.BarRect.Min.x += tab_list_popup_button_width;

    arrow_col: ImVec4 = g.Style.Colors[ImGuiCol_Text];
    arrow_col.w *= 0.5;
    PushStyleColor(ImGuiCol_Text, arrow_col);
    PushStyleColor(ImGuiCol_Button, ImVec4(0, 0, 0, 0));
    let mut open: bool =  BeginCombo("##v", null_mut(), ImGuiComboFlags_NoPreview | ImGuiComboFlags_HeightLargest);
    PopStyleColor(2);

    ImGuiTabItem* tab_to_select= null_mut();
    if (open)
    {
        for (let tab_n: c_int = 0; tab_n < tab_bar.Tabs.Size; tab_n++)
        {
            ImGuiTabItem* tab = &tab_bar.Tabs[tab_n];
            if flag_set(tab.Flags, ImGuiTabItemFlags_Button)
                continue;

            let mut  tab_name: &str = tab_bar.GetTabNametab);
            if Selectable(tab_name, tab_bar.SelectedTabId == tab.ID) {
                tab_to_select = tab;}
        }
        EndCombo();
    }

    window.DC.CursorPos = backup_cursor_pos;
    return tab_to_select;
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: BeginTabItem, EndTabItem, etc.
//-------------------------------------------------------------------------
// - BeginTabItem()
// - EndTabItem()
// - TabItemButton()
// - TabItemEx() [Internal]
// - SetTabItemClosed()
// - TabItemCalcSize() [Internal]
// - TabItemBackground() [Internal]
// - TabItemLabelAndCloseButton() [Internal]
//-------------------------------------------------------------------------

bool    BeginTabItem(label: &str,p_open: *mut bool, ImGuiTabItemFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if window.SkipItems { return  false; }

    ImGuiTabBar* tab_bar = g.CurrentTabBar;
    if (tab_bar == null_mut())
    {
        // IM_ASSERT_USER_ERROR(tab_bar, "Needs to be called between BeginTabBar() and EndTabBar()!");
        return false;
    }
    // IM_ASSERT(flag_set(flags, ImGuiTabItemFlags_Button) == 0);             // BeginTabItem() Can't be used with button flags, use TabItemButton() instead!

    let mut ret: bool =  TabItemEx(tab_bar, label, p_open, flags, null_mut());
    if (ret && flag_clear(flags, ImGuiTabItemFlags_NoPushId))
    {
        ImGuiTabItem* tab = &tab_bar.Tabs[tab_bar->LastTabItemIdx];
        PushOverrideID(tab.ID); // We already hashed 'label' so push into the ID stack directly instead of doing another hash through PushID(label)
    }
    return ret;
}

c_void    EndTabItem()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if window.SkipItems { return ; }

    ImGuiTabBar* tab_bar = g.CurrentTabBar;
    if (tab_bar == null_mut())
    {
        // IM_ASSERT_USER_ERROR(tab_bar != NULL, "Needs to be called between BeginTabBar() and EndTabBar()!");
        return;
    }
    // IM_ASSERT(tab_bar->LastTabItemIdx >= 0);
    ImGuiTabItem* tab = &tab_bar.Tabs[tab_bar->LastTabItemIdx];
    if (flag_clear(tab.Flags, ImGuiTabItemFlags_NoPushId))
        PopID();
}

bool    TabItemButton(label: &str, ImGuiTabItemFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if window.SkipItems { return  false; }

    ImGuiTabBar* tab_bar = g.CurrentTabBar;
    if (tab_bar == null_mut())
    {
        // IM_ASSERT_USER_ERROR(tab_bar != NULL, "Needs to be called between BeginTabBar() and EndTabBar()!");
        return false;
    }
    return TabItemEx(tab_bar, label, null_mut(), flags | ImGuiTabItemFlags_Button | ImGuiTabItemFlags_NoReorder, null_mut());
}

bool    TabItemEx(ImGuiTabBar* tab_bar, label: &str,p_open: *mut bool, ImGuiTabItemFlags flags, docked_window: *mut ImGuiWindow)
{
    // Layout whole tab bar if not already done
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (tab_bar->WantLayout)
    {
        ImGuiNextItemData backup_next_item_data = g.NextItemData;
        TabBarLayout(tab_bar);
        g.NextItemData = backup_next_item_data;
    }
    let mut window = g.CurrentWindow;
    if window.SkipItems { return  false; }

    let setyle = &mut g.Style;
    let mut id: ImGuiID =  TabBarCalcTabID(tab_bar, label, docked_window);

    // If the user called us with *p_open == false, we early out and don't render.
    // We make a call to ItemAdd() so that attempts to use a contextual popup menu with an implicit ID won't use an older ID.
    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags);
    if (p_open && !*p_open)
    {
        ItemAdd(ImRect(), id, null_mut(), ImGuiItemFlags_NoNav | ImGuiItemFlags_NoNavDefaultFocus);
        return false;
    }

    // IM_ASSERT(!p_open || flag_clear(flags, ImGuiTabItemFlags_Button));
    // IM_ASSERT((flags & (ImGuiTabItemFlags_Leading | ImGuiTabItemFlags_Trailing)) != (ImGuiTabItemFlags_Leading | ImGuiTabItemFlags_Trailing)); // Can't use both Leading and Trailing

    // Store into ImGuiTabItemFlags_NoCloseButton, also honor ImGuiTabItemFlags_NoCloseButton passed by user (although not documented)
    if (flags & ImGuiTabItemFlags_NoCloseButton)
        p_open= null_mut();
    else if (p_open == null_mut())
        flags |= ImGuiTabItemFlags_NoCloseButton;

    // Acquire tab data
    ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, id);
    let mut tab_is_new: bool =  false;
    if (tab == null_mut())
    {
        tab_bar.Tabs.push(ImGuiTabItem());
        tab = &tab_bar.Tabs.last().unwrap();
        tab.ID = id;
        tab_bar.TabsAddedNew = tab_is_new = true;
    }
    tab_bar->LastTabItemIdx = tab_bar.Tabs.index_from_ptr(tab);

    // Calculate tab contents size
    let size: ImVec2 = TabItemCalcSize(label, p_open != null_mut());
    tab->RequestedWidth = -1.0;
    if (g.NextItemData.Flags & ImGuiNextItemDataFlags_HasWidth)
        size.x = tab->RequestedWidth = g.NextItemData.Width;
    if (tab_is_new)
        tab->Width = ImMax(1.0, size.x);
    tab->ContentWidth = size.x;
    tab->BeginOrder = tab_bar.TabsActiveCount+= 1;

    let tab_bar_appearing: bool = (tab_bar->PrevFrameVisible + 1 < g.FrameCount);
    let tab_bar_focused: bool = flag_set(tab_bar.Flags, ImGuiTabBarFlags_IsFocused) != 0;
    let tab_appearing: bool = (tab->LastFrameVisible + 1 < g.FrameCount);
    let is_tab_button: bool = flag_set(flags, ImGuiTabItemFlags_Button);
    tab->LastFrameVisible = g.FrameCount;
    tab.Flags = flags;
    tab.Window = docked_window;

    // Append name with zero-terminator
    // (regular tabs are permitted in a DockNode tab bar, but window tabs not permitted in a non-DockNode tab bar)
    if (tab.Window != null_mut())
    {
        // IM_ASSERT(tab_bar->Flags & ImGuiTabBarFlags_DockNode);
        tab.NameOffset = -1;
    }
    else
    {
        // IM_ASSERT(tab.Window == NULL);
        tab.NameOffset = tab_bar.TabsNames.size();
        tab_bar.TabsNames.append(label, label + strlen(label) + 1); // Append name _with_ the zero-terminator.
    }

    // Update selected tab
    if (!is_tab_button)
    {
        if (tab_appearing && flag_set(tab_bar.Flags, ImGuiTabBarFlags_AutoSelectNewTabs) && tab_bar.NextSelectedTabId == 0)
            if (!tab_bar_appearing || tab_bar.SelectedTabId == 0)
                tab_bar.NextSelectedTabId = id;  // New tabs gets activated
        if (flag_set(flags, ImGuiTabItemFlags_SetSelected) && (tab_bar.SelectedTabId != id)) // _SetSelected can only be passed on explicit tab bar
            tab_bar.NextSelectedTabId = id;
    }

    // Lock visibility
    // (Note: tab_contents_visible != tab_selected... because CTRL+TAB operations may preview some tabs without selecting them!)
    let mut tab_contents_visible: bool =  (tab_bar.VisibleTabId == id);
    if (tab_contents_visible)
        tab_bar->VisibleTabWasSubmitted = true;

    // On the very first frame of a tab bar we let first tab contents be visible to minimize appearing glitches
    if (!tab_contents_visible && tab_bar.SelectedTabId == 0 && tab_bar_appearing && docked_window == null_mut())
        if tab_bar.Tabs.Size == 1 && flag_clear(tab_bar.Flags, ImGuiTabBarFlags_AutoSelectNewTabs) {
            tab_contents_visible = true;}

    // Note that tab_is_new is not necessarily the same as tab_appearing! When a tab bar stops being submitted
    // and then gets submitted again, the tabs will have 'tab_appearing=true' but 'tab_is_new=false'.
    if (tab_appearing && (!tab_bar_appearing || tab_is_new))
    {
        ItemAdd(ImRect(), id, null_mut(), ImGuiItemFlags_NoNav | ImGuiItemFlags_NoNavDefaultFocus);
        if is_tab_button { return  false; }
        return tab_contents_visible;
    }

    if (tab_bar.SelectedTabId == id)
        tab->LastFrameSelected = g.FrameCount;

    // Backup current layout position
    let backup_main_cursor_pos: ImVec2 = window.DC.CursorPos;

    // Layout
    let is_central_section: bool = flag_set(tab.Flags, ImGuiTabItemFlags_SectionMask_) == 0;
    size.x = tab->Width;
    if (is_central_section)
        window.DC.CursorPos = tab_bar.BarRect.Min + ImVec2::new(IM_FLOOR(tab->Offset - tab_bar->ScrollingAnim), 0.0);
    else
        window.DC.CursorPos = tab_bar.BarRect.Min + ImVec2::new(tab->Offset, 0.0);
    let pos: ImVec2 = window.DC.CursorPos;
    let mut bb: ImRect = ImRect::new(pos, pos + size);

    // We don't have CPU clipping primitives to clip the CloseButton (until it becomes a texture), so need to add an extra draw call (temporary in the case of vertical animation)
    let want_clip_rect: bool = is_central_section && (bb.Min.x < tab_bar->ScrollingRectMinX || bb.Max.x > tab_bar->ScrollingRectMaxX);
    if (want_clip_rect)
        PushClipRect(ImVec2::new(ImMax(bb.Min.x, tab_bar->ScrollingRectMinX), bb.Min.y - 1), ImVec2::new(tab_bar->ScrollingRectMaxX, bb.Max.y), true);

    let backup_cursor_max_pos: ImVec2 = window.DC.CursorMaxPos;
    ItemSize(bb.GetSize(), style.FramePadding.y);
    window.DC.CursorMaxPos = backup_cursor_max_pos;

    if (!ItemAdd(bb, id))
    {
        if want_clip_rect {
            PopClipRect(); }
        window.DC.CursorPos = backup_main_cursor_pos;
        return tab_contents_visible;
    }

    // Click to Select a tab
    button_flags: ImGuiButtonFlags = ((if is_tab_button {ImGuiButtonFlags_PressedOnClickRelease} else { ImGuiButtonFlags_PressedOnClick }) | ImGuiButtonFlags_AllowItemOverlap);
    if (g.DragDropActive && !g.DragDropPayload.IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW)) // FIXME: May be an opt-in property of the payload to disable this
        button_flags |= ImGuiButtonFlags_PressedOnDragDropHold;
    let mut hovered = false; let mut held = false;
    let mut pressed: bool =  ButtonBehavior(bb, id, &hovered, &held, button_flags);
    if (pressed && !is_tab_button)
        tab_bar.NextSelectedTabId = id;

    // Transfer active id window so the active id is not owned by the dock host (as StartMouseMovingWindow()
    // will only do it on the drag). This allows FocusWindow() to be more conservative in how it clears active id.
    if (held && docked_window && g.ActiveId == id && g.ActiveIdIsJustActivated)
        g.ActiveIdWindow = docked_window;

    // Allow the close button to overlap unless we are dragging (in which case we don't want any overlapping tabs to be hovered)
    if (g.ActiveId != id)
        SetItemAllowOverlap();

    // Drag and drop a single floating window node moves it
    node:*mut ImGuiDockNode = if docked_window { docked_window.DockNode} else {null_mut()};
    let single_floating_window_node: bool = node && node.IsFloatingNode() && (node.Windows.len() == 1);
    if (held && single_floating_window_node && IsMouseDragging(0, 0.0))
    {
        // Move
        StartMouseMovingWindow(docked_window);
    }
    else if (held && !tab_appearing && IsMouseDragging(0))
    {
        // Drag and drop: re-order tabs
        let drag_dir: c_int = 0;
        let drag_distance_from_edge_x: c_float =  0.0;
        if (!g.DragDropActive && ((tab_bar.Flags & ImGuiTabBarFlags_Reorderable) || (docked_window != null_mut())))
        {
            // While moving a tab it will jump on the other side of the mouse, so we also test for MouseDelta.x
            if (g.IO.MouseDelta.x < 0.0 && g.IO.MousePos.x < bb.Min.x)
            {
                drag_dir = -1;
                drag_distance_from_edge_x = bb.Min.x - g.IO.MousePos.x;
                TabBarQueueReorderFromMousePos(tab_bar, tab, g.IO.MousePos);
            }
            else if (g.IO.MouseDelta.x > 0.0 && g.IO.MousePos.x > bb.Max.x)
            {
                drag_dir = 1;
                drag_distance_from_edge_x = g.IO.MousePos.x - bb.Max.x;
                TabBarQueueReorderFromMousePos(tab_bar, tab, g.IO.MousePos);
            }
        }

        // Extract a Dockable window out of it's tab bar
        if (docked_window != null_mut() && flag_clear(docked_window.Flags, ImGuiWindowFlags_NoMove))
        {
            // We use a variable threshold to distinguish dragging tabs within a tab bar and extracting them out of the tab bar
            let mut undocking_tab: bool =  (g.DragDropActive && g.DragDropPayload.SourceId == id);
            if (!undocking_tab) //&& (!g.IO.ConfigDockingWithShift || g.IO.KeyShift)
            {
                let threshold_base: c_float =  g.FontSize;
                let threshold_x: c_float =  (threshold_base * 2.20);
                let threshold_y: c_float =  (threshold_base * 1.5) + ImClamp((ImFabs(g.IO.MouseDragMaxDistanceAbs[0].x) - threshold_base * 2.0) * 0.20, 0.0, threshold_base * 4.0);
                //GetForegroundDrawList().AddRect(ImVec2::new(bb.Min.x - threshold_x, bb.Min.y - threshold_y), ImVec2::new(bb.Max.x + threshold_x, bb.Max.y + threshold_y), IM_COL32_WHITE); // [DEBUG]

                let distance_from_edge_y: c_float =  ImMax(bb.Min.y - g.IO.MousePos.y, g.IO.MousePos.y - bb.Max.y);
                if distance_from_edge_y >= threshold_y {
                    undocking_tab = true;}
                if (drag_distance_from_edge_x > threshold_x)
                    if (drag_dir < 0 && tab_bar->GetTabOrder(tab) == 0) || (drag_dir > 0 && tab_bar->GetTabOrder(tab) == tab_bar.Tabs.Size - 1) {
                        undocking_tab = true;}
            }

            if (undocking_tab)
            {
                // Undock
                // FIXME: refactor to share more code with e.g. StartMouseMovingWindow
                DockContextQueueUndockWindow(&g, docked_window);
                g.MovingWindow = docked_window;
                SetActiveID(g.Movingwindow.MoveId, g.MovingWindow);
                g.ActiveIdClickOffset -= g.Movingwindow.Pos - bb.Min;
                g.ActiveIdNoClearOnFocusLoss = true;
                SetActiveIdUsingAllKeyboardKeys();
            }
        }
    }

// #if 0
    if (hovered && g.HoveredIdNotActiveTimer > TOOLTIP_DELAY && bb.GetWidth() < tab->ContentWidth)
    {
        // Enlarge tab display when hovering
        bb.Max.x = bb.Min.x + IM_FLOOR(ImLerp(bb.GetWidth(), tab->ContentWidth, ImSaturate((g.HoveredIdNotActiveTimer - 0.4) * 6.0)));
        display_draw_list = GetForegroundDrawList(window);
        TabItemBackground(display_draw_list, bb, flags, GetColorU32(ImGuiCol_TitleBgActive, 0.0));
    }
// #endif

    // Render tab shape
    let mut  display_draw_list: *mut ImDrawList =  window.DrawList;
    tab_col: u32 = GetColorU32(if (held || hovered) { ImGuiCol_TabHovered} else{ if tab_contents_visible { ( if tab_bar_focused {ImGuiCol_TabActive} else { ImGuiCol_TabUnfocusedActive })}else{ ( if tab_bar_focused {ImGuiCol_Tab} else { ImGuiCol_TabUnfocused }})});
    TabItemBackground(display_draw_list, bb, flags, tab_col);
    RenderNavHighlight(bb, id);

    // Select with right mouse button. This is so the common idiom for context menu automatically highlight the current widget.
    let hovered_unblocked: bool = IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup);
    if (hovered_unblocked && (IsMouseClicked(1) || IsMouseReleased(1)))
        if (!is_tab_button)
            tab_bar.NextSelectedTabId = id;

    if flag_set(tab_bar.Flags, ImGuiTabBarFlags_NoCloseWithMiddleMouseButton)
        flags |= ImGuiTabItemFlags_NoCloseWithMiddleMouseButton;

    // Render tab label, process close button
    let mut close_button_id: ImGuiID =  p_open ? GetIDWithSeed("#CLOSE", null_mut(), docked_window ? docked_window.ID : id) : 0;
    just_closed: bool;
    text_clipped: bool;
    TabItemLabelAndCloseButton(display_draw_list, bb, flags, tab_bar->FramePadding, label, id, close_button_id, tab_contents_visible, &just_closed, &text_clipped);
    if (just_closed && p_open != null_mut())
    {
        *p_open = false;
        TabBarCloseTab(tab_bar, tab);
    }

    // Forward Hovered state so IsItemHovered() after Begin() can work (even though we are technically hovering our parent)
    // That state is copied to window.DockTabItemStatusFlags by our caller.
    if (docked_window && (hovered || g.HoveredId == close_button_id))
        g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_HoveredWindow;

    // Restore main window position so user can draw there
    if want_clip_rect {
        PopClipRect(); }
    window.DC.CursorPos = backup_main_cursor_pos;

    // Tooltip
    // (Won't work over the close button because ItemOverlap systems messes up with HoveredIdTimer-> seems ok)
    // (We test IsItemHovered() to discard e.g. when another item is active or drag and drop over the tab bar, which g.HoveredId ignores)
    // FIXME: This is a mess.
    // FIXME: We may want disabled tab to still display the tooltip?
    if (text_clipped && g.HoveredId == id && !held)
        if (flag_clear(tab_bar.Flags, ImGuiTabBarFlags_NoTooltip) && flag_clear(tab.Flags, ImGuiTabItemFlags_NoTooltip))
            if (IsItemHovered(ImGuiHoveredFlags_DelayNormal))
                SetTooltip("%.*s", (FindRenderedTextEnd(label) - label), label);

    // IM_ASSERT(!is_tab_button || !(tab_bar->SelectedTabId == tab.ID && is_tab_button)); // TabItemButton should not be selected
    if is_tab_button { return  pressed; }
    return tab_contents_visible;
}

// [Public] This is call is 100% optional but it allows to remove some one-frame glitches when a tab has been unexpectedly removed.
// To use it to need to call the function SetTabItemClosed() between BeginTabBar() and EndTabBar().
// Tabs closed by the close button will automatically be flagged to avoid this issue.
c_void    SetTabItemClosed(label: &str)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut is_within_manual_tab_bar: bool =  g.CurrentTabBar && !(g.CurrentTabBar.Flags & ImGuiTabBarFlags_DockNode);
    if (is_within_manual_tab_bar)
    {
        ImGuiTabBar* tab_bar = g.CurrentTabBar;
        let mut tab_id: ImGuiID =  TabBarCalcTabID(tab_bar, label, null_mut());
        if (ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, tab_id))
            tab->WantClose = true; // Will be processed by next call to TabBarLayout()
    }
    else if (let mut window: *mut ImGuiWindow =  FindWindowByName(label))
    {
        if (window.DockIsActive)
            if (node:*mut ImGuiDockNode = window.DockNode)
            {
                let mut tab_id: ImGuiID =  TabBarCalcTabID(node.TabBar, label, window);
                TabBarRemoveTab(node.TabBar, tab_id);
                window.DockTabWantClose = true;
            }
    }
}

pub unsafe fn TabItemCalcSize(label: &str, has_close_button: bool) -> ImVec2
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);
    let size: ImVec2 = ImVec2::new(label_size.x + g.Style.FramePadding.x, label_size.y + g.Style.FramePadding.y * 2.0);
    if (has_close_button)
        size.x += g.Style.FramePadding.x + (g.Style.ItemInnerSpacing.x + g.FontSize); // We use Y intentionally to fit the close button circle.
    else
        size.x += g.Style.FramePadding.x + 1.0;
    return ImVec2::new(ImMin(size.x, TabBarCalcMaxTabWidth()), size.y);
}

pub unsafe fn TabItemBackground(draw_list: *mut ImDrawList, bb: &ImRect, ImGuiTabItemFlags flags, col: u32)
{
    // While rendering tabs, we trim 1 pixel off the top of our bounding box so they can fit within a regular frame height while looking "detached" from it.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let width: c_float =  bb.GetWidth();
    IM_UNUSED(flags);
    // IM_ASSERT(width > 0.0);
    let rounding: c_float =  ImMax(0.0, ImMin(if flag_set(flags, ImGuiTabItemFlags_Button) { g.Style.FrameRounding} else {g.Style.TabRounding}, width * 0.5 - 1.0));
    let y1: c_float =  bb.Min.y + 1.0;
    let y2: c_float =  bb.Max.y + (if flag_set(flags, ImGuiTabItemFlags_Preview) { 0.0} else {- 1.0});
    draw_list.PathLineTo(ImVec2::new(bb.Min.x, y2));
    draw_list.PathArcToFast(ImVec2::new(bb.Min.x + rounding, y1 + rounding), rounding, 6, 9);
    draw_list.PathArcToFast(ImVec2::new(bb.Max.x - rounding, y1 + rounding), rounding, 9, 12);
    draw_list.PathLineTo(ImVec2::new(bb.Max.x, y2));
    draw_list.PathFillConvex(col);
    if (g.Style.TabBorderSize > 0.0)
    {
        draw_list.PathLineTo(ImVec2::new(bb.Min.x + 0.5, y2));
        draw_list.PathArcToFast(ImVec2::new(bb.Min.x + rounding + 0.5, y1 + rounding + 0.5), rounding, 6, 9);
        draw_list.PathArcToFast(ImVec2::new(bb.Max.x - rounding - 0.5, y1 + rounding + 0.5), rounding, 9, 12);
        draw_list.PathLineTo(ImVec2::new(bb.Max.x - 0.5, y2));
        draw_list.PathStroke(GetColorU32(ImGuiCol_Border, 0.0), 0, g.Style.TabBorderSize);
    }
}

// Render text label (with custom clipping) + Unsaved Document marker + Close Button logic
// We tend to lock style.FramePadding for a given tab-bar, hence the 'frame_padding' parameter.
pub unsafe fn TabItemLabelAndCloseButton(draw_list: *mut ImDrawList, bb: &ImRect, ImGuiTabItemFlags flags, frame_padding: ImVec2, label: &str, tab_id: ImGuiID, close_button_id: ImGuiID, is_contents_visible: bool,out_just_closed: *mut bool,out_text_clipped: *mut bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);

    if (out_just_closed)
        *out_just_closed = false;
    if (out_text_clipped)
        *out_text_clipped = false;

    if bb.GetWidth() <= 1.0 { return ; }

    // In Style V2 we'll have full override of all colors per state (e.g. focused, selected)
    // But right now if you want to alter text color of tabs this is what you need to do.
// #if 0
    let backup_alpha: c_float =  g.Style.Alpha;
    if (!is_contents_visible)
        g.Style.Alpha *= 0.7f;
// #endif

    // Render text label (with clipping + alpha gradient) + unsaved marker
    let mut text_pixel_clip_bb: ImRect = ImRect::new(bb.Min.x + frame_padding.x, bb.Min.y + frame_padding.y, bb.Max.x - frame_padding.x, bb.Max.y);
    let text_ellipsis_clip_bb: ImRect =  text_pixel_clip_bb;

    // Return clipped state ignoring the close button
    if (out_text_clipped)
    {
        *out_text_clipped = (text_ellipsis_clip_bb.Min.x + label_size.x) > text_pixel_clip_bb.Max.x;
        //draw_list.AddCircle(text_ellipsis_clip_bb.Min, 3.0, *out_text_clipped ? IM_COL32(255, 0, 0, 255) : IM_COL32(0, 255, 0, 255));
    }

    let button_sz: c_float =  g.FontSize;
    const button_pos: ImVec2(ImMax(bb.Min.x, bb.Max.x - frame_padding.x * 2.0 - button_sz), bb.Min.y);

    // Close Button & Unsaved Marker
    // We are relying on a subtle and confusing distinction between 'hovered' and 'g.HoveredId' which happens because we are using ImGuiButtonFlags_AllowOverlapMode + SetItemAllowOverlap()
    //  'hovered' will be true when hovering the Tab but NOT when hovering the close button
    //  'g.HoveredId==id' will be true when hovering the Tab including when hovering the close button
    //  'g.ActiveId==close_button_id' will be true when we are holding on the close button, in which case both hovered booleans are false
    let mut close_button_pressed: bool =  false;
    let mut close_button_visible: bool =  false;
    if (close_button_id != 0)
        if (is_contents_visible || bb.GetWidth() >= ImMax(button_sz, g.Style.TabMinWidthForCloseButton))
            if g.HoveredId == tab_id || g.HoveredId == close_button_id || g.ActiveId == tab_id || g.ActiveId == close_button_id {
                close_button_visible = true;}
    let mut unsaved_marker_visible: bool =  flag_set(flags, ImGuiTabItemFlags_UnsavedDocument) && (button_pos.x + button_sz <= bb.Max.x);

    if (close_button_visible)
    {
        last_item_backup: ImGuiLastItemData = g.LastItemData;
        PushStyleVar(ImGuiStyleVar_FramePadding, frame_padding);
        if button_ops::CloseButton(close_button_id, button_pos) {
            close_button_pressed = true;}
        PopStyleVar();
        g.LastItemData = last_item_backup;

        // Close with middle mouse button
        if flag_clear(flags, ImGuiTabItemFlags_NoCloseWithMiddleMouseButton) && IsMouseClicked(2) {
            close_button_pressed = true;}
    }
    else if (unsaved_marker_visible)
    {
        let mut bullet_bb: ImRect = ImRect::new(button_pos, button_pos + ImVec2::new(button_sz, button_sz) + g.Style.FramePadding * 2.0);
        RenderBullet(draw_list, bullet_bb.GetCenter(), GetColorU32(ImGuiCol_Text, 0.0));
    }

    // This is all rather complicated
    // (the main idea is that because the close button only appears on hover, we don't want it to alter the ellipsis position)
    // FIXME: if FramePadding is noticeably large, ellipsis_max_x will be wrong here (e.g. #3497), maybe for consistency that parameter of RenderTextEllipsis() shouldn't exist..
    let ellipsis_max_x: c_float =  if close_button_visible { text_pixel_clip_bb.Max.x} else {bb.Max.x - 1.0};
    if (close_button_visible || unsaved_marker_visible)
    {
        text_pixel_clip_bb.Max.x -= if close_button_visible { (button_sz)} else {button_sz * 0.8};
        text_ellipsis_clip_bb.Max.x -= if unsaved_marker_visible { (button_sz * 0.8)} else {0.0};
        ellipsis_max_x = text_pixel_clip_bb.Max.x;
    }
    RenderTextEllipsis(draw_list, text_ellipsis_clip_bb.Min, text_ellipsis_clip_bb.Max, text_pixel_clip_bb.Max.x, ellipsis_max_x, label, null_mut(), &label_size);

// #if 0
    if (!is_contents_visible)
        g.Style.Alpha = backup_alpha;
// #endif

    if (out_just_closed)
        *out_just_closed = close_button_pressed;
}


// #endif // #ifndef IMGUI_DISABLE
