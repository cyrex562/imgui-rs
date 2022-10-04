// dear imgui, v1.89 WIP
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
// #pragma clang diagnostic ignored "-Wfloat-equal"                    // warning: comparing floating point with == or != is unsafe // storing and comparing against same constants (typically 0f32) is ok.
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

// Widgets
static const c_float          DRAGDROP_HOLD_TO_OPEN_TIMER = 0.70f32;    // Time for drag-hold to activate items accepting the ImGuiButtonFlags_PressedOnDragDropHold button behavior.
static const c_float          DRAG_MOUSE_THRESHOLD_FACTOR = 0.50f32;    // Multiplier for the default value of io.MouseDragThreshold to make DragFloat/DragInt react faster to mouse drags.

// Those MIN/MAX values are not define because we need to point to them
static const i8    IM_S8_MIN  = -128;
static const i8    IM_S8_MAX  = 127;
static const c_uchar  IM_U8_MIN  = 0;
static const c_uchar  IM_U8_MAX  = 0xFF;
static const signed c_short   IM_S16_MIN = -32768;
static const signed c_short   IM_S16_MAX = 32767;
static const unsigned c_short IM_U16_MIN = 0;
static const unsigned c_short IM_U16_MAX = 0xFFFF;
static const i32          IM_S32_MIN = INT_MIN;    // (-2147483647 - 1), (0x80000000);
static const i32          IM_S32_MAX = INT_MAX;    // (2147483647), (0x7FFFFFF0f32)
static const u32          IM_U32_MIN = 0;
static const u32          IM_U32_MAX = UINT_MAX;   // (0xFFFFFFF0f32)
// #ifdef LLONG_MIN
static const ImS64          IM_S64_MIN = LLONG_MIN;  // (-9223372036854775807ll - 1ll);
static const ImS64          IM_S64_MAX = LLONG_MAX;  // (9223372036854775807ll);
// #else
static const ImS64          IM_S64_MIN = -9223372036854775807LL - 1;
static const ImS64          IM_S64_MAX = 9223372036854775807LL;
// #endif
static const u64          IM_U64_MIN = 0;
// #ifdef ULLONG_MAX
static const u64          IM_U64_MAX = ULLONG_MAX; // (0xFFFFFFFFFFFFFFFFull);
// #else
static const u64          IM_U64_MAX = (2ULL * 9223372036854775807LL + 1);
// #endif

//-------------------------------------------------------------------------
// [SECTION] Forward Declarations
//-------------------------------------------------------------------------

// For InputTextEx()
static bool             InputTextFilterCharacter(*mut c_uint p_char, ImGuiInputTextFlags flags, ImGuiInputTextCallback callback, *mut c_void user_data, ImGuiInputSource input_source);
static c_int              InputTextCalcTextLenAndLineCount(text_begin: *const c_char, *const *mut char out_text_end);
static ImVec2           InputTextCalcTextSizeW(*const ImWchar text_begin, *const ImWchar text_end, *const *mut let remaining: ImWchar = null_mut(), *mut let mut out_offset: ImVec2 =  null_mut(), let mut stop_on_new_line: bool =  false);

//-------------------------------------------------------------------------
// [SECTION] Widgets: Text, etc.
//-------------------------------------------------------------------------
// - TextEx() [Internal]
// - TextUnformatted()
// - Text()
// - TextV()
// - TextColored()
// - TextColoredV()
// - TextDisabled()
// - TextDisabledV()
// - TextWrapped()
// - TextWrappedV()
// - LabelText()
// - LabelTextV()
// - BulletText()
// - BulletTextV()
//-------------------------------------------------------------------------

c_void TextEx(text: *const c_char, text_end: *const c_char, ImGuiTextFlags flags)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return;
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Accept null ranges
    if (text == text_end)
        text = text_end = "";

    // Calculate length
    let mut  text_begin: *const c_char = text;
    if (text_end == null_mut())
        text_end = text + strlen(text); // FIXME-OPT

    const ImVec2 text_pos(window.DC.CursorPos.x, window.DC.CursorPos.y + window.DC.CurrLineTextBaseOffset);
    let wrap_pos_x: c_float =  window.DC.TextWrapPos;
    let wrap_enabled: bool = (wrap_pos_x >= 0f32);
    if (text_end - text <= 2000 || wrap_enabled)
    {
        // Common case
        let wrap_width: c_float =  wrap_enabled ? CalcWrapWidthForPos(window.DC.CursorPos, wrap_pos_x) : 0f32;
        let text_size: ImVec2 = CalcTextSize(text_begin, text_end, false, wrap_width);

        let mut bb: ImRect = ImRect::new(text_pos, text_pos + text_size);
        ItemSize(text_size, 0f32);
        if (!ItemAdd(bb, 0))
            return;

        // Render (we don't hide text after ## in this end-user function)
        RenderTextWrapped(bb.Min, text_begin, text_end, wrap_width);
    }
    else
    {
        // Long text!
        // Perform manual coarse clipping to optimize for long multi-line text
        // - From this point we will only compute the width of lines that are visible. Optimization only available when word-wrapping is disabled.
        // - We also don't vertically center the text within the line full height, which is unlikely to matter because we are likely the biggest and only item on the line.
        // - We use memchr(), pay attention that well optimized versions of those str/mem functions are much faster than a casually written loop.
        let mut  line: *const c_char = text;
        let line_height: c_float =  GetTextLineHeight();
        ImVec2 text_size(0, 0);

        // Lines to skip (can't skip when logging text)
        let pos: ImVec2 = text_pos;
        if (!g.LogEnabled)
        {
            let lines_skippable: c_int = ((window.ClipRect.Min.y - text_pos.y) / line_height);
            if (lines_skippable > 0)
            {
                let lines_skipped: c_int = 0;
                while (line < text_end && lines_skipped < lines_skippable)
                {
                    let mut  line_end: *const c_char =memchr(line, '\n', text_end - line);
                    if (!line_end)
                        line_end = text_end;
                    if ((flags & ImGuiTextFlags_NoWidthForLargeClippedText) == 0)
                        text_size.x = ImMax(text_size.x, CalcTextSize(line, line_end).x);
                    line = line_end + 1;
                    lines_skipped+= 1;
                }
                pos.y += lines_skipped * line_height;
            }
        }

        // Lines to render
        if (line < text_end)
        {
            let mut line_rect: ImRect = ImRect::new(pos, pos + ImVec2::new(f32::MAX, line_height));
            while (line < text_end)
            {
                if (IsClippedEx(line_rect, 0))
                    break;

                let mut  line_end: *const c_char =memchr(line, '\n', text_end - line);
                if (!line_end)
                    line_end = text_end;
                text_size.x = ImMax(text_size.x, CalcTextSize(line, line_end).x);
                RenderText(pos, line, line_end, false);
                line = line_end + 1;
                line_rect.Min.y += line_height;
                line_rect.Max.y += line_height;
                pos.y += line_height;
            }

            // Count remaining lines
            let lines_skipped: c_int = 0;
            while (line < text_end)
            {
                let mut  line_end: *const c_char =memchr(line, '\n', text_end - line);
                if (!line_end)
                    line_end = text_end;
                if ((flags & ImGuiTextFlags_NoWidthForLargeClippedText) == 0)
                    text_size.x = ImMax(text_size.x, CalcTextSize(line, line_end).x);
                line = line_end + 1;
                lines_skipped+= 1;
            }
            pos.y += lines_skipped * line_height;
        }
        text_size.y = (pos - text_pos).y;

        let mut bb: ImRect = ImRect::new(text_pos, text_pos + text_size);
        ItemSize(text_size, 0f32);
        ItemAdd(bb, 0);
    }
}

c_void TextUnformatted(text: *const c_char, text_end: *const c_char)
{
    TextEx(text, text_end, ImGuiTextFlags_NoWidthForLargeClippedText);
}

c_void Text(fmt: *const c_char, ...)
{
    va_list args;
    va_start(args, fmt);
    TextV(fmt, args);
    va_end(args);
}

c_void TextV(fmt: *const c_char, va_list args)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    // FIXME-OPT: Handle the %s shortcut?
    text: *const c_char, *text_end;
    ImFormatStringToTempBufferV(&text, &text_end, fmt, args);
    TextEx(text, text_end, ImGuiTextFlags_NoWidthForLargeClippedText);
}

c_void TextColored(const ImVec4& col, fmt: *const c_char, ...)
{
    va_list args;
    va_start(args, fmt);
    TextColoredV(col, fmt, args);
    va_end(args);
}

c_void TextColoredV(const ImVec4& col, fmt: *const c_char, va_list args)
{
    PushStyleColor(ImGuiCol_Text, col);
    if (fmt[0] == '%' && fmt[1] == 's' && fmt[2] == 0)
        TextEx(va_arg(args, *const char), null_mut(), ImGuiTextFlags_NoWidthForLargeClippedText); // Skip formatting
    else
        TextV(fmt, args);
    PopStyleColor();
}

c_void TextDisabled(fmt: *const c_char, ...)
{
    va_list args;
    va_start(args, fmt);
    TextDisabledV(fmt, args);
    va_end(args);
}

c_void TextDisabledV(fmt: *const c_char, va_list args)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    PushStyleColor(ImGuiCol_Text, g.Style.Colors[ImGuiCol_TextDisabled]);
    if (fmt[0] == '%' && fmt[1] == 's' && fmt[2] == 0)
        TextEx(va_arg(args, *const char), null_mut(), ImGuiTextFlags_NoWidthForLargeClippedText); // Skip formatting
    else
        TextV(fmt, args);
    PopStyleColor();
}

c_void TextWrapped(fmt: *const c_char, ...)
{
    va_list args;
    va_start(args, fmt);
    TextWrappedV(fmt, args);
    va_end(args);
}

c_void TextWrappedV(fmt: *const c_char, va_list args)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut need_backup: bool =  (g.Currentwindow.DC.TextWrapPos < 0f32);  // Keep existing wrap position if one is already set
    if (need_backup)
        PushTextWrapPos(0f32);
    if (fmt[0] == '%' && fmt[1] == 's' && fmt[2] == 0)
        TextEx(va_arg(args, *const char), null_mut(), ImGuiTextFlags_NoWidthForLargeClippedText); // Skip formatting
    else
        TextV(fmt, args);
    if (need_backup)
        PopTextWrapPos();
}

c_void LabelText(label: *const c_char, fmt: *const c_char, ...)
{
    va_list args;
    va_start(args, fmt);
    LabelTextV(label, fmt, args);
    va_end(args);
}

// Add a label+text combo aligned to other label+value widgets
c_void LabelTextV(label: *const c_char, fmt: *const c_char, va_list args)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    const let mut style = &mut g.Style;
    let w: c_float =  CalcItemWidth();

    value_text_begin: *const c_char, *value_text_end;
    ImFormatStringToTempBufferV(&value_text_begin, &value_text_end, fmt, args);
    let value_size: ImVec2 = CalcTextSize(value_text_begin, value_text_end, false);
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);

    let pos: ImVec2 = window.DC.CursorPos;
    let mut value_bb: ImRect = ImRect::new(pos, pos + ImVec2::new(w, value_size.y + style.FramePadding.y * 2));
    let mut total_bb: ImRect = ImRect::new(pos, pos + ImVec2::new(w + (label_size.x > 0f32 ? style.ItemInnerSpacing.x + label_size.x : 0f32), ImMax(value_size.y, label_size.y) + style.FramePadding.y * 2));
    ItemSize(total_bb, style.FramePadding.y);
    if (!ItemAdd(total_bb, 0))
        return;

    // Render
    RenderTextClipped(value_bb.Min + style.FramePadding, value_bb.Max, value_text_begin, value_text_end, &value_size, ImVec2::new2(0f32, 0f32));
    if (label_size.x > 0f32)
        RenderText(ImVec2::new(value_bb.Max.x + style.ItemInnerSpacing.x, value_bb.Min.y + style.FramePadding.y), label);
}

c_void BulletText(fmt: *const c_char, ...)
{
    va_list args;
    va_start(args, fmt);
    BulletTextV(fmt, args);
    va_end(args);
}

// Text with a little bullet aligned to the typical tree node.
c_void BulletTextV(fmt: *const c_char, va_list args)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    const let mut style = &mut g.Style;

    text_begin: *const c_char, *text_end;
    ImFormatStringToTempBufferV(&text_begin, &text_end, fmt, args);
    let label_size: ImVec2 = CalcTextSize(text_begin, text_end, false);
    let total_size: ImVec2 = ImVec2::new(g.FontSize + (label_size.x > 0f32 ? (label_size.x + style.FramePadding.x * 2) : 0f32), label_size.y);  // Empty text doesn't add padding
    let pos: ImVec2 = window.DC.CursorPos;
    pos.y += window.DC.CurrLineTextBaseOffset;
    ItemSize(total_size, 0f32);
    let mut bb: ImRect = ImRect::new(pos, pos + total_size);
    if (!ItemAdd(bb, 0))
        return;

    // Render
    let mut text_col: u32 = GetColorU32(ImGuiCol_Text);
    RenderBullet(window.DrawList, bb.Min + ImVec2::new(style.FramePadding.x + g.FontSize * 0.5f32, g.FontSize * 0.5f32), text_col);
    RenderText(bb.Min + ImVec2::new(g.FontSize + style.FramePadding.x * 2, 0f32), text_begin, text_end, false);
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: Main
//-------------------------------------------------------------------------
// - ButtonBehavior() [Internal]
// - Button()
// - SmallButton()
// - InvisibleButton()
// - ArrowButton()
// - CloseButton() [Internal]
// - CollapseButton() [Internal]
// - GetWindowScrollbarID() [Internal]
// - GetWindowScrollbarRect() [Internal]
// - Scrollbar() [Internal]
// - ScrollbarEx() [Internal]
// - Image()
// - ImageButton()
// - Checkbox()
// - CheckboxFlagsT() [Internal]
// - CheckboxFlags()
// - RadioButton()
// - ProgressBar()
// - Bullet()
//-------------------------------------------------------------------------

// The ButtonBehavior() function is key to many interactions and used by many/most widgets.
// Because we handle so many cases (keyboard/gamepad navigation, drag and drop) and many specific behavior (via ImGuiButtonFlags_),
// this code is a little complex.
// By far the most common path is interacting with the Mouse using the default ImGuiButtonFlags_PressedOnClickRelease button behavior.
// See the series of events below and the corresponding state reported by dear imgui:
//------------------------------------------------------------------------------------------------------------------------------------------------
// with PressedOnClickRelease:             return-value  IsItemHovered()  IsItemActive()  IsItemActivated()  IsItemDeactivated()  IsItemClicked()
//   Frame N+0 (mouse is outside bb)        -             -                -               -                  -                    -
//   Frame N+1 (mouse moves inside bb)      -             true             -               -                  -                    -
//   Frame N+2 (mouse button is down)       -             true             true            true               -                    true
//   Frame N+3 (mouse button is down)       -             true             true            -                  -                    -
//   Frame N+4 (mouse moves outside bb)     -             -                true            -                  -                    -
//   Frame N+5 (mouse moves inside bb)      -             true             true            -                  -                    -
//   Frame N+6 (mouse button is released)   true          true             -               -                  true                 -
//   Frame N+7 (mouse button is released)   -             true             -               -                  -                    -
//   Frame N+8 (mouse moves outside bb)     -             -                -               -                  -                    -
//------------------------------------------------------------------------------------------------------------------------------------------------
// with PressedOnClick:                    return-value  IsItemHovered()  IsItemActive()  IsItemActivated()  IsItemDeactivated()  IsItemClicked()
//   Frame N+2 (mouse button is down)       true          true             true            true               -                    true
//   Frame N+3 (mouse button is down)       -             true             true            -                  -                    -
//   Frame N+6 (mouse button is released)   -             true             -               -                  true                 -
//   Frame N+7 (mouse button is released)   -             true             -               -                  -                    -
//------------------------------------------------------------------------------------------------------------------------------------------------
// with PressedOnRelease:                  return-value  IsItemHovered()  IsItemActive()  IsItemActivated()  IsItemDeactivated()  IsItemClicked()
//   Frame N+2 (mouse button is down)       -             true             -               -                  -                    true
//   Frame N+3 (mouse button is down)       -             true             -               -                  -                    -
//   Frame N+6 (mouse button is released)   true          true             -               -                  -                    -
//   Frame N+7 (mouse button is released)   -             true             -               -                  -                    -
//------------------------------------------------------------------------------------------------------------------------------------------------
// with PressedOnDoubleClick:              return-value  IsItemHovered()  IsItemActive()  IsItemActivated()  IsItemDeactivated()  IsItemClicked()
//   Frame N+0 (mouse button is down)       -             true             -               -                  -                    true
//   Frame N+1 (mouse button is down)       -             true             -               -                  -                    -
//   Frame N+2 (mouse button is released)   -             true             -               -                  -                    -
//   Frame N+3 (mouse button is released)   -             true             -               -                  -                    -
//   Frame N+4 (mouse button is down)       true          true             true            true               -                    true
//   Frame N+5 (mouse button is down)       -             true             true            -                  -                    -
//   Frame N+6 (mouse button is released)   -             true             -               -                  true                 -
//   Frame N+7 (mouse button is released)   -             true             -               -                  -                    -
//------------------------------------------------------------------------------------------------------------------------------------------------
// Note that some combinations are supported,
// - PressedOnDragDropHold can generally be associated with any flag.
// - PressedOnDoubleClick can be associated by PressedOnClickRelease/PressedOnRelease, in which case the second release event won't be reported.
//------------------------------------------------------------------------------------------------------------------------------------------------
// The behavior of the return-value changes when ImGuiButtonFlags_Repeat is set:
//                                         Repeat+                  Repeat+           Repeat+             Repeat+
//                                         PressedOnClickRelease    PressedOnClick    PressedOnRelease    PressedOnDoubleClick
//-------------------------------------------------------------------------------------------------------------------------------------------------
//   Frame N+0 (mouse button is down)       -                        true              -                   true
//   ...                                    -                        -                 -                   -
//   Frame N + RepeatDelay                  true                     true              -                   true
//   ...                                    -                        -                 -                   -
//   Frame N + RepeatDelay + RepeatRate*N   true                     true              -                   true
//-------------------------------------------------------------------------------------------------------------------------------------------------

bool ButtonBehavior(bb: &ImRect, id: ImGuiID, *mut out_hovered: bool, *mut out_held: bool, ImGuiButtonFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = GetCurrentWindow();

    // Default only reacts to left mouse button
    if ((flags & ImGuiButtonFlags_MouseButtonMask_) == 0)
        flags |= ImGuiButtonFlags_MouseButtonDefault_;

    // Default behavior requires click + release inside bounding box
    if ((flags & ImGuiButtonFlags_PressedOnMask_) == 0)
        flags |= ImGuiButtonFlags_PressedOnDefault_;

    *mut ImGuiWindow backup_hovered_window = g.HoveredWindow;
    let flatten_hovered_children: bool = (flags & ImGuiButtonFlags_FlattenChildren) && g.HoveredWindow && g.Hoveredwindow.RootWindowDockTree == window.RootWindowDockTree;
    if (flatten_hovered_children)
        g.HoveredWindow = window;

// #ifdef IMGUI_ENABLE_TEST_ENGINE
    if (id != 0 && g.LastItemData.ID != id)
        IMGUI_TEST_ENGINE_ITEM_ADD(bb, id);
// #endif

    let mut pressed: bool =  false;
    let mut hovered: bool =  ItemHoverable(bb, id);

    // Drag source doesn't report as hovered
    if (hovered && g.DragDropActive && g.DragDropPayload.SourceId == id && !(g.DragDropSourceFlags & ImGuiDragDropFlags_SourceNoDisableHover))
        hovered = false;

    // Special mode for Drag and Drop where holding button pressed for a long time while dragging another item triggers the button
    if (g.DragDropActive && (flags & ImGuiButtonFlags_PressedOnDragDropHold) && !(g.DragDropSourceFlags & ImGuiDragDropFlags_SourceNoHoldToOpenOthers))
        if (IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByActiveItem))
        {
            hovered = true;
            SetHoveredID(id);
            if (g.HoveredIdTimer - g.IO.DeltaTime <= DRAGDROP_HOLD_TO_OPEN_TIMER && g.HoveredIdTimer >= DRAGDROP_HOLD_TO_OPEN_TIMER)
            {
                pressed = true;
                g.DragDropHoldJustPressedId = id;
                FocusWindow(window);
            }
        }

    if (flatten_hovered_children)
        g.HoveredWindow = backup_hovered_window;

    // AllowOverlap mode (rarely used) requires previous frame HoveredId to be null or to match. This allows using patterns where a later submitted widget overlaps a previous one.
    if (hovered && (flags & ImGuiButtonFlags_AllowItemOverlap) && (g.HoveredIdPreviousFrame != id && g.HoveredIdPreviousFrame != 0))
        hovered = false;

    // Mouse handling
    if (hovered)
    {
        if (!(flags & ImGuiButtonFlags_NoKeyModifiers) || (!g.IO.KeyCtrl && !g.IO.KeyShift && !g.IO.KeyAlt))
        {
            // Poll buttons
            let mouse_button_clicked: c_int = -1;
            if ((flags & ImGuiButtonFlags_MouseButtonLeft) && g.IO.MouseClicked[0])         { mouse_button_clicked = 0; }
            else if ((flags & ImGuiButtonFlags_MouseButtonRight) && g.IO.MouseClicked[1])   { mouse_button_clicked = 1; }
            else if ((flags & ImGuiButtonFlags_MouseButtonMiddle) && g.IO.MouseClicked[2])  { mouse_button_clicked = 2; }

            if (mouse_button_clicked != -1 && g.ActiveId != id)
            {
                if (flags & (ImGuiButtonFlags_PressedOnClickRelease | ImGuiButtonFlags_PressedOnClickReleaseAnywhere))
                {
                    SetActiveID(id, window);
                    g.ActiveIdMouseButton = mouse_button_clicked;
                    if (!(flags & ImGuiButtonFlags_NoNavFocus))
                        SetFocusID(id, window);
                    FocusWindow(window);
                }
                if ((flags & ImGuiButtonFlags_PressedOnClick) || ((flags & ImGuiButtonFlags_PressedOnDoubleClick) && g.IO.MouseClickedCount[mouse_button_clicked] == 2))
                {
                    pressed = true;
                    if (flags & ImGuiButtonFlags_NoHoldingActiveId)
                        ClearActiveID();
                    else
                        SetActiveID(id, window); // Hold on ID
                    if (!(flags & ImGuiButtonFlags_NoNavFocus))
                        SetFocusID(id, window);
                    g.ActiveIdMouseButton = mouse_button_clicked;
                    FocusWindow(window);
                }
            }
            if (flags & ImGuiButtonFlags_PressedOnRelease)
            {
                let mouse_button_released: c_int = -1;
                if ((flags & ImGuiButtonFlags_MouseButtonLeft) && g.IO.MouseReleased[0])        { mouse_button_released = 0; }
                else if ((flags & ImGuiButtonFlags_MouseButtonRight) && g.IO.MouseReleased[1])  { mouse_button_released = 1; }
                else if ((flags & ImGuiButtonFlags_MouseButtonMiddle) && g.IO.MouseReleased[2]) { mouse_button_released = 2; }
                if (mouse_button_released != -1)
                {
                    let has_repeated_at_least_once: bool = (flags & ImGuiButtonFlags_Repeat) && g.IO.MouseDownDurationPrev[mouse_button_released] >= g.IO.KeyRepeatDelay; // Repeat mode trumps on release behavior
                    if (!has_repeated_at_least_once)
                        pressed = true;
                    if (!(flags & ImGuiButtonFlags_NoNavFocus))
                        SetFocusID(id, window);
                    ClearActiveID();
                }
            }

            // 'Repeat' mode acts when held regardless of _PressedOn flags (see table above).
            // Relies on repeat logic of IsMouseClicked() but we may as well do it ourselves if we end up exposing finer RepeatDelay/RepeatRate settings.
            if (g.ActiveId == id && (flags & ImGuiButtonFlags_Repeat))
                if (g.IO.MouseDownDuration[g.ActiveIdMouseButton] > 0f32 && IsMouseClicked(g.ActiveIdMouseButton, true))
                    pressed = true;
        }

        if (pressed)
            g.NavDisableHighlight = true;
    }

    // Gamepad/Keyboard navigation
    // We report navigated item as hovered but we don't set g.HoveredId to not interfere with mouse.
    if (g.NavId == id && !g.NavDisableHighlight && g.NavDisableMouseHover && (g.ActiveId == 0 || g.ActiveId == id || g.ActiveId == window.MoveId))
        if (!(flags & ImGuiButtonFlags_NoHoveredOnFocus))
            hovered = true;
    if (g.NavActivateDownId == id)
    {
        let mut nav_activated_by_code: bool =  (g.NavActivateId == id);
        let mut nav_activated_by_inputs: bool =  (g.NavActivatePressedId == id);
        if (!nav_activated_by_inputs && (flags & ImGuiButtonFlags_Repeat))
        {
            // Avoid pressing both keys from triggering double amount of repeat events
            let key1: *let mut Data: ImGuiKey =  GetKeyData(ImGuiKey_Space);
            let key2: *let mut Data: ImGuiKey =  GetKeyData(ImGuiKey_NavGamepadActivate);
            let t1: c_float =  ImMax(key1.DownDuration, key2.DownDuration);
            nav_activated_by_inputs = CalcTypematicRepeatAmount(t1 - g.IO.DeltaTime, t1, g.IO.KeyRepeatDelay, g.IO.KeyRepeatRate) > 0;
        }
        if (nav_activated_by_code || nav_activated_by_inputs)
        {
            // Set active id so it can be queried by user via IsItemActive(), equivalent of holding the mouse button.
            pressed = true;
            SetActiveID(id, window);
            g.ActiveIdSource = ImGuiInputSource_Nav;
            if (!(flags & ImGuiButtonFlags_NoNavFocus))
                SetFocusID(id, window);
        }
    }

    // Process while held
    let mut held: bool =  false;
    if (g.ActiveId == id)
    {
        if (g.ActiveIdSource == ImGuiInputSource_Mouse)
        {
            if (g.ActiveIdIsJustActivated)
                g.ActiveIdClickOffset = g.IO.MousePos - bb.Min;

            let mouse_button: c_int = g.ActiveIdMouseButton;
            // IM_ASSERT(mouse_button >= 0 && mouse_button < ImGuiMouseButton_COUNT);
            if (g.IO.MouseDown[mouse_button])
            {
                held = true;
            }
            else
            {
                let mut release_in: bool =  hovered && (flags & ImGuiButtonFlags_PressedOnClickRelease) != 0;
                let mut release_anywhere: bool =  (flags & ImGuiButtonFlags_PressedOnClickReleaseAnywhere) != 0;
                if ((release_in || release_anywhere) && !g.DragDropActive)
                {
                    // Report as pressed when releasing the mouse (this is the most common path)
                    let mut is_double_click_release: bool =  (flags & ImGuiButtonFlags_PressedOnDoubleClick) && g.IO.MouseReleased[mouse_button] && g.IO.MouseClickedLastCount[mouse_button] == 2;
                    let mut is_repeating_already: bool =  (flags & ImGuiButtonFlags_Repeat) && g.IO.MouseDownDurationPrev[mouse_button] >= g.IO.KeyRepeatDelay; // Repeat mode trumps <on release>
                    if (!is_double_click_release && !is_repeating_already)
                        pressed = true;
                }
                ClearActiveID();
            }
            if (!(flags & ImGuiButtonFlags_NoNavFocus))
                g.NavDisableHighlight = true;
        }
        else if (g.ActiveIdSource == ImGuiInputSource_Nav)
        {
            // When activated using Nav, we hold on the ActiveID until activation button is released
            if (g.NavActivateDownId != id)
                ClearActiveID();
        }
        if (pressed)
            g.ActiveIdHasBeenPressedBefore = true;
    }

    if (out_hovered) *out_hovered = hovered;
    if (out_held) *out_held = held;

    return pressed;
}

bool ButtonEx(label: *const c_char, const size_arg: &ImVec2, ImGuiButtonFlags flags)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    const let mut style = &mut g.Style;
    let mut id: ImGuiID =  window.GetID(label);
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);

    let pos: ImVec2 = window.DC.CursorPos;
    if ((flags & ImGuiButtonFlags_AlignTextBaseLine) && style.FramePadding.y < window.DC.CurrLineTextBaseOffset) // Try to vertically align buttons that are smaller/have no padding so that text baseline matches (bit hacky, since it shouldn't be a flag)
        pos.y += window.DC.CurrLineTextBaseOffset - style.FramePadding.y;
    let size: ImVec2 = CalcItemSize(size_arg, label_size.x + style.FramePadding.x * 2.0f32, label_size.y + style.FramePadding.y * 2.00f32);

    let mut bb: ImRect = ImRect::new(pos, pos + size);
    ItemSize(size, style.FramePadding.y);
    if (!ItemAdd(bb, id))
        return false;

    if (g.LastItemData.InFlags & ImGuiItemFlags_ButtonRepeat)
        flags |= ImGuiButtonFlags_Repeat;

    hovered: bool, held;
    let mut pressed: bool =  ButtonBehavior(bb, id, &hovered, &held, flags);

    // Render
    let col: u32 = GetColorU32((held && hovered) ? ImGuiCol_ButtonActive : hovered ? ImGuiCol_ButtonHovered : ImGuiCol_Button);
    RenderNavHighlight(bb, id);
    RenderFrame(bb.Min, bb.Max, col, true, style.FrameRounding);

    if (g.LogEnabled)
        LogSetNextTextDecoration("[", "]");
    RenderTextClipped(bb.Min + style.FramePadding, bb.Max - style.FramePadding, label, null_mut(), &label_size, style.ButtonTextAlign, &bb);

    // Automatically close popups
    //if (pressed && !(flags & ImGuiButtonFlags_DontClosePopups) && (window.Flags & ImGuiWindowFlags_Popup))
    //    CloseCurrentPopup();

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags);
    return pressed;
}

bool Button(label: *const c_char, const size_arg: &ImVec2)
{
    return ButtonEx(label, size_arg, ImGuiButtonFlags_None);
}

// Small buttons fits within text without additional vertical spacing.
bool SmallButton(label: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let backup_padding_y: c_float =  g.Style.FramePadding.y;
    g.Style.FramePadding.y = 0f32;
    let mut pressed: bool =  ButtonEx(label, ImVec2::new2(0, 0), ImGuiButtonFlags_AlignTextBaseLine);
    g.Style.FramePadding.y = backup_padding_y;
    return pressed;
}

// Tip: use PushID()/PopID() to push indices or pointers in the ID stack.
// Then you can keep 'str_id' empty or the same for all your buttons (instead of creating a string based on a non-string id)
bool InvisibleButton(str_id: *const c_char, const size_arg: &ImVec2, ImGuiButtonFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // Cannot use zero-size for InvisibleButton(). Unlike Button() there is not way to fallback using the label size.
    // IM_ASSERT(size_arg.x != 0f32 && size_arg.y != 0f32);

    let mut id: ImGuiID =  window.GetID(str_id);
    let size: ImVec2 = CalcItemSize(size_arg, 0f32, 0f32);
    let mut bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + size);
    ItemSize(size);
    if (!ItemAdd(bb, id))
        return false;

    hovered: bool, held;
    let mut pressed: bool =  ButtonBehavior(bb, id, &hovered, &held, flags);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, str_id, g.LastItemData.StatusFlags);
    return pressed;
}

bool ArrowButtonEx(str_id: *const c_char, dir: ImGuiDir, size: ImVec2, ImGuiButtonFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let mut id: ImGuiID =  window.GetID(str_id);
    let mut bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + size);
    let default_size: c_float =  GetFrameHeight();
    ItemSize(size, (size.y >= default_size) ? g.Style.FramePadding.y : -1f32);
    if (!ItemAdd(bb, id))
        return false;

    if (g.LastItemData.InFlags & ImGuiItemFlags_ButtonRepeat)
        flags |= ImGuiButtonFlags_Repeat;

    hovered: bool, held;
    let mut pressed: bool =  ButtonBehavior(bb, id, &hovered, &held, flags);

    // Render
    let bg_col: u32 = GetColorU32((held && hovered) ? ImGuiCol_ButtonActive : hovered ? ImGuiCol_ButtonHovered : ImGuiCol_Button);
    let text_col: u32 = GetColorU32(ImGuiCol_Text);
    RenderNavHighlight(bb, id);
    RenderFrame(bb.Min, bb.Max, bg_col, true, g.Style.FrameRounding);
    RenderArrow(window.DrawList, bb.Min + ImVec2::new(ImMax(0f32, (size.x - g.FontSize) * 0.5f32), ImMax(0f32, (size.y - g.FontSize) * 0.5f32)), text_col, dir);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, str_id, g.LastItemData.StatusFlags);
    return pressed;
}

bool ArrowButton(str_id: *const c_char, dir: ImGuiDir)
{
    let sz: c_float =  GetFrameHeight();
    return ArrowButtonEx(str_id, dir, ImVec2::new(sz, sz), ImGuiButtonFlags_None);
}

// Button to close a window
bool CloseButton(id: ImGuiID, const pos: &ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = g.CurrentWindow;

    // Tweak 1: Shrink hit-testing area if button covers an abnormally large proportion of the visible region. That's in order to facilitate moving the window away. (#3825)
    // This may better be applied as a general hit-rect reduction mechanism for all widgets to ensure the area to move window is always accessible?
    let mut bb: ImRect = ImRect::new(pos, pos + ImVec2::new(g.FontSize, g.FontSize) + g.Style.FramePadding * 2.00f32);
    let bb_interact: ImRect =  bb;
    let area_to_visible_ratio: c_float =  window.OuterRectClipped.GetArea() / bb.GetArea();
    if (area_to_visible_ratio < 1.5f32)
        bb_interact.Expand(ImFloor(bb_interact.GetSize() * -0.250f32));

    // Tweak 2: We intentionally allow interaction when clipped so that a mechanical Alt,Right,Activate sequence can always close a window.
    // (this isn't the regular behavior of buttons, but it doesn't affect the user much because navigation tends to keep items visible).
    let mut is_clipped: bool =  !ItemAdd(bb_interact, id);

    hovered: bool, held;
    let mut pressed: bool =  ButtonBehavior(bb_interact, id, &hovered, &held);
    if (is_clipped)
        return pressed;

    // Render
    // FIXME: Clarify this mess
    let mut col: u32 = GetColorU32(held ? ImGuiCol_ButtonActive : ImGuiCol_ButtonHovered);
    let center: ImVec2 = bb.GetCenter();
    if (hovered)
        window.DrawList.AddCircleFilled(center, ImMax(2.0f32, g.FontSize * 0.5f32 + 1f32), col, 12);

    let cross_extent: c_float =  g.FontSize * 0.5f32 * 0.7071f - 1f32;
    let mut cross_col: u32 = GetColorU32(ImGuiCol_Text);
    center -= ImVec2::new2(0.5f32, 0.5f32);
    window.DrawList.AddLine(center + ImVec2::new(+cross_extent, +cross_extent), center + ImVec2::new(-cross_extent, -cross_extent), cross_col, 1f32);
    window.DrawList.AddLine(center + ImVec2::new(+cross_extent, -cross_extent), center + ImVec2::new(-cross_extent, +cross_extent), cross_col, 1f32);

    return pressed;
}

// The Collapse button also functions as a Dock Menu button.
bool CollapseButton(id: ImGuiID, const pos: &ImVec2, *mut ImGuiDockNode dock_node)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = g.CurrentWindow;

    let mut bb: ImRect = ImRect::new(pos, pos + ImVec2::new(g.FontSize, g.FontSize) + g.Style.FramePadding * 2.00f32);
    ItemAdd(bb, id);
    hovered: bool, held;
    let mut pressed: bool =  ButtonBehavior(bb, id, &hovered, &held, ImGuiButtonFlags_None);

    // Render
    //bool is_dock_menu = (window.DockNodeAsHost && !window.Collapsed);
    let mut bg_col: u32 = GetColorU32((held && hovered) ? ImGuiCol_ButtonActive : hovered ? ImGuiCol_ButtonHovered : ImGuiCol_Button);
    let mut text_col: u32 = GetColorU32(ImGuiCol_Text);
    if (hovered || held)
        window.DrawList.AddCircleFilled(bb.GetCenter() + ImVec2::new2(0,-0.5f32), g.FontSize * 0.5f32 + 1f32, bg_col, 12);

    if (dock_node)
        RenderArrowDockMenu(window.DrawList, bb.Min + g.Style.FramePadding, g.FontSize, text_col);
    else
        RenderArrow(window.DrawList, bb.Min + g.Style.FramePadding, text_col, window.Collapsed ? ImGuiDir_Right : ImGuiDir_Down, 1f32);

    // Switch to moving the window after mouse is moved beyond the initial drag threshold
    if (IsItemActive() && IsMouseDragging(0))
        StartMouseMovingWindowOrNode(window, dock_node, true);

    return pressed;
}

ImGuiID GetWindowScrollbarID(*mut ImGuiWindow window, axis: ImGuiAxis)
{
    return window.GetID(axis == ImGuiAxis_X ? "#SCROLLX" : "#SCROLLY");
}

// Return scrollbar rectangle, must only be called for corresponding axis if window.ScrollbarX/Y is set.
ImRect GetWindowScrollbarRect(*mut ImGuiWindow window, axis: ImGuiAxis)
{
    const let outer_rect: ImRect =  window.Rect();
    const let inner_rect: ImRect =  window.InnerRect;
    let border_size: c_float =  window.WindowBorderSize;
    let scrollbar_size: c_float =  window.ScrollbarSizes[axis ^ 1]; // (ScrollbarSizes.x = width of Y scrollbar; ScrollbarSizes.y = height of X scrollbar)
    // IM_ASSERT(scrollbar_size > 0f32);
    if (axis == ImGuiAxis_X)
        return ImRect::new(inner_rect.Min.x, ImMax(outer_rect.Min.y, outer_rect.Max.y - border_size - scrollbar_size), inner_rect.Max.x, outer_rect.Max.y);
    else
        return ImRect::new(ImMax(outer_rect.Min.x, outer_rect.Max.x - border_size - scrollbar_size), inner_rect.Min.y, outer_rect.Max.x, inner_rect.Max.y);
}

c_void Scrollbar(axis: ImGuiAxis)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = g.CurrentWindow;
    let mut id: ImGuiID =  GetWindowScrollbarID(window, axis);

    // Calculate scrollbar bounding box
    let bb: ImRect =  GetWindowScrollbarRect(window, axis);
    ImDrawFlags rounding_corners = ImDrawFlags_RoundCornersNone;
    if (axis == ImGuiAxis_X)
    {
        rounding_corners |= ImDrawFlags_RoundCornersBottomLeft;
        if (!window.ScrollbarY)
            rounding_corners |= ImDrawFlags_RoundCornersBottomRight;
    }
    else
    {
        if ((window.Flags & ImGuiWindowFlags_NoTitleBar) && !(window.Flags & ImGuiWindowFlags_MenuBar))
            rounding_corners |= ImDrawFlags_RoundCornersTopRight;
        if (!window.ScrollbarX)
            rounding_corners |= ImDrawFlags_RoundCornersBottomRight;
    }
    let size_avail: c_float =  window.InnerRect.Max[axis] - window.InnerRect.Min[axis];
    let size_contents: c_float =  window.ContentSize[axis] + window.WindowPadding[axis] * 2.0f32;
    ImS64 scroll = (ImS64)window.Scroll[axis];
    ScrollbarEx(bb, id, axis, &scroll, (ImS64)size_avail, (ImS64)size_contents, rounding_corners);
    window.Scroll[axis] = scroll;
}

// Vertical/Horizontal scrollbar
// The entire piece of code below is rather confusing because:
// - We handle absolute seeking (when first clicking outside the grab) and relative manipulation (afterward or when clicking inside the grab)
// - We store values as normalized ratio and in a form that allows the window content to change while we are holding on a scrollbar
// - We handle both horizontal and vertical scrollbars, which makes the terminology not ideal.
// Still, the code should probably be made simpler..
bool ScrollbarEx(bb_frame: &ImRect, id: ImGuiID, axis: ImGuiAxis, *mut ImS64 p_scroll_v, ImS64 size_avail_v, ImS64 size_contents_v, ImDrawFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = g.CurrentWindow;
    if (window.SkipItems)
        return false;

    KeepAliveID(id);

    let bb_frame_width: c_float =  bb_frame.GetWidth();
    let bb_frame_height: c_float =  bb_frame.GetHeight();
    if (bb_frame_width <= 0f32 || bb_frame_height <= 0f32)
        return false;

    // When we are too small, start hiding and disabling the grab (this reduce visual noise on very small window and facilitate using the window resize grab)
    let alpha: c_float =  1f32;
    if ((axis == ImGuiAxis_Y) && bb_frame_height < g.FontSize + g.Style.FramePadding.y * 2.00f32)
        alpha = ImSaturate((bb_frame_height - g.FontSize) / (g.Style.FramePadding.y * 2.00f32));
    if (alpha <= 0f32)
        return false;

    const let mut style = &mut g.Style;
    let allow_interaction: bool = (alpha >= 1f32);

    let bb: ImRect =  bb_frame;
    bb.Expand(ImVec2::new(-ImClamp(IM_FLOOR((bb_frame_width - 2.00f32) * 0.5f32), 0f32, 3.00f32), -ImClamp(IM_FLOOR((bb_frame_height - 2.00f32) * 0.5f32), 0f32, 3.00f32)));

    // V denote the main, longer axis of the scrollbar (= height for a vertical scrollbar)
    let scrollbar_size_v: c_float =  (axis == ImGuiAxis_X) ? bb.GetWidth() : bb.GetHeight();

    // Calculate the height of our grabbable box. It generally represent the amount visible (vs the total scrollable amount)
    // But we maintain a minimum size in pixel to allow for the user to still aim inside.
    // IM_ASSERT(ImMax(size_contents_v, size_avail_v) > 0f32); // Adding this assert to check if the ImMax(XXX,1f32) is still needed. PLEASE CONTACT ME if this triggers.
    const ImS64 win_size_v = ImMax(ImMax(size_contents_v, size_avail_v), (ImS64)1);
    let grab_h_pixels: c_float =  ImClamp(scrollbar_size_v * (size_avail_v / win_size_v), style.GrabMinSize, scrollbar_size_v);
    let grab_h_norm: c_float =  grab_h_pixels / scrollbar_size_v;

    // Handle input right away. None of the code of Begin() is relying on scrolling position before calling Scrollbar().
    let mut held: bool =  false;
    let mut hovered: bool =  false;
    ButtonBehavior(bb, id, &hovered, &held, ImGuiButtonFlags_NoNavFocus);

    const ImS64 scroll_max = ImMax((ImS64)1, size_contents_v - size_avail_v);
    let scroll_ratio: c_float =  ImSaturate(*p_scroll_v / scroll_max);
    let grab_v_norm: c_float =  scroll_ratio * (scrollbar_size_v - grab_h_pixels) / scrollbar_size_v; // Grab position in normalized space
    if (held && allow_interaction && grab_h_norm < 1f32)
    {
        let scrollbar_pos_v: c_float =  bb.Min[axis];
        let mouse_pos_v: c_float =  g.IO.MousePos[axis];

        // Click position in scrollbar normalized space (0f32.1f32)
        let clicked_v_norm: c_float =  ImSaturate((mouse_pos_v - scrollbar_pos_v) / scrollbar_size_v);
        SetHoveredID(id);

        let mut seek_absolute: bool =  false;
        if (g.ActiveIdIsJustActivated)
        {
            // On initial click calculate the distance between mouse and the center of the grab
            seek_absolute = (clicked_v_norm < grab_v_norm || clicked_v_norm > grab_v_norm + grab_h_norm);
            if (seek_absolute)
                g.ScrollbarClickDeltaToGrabCenter = 0f32;
            else
                g.ScrollbarClickDeltaToGrabCenter = clicked_v_norm - grab_v_norm - grab_h_norm * 0.5f32;
        }

        // Apply scroll (p_scroll_v will generally point on one member of window.Scroll)
        // It is ok to modify Scroll here because we are being called in Begin() after the calculation of ContentSize and before setting up our starting position
        let scroll_v_norm: c_float =  ImSaturate((clicked_v_norm - g.ScrollbarClickDeltaToGrabCenter - grab_h_norm * 0.5f32) / (1f32 - grab_h_norm));
        *p_scroll_v = (ImS64)(scroll_v_norm * scroll_max);

        // Update values for rendering
        scroll_ratio = ImSaturate(*p_scroll_v / scroll_max);
        grab_v_norm = scroll_ratio * (scrollbar_size_v - grab_h_pixels) / scrollbar_size_v;

        // Update distance to grab now that we have seeked and saturated
        if (seek_absolute)
            g.ScrollbarClickDeltaToGrabCenter = clicked_v_norm - grab_v_norm - grab_h_norm * 0.5f32;
    }

    // Render
    let bg_col: u32 = GetColorU32(ImGuiCol_ScrollbarBg);
    let grab_col: u32 = GetColorU32(held ? ImGuiCol_ScrollbarGrabActive : hovered ? ImGuiCol_ScrollbarGrabHovered : ImGuiCol_ScrollbarGrab, alpha);
    window.DrawList.AddRectFilled(bb_frame.Min, bb_frame.Max, bg_col, window.WindowRounding, flags);
    let mut grab_rect: ImRect = ImRect::default();
    if (axis == ImGuiAxis_X)
        grab_rect = ImRect::new(ImLerp(bb.Min.x, bb.Max.x, grab_v_norm), bb.Min.y, ImLerp(bb.Min.x, bb.Max.x, grab_v_norm) + grab_h_pixels, bb.Max.y);
    else
        grab_rect = ImRect::new(bb.Min.x, ImLerp(bb.Min.y, bb.Max.y, grab_v_norm), bb.Max.x, ImLerp(bb.Min.y, bb.Max.y, grab_v_norm) + grab_h_pixels);
    window.DrawList.AddRectFilled(grab_rect.Min, grab_rect.Max, grab_col, style.ScrollbarRounding);

    return held;
}

c_void Image(ImTextureID user_texture_id, const size: &ImVec2, const uv0: &ImVec2, const uv1: &ImVec2, const ImVec4& tint_col, const ImVec4& border_col)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    let mut bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + size);
    if (border_col.w > 0f32)
        bb.Max += ImVec2::new2(2, 2);
    ItemSize(bb);
    if (!ItemAdd(bb, 0))
        return;

    if (border_col.w > 0f32)
    {
        window.DrawList.AddRect(bb.Min, bb.Max, GetColorU32(border_col), 0f32);
        window.DrawList.AddImage(user_texture_id, bb.Min + ImVec2::new2(1, 1), bb.Max - ImVec2::new2(1, 1), uv0, uv1, GetColorU32(tint_col));
    }
    else
    {
        window.DrawList.AddImage(user_texture_id, bb.Min, bb.Max, uv0, uv1, GetColorU32(tint_col));
    }
}

// ImageButton() is flawed as 'id' is always derived from 'texture_id' (see #2464 #1390)
// We provide this internal helper to write your own variant while we figure out how to redesign the public ImageButton() API.
bool ImageButtonEx(id: ImGuiID, ImTextureID texture_id, const size: &ImVec2, const uv0: &ImVec2, const uv1: &ImVec2, const ImVec4& bg_col, const ImVec4& tint_col)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let padding: ImVec2 = g.Style.FramePadding;
    let mut bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + size + padding * 2.00f32);
    ItemSize(bb);
    if (!ItemAdd(bb, id))
        return false;

    hovered: bool, held;
    let mut pressed: bool =  ButtonBehavior(bb, id, &hovered, &held);

    // Render
    let col: u32 = GetColorU32((held && hovered) ? ImGuiCol_ButtonActive : hovered ? ImGuiCol_ButtonHovered : ImGuiCol_Button);
    RenderNavHighlight(bb, id);
    RenderFrame(bb.Min, bb.Max, col, true, ImClamp(ImMin(padding.x, padding.y), 0f32, g.Style.FrameRounding));
    if (bg_col.w > 0f32)
        window.DrawList.AddRectFilled(bb.Min + padding, bb.Max - padding, GetColorU32(bg_col));
    window.DrawList.AddImage(texture_id, bb.Min + padding, bb.Max - padding, uv0, uv1, GetColorU32(tint_col));

    return pressed;
}

bool ImageButton(str_id: *const c_char, ImTextureID user_texture_id, const size: &ImVec2, const uv0: &ImVec2, const uv1: &ImVec2, const ImVec4& bg_col, const ImVec4& tint_col)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = g.CurrentWindow;
    if (window.SkipItems)
        return false;

    return ImageButtonEx(window.GetID(str_id), user_texture_id, size, uv0, uv1, bg_col, tint_col);
}

// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
// Legacy API obsoleted in 1.89. Two differences with new ImageButton()
// - new ImageButton() requires an explicit 'const char* str_id'    Old ImageButton() used opaque imTextureId (created issue with: multiple buttons with same image, transient texture id values, opaque computation of ID)
// - new ImageButton() always use style.FramePadding                Old ImageButton() had an override argument.
// If you need to change padding with new ImageButton() you can use PushStyleVar(ImGuiStyleVar_FramePadding, value), consistent with other Button functions.
bool ImageButton(ImTextureID user_texture_id, const size: &ImVec2, const uv0: &ImVec2, const uv1: &ImVec2, frame_padding: c_int, const ImVec4& bg_col, const ImVec4& tint_col)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = g.CurrentWindow;
    if (window.SkipItems)
        return false;

    // Default to using texture ID as ID. User can still push string/integer prefixes.
    PushID(user_texture_id);
    let mut id: ImGuiID =  window.GetID("#image");
    PopID();

    if (frame_padding >= 0)
        PushStyleVar(ImGuiStyleVar_FramePadding, ImVec2::new(frame_padding, frame_padding));
    let mut ret: bool =  ImageButtonEx(id, user_texture_id, size, uv0, uv1, bg_col, tint_col);
    if (frame_padding >= 0)
        PopStyleVar();
    return ret;
}
// #endif // #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS

bool Checkbox(label: *const c_char, *mut v: bool)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    const let mut style = &mut g.Style;
    let mut id: ImGuiID =  window.GetID(label);
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);

    let square_sz: c_float =  GetFrameHeight();
    let pos: ImVec2 = window.DC.CursorPos;
    let mut total_bb: ImRect = ImRect::new(pos, pos + ImVec2::new(square_sz + (label_size.x > 0f32 ? style.ItemInnerSpacing.x + label_size.x : 0f32), label_size.y + style.FramePadding.y * 2.00f32));
    ItemSize(total_bb, style.FramePadding.y);
    if (!ItemAdd(total_bb, id))
    {
        IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags | ImGuiItemStatusFlags_Checkable | (*v ? ImGuiItemStatusFlags_Checked : 0));
        return false;
    }

    hovered: bool, held;
    let mut pressed: bool =  ButtonBehavior(total_bb, id, &hovered, &held);
    if (pressed)
    {
        *v = !(*v);
        MarkItemEdited(id);
    }

    let mut check_bb: ImRect = ImRect::new(pos, pos + ImVec2::new(square_sz, square_sz));
    RenderNavHighlight(total_bb, id);
    RenderFrame(check_bb.Min, check_bb.Max, GetColorU32((held && hovered) ? ImGuiCol_FrameBgActive : hovered ? ImGuiCol_FrameBgHovered : ImGuiCol_FrameBg), true, style.FrameRounding);
    let mut check_col: u32 = GetColorU32(ImGuiCol_CheckMark);
    let mut mixed_value: bool =  (g.LastItemData.InFlags & ImGuiItemFlags_MixedValue) != 0;
    if (mixed_value)
    {
        // Undocumented tristate/mixed/indeterminate checkbox (#2644)
        // This may seem awkwardly designed because the aim is to make ImGuiItemFlags_MixedValue supported by all widgets (not just checkbox)
        ImVec2 pad(ImMax(1f32, IM_FLOOR(square_sz / 3.60f32)), ImMax(1f32, IM_FLOOR(square_sz / 3.60f32)));
        window.DrawList.AddRectFilled(check_bb.Min + pad, check_bb.Max - pad, check_col, style.FrameRounding);
    }
    else if (*v)
    {
        let pad: c_float =  ImMax(1f32, IM_FLOOR(square_sz / 6.00f32));
        RenderCheckMark(window.DrawList, check_bb.Min + ImVec2::new(pad, pad), check_col, square_sz - pad * 2.00f32);
    }

    let label_pos: ImVec2 = ImVec2::new(check_bb.Max.x + style.ItemInnerSpacing.x, check_bb.Min.y + style.FramePadding.y);
    if (g.LogEnabled)
        LogRenderedText(&label_pos, mixed_value ? "[~]" : *v ? "[x]" : "[ ]");
    if (label_size.x > 0f32)
        RenderText(label_pos, label);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags | ImGuiItemStatusFlags_Checkable | (*v ? ImGuiItemStatusFlags_Checked : 0));
    return pressed;
}

template<typename T>
bool CheckboxFlagsT(label: *const c_char, *mut T flags, T flags_value)
{
    let mut all_on: bool =  (*flags & flags_value) == flags_value;
    let mut any_on: bool =  (*flags & flags_value) != 0;
    let mut pressed: bool;
    if (!all_on && any_on)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        let mut backup_item_flags: ImGuiItemFlags =  g.CurrentItemFlags;
        g.CurrentItemFlags |= ImGuiItemFlags_MixedValue;
        pressed = Checkbox(label, &all_on);
        g.CurrentItemFlags = backup_item_flags;
    }
    else
    {
        pressed = Checkbox(label, &all_on);

    }
    if (pressed)
    {
        if (all_on)
            *flags |= flags_value;
        else
            *flags &= !flags_value;
    }
    return pressed;
}

bool CheckboxFlags(label: *const c_char, *mut flags: c_int, flags_value: c_int)
{
    return CheckboxFlagsT(label, flags, flags_value);
}

bool CheckboxFlags(label: *const c_char, *mut c_uint flags, c_uint flags_value)
{
    return CheckboxFlagsT(label, flags, flags_value);
}

bool CheckboxFlags(label: *const c_char, *mut ImS64 flags, ImS64 flags_value)
{
    return CheckboxFlagsT(label, flags, flags_value);
}

bool CheckboxFlags(label: *const c_char, *mut u64 flags, u64 flags_value)
{
    return CheckboxFlagsT(label, flags, flags_value);
}

bool RadioButton(label: *const c_char, active: bool)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    const let mut style = &mut g.Style;
    let mut id: ImGuiID =  window.GetID(label);
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);

    let square_sz: c_float =  GetFrameHeight();
    let pos: ImVec2 = window.DC.CursorPos;
    let mut check_bb: ImRect = ImRect::new(pos, pos + ImVec2::new(square_sz, square_sz));
    let mut total_bb: ImRect = ImRect::new(pos, pos + ImVec2::new(square_sz + (label_size.x > 0f32 ? style.ItemInnerSpacing.x + label_size.x : 0f32), label_size.y + style.FramePadding.y * 2.00f32));
    ItemSize(total_bb, style.FramePadding.y);
    if (!ItemAdd(total_bb, id))
        return false;

    let center: ImVec2 = check_bb.GetCenter();
    center.x = IM_ROUND(center.x);
    center.y = IM_ROUND(center.y);
    let radius: c_float =  (square_sz - 1f32) * 0.5f32;

    hovered: bool, held;
    let mut pressed: bool =  ButtonBehavior(total_bb, id, &hovered, &held);
    if (pressed)
        MarkItemEdited(id);

    RenderNavHighlight(total_bb, id);
    window.DrawList.AddCircleFilled(center, radius, GetColorU32((held && hovered) ? ImGuiCol_FrameBgActive : hovered ? ImGuiCol_FrameBgHovered : ImGuiCol_FrameBg), 16);
    if (active)
    {
        let pad: c_float =  ImMax(1f32, IM_FLOOR(square_sz / 6.00f32));
        window.DrawList.AddCircleFilled(center, radius - pad, GetColorU32(ImGuiCol_CheckMark), 16);
    }

    if (style.FrameBorderSize > 0f32)
    {
        window.DrawList.AddCircle(center + ImVec2::new2(1, 1), radius, GetColorU32(ImGuiCol_BorderShadow), 16, style.FrameBorderSize);
        window.DrawList.AddCircle(center, radius, GetColorU32(ImGuiCol_Border), 16, style.FrameBorderSize);
    }

    let label_pos: ImVec2 = ImVec2::new(check_bb.Max.x + style.ItemInnerSpacing.x, check_bb.Min.y + style.FramePadding.y);
    if (g.LogEnabled)
        LogRenderedText(&label_pos, active ? "(x)" : "( )");
    if (label_size.x > 0f32)
        RenderText(label_pos, label);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags);
    return pressed;
}

// FIXME: This would work nicely if it was a public template, e.g. 'template<T> RadioButton(const char* label, T* v, T v_button)', but I'm not sure how we would expose it..
bool RadioButton(label: *const c_char, *mut v: c_int, v_button: c_int)
{
    let pressed: bool = RadioButton(label, *v == v_button);
    if (pressed)
        *v = v_button;
    return pressed;
}

// size_arg (for each axis) < 0f32: align to end, 0f32: auto, > 0f32: specified size
c_void ProgressBar(fraction: c_float, const size_arg: &ImVec2, overlay: *const c_char)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    const let mut style = &mut g.Style;

    let pos: ImVec2 = window.DC.CursorPos;
    let size: ImVec2 = CalcItemSize(size_arg, CalcItemWidth(), g.FontSize + style.FramePadding.y * 2.00f32);
    let mut bb: ImRect = ImRect::new(pos, pos + size);
    ItemSize(size, style.FramePadding.y);
    if (!ItemAdd(bb, 0))
        return;

    // Render
    fraction = ImSaturate(fraction);
    RenderFrame(bb.Min, bb.Max, GetColorU32(ImGuiCol_FrameBg), true, style.FrameRounding);
    bb.Expand(ImVec2::new(-style.FrameBorderSize, -style.FrameBorderSize));
    let fill_br: ImVec2 = ImVec2::new(ImLerp(bb.Min.x, bb.Max.x, fraction), bb.Max.y);
    RenderRectFilledRangeH(window.DrawList, bb, GetColorU32(ImGuiCol_PlotHistogram), 0f32, fraction, style.FrameRounding);

    // Default displaying the fraction as percentage string, but user can override it
    overlay_buf: [c_char;32];
    if (!overlay)
    {
        ImFormatString(overlay_buf, IM_ARRAYSIZE(overlay_buf), "%.0f%%", fraction * 100 + 0.010f32);
        overlay = overlay_buf;
    }

    let overlay_size: ImVec2 = CalcTextSize(overlay, null_mut());
    if (overlay_size.x > 0f32)
        RenderTextClipped(ImVec2::new(ImClamp(fill_br.x + style.ItemSpacing.x, bb.Min.x, bb.Max.x - overlay_size.x - style.ItemInnerSpacing.x), bb.Min.y), bb.Max, overlay, null_mut(), &overlay_size, ImVec2::new2(0f32, 0.5f32), &bb);
}

c_void Bullet()
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    const let mut style = &mut g.Style;
    let line_height: c_float =  ImMax(ImMin(window.DC.CurrLineSize.y, g.FontSize + style.FramePadding.y * 2), g.FontSize);
    let mut bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + ImVec2::new(g.FontSize, line_height));
    ItemSize(bb);
    if (!ItemAdd(bb, 0))
    {
        SameLine(0, style.FramePadding.x * 2);
        return;
    }

    // Render and stay on same line
    let mut text_col: u32 = GetColorU32(ImGuiCol_Text);
    RenderBullet(window.DrawList, bb.Min + ImVec2::new(style.FramePadding.x + g.FontSize * 0.5f32, line_height * 0.5f32), text_col);
    SameLine(0, style.FramePadding.x * 2.00f32);
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: Low-level Layout helpers
//-------------------------------------------------------------------------
// - Spacing()
// - Dummy()
// - NewLine()
// - AlignTextToFramePadding()
// - SeparatorEx() [Internal]
// - Separator()
// - SplitterBehavior() [Internal]
// - ShrinkWidths() [Internal]
//-------------------------------------------------------------------------

c_void Spacing()
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return;
    ItemSize(ImVec2::new2(0, 0));
}

c_void Dummy(const size: &ImVec2)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    let mut bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + size);
    ItemSize(size);
    ItemAdd(bb, 0);
}

c_void NewLine()
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    const ImGuiLayoutType backup_layout_type = window.DC.LayoutType;
    window.DC.LayoutType = ImGuiLayoutType_Vertical;
    window.DC.IsSameLine = false;
    if (window.DC.CurrLineSize.y > 0f32)     // In the event that we are on a line with items that is smaller that FontSize high, we will preserve its height.
        ItemSize(ImVec2::new2(0, 0));
    else
        ItemSize(ImVec2::new2(0f32, g.FontSize));
    window.DC.LayoutType = backup_layout_type;
}

c_void AlignTextToFramePadding()
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    window.DC.CurrLineSize.y = ImMax(window.DC.CurrLineSize.y, g.FontSize + g.Style.FramePadding.y * 2);
    window.DC.CurrLineTextBaseOffset = ImMax(window.DC.CurrLineTextBaseOffset, g.Style.FramePadding.y);
}

// Horizontal/vertical separating line
c_void SeparatorEx(ImGuiSeparatorFlags flags)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(ImIsPowerOfTwo(flags & (ImGuiSeparatorFlags_Horizontal | ImGuiSeparatorFlags_Vertical)));   // Check that only 1 option is selected

    let thickness_draw: c_float =  1f32;
    let thickness_layout: c_float =  0f32;
    if (flags & ImGuiSeparatorFlags_Vertical)
    {
        // Vertical separator, for menu bars (use current line height). Not exposed because it is misleading and it doesn't have an effect on regular layout.
        let y1: c_float =  window.DC.CursorPos.y;
        let y2: c_float =  window.DC.CursorPos.y + window.DC.CurrLineSize.y;
        let mut bb: ImRect = ImRect::new(ImVec2::new(window.DC.CursorPos.x, y1), ImVec2::new(window.DC.CursorPos.x + thickness_draw, y2));
        ItemSize(ImVec2::new(thickness_layout, 0f32));
        if (!ItemAdd(bb, 0))
            return;

        // Draw
        window.DrawList.AddLine(ImVec2::new(bb.Min.x, bb.Min.y), ImVec2::new(bb.Min.x, bb.Max.y), GetColorU32(ImGuiCol_Separator));
        if (g.LogEnabled)
            LogText(" |");
    }
    else if (flags & ImGuiSeparatorFlags_Horizontal)
    {
        // Horizontal Separator
        let x1: c_float =  window.Pos.x;
        let x2: c_float =  window.Pos.x + window.Size.x;

        // FIXME-WORKRECT: old hack (#205) until we decide of consistent behavior with WorkRect/Indent and Separator
        if (g.GroupStack.Size > 0 && g.GroupStack.last().unwrap().WindowID == window.ID)
            x1 += window.DC.Indent.x;

        // FIXME-WORKRECT: In theory we should simply be using WorkRect.Min.x/Max.x everywhere but it isn't aesthetically what we want,
        // need to introduce a variant of WorkRect for that purpose. (#4787)
        if (*mut ImGuiTable table = g.CurrentTable)
        {
            x1 = table.Columns[table.CurrentColumn].MinX;
            x2 = table.Columns[table.CurrentColumn].MaxX;
        }

        *mut ImGuiOldColumns columns = (flags & ImGuiSeparatorFlags_SpanAllColumns) ? window.DC.CurrentColumns : null_mut();
        if (columns)
            PushColumnsBackground();

        // We don't provide our width to the layout so that it doesn't get feed back into AutoFit
        // FIXME: This prevents .CursorMaxPos based bounding box evaluation from working (e.g. TableEndCell)
        let mut bb: ImRect = ImRect::new(ImVec2::new(x1, window.DC.CursorPos.y), ImVec2::new(x2, window.DC.CursorPos.y + thickness_draw));
        ItemSize(ImVec2::new2(0f32, thickness_layout));
        let item_visible: bool = ItemAdd(bb, 0);
        if (item_visible)
        {
            // Draw
            window.DrawList.AddLine(bb.Min, ImVec2::new(bb.Max.x, bb.Min.y), GetColorU32(ImGuiCol_Separator));
            if (g.LogEnabled)
                LogRenderedText(&bb.Min, "--------------------------------\n");

        }
        if (columns)
        {
            PopColumnsBackground();
            columns.LineMinY = window.DC.CursorPos.y;
        }
    }
}

c_void Separator()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = g.CurrentWindow;
    if (window.SkipItems)
        return;

    // Those flags should eventually be overridable by the user
    ImGuiSeparatorFlags flags = (window.DC.LayoutType == ImGuiLayoutType_Horizontal) ? ImGuiSeparatorFlags_Vertical : ImGuiSeparatorFlags_Horizontal;
    flags |= ImGuiSeparatorFlags_SpanAllColumns; // NB: this only applies to legacy Columns() api as they relied on Separator() a lot.
    SeparatorEx(flags);
}

// Using 'hover_visibility_delay' allows us to hide the highlight and mouse cursor for a short time, which can be convenient to reduce visual noise.
bool SplitterBehavior(bb: &ImRect, id: ImGuiID, axis: ImGuiAxis, *mut size1: c_float, *mut size2: c_float, min_size1: c_float, min_size2: c_float, hover_extend: c_float, hover_visibility_delay: c_float, u32 bg_col)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = g.CurrentWindow;

    let mut item_flags_backup: ImGuiItemFlags =  g.CurrentItemFlags;
    g.CurrentItemFlags |= ImGuiItemFlags_NoNav | ImGuiItemFlags_NoNavDefaultFocus;
    let mut item_add: bool =  ItemAdd(bb, id);
    g.CurrentItemFlags = item_flags_backup;
    if (!item_add)
        return false;

    hovered: bool, held;
    let bb_interact: ImRect =  bb;
    bb_interact.Expand(axis == ImGuiAxis_Y ? ImVec2::new2(0f32, hover_extend) : ImVec2::new(hover_extend, 0f32));
    ButtonBehavior(bb_interact, id, &hovered, &held, ImGuiButtonFlags_FlattenChildren | ImGuiButtonFlags_AllowItemOverlap);
    if (hovered)
        g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_HoveredRect; // for IsItemHovered(), because bb_interact is larger than bb
    if (g.ActiveId != id)
        SetItemAllowOverlap();

    if (held || (hovered && g.HoveredIdPreviousFrame == id && g.HoveredIdTimer >= hover_visibility_delay))
        SetMouseCursor(axis == ImGuiAxis_Y ? ImGuiMouseCursor_ResizeNS : ImGuiMouseCursor_ResizeEW);

    let bb_render: ImRect =  bb;
    if (held)
    {
        let mouse_delta_2d: ImVec2 = g.IO.MousePos - g.ActiveIdClickOffset - bb_interact.Min;
        let mouse_delta: c_float =  (axis == ImGuiAxis_Y) ? mouse_delta_2d.y : mouse_delta_2d.x;

        // Minimum pane size
        let size_1_maximum_delta: c_float =  ImMax(0f32, *size1 - min_size1);
        let size_2_maximum_delta: c_float =  ImMax(0f32, *size2 - min_size2);
        if (mouse_delta < -size_1_maximum_delta)
            mouse_delta = -size_1_maximum_delta;
        if (mouse_delta > size_2_maximum_delta)
            mouse_delta = size_2_maximum_delta;

        // Apply resize
        if (mouse_delta != 0f32)
        {
            if (mouse_delta < 0f32)
                // IM_ASSERT(*size1 + mouse_delta >= min_size1);
            if (mouse_delta > 0f32)
                // IM_ASSERT(*size2 - mouse_delta >= min_size2);
            *size1 += mouse_delta;
            *size2 -= mouse_delta;
            bb_render.Translate((axis == ImGuiAxis_X) ? ImVec2::new(mouse_delta, 0f32) : ImVec2::new2(0f32, mouse_delta));
            MarkItemEdited(id);
        }
    }

    // Render at new position
    if (bg_col & IM_COL32_A_MASK)
        window.DrawList.AddRectFilled(bb_render.Min, bb_render.Max, bg_col, 0f32);
    let col: u32 = GetColorU32(held ? ImGuiCol_SeparatorActive : (hovered && g.HoveredIdTimer >= hover_visibility_delay) ? ImGuiCol_SeparatorHovered : ImGuiCol_Separator);
    window.DrawList.AddRectFilled(bb_render.Min, bb_render.Max, col, 0f32);

    return held;
}

static c_int IMGUI_CDECL ShrinkWidthItemComparer(*const c_void lhs, *const c_void rhs)
{
    let a: *const ImGuiShrinkWidthItem = (*const ImGuiShrinkWidthItem)lhs;
    let b: *const ImGuiShrinkWidthItem = (*const ImGuiShrinkWidthItem)rhs;
    if (let d: c_int = (b.Width - a.Width))
        return d;
    return (b.Index - a.Index);
}

// Shrink excess width from a set of item, by removing width from the larger items first.
// Set items Width to -1f32 to disable shrinking this item.
c_void ShrinkWidths(*mut ImGuiShrinkWidthItem items, count: c_int, width_excess: c_float)
{
    if (count == 1)
    {
        if (items[0].Width >= 0f32)
            items[0].Width = ImMax(items[0].Width - width_excess, 1f32);
        return;
    }
    ImQsort(items, count, sizeof(ImGuiShrinkWidthItem), ShrinkWidthItemComparer);
    let count_same_width: c_int = 1;
    while (width_excess > 0f32 && count_same_width < count)
    {
        while (count_same_width < count && items[0].Width <= items[count_same_width].Width)
            count_same_width+= 1;
        let max_width_to_remove_per_item: c_float =  (count_same_width < count && items[count_same_width].Width >= 0f32) ? (items[0].Width - items[count_same_width].Width) : (items[0].Width - 1f32);
        if (max_width_to_remove_per_item <= 0f32)
            break;
        let width_to_remove_per_item: c_float =  ImMin(width_excess / count_same_width, max_width_to_remove_per_item);
        for (let item_n: c_int = 0; item_n < count_same_width; item_n++)
            items[item_n].Width -= width_to_remove_per_item;
        width_excess -= width_to_remove_per_item * count_same_width;
    }

    // Round width and redistribute remainder
    // Ensure that e.g. the right-most tab of a shrunk tab-bar always reaches exactly at the same distance from the right-most edge of the tab bar separator.
    width_excess = 0f32;
    for (let n: c_int = 0; n < count; n++)
    {
        let width_rounded: c_float =  ImFloor(items[n].Width);
        width_excess += items[n].Width - width_rounded;
        items[n].Width = width_rounded;
    }
    while (width_excess > 0f32)
        for (let n: c_int = 0; n < count && width_excess > 0f32; n++)
        {
            let width_to_add: c_float =  ImMin(items[n].InitialWidth - items[n].Width, 1f32);
            items[n].Width += width_to_add;
            width_excess -= width_to_add;
        }
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: ComboBox
//-------------------------------------------------------------------------
// - CalcMaxPopupHeightFromItemCount() [Internal]
// - BeginCombo()
// - BeginComboPopup() [Internal]
// - EndCombo()
// - BeginComboPreview() [Internal]
// - EndComboPreview() [Internal]
// - Combo()
//-------------------------------------------------------------------------

static c_float CalcMaxPopupHeightFromItemCount(items_count: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (items_count <= 0)
        return f32::MAX;
    return (g.FontSize + g.Style.ItemSpacing.y) * items_count - g.Style.ItemSpacing.y + (g.Style.WindowPadding.y * 2);
}

bool BeginCombo(label: *const c_char, preview_value: *const c_char, ImGuiComboFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = GetCurrentWindow();

    ImGuiNextWindowDataFlags backup_next_window_data_flags = g.NextWindowData.Flags;
    g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
    if (window.SkipItems)
        return false;

    const let mut style = &mut g.Style;
    let mut id: ImGuiID =  window.GetID(label);
    // IM_ASSERT((flags & (ImGuiComboFlags_NoArrowButton | ImGuiComboFlags_NoPreview)) != (ImGuiComboFlags_NoArrowButton | ImGuiComboFlags_NoPreview)); // Can't use both flags together

    let arrow_size: c_float =  (flags & ImGuiComboFlags_NoArrowButton) ? 0f32 : GetFrameHeight();
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);
    let w: c_float =  (flags & ImGuiComboFlags_NoPreview) ? arrow_size : CalcItemWidth();
    let mut bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + ImVec2::new(w, label_size.y + style.FramePadding.y * 2.00f32));
    let mut total_bb: ImRect = ImRect::new(bb.Min, bb.Max + ImVec2::new(label_size.x > 0f32 ? style.ItemInnerSpacing.x + label_size.x : 0f32, 0f32));
    ItemSize(total_bb, style.FramePadding.y);
    if (!ItemAdd(total_bb, id, &bb))
        return false;

    // Open on click
    hovered: bool, held;
    let mut pressed: bool =  ButtonBehavior(bb, id, &hovered, &held);
    let mut popup_id: ImGuiID =  ImHashStr("##ComboPopup", 0, id);
    let mut popup_open: bool =  IsPopupOpen(popup_id, ImGuiPopupFlags_None);
    if (pressed && !popup_open)
    {
        OpenPopupEx(popup_id, ImGuiPopupFlags_None);
        popup_open = true;
    }

    // Render shape
    let frame_col: u32 = GetColorU32(hovered ? ImGuiCol_FrameBgHovered : ImGuiCol_FrameBg);
    let value_x2: c_float =  ImMax(bb.Min.x, bb.Max.x - arrow_size);
    RenderNavHighlight(bb, id);
    if (!(flags & ImGuiComboFlags_NoPreview))
        window.DrawList.AddRectFilled(bb.Min, ImVec2::new(value_x2, bb.Max.y), frame_col, style.FrameRounding, (flags & ImGuiComboFlags_NoArrowButton) ? ImDrawFlags_RoundCornersAll : ImDrawFlags_RoundCornersLeft);
    if (!(flags & ImGuiComboFlags_NoArrowButton))
    {
        let mut bg_col: u32 = GetColorU32((popup_open || hovered) ? ImGuiCol_ButtonHovered : ImGuiCol_Button);
        let mut text_col: u32 = GetColorU32(ImGuiCol_Text);
        window.DrawList.AddRectFilled(ImVec2::new(value_x2, bb.Min.y), bb.Max, bg_col, style.FrameRounding, (w <= arrow_size) ? ImDrawFlags_RoundCornersAll : ImDrawFlags_RoundCornersRight);
        if (value_x2 + arrow_size - style.FramePadding.x <= bb.Max.x)
            RenderArrow(window.DrawList, ImVec2::new(value_x2 + style.FramePadding.y, bb.Min.y + style.FramePadding.y), text_col, ImGuiDir_Down, 1f32);
    }
    RenderFrameBorder(bb.Min, bb.Max, style.FrameRounding);

    // Custom preview
    if (flags & ImGuiComboFlags_CustomPreview)
    {
        g.ComboPreviewData.PreviewRect = ImRect::new(bb.Min.x, bb.Min.y, value_x2, bb.Max.y);
        // IM_ASSERT(preview_value == NULL || preview_value[0] == 0);
        preview_value= null_mut();
    }

    // Render preview and label
    if (preview_value != null_mut() && !(flags & ImGuiComboFlags_NoPreview))
    {
        if (g.LogEnabled)
            LogSetNextTextDecoration("{", "}");
        RenderTextClipped(bb.Min + style.FramePadding, ImVec2::new(value_x2, bb.Max.y), preview_value, null_mut(), null_mut());
    }
    if (label_size.x > 0)
        RenderText(ImVec2::new(bb.Max.x + style.ItemInnerSpacing.x, bb.Min.y + style.FramePadding.y), label);

    if (!popup_open)
        return false;

    g.NextWindowData.Flags = backup_next_window_data_flags;
    return BeginComboPopup(popup_id, bb, flags);
}

bool BeginComboPopup(popup_id: ImGuiID, bb: &ImRect, ImGuiComboFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!IsPopupOpen(popup_id, ImGuiPopupFlags_None))
    {
        g.NextWindowData.ClearFlags();
        return false;
    }

    // Set popup size
    let w: c_float =  bb.GetWidth();
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSizeConstraint)
    {
        g.NextWindowData.SizeConstraintRect.Min.x = ImMax(g.NextWindowData.SizeConstraintRect.Min.x, w);
    }
    else
    {
        if ((flags & ImGuiComboFlags_HeightMask_) == 0)
            flags |= ImGuiComboFlags_HeightRegular;
        // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiComboFlags_HeightMask_)); // Only one
        let popup_max_height_in_items: c_int = -1;
        if (flags & ImGuiComboFlags_HeightRegular)     popup_max_height_in_items = 8;
        else if (flags & ImGuiComboFlags_HeightSmall)  popup_max_height_in_items = 4;
        else if (flags & ImGuiComboFlags_HeightLarge)  popup_max_height_in_items = 20;
        SetNextWindowSizeConstraints(ImVec2::new(w, 0f32), ImVec2::new(f32::MAX, CalcMaxPopupHeightFromItemCount(popup_max_height_in_items)));
    }

    // This is essentially a specialized version of BeginPopupEx()
    name: [c_char;16];
    ImFormatString(name, IM_ARRAYSIZE(name), "##Combo_%02d", g.BeginPopupStack.Size); // Recycle windows based on depth

    // Set position given a custom constraint (peak into expected window size so we can position it)
    // FIXME: This might be easier to express with an hypothetical SetNextWindowPosConstraints() function?
    // FIXME: This might be moved to Begin() or at least around the same spot where Tooltips and other Popups are calling FindBestWindowPosForPopupEx()?
    if (*mut ImGuiWindow popup_window = FindWindowByName(name))
        if (popup_window.WasActive)
        {
            // Always override 'AutoPosLastDirection' to not leave a chance for a past value to affect us.
            let size_expected: ImVec2 = CalcWindowNextAutoFitSize(popup_window);
            popup_window.AutoPosLastDirection = (flags & ImGuiComboFlags_PopupAlignLeft) ? ImGuiDir_Left : ImGuiDir_Down; // Left = "Below, Toward Left", Down = "Below, Toward Right (default)"
            let r_outer: ImRect =  GetPopupAllowedExtentRect(popup_window);
            let pos: ImVec2 = FindBestWindowPosForPopupEx(bb.GetBL(), size_expected, &popup_window.AutoPosLastDirection, r_outer, bb, ImGuiPopupPositionPolicy_ComboBox);
            SetNextWindowPos(pos);
        }

    // We don't use BeginPopupEx() solely because we have a custom name string, which we could make an argument to BeginPopupEx()
    let mut window_flags: ImGuiWindowFlags = ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_Popup | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_NoMove;
    PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2::new(g.Style.FramePadding.x, g.Style.WindowPadding.y)); // Horizontally align ourselves with the framed text
    let mut ret: bool =  Begin(name, null_mut(), window_flags);
    PopStyleVar();
    if (!ret)
    {
        EndPopup();
        // IM_ASSERT(0);   // This should never happen as we tested for IsPopupOpen() above
        return false;
    }
    return true;
}

c_void EndCombo()
{
    EndPopup();
}

// Call directly after the BeginCombo/EndCombo block. The preview is designed to only host non-interactive elements
// (Experimental, see GitHub issues: #1658, #4168)
bool BeginComboPreview()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = g.CurrentWindow;
    *mut ImGuiComboPreviewData preview_data = &g.ComboPreviewData;

    if (window.SkipItems || !window.ClipRect.Overlaps(g.LastItemData.Rect)) // FIXME: Because we don't have a ImGuiItemStatusFlags_Visible flag to test last ItemAdd() result
        return false;
    // IM_ASSERT(g.LastItemData.Rect.Min.x == preview_Data.PreviewRect.Min.x && g.LastItemData.Rect.Min.y == preview_Data.PreviewRect.Min.y); // Didn't call after BeginCombo/EndCombo block or forgot to pass ImGuiComboFlags_CustomPreview flag?
    if (!window.ClipRect.Contains(preview_data.PreviewRect)) // Narrower test (optional)
        return false;

    // FIXME: This could be contained in a PushWorkRect() api
    preview_data.BackupCursorPos = window.DC.CursorPos;
    preview_data.BackupCursorMaxPos = window.DC.CursorMaxPos;
    preview_data.BackupCursorPosPrevLine = window.DC.CursorPosPrevLine;
    preview_data.BackupPrevLineTextBaseOffset = window.DC.PrevLineTextBaseOffset;
    preview_data.BackupLayout = window.DC.LayoutType;
    window.DC.CursorPos = preview_data.PreviewRect.Min + g.Style.FramePadding;
    window.DC.CursorMaxPos = window.DC.CursorPos;
    window.DC.LayoutType = ImGuiLayoutType_Horizontal;
    window.DC.IsSameLine = false;
    PushClipRect(preview_Data.PreviewRect.Min, preview_Data.PreviewRect.Max, true);

    return true;
}

c_void EndComboPreview()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = g.CurrentWindow;
    *mut ImGuiComboPreviewData preview_data = &g.ComboPreviewData;

    // FIXME: Using CursorMaxPos approximation instead of correct AABB which we will store in ImDrawCmd in the future
    *mut ImDrawList draw_list = window.DrawList;
    if (window.DC.CursorMaxPos.x < preview_Data.PreviewRect.Max.x && window.DC.CursorMaxPos.y < preview_Data.PreviewRect.Max.y)
        if (draw_list.CmdBuffer.Size > 1) // Unlikely case that the PushClipRect() didn't create a command
        {
            draw_list._CmdHeader.ClipRect = draw_list.CmdBuffer[draw_list.CmdBuffer.Size - 1].ClipRect = draw_list.CmdBuffer[draw_list.CmdBuffer.Size - 2].ClipRect;
            draw_list._TryMergeDrawCmds();
        }
    PopClipRect();
    window.DC.CursorPos = preview_Data.BackupCursorPos;
    window.DC.CursorMaxPos = ImMax(window.DC.CursorMaxPos, preview_Data.BackupCursorMaxPos);
    window.DC.CursorPosPrevLine = preview_Data.BackupCursorPosPrevLine;
    window.DC.PrevLineTextBaseOffset = preview_Data.BackupPrevLineTextBaseOffset;
    window.DC.LayoutType = preview_Data.BackupLayout;
    window.DC.IsSameLine = false;
    preview_Data.PreviewRect = ImRect::new();
}

// Getter for the old Combo() API: const char*[]
static bool Items_ArrayGetter(*mut c_void data, idx: c_int, *const *mut char out_text)
{
    *const char *mut const items = (*const char *mut const)data;
    if (out_text)
        *out_text = items[idx];
    return true;
}

// Getter for the old Combo() API: "item1\0item2\0item3\0"
static bool Items_SingleStringGetter(*mut c_void data, idx: c_int, *const *mut char out_text)
{
    // FIXME-OPT: we could pre-compute the indices to fasten this. But only 1 active combo means the waste is limited.
    let mut  items_separated_by_zeros: *const c_char =data;
    let items_count: c_int = 0;
    let mut  p: *const c_char = items_separated_by_zeros;
    while (*p)
    {
        if (idx == items_count)
            break;
        p += strlen(p) + 1;
        items_count+= 1;
    }
    if (!*p)
        return false;
    if (out_text)
        *out_text = p;
    return true;
}

// Old API, prefer using BeginCombo() nowadays if you can.
bool Combo(label: *const c_char, *mut current_item: c_int, bool (*items_getter)(*mut c_void, c_int, *const *mut char), *mut c_void data, items_count: c_int, popup_max_height_in_items: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Call the getter to obtain the preview string which is a parameter to BeginCombo()
    let mut  preview_value: *const c_char= null_mut();
    if (*current_item >= 0 && *current_item < items_count)
        items_getter(data, *current_item, &preview_value);

    // The old Combo() API exposed "popup_max_height_in_items". The new more general BeginCombo() API doesn't have/need it, but we emulate it here.
    if (popup_max_height_in_items != -1 && !(g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSizeConstraint))
        SetNextWindowSizeConstraints(ImVec2::new2(0, 0), ImVec2::new(f32::MAX, CalcMaxPopupHeightFromItemCount(popup_max_height_in_items)));

    if (!BeginCombo(label, preview_value, ImGuiComboFlags_None))
        return false;

    // Display items
    // FIXME-OPT: Use clipper (but we need to disable it on the appearing frame to make sure our call to SetItemDefaultFocus() is processed)
    let mut value_changed: bool =  false;
    for (let i: c_int = 0; i < items_count; i++)
    {
        PushID(i);
        let item_selected: bool = (i == *current_item);
let item_text: *const c_char;
        if (!items_getter(data, i, &item_text))
            item_text = "*Unknown item*";
        if (Selectable(item_text, item_selected))
        {
            value_changed = true;
            *current_item = i;
        }
        if (item_selected)
            SetItemDefaultFocus();
        PopID();
    }

    EndCombo();

    if (value_changed)
        MarkItemEdited(g.LastItemData.ID);

    return value_changed;
}

// Combo box helper allowing to pass an array of strings.
bool Combo(label: *const c_char, *mut current_item: c_int, *const char const items[], items_count: c_int, height_in_items: c_int)
{
    let value_changed: bool = Combo(label, current_item, Items_ArrayGetter, items, items_count, height_in_items);
    return value_changed;
}

// Combo box helper allowing to pass all items in a single string literal holding multiple zero-terminated items "item1\0item2\0"
bool Combo(label: *const c_char, *mut current_item: c_int, items_separated_by_zeros: *const c_char, height_in_items: c_int)
{
    let items_count: c_int = 0;
    let mut  p: *const c_char = items_separated_by_zeros;       // FIXME-OPT: Avoid computing this, or at least only when combo is open
    while (*p)
    {
        p += strlen(p) + 1;
        items_count+= 1;
    }
    let mut value_changed: bool =  Combo(label, current_item, Items_SingleStringGetter, items_separated_by_zeros, items_count, height_in_items);
    return value_changed;
}

//-------------------------------------------------------------------------
// [SECTION] Data Type and Data Formatting Helpers [Internal]
//-------------------------------------------------------------------------
// - PatchFormatStringFloatToInt()
// - DataTypeGetInfo()
// - DataTypeFormatString()
// - DataTypeApplyOp()
// - DataTypeApplyOpFromText()
// - DataTypeCompare()
// - DataTypeClamp()
// - GetMinimumStepAtDecimalPrecision
// - RoundScalarWithFormat<>()
//-------------------------------------------------------------------------

static const ImGuiDataTypeInfo GDataTypeInfo[] =
{
    { sizeof,             "S8",   "%d",   "%d"    },  // ImGuiDataType_S8
    { sizeof(c_uchar),    "U8",   "%u",   "%u"    },
    { sizeof,            "S16",  "%d",   "%d"    },  // ImGuiDataType_S16
    { sizeof(unsigned c_short),   "U16",  "%u",   "%u"    },
    { sizeof,              "S32",  "%d",   "%d"    },  // ImGuiDataType_S32
    { sizeof,     "U32",  "%u",   "%u"    },
// #ifdef _MSC_VER
    { sizeof(ImS64),            "S64",  "%I64d","%I64d" },  // ImGuiDataType_S64
    { sizeof,            "U64",  "%I64u","%I64u" },
// #else
    { sizeof(ImS64),            "S64",  "%lld", "%lld"  },  // ImGuiDataType_S64
    { sizeof,            "U64",  "%llu", "%llu"  },
// #endif
    { sizeof,            "float", "%.3f","%f"    },  // ImGuiDataType_Float (float are promoted to double in va_arg)
    { sizeof,           "double","%f",  "%lf"   },  // ImGuiDataType_Double
};
IM_STATIC_ASSERT(IM_ARRAYSIZE(GDataTypeInfo) == ImGuiDataType_COUNT);

// FIXME-LEGACY: Prior to 1.61 our DragInt() function internally used floats and because of this the compile-time default value for format was "%.0f".
// Even though we changed the compile-time default, we expect users to have carried %f around, which would break the display of DragInt() calls.
// To honor backward compatibility we are rewriting the format string, unless IMGUI_DISABLE_OBSOLETE_FUNCTIONS is enabled. What could possibly go wrong?!
static *const char PatchFormatStringFloatToInt(fmt: *const c_char)
{
    if (fmt[0] == '%' && fmt[1] == '.' && fmt[2] == '0' && fmt[3] == 'f' && fmt[4] == 0) // Fast legacy path for "%.0f" which is expected to be the most common case.
        return "%d";
    let mut  fmt_start: *const c_char = ImParseFormatFindStart(fmt);    // Find % (if any, and ignore %%)
    let mut  fmt_end: *const c_char = ImParseFormatFindEnd(fmt_start);  // Find end of format specifier, which itself is an exercise of confidence/recklessness (because snprintf is dependent on libc or user).
    if (fmt_end > fmt_start && fmt_end[-1] == 'f')
    {
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
        if (fmt_start == fmt && fmt_end[0] == 0)
            return "%d";
let tmp_format: *const c_char;
        ImFormatStringToTempBuffer(&tmp_format, null_mut(), "%.*s%%d%s", (fmt_start - fmt), fmt, fmt_end); // Honor leading and trailing decorations, but lose alignment/precision.
        return tmp_format;
// #else
        // IM_ASSERT(0 && "DragInt(): Invalid format string!"); // Old versions used a default parameter of "%.0f", please replace with e.g. "%d"
// #endif
    }
    return fmt;
}

*const ImGuiDataTypeInfo DataTypeGetInfo(ImGuiDataType data_type)
{
    // IM_ASSERT(data_type >= 0 && data_type < ImGuiDataType_COUNT);
    return &GDataTypeInfo[data_type];
}

c_int DataTypeFormatString(*mut char buf, buf_size: c_int, ImGuiDataType data_type, *const c_void p_data, format: *const c_char)
{
    // Signedness doesn't matter when pushing integer arguments
    if (data_type == ImGuiDataType_S32 || data_type == ImGuiDataType_U32)
        return ImFormatString(buf, buf_size, format, *(*const u32)p_data);
    if (data_type == ImGuiDataType_S64 || data_type == ImGuiDataType_U64)
        return ImFormatString(buf, buf_size, format, *(*const u64)p_data);
    if (data_type == ImGuiDataType_Float)
        return ImFormatString(buf, buf_size, format, *(*const c_float)p_data);
    if (data_type == ImGuiDataType_Double)
        return ImFormatString(buf, buf_size, format, *(*const double)p_data);
    if (data_type == ImGuiDataType_S8)
        return ImFormatString(buf, buf_size, format, *(*const i8)p_data);
    if (data_type == ImGuiDataType_U8)
        return ImFormatString(buf, buf_size, format, *(*const u8)p_data);
    if (data_type == ImGuiDataType_S16)
        return ImFormatString(buf, buf_size, format, *(*const i16)p_data);
    if (data_type == ImGuiDataType_U16)
        return ImFormatString(buf, buf_size, format, *(*const u16)p_data);
    // IM_ASSERT(0);
    return 0;
}

c_void DataTypeApplyOp(ImGuiDataType data_type, op: c_int, *mut c_void output, *const c_void arg1, *const c_void arg2)
{
    // IM_ASSERT(op == '+' || op == '-');
    switch (data_type)
    {
        case ImGuiDataType_S8:
            if (op == '+') { *(*mut i8)output  = ImAddClampOverflow(*(*const i8)arg1,  *(*const i8)arg2,  IM_S8_MIN,  IM_S8_MAX); }
            if (op == '-') { *(*mut i8)output  = ImSubClampOverflow(*(*const i8)arg1,  *(*const i8)arg2,  IM_S8_MIN,  IM_S8_MAX); }
            return;
        case ImGuiDataType_U8:
            if (op == '+') { *(*mut u8)output  = ImAddClampOverflow(*(*const u8)arg1,  *(*const u8)arg2,  IM_U8_MIN,  IM_U8_MAX); }
            if (op == '-') { *(*mut u8)output  = ImSubClampOverflow(*(*const u8)arg1,  *(*const u8)arg2,  IM_U8_MIN,  IM_U8_MAX); }
            return;
        case ImGuiDataType_S16:
            if (op == '+') { *(*mut i16)output = ImAddClampOverflow(*(*const i16)arg1, *(*const i16)arg2, IM_S16_MIN, IM_S16_MAX); }
            if (op == '-') { *(*mut i16)output = ImSubClampOverflow(*(*const i16)arg1, *(*const i16)arg2, IM_S16_MIN, IM_S16_MAX); }
            return;
        case ImGuiDataType_U16:
            if (op == '+') { *(*mut u16)output = ImAddClampOverflow(*(*const u16)arg1, *(*const u16)arg2, IM_U16_MIN, IM_U16_MAX); }
            if (op == '-') { *(*mut u16)output = ImSubClampOverflow(*(*const u16)arg1, *(*const u16)arg2, IM_U16_MIN, IM_U16_MAX); }
            return;
        case ImGuiDataType_S32:
            if (op == '+') { *(*mut i32)output = ImAddClampOverflow(*(*const i32)arg1, *(*const i32)arg2, IM_S32_MIN, IM_S32_MAX); }
            if (op == '-') { *(*mut i32)output = ImSubClampOverflow(*(*const i32)arg1, *(*const i32)arg2, IM_S32_MIN, IM_S32_MAX); }
            return;
        case ImGuiDataType_U32:
            if (op == '+') { *(*mut u32)output = ImAddClampOverflow(*(*const u32)arg1, *(*const u32)arg2, IM_U32_MIN, IM_U32_MAX); }
            if (op == '-') { *(*mut u32)output = ImSubClampOverflow(*(*const u32)arg1, *(*const u32)arg2, IM_U32_MIN, IM_U32_MAX); }
            return;
        case ImGuiDataType_S64:
            if (op == '+') { *(*mut ImS64)output = ImAddClampOverflow(*(*const ImS64)arg1, *(*const ImS64)arg2, IM_S64_MIN, IM_S64_MAX); }
            if (op == '-') { *(*mut ImS64)output = ImSubClampOverflow(*(*const ImS64)arg1, *(*const ImS64)arg2, IM_S64_MIN, IM_S64_MAX); }
            return;
        case ImGuiDataType_U64:
            if (op == '+') { *(*mut u64)output = ImAddClampOverflow(*(*const u64)arg1, *(*const u64)arg2, IM_U64_MIN, IM_U64_MAX); }
            if (op == '-') { *(*mut u64)output = ImSubClampOverflow(*(*const u64)arg1, *(*const u64)arg2, IM_U64_MIN, IM_U64_MAX); }
            return;
        case ImGuiDataType_Float:
            if (op == '+') { *(*mut c_float)output = *(*const c_float)arg1 + *(*const c_float)arg2; }
            if (op == '-') { *(*mut c_float)output = *(*const c_float)arg1 - *(*const c_float)arg2; }
            return;
        case ImGuiDataType_Double:
            if (op == '+') { *(*mut double)output = *(*const double)arg1 + *(*const double)arg2; }
            if (op == '-') { *(*mut double)output = *(*const double)arg1 - *(*const double)arg2; }
            return;
        case ImGuiDataType_COUNT: break;
    }
    // IM_ASSERT(0);
}

// User can input math operators (e.g. +100) to edit a numerical values.
// NB: This is _not_ a full expression evaluator. We should probably add one and replace this dumb mess..
bool DataTypeApplyFromText(buf: *const c_char, ImGuiDataType data_type, *mut c_void p_data, format: *const c_char)
{
    while (ImCharIsBlankA(*buf))
        buf+= 1;
    if (!buf[0])
        return false;

    // Copy the value in an opaque buffer so we can compare at the end of the function if it changed at all.
    let type_info: *const ImGuiDataTypeInfo = DataTypeGetInfo(data_type);
    ImGuiDataTypeTempStorage data_backup;
    memcpy(&data_backup, p_data, );

    // Sanitize format
    // For float/double we have to ignore format with precision (e.g. "%.2f") because sscanf doesn't take them in, so force them into %f and %lf
    format_sanitized: [c_char;32];
    if (data_type == ImGuiDataType_Float || data_type == ImGuiDataType_Double)
        format = ;
    else
        format = ImParseFormatSanitizeForScanning(format, format_sanitized, IM_ARRAYSIZE(format_sanitized));

    // Small types need a 32-bit buffer to receive the result from scanf()
    let v32: c_int = 0;
    if (sscanf(buf, format,  >= 4 ? p_data : &v32) < 1)
        return false;
    if ( < 4)
    {
        if (data_type == ImGuiDataType_S8)
            *(*mut i8)p_data = ImClamp(v32, IM_S8_MIN, IM_S8_MAX);
        else if (data_type == ImGuiDataType_U8)
            *(*mut u8)p_data = ImClamp(v32, IM_U8_MIN, IM_U8_MAX);
        else if (data_type == ImGuiDataType_S16)
            *(*mut i16)p_data = (i16)ImClamp(v32, IM_S16_MIN, IM_S16_MAX);
        else if (data_type == ImGuiDataType_U16)
            *(*mut u16)p_data = (u16)ImClamp(v32, IM_U16_MIN, IM_U16_MAX);
        else
            // IM_ASSERT(0);
    }

    return memcmp(&data_backup, p_data, ) != 0;
}

template<typename T>
static c_int DataTypeCompareT(*const T lhs, *const T rhs)
{
    if (*lhs < *rhs) return -1;
    if (*lhs > *rhs) return +1;
    return 0;
}

c_int DataTypeCompare(ImGuiDataType data_type, *const c_void arg_1, *const c_void arg_2)
{
    switch (data_type)
    {
    case ImGuiDataType_S8:     return DataTypeCompareT<i8  >((*const i8  )arg_1, (*const i8  )arg_2);
    case ImGuiDataType_U8:     return DataTypeCompareT<u8  >((*const u8  )arg_1, (*const u8  )arg_2);
    case ImGuiDataType_S16:    return DataTypeCompareT<i16 >((*const i16 )arg_1, (*const i16 )arg_2);
    case ImGuiDataType_U16:    return DataTypeCompareT<u16 >((*const u16 )arg_1, (*const u16 )arg_2);
    case ImGuiDataType_S32:    return DataTypeCompareT<i32 >((*const i32 )arg_1, (*const i32 )arg_2);
    case ImGuiDataType_U32:    return DataTypeCompareT<u32 >((*const u32 )arg_1, (*const u32 )arg_2);
    case ImGuiDataType_S64:    return DataTypeCompareT<ImS64 >((*const ImS64 )arg_1, (*const ImS64 )arg_2);
    case ImGuiDataType_U64:    return DataTypeCompareT<u64 >((*const u64 )arg_1, (*const u64 )arg_2);
    case ImGuiDataType_Float:  return DataTypeCompareT<c_float >((*const c_float )arg_1, (*const c_float )arg_2);
    case ImGuiDataType_Double: return DataTypeCompareT<double>((*const double)arg_1, (*const double)arg_2);
    case ImGuiDataType_COUNT:  break;
    }
    // IM_ASSERT(0);
    return 0;
}

template<typename T>
static bool DataTypeClampT(*mut T v, *const T v_min, *const T v_max)
{
    // Clamp, both sides are optional, return true if modified
    if (v_min && *v < *v_min) { *v = *v_min; return true; }
    if (v_max && *v > *v_max) { *v = *v_max; return true; }
    return false;
}

bool DataTypeClamp(ImGuiDataType data_type, *mut c_void p_data, *const c_void p_min, *const c_void p_max)
{
    switch (data_type)
    {
    case ImGuiDataType_S8:     return DataTypeClampT<i8  >((*mut i8  )p_data, (*const i8  )p_min, (*const i8  )p_max);
    case ImGuiDataType_U8:     return DataTypeClampT<u8  >((*mut u8  )p_data, (*const u8  )p_min, (*const u8  )p_max);
    case ImGuiDataType_S16:    return DataTypeClampT<i16 >((*mut i16 )p_data, (*const i16 )p_min, (*const i16 )p_max);
    case ImGuiDataType_U16:    return DataTypeClampT<u16 >((*mut u16 )p_data, (*const u16 )p_min, (*const u16 )p_max);
    case ImGuiDataType_S32:    return DataTypeClampT<i32 >((*mut i32 )p_data, (*const i32 )p_min, (*const i32 )p_max);
    case ImGuiDataType_U32:    return DataTypeClampT<u32 >((*mut u32 )p_data, (*const u32 )p_min, (*const u32 )p_max);
    case ImGuiDataType_S64:    return DataTypeClampT<ImS64 >((*mut ImS64 )p_data, (*const ImS64 )p_min, (*const ImS64 )p_max);
    case ImGuiDataType_U64:    return DataTypeClampT<u64 >((*mut u64 )p_data, (*const u64 )p_min, (*const u64 )p_max);
    case ImGuiDataType_Float:  return DataTypeClampT<c_float >((*mut c_float )p_data, (*const c_float )p_min, (*const c_float )p_max);
    case ImGuiDataType_Double: return DataTypeClampT<double>((*mut double)p_data, (*const double)p_min, (*const double)p_max);
    case ImGuiDataType_COUNT:  break;
    }
    // IM_ASSERT(0);
    return false;
}

static c_float GetMinimumStepAtDecimalPrecision(decimal_precision: c_int)
{
    static const c_float min_steps[10] = { 1f32, 0.1f, 0.01f, 0.001f, 0.0001f, 0.00001f, 0.000001f, 0.0000001f, 0.00000001f, 0.000000001f };
    if (decimal_precision < 0)
        return FLT_MIN;
    return (decimal_precision < IM_ARRAYSIZE(min_steps)) ? min_steps[decimal_precision] : ImPow(10f32, -decimal_precision);
}

template<typename TYPE>
TYPE RoundScalarWithFormatT(format: *const c_char, ImGuiDataType data_type, TYPE v)
{
    IM_UNUSED(data_type);
    // IM_ASSERT(data_type == ImGuiDataType_Float || data_type == ImGuiDataType_Double);
    let mut  fmt_start: *const c_char = ImParseFormatFindStart(format);
    if (fmt_start[0] != '%' || fmt_start[1] == '%') // Don't apply if the value is not visible in the format string
        return v;

    // Sanitize format
    fmt_sanitized: [c_char;32];
    ImParseFormatSanitizeForPrinting(fmt_start, fmt_sanitized, IM_ARRAYSIZE(fmt_sanitized));
    fmt_start = fmt_sanitized;

    // Format value with our rounding, and read back
    v_str: [c_char;64];
    ImFormatString(v_str, IM_ARRAYSIZE(v_str), fmt_start, v);
    let mut  p: *const c_char = v_str;
    while (*p == ' ')
        p+= 1;
    v = (TYPE)ImAtof(p);

    return v;
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: DragScalar, DragFloat, DragInt, etc.
//-------------------------------------------------------------------------
// - DragBehaviorT<>() [Internal]
// - DragBehavior() [Internal]
// - DragScalar()
// - DragScalarN()
// - DragFloat()
// - DragFloat2()
// - DragFloat3()
// - DragFloat4()
// - DragFloatRange2()
// - DragInt()
// - DragInt2()
// - DragInt3()
// - DragInt4()
// - DragIntRange2()
//-------------------------------------------------------------------------

// This is called by DragBehavior() when the widget is active (held by mouse or being manipulated with Nav controls)
template<typename TYPE, typename SIGNEDTYPE, typename FLOATTYPE>
bool DragBehaviorT(ImGuiDataType data_type, *mut TYPE v, v_speed: c_float, const TYPE v_min, const TYPE v_max, format: *const c_char, ImGuiSliderFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let axis: ImGuiAxis = (flags & ImGuiSliderFlags_Vertical) ? ImGuiAxis_Y : ImGuiAxis_X;
    let is_clamped: bool = (v_min < v_max);
    let is_logarithmic: bool = (flags & ImGuiSliderFlags_Logarithmic) != 0;
    let is_floating_point: bool = (data_type == ImGuiDataType_Float) || (data_type == ImGuiDataType_Double);

    // Default tweak speed
    if (v_speed == 0f32 && is_clamped && (v_max - v_min < f32::MAX))
        v_speed = ((v_max - v_min) * g.DragSpeedDefaultRatio);

    // Inputs accumulates into g.DragCurrentAccum, which is flushed into the current value as soon as it makes a difference with our precision settings
    let adjust_delta: c_float =  0f32;
    if (g.ActiveIdSource == ImGuiInputSource_Mouse && IsMousePosValid() && IsMouseDragPastThreshold(0, g.IO.MouseDragThreshold * DRAG_MOUSE_THRESHOLD_FACTOR))
    {
        adjust_delta = g.IO.MouseDelta[axis];
        if (g.IO.KeyAlt)
            adjust_delta *= 1f32 / 100f32;
        if (g.IO.KeyShift)
            adjust_delta *= 10f32;
    }
    else if (g.ActiveIdSource == ImGuiInputSource_Nav)
    {
        let decimal_precision: c_int = is_floating_point ? ImParseFormatPrecision(format, 3) : 0;
        let tweak_slow: bool = IsKeyDown((g.NavInputSource == ImGuiInputSource_Gamepad) ? ImGuiKey_NavGamepadTweakSlow : ImGuiKey_NavKeyboardTweakSlow);
        let tweak_fast: bool = IsKeyDown((g.NavInputSource == ImGuiInputSource_Gamepad) ? ImGuiKey_NavGamepadTweakFast : ImGuiKey_NavKeyboardTweakFast);
        let tweak_factor: c_float =  tweak_slow ? 1f32 / 1f32 : tweak_fast ? 10f32 : 1f32;
        adjust_delta = GetNavTweakPressedAmount(axis) * tweak_factor;
        v_speed = ImMax(v_speed, GetMinimumStepAtDecimalPrecision(decimal_precision));
    }
    adjust_delta *= v_speed;

    // For vertical drag we currently assume that Up=higher value (like we do with vertical sliders). This may become a parameter.
    if (axis == ImGuiAxis_Y)
        adjust_delta = -adjust_delta;

    // For logarithmic use our range is effectively 0..1 so scale the delta into that range
    if (is_logarithmic && (v_max - v_min < f32::MAX) && ((v_max - v_min) > 0.0000010f32)) // Epsilon to avoid /0
        adjust_delta /= (v_max - v_min);

    // Clear current value on activation
    // Avoid altering values and clamping when we are _already_ past the limits and heading in the same direction, so e.g. if range is 0..255, current value is 300 and we are pushing to the right side, keep the 300.
    let mut is_just_activated: bool =  g.ActiveIdIsJustActivated;
    let mut is_already_past_limits_and_pushing_outward: bool =  is_clamped && ((*v >= v_max && adjust_delta > 0f32) || (*v <= v_min && adjust_delta < 0f32));
    if (is_just_activated || is_already_past_limits_and_pushing_outward)
    {
        g.DragCurrentAccum = 0f32;
        g.DragCurrentAccumDirty = false;
    }
    else if (adjust_delta != 0f32)
    {
        g.DragCurrentAccum += adjust_delta;
        g.DragCurrentAccumDirty = true;
    }

    if (!g.DragCurrentAccumDirty)
        return false;

    TYPE v_cur = *v;
    FLOATTYPE v_old_ref_for_accum_remainder = (FLOATTYPE)0f32;

    let logarithmic_zero_epsilon: c_float =  0f32; // Only valid when is_logarithmic is true
    let zero_deadzone_halfsize: c_float =  0f32; // Drag widgets have no deadzone (as it doesn't make sense)
    if (is_logarithmic)
    {
        // When using logarithmic sliders, we need to clamp to avoid hitting zero, but our choice of clamp value greatly affects slider precision. We attempt to use the specified precision to estimate a good lower bound.
        let decimal_precision: c_int = is_floating_point ? ImParseFormatPrecision(format, 3) : 1;
        logarithmic_zero_epsilon = ImPow(0.1f, decimal_precision);

        // Convert to parametric space, apply delta, convert back
        let v_old_parametric: c_float =  ScaleRatioFromValueT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, v_cur, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);
        let v_new_parametric: c_float =  v_old_parametric + g.DragCurrentAccum;
        v_cur = ScaleValueFromRatioT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, v_new_parametric, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);
        v_old_ref_for_accum_remainder = v_old_parametric;
    }
    else
    {
        v_cur += (SIGNEDTYPE)g.DragCurrentAccum;
    }

    // Round to user desired precision based on format string
    if (is_floating_point && !(flags & ImGuiSliderFlags_NoRoundToFormat))
        v_cur = RoundScalarWithFormatT<TYPE>(format, data_type, v_cur);

    // Preserve remainder after rounding has been applied. This also allow slow tweaking of values.
    g.DragCurrentAccumDirty = false;
    if (is_logarithmic)
    {
        // Convert to parametric space, apply delta, convert back
        let v_new_parametric: c_float =  ScaleRatioFromValueT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, v_cur, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);
        g.DragCurrentAccum -= (v_new_parametric - v_old_ref_for_accum_remainder);
    }
    else
    {
        g.DragCurrentAccum -= ((SIGNEDTYPE)v_cur - (SIGNEDTYPE)*v);
    }

    // Lose zero sign for float/double
    if (v_cur == (TYPE)-0)
        v_cur = (TYPE)0;

    // Clamp values (+ handle overflow/wrap-around for integer types)
    if (*v != v_cur && is_clamped)
    {
        if (v_cur < v_min || (v_cur > *v && adjust_delta < 0f32 && !is_floating_point))
            v_cur = v_min;
        if (v_cur > v_max || (v_cur < *v && adjust_delta > 0f32 && !is_floating_point))
            v_cur = v_max;
    }

    // Apply result
    if (*v == v_cur)
        return false;
    *v = v_cur;
    return true;
}

bool DragBehavior(id: ImGuiID, ImGuiDataType data_type, *mut c_void p_v, v_speed: c_float, *const c_void p_min, *const c_void p_max, format: *const c_char, ImGuiSliderFlags flags)
{
    // Read imgui.cpp "API BREAKING CHANGES" section for 1.78 if you hit this assert.
    // IM_ASSERT((flags == 1 || (flags & ImGuiSliderFlags_InvalidMask_) == 0) && "Invalid ImGuiSliderFlags flags! Has the 'float power' argument been mistakenly cast to flags? Call function with ImGuiSliderFlags_Logarithmic flags instead.");

    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.ActiveId == id)
    {
        // Those are the things we can do easily outside the DragBehaviorT<> template, saves code generation.
        if (g.ActiveIdSource == ImGuiInputSource_Mouse && !g.IO.MouseDown[0])
            ClearActiveID();
        else if (g.ActiveIdSource == ImGuiInputSource_Nav && g.NavActivatePressedId == id && !g.ActiveIdIsJustActivated)
            ClearActiveID();
    }
    if (g.ActiveId != id)
        return false;
    if ((g.LastItemData.InFlags & ImGuiItemFlags_ReadOnly) || (flags & ImGuiSliderFlags_ReadOnly))
        return false;

    switch (data_type)
    {
    case ImGuiDataType_S8:     { i32 v32 = (i32)*(*mut i8)p_v;  let mut r: bool =  DragBehaviorT<i32, i32, c_float>(ImGuiDataType_S32, &v32, v_speed, p_min ? *(*const i8) p_min : IM_S8_MIN,  p_max ? *(*const i8)p_max  : IM_S8_MAX,  format, flags); if (r) *(*mut i8)p_v = v32; return r; }
    case ImGuiDataType_U8:     { let mut v32: u32 = *(*mut u8)p_v;  let mut r: bool =  DragBehaviorT<u32, i32, c_float>(ImGuiDataType_U32, &v32, v_speed, p_min ? *(*const u8) p_min : IM_U8_MIN,  p_max ? *(*const u8)p_max  : IM_U8_MAX,  format, flags); if (r) *(*mut u8)p_v = v32; return r; }
    case ImGuiDataType_S16:    { i32 v32 = (i32)*(*mut i16)p_v; let mut r: bool =  DragBehaviorT<i32, i32, c_float>(ImGuiDataType_S32, &v32, v_speed, p_min ? *(*const i16)p_min : IM_S16_MIN, p_max ? *(*const i16)p_max : IM_S16_MAX, format, flags); if (r) *(*mut i16)p_v = (i16)v32; return r; }
    case ImGuiDataType_U16:    { let mut v32: u32 = *(*mut u16)p_v; let mut r: bool =  DragBehaviorT<u32, i32, c_float>(ImGuiDataType_U32, &v32, v_speed, p_min ? *(*const u16)p_min : IM_U16_MIN, p_max ? *(*const u16)p_max : IM_U16_MAX, format, flags); if (r) *(*mut u16)p_v = (u16)v32; return r; }
    case ImGuiDataType_S32:    return DragBehaviorT<i32, i32, c_float >(data_type, (*mut i32)p_v,  v_speed, p_min ? *(*const i32 )p_min : IM_S32_MIN, p_max ? *(*const i32 )p_max : IM_S32_MAX, format, flags);
    case ImGuiDataType_U32:    return DragBehaviorT<u32, i32, c_float >(data_type, (*mut u32)p_v,  v_speed, p_min ? *(*const u32 )p_min : IM_U32_MIN, p_max ? *(*const u32 )p_max : IM_U32_MAX, format, flags);
    case ImGuiDataType_S64:    return DragBehaviorT<ImS64, ImS64, double>(data_type, (*mut ImS64)p_v,  v_speed, p_min ? *(*const ImS64 )p_min : IM_S64_MIN, p_max ? *(*const ImS64 )p_max : IM_S64_MAX, format, flags);
    case ImGuiDataType_U64:    return DragBehaviorT<u64, ImS64, double>(data_type, (*mut u64)p_v,  v_speed, p_min ? *(*const u64 )p_min : IM_U64_MIN, p_max ? *(*const u64 )p_max : IM_U64_MAX, format, flags);
    case ImGuiDataType_Float:  return DragBehaviorT<c_float, c_float, c_float >(data_type, (*mut c_float)p_v,  v_speed, p_min ? *(*const c_float )p_min : -f32::MAX,   p_max ? *(*const c_float )p_max : f32::MAX,    format, flags);
    case ImGuiDataType_Double: return DragBehaviorT<double,double,double>(data_type, (*mut double)p_v, v_speed, p_min ? *(*const double)p_min : -DBL_MAX,   p_max ? *(*const double)p_max : DBL_MAX,    format, flags);
    case ImGuiDataType_COUNT:  break;
    }
    // IM_ASSERT(0);
    return false;
}

// Note: p_data, p_min and p_max are _pointers_ to a memory address holding the data. For a Drag widget, p_min and p_max are optional.
// Read code of e.g. DragFloat(), DragInt() etc. or examples in 'Demo.Widgets.Data Types' to understand how to use this function directly.
bool DragScalar(label: *const c_char, ImGuiDataType data_type, *mut c_void p_data, v_speed: c_float, *const c_void p_min, *const c_void p_max, format: *const c_char, ImGuiSliderFlags flags)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    const let mut style = &mut g.Style;
    let mut id: ImGuiID =  window.GetID(label);
    let w: c_float =  CalcItemWidth();

    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);
    let mut frame_bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + ImVec2::new(w, label_size.y + style.FramePadding.y * 2.00f32));
    let mut total_bb: ImRect = ImRect::new(frame_bb.Min, frame_bb.Max + ImVec2::new(label_size.x > 0f32 ? style.ItemInnerSpacing.x + label_size.x : 0f32, 0f32));

    let temp_input_allowed: bool = (flags & ImGuiSliderFlags_NoInput) == 0;
    ItemSize(total_bb, style.FramePadding.y);
    if (!ItemAdd(total_bb, id, &frame_bb, temp_input_allowed ? ImGuiItemFlags_Inputable : 0))
        return false;

    // Default format string when passing NULL
    if (format == null_mut())
        format = DataTypeGetInfo(data_type).PrintFmt;
    else if (data_type == ImGuiDataType_S32 && strcmp(format, "%d") != 0) // (FIXME-LEGACY: Patch old "%.0f" format string to use "%d", read function more details.)
        format = PatchFormatStringFloatToInt(format);

    let hovered: bool = ItemHoverable(frame_bb, id);
    let mut temp_input_is_active: bool =  temp_input_allowed && TempInputIsActive(id);
    if (!temp_input_is_active)
    {
        // Tabbing or CTRL-clicking on Drag turns it into an InputText
        let input_requested_by_tabbing: bool = temp_input_allowed && (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_FocusedByTabbing) != 0;
        let clicked: bool = (hovered && g.IO.MouseClicked[0]);
        let double_clicked: bool = (hovered && g.IO.MouseClickedCount[0] == 2);
        let make_active: bool = (input_requested_by_tabbing || clicked || double_clicked || g.NavActivateId == id || g.NavActivateInputId == id);
        if (make_active && temp_input_allowed)
            if (input_requested_by_tabbing || (clicked && g.IO.KeyCtrl) || double_clicked || g.NavActivateInputId == id)
                temp_input_is_active = true;

        // (Optional) simple click (without moving) turns Drag into an InputText
        if (g.IO.ConfigDragClickToInputText && temp_input_allowed && !temp_input_is_active)
            if (g.ActiveId == id && hovered && g.IO.MouseReleased[0] && !IsMouseDragPastThreshold(0, g.IO.MouseDragThreshold * DRAG_MOUSE_THRESHOLD_FACTOR))
            {
                g.NavActivateId = g.NavActivateInputId = id;
                g.NavActivateFlags = ImGuiActivateFlags_PreferInput;
                temp_input_is_active = true;
            }

        if (make_active && !temp_input_is_active)
        {
            SetActiveID(id, window);
            SetFocusID(id, window);
            FocusWindow(window);
            g.ActiveIdUsingNavDirMask = (1 << ImGuiDir_Left) | (1 << ImGuiDir_Right);
        }
    }

    if (temp_input_is_active)
    {
        // Only clamp CTRL+Click input when ImGuiSliderFlags_AlwaysClamp is set
        let is_clamp_input: bool = (flags & ImGuiSliderFlags_AlwaysClamp) != 0 && (p_min == null_mut() || p_max == null_mut() || DataTypeCompare(data_type, p_min, p_max) < 0);
        return TempInputScalar(frame_bb, id, label, data_type, p_data, format, is_clamp_input ? p_min : null_mut(), is_clamp_input ? p_max : null_mut());
    }

    // Draw frame
    let frame_col: u32 = GetColorU32(g.ActiveId == id ? ImGuiCol_FrameBgActive : hovered ? ImGuiCol_FrameBgHovered : ImGuiCol_FrameBg);
    RenderNavHighlight(frame_bb, id);
    RenderFrame(frame_bb.Min, frame_bb.Max, frame_col, true, style.FrameRounding);

    // Drag behavior
    let value_changed: bool = DragBehavior(id, data_type, p_data, v_speed, p_min, p_max, format, flags);
    if (value_changed)
        MarkItemEdited(id);

    // Display value using user-provided display format so user can add prefix/suffix/decorations to the value.
    value_buf: [c_char;64];
    let mut  value_buf_end: *const c_char = value_buf + DataTypeFormatString(value_buf, IM_ARRAYSIZE(value_buf), data_type, p_data, format);
    if (g.LogEnabled)
        LogSetNextTextDecoration("{", "}");
    RenderTextClipped(frame_bb.Min, frame_bb.Max, value_buf, value_buf_end, null_mut(), ImVec2::new2(0.5f32, 0.5f32));

    if (label_size.x > 0f32)
        RenderText(ImVec2::new(frame_bb.Max.x + style.ItemInnerSpacing.x, frame_bb.Min.y + style.FramePadding.y), label);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags);
    return value_changed;
}

bool DragScalarN(label: *const c_char, ImGuiDataType data_type, *mut c_void p_data, components: c_int, v_speed: c_float, *const c_void p_min, *const c_void p_max, format: *const c_char, ImGuiSliderFlags flags)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut value_changed: bool =  false;
    BeginGroup();
    PushID(label);
    PushMultiItemsWidths(components, CalcItemWidth());
    size_t type_size = GDataTypeInfo[data_type].Size;
    for (let i: c_int = 0; i < components; i++)
    {
        PushID(i);
        if (i > 0)
            SameLine(0, g.Style.ItemInnerSpacing.x);
        value_changed |= DragScalar("", data_type, p_data, v_speed, p_min, p_max, format, flags);
        PopID();
        PopItemWidth();
        p_data = ((*mut char)p_data + type_size);
    }
    PopID();

    let mut  label_end: *const c_char = FindRenderedTextEnd(label);
    if (label != label_end)
    {
        SameLine(0, g.Style.ItemInnerSpacing.x);
        TextEx(label, label_end);
    }

    EndGroup();
    return value_changed;
}

bool DragFloat(label: *const c_char, *mut v: c_float, v_speed: c_float, v_min: c_float, v_max: c_float, format: *const c_char, ImGuiSliderFlags flags)
{
    return DragScalar(label, ImGuiDataType_Float, v, v_speed, &v_min, &v_max, format, flags);
}

bool DragFloat2(label: *const c_char, c_float v[2], v_speed: c_float, v_min: c_float, v_max: c_float, format: *const c_char, ImGuiSliderFlags flags)
{
    return DragScalarN(label, ImGuiDataType_Float, v, 2, v_speed, &v_min, &v_max, format, flags);
}

bool DragFloat3(label: *const c_char, c_float v[3], v_speed: c_float, v_min: c_float, v_max: c_float, format: *const c_char, ImGuiSliderFlags flags)
{
    return DragScalarN(label, ImGuiDataType_Float, v, 3, v_speed, &v_min, &v_max, format, flags);
}

bool DragFloat4(label: *const c_char, c_float v[4], v_speed: c_float, v_min: c_float, v_max: c_float, format: *const c_char, ImGuiSliderFlags flags)
{
    return DragScalarN(label, ImGuiDataType_Float, v, 4, v_speed, &v_min, &v_max, format, flags);
}

// NB: You likely want to specify the ImGuiSliderFlags_AlwaysClamp when using this.
bool DragFloatRange2(label: *const c_char, *mut v_current_min: c_float, *mut v_current_max: c_float, v_speed: c_float, v_min: c_float, v_max: c_float, format: *const c_char, format_max: *const c_char, ImGuiSliderFlags flags)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    PushID(label);
    BeginGroup();
    PushMultiItemsWidths(2, CalcItemWidth());

    let min_min: c_float =  (v_min >= v_max) ? -f32::MAX : v_min;
    let min_max: c_float =  (v_min >= v_max) ? *v_current_max : ImMin(v_max, *v_current_max);
    ImGuiSliderFlags min_flags = flags | ((min_min == min_max) ? ImGuiSliderFlags_ReadOnly : 0);
    let mut value_changed: bool =  DragScalar("##min", ImGuiDataType_Float, v_current_min, v_speed, &min_min, &min_max, format, min_flags);
    PopItemWidth();
    SameLine(0, g.Style.ItemInnerSpacing.x);

    let max_min: c_float =  (v_min >= v_max) ? *v_current_min : ImMax(v_min, *v_current_min);
    let max_max: c_float =  (v_min >= v_max) ? f32::MAX : v_max;
    ImGuiSliderFlags max_flags = flags | ((max_min == max_max) ? ImGuiSliderFlags_ReadOnly : 0);
    value_changed |= DragScalar("##max", ImGuiDataType_Float, v_current_max, v_speed, &max_min, &max_max, format_max ? format_max : format, max_flags);
    PopItemWidth();
    SameLine(0, g.Style.ItemInnerSpacing.x);

    TextEx(label, FindRenderedTextEnd(label));
    EndGroup();
    PopID();

    return value_changed;
}

// NB: v_speed is float to allow adjusting the drag speed with more precision
bool DragInt(label: *const c_char, *mut v: c_int, v_speed: c_float, v_min: c_int, v_max: c_int, format: *const c_char, ImGuiSliderFlags flags)
{
    return DragScalar(label, ImGuiDataType_S32, v, v_speed, &v_min, &v_max, format, flags);
}

bool DragInt2(label: *const c_char, c_int v[2], v_speed: c_float, v_min: c_int, v_max: c_int, format: *const c_char, ImGuiSliderFlags flags)
{
    return DragScalarN(label, ImGuiDataType_S32, v, 2, v_speed, &v_min, &v_max, format, flags);
}

bool DragInt3(label: *const c_char, c_int v[3], v_speed: c_float, v_min: c_int, v_max: c_int, format: *const c_char, ImGuiSliderFlags flags)
{
    return DragScalarN(label, ImGuiDataType_S32, v, 3, v_speed, &v_min, &v_max, format, flags);
}

bool DragInt4(label: *const c_char, c_int v[4], v_speed: c_float, v_min: c_int, v_max: c_int, format: *const c_char, ImGuiSliderFlags flags)
{
    return DragScalarN(label, ImGuiDataType_S32, v, 4, v_speed, &v_min, &v_max, format, flags);
}

// NB: You likely want to specify the ImGuiSliderFlags_AlwaysClamp when using this.
bool DragIntRange2(label: *const c_char, *mut v_current_min: c_int, *mut v_current_max: c_int, v_speed: c_float, v_min: c_int, v_max: c_int, format: *const c_char, format_max: *const c_char, ImGuiSliderFlags flags)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    PushID(label);
    BeginGroup();
    PushMultiItemsWidths(2, CalcItemWidth());

    let min_min: c_int = (v_min >= v_max) ? INT_MIN : v_min;
    let min_max: c_int = (v_min >= v_max) ? *v_current_max : ImMin(v_max, *v_current_max);
    ImGuiSliderFlags min_flags = flags | ((min_min == min_max) ? ImGuiSliderFlags_ReadOnly : 0);
    let mut value_changed: bool =  DragInt("##min", v_current_min, v_speed, min_min, min_max, format, min_flags);
    PopItemWidth();
    SameLine(0, g.Style.ItemInnerSpacing.x);

    let max_min: c_int = (v_min >= v_max) ? *v_current_min : ImMax(v_min, *v_current_min);
    let max_max: c_int = (v_min >= v_max) ? INT_MAX : v_max;
    ImGuiSliderFlags max_flags = flags | ((max_min == max_max) ? ImGuiSliderFlags_ReadOnly : 0);
    value_changed |= DragInt("##max", v_current_max, v_speed, max_min, max_max, format_max ? format_max : format, max_flags);
    PopItemWidth();
    SameLine(0, g.Style.ItemInnerSpacing.x);

    TextEx(label, FindRenderedTextEnd(label));
    EndGroup();
    PopID();

    return value_changed;
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: SliderScalar, SliderFloat, SliderInt, etc.
//-------------------------------------------------------------------------
// - ScaleRatioFromValueT<> [Internal]
// - ScaleValueFromRatioT<> [Internal]
// - SliderBehaviorT<>() [Internal]
// - SliderBehavior() [Internal]
// - SliderScalar()
// - SliderScalarN()
// - SliderFloat()
// - SliderFloat2()
// - SliderFloat3()
// - SliderFloat4()
// - SliderAngle()
// - SliderInt()
// - SliderInt2()
// - SliderInt3()
// - SliderInt4()
// - VSliderScalar()
// - VSliderFloat()
// - VSliderInt()
//-------------------------------------------------------------------------

// Convert a value v in the output space of a slider into a parametric position on the slider itself (the logical opposite of ScaleValueFromRatioT)
template<typename TYPE, typename SIGNEDTYPE, typename FLOATTYPE>
c_float ScaleRatioFromValueT(ImGuiDataType data_type, TYPE v, TYPE v_min, TYPE v_max, is_logarithmic: bool, logarithmic_zero_epsilon: c_float, zero_deadzone_halfsize: c_float)
{
    if (v_min == v_max)
        return 0f32;
    IM_UNUSED(data_type);

    const TYPE v_clamped = (v_min < v_max) ? ImClamp(v, v_min, v_max) : ImClamp(v, v_max, v_min);
    if (is_logarithmic)
    {
        let mut flipped: bool =  v_max < v_min;

        if (flipped) // Handle the case where the range is backwards
            ImSwap(v_min, v_max);

        // Fudge min/max to avoid getting close to log(0)
        FLOATTYPE v_min_fudged = (ImAbs((FLOATTYPE)v_min) < logarithmic_zero_epsilon) ? ((v_min < 0f32) ? -logarithmic_zero_epsilon : logarithmic_zero_epsilon) : (FLOATTYPE)v_min;
        FLOATTYPE v_max_fudged = (ImAbs((FLOATTYPE)v_max) < logarithmic_zero_epsilon) ? ((v_max < 0f32) ? -logarithmic_zero_epsilon : logarithmic_zero_epsilon) : (FLOATTYPE)v_max;

        // Awkward special cases - we need ranges of the form (-100 .. 0) to convert to (-100 .. -epsilon), not (-100 .. epsilon)
        if ((v_min == 0f32) && (v_max < 0f32))
            v_min_fudged = -logarithmic_zero_epsilon;
        else if ((v_max == 0f32) && (v_min < 0f32))
            v_max_fudged = -logarithmic_zero_epsilon;

        let mut result: c_float = 0f32;
        if (v_clamped <= v_min_fudged)
            result = 0f32; // Workaround for values that are in-range but below our fudge
        else if (v_clamped >= v_max_fudged)
            result = 1f32; // Workaround for values that are in-range but above our fudge
        else if ((v_min * v_max) < 0f32) // Range crosses zero, so split into two portions
        {
            let zero_point_center: c_float =  (-v_min) / (v_max - v_min); // The zero point in parametric space.  There's an argument we should take the logarithmic nature into account when calculating this, but for now this should do (and the most common case of a symmetrical range works fine)
            let zero_point_snap_L: c_float =  zero_point_center - zero_deadzone_halfsize;
            let zero_point_snap_R: c_float =  zero_point_center + zero_deadzone_halfsize;
            if (v == 0f32)
                result = zero_point_center; // Special case for exactly zero
            else if (v < 0f32)
                result = (1f32 - (ImLog(-(FLOATTYPE)v_clamped / logarithmic_zero_epsilon) / ImLog(-v_min_fudged / logarithmic_zero_epsilon))) * zero_point_snap_L;
            else
                result = zero_point_snap_R + ((ImLog((FLOATTYPE)v_clamped / logarithmic_zero_epsilon) / ImLog(v_max_fudged / logarithmic_zero_epsilon)) * (1f32 - zero_point_snap_R));
        }
        else if ((v_min < 0f32) || (v_max < 0f32)) // Entirely negative slider
            result = 1f32 - (ImLog(-(FLOATTYPE)v_clamped / -v_max_fudged) / ImLog(-v_min_fudged / -v_max_fudged));
        else
            result = (ImLog((FLOATTYPE)v_clamped / v_min_fudged) / ImLog(v_max_fudged / v_min_fudged));

        return flipped ? (1f32 - result) : result;
    }
    else
    {
        // Linear slider
        return ((FLOATTYPE)(SIGNEDTYPE)(v_clamped - v_min) / (FLOATTYPE)(SIGNEDTYPE)(v_max - v_min));
    }
}

// Convert a parametric position on a slider into a value v in the output space (the logical opposite of ScaleRatioFromValueT)
template<typename TYPE, typename SIGNEDTYPE, typename FLOATTYPE>
TYPE ScaleValueFromRatioT(ImGuiDataType data_type, t: c_float, TYPE v_min, TYPE v_max, is_logarithmic: bool, logarithmic_zero_epsilon: c_float, zero_deadzone_halfsize: c_float)
{
    // We special-case the extents because otherwise our logarithmic fudging can lead to "mathematically correct"
    // but non-intuitive behaviors like a fully-left slider not actually reaching the minimum value. Also generally simpler.
    if (t <= 0f32 || v_min == v_max)
        return v_min;
    if (t >= 1f32)
        return v_max;

    TYPE result = (TYPE)0;
    if (is_logarithmic)
    {
        // Fudge min/max to avoid getting silly results close to zero
        FLOATTYPE v_min_fudged = (ImAbs((FLOATTYPE)v_min) < logarithmic_zero_epsilon) ? ((v_min < 0f32) ? -logarithmic_zero_epsilon : logarithmic_zero_epsilon) : (FLOATTYPE)v_min;
        FLOATTYPE v_max_fudged = (ImAbs((FLOATTYPE)v_max) < logarithmic_zero_epsilon) ? ((v_max < 0f32) ? -logarithmic_zero_epsilon : logarithmic_zero_epsilon) : (FLOATTYPE)v_max;

        let flipped: bool = v_max < v_min; // Check if range is "backwards"
        if (flipped)
            ImSwap(v_min_fudged, v_max_fudged);

        // Awkward special case - we need ranges of the form (-100 .. 0) to convert to (-100 .. -epsilon), not (-100 .. epsilon)
        if ((v_max == 0f32) && (v_min < 0f32))
            v_max_fudged = -logarithmic_zero_epsilon;

        let t_with_flip: c_float =  flipped ? (1f32 - t) : t; // t, but flipped if necessary to account for us flipping the range

        if ((v_min * v_max) < 0f32) // Range crosses zero, so we have to do this in two parts
        {
            let zero_point_center: c_float =  (-ImMin(v_min, v_max)) / ImAbs(v_max - v_min); // The zero point in parametric space
            let zero_point_snap_L: c_float =  zero_point_center - zero_deadzone_halfsize;
            let zero_point_snap_R: c_float =  zero_point_center + zero_deadzone_halfsize;
            if (t_with_flip >= zero_point_snap_L && t_with_flip <= zero_point_snap_R)
                result = (TYPE)0f32; // Special case to make getting exactly zero possible (the epsilon prevents it otherwise)
            else if (t_with_flip < zero_point_center)
                result = (TYPE)-(logarithmic_zero_epsilon * ImPow(-v_min_fudged / logarithmic_zero_epsilon, (FLOATTYPE)(1f32 - (t_with_flip / zero_point_snap_L))));
            else
                result = (TYPE)(logarithmic_zero_epsilon * ImPow(v_max_fudged / logarithmic_zero_epsilon, (FLOATTYPE)((t_with_flip - zero_point_snap_R) / (1f32 - zero_point_snap_R))));
        }
        else if ((v_min < 0f32) || (v_max < 0f32)) // Entirely negative slider
            result = (TYPE)-(-v_max_fudged * ImPow(-v_min_fudged / -v_max_fudged, (FLOATTYPE)(1f32 - t_with_flip)));
        else
            result = (TYPE)(v_min_fudged * ImPow(v_max_fudged / v_min_fudged, (FLOATTYPE)t_with_flip));
    }
    else
    {
        // Linear slider
        let is_floating_point: bool = (data_type == ImGuiDataType_Float) || (data_type == ImGuiDataType_Double);
        if (is_floating_point)
        {
            result = ImLerp(v_min, v_max, t);
        }
        else if (t < 1.0)
        {
            // - For integer values we want the clicking position to match the grab box so we round above
            //   This code is carefully tuned to work with large values (e.g. high ranges of U64) while preserving this property..
            // - Not doing a *1.0 multiply at the end of a range as it tends to be lossy. While absolute aiming at a large s64/u64
            //   range is going to be imprecise anyway, with this check we at least make the edge values matches expected limits.
            FLOATTYPE v_new_off_f = (SIGNEDTYPE)(v_max - v_min) * t;
            result = (TYPE)((SIGNEDTYPE)v_min + (SIGNEDTYPE)(v_new_off_f + (FLOATTYPE)(v_min > v_max ? -0.5 : 0.5)));
        }
    }

    return result;
}

// FIXME: Try to move more of the code into shared SliderBehavior()
template<typename TYPE, typename SIGNEDTYPE, typename FLOATTYPE>
bool SliderBehaviorT(bb: &ImRect, id: ImGuiID, ImGuiDataType data_type, *mut TYPE v, const TYPE v_min, const TYPE v_max, format: *const c_char, ImGuiSliderFlags flags, *mut ImRect out_grab_bb)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    const let mut style = &mut g.Style;

    let axis: ImGuiAxis = (flags & ImGuiSliderFlags_Vertical) ? ImGuiAxis_Y : ImGuiAxis_X;
    let is_logarithmic: bool = (flags & ImGuiSliderFlags_Logarithmic) != 0;
    let is_floating_point: bool = (data_type == ImGuiDataType_Float) || (data_type == ImGuiDataType_Double);
    const SIGNEDTYPE v_range = (v_min < v_max ? v_max - v_min : v_min - v_max);

    // Calculate bounds
    let grab_padding: c_float =  2.0f32; // FIXME: Should be part of style.
    let slider_sz: c_float =  (bb.Max[axis] - bb.Min[axis]) - grab_padding * 2.0f32;
    let grab_sz: c_float =  style.GrabMinSize;
    if (!is_floating_point && v_range >= 0)                                     // v_range < 0 may happen on integer overflows
        grab_sz = ImMax((slider_sz / (v_range + 1)), style.GrabMinSize); // For integer sliders: if possible have the grab size represent 1 unit
    grab_sz = ImMin(grab_sz, slider_sz);
    let slider_usable_sz: c_float =  slider_sz - grab_sz;
    let slider_usable_pos_min: c_float =  bb.Min[axis] + grab_padding + grab_sz * 0.5f32;
    let slider_usable_pos_max: c_float =  bb.Max[axis] - grab_padding - grab_sz * 0.5f32;

    let logarithmic_zero_epsilon: c_float =  0f32; // Only valid when is_logarithmic is true
    let zero_deadzone_halfsize: c_float =  0f32; // Only valid when is_logarithmic is true
    if (is_logarithmic)
    {
        // When using logarithmic sliders, we need to clamp to avoid hitting zero, but our choice of clamp value greatly affects slider precision. We attempt to use the specified precision to estimate a good lower bound.
        let decimal_precision: c_int = is_floating_point ? ImParseFormatPrecision(format, 3) : 1;
        logarithmic_zero_epsilon = ImPow(0.1f, decimal_precision);
        zero_deadzone_halfsize = (style.LogSliderDeadzone * 0.5f32) / ImMax(slider_usable_sz, 1f32);
    }

    // Process interacting with the slider
    let mut value_changed: bool =  false;
    if (g.ActiveId == id)
    {
        let mut set_new_value: bool =  false;
        let clicked_t: c_float =  0f32;
        if (g.ActiveIdSource == ImGuiInputSource_Mouse)
        {
            if (!g.IO.MouseDown[0])
            {
                ClearActiveID();
            }
            else
            {
                let mouse_abs_pos: c_float =  g.IO.MousePos[axis];
                if (g.ActiveIdIsJustActivated)
                {
                    let grab_t: c_float =  ScaleRatioFromValueT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, *v, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);
                    if (axis == ImGuiAxis_Y)
                        grab_t = 1f32 - grab_t;
                    let grab_pos: c_float =  ImLerp(slider_usable_pos_min, slider_usable_pos_max, grab_t);
                    let clicked_around_grab: bool = (mouse_abs_pos >= grab_pos - grab_sz * 0.5f32 - 1f32) && (mouse_abs_pos <= grab_pos + grab_sz * 0.5f32 + 1f32); // No harm being extra generous here.
                    g.SliderGrabClickOffset = (clicked_around_grab && is_floating_point) ? mouse_abs_pos - grab_pos : 0f32;
                }
                if (slider_usable_sz > 0f32)
                    clicked_t = ImSaturate((mouse_abs_pos - g.SliderGrabClickOffset - slider_usable_pos_min) / slider_usable_sz);
                if (axis == ImGuiAxis_Y)
                    clicked_t = 1f32 - clicked_t;
                set_new_value = true;
            }
        }
        else if (g.ActiveIdSource == ImGuiInputSource_Nav)
        {
            if (g.ActiveIdIsJustActivated)
            {
                g.SliderCurrentAccum = 0f32; // Reset any stored nav delta upon activation
                g.SliderCurrentAccumDirty = false;
            }

            let input_delta: c_float =  (axis == ImGuiAxis_X) ? GetNavTweakPressedAmount(axis) : -GetNavTweakPressedAmount(axis);
            if (input_delta != 0f32)
            {
                let tweak_slow: bool = IsKeyDown((g.NavInputSource == ImGuiInputSource_Gamepad) ? ImGuiKey_NavGamepadTweakSlow : ImGuiKey_NavKeyboardTweakSlow);
                let tweak_fast: bool = IsKeyDown((g.NavInputSource == ImGuiInputSource_Gamepad) ? ImGuiKey_NavGamepadTweakFast : ImGuiKey_NavKeyboardTweakFast);
                let decimal_precision: c_int = is_floating_point ? ImParseFormatPrecision(format, 3) : 0;
                if (decimal_precision > 0)
                {
                    input_delta /= 100f32;    // Gamepad/keyboard tweak speeds in % of slider bounds
                    if (tweak_slow)
                        input_delta /= 10f32;
                }
                else
                {
                    if ((v_range >= -100f32 && v_range <= 100f32) || tweak_slow)
                        input_delta = ((input_delta < 0f32) ? -1f32 : +1f32) / v_range; // Gamepad/keyboard tweak speeds in integer steps
                    else
                        input_delta /= 100f32;
                }
                if (tweak_fast)
                    input_delta *= 10f32;

                g.SliderCurrentAccum += input_delta;
                g.SliderCurrentAccumDirty = true;
            }

            let delta: c_float =  g.SliderCurrentAccum;
            if (g.NavActivatePressedId == id && !g.ActiveIdIsJustActivated)
            {
                ClearActiveID();
            }
            else if (g.SliderCurrentAccumDirty)
            {
                clicked_t = ScaleRatioFromValueT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, *v, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);

                if ((clicked_t >= 1f32 && delta > 0f32) || (clicked_t <= 0f32 && delta < 0f32)) // This is to avoid applying the saturation when already past the limits
                {
                    set_new_value = false;
                    g.SliderCurrentAccum = 0f32; // If pushing up against the limits, don't continue to accumulate
                }
                else
                {
                    set_new_value = true;
                    let old_clicked_t: c_float =  clicked_t;
                    clicked_t = ImSaturate(clicked_t + delta);

                    // Calculate what our "new" clicked_t will be, and thus how far we actually moved the slider, and subtract this from the accumulator
                    TYPE v_new = ScaleValueFromRatioT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, clicked_t, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);
                    if (is_floating_point && !(flags & ImGuiSliderFlags_NoRoundToFormat))
                        v_new = RoundScalarWithFormatT<TYPE>(format, data_type, v_new);
                    let new_clicked_t: c_float =  ScaleRatioFromValueT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, v_new, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);

                    if (delta > 0)
                        g.SliderCurrentAccum -= ImMin(new_clicked_t - old_clicked_t, delta);
                    else
                        g.SliderCurrentAccum -= ImMax(new_clicked_t - old_clicked_t, delta);
                }

                g.SliderCurrentAccumDirty = false;
            }
        }

        if (set_new_value)
        {
            TYPE v_new = ScaleValueFromRatioT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, clicked_t, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);

            // Round to user desired precision based on format string
            if (is_floating_point && !(flags & ImGuiSliderFlags_NoRoundToFormat))
                v_new = RoundScalarWithFormatT<TYPE>(format, data_type, v_new);

            // Apply result
            if (*v != v_new)
            {
                *v = v_new;
                value_changed = true;
            }
        }
    }

    if (slider_sz < 1f32)
    {
        *out_grab_bb = ImRect::new(bb.Min, bb.Min);
    }
    else
    {
        // Output grab position so it can be displayed by the caller
        let grab_t: c_float =  ScaleRatioFromValueT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, *v, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);
        if (axis == ImGuiAxis_Y)
            grab_t = 1f32 - grab_t;
        let grab_pos: c_float =  ImLerp(slider_usable_pos_min, slider_usable_pos_max, grab_t);
        if (axis == ImGuiAxis_X)
            *out_grab_bb = ImRect::new(grab_pos - grab_sz * 0.5f32, bb.Min.y + grab_padding, grab_pos + grab_sz * 0.5f32, bb.Max.y - grab_padding);
        else
            *out_grab_bb = ImRect::new(bb.Min.x + grab_padding, grab_pos - grab_sz * 0.5f32, bb.Max.x - grab_padding, grab_pos + grab_sz * 0.5f32);
    }

    return value_changed;
}

// For 32-bit and larger types, slider bounds are limited to half the natural type range.
// So e.g. an integer Slider between INT_MAX-10 and INT_MAX will fail, but an integer Slider between INT_MAX/2-10 and INT_MAX/2 will be ok.
// It would be possible to lift that limitation with some work but it doesn't seem to be worth it for sliders.
bool SliderBehavior(bb: &ImRect, id: ImGuiID, ImGuiDataType data_type, *mut c_void p_v, *const c_void p_min, *const c_void p_max, format: *const c_char, ImGuiSliderFlags flags, *mut ImRect out_grab_bb)
{
    // Read imgui.cpp "API BREAKING CHANGES" section for 1.78 if you hit this assert.
    // IM_ASSERT((flags == 1 || (flags & ImGuiSliderFlags_InvalidMask_) == 0) && "Invalid ImGuiSliderFlags flag!  Has the 'float power' argument been mistakenly cast to flags? Call function with ImGuiSliderFlags_Logarithmic flags instead.");

    // Those are the things we can do easily outside the SliderBehaviorT<> template, saves code generation.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if ((g.LastItemData.InFlags & ImGuiItemFlags_ReadOnly) || (flags & ImGuiSliderFlags_ReadOnly))
        return false;

    switch (data_type)
    {
    case ImGuiDataType_S8:  { i32 v32 = (i32)*(*mut i8)p_v;  let mut r: bool =  SliderBehaviorT<i32, i32, c_float>(bb, id, ImGuiDataType_S32, &v32, *(*const i8)p_min,  *(*const i8)p_max,  format, flags, out_grab_bb); if (r) *(*mut i8)p_v  = v32;  return r; }
    case ImGuiDataType_U8:  { let mut v32: u32 = *(*mut u8)p_v;  let mut r: bool =  SliderBehaviorT<u32, i32, c_float>(bb, id, ImGuiDataType_U32, &v32, *(*const u8)p_min,  *(*const u8)p_max,  format, flags, out_grab_bb); if (r) *(*mut u8)p_v  = v32;  return r; }
    case ImGuiDataType_S16: { i32 v32 = (i32)*(*mut i16)p_v; let mut r: bool =  SliderBehaviorT<i32, i32, c_float>(bb, id, ImGuiDataType_S32, &v32, *(*const i16)p_min, *(*const i16)p_max, format, flags, out_grab_bb); if (r) *(*mut i16)p_v = (i16)v32; return r; }
    case ImGuiDataType_U16: { let mut v32: u32 = *(*mut u16)p_v; let mut r: bool =  SliderBehaviorT<u32, i32, c_float>(bb, id, ImGuiDataType_U32, &v32, *(*const u16)p_min, *(*const u16)p_max, format, flags, out_grab_bb); if (r) *(*mut u16)p_v = (u16)v32; return r; }
    case ImGuiDataType_S32:
        // IM_ASSERT(*(*const i32)p_min >= IM_S32_MIN / 2 && *(*const i32)p_max <= IM_S32_MAX / 2);
        return SliderBehaviorT<i32, i32, c_float >(bb, id, data_type, (*mut i32)p_v,  *(*const i32)p_min,  *(*const i32)p_max,  format, flags, out_grab_bb);
    case ImGuiDataType_U32:
        // IM_ASSERT(*(*const u32)p_max <= IM_U32_MAX / 2);
        return SliderBehaviorT<u32, i32, c_float >(bb, id, data_type, (*mut u32)p_v,  *(*const u32)p_min,  *(*const u32)p_max,  format, flags, out_grab_bb);
    case ImGuiDataType_S64:
        // IM_ASSERT(*(*const ImS64)p_min >= IM_S64_MIN / 2 && *(*const ImS64)p_max <= IM_S64_MAX / 2);
        return SliderBehaviorT<ImS64, ImS64, double>(bb, id, data_type, (*mut ImS64)p_v,  *(*const ImS64)p_min,  *(*const ImS64)p_max,  format, flags, out_grab_bb);
    case ImGuiDataType_U64:
        // IM_ASSERT(*(*const u64)p_max <= IM_U64_MAX / 2);
        return SliderBehaviorT<u64, ImS64, double>(bb, id, data_type, (*mut u64)p_v,  *(*const u64)p_min,  *(*const u64)p_max,  format, flags, out_grab_bb);
    case ImGuiDataType_Float:
        // IM_ASSERT(*(*const c_float)p_min >= -f32::MAX / 2.0f32 && *(*const c_float)p_max <= f32::MAX / 2.00f32);
        return SliderBehaviorT<c_float, c_float, c_float >(bb, id, data_type, (*mut c_float)p_v,  *(*const c_float)p_min,  *(*const c_float)p_max,  format, flags, out_grab_bb);
    case ImGuiDataType_Double:
        // IM_ASSERT(*(*const double)p_min >= -DBL_MAX / 2.0f32 && *(*const double)p_max <= DBL_MAX / 2.00f32);
        return SliderBehaviorT<double, double, double>(bb, id, data_type, (*mut double)p_v, *(*const double)p_min, *(*const double)p_max, format, flags, out_grab_bb);
    case ImGuiDataType_COUNT: break;
    }
    // IM_ASSERT(0);
    return false;
}

// Note: p_data, p_min and p_max are _pointers_ to a memory address holding the data. For a slider, they are all required.
// Read code of e.g. SliderFloat(), SliderInt() etc. or examples in 'Demo.Widgets.Data Types' to understand how to use this function directly.
bool SliderScalar(label: *const c_char, ImGuiDataType data_type, *mut c_void p_data, *const c_void p_min, *const c_void p_max, format: *const c_char, ImGuiSliderFlags flags)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    const let mut style = &mut g.Style;
    let mut id: ImGuiID =  window.GetID(label);
    let w: c_float =  CalcItemWidth();

    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);
    let mut frame_bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + ImVec2::new(w, label_size.y + style.FramePadding.y * 2.00f32));
    let mut total_bb: ImRect = ImRect::new(frame_bb.Min, frame_bb.Max + ImVec2::new(label_size.x > 0f32 ? style.ItemInnerSpacing.x + label_size.x : 0f32, 0f32));

    let temp_input_allowed: bool = (flags & ImGuiSliderFlags_NoInput) == 0;
    ItemSize(total_bb, style.FramePadding.y);
    if (!ItemAdd(total_bb, id, &frame_bb, temp_input_allowed ? ImGuiItemFlags_Inputable : 0))
        return false;

    // Default format string when passing NULL
    if (format == null_mut())
        format = DataTypeGetInfo(data_type).PrintFmt;
    else if (data_type == ImGuiDataType_S32 && strcmp(format, "%d") != 0) // (FIXME-LEGACY: Patch old "%.0f" format string to use "%d", read function more details.)
        format = PatchFormatStringFloatToInt(format);

    let hovered: bool = ItemHoverable(frame_bb, id);
    let mut temp_input_is_active: bool =  temp_input_allowed && TempInputIsActive(id);
    if (!temp_input_is_active)
    {
        // Tabbing or CTRL-clicking on Slider turns it into an input box
        let input_requested_by_tabbing: bool = temp_input_allowed && (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_FocusedByTabbing) != 0;
        let clicked: bool = (hovered && g.IO.MouseClicked[0]);
        let make_active: bool = (input_requested_by_tabbing || clicked || g.NavActivateId == id || g.NavActivateInputId == id);
        if (make_active && temp_input_allowed)
            if (input_requested_by_tabbing || (clicked && g.IO.KeyCtrl) || g.NavActivateInputId == id)
                temp_input_is_active = true;

        if (make_active && !temp_input_is_active)
        {
            SetActiveID(id, window);
            SetFocusID(id, window);
            FocusWindow(window);
            g.ActiveIdUsingNavDirMask |= (1 << ImGuiDir_Left) | (1 << ImGuiDir_Right);
        }
    }

    if (temp_input_is_active)
    {
        // Only clamp CTRL+Click input when ImGuiSliderFlags_AlwaysClamp is set
        let is_clamp_input: bool = (flags & ImGuiSliderFlags_AlwaysClamp) != 0;
        return TempInputScalar(frame_bb, id, label, data_type, p_data, format, is_clamp_input ? p_min : null_mut(), is_clamp_input ? p_max : null_mut());
    }

    // Draw frame
    let frame_col: u32 = GetColorU32(g.ActiveId == id ? ImGuiCol_FrameBgActive : hovered ? ImGuiCol_FrameBgHovered : ImGuiCol_FrameBg);
    RenderNavHighlight(frame_bb, id);
    RenderFrame(frame_bb.Min, frame_bb.Max, frame_col, true, g.Style.FrameRounding);

    // Slider behavior
    let mut grab_bb: ImRect = ImRect::default();
    let value_changed: bool = SliderBehavior(frame_bb, id, data_type, p_data, p_min, p_max, format, flags, &grab_bb);
    if (value_changed)
        MarkItemEdited(id);

    // Render grab
    if (grab_bb.Max.x > grab_bb.Min.x)
        window.DrawList.AddRectFilled(grab_bb.Min, grab_bb.Max, GetColorU32(g.ActiveId == id ? ImGuiCol_SliderGrabActive : ImGuiCol_SliderGrab), style.GrabRounding);

    // Display value using user-provided display format so user can add prefix/suffix/decorations to the value.
    value_buf: [c_char;64];
    let mut  value_buf_end: *const c_char = value_buf + DataTypeFormatString(value_buf, IM_ARRAYSIZE(value_buf), data_type, p_data, format);
    if (g.LogEnabled)
        LogSetNextTextDecoration("{", "}");
    RenderTextClipped(frame_bb.Min, frame_bb.Max, value_buf, value_buf_end, null_mut(), ImVec2::new2(0.5f32, 0.5f32));

    if (label_size.x > 0f32)
        RenderText(ImVec2::new(frame_bb.Max.x + style.ItemInnerSpacing.x, frame_bb.Min.y + style.FramePadding.y), label);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags);
    return value_changed;
}

// Add multiple sliders on 1 line for compact edition of multiple components
bool SliderScalarN(label: *const c_char, ImGuiDataType data_type, *mut c_void v, components: c_int, *const c_void v_min, *const c_void v_max, format: *const c_char, ImGuiSliderFlags flags)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut value_changed: bool =  false;
    BeginGroup();
    PushID(label);
    PushMultiItemsWidths(components, CalcItemWidth());
    size_t type_size = GDataTypeInfo[data_type].Size;
    for (let i: c_int = 0; i < components; i++)
    {
        PushID(i);
        if (i > 0)
            SameLine(0, g.Style.ItemInnerSpacing.x);
        value_changed |= SliderScalar("", data_type, v, v_min, v_max, format, flags);
        PopID();
        PopItemWidth();
        v = ((*mut char)v + type_size);
    }
    PopID();

    let mut  label_end: *const c_char = FindRenderedTextEnd(label);
    if (label != label_end)
    {
        SameLine(0, g.Style.ItemInnerSpacing.x);
        TextEx(label, label_end);
    }

    EndGroup();
    return value_changed;
}

bool SliderFloat(label: *const c_char, *mut v: c_float, v_min: c_float, v_max: c_float, format: *const c_char, ImGuiSliderFlags flags)
{
    return SliderScalar(label, ImGuiDataType_Float, v, &v_min, &v_max, format, flags);
}

bool SliderFloat2(label: *const c_char, c_float v[2], v_min: c_float, v_max: c_float, format: *const c_char, ImGuiSliderFlags flags)
{
    return SliderScalarN(label, ImGuiDataType_Float, v, 2, &v_min, &v_max, format, flags);
}

bool SliderFloat3(label: *const c_char, c_float v[3], v_min: c_float, v_max: c_float, format: *const c_char, ImGuiSliderFlags flags)
{
    return SliderScalarN(label, ImGuiDataType_Float, v, 3, &v_min, &v_max, format, flags);
}

bool SliderFloat4(label: *const c_char, c_float v[4], v_min: c_float, v_max: c_float, format: *const c_char, ImGuiSliderFlags flags)
{
    return SliderScalarN(label, ImGuiDataType_Float, v, 4, &v_min, &v_max, format, flags);
}

bool SliderAngle(label: *const c_char, *mut v_rad: c_float, v_degrees_min: c_float, v_degrees_max: c_float, format: *const c_char, ImGuiSliderFlags flags)
{
    if (format == null_mut())
        format = "%.0f32 deg";
    let v_deg: c_float =  (*v_rad) * 360f32 / (2 * IM_PI);
    let mut value_changed: bool =  SliderFloat(label, &v_deg, v_degrees_min, v_degrees_max, format, flags);
    *v_rad = v_deg * (2 * IM_PI) / 360f32;
    return value_changed;
}

bool SliderInt(label: *const c_char, *mut v: c_int, v_min: c_int, v_max: c_int, format: *const c_char, ImGuiSliderFlags flags)
{
    return SliderScalar(label, ImGuiDataType_S32, v, &v_min, &v_max, format, flags);
}

bool SliderInt2(label: *const c_char, c_int v[2], v_min: c_int, v_max: c_int, format: *const c_char, ImGuiSliderFlags flags)
{
    return SliderScalarN(label, ImGuiDataType_S32, v, 2, &v_min, &v_max, format, flags);
}

bool SliderInt3(label: *const c_char, c_int v[3], v_min: c_int, v_max: c_int, format: *const c_char, ImGuiSliderFlags flags)
{
    return SliderScalarN(label, ImGuiDataType_S32, v, 3, &v_min, &v_max, format, flags);
}

bool SliderInt4(label: *const c_char, c_int v[4], v_min: c_int, v_max: c_int, format: *const c_char, ImGuiSliderFlags flags)
{
    return SliderScalarN(label, ImGuiDataType_S32, v, 4, &v_min, &v_max, format, flags);
}

bool VSliderScalar(label: *const c_char, const size: &ImVec2, ImGuiDataType data_type, *mut c_void p_data, *const c_void p_min, *const c_void p_max, format: *const c_char, ImGuiSliderFlags flags)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    const let mut style = &mut g.Style;
    let mut id: ImGuiID =  window.GetID(label);

    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);
    let mut frame_bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + size);
    let mut bb: ImRect = ImRect::new(frame_bb.Min, frame_bb.Max + ImVec2::new(label_size.x > 0f32 ? style.ItemInnerSpacing.x + label_size.x : 0f32, 0f32));

    ItemSize(bb, style.FramePadding.y);
    if (!ItemAdd(frame_bb, id))
        return false;

    // Default format string when passing NULL
    if (format == null_mut())
        format = DataTypeGetInfo(data_type).PrintFmt;
    else if (data_type == ImGuiDataType_S32 && strcmp(format, "%d") != 0) // (FIXME-LEGACY: Patch old "%.0f" format string to use "%d", read function more details.)
        format = PatchFormatStringFloatToInt(format);

    let hovered: bool = ItemHoverable(frame_bb, id);
    if ((hovered && g.IO.MouseClicked[0]) || g.NavActivateId == id || g.NavActivateInputId == id)
    {
        SetActiveID(id, window);
        SetFocusID(id, window);
        FocusWindow(window);
        g.ActiveIdUsingNavDirMask |= (1 << ImGuiDir_Up) | (1 << ImGuiDir_Down);
    }

    // Draw frame
    let frame_col: u32 = GetColorU32(g.ActiveId == id ? ImGuiCol_FrameBgActive : hovered ? ImGuiCol_FrameBgHovered : ImGuiCol_FrameBg);
    RenderNavHighlight(frame_bb, id);
    RenderFrame(frame_bb.Min, frame_bb.Max, frame_col, true, g.Style.FrameRounding);

    // Slider behavior
    let mut grab_bb: ImRect = ImRect::default();
    let value_changed: bool = SliderBehavior(frame_bb, id, data_type, p_data, p_min, p_max, format, flags | ImGuiSliderFlags_Vertical, &grab_bb);
    if (value_changed)
        MarkItemEdited(id);

    // Render grab
    if (grab_bb.Max.y > grab_bb.Min.y)
        window.DrawList.AddRectFilled(grab_bb.Min, grab_bb.Max, GetColorU32(g.ActiveId == id ? ImGuiCol_SliderGrabActive : ImGuiCol_SliderGrab), style.GrabRounding);

    // Display value using user-provided display format so user can add prefix/suffix/decorations to the value.
    // For the vertical slider we allow centered text to overlap the frame padding
    value_buf: [c_char;64];
    let mut  value_buf_end: *const c_char = value_buf + DataTypeFormatString(value_buf, IM_ARRAYSIZE(value_buf), data_type, p_data, format);
    RenderTextClipped(ImVec2::new(frame_bb.Min.x, frame_bb.Min.y + style.FramePadding.y), frame_bb.Max, value_buf, value_buf_end, null_mut(), ImVec2::new2(0.5f32, 0f32));
    if (label_size.x > 0f32)
        RenderText(ImVec2::new(frame_bb.Max.x + style.ItemInnerSpacing.x, frame_bb.Min.y + style.FramePadding.y), label);

    return value_changed;
}

bool VSliderFloat(label: *const c_char, const size: &ImVec2, *mut v: c_float, v_min: c_float, v_max: c_float, format: *const c_char, ImGuiSliderFlags flags)
{
    return VSliderScalar(label, size, ImGuiDataType_Float, v, &v_min, &v_max, format, flags);
}

bool VSliderInt(label: *const c_char, const size: &ImVec2, *mut v: c_int, v_min: c_int, v_max: c_int, format: *const c_char, ImGuiSliderFlags flags)
{
    return VSliderScalar(label, size, ImGuiDataType_S32, v, &v_min, &v_max, format, flags);
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: InputScalar, InputFloat, InputInt, etc.
//-------------------------------------------------------------------------
// - ImParseFormatFindStart() [Internal]
// - ImParseFormatFindEnd() [Internal]
// - ImParseFormatTrimDecorations() [Internal]
// - ImParseFormatSanitizeForPrinting() [Internal]
// - ImParseFormatSanitizeForScanning() [Internal]
// - ImParseFormatPrecision() [Internal]
// - TempInputTextScalar() [Internal]
// - InputScalar()
// - InputScalarN()
// - InputFloat()
// - InputFloat2()
// - InputFloat3()
// - InputFloat4()
// - InputInt()
// - InputInt2()
// - InputInt3()
// - InputInt4()
// - InputDouble()
//-------------------------------------------------------------------------

// We don't use strchr() because our strings are usually very short and often start with '%'
*const char ImParseFormatFindStart(fmt: *const c_char)
{
    while (char c = fmt[0])
    {
        if (c == '%' && fmt[1] != '%')
            return fmt;
        else if (c == '%')
            fmt+= 1;
        fmt+= 1;
    }
    return fmt;
}

*const char ImParseFormatFindEnd(fmt: *const c_char)
{
    // Printf/scanf types modifiers: I/L/h/j/l/t/w/z. Other uppercase letters qualify as types aka end of the format.
    if (fmt[0] != '%')
        return fmt;
    let mut ignored_uppercase_mask: c_uint =  (1 << ('I'-'A')) | (1 << ('L'-'A'));
    let mut ignored_lowercase_mask: c_uint =  (1 << ('h'-'a')) | (1 << ('j'-'a')) | (1 << ('l'-'a')) | (1 << ('t'-'a')) | (1 << ('w'-'a')) | (1 << ('z'-'a'));
    for (char c; (c = *fmt) != 0; fmt++)
    {
        if (c >= 'A' && c <= 'Z' && ((1 << (c - 'A')) & ignored_uppercase_mask) == 0)
            return fmt + 1;
        if (c >= 'a' && c <= 'z' && ((1 << (c - 'a')) & ignored_lowercase_mask) == 0)
            return fmt + 1;
    }
    return fmt;
}

// Extract the format out of a format string with leading or trailing decorations
//  fmt = "blah blah"  -> return fmt
//  fmt = "%.3f"       -> return fmt
//  fmt = "hello %.3f" -> return fmt + 6
//  fmt = "%.3f hello" -> return buf written with "%.3f"
*const char ImParseFormatTrimDecorations(fmt: *const c_char, *mut char buf, size_t buf_size)
{
    let mut  fmt_start: *const c_char = ImParseFormatFindStart(fmt);
    if (fmt_start[0] != '%')
        return fmt;
    let mut  fmt_end: *const c_char = ImParseFormatFindEnd(fmt_start);
    if (fmt_end[0] == 0) // If we only have leading decoration, we don't need to copy the data.
        return fmt_start;
    ImStrncpy(buf, fmt_start, ImMin((fmt_end - fmt_start) + 1, buf_size));
    return buf;
}

// Sanitize format
// - Zero terminate so extra characters after format (e.g. "%f123") don't confuse atof/atoi
// - stb_sprintf.h supports several new modifiers which format numbers in a way that also makes them incompatible atof/atoi.
c_void ImParseFormatSanitizeForPrinting(fmt_in: *const c_char, *mut char fmt_out, size_t fmt_out_size)
{
    let mut  fmt_end: *const c_char = ImParseFormatFindEnd(fmt_in);
    IM_UNUSED(fmt_out_size);
    // IM_ASSERT((fmt_end - fmt_in + 1) < fmt_out_size); // Format is too long, let us know if this happens to you!
    while (fmt_in < fmt_end)
    {
        char c = *fmt_in+= 1;
        if (c != '\'' && c != '$' && c != '_') // Custom flags provided by stb_sprintf.h. POSIX 2008 also supports '.
            *(fmt_out++) = c;
    }
    *fmt_out = 0; // Zero-terminate
}

// - For scanning we need to remove all width and precision fields "%3.7f" -> "%f". BUT don't strip types like "%I64d" which includes digits. ! "%07I64d" -> "%I64d"
*const char ImParseFormatSanitizeForScanning(fmt_in: *const c_char, *mut char fmt_out, size_t fmt_out_size)
{
    let mut  fmt_end: *const c_char = ImParseFormatFindEnd(fmt_in);
    let mut  fmt_out_begin: *const c_char = fmt_out;
    IM_UNUSED(fmt_out_size);
    // IM_ASSERT((fmt_end - fmt_in + 1) < fmt_out_size); // Format is too long, let us know if this happens to you!
    let mut has_type: bool =  false;
    while (fmt_in < fmt_end)
    {
        char c = *fmt_in+= 1;
        if (!has_type && ((c >= '0' && c <= '9') || c == '.'))
            continue;
        has_type |= ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')); // Stop skipping digits
        if (c != '\'' && c != '$' && c != '_') // Custom flags provided by stb_sprintf.h. POSIX 2008 also supports '.
            *(fmt_out++) = c;
    }
    *fmt_out = 0; // Zero-terminate
    return fmt_out_begin;
}

template<typename TYPE>
static *const char ImAtoi(src: *const c_char, *mut TYPE output)
{
    let negative: c_int = 0;
    if (*src == '-') { negative = 1; src+= 1; }
    if (*src == '+') { src+= 1; }
    TYPE v = 0;
    while (*src >= '0' && *src <= '9')
        v = (v * 10) + (*src++ - '0');
    *output = negative ? -v : v;
    return src;
}

// Parse display precision back from the display format string
// FIXME: This is still used by some navigation code path to infer a minimum tweak step, but we should aim to rework widgets so it isn't needed.
c_int ImParseFormatPrecision(fmt: *const c_char, default_precision: c_int)
{
    fmt = ImParseFormatFindStart(fmt);
    if (fmt[0] != '%')
        return default_precision;
    fmt+= 1;
    while (*fmt >= '0' && *fmt <= '9')
        fmt+= 1;
    let precision: c_int = INT_MAX;
    if (*fmt == '.')
    {
        fmt = ImAtoi<c_int>(fmt + 1, &precision);
        if (precision < 0 || precision > 99)
            precision = default_precision;
    }
    if (*fmt == 'e' || *fmt == 'E') // Maximum precision with scientific notation
        precision = -1;
    if ((*fmt == 'g' || *fmt == 'G') && precision == INT_MAX)
        precision = -1;
    return (precision == INT_MAX) ? default_precision : precision;
}

// Create text input in place of another active widget (e.g. used when doing a CTRL+Click on drag/slider widgets)
// FIXME: Facilitate using this in variety of other situations.
bool TempInputText(bb: &ImRect, id: ImGuiID, label: *const c_char, *mut char buf, buf_size: c_int, ImGuiInputTextFlags flags)
{
    // On the first frame, g.TempInputTextId == 0, then on subsequent frames it becomes == id.
    // We clear ActiveID on the first frame to allow the InputText() taking it back.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let init: bool = (g.TempInputId != id);
    if (init)
        ClearActiveID();

    g.Currentwindow.DC.CursorPos = bb.Min;
    let mut value_changed: bool =  InputTextEx(label, null_mut(), buf, buf_size, bb.GetSize(), flags | ImGuiInputTextFlags_MergedItem);
    if (init)
    {
        // First frame we started displaying the InputText widget, we expect it to take the active id.
        // IM_ASSERT(g.ActiveId == id);
        g.TempInputId = g.ActiveId;
    }
    return value_changed;
}

static inline ImGuiInputTextFlags InputScalar_DefaultCharsFilter(ImGuiDataType data_type, format: *const c_char)
{
    if (data_type == ImGuiDataType_Float || data_type == ImGuiDataType_Double)
        return ImGuiInputTextFlags_CharsScientific;
    const char format_last_char = format[0] ? format[strlen(format) - 1] : 0;
    return (format_last_char == 'x' || format_last_char == 'X') ? ImGuiInputTextFlags_CharsHexadecimal : ImGuiInputTextFlags_CharsDecimal;
}

// Note that Drag/Slider functions are only forwarding the min/max values clamping values if the ImGuiSliderFlags_AlwaysClamp flag is set!
// This is intended: this way we allow CTRL+Click manual input to set a value out of bounds, for maximum flexibility.
// However this may not be ideal for all uses, as some user code may break on out of bound values.
bool TempInputScalar(bb: &ImRect, id: ImGuiID, label: *const c_char, ImGuiDataType data_type, *mut c_void p_data, format: *const c_char, *const c_void p_clamp_min, *const c_void p_clamp_max)
{
    fmt_buf: [c_char;32];
    data_buf: [c_char;32];
    format = ImParseFormatTrimDecorations(format, fmt_buf, IM_ARRAYSIZE(fmt_buf));
    DataTypeFormatString(data_buf, IM_ARRAYSIZE(data_buf), data_type, p_data, format);
    ImStrTrimBlanks(data_buf);

    ImGuiInputTextFlags flags = ImGuiInputTextFlags_AutoSelectAll | ImGuiInputTextFlags_NoMarkEdited;
    flags |= InputScalar_DefaultCharsFilter(data_type, format);

    let mut value_changed: bool =  false;
    if (TempInputText(bb, id, label, data_buf, IM_ARRAYSIZE(data_buf), flags))
    {
        // Backup old value
        size_t data_type_size = DataTypeGetInfo(data_type).Size;
        ImGuiDataTypeTempStorage data_backup;
        memcpy(&data_backup, p_data, data_type_size);

        // Apply new value (or operations) then clamp
        DataTypeApplyFromText(data_buf, data_type, p_data, format);
        if (p_clamp_min || p_clamp_max)
        {
            if (p_clamp_min && p_clamp_max && DataTypeCompare(data_type, p_clamp_min, p_clamp_max) > 0)
                ImSwap(p_clamp_min, p_clamp_max);
            DataTypeClamp(data_type, p_data, p_clamp_min, p_clamp_max);
        }

        // Only mark as edited if new value is different
        value_changed = memcmp(&data_backup, p_data, data_type_size) != 0;
        if (value_changed)
            MarkItemEdited(id);
    }
    return value_changed;
}

// Note: p_data, p_step, p_step_fast are _pointers_ to a memory address holding the data. For an Input widget, p_step and p_step_fast are optional.
// Read code of e.g. InputFloat(), InputInt() etc. or examples in 'Demo.Widgets.Data Types' to understand how to use this function directly.
bool InputScalar(label: *const c_char, ImGuiDataType data_type, *mut c_void p_data, *const c_void p_step, *const c_void p_step_fast, format: *const c_char, ImGuiInputTextFlags flags)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut style = &mut g.Style;

    if (format == null_mut())
        format = DataTypeGetInfo(data_type).PrintFmt;

    buf: [c_char;64];
    DataTypeFormatString(buf, IM_ARRAYSIZE(buf), data_type, p_data, format);

    // Testing ActiveId as a minor optimization as filtering is not needed until active
    if (g.ActiveId == 0 && (flags & (ImGuiInputTextFlags_CharsDecimal | ImGuiInputTextFlags_CharsHexadecimal | ImGuiInputTextFlags_CharsScientific)) == 0)
        flags |= InputScalar_DefaultCharsFilter(data_type, format);
    flags |= ImGuiInputTextFlags_AutoSelectAll | ImGuiInputTextFlags_NoMarkEdited; // We call MarkItemEdited() ourselves by comparing the actual data rather than the string.

    let mut value_changed: bool =  false;
    if (p_step != null_mut())
    {
        let button_size: c_float =  GetFrameHeight();

        BeginGroup(); // The only purpose of the group here is to allow the caller to query item data e.g. IsItemActive()
        PushID(label);
        SetNextItemWidth(ImMax(1f32, CalcItemWidth() - (button_size + style.ItemInnerSpacing.x) * 2));
        if (InputText("", buf, IM_ARRAYSIZE(buf), flags)) // PushId(label) + "" gives us the expected ID from outside point of view
            value_changed = DataTypeApplyFromText(buf, data_type, p_data, format);
        IMGUI_TEST_ENGINE_ITEM_INFO(g.LastItemData.ID, label, g.LastItemData.StatusFlags);

        // Step buttons
        let backup_frame_padding: ImVec2 = style.FramePadding;
        style.FramePadding.x = style.FramePadding.y;
        ImGuiButtonFlags button_flags = ImGuiButtonFlags_Repeat | ImGuiButtonFlags_DontClosePopups;
        if (flags & ImGuiInputTextFlags_ReadOnly)
            BeginDisabled();
        SameLine(0, style.ItemInnerSpacing.x);
        if (ButtonEx("-", ImVec2::new(button_size, button_size), button_flags))
        {
            DataTypeApplyOp(data_type, '-', p_data, p_data, g.IO.KeyCtrl && p_step_fast ? p_step_fast : p_step);
            value_changed = true;
        }
        SameLine(0, style.ItemInnerSpacing.x);
        if (ButtonEx("+", ImVec2::new(button_size, button_size), button_flags))
        {
            DataTypeApplyOp(data_type, '+', p_data, p_data, g.IO.KeyCtrl && p_step_fast ? p_step_fast : p_step);
            value_changed = true;
        }
        if (flags & ImGuiInputTextFlags_ReadOnly)
            EndDisabled();

        let mut  label_end: *const c_char = FindRenderedTextEnd(label);
        if (label != label_end)
        {
            SameLine(0, style.ItemInnerSpacing.x);
            TextEx(label, label_end);
        }
        style.FramePadding = backup_frame_padding;

        PopID();
        EndGroup();
    }
    else
    {
        if (InputText(label, buf, IM_ARRAYSIZE(buf), flags))
            value_changed = DataTypeApplyFromText(buf, data_type, p_data, format);
    }
    if (value_changed)
        MarkItemEdited(g.LastItemData.ID);

    return value_changed;
}

bool InputScalarN(label: *const c_char, ImGuiDataType data_type, *mut c_void p_data, components: c_int, *const c_void p_step, *const c_void p_step_fast, format: *const c_char, ImGuiInputTextFlags flags)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut value_changed: bool =  false;
    BeginGroup();
    PushID(label);
    PushMultiItemsWidths(components, CalcItemWidth());
    size_t type_size = GDataTypeInfo[data_type].Size;
    for (let i: c_int = 0; i < components; i++)
    {
        PushID(i);
        if (i > 0)
            SameLine(0, g.Style.ItemInnerSpacing.x);
        value_changed |= InputScalar("", data_type, p_data, p_step, p_step_fast, format, flags);
        PopID();
        PopItemWidth();
        p_data = ((*mut char)p_data + type_size);
    }
    PopID();

    let mut  label_end: *const c_char = FindRenderedTextEnd(label);
    if (label != label_end)
    {
        SameLine(0f32, g.Style.ItemInnerSpacing.x);
        TextEx(label, label_end);
    }

    EndGroup();
    return value_changed;
}

bool InputFloat(label: *const c_char, *mut v: c_float, step: c_float, step_fast: c_float, format: *const c_char, ImGuiInputTextFlags flags)
{
    flags |= ImGuiInputTextFlags_CharsScientific;
    return InputScalar(label, ImGuiDataType_Float, v, (step > 0f32 ? &step : null_mut()), (step_fast > 0f32 ? &step_fast : null_mut()), format, flags);
}

bool InputFloat2(label: *const c_char, c_float v[2], format: *const c_char, ImGuiInputTextFlags flags)
{
    return InputScalarN(label, ImGuiDataType_Float, v, 2, null_mut(), null_mut(), format, flags);
}

bool InputFloat3(label: *const c_char, c_float v[3], format: *const c_char, ImGuiInputTextFlags flags)
{
    return InputScalarN(label, ImGuiDataType_Float, v, 3, null_mut(), null_mut(), format, flags);
}

bool InputFloat4(label: *const c_char, c_float v[4], format: *const c_char, ImGuiInputTextFlags flags)
{
    return InputScalarN(label, ImGuiDataType_Float, v, 4, null_mut(), null_mut(), format, flags);
}

bool InputInt(label: *const c_char, *mut v: c_int, step: c_int, step_fast: c_int, ImGuiInputTextFlags flags)
{
    // Hexadecimal input provided as a convenience but the flag name is awkward. Typically you'd use InputText() to parse your own data, if you want to handle prefixes.
    let mut  format: *const c_char = (flags & ImGuiInputTextFlags_CharsHexadecimal) ? "%08X" : "%d";
    return InputScalar(label, ImGuiDataType_S32, v, (step > 0 ? &step : null_mut()), (step_fast > 0 ? &step_fast : null_mut()), format, flags);
}

bool InputInt2(label: *const c_char, c_int v[2], ImGuiInputTextFlags flags)
{
    return InputScalarN(label, ImGuiDataType_S32, v, 2, null_mut(), null_mut(), "%d", flags);
}

bool InputInt3(label: *const c_char, c_int v[3], ImGuiInputTextFlags flags)
{
    return InputScalarN(label, ImGuiDataType_S32, v, 3, null_mut(), null_mut(), "%d", flags);
}

bool InputInt4(label: *const c_char, c_int v[4], ImGuiInputTextFlags flags)
{
    return InputScalarN(label, ImGuiDataType_S32, v, 4, null_mut(), null_mut(), "%d", flags);
}

bool InputDouble(label: *const c_char, *mut double v, double step, double step_fast, format: *const c_char, ImGuiInputTextFlags flags)
{
    flags |= ImGuiInputTextFlags_CharsScientific;
    return InputScalar(label, ImGuiDataType_Double, v, (step > 0.0 ? &step : null_mut()), (step_fast > 0.0 ? &step_fast : null_mut()), format, flags);
}

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

bool InputText(label: *const c_char, *mut char buf, size_t buf_size, ImGuiInputTextFlags flags, ImGuiInputTextCallback callback, *mut c_void user_data)
{
    // IM_ASSERT(!(flags & ImGuiInputTextFlags_Multiline)); // call InputTextMultiline()
    return InputTextEx(label, null_mut(), buf, buf_size, ImVec2::new2(0, 0), flags, callback, user_data);
}

bool InputTextMultiline(label: *const c_char, *mut char buf, size_t buf_size, const size: &ImVec2, ImGuiInputTextFlags flags, ImGuiInputTextCallback callback, *mut c_void user_data)
{
    return InputTextEx(label, null_mut(), buf, buf_size, size, flags | ImGuiInputTextFlags_Multiline, callback, user_data);
}

bool InputTextWithHint(label: *const c_char, hint: *const c_char, *mut char buf, size_t buf_size, ImGuiInputTextFlags flags, ImGuiInputTextCallback callback, *mut c_void user_data)
{
    // IM_ASSERT(!(flags & ImGuiInputTextFlags_Multiline)); // call InputTextMultiline() or  InputTextEx() manually if you need multi-line + hint.
    return InputTextEx(label, hint, buf, buf_size, ImVec2::new2(0, 0), flags, callback, user_data);
}

static c_int InputTextCalcTextLenAndLineCount(text_begin: *const c_char, *const *mut char out_text_end)
{
    let line_count: c_int = 0;
    let mut  s: *const c_char = text_begin;
    while (char c = *s++) // We are only matching for \n so we can ignore UTF-8 decoding
        if (c == '\n')
            line_count+= 1;
    s-= 1;
    if (s[0] != '\n' && s[0] != '\r')
        line_count+= 1;
    *out_text_end = s;
    return line_count;
}

static ImVec2 InputTextCalcTextSizeW(*const ImWchar text_begin, *const ImWchar text_end, *const *mut ImWchar remaining, *mut out_offset: ImVec2, stop_on_new_line: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImFont font = g.Font;
    let line_height: c_float =  g.FontSize;
    let scale: c_float =  line_height / ;

    let text_size: ImVec2 = ImVec2::new2(0, 0);
    let line_width: c_float =  0f32;

    let s: *const ImWchar = text_begin;
    while (s < text_end)
    {
        let mut c: c_uint =  (*s++);
        if (c == '\n')
        {
            text_size.x = ImMax(text_size.x, line_width);
            text_size.y += line_height;
            line_width = 0f32;
            if (stop_on_new_line)
                break;
            continue;
        }
        if (c == '\r')
            continue;

        let char_width: c_float =  (c) * scale;
        line_width += char_width;
    }

    if (text_size.x < line_width)
        text_size.x = line_width;

    if (out_offset)
        *out_offset = ImVec2::new(line_width, text_size.y + line_height);  // offset allow for the possibility of sitting after a trailing \n

    if (line_width > 0 || text_size.y == 0f32)                        // whereas size.y will ignore the trailing \n
        text_size.y += line_height;

    if (remaining)
        *remaining = s;

    return text_size;
}

// Wrapper for stb_textedit.h to edit text (our wrapper is for: statically sized buffer, single-line, wchar characters. InputText converts between UTF-8 and wchar)
namespace ImStb
{

static c_int     STB_TEXTEDIT_STRINGLEN(*const ImGuiInputTextState obj)                             { return ; }
static ImWchar STB_TEXTEDIT_GETCHAR(*const ImGuiInputTextState obj, idx: c_int)                      { return obj.TextW[idx]; }
static c_float   STB_TEXTEDIT_GETWIDTH(*mut ImGuiInputTextState obj, line_start_idx: c_int, char_idx: c_int)  { let c: ImWchar = obj.TextW[line_start_idx + char_idx]; if (c == '\n') return STB_TEXTEDIT_GETWIDTH_NEWLINE; let g = GImGui; // ImGuiContext& g = *GImGui; return g.Font.GetCharAdvance(c) * (g.FontSize / g.Font.FontSize); }
static c_int     STB_TEXTEDIT_KEYTOTEXT(key: c_int)                                                    { return key >= 0x200000 ? 0 : key; }
static let STB_TEXTEDIT_NEWLINE: ImWchar = '\n';
static c_void    STB_TEXTEDIT_LAYOUTROW(*mut StbTexteditRow r, *mut ImGuiInputTextState obj, line_start_idx: c_int)
{
    let text: *const ImWchar = obj.TextW.Data;
    let text_remaining: *const ImWchar= null_mut();
    let size: ImVec2 = InputTextCalcTextSizeW(text + line_start_idx, text + , &text_remaining, null_mut(), true);
     = 0f32;
     = size.x;
     = size.y;
     = 0f32;
     = size.y;
     = (text_remaining - (text + line_start_idx));
}

// When ImGuiInputTextFlags_Password is set, we don't want actions such as CTRL+Arrow to leak the fact that underlying data are blanks or separators.
static bool is_separator(c_uint c)                                        { return ImCharIsBlankW(c) || c==',' || c==';' || c=='(' || c==')' || c=='{' || c=='}' || c=='[' || c==']' || c=='|' || c=='\n' || c=='\r'; }
static c_int  is_word_boundary_from_right(*mut ImGuiInputTextState obj, idx: c_int)      { if ( & ImGuiInputTextFlags_Password) return 0; return idx > 0 ? (is_separator(obj.TextW[idx - 1]) && !is_separator(obj.TextW[idx]) ) : 1; }
static c_int  is_word_boundary_from_left(*mut ImGuiInputTextState obj, idx: c_int)       { if ( & ImGuiInputTextFlags_Password) return 0; return idx > 0 ? (!is_separator(obj.TextW[idx - 1]) && is_separator(obj.TextW[idx])) : 1; }
static c_int  STB_TEXTEDIT_MOVEWORDLEFT_IMPL(*mut ImGuiInputTextState obj, idx: c_int)   { idx-= 1; while (idx >= 0 && !is_word_boundary_from_right(obj, idx)) idx-= 1; return idx < 0 ? 0 : idx; }
static c_int  STB_TEXTEDIT_MOVEWORDRIGHT_MAC(*mut ImGuiInputTextState obj, idx: c_int)   { idx+= 1; let len: c_int = ; while (idx < len && !is_word_boundary_from_left(obj, idx)) idx+= 1; return idx > len ? len : idx; }
static c_int  STB_TEXTEDIT_MOVEWORDRIGHT_WIN(*mut ImGuiInputTextState obj, idx: c_int)   { idx+= 1; let len: c_int = ; while (idx < len && !is_word_boundary_from_right(obj, idx)) idx+= 1; return idx > len ? len : idx; }
static c_int  STB_TEXTEDIT_MOVEWORDRIGHT_IMPL(*mut ImGuiInputTextState obj, idx: c_int)  { if (GetIO().ConfigMacOSXBehaviors) return STB_TEXTEDIT_MOVEWORDRIGHT_MAC(obj, idx); else return STB_TEXTEDIT_MOVEWORDRIGHT_WIN(obj, idx); }
// #define STB_TEXTEDIT_MOVEWORDLEFT   STB_TEXTEDIT_MOVEWORDLEFT_IMPL  // They need to be #define for stb_textedit.h
// #define STB_TEXTEDIT_MOVEWORDRIGHT  STB_TEXTEDIT_MOVEWORDRIGHT_IMPL

static c_void STB_TEXTEDIT_DELETECHARS(*mut ImGuiInputTextState obj, pos: c_int, n: c_int)
{
    *mut let dst: ImWchar = obj.TextW.Data + pos;

    // We maintain our buffer length in both UTF-8 and wchar formats
     = true;
     -= ImTextCountUtf8BytesFromStr(dst, dst + n);
     -= n;

    // Offset remaining text (FIXME-OPT: Use memmove)
    let src: *const ImWchar = obj.TextW.Data + pos + n;
    while (let c: ImWchar = *src++)
        *dst++ = c;
    *dst = '\0';
}

static bool STB_TEXTEDIT_INSERTCHARS(*mut ImGuiInputTextState obj, pos: c_int, *const ImWchar new_text, new_text_len: c_int)
{
    let is_resizable: bool = ( & ImGuiInputTextFlags_CallbackResize) != 0;
    let text_len: c_int = ;
    // IM_ASSERT(pos <= text_len);

    let new_text_len_utf8: c_int = ImTextCountUtf8BytesFromStr(new_text, new_text + new_text_len);
    if (!is_resizable && (new_text_len_utf8 +  + 1 > ))
        return false;

    // Grow internal buffer if needed
    if (new_text_len + text_len + 1 > obj.TextW.Size)
    {
        if (!is_resizable)
            return false;
        // IM_ASSERT(text_len < obj.TextW.Size);
        obj.TextW.resize(text_len + ImClamp(new_text_len * 4, 32, ImMax(256, new_text_len)) + 1);
    }

    *mut let text: ImWchar = obj.TextW.Data;
    if (pos != text_len)
        memmove(text + pos + new_text_len, text + pos, (text_len - pos) * sizeof);
    memcpy(text + pos, new_text, new_text_len * sizeof);

     = true;
     += new_text_len;
     += new_text_len_utf8;
    obj.TextW[] = '\0';

    return true;
}

// We don't use an enum so we can build even with conflicting symbols (if another user of stb_textedit.h leak their STB_TEXTEDIT_K_* symbols)
// #define STB_TEXTEDIT_K_LEFT         0x200000 // keyboard input to move cursor left
// #define STB_TEXTEDIT_K_RIGHT        0x200001 // keyboard input to move cursor right
// #define STB_TEXTEDIT_K_UP           0x200002 // keyboard input to move cursor up
// #define STB_TEXTEDIT_K_DOWN         0x200003 // keyboard input to move cursor down
// #define STB_TEXTEDIT_K_LINESTART    0x200004 // keyboard input to move cursor to start of line
// #define STB_TEXTEDIT_K_LINEEND      0x200005 // keyboard input to move cursor to end of line
// #define STB_TEXTEDIT_K_TEXTSTART    0x200006 // keyboard input to move cursor to start of text
// #define STB_TEXTEDIT_K_TEXTEND      0x200007 // keyboard input to move cursor to end of text
// #define STB_TEXTEDIT_K_DELETE       0x200008 // keyboard input to delete selection or character under cursor
// #define STB_TEXTEDIT_K_BACKSPACE    0x200009 // keyboard input to delete selection or character left of cursor
// #define STB_TEXTEDIT_K_UNDO         0x20000A // keyboard input to perform undo
// #define STB_TEXTEDIT_K_REDO         0x20000B // keyboard input to perform redo
// #define STB_TEXTEDIT_K_WORDLEFT     0x20000C // keyboard input to move cursor left one word
// #define STB_TEXTEDIT_K_WORDRIGHT    0x20000D // keyboard input to move cursor right one word
// #define STB_TEXTEDIT_K_PGUP         0x20000E // keyboard input to move cursor up a page
// #define STB_TEXTEDIT_K_PGDOWN       0x20000f32 // keyboard input to move cursor down a page
// #define STB_TEXTEDIT_K_SHIFT        0x400000

// #define STB_TEXTEDIT_IMPLEMENTATION
// #include "imstb_textedit.h"

// stb_textedit internally allows for a single undo record to do addition and deletion, but somehow, calling
// the stb_textedit_paste() function creates two separate records, so we perform it manually. (FIXME: Report to nothings/stb?)
static c_void stb_textedit_replace(*mut ImGuiInputTextState str, *mut STB_TexteditState state, *const STB_TEXTEDIT_CHARTYPE text, text_len: c_int)
{
    stb_text_makeundo_replace(str, state, 0, , text_len);
    ImStb::STB_TEXTEDIT_DELETECHARS(str, 0, );
    if (text_len <= 0)
        return;
    if (ImStb::STB_TEXTEDIT_INSERTCHARS(str, 0, text, text_len))
    {
        state.cursor = text_len;
        state.has_preferred_x = 0;
        return;
    }
    // IM_ASSERT(0); // Failed to insert character, normally shouldn't happen because of how we currently use stb_textedit_replace()
}

} // namespace ImStb

c_void ImGuiInputTextState::OnKeyPressed(key: c_int)
{
    stb_textedit_key(this, &Stb, key);
    CursorFollow = true;
    CursorAnimReset();
}

ImGuiInputTextCallbackData::ImGuiInputTextCallbackData()
{
    memset(this, 0, sizeof(*this));
}

// Public API to manipulate UTF-8 text
// We expose UTF-8 to the user (unlike the STB_TEXTEDIT_* functions which are manipulating wchar)
// FIXME: The existence of this rarely exercised code path is a bit of a nuisance.
c_void ImGuiInputTextCallbackData::DeleteChars(pos: c_int, bytes_count: c_int)
{
    // IM_ASSERT(pos + bytes_count <= BufTextLen);
    *mut char dst = Buf + pos;
    let mut  src: *const c_char = Buf + pos + bytes_count;
    while (char c = *src++)
        *dst++ = c;
    *dst = '\0';

    if (CursorPos >= pos + bytes_count)
        CursorPos -= bytes_count;
    else if (CursorPos >= pos)
        CursorPos = pos;
    SelectionStart = SelectionEnd = CursorPos;
    BufDirty = true;
    BufTextLen -= bytes_count;
}

c_void ImGuiInputTextCallbackData::InsertChars(pos: c_int, new_text: *const c_char, new_text_end: *const c_char)
{
    let is_resizable: bool = (Flags & ImGuiInputTextFlags_CallbackResize) != 0;
    let new_text_len: c_int = new_text_end ? (new_text_end - new_text) : strlen(new_text);
    if (new_text_len + BufTextLen >= BufSize)
    {
        if (!is_resizable)
            return;

        // Contrary to STB_TEXTEDIT_INSERTCHARS() this is working in the UTF8 buffer, hence the mildly similar code (until we remove the U16 buffer altogether!)
        let g = GImGui; // ImGuiContext& g = *GImGui;
        *mut ImGuiInputTextState edit_state = &g.InputTextState;
        // IM_ASSERT(edit_state.ID != 0 && g.ActiveId == edit_state.ID);
        // IM_ASSERT(Buf == edit_state.TextA.Data);
        let new_buf_size: c_int = BufTextLen + ImClamp(new_text_len * 4, 32, ImMax(256, new_text_len)) + 1;
        edit_state.TextA.reserve(new_buf_size + 1);
        Buf = edit_state.TextA.Data;
        BufSize = edit_state.BufCapacityA = new_buf_size;
    }

    if (BufTextLen != pos)
        memmove(Buf + pos + new_text_len, Buf + pos, (BufTextLen - pos));
    memcpy(Buf + pos, new_text, new_text_len * sizeof);
    Buf[BufTextLen + new_text_len] = '\0';

    if (CursorPos >= pos)
        CursorPos += new_text_len;
    SelectionStart = SelectionEnd = CursorPos;
    BufDirty = true;
    BufTextLen += new_text_len;
}

// Return false to discard a character.
static bool InputTextFilterCharacter(*mut c_uint p_char, ImGuiInputTextFlags flags, ImGuiInputTextCallback callback, *mut c_void user_data, ImGuiInputSource input_source)
{
    // IM_ASSERT(input_source == ImGuiInputSource_Keyboard || input_source == ImGuiInputSource_Clipboard);
    let mut c: c_uint =  *p_char;

    // Filter non-printable (NB: isprint is unreliable! see #2467)
    let mut apply_named_filters: bool =  true;
    if (c < 0x20)
    {
        let mut pass: bool =  false;
        pass |= (c == '\n' && (flags & ImGuiInputTextFlags_Multiline)); // Note that an Enter KEY will emit \r and be ignored (we poll for KEY in InputText() code)
        pass |= (c == '\t' && (flags & ImGuiInputTextFlags_AllowTabInput));
        if (!pass)
            return false;
        apply_named_filters = false; // Override named filters below so newline and tabs can still be inserted.
    }

    if (input_source != ImGuiInputSource_Clipboard)
    {
        // We ignore Ascii representation of delete (emitted from Backspace on OSX, see #2578, #2817)
        if (c == 127)
            return false;

        // Filter private Unicode range. GLFW on OSX seems to send private characters for special keys like arrow keys (FIXME)
        if (c >= 0xE000 && c <= 0xF8F0f32)
            return false;
    }

    // Filter Unicode ranges we are not handling in this build
    if (c > IM_UNICODE_CODEPOINT_MAX)
        return false;

    // Generic named filters
    if (apply_named_filters && (flags & (ImGuiInputTextFlags_CharsDecimal | ImGuiInputTextFlags_CharsHexadecimal | ImGuiInputTextFlags_CharsUppercase | ImGuiInputTextFlags_CharsNoBlank | ImGuiInputTextFlags_CharsScientific)))
    {
        // The libc allows overriding locale, with e.g. 'setlocale(LC_NUMERIC, "de_DE.UTF-8");' which affect the output/input of printf/scanf to use e.g. ',' instead of '.'.
        // The standard mandate that programs starts in the "C" locale where the decimal point is '.'.
        // We don't really intend to provide widespread support for it, but out of empathy for people stuck with using odd API, we support the bare minimum aka overriding the decimal point.
        // Change the default decimal_point with:
        //   GetCurrentContext().PlatformLocaleDecimalPoint = *localeconv().decimal_point;
        // Users of non-default decimal point (in particular ',') may be affected by word-selection logic (is_word_boundary_from_right/is_word_boundary_from_left) functions.
        let g = GImGui; // ImGuiContext& g = *GImGui;
        const unsigned c_decimal_point = g.PlatformLocaleDecimalPoint;

        // Full-width -> half-width conversion for numeric fields (https://en.wikipedia.org/wiki/Halfwidth_and_Fullwidth_Forms_(Unicode_block)
        // While this is mostly convenient, this has the side-effect for uninformed users accidentally inputting full-width characters that they may
        // scratch their head as to why it works in numerical fields vs in generic text fields it would require support in the font.
        if (flags & (ImGuiInputTextFlags_CharsDecimal | ImGuiInputTextFlags_CharsScientific | ImGuiInputTextFlags_CharsHexadecimal))
            if (c >= 0xFF01 && c <= 0xFF5E)
                c = c - 0xFF01 + 0x21;

        // Allow 0-9 . - + * /
        if (flags & ImGuiInputTextFlags_CharsDecimal)
            if (!(c >= '0' && c <= '9') && (c != c_decimal_point) && (c != '-') && (c != '+') && (c != '*') && (c != '/'))
                return false;

        // Allow 0-9 . - + * / e E
        if (flags & ImGuiInputTextFlags_CharsScientific)
            if (!(c >= '0' && c <= '9') && (c != c_decimal_point) && (c != '-') && (c != '+') && (c != '*') && (c != '/') && (c != 'e') && (c != 'E'))
                return false;

        // Allow 0-9 a-F A-F
        if (flags & ImGuiInputTextFlags_CharsHexadecimal)
            if (!(c >= '0' && c <= '9') && !(c >= 'a' && c <= 'f') && !(c >= 'A' && c <= 'F'))
                return false;

        // Turn a-z into A-Z
        if (flags & ImGuiInputTextFlags_CharsUppercase)
            if (c >= 'a' && c <= 'z')
                c += ('A' - 'a');

        if (flags & ImGuiInputTextFlags_CharsNoBlank)
            if (ImCharIsBlankW(c))
                return false;

        *p_char = c;
    }

    // Custom callback filter
    if (flags & ImGuiInputTextFlags_CallbackCharFilter)
    {
        ImGuiInputTextCallbackData callback_data;
        memset(&callback_data, 0, sizeof(ImGuiInputTextCallbackData));
        callback_data.EventFlag = ImGuiInputTextFlags_CallbackCharFilter;
        callback_data.EventChar = c;
        callback_data.Flags = flags;
        callback_data.UserData = user_data;
        if (callback(&callback_data) != 0)
            return false;
        *p_char = callback_data.EventChar;
        if (!callback_data.EventChar)
            return false;
    }

    return true;
}

// Find the shortest single replacement we can make to get the new text from the old text.
// Important: needs to be run before TextW is rewritten with the new characters because calling STB_TEXTEDIT_GETCHAR() at the end.
// FIXME: Ideally we should transition toward (1) making InsertChars()/DeleteChars() update undo-stack (2) discourage (and keep reconcile) or obsolete (and remove reconcile) accessing buffer directly.
static c_void InputTextReconcileUndoStateAfterUserCallback(*mut ImGuiInputTextState state, new_buf_a: *const c_char, new_length_a: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let old_buf: *const ImWchar = state.TextW.Data;
    let old_length: c_int = state.CurLenW;
    let new_length: c_int = ImTextCountCharsFromUtf8(new_buf_a, new_buf_a + new_length_a);
    g.TempBuffer.reserve_discard((new_length + 1) * sizeof);
    *mut let new_buf: ImWchar = (*mut ImWchar)g.TempBuffer.Data;
    ImTextStrFromUtf8(new_buf, new_length + 1, new_buf_a, new_buf_a + new_length_a);

    let shorter_length: c_int = ImMin(old_length, new_length);
    let mut first_diff: c_int = 0;
    for (first_diff = 0; first_diff < shorter_length; first_diff++)
        if (old_buf[first_diff] != new_buf[first_diff])
            break;
    if (first_diff == old_length && first_diff == new_length)
        return;

    let old_last_diff: c_int = old_length - 1;
    let new_last_diff: c_int = new_length - 1;
    for (; old_last_diff >= first_diff && new_last_diff >= first_diff; old_last_diff--, new_last_diff--)
        if (old_buf[old_last_diff] != new_buf[new_last_diff])
            break;

    let insert_len: c_int = new_last_diff - first_diff + 1;
    let delete_len: c_int = old_last_diff - first_diff + 1;
    if (insert_len > 0 || delete_len > 0)
        if (*mut STB_TEXTEDIT_CHARTYPE p = stb_text_createundo(&state.Stb.undostate, first_diff, delete_len, insert_len))
            for (let i: c_int = 0; i < delete_len; i++)
                p[i] = ImStb::STB_TEXTEDIT_GETCHAR(state, first_diff + i);
}

// Edit a string of text
// - buf_size account for the zero-terminator, so a buf_size of 6 can hold "Hello" but not "Hello!".
//   This is so we can easily call InputText() on static arrays using ARRAYSIZE() and to match
//   Note that in std::string world, capacity() would omit 1 byte used by the zero-terminator.
// - When active, hold on a privately held copy of the text (and apply back to 'buf'). So changing 'buf' while the InputText is active has no effect.
// - If you want to use InputText() with std::string, see misc/cpp/imgui_stdlib.h
// (FIXME: Rather confusing and messy function, among the worse part of our codebase, expecting to rewrite a V2 at some point.. Partly because we are
//  doing UTF8 > U16 > UTF8 conversions on the go to easily interface with stb_textedit. Ideally should stay in UTF-8 all the time. See https://github.com/nothings/stb/issues/188)
bool InputTextEx(label: *const c_char, hint: *const c_char, *mut char buf, buf_size: c_int, const size_arg: &ImVec2, ImGuiInputTextFlags flags, ImGuiInputTextCallback callback, *mut c_void callback_user_data)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // IM_ASSERT(buf != NULL && buf_size >= 0);
    // IM_ASSERT(!((flags & ImGuiInputTextFlags_CallbackHistory) && (flags & ImGuiInputTextFlags_Multiline)));        // Can't use both together (they both use up/down keys)
    // IM_ASSERT(!((flags & ImGuiInputTextFlags_CallbackCompletion) && (flags & ImGuiInputTextFlags_AllowTabInput))); // Can't use both together (they both use tab key)

    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.IO;
    const let mut style = &mut g.Style;

    let RENDER_SELECTION_WHEN_INACTIVE: bool = false;
    let is_multiline: bool = (flags & ImGuiInputTextFlags_Multiline) != 0;
    let is_readonly: bool = (flags & ImGuiInputTextFlags_ReadOnly) != 0;
    let is_password: bool = (flags & ImGuiInputTextFlags_Password) != 0;
    let is_undoable: bool = (flags & ImGuiInputTextFlags_NoUndoRedo) == 0;
    let is_resizable: bool = (flags & ImGuiInputTextFlags_CallbackResize) != 0;
    if (is_resizable)
        // IM_ASSERT(callback != NULL); // Must provide a callback if you set the ImGuiInputTextFlags_CallbackResize flag!

    if (is_multiline) // Open group before calling GetID() because groups tracks id created within their scope (including the scrollbar)
        BeginGroup();
    let mut id: ImGuiID =  window.GetID(label);
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);
    let frame_size: ImVec2 = CalcItemSize(size_arg, CalcItemWidth(), (is_multiline ? g.FontSize * 8.0f32 : label_size.y) + style.FramePadding.y * 2.00f32); // Arbitrary default of 8 lines high for multi-line
    let total_size: ImVec2 = ImVec2::new(frame_size.x + (label_size.x > 0f32 ? style.ItemInnerSpacing.x + label_size.x : 0f32), frame_size.y);

    let mut frame_bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + frame_size);
    let mut total_bb: ImRect = ImRect::new(frame_bb.Min, frame_bb.Min + total_size);

    *mut ImGuiWindow draw_window = window;
    let inner_size: ImVec2 = frame_size;
    let mut item_status_flags: ImGuiItemStatusFlags =  0;
    ImGuiLastItemData item_data_backup;
    if (is_multiline)
    {
        let backup_pos: ImVec2 = window.DC.CursorPos;
        ItemSize(total_bb, style.FramePadding.y);
        if (!ItemAdd(total_bb, id, &frame_bb, ImGuiItemFlags_Inputable))
        {
            EndGroup();
            return false;
        }
        item_status_flags = g.LastItemData.StatusFlags;
        item_data_backup = g.LastItemData;
        window.DC.CursorPos = backup_pos;

        // We reproduce the contents of BeginChildFrame() in order to provide 'label' so our window internal data are easier to read/debug.
        // FIXME-NAV: Pressing NavActivate will trigger general child activation right before triggering our own below. Harmless but bizarre.
        PushStyleColor(ImGuiCol_ChildBg, style.Colors[ImGuiCol_FrameBg]);
        PushStyleVar(ImGuiStyleVar_ChildRounding, style.FrameRounding);
        PushStyleVar(ImGuiStyleVar_ChildBorderSize, style.FrameBorderSize);
        PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2::new2(0, 0)); // Ensure no clip rect so mouse hover can reach FramePadding edges
        let mut child_visible: bool =  BeginChildEx(label, id, frame_bb.GetSize(), true, ImGuiWindowFlags_NoMove);
        PopStyleVar(3);
        PopStyleColor();
        if (!child_visible)
        {
            EndChild();
            EndGroup();
            return false;
        }
        draw_window = g.CurrentWindow; // Child window
        draw_window.DC.NavLayersActiveMaskNext |= (1 << draw_window.DC.NavLayerCurrent); // This is to ensure that EndChild() will display a navigation highlight so we can "enter" into it.
        draw_window.DC.CursorPos += style.FramePadding;
        inner_size.x -= draw_window.ScrollbarSizes.x;
    }
    else
    {
        // Support for internal ImGuiInputTextFlags_MergedItem flag, which could be redesigned as an ItemFlags if needed (with test performed in ItemAdd)
        ItemSize(total_bb, style.FramePadding.y);
        if (!(flags & ImGuiInputTextFlags_MergedItem))
            if (!ItemAdd(total_bb, id, &frame_bb, ImGuiItemFlags_Inputable))
                return false;
        item_status_flags = g.LastItemData.StatusFlags;
    }
    let hovered: bool = ItemHoverable(frame_bb, id);
    if (hovered)
        g.MouseCursor = ImGuiMouseCursor_TextInput;

    // We are only allowed to access the state if we are already the active widget.
    *mut ImGuiInputTextState state = GetInputTextState(id);

    let input_requested_by_tabbing: bool = (item_status_flags & ImGuiItemStatusFlags_FocusedByTabbing) != 0;
    let input_requested_by_nav: bool = (g.ActiveId != id) && ((g.NavActivateInputId == id) || (g.NavActivateId == id && g.NavInputSource == ImGuiInputSource_Keyboard));

    let user_clicked: bool = hovered && io.MouseClicked[0];
    let user_scroll_finish: bool = is_multiline && state != null_mut() && g.ActiveId == 0 && g.ActiveIdPreviousFrame == GetWindowScrollbarID(draw_window, ImGuiAxis_Y);
    let user_scroll_active: bool = is_multiline && state != null_mut() && g.ActiveId == GetWindowScrollbarID(draw_window, ImGuiAxis_Y);
    let mut clear_active_id: bool =  false;
    let mut select_all: bool =  false;

    let scroll_y: c_float =  is_multiline ? draw_window.Scroll.y : f32::MAX;

    let init_changed_specs: bool = (state != null_mut() && state.Stb.single_line != !is_multiline);
    let init_make_active: bool = (user_clicked || user_scroll_finish || input_requested_by_nav || input_requested_by_tabbing);
    let init_state: bool = (init_make_active || user_scroll_active);
    if ((init_state && g.ActiveId != id) || init_changed_specs)
    {
        // Access state even if we don't own it yet.
        state = &g.InputTextState;
        state.CursorAnimReset();

        // Take a copy of the initial buffer value (both in original UTF-8 format and converted to wchar)
        // From the moment we focused we are ignoring the content of 'buf' (unless we are in read-only mode)
        let buf_len: c_int = strlen(buf);
        state.InitialTextA.resize(buf_len + 1);    // UTF-8. we use +1 to make sure that .Data is always pointing to at least an empty string.
        memcpy(state.InitialTextA.Data, buf, buf_len + 1);

        // Preserve cursor position and undo/redo stack if we come back to same widget
        // FIXME: Since we reworked this on 2022/06, may want to differenciate recycle_cursor vs recycle_undostate?
        let mut recycle_state: bool =  (state.ID == id && !init_changed_specs);
        if (recycle_state && (state.CurLenA != buf_len || (state.TextAIsValid && strncmp(state.TextA.Data, buf, buf_len) != 0)))
            recycle_state = false;

        // Start edition
        let mut  buf_end: *const c_char= null_mut();
        state.ID = id;
        state.TextW.resize(buf_size + 1);          // wchar count <= UTF-8 count. we use +1 to make sure that .Data is always pointing to at least an empty string.
        state.TextA.clear();
        state.TextAIsValid = false;                // TextA is not valid yet (we will display buf until then)
        state.CurLenW = ImTextStrFromUtf8(state.TextW.Data, buf_size, buf, null_mut(), &buf_end);
        state.CurLenA = (buf_end - buf);      // We can't get the result from ImStrncpy() above because it is not UTF-8 aware. Here we'll cut off malformed UTF-8.

        if (recycle_state)
        {
            // Recycle existing cursor/selection/undo stack but clamp position
            // Note a single mouse click will override the cursor/position immediately by calling stb_textedit_click handler.
            state.CursorClamp();
        }
        else
        {
            state.ScrollX = 0f32;
            stb_textedit_initialize_state(&state.Stb, !is_multiline);
        }

        if (!is_multiline)
        {
            if (flags & ImGuiInputTextFlags_AutoSelectAll)
                select_all = true;
            if (input_requested_by_nav && (!recycle_state || !(g.NavActivateFlags & ImGuiActivateFlags_TryToPreserveState)))
                select_all = true;
            if (input_requested_by_tabbing || (user_clicked && io.KeyCtrl))
                select_all = true;
        }

        if (flags & ImGuiInputTextFlags_AlwaysOverwrite)
            state.Stb.insert_mode = 1; // stb field name is indeed incorrect (see #2863)
    }

    if (g.ActiveId != id && init_make_active)
    {
        // IM_ASSERT(state && state.ID == id);
        SetActiveID(id, window);
        SetFocusID(id, window);
        FocusWindow(window);

        // Declare our inputs
        g.ActiveIdUsingNavDirMask |= (1 << ImGuiDir_Left) | (1 << ImGuiDir_Right);
        if (is_multiline || (flags & ImGuiInputTextFlags_CallbackHistory))
            g.ActiveIdUsingNavDirMask |= (1 << ImGuiDir_Up) | (1 << ImGuiDir_Down);
        SetActiveIdUsingKey(ImGuiKey_Escape);
        SetActiveIdUsingKey(ImGuiKey_NavGamepadCancel);
        SetActiveIdUsingKey(ImGuiKey_Home);
        SetActiveIdUsingKey(ImGuiKey_End);
        if (is_multiline)
        {
            SetActiveIdUsingKey(ImGuiKey_PageUp);
            SetActiveIdUsingKey(ImGuiKey_PageDown);
        }
        if (flags & (ImGuiInputTextFlags_CallbackCompletion | ImGuiInputTextFlags_AllowTabInput)) // Disable keyboard tabbing out as we will use the \t character.
        {
            SetActiveIdUsingKey(ImGuiKey_Tab);
        }
    }

    // We have an edge case if ActiveId was set through another widget (e.g. widget being swapped), clear id immediately (don't wait until the end of the function)
    if (g.ActiveId == id && state == null_mut())
        ClearActiveID();

    // Release focus when we click outside
    if (g.ActiveId == id && io.MouseClicked[0] && !init_state && !init_make_active) //-V560
        clear_active_id = true;

    // Lock the decision of whether we are going to take the path displaying the cursor or selection
    let render_cursor: bool = (g.ActiveId == id) || (state && user_scroll_active);
    let mut render_selection: bool =  state && (state.HasSelection() || select_all) && (RENDER_SELECTION_WHEN_INACTIVE || render_cursor);
    let mut value_changed: bool =  false;
    let mut validated: bool =  false;

    // When read-only we always use the live data passed to the function
    // FIXME-OPT: Because our selection/cursor code currently needs the wide text we need to convert it when active, which is not ideal :(
    if (is_readonly && state != null_mut() && (render_cursor || render_selection))
    {
        let mut  buf_end: *const c_char= null_mut();
        state.TextW.resize(buf_size + 1);
        state.CurLenW = ImTextStrFromUtf8(state.TextW.Data, state.TextW.Size, buf, null_mut(), &buf_end);
        state.CurLenA = (buf_end - buf);
        state.CursorClamp();
        render_selection &= state.HasSelection();
    }

    // Select the buffer to render.
    let buf_display_from_state: bool = (render_cursor || render_selection || g.ActiveId == id) && !is_readonly && state && state.TextAIsValid;
    let is_displaying_hint: bool = (hint != null_mut() && (buf_display_from_state ? state.TextA.Data : buf)[0] == 0);

    // Password pushes a temporary font with only a fallback glyph
    if (is_password && !is_displaying_hint)
    {
        let glyph: *const ImFontGlyph = g.Font.FindGlyph('*');
        *mut ImFont password_font = &g.InputTextPasswordFont;
         = g.Font.FontSize;
         = g.Font.Scale;
         = g.Font.Ascent;
         = g.Font.Descent;
         = g.Font.ContainerAtlas;
         = glyph;
         = ;
        // IM_ASSERT(password_font.Glyphs.empty() && password_font.IndexAdvanceX.empty() && password_font.IndexLookup.empty());
        PushFont(password_font);
    }

    // Process mouse inputs and character inputs
    let backup_current_text_length: c_int = 0;
    if (g.ActiveId == id)
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
        let mouse_x: c_float =  (io.MousePos.x - frame_bb.Min.x - style.FramePadding.x) + state.ScrollX;
        let mouse_y: c_float =  (is_multiline ? (io.MousePos.y - draw_window.DC.CursorPos.y) : (g.FontSize * 0.5f32));

        let is_osx: bool = io.ConfigMacOSXBehaviors;
        if (select_all)
        {
            state.SelectAll();
            state.SelectedAllMouseLock = true;
        }
        else if (hovered && io.MouseClickedCount[0] >= 2 && !io.KeyShift)
        {
            stb_textedit_click(state, &state.Stb, mouse_x, mouse_y);
            let multiclick_count: c_int = (io.MouseClickedCount[0] - 2);
            if ((multiclick_count % 2) == 0)
            {
                // Double-click: Select word
                // We always use the "Mac" word advance for double-click select vs CTRL+Right which use the platform dependent variant:
                // FIXME: There are likely many ways to improve this behavior, but there's no "right" behavior (depends on use-case, software, OS)
                let is_bol: bool = (state.Stb.cursor == 0) || ImStb::STB_TEXTEDIT_GETCHAR(state, state.Stb.cursor - 1) == '\n';
                if (STB_TEXT_HAS_SELECTION(&state.Stb) || !is_bol)
                    state.OnKeyPressed(STB_TEXTEDIT_K_WORDLEFT);
                //state.OnKeyPressed(STB_TEXTEDIT_K_WORDRIGHT | STB_TEXTEDIT_K_SHIFT);
                if (!STB_TEXT_HAS_SELECTION(&state.Stb))
                    ImStb::stb_textedit_prep_selection_at_cursor(&state.Stb);
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
                if (io.KeyShift)
                    stb_textedit_drag(state, &state.Stb, mouse_x, mouse_y);
                else
                    stb_textedit_click(state, &state.Stb, mouse_x, mouse_y);
                state.CursorAnimReset();
            }
        }
        else if (io.MouseDown[0] && !state.SelectedAllMouseLock && (io.MouseDelta.x != 0f32 || io.MouseDelta.y != 0f32))
        {
            stb_textedit_drag(state, &state.Stb, mouse_x, mouse_y);
            state.CursorAnimReset();
            state.CursorFollow = true;
        }
        if (state.SelectedAllMouseLock && !io.MouseDown[0])
            state.SelectedAllMouseLock = false;

        // We expect backends to emit a Tab key but some also emit a Tab character which we ignore (#2467, #1336)
        // (For Tab and Enter: Win32/SFML/Allegro are sending both keys and chars, GLFW and SDL are only sending keys. For Space they all send all threes)
        let ignore_char_inputs: bool = (io.KeyCtrl && !io.KeyAlt) || (is_osx && io.KeySuper);
        if ((flags & ImGuiInputTextFlags_AllowTabInput) && IsKeyPressed(ImGuiKey_Tab) && !ignore_char_inputs && !io.KeyShift && !is_readonly)
        {
            let mut c: c_uint =  '\t'; // Insert TAB
            if (InputTextFilterCharacter(&c, flags, callback, callback_user_data, ImGuiInputSource_Keyboard))
                state.OnKeyPressed(c);
        }

        // Process regular text input (before we check for Return because using some IME will effectively send a Return?)
        // We ignore CTRL inputs, but need to allow ALT+CTRL as some keyboards (e.g. German) use AltGR (which _is_ Alt+Ctrl) to input certain characters.
        if (io.InputQueueCharacters.Size > 0)
        {
            if (!ignore_char_inputs && !is_readonly && !input_requested_by_nav)
                for (let n: c_int = 0; n < io.InputQueueCharacters.Size; n++)
                {
                    // Insert character if they pass filtering
                    let mut c: c_uint =  io.InputQueueCharacters[n];
                    if (c == '\t') // Skip Tab, see above.
                        continue;
                    if (InputTextFilterCharacter(&c, flags, callback, callback_user_data, ImGuiInputSource_Keyboard))
                        state.OnKeyPressed(c);
                }

            // Consume characters
            io.InputQueueCharacters.clear();
        }
    }

    // Process other shortcuts/key-presses
    let mut cancel_edit: bool =  false;
    if (g.ActiveId == id && !g.ActiveIdIsJustActivated && !clear_active_id)
    {
        // IM_ASSERT(state != NULL);

        let row_count_per_page: c_int = ImMax(((inner_size.y - style.FramePadding.y) / g.FontSize), 1);
        state.Stb.row_count_per_page = row_count_per_page;

        let k_mask: c_int = (io.KeyShift ? STB_TEXTEDIT_K_SHIFT : 0);
        let is_osx: bool = io.ConfigMacOSXBehaviors;
        let is_osx_shift_shortcut: bool = is_osx && (io.KeyMods == (ImGuiModFlags_Super | ImGuiModFlags_Shift));
        let is_wordmove_key_down: bool = is_osx ? io.KeyAlt : io.KeyCtrl;                     // OS X style: Text editing cursor movement using Alt instead of Ctrl
        let is_startend_key_down: bool = is_osx && io.KeySuper && !io.KeyCtrl && !io.KeyAlt;  // OS X style: Line/Text Start and End using Cmd+Arrows instead of Home/End
        let is_ctrl_key_only: bool = (io.KeyMods == ImGuiModFlags_Ctrl);
        let is_shift_key_only: bool = (io.KeyMods == ImGuiModFlags_Shift);
        let is_shortcut_key: bool = g.IO.ConfigMacOSXBehaviors ? (io.KeyMods == ImGuiModFlags_Super) : (io.KeyMods == ImGuiModFlags_Ctrl);

        let is_cut: bool = ((is_shortcut_key && IsKeyPressed(ImGuiKey_X)) || (is_shift_key_only && IsKeyPressed(ImGuiKey_Delete))) && !is_readonly && !is_password && (!is_multiline || state.HasSelection());
        let is_copy: bool = ((is_shortcut_key && IsKeyPressed(ImGuiKey_C)) || (is_ctrl_key_only  && IsKeyPressed(ImGuiKey_Insert))) && !is_password && (!is_multiline || state.HasSelection());
        let is_paste: bool = ((is_shortcut_key && IsKeyPressed(ImGuiKey_V)) || (is_shift_key_only && IsKeyPressed(ImGuiKey_Insert))) && !is_readonly;
        let is_undo: bool = ((is_shortcut_key && IsKeyPressed(ImGuiKey_Z)) && !is_readonly && is_undoable);
        let is_redo: bool = ((is_shortcut_key && IsKeyPressed(ImGuiKey_Y)) || (is_osx_shift_shortcut && IsKeyPressed(ImGuiKey_Z))) && !is_readonly && is_undoable;
        let is_select_all: bool = is_shortcut_key && IsKeyPressed(ImGuiKey_A);

        // We allow validate/cancel with Nav source (gamepad) to makes it easier to undo an accidental NavInput press with no keyboard wired, but otherwise it isn't very useful.
        let nav_gamepad_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (io.BackendFlags & ImGuiBackendFlags_HasGamepad) != 0;
        let is_enter_pressed: bool = IsKeyPressed(ImGuiKey_Enter, true) || IsKeyPressed(ImGuiKey_KeypadEnter, true);
        let is_gamepad_validate: bool = nav_gamepad_active && (IsKeyPressed(ImGuiKey_NavGamepadActivate, false) || IsKeyPressed(ImGuiKey_NavGamepadInput, false));
        let is_cancel: bool = IsKeyPressed(ImGuiKey_Escape, false) || (nav_gamepad_active && IsKeyPressed(ImGuiKey_NavGamepadCancel, false));

        if (IsKeyPressed(ImGuiKey_LeftArrow))                        { state.OnKeyPressed((is_startend_key_down ? STB_TEXTEDIT_K_LINESTART : is_wordmove_key_down ? STB_TEXTEDIT_K_WORDLEFT : STB_TEXTEDIT_K_LEFT) | k_mask); }
        else if (IsKeyPressed(ImGuiKey_RightArrow))                  { state.OnKeyPressed((is_startend_key_down ? STB_TEXTEDIT_K_LINEEND : is_wordmove_key_down ? STB_TEXTEDIT_K_WORDRIGHT : STB_TEXTEDIT_K_RIGHT) | k_mask); }
        else if (IsKeyPressed(ImGuiKey_UpArrow) && is_multiline)     { if (io.KeyCtrl) SetScrollY(draw_window, ImMax(draw_window.Scroll.y - g.FontSize, 0f32)); else state.OnKeyPressed((is_startend_key_down ? STB_TEXTEDIT_K_TEXTSTART : STB_TEXTEDIT_K_UP) | k_mask); }
        else if (IsKeyPressed(ImGuiKey_DownArrow) && is_multiline)   { if (io.KeyCtrl) SetScrollY(draw_window, ImMin(draw_window.Scroll.y + g.FontSize, GetScrollMaxY())); else state.OnKeyPressed((is_startend_key_down ? STB_TEXTEDIT_K_TEXTEND : STB_TEXTEDIT_K_DOWN) | k_mask); }
        else if (IsKeyPressed(ImGuiKey_PageUp) && is_multiline)      { state.OnKeyPressed(STB_TEXTEDIT_K_PGUP | k_mask); scroll_y -= row_count_per_page * g.FontSize; }
        else if (IsKeyPressed(ImGuiKey_PageDown) && is_multiline)    { state.OnKeyPressed(STB_TEXTEDIT_K_PGDOWN | k_mask); scroll_y += row_count_per_page * g.FontSize; }
        else if (IsKeyPressed(ImGuiKey_Home))                        { state.OnKeyPressed(io.KeyCtrl ? STB_TEXTEDIT_K_TEXTSTART | k_mask : STB_TEXTEDIT_K_LINESTART | k_mask); }
        else if (IsKeyPressed(ImGuiKey_End))                         { state.OnKeyPressed(io.KeyCtrl ? STB_TEXTEDIT_K_TEXTEND | k_mask : STB_TEXTEDIT_K_LINEEND | k_mask); }
        else if (IsKeyPressed(ImGuiKey_Delete) && !is_readonly && !is_cut) { state.OnKeyPressed(STB_TEXTEDIT_K_DELETE | k_mask); }
        else if (IsKeyPressed(ImGuiKey_Backspace) && !is_readonly)
        {
            if (!state.HasSelection())
            {
                if (is_wordmove_key_down)
                    state.OnKeyPressed(STB_TEXTEDIT_K_WORDLEFT | STB_TEXTEDIT_K_SHIFT);
                else if (is_osx && io.KeySuper && !io.KeyAlt && !io.KeyCtrl)
                    state.OnKeyPressed(STB_TEXTEDIT_K_LINESTART | STB_TEXTEDIT_K_SHIFT);
            }
            state.OnKeyPressed(STB_TEXTEDIT_K_BACKSPACE | k_mask);
        }
        else if (is_enter_pressed || is_gamepad_validate)
        {
            // Determine if we turn Enter into a \n character
            let mut ctrl_enter_for_new_line: bool =  (flags & ImGuiInputTextFlags_CtrlEnterForNewLine) != 0;
            if (!is_multiline || is_gamepad_validate || (ctrl_enter_for_new_line && !io.KeyCtrl) || (!ctrl_enter_for_new_line && io.KeyCtrl))
            {
                validated = true;
                if (io.ConfigInputTextEnterKeepActive && !is_multiline)
                    state.SelectAll(); // No need to scroll
                else
                    clear_active_id = true;
            }
            else if (!is_readonly)
            {
                let mut c: c_uint =  '\n'; // Insert new line
                if (InputTextFilterCharacter(&c, flags, callback, callback_user_data, ImGuiInputSource_Keyboard))
                    state.OnKeyPressed(c);
            }
        }
        else if (is_cancel)
        {
            clear_active_id = cancel_edit = true;
        }
        else if (is_undo || is_redo)
        {
            state.OnKeyPressed(is_undo ? STB_TEXTEDIT_K_UNDO : STB_TEXTEDIT_K_REDO);
            state.ClearSelection();
        }
        else if (is_select_all)
        {
            state.SelectAll();
            state.CursorFollow = true;
        }
        else if (is_cut || is_copy)
        {
            // Cut, Copy
            if (io.SetClipboardTextFn)
            {
                let ib: c_int = state.HasSelection() ? ImMin(state.Stb.select_start, state.Stb.select_end) : 0;
                let ie: c_int = state.HasSelection() ? ImMax(state.Stb.select_start, state.Stb.select_end) : state.CurLenW;
                let clipboard_data_len: c_int = ImTextCountUtf8BytesFromStr(state.TextW.Data + ib, state.TextW.Data + ie) + 1;
                *mut char clipboard_data = (*mut char)IM_ALLOC(clipboard_data_len * sizeof);
                ImTextStrToUtf8(clipboard_data, clipboard_data_len, state.TextW.Data + ib, state.TextW.Data + ie);
                SetClipboardText(clipboard_data);
                MemFree(clipboard_data);
            }
            if (is_cut)
            {
                if (!state.HasSelection())
                    state.SelectAll();
                state.CursorFollow = true;
                stb_textedit_cut(state, &state.Stb);
            }
        }
        else if (is_paste)
        {
            if (*const char clipboard = GetClipboardText())
            {
                // Filter pasted buffer
                let clipboard_len: c_int = strlen(clipboard);
                *mut let clipboard_filtered: ImWchar = (*mut ImWchar)IM_ALLOC((clipboard_len + 1) * sizeof);
                let clipboard_filtered_len: c_int = 0;
                for (*const char s = clipboard; *s; )
                {
                    c_uint c;
                    s += ImTextCharFromUtf8(&c, s, null_mut());
                    if (c == 0)
                        break;
                    if (!InputTextFilterCharacter(&c, flags, callback, callback_user_data, ImGuiInputSource_Clipboard))
                        continue;
                    clipboard_filtered[clipboard_filtered_len++] = c;
                }
                clipboard_filtered[clipboard_filtered_len] = 0;
                if (clipboard_filtered_len > 0) // If everything was filtered, ignore the pasting operation
                {
                    stb_textedit_paste(state, &state.Stb, clipboard_filtered, clipboard_filtered_len);
                    state.CursorFollow = true;
                }
                MemFree(clipboard_filtered);
            }
        }

        // Update render selection flag after events have been handled, so selection highlight can be displayed during the same frame.
        render_selection |= state.HasSelection() && (RENDER_SELECTION_WHEN_INACTIVE || render_cursor);
    }

    // Process callbacks and apply result back to user's buffer.
    let mut  apply_new_text: *const c_char= null_mut();
    let apply_new_text_length: c_int = 0;
    if (g.ActiveId == id)
    {
        // IM_ASSERT(state != NULL);
        if (cancel_edit)
        {
            // Restore initial value. Only return true if restoring to the initial value changes the current buffer contents.
            if (!is_readonly && strcmp(buf, state.InitialTextA.Data) != 0)
            {
                // Push records into the undo stack so we can CTRL+Z the revert operation itself
                apply_new_text = state.InitialTextA.Data;
                apply_new_text_length = state.InitialTextA.Size - 1;
                Vec<ImWchar> w_text;
                if (apply_new_text_length > 0)
                {
                    w_text.resize(ImTextCountCharsFromUtf8(apply_new_text, apply_new_text + apply_new_text_length) + 1);
                    ImTextStrFromUtf8(w_text.Data, w_text.Size, apply_new_text, apply_new_text + apply_new_text_length);
                }
                stb_textedit_replace(state, &state.Stb, w_text.Data, (apply_new_text_length > 0) ? (w_text.Size - 1) : 0);
            }
        }

        // Apply ASCII value
        if (!is_readonly)
        {
            state.TextAIsValid = true;
            state.TextA.resize(state.TextW.Size * 4 + 1);
            ImTextStrToUtf8(state.TextA.Data, state.TextA.Size, state.TextW.Data, null_mut());
        }

        // When using 'ImGuiInputTextFlags_EnterReturnsTrue' as a special case we reapply the live buffer back to the input buffer before clearing ActiveId, even though strictly speaking it wasn't modified on this frame.
        // If we didn't do that, code like InputInt() with ImGuiInputTextFlags_EnterReturnsTrue would fail.
        // This also allows the user to use InputText() with ImGuiInputTextFlags_EnterReturnsTrue without maintaining any user-side storage (please note that if you use this property along ImGuiInputTextFlags_CallbackResize you can end up with your temporary string object unnecessarily allocating once a frame, either store your string data, either if you don't then don't use ImGuiInputTextFlags_CallbackResize).
        let apply_edit_back_to_user_buffer: bool = !cancel_edit || (validated && (flags & ImGuiInputTextFlags_EnterReturnsTrue) != 0);
        if (apply_edit_back_to_user_buffer)
        {
            // Apply new value immediately - copy modified buffer back
            // Note that as soon as the input box is active, the in-widget value gets priority over any underlying modification of the input buffer
            // FIXME: We actually always render 'buf' when calling DrawList.AddText, making the comment above incorrect.
            // FIXME-OPT: CPU waste to do this every time the widget is active, should mark dirty state from the stb_textedit callbacks.

            // User callback
            if ((flags & (ImGuiInputTextFlags_CallbackCompletion | ImGuiInputTextFlags_CallbackHistory | ImGuiInputTextFlags_CallbackEdit | ImGuiInputTextFlags_CallbackAlways)) != 0)
            {
                // IM_ASSERT(callback != NULL);

                // The reason we specify the usage semantic (Completion/History) is that Completion needs to disable keyboard TABBING at the moment.
                ImGuiInputTextFlags event_flag = 0;
                let mut event_key: ImGuiKey =  ImGuiKey_None;
                if ((flags & ImGuiInputTextFlags_CallbackCompletion) != 0 && IsKeyPressed(ImGuiKey_Tab))
                {
                    event_flag = ImGuiInputTextFlags_CallbackCompletion;
                    event_key = ImGuiKey_Tab;
                }
                else if ((flags & ImGuiInputTextFlags_CallbackHistory) != 0 && IsKeyPressed(ImGuiKey_UpArrow))
                {
                    event_flag = ImGuiInputTextFlags_CallbackHistory;
                    event_key = ImGuiKey_UpArrow;
                }
                else if ((flags & ImGuiInputTextFlags_CallbackHistory) != 0 && IsKeyPressed(ImGuiKey_DownArrow))
                {
                    event_flag = ImGuiInputTextFlags_CallbackHistory;
                    event_key = ImGuiKey_DownArrow;
                }
                else if ((flags & ImGuiInputTextFlags_CallbackEdit) && state.Edited)
                {
                    event_flag = ImGuiInputTextFlags_CallbackEdit;
                }
                else if (flags & ImGuiInputTextFlags_CallbackAlways)
                {
                    event_flag = ImGuiInputTextFlags_CallbackAlways;
                }

                if (event_flag)
                {
                    ImGuiInputTextCallbackData callback_data;
                    memset(&callback_data, 0, sizeof(ImGuiInputTextCallbackData));
                    callback_data.EventFlag = event_flag;
                    callback_data.Flags = flags;
                    callback_data.UserData = callback_user_data;

                    *mut char callback_buf = is_readonly ? buf : state.TextA.Data;
                    callback_data.EventKey = event_key;
                    callback_data.Buf = callback_buf;
                    callback_data.BufTextLen = state.CurLenA;
                    callback_data.BufSize = state.BufCapacityA;
                    callback_data.BufDirty = false;

                    // We have to convert from wchar-positions to UTF-8-positions, which can be pretty slow (an incentive to ditch the ImWchar buffer, see https://github.com/nothings/stb/issues/188)
                    *mut let text: ImWchar = state.TextW.Data;
                    let utf8_cursor_pos: c_int = callback_data.CursorPos = ImTextCountUtf8BytesFromStr(text, text + state.Stb.cursor);
                    let utf8_selection_start: c_int = callback_data.SelectionStart = ImTextCountUtf8BytesFromStr(text, text + state.Stb.select_start);
                    let utf8_selection_end: c_int = callback_data.SelectionEnd = ImTextCountUtf8BytesFromStr(text, text + state.Stb.select_end);

                    // Call user code
                    callback(&callback_data);

                    // Read back what user may have modified
                    callback_buf = is_readonly ? buf : state.TextA.Data; // Pointer may have been invalidated by a resize callback
                    // IM_ASSERT(callback_data.Buf == callback_buf);         // Invalid to modify those fields
                    // IM_ASSERT(callback_data.BufSize == state.BufCapacityA);
                    // IM_ASSERT(callback_data.Flags == flags);
                    let buf_dirty: bool = callback_data.BufDirty;
                    if (callback_data.CursorPos != utf8_cursor_pos || buf_dirty)            { state.Stb.cursor = ImTextCountCharsFromUtf8(callback_data.Buf, callback_data.Buf + callback_data.CursorPos); state.CursorFollow = true; }
                    if (callback_data.SelectionStart != utf8_selection_start || buf_dirty)  { state.Stb.select_start = (callback_data.SelectionStart == callback_data.CursorPos) ? state.Stb.cursor : ImTextCountCharsFromUtf8(callback_data.Buf, callback_data.Buf + callback_data.SelectionStart); }
                    if (callback_data.SelectionEnd != utf8_selection_end || buf_dirty)      { state.Stb.select_end = (callback_data.SelectionEnd == callback_data.SelectionStart) ? state.Stb.select_start : ImTextCountCharsFromUtf8(callback_data.Buf, callback_data.Buf + callback_data.SelectionEnd); }
                    if (buf_dirty)
                    {
                        // IM_ASSERT((flags & ImGuiInputTextFlags_ReadOnly) == 0);
                        // IM_ASSERT(callback_data.BufTextLen == strlen(callback_data.buf)); // You need to maintain BufTextLen if you change the text!
                        InputTextReconcileUndoStateAfterUserCallback(state, callback_data.Buf, callback_data.BufTextLen); // FIXME: Move the rest of this block inside function and rename to InputTextReconcileStateAfterUserCallback() ?
                        if (callback_data.BufTextLen > backup_current_text_length && is_resizable)
                            state.TextW.resize(state.TextW.Size + (callback_data.BufTextLen - backup_current_text_length)); // Worse case scenario resize
                        state.CurLenW = ImTextStrFromUtf8(state.TextW.Data, state.TextW.Size, callback_data.Buf, null_mut());
                        state.CurLenA = callback_data.BufTextLen;  // Assume correct length and valid UTF-8 from user, saves us an extra strlen()
                        state.CursorAnimReset();
                    }
                }
            }

            // Will copy result string if modified
            if (!is_readonly && strcmp(state.TextA.Data, buf) != 0)
            {
                apply_new_text = state.TextA.Data;
                apply_new_text_length = state.CurLenA;
            }
        }

        // Clear temporary user storage
        state.Flags = ImGuiInputTextFlags_None;
    }

    // Copy result to user buffer. This can currently only happen when (g.ActiveId == id)
    if (apply_new_text != null_mut())
    {
        // We cannot test for 'backup_current_text_length != apply_new_text_length' here because we have no guarantee that the size
        // of our owned buffer matches the size of the string object held by the user, and by design we allow InputText() to be used
        // without any storage on user's side.
        // IM_ASSERT(apply_new_text_length >= 0);
        if (is_resizable)
        {
            ImGuiInputTextCallbackData callback_data;
            callback_data.EventFlag = ImGuiInputTextFlags_CallbackResize;
            callback_data.Flags = flags;
            callback_data.Buf = buf;
            callback_data.BufTextLen = apply_new_text_length;
            callback_data.BufSize = ImMax(buf_size, apply_new_text_length + 1);
            callback_data.UserData = callback_user_data;
            callback(&callback_data);
            buf = callback_data.Buf;
            buf_size = callback_data.BufSize;
            apply_new_text_length = ImMin(callback_data.BufTextLen, buf_size - 1);
            // IM_ASSERT(apply_new_text_length <= buf_size);
        }
        //IMGUI_DEBUG_PRINT("InputText(\"%s\"): apply_new_text length %d\n", label, apply_new_text_length);

        // If the underlying buffer resize was denied or not carried to the next frame, apply_new_text_length+1 may be >= buf_size.
        ImStrncpy(buf, apply_new_text, ImMin(apply_new_text_length + 1, buf_size));
        value_changed = true;
    }

    // Release active ID at the end of the function (so e.g. pressing Return still does a final application of the value)
    if (clear_active_id && g.ActiveId == id)
        ClearActiveID();

    // Render frame
    if (!is_multiline)
    {
        RenderNavHighlight(frame_bb, id);
        RenderFrame(frame_bb.Min, frame_bb.Max, GetColorU32(ImGuiCol_FrameBg), true, style.FrameRounding);
    }

    const ImVec4 clip_rect(frame_bb.Min.x, frame_bb.Min.y, frame_bb.Min.x + inner_size.x, frame_bb.Min.y + inner_size.y); // Not using frame_bb.Max because we have adjusted size
    let draw_pos: ImVec2 = is_multiline ? draw_window.DC.CursorPos : frame_bb.Min + style.FramePadding;
    ImVec2 text_size(0f32, 0f32);

    // Set upper limit of single-line InputTextEx() at 2 million characters strings. The current pathological worst case is a long line
    // without any carriage return, which would makes ImFont::RenderText() reserve too many vertices and probably crash. Avoid it altogether.
    // Note that we only use this limit on single-line InputText(), so a pathologically large line on a InputTextMultiline() would still crash.
    let buf_display_max_length: c_int = 2 * 1024 * 1024;
    let mut  buf_display: *const c_char = buf_display_from_state ? state.TextA.Data : buf; //-V595
    let mut  buf_display_end: *const c_char= null_mut(); // We have specialized paths below for setting the length
    if (is_displaying_hint)
    {
        buf_display = hint;
        buf_display_end = hint + strlen(hint);
    }

    // Render text. We currently only render selection when the widget is active or while scrolling.
    // FIXME: We could remove the '&& render_cursor' to keep rendering selection when inactive.
    if (render_cursor || render_selection)
    {
        // IM_ASSERT(state != NULL);
        if (!is_displaying_hint)
            buf_display_end = buf_display + state.CurLenA;

        // Render text (with cursor and selection)
        // This is going to be messy. We need to:
        // - Display the text (this alone can be more easily clipped)
        // - Handle scrolling, highlight selection, display cursor (those all requires some form of 1d.2d cursor position calculation)
        // - Measure text height (for scrollbar)
        // We are attempting to do most of that in **one main pass** to minimize the computation cost (non-negligible for large amount of text) + 2nd pass for selection rendering (we could merge them by an extra refactoring effort)
        // FIXME: This should occur on buf_display but we'd need to maintain cursor/select_start/select_end for UTF-8.
        let text_begin: *const ImWchar = state.TextW.Data;
        cursor_offset: ImVec2, select_start_offset;

        {
            // Find lines numbers straddling 'cursor' (slot 0) and 'select_start' (slot 1) positions.
            *const ImWsearches_input_ptr: [c_char;2] = { null_mut(), null_mut() };
            c_int searches_result_line_no[2] = { -1000, -1000 };
            let searches_remaining: c_int = 0;
            if (render_cursor)
            {
                searches_input_ptr[0] = text_begin + state.Stb.cursor;
                searches_result_line_no[0] = -1;
                searches_remaining+= 1;
            }
            if (render_selection)
            {
                searches_input_ptr[1] = text_begin + ImMin(state.Stb.select_start, state.Stb.select_end);
                searches_result_line_no[1] = -1;
                searches_remaining+= 1;
            }

            // Iterate all lines to find our line numbers
            // In multi-line mode, we never exit the loop until all lines are counted, so add one extra to the searches_remaining counter.
            searches_remaining += is_multiline ? 1 : 0;
            let line_count: c_int = 0;
            //for (const ImWchar* s = text_begin; (s = (const ImWchar*)wcschr((const wchar_t*)s, (wchar_t)'\n')) != None; s++)  // FIXME-OPT: Could use this when wchar_t are 16-bit
            for (*const let s: ImWchar = text_begin; *s != 0; s++)
                if (*s == '\n')
                {
                    line_count+= 1;
                    if (searches_result_line_no[0] == -1 && s >= searches_input_ptr[0]) { searches_result_line_no[0] = line_count; if (--searches_remaining <= 0) break; }
                    if (searches_result_line_no[1] == -1 && s >= searches_input_ptr[1]) { searches_result_line_no[1] = line_count; if (--searches_remaining <= 0) break; }
                }
            line_count+= 1;
            if (searches_result_line_no[0] == -1)
                searches_result_line_no[0] = line_count;
            if (searches_result_line_no[1] == -1)
                searches_result_line_no[1] = line_count;

            // Calculate 2d position by finding the beginning of the line and measuring distance
            cursor_offset.x = InputTextCalcTextSizeW(ImStrbolW(searches_input_ptr[0], text_begin), searches_input_ptr[0]).x;
            cursor_offset.y = searches_result_line_no[0] * g.FontSize;
            if (searches_result_line_no[1] >= 0)
            {
                select_start_offset.x = InputTextCalcTextSizeW(ImStrbolW(searches_input_ptr[1], text_begin), searches_input_ptr[1]).x;
                select_start_offset.y = searches_result_line_no[1] * g.FontSize;
            }

            // Store text height (note that we haven't calculated text width at all, see GitHub issues #383, #1224)
            if (is_multiline)
                text_size = ImVec2::new(inner_size.x, line_count * g.FontSize);
        }

        // Scroll
        if (render_cursor && state.CursorFollow)
        {
            // Horizontal scroll in chunks of quarter width
            if (!(flags & ImGuiInputTextFlags_NoHorizontalScroll))
            {
                let scroll_increment_x: c_float =  inner_size.x * 0.25f32;
                let visible_width: c_float =  inner_size.x - style.FramePadding.x;
                if (cursor_offset.x < state.ScrollX)
                    state.ScrollX = IM_FLOOR(ImMax(0f32, cursor_offset.x - scroll_increment_x));
                else if (cursor_offset.x - visible_width >= state.ScrollX)
                    state.ScrollX = IM_FLOOR(cursor_offset.x - visible_width + scroll_increment_x);
            }
            else
            {
                state.ScrollX = 0f32;
            }

            // Vertical scroll
            if (is_multiline)
            {
                // Test if cursor is vertically visible
                if (cursor_offset.y - g.FontSize < scroll_y)
                    scroll_y = ImMax(0f32, cursor_offset.y - g.FontSize);
                else if (cursor_offset.y - (inner_size.y - style.FramePadding.y * 2.00f32) >= scroll_y)
                    scroll_y = cursor_offset.y - inner_size.y + style.FramePadding.y * 2.0f32;
                let scroll_max_y: c_float =  ImMax((text_size.y + style.FramePadding.y * 2.00f32) - inner_size.y, 0f32);
                scroll_y = ImClamp(scroll_y, 0f32, scroll_max_y);
                draw_pos.y += (draw_window.Scroll.y - scroll_y);   // Manipulate cursor pos immediately avoid a frame of lag
                draw_window.Scroll.y = scroll_y;
            }

            state.CursorFollow = false;
        }

        // Draw selection
        let draw_scroll: ImVec2 = ImVec2::new(state.ScrollX, 0f32);
        if (render_selection)
        {
            let text_selected_begin: *const ImWchar = text_begin + ImMin(state.Stb.select_start, state.Stb.select_end);
            let text_selected_end: *const ImWchar = text_begin + ImMax(state.Stb.select_start, state.Stb.select_end);

            let mut bg_color: u32 = GetColorU32(ImGuiCol_TextSelectedBg, render_cursor ? 1f32 : 0.60f32); // FIXME: current code flow mandate that render_cursor is always true here, we are leaving the transparent one for tests.
            let bg_offy_up: c_float =  is_multiline ? 0f32 : -1f32;    // FIXME: those offsets should be part of the style? they don't play so well with multi-line selection.
            let bg_offy_dn: c_float =  is_multiline ? 0f32 : 2.0f32;
            let rect_pos: ImVec2 = draw_pos + select_start_offset - draw_scroll;
            for (*const let p: ImWchar = text_selected_begin; p < text_selected_end; )
            {
                if (rect_pos.y > clip_rect.w + g.FontSize)
                    break;
                if (rect_pos.y < clip_rect.y)
                {
                    //p = (const ImWchar*)wmemchr((const wchar_t*)p, '\n', text_selected_end - p);  // FIXME-OPT: Could use this when wchar_t are 16-bit
                    //p = p ? p + 1 : text_selected_end;
                    while (p < text_selected_end)
                        if (*p++ == '\n')
                            break;
                }
                else
                {
                    let rect_size: ImVec2 = InputTextCalcTextSizeW(p, text_selected_end, &p, null_mut(), true);
                    if (rect_size.x <= 0f32) rect_size.x = IM_FLOOR(g.Font.GetCharAdvance(' ') * 0.500f32); // So we can see selected empty lines
                    let mut rect: ImRect = ImRect::new(rect_pos + ImVec2::new2(0f32, bg_offy_up - g.FontSize), rect_pos + ImVec2::new(rect_size.x, bg_offy_dn));
                    rect.ClipWith(clip_rect);
                    if (rect.Overlaps(clip_rect))
                        draw_window.DrawList.AddRectFilled(rect.Min, rect.Max, bg_color);
                }
                rect_pos.x = draw_pos.x - draw_scroll.x;
                rect_pos.y += g.FontSize;
            }
        }

        // We test for 'buf_display_max_length' as a way to avoid some pathological cases (e.g. single-line 1 MB string) which would make ImDrawList crash.
        if (is_multiline || (buf_display_end - buf_display) < buf_display_max_length)
        {
            let mut col: u32 = GetColorU32(is_displaying_hint ? ImGuiCol_TextDisabled : ImGuiCol_Text);
            draw_window.DrawList.AddText(g.Font, g.FontSize, draw_pos - draw_scroll, col, buf_display, buf_display_end, 0f32, is_multiline ? null_mut() : &clip_rect);
        }

        // Draw blinking cursor
        if (render_cursor)
        {
            state.CursorAnim += io.DeltaTime;
            let mut cursor_is_visible: bool =  (!g.IO.ConfigInputTextCursorBlink) || (state.CursorAnim <= 0f32) || ImFmod(state.CursorAnim, 1.200f32) <= 0.80f32;
            let cursor_screen_pos: ImVec2 = ImFloor(draw_pos + cursor_offset - draw_scroll);
            let mut cursor_screen_rect: ImRect = ImRect::new(cursor_screen_pos.x, cursor_screen_pos.y - g.FontSize + 0.5f32, cursor_screen_pos.x + 1f32, cursor_screen_pos.y - 1.5f32);
            if (cursor_is_visible && cursor_screen_rect.Overlaps(clip_rect))
                draw_window.DrawList.AddLine(cursor_screen_rect.Min, cursor_screen_rect.GetBL(), GetColorU32(ImGuiCol_Text));

            // Notify OS of text input position for advanced IME (-1 x offset so that Windows IME can cover our cursor. Bit of an extra nicety.)
            if (!is_readonly)
            {
                g.PlatformImeData.WantVisible = true;
                g.PlatformImeData.InputPos = ImVec2::new(cursor_screen_pos.x - 1f32, cursor_screen_pos.y - g.FontSize);
                g.PlatformImeData.InputLineHeight = g.FontSize;
                g.PlatformImeViewport = window.Viewport.ID;
            }
        }
    }
    else
    {
        // Render text only (no selection, no cursor)
        if (is_multiline)
            text_size = ImVec2::new(inner_size.x, InputTextCalcTextLenAndLineCount(buf_display, &buf_display_end) * g.FontSize); // We don't need width
        else if (!is_displaying_hint && g.ActiveId == id)
            buf_display_end = buf_display + state.CurLenA;
        else if (!is_displaying_hint)
            buf_display_end = buf_display + strlen(buf_display);

        if (is_multiline || (buf_display_end - buf_display) < buf_display_max_length)
        {
            let mut col: u32 = GetColorU32(is_displaying_hint ? ImGuiCol_TextDisabled : ImGuiCol_Text);
            draw_window.DrawList.AddText(g.Font, g.FontSize, draw_pos, col, buf_display, buf_display_end, 0f32, is_multiline ? null_mut() : &clip_rect);
        }
    }

    if (is_password && !is_displaying_hint)
        PopFont();

    if (is_multiline)
    {
        // For focus requests to work on our multiline we need to ensure our child ItemAdd() call specifies the ImGuiItemFlags_Inputable (ref issue #4761)...
        Dummy(ImVec2::new(text_size.x, text_size.y + style.FramePadding.y));
        let mut backup_item_flags: ImGuiItemFlags =  g.CurrentItemFlags;
        g.CurrentItemFlags |= ImGuiItemFlags_Inputable | ImGuiItemFlags_NoTabStop;
        EndChild();
        item_data_backup.StatusFlags |= (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_HoveredWindow);
        g.CurrentItemFlags = backup_item_flags;

        // ...and then we need to undo the group overriding last item data, which gets a bit messy as EndGroup() tries to forward scrollbar being active...
        // FIXME: This quite messy/tricky, should attempt to get rid of the child window.
        EndGroup();
        if (g.LastItemData.ID == 0)
        {
            g.LastItemData.ID = id;
            g.LastItemData.InFlags = item_data_backup.InFlags;
            g.LastItemData.StatusFlags = item_data_backup.StatusFlags;
        }
    }

    // Log as text
    if (g.LogEnabled && (!is_password || is_displaying_hint))
    {
        LogSetNextTextDecoration("{", "}");
        LogRenderedText(&draw_pos, buf_display, buf_display_end);
    }

    if (label_size.x > 0)
        RenderText(ImVec2::new(frame_bb.Max.x + style.ItemInnerSpacing.x, frame_bb.Min.y + style.FramePadding.y), label);

    if (value_changed && !(flags & ImGuiInputTextFlags_NoMarkEdited))
        MarkItemEdited(id);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags);
    if ((flags & ImGuiInputTextFlags_EnterReturnsTrue) != 0)
        return validated;
    else
        return value_changed;
}

c_void DebugNodeInputTextState(*mut ImGuiInputTextState state)
{
// #ifndef IMGUI_DISABLE_DEBUG_TOOLS
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImStb::*mut STB_TexteditState stb_state = &state.Stb;
    ImStb::*mut StbUndoState undo_state = &stb_state.undostate;
    Text("ID: 0x%08X, ActiveID: 0x%08X", state.ID, g.ActiveId);
    Text("CurLenW: %d, CurLenA: %d, Cursor: %d, Selection: %d..%d", state.CurLenA, state.CurLenW, stb_state.cursor, stb_state.select_start, stb_state.select_end);
    Text("undo_point: %d, redo_point: %d, undo_char_point: %d, redo_char_point: %d", undo_state.undo_point, undo_state.redo_point, undo_state.undo_char_point, undo_state.redo_char_point);
    if (BeginChild("undopoints", ImVec2::new2(0f32, GetTextLineHeight() * 15), true)) // Visualize undo state
    {
        PushStyleVar(ImGuiStyleVar_ItemSpacing, ImVec2::new2(0, 0));
        for (let n: c_int = 0; n < STB_TEXTEDIT_UNDOSTATECOUNT; n++)
        {
            ImStb::*mut StbUndoRecord undo_rec = &undo_state.undo_rec[n];
            const char undo_rec_type = (n < undo_state.undo_point) ? 'u' : (n >= undo_state.redo_point) ? 'r' : ' ';
            if (undo_rec_type == ' ')
                BeginDisabled();
            buf: [c_char;64] = "";
            if (undo_rec_type != ' ' &&  != -1)
                ImTextStrToUtf8(buf, IM_ARRAYSIZE(buf), undo_state.undo_char + , undo_state.undo_char +  + );
            Text("%c [%02d] where %03d, insert %03d, delete %03d, char_storage %03d \"%s\"",
                undo_rec_type, n, , , , , buf);
            if (undo_rec_type == ' ')
                EndDisabled();
        }
        PopStyleVar();
    }
    EndChild();
// #else
    IM_UNUSED(state);
// #endif
}

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

bool ColorEdit3(label: *const c_char, c_float col[3], ImGuiColorEditFlags flags)
{
    return ColorEdit4(label, col, flags | ImGuiColorEditFlags_NoAlpha);
}

// ColorEdit supports RGB and HSV inputs. In case of RGB input resulting color may have undefined hue and/or saturation.
// Since widget displays both RGB and HSV values we must preserve hue and saturation to prevent these values resetting.
static c_void ColorEditRestoreHS(*const col: c_float, *mut H: c_float, *mut S: c_float, *mut V: c_float)
{
    // This check is optional. Suppose we have two color widgets side by side, both widgets display different colors, but both colors have hue and/or saturation undefined.
    // With color check: hue/saturation is preserved in one widget. Editing color in one widget would reset hue/saturation in another one.
    // Without color check: common hue/saturation would be displayed in all widgets that have hue/saturation undefined.
    // g.ColorEditLastColor is stored as ImU32 RGB value: this essentially gives us color equality check with reduced precision.
    // Tiny external color changes would not be detected and this check would still pass. This is OK, since we only restore hue/saturation _only_ if they are undefined,
    // therefore this change flipping hue/saturation from undefined to a very tiny value would still be represented in color picker.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.ColorEditLastColor != ColorConvertFloat4ToU32(ImVec4(col[0], col[1], col[2], 0)))
        return;

    // When S == 0, H is undefined.
    // When H == 1 it wraps around to 0.
    if (*S == 0f32 || (*H == 0f32 && g.ColorEditLastHue == 1))
        *H = g.ColorEditLastHue;

    // When V == 0, S is undefined.
    if (*V == 0f32)
        *S = g.ColorEditLastSat;
}

// Edit colors components (each component in 0f32..1f32 range).
// See enum ImGuiColorEditFlags_ for available options. e.g. Only access 3 floats if ImGuiColorEditFlags_NoAlpha flag is set.
// With typical options: Left-click on color square to open color picker. Right-click to open option menu. CTRL-Click over input fields to edit them and TAB to go to next item.
bool ColorEdit4(label: *const c_char, c_float col[4], ImGuiColorEditFlags flags)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    const let mut style = &mut g.Style;
    let square_sz: c_float =  GetFrameHeight();
    let w_full: c_float =  CalcItemWidth();
    let w_button: c_float =  (flags & ImGuiColorEditFlags_NoSmallPreview) ? 0f32 : (square_sz + style.ItemInnerSpacing.x);
    let w_inputs: c_float =  w_full - w_button;
    let mut  label_display_end: *const c_char = FindRenderedTextEnd(label);
    g.NextItemData.ClearFlags();

    BeginGroup();
    PushID(label);

    // If we're not showing any slider there's no point in doing any HSV conversions
    const ImGuiColorEditFlags flags_untouched = flags;
    if (flags & ImGuiColorEditFlags_NoInputs)
        flags = (flags & (~ImGuiColorEditFlags_DisplayMask_)) | ImGuiColorEditFlags_DisplayRGB | ImGuiColorEditFlags_NoOptions;

    // Context menu: display and modify options (before defaults are applied)
    if (!(flags & ImGuiColorEditFlags_NoOptions))
        ColorEditOptionsPopup(col, flags);

    // Read stored options
    if (!(flags & ImGuiColorEditFlags_DisplayMask_))
        flags |= (g.ColorEditOptions & ImGuiColorEditFlags_DisplayMask_);
    if (!(flags & ImGuiColorEditFlags_DataTypeMask_))
        flags |= (g.ColorEditOptions & ImGuiColorEditFlags_DataTypeMask_);
    if (!(flags & ImGuiColorEditFlags_PickerMask_))
        flags |= (g.ColorEditOptions & ImGuiColorEditFlags_PickerMask_);
    if (!(flags & ImGuiColorEditFlags_InputMask_))
        flags |= (g.ColorEditOptions & ImGuiColorEditFlags_InputMask_);
    flags |= (g.ColorEditOptions & ~(ImGuiColorEditFlags_DisplayMask_ | ImGuiColorEditFlags_DataTypeMask_ | ImGuiColorEditFlags_PickerMask_ | ImGuiColorEditFlags_InputMask_));
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_DisplayMask_)); // Check that only 1 is selected
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_InputMask_));   // Check that only 1 is selected

    let alpha: bool = (flags & ImGuiColorEditFlags_NoAlpha) == 0;
    let hdr: bool = (flags & ImGuiColorEditFlags_HDR) != 0;
    let components: c_int = alpha ? 4 : 3;

    // Convert to the formats we need
    c_float f[4] = { col[0], col[1], col[2], alpha ? col[3] : 1f32 };
    if ((flags & ImGuiColorEditFlags_InputHSV) && (flags & ImGuiColorEditFlags_DisplayRGB))
        ColorConvertHSVtoRGB(f[0], f[1], f[2], f[0], f[1], f[2]);
    else if ((flags & ImGuiColorEditFlags_InputRGB) && (flags & ImGuiColorEditFlags_DisplayHSV))
    {
        // Hue is lost when converting from greyscale rgb (saturation=0). Restore it.
        ColorConvertRGBtoHSV(f[0], f[1], f[2], f[0], f[1], f[2]);
        ColorEditRestoreHS(col, &f[0], &f[1], &f[2]);
    }
    c_int i[4] = { IM_F32_TO_INT8_UNBOUND(f[0]), IM_F32_TO_INT8_UNBOUND(f[1]), IM_F32_TO_INT8_UNBOUND(f[2]), IM_F32_TO_INT8_UNBOUND(f[3]) };

    let mut value_changed: bool =  false;
    let mut value_changed_as_float: bool =  false;

    let pos: ImVec2 = window.DC.CursorPos;
    let inputs_offset_x: c_float =  (style.ColorButtonPosition == ImGuiDir_Left) ? w_button : 0f32;
    window.DC.CursorPos.x = pos.x + inputs_offset_x;

    if ((flags & (ImGuiColorEditFlags_DisplayRGB | ImGuiColorEditFlags_DisplayHSV)) != 0 && (flags & ImGuiColorEditFlags_NoInputs) == 0)
    {
        // RGB/HSV 0..255 Sliders
        const c_float w_item_one  = ImMax(1f32, IM_FLOOR((w_inputs - (style.ItemInnerSpacing.x) * (components - 1)) / components));
        let w_item_last: c_float =  ImMax(1f32, IM_FLOOR(w_inputs - (w_item_one + style.ItemInnerSpacing.x) * (components - 1)));

        let hide_prefix: bool = (w_item_one <= CalcTextSize((flags & ImGuiColorEditFlags_Float) ? "M:0.000" : "M:000").x);
        static *const ids: [c_char;4] = { "##X", "##Y", "##Z", "##W" };
        static *const fmt_table_int: [c_char;3][4] =
        {
            {   "%3d",   "%3d",   "%3d",   "%3d" }, // Short display
            { "R:%3d", "G:%3d", "B:%3d", "A:%3d" }, // Long display for RGBA
            { "H:%3d", "S:%3d", "V:%3d", "A:%3d" }  // Long display for HSVA
        };
        static *const fmt_table_float: [c_char;3][4] =
        {
            {   "%0.3f",   "%0.3f",   "%0.3f",   "%0.3f" }, // Short display
            { "R:%0.3f", "G:%0.3f", "B:%0.3f", "A:%0.3f" }, // Long display for RGBA
            { "H:%0.3f", "S:%0.3f", "V:%0.3f", "A:%0.3f" }  // Long display for HSVA
        };
        let fmt_idx: c_int = hide_prefix ? 0 : (flags & ImGuiColorEditFlags_DisplayHSV) ? 2 : 1;

        for (let n: c_int = 0; n < components; n++)
        {
            if (n > 0)
                SameLine(0, style.ItemInnerSpacing.x);
            SetNextItemWidth((n + 1 < components) ? w_item_one : w_item_last);

            // FIXME: When ImGuiColorEditFlags_HDR flag is passed HS values snap in weird ways when SV values go below 0.
            if (flags & ImGuiColorEditFlags_Float)
            {
                value_changed |= DragFloat(ids[n], &f[n], 1f32 / 255f32, 0f32, hdr ? 0f32 : 1f32, fmt_table_float[fmt_idx][n]);
                value_changed_as_float |= value_changed;
            }
            else
            {
                value_changed |= DragInt(ids[n], &i[n], 1f32, 0, hdr ? 0 : 255, fmt_table_int[fmt_idx][n]);
            }
            if (!(flags & ImGuiColorEditFlags_NoOptions))
                OpenPopupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);
        }
    }
    else if ((flags & ImGuiColorEditFlags_DisplayHex) != 0 && (flags & ImGuiColorEditFlags_NoInputs) == 0)
    {
        // RGB Hexadecimal Input
        buf: [c_char;64];
        if (alpha)
            ImFormatString(buf, IM_ARRAYSIZE(buf), "#%02X%02X%02X%02X", ImClamp(i[0], 0, 255), ImClamp(i[1], 0, 255), ImClamp(i[2], 0, 255), ImClamp(i[3], 0, 255));
        else
            ImFormatString(buf, IM_ARRAYSIZE(buf), "#%02X%02X%02X", ImClamp(i[0], 0, 255), ImClamp(i[1], 0, 255), ImClamp(i[2], 0, 255));
        SetNextItemWidth(w_inputs);
        if (InputText("##Text", buf, IM_ARRAYSIZE(buf), ImGuiInputTextFlags_CharsHexadecimal | ImGuiInputTextFlags_CharsUppercase))
        {
            value_changed = true;
            *mut char p = buf;
            while (*p == '#' || ImCharIsBlankA(*p))
                p+= 1;
            i[0] = i[1] = i[2] = 0;
            i[3] = 0xFF; // alpha default to 255 is not parsed by scanf (e.g. inputting #FFFFFF omitting alpha)
            let mut r: c_int = 0;
            if (alpha)
                r = sscanf(p, "%02X%02X%02X%02X", (*mut c_uint)&i[0], (*mut c_uint)&i[1], (*mut c_uint)&i[2], (*mut c_uint)&i[3]); // Treat at unsigned (%X is unsigned)
            else
                r = sscanf(p, "%02X%02X%02X", (*mut c_uint)&i[0], (*mut c_uint)&i[1], (*mut c_uint)&i[2]);
            IM_UNUSED(r); // Fixes C6031: Return value ignored: 'sscanf'.
        }
        if (!(flags & ImGuiColorEditFlags_NoOptions))
            OpenPopupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);
    }

    *mut ImGuiWindow picker_active_window= null_mut();
    if (!(flags & ImGuiColorEditFlags_NoSmallPreview))
    {
        let button_offset_x: c_float =  ((flags & ImGuiColorEditFlags_NoInputs) || (style.ColorButtonPosition == ImGuiDir_Left)) ? 0f32 : w_inputs + style.ItemInnerSpacing.x;
        window.DC.CursorPos = ImVec2::new(pos.x + button_offset_x, pos.y);

        const ImVec4 col_v4(col[0], col[1], col[2], alpha ? col[3] : 1f32);
        if (ColorButton("##ColorButton", col_v4, flags))
        {
            if (!(flags & ImGuiColorEditFlags_NoPicker))
            {
                // Store current color and open a picker
                g.ColorPickerRef = col_v4;
                OpenPopup("picker");
                SetNextWindowPos(g.LastItemData.Rect.GetBL() + ImVec2::new2(0f32, style.ItemSpacing.y));
            }
        }
        if (!(flags & ImGuiColorEditFlags_NoOptions))
            OpenPopupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);

        if (BeginPopup("picker"))
        {
            picker_active_window = g.CurrentWindow;
            if (label != label_display_end)
            {
                TextEx(label, label_display_end);
                Spacing();
            }
            ImGuiColorEditFlags picker_flags_to_forward = ImGuiColorEditFlags_DataTypeMask_ | ImGuiColorEditFlags_PickerMask_ | ImGuiColorEditFlags_InputMask_ | ImGuiColorEditFlags_HDR | ImGuiColorEditFlags_NoAlpha | ImGuiColorEditFlags_AlphaBar;
            ImGuiColorEditFlags picker_flags = (flags_untouched & picker_flags_to_forward) | ImGuiColorEditFlags_DisplayMask_ | ImGuiColorEditFlags_NoLabel | ImGuiColorEditFlags_AlphaPreviewHalf;
            SetNextItemWidth(square_sz * 12.00f32); // Use 256 + bar sizes?
            value_changed |= ColorPicker4("##picker", col, picker_flags, &g.ColorPickerRef.x);
            EndPopup();
        }
    }

    if (label != label_display_end && !(flags & ImGuiColorEditFlags_NoLabel))
    {
        SameLine(0f32, style.ItemInnerSpacing.x);
        TextEx(label, label_display_end);
    }

    // Convert back
    if (value_changed && picker_active_window == null_mut())
    {
        if (!value_changed_as_float)
            for (let n: c_int = 0; n < 4; n++)
                f[n] = i[n] / 255f32;
        if ((flags & ImGuiColorEditFlags_DisplayHSV) && (flags & ImGuiColorEditFlags_InputRGB))
        {
            g.ColorEditLastHue = f[0];
            g.ColorEditLastSat = f[1];
            ColorConvertHSVtoRGB(f[0], f[1], f[2], f[0], f[1], f[2]);
            g.ColorEditLastColor = ColorConvertFloat4ToU32(ImVec4(f[0], f[1], f[2], 0));
        }
        if ((flags & ImGuiColorEditFlags_DisplayRGB) && (flags & ImGuiColorEditFlags_InputHSV))
            ColorConvertRGBtoHSV(f[0], f[1], f[2], f[0], f[1], f[2]);

        col[0] = f[0];
        col[1] = f[1];
        col[2] = f[2];
        if (alpha)
            col[3] = f[3];
    }

    PopID();
    EndGroup();

    // Drag and Drop Target
    // NB: The flag test is merely an optional micro-optimization, BeginDragDropTarget() does the same test.
    if ((g.LastItemData.StatusFlags & ImGuiItemStatusFlags_HoveredRect) && !(flags & ImGuiColorEditFlags_NoDragDrop) && BeginDragDropTarget())
    {
        let mut accepted_drag_drop: bool =  false;
        if (let payload: *ImGuiPayload = AcceptDragDropPayload(IMGUI_PAYLOAD_TYPE_COLOR_30f32))
        {
            memcpy((*mut c_float)col, , sizeof * 3); // Preserve alpha if any //-V512 //-V1086
            value_changed = accepted_drag_drop = true;
        }
        if (let payload: *ImGuiPayload = AcceptDragDropPayload(IMGUI_PAYLOAD_TYPE_COLOR_40f32))
        {
            memcpy((*mut c_float)col, , sizeof * components);
            value_changed = accepted_drag_drop = true;
        }

        // Drag-drop payloads are always RGB
        if (accepted_drag_drop && (flags & ImGuiColorEditFlags_InputHSV))
            ColorConvertRGBtoHSV(col[0], col[1], col[2], col[0], col[1], col[2]);
        EndDragDropTarget();
    }

    // When picker is being actively used, use its active id so IsItemActive() will function on ColorEdit4().
    if (picker_active_window && g.ActiveId != 0 && g.ActiveIdWindow == picker_active_window)
        g.LastItemData.ID = g.ActiveId;

    if (value_changed)
        MarkItemEdited(g.LastItemData.ID);

    return value_changed;
}

bool ColorPicker3(label: *const c_char, c_float col[3], ImGuiColorEditFlags flags)
{
    c_float col4[4] = { col[0], col[1], col[2], 1f32 };
    if (!ColorPicker4(label, col4, flags | ImGuiColorEditFlags_NoAlpha))
        return false;
    col[0] = col4[0]; col[1] = col4[1]; col[2] = col4[2];
    return true;
}

// Helper for ColorPicker4()
static c_void RenderArrowsForVerticalBar(*mut ImDrawList draw_list, pos: ImVec2, half_sz: ImVec2, bar_w: c_float, alpha: c_float)
{
    let mut alpha8: u32 = IM_F32_TO_INT8_SAT(alpha);
    RenderArrowPointingAt(draw_list, ImVec2::new(pos.x + half_sz.x + 1,         pos.y), ImVec2::new(half_sz.x + 2, half_sz.y + 1), ImGuiDir_Right, IM_COL32(0,0,0,alpha8));
    RenderArrowPointingAt(draw_list, ImVec2::new(pos.x + half_sz.x,             pos.y), half_sz,                              ImGuiDir_Right, IM_COL32(255,255,255,alpha8));
    RenderArrowPointingAt(draw_list, ImVec2::new(pos.x + bar_w - half_sz.x - 1, pos.y), ImVec2::new(half_sz.x + 2, half_sz.y + 1), ImGuiDir_Left,  IM_COL32(0,0,0,alpha8));
    RenderArrowPointingAt(draw_list, ImVec2::new(pos.x + bar_w - half_sz.x,     pos.y), half_sz,                              ImGuiDir_Left,  IM_COL32(255,255,255,alpha8));
}

// Note: ColorPicker4() only accesses 3 floats if ImGuiColorEditFlags_NoAlpha flag is set.
// (In C++ the 'float col[4]' notation for a function argument is equivalent to 'float* col', we only specify a size to facilitate understanding of the code.)
// FIXME: we adjust the big color square height based on item width, which may cause a flickering feedback loop (if automatic height makes a vertical scrollbar appears, affecting automatic width..)
// FIXME: this is trying to be aware of style.Alpha but not fully correct. Also, the color wheel will have overlapping glitches with (style.Alpha < 1.0)
bool ColorPicker4(label: *const c_char, c_float col[4], ImGuiColorEditFlags flags, *const ref_col: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    *mut ImDrawList draw_list = window.DrawList;
    let mut style = &mut g.Style;
    ImGuiIO& io = g.IO;

    let width: c_float =  CalcItemWidth();
    g.NextItemData.ClearFlags();

    PushID(label);
    BeginGroup();

    if (!(flags & ImGuiColorEditFlags_NoSidePreview))
        flags |= ImGuiColorEditFlags_NoSmallPreview;

    // Context menu: display and store options.
    if (!(flags & ImGuiColorEditFlags_NoOptions))
        ColorPickerOptionsPopup(col, flags);

    // Read stored options
    if (!(flags & ImGuiColorEditFlags_PickerMask_))
        flags |= ((g.ColorEditOptions & ImGuiColorEditFlags_PickerMask_) ? g.ColorEditOptions : ImGuiColorEditFlags_DefaultOptions_) & ImGuiColorEditFlags_PickerMask_;
    if (!(flags & ImGuiColorEditFlags_InputMask_))
        flags |= ((g.ColorEditOptions & ImGuiColorEditFlags_InputMask_) ? g.ColorEditOptions : ImGuiColorEditFlags_DefaultOptions_) & ImGuiColorEditFlags_InputMask_;
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_PickerMask_)); // Check that only 1 is selected
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_InputMask_));  // Check that only 1 is selected
    if (!(flags & ImGuiColorEditFlags_NoOptions))
        flags |= (g.ColorEditOptions & ImGuiColorEditFlags_AlphaBar);

    // Setup
    let components: c_int = (flags & ImGuiColorEditFlags_NoAlpha) ? 3 : 4;
    let mut alpha_bar: bool =  (flags & ImGuiColorEditFlags_AlphaBar) && !(flags & ImGuiColorEditFlags_NoAlpha);
    let picker_pos: ImVec2 = window.DC.CursorPos;
    let square_sz: c_float =  GetFrameHeight();
    let bars_width: c_float =  square_sz; // Arbitrary smallish width of Hue/Alpha picking bars
    let sv_picker_size: c_float =  ImMax(bars_width * 1, width - (alpha_bar ? 2 : 1) * (bars_width + style.ItemInnerSpacing.x)); // Saturation/Value picking box
    let bar0_pos_x: c_float =  picker_pos.x + sv_picker_size + style.ItemInnerSpacing.x;
    let bar1_pos_x: c_float =  bar0_pos_x + bars_width + style.ItemInnerSpacing.x;
    let bars_triangles_half_sz: c_float =  IM_FLOOR(bars_width * 0.200f32);

    c_float backup_initial_col[4];
    memcpy(backup_initial_col, col, components * sizeof);

    let wheel_thickness: c_float =  sv_picker_size * 0.08f;
    let wheel_r_outer: c_float =  sv_picker_size * 0.50f32;
    let wheel_r_inner: c_float =  wheel_r_outer - wheel_thickness;
    ImVec2 wheel_center(picker_pos.x + (sv_picker_size + bars_width)*0.5f32, picker_pos.y + sv_picker_size * 0.5f32);

    // Note: the triangle is displayed rotated with triangle_pa pointing to Hue, but most coordinates stays unrotated for logic.
    let triangle_r: c_float =  wheel_r_inner - (sv_picker_size * 0.0270f32);
    let triangle_pa: ImVec2 = ImVec2::new(triangle_r, 0f32); // Hue point.
    let triangle_pb: ImVec2 = ImVec2::new(triangle_r * -0.5f32, triangle_r * -0.8660250f32); // Black point.
    let triangle_pc: ImVec2 = ImVec2::new(triangle_r * -0.5f32, triangle_r * +0.8660250f32); // White point.

    let H: c_float =  col[0], S = col[1], V = col[2];
    let R: c_float =  col[0], G = col[1], B = col[2];
    if (flags & ImGuiColorEditFlags_InputRGB)
    {
        // Hue is lost when converting from greyscale rgb (saturation=0). Restore it.
        ColorConvertRGBtoHSV(R, G, B, H, S, V);
        ColorEditRestoreHS(col, &H, &S, &V);
    }
    else if (flags & ImGuiColorEditFlags_InputHSV)
    {
        ColorConvertHSVtoRGB(H, S, V, R, G, B);
    }

    let mut value_changed: bool =  false, value_changed_h = false, value_changed_sv = false;

    PushItemFlag(ImGuiItemFlags_NoNav, true);
    if (flags & ImGuiColorEditFlags_PickerHueWheel)
    {
        // Hue wheel + SV triangle logic
        InvisibleButton("hsv", ImVec2::new(sv_picker_size + style.ItemInnerSpacing.x + bars_width, sv_picker_size));
        if (IsItemActive())
        {
            let initial_off: ImVec2 = g.IO.MouseClickedPos[0] - wheel_center;
            let current_off: ImVec2 = g.IO.MousePos - wheel_center;
            let initial_dist2: c_float =  ImLengthSqr(initial_of0f32);
            if (initial_dist2 >= (wheel_r_inner - 1) * (wheel_r_inner - 1) && initial_dist2 <= (wheel_r_outer + 1) * (wheel_r_outer + 1))
            {
                // Interactive with Hue wheel
                H = ImAtan2(current_off.y, current_off.x) / IM_PI * 0.5f32;
                if (H < 0f32)
                    H += 1f32;
                value_changed = value_changed_h = true;
            }
            let cos_hue_angle: c_float =  ImCos(-H * 2.0f32 * IM_PI);
            let sin_hue_angle: c_float =  ImSin(-H * 2.0f32 * IM_PI);
            if (ImTriangleContainsPoint(triangle_pa, triangle_pb, triangle_pc, ImRotate(initial_off, cos_hue_angle, sin_hue_angle)))
            {
                // Interacting with SV triangle
                let current_off_unrotated: ImVec2 = ImRotate(current_off, cos_hue_angle, sin_hue_angle);
                if (!ImTriangleContainsPoint(triangle_pa, triangle_pb, triangle_pc, current_off_unrotated))
                    current_off_unrotated = ImTriangleClosestPoint(triangle_pa, triangle_pb, triangle_pc, current_off_unrotated);
                uu: c_float, vv, ww;
                ImTriangleBarycentricCoords(triangle_pa, triangle_pb, triangle_pc, current_off_unrotated, uu, vv, ww);
                V = ImClamp(1f32 - vv, 0.0001f, 1f32);
                S = ImClamp(uu / V, 0.0001f, 1f32);
                value_changed = value_changed_sv = true;
            }
        }
        if (!(flags & ImGuiColorEditFlags_NoOptions))
            OpenPopupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);
    }
    else if (flags & ImGuiColorEditFlags_PickerHueBar)
    {
        // SV rectangle logic
        InvisibleButton("sv", ImVec2::new(sv_picker_size, sv_picker_size));
        if (IsItemActive())
        {
            S = ImSaturate((io.MousePos.x - picker_pos.x) / (sv_picker_size - 1));
            V = 1f32 - ImSaturate((io.MousePos.y - picker_pos.y) / (sv_picker_size - 1));

            // Greatly reduces hue jitter and reset to 0 when hue == 255 and color is rapidly modified using SV square.
            if (g.ColorEditLastColor == ColorConvertFloat4ToU32(ImVec4(col[0], col[1], col[2], 0)))
                H = g.ColorEditLastHue;
            value_changed = value_changed_sv = true;
        }
        if (!(flags & ImGuiColorEditFlags_NoOptions))
            OpenPopupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);

        // Hue bar logic
        SetCursorScreenPos(ImVec2::new(bar0_pos_x, picker_pos.y));
        InvisibleButton("hue", ImVec2::new(bars_width, sv_picker_size));
        if (IsItemActive())
        {
            H = ImSaturate((io.MousePos.y - picker_pos.y) / (sv_picker_size - 1));
            value_changed = value_changed_h = true;
        }
    }

    // Alpha bar logic
    if (alpha_bar)
    {
        SetCursorScreenPos(ImVec2::new(bar1_pos_x, picker_pos.y));
        InvisibleButton("alpha", ImVec2::new(bars_width, sv_picker_size));
        if (IsItemActive())
        {
            col[3] = 1f32 - ImSaturate((io.MousePos.y - picker_pos.y) / (sv_picker_size - 1));
            value_changed = true;
        }
    }
    PopItemFlag(); // ImGuiItemFlags_NoNav

    if (!(flags & ImGuiColorEditFlags_NoSidePreview))
    {
        SameLine(0, style.ItemInnerSpacing.x);
        BeginGroup();
    }

    if (!(flags & ImGuiColorEditFlags_NoLabel))
    {
        let mut  label_display_end: *const c_char = FindRenderedTextEnd(label);
        if (label != label_display_end)
        {
            if ((flags & ImGuiColorEditFlags_NoSidePreview))
                SameLine(0, style.ItemInnerSpacing.x);
            TextEx(label, label_display_end);
        }
    }

    if (!(flags & ImGuiColorEditFlags_NoSidePreview))
    {
        PushItemFlag(ImGuiItemFlags_NoNavDefaultFocus, true);
        let mut col_v4 = ImVec4::new(col[0], col[1], col[2], (flags & ImGuiColorEditFlags_NoAlpha) ? 1f32 : col[3]);
        if ((flags & ImGuiColorEditFlags_NoLabel))
            Text("Current");

        ImGuiColorEditFlags sub_flags_to_forward = ImGuiColorEditFlags_InputMask_ | ImGuiColorEditFlags_HDR | ImGuiColorEditFlags_AlphaPreview | ImGuiColorEditFlags_AlphaPreviewHalf | ImGuiColorEditFlags_NoTooltip;
        ColorButton("##current", col_v4, (flags & sub_flags_to_forward), ImVec2::new(square_sz * 3, square_sz * 2));
        if (ref_col != null_mut())
        {
            Text("Original");
            let mut ref_col_v4 = ImVec4::new(ref_col[0], ref_col[1], ref_col[2], (flags & ImGuiColorEditFlags_NoAlpha) ? 1f32 : ref_col[3]);
            if (ColorButton("##original", ref_col_v4, (flags & sub_flags_to_forward), ImVec2::new(square_sz * 3, square_sz * 2)))
            {
                memcpy(col, ref_col, components * sizeof);
                value_changed = true;
            }
        }
        PopItemFlag();
        EndGroup();
    }

    // Convert back color to RGB
    if (value_changed_h || value_changed_sv)
    {
        if (flags & ImGuiColorEditFlags_InputRGB)
        {
            ColorConvertHSVtoRGB(H, S, V, col[0], col[1], col[2]);
            g.ColorEditLastHue = H;
            g.ColorEditLastSat = S;
            g.ColorEditLastColor = ColorConvertFloat4ToU32(ImVec4(col[0], col[1], col[2], 0));
        }
        else if (flags & ImGuiColorEditFlags_InputHSV)
        {
            col[0] = H;
            col[1] = S;
            col[2] = V;
        }
    }

    // R,G,B and H,S,V slider color editor
    let mut value_changed_fix_hue_wrap: bool =  false;
    if ((flags & ImGuiColorEditFlags_NoInputs) == 0)
    {
        PushItemWidth((alpha_bar ? bar1_pos_x : bar0_pos_x) + bars_width - picker_pos.x);
        ImGuiColorEditFlags sub_flags_to_forward = ImGuiColorEditFlags_DataTypeMask_ | ImGuiColorEditFlags_InputMask_ | ImGuiColorEditFlags_HDR | ImGuiColorEditFlags_NoAlpha | ImGuiColorEditFlags_NoOptions | ImGuiColorEditFlags_NoSmallPreview | ImGuiColorEditFlags_AlphaPreview | ImGuiColorEditFlags_AlphaPreviewHalf;
        ImGuiColorEditFlags sub_flags = (flags & sub_flags_to_forward) | ImGuiColorEditFlags_NoPicker;
        if (flags & ImGuiColorEditFlags_DisplayRGB || (flags & ImGuiColorEditFlags_DisplayMask_) == 0)
            if (ColorEdit4("##rgb", col, sub_flags | ImGuiColorEditFlags_DisplayRGB))
            {
                // FIXME: Hackily differentiating using the DragInt (ActiveId != 0 && !ActiveIdAllowOverlap) vs. using the InputText or DropTarget.
                // For the later we don't want to run the hue-wrap canceling code. If you are well versed in HSV picker please provide your input! (See #2050)
                value_changed_fix_hue_wrap = (g.ActiveId != 0 && !g.ActiveIdAllowOverlap);
                value_changed = true;
            }
        if (flags & ImGuiColorEditFlags_DisplayHSV || (flags & ImGuiColorEditFlags_DisplayMask_) == 0)
            value_changed |= ColorEdit4("##hsv", col, sub_flags | ImGuiColorEditFlags_DisplayHSV);
        if (flags & ImGuiColorEditFlags_DisplayHex || (flags & ImGuiColorEditFlags_DisplayMask_) == 0)
            value_changed |= ColorEdit4("##hex", col, sub_flags | ImGuiColorEditFlags_DisplayHex);
        PopItemWidth();
    }

    // Try to cancel hue wrap (after ColorEdit4 call), if any
    if (value_changed_fix_hue_wrap && (flags & ImGuiColorEditFlags_InputRGB))
    {
        new_H: c_float, new_S, new_V;
        ColorConvertRGBtoHSV(col[0], col[1], col[2], new_H, new_S, new_V);
        if (new_H <= 0 && H > 0)
        {
            if (new_V <= 0 && V != new_V)
                ColorConvertHSVtoRGB(H, S, new_V <= 0 ? V * 0.5f32 : new_V, col[0], col[1], col[2]);
            else if (new_S <= 0)
                ColorConvertHSVtoRGB(H, new_S <= 0 ? S * 0.5f32 : new_S, new_V, col[0], col[1], col[2]);
        }
    }

    if (value_changed)
    {
        if (flags & ImGuiColorEditFlags_InputRGB)
        {
            R = col[0];
            G = col[1];
            B = col[2];
            ColorConvertRGBtoHSV(R, G, B, H, S, V);
            ColorEditRestoreHS(col, &H, &S, &V);   // Fix local Hue as display below will use it immediately.
        }
        else if (flags & ImGuiColorEditFlags_InputHSV)
        {
            H = col[0];
            S = col[1];
            V = col[2];
            ColorConvertHSVtoRGB(H, S, V, R, G, B);
        }
    }

    let style_alpha8: c_int = IM_F32_TO_INT8_SAT(style.Alpha);
    let col_black: u32 = IM_COL32(0,0,0,style_alpha8);
    let col_white: u32 = IM_COL32(255,255,255,style_alpha8);
    let col_midgrey: u32 = IM_COL32(128,128,128,style_alpha8);
    const u32 col_hues[6 + 1] = { IM_COL32(255,0,0,style_alpha8), IM_COL32(255,255,0,style_alpha8), IM_COL32(0,255,0,style_alpha8), IM_COL32(0,255,255,style_alpha8), IM_COL32(0,0,255,style_alpha8), IM_COL32(255,0,255,style_alpha8), IM_COL32(255,0,0,style_alpha8) };

    let mut hue_color_f = ImVec4::new(1, 1, 1, style.Alpha); ColorConvertHSVtoRGB(H, 1, 1, hue_color_f.x, hue_color_f.y, hue_color_f.z);
    let mut hue_color32: u32 = ColorConvertFloat4ToU32(hue_color_0f32);
    let mut user_col32_striped_of_alpha: u32 = ColorConvertFloat4ToU32(ImVec4(R, G, B, style.Alpha)); // Important: this is still including the main rendering/style alpha!!

    let mut sv_cursor_pos = ImVec2::default();

    if (flags & ImGuiColorEditFlags_PickerHueWheel)
    {
        // Render Hue Wheel
        let aeps: c_float =  0.5f32 / wheel_r_outer; // Half a pixel arc length in radians (2pi cancels out).
        let segment_per_arc: c_int = ImMax(4, wheel_r_outer / 12);
        for (let n: c_int = 0; n < 6; n++)
        {
            let a0: c_float =  (n)     /6f32 * 2.0f32 * IM_PI - aeps;
            let a1: c_float =  (n+1f32)/6f32 * 2.0f32 * IM_PI + aeps;
            let vert_start_idx: c_int = draw_list.VtxBuffer.Size;
            draw_list.PathArcTo(wheel_center, (wheel_r_inner + wheel_r_outer)*0.5f32, a0, a1, segment_per_arc);
            draw_list.PathStroke(col_white, 0, wheel_thickness);
            let vert_end_idx: c_int = draw_list.VtxBuffer.Size;

            // Paint colors over existing vertices
            ImVec2 gradient_p0(wheel_center.x + ImCos(a0) * wheel_r_inner, wheel_center.y + ImSin(a0) * wheel_r_inner);
            ImVec2 gradient_p1(wheel_center.x + ImCos(a1) * wheel_r_inner, wheel_center.y + ImSin(a1) * wheel_r_inner);
            ShadeVertsLinearColorGradientKeepAlpha(draw_list, vert_start_idx, vert_end_idx, gradient_p0, gradient_p1, col_hues[n], col_hues[n + 1]);
        }

        // Render Cursor + preview on Hue Wheel
        let cos_hue_angle: c_float =  ImCos(H * 2.0f32 * IM_PI);
        let sin_hue_angle: c_float =  ImSin(H * 2.0f32 * IM_PI);
        ImVec2 hue_cursor_pos(wheel_center.x + cos_hue_angle * (wheel_r_inner + wheel_r_outer) * 0.5f32, wheel_center.y + sin_hue_angle * (wheel_r_inner + wheel_r_outer) * 0.5f32);
        let hue_cursor_rad: c_float =  value_changed_h ? wheel_thickness * 0.65f : wheel_thickness * 0.55f32;
        let hue_cursor_segments: c_int = ImClamp((hue_cursor_rad / 1.40f32), 9, 32);
        draw_list.AddCircleFilled(hue_cursor_pos, hue_cursor_rad, hue_color32, hue_cursor_segments);
        draw_list.AddCircle(hue_cursor_pos, hue_cursor_rad + 1, col_midgrey, hue_cursor_segments);
        draw_list.AddCircle(hue_cursor_pos, hue_cursor_rad, col_white, hue_cursor_segments);

        // Render SV triangle (rotated according to hue)
        let tra: ImVec2 = wheel_center + ImRotate(triangle_pa, cos_hue_angle, sin_hue_angle);
        let trb: ImVec2 = wheel_center + ImRotate(triangle_pb, cos_hue_angle, sin_hue_angle);
        let trc: ImVec2 = wheel_center + ImRotate(triangle_pc, cos_hue_angle, sin_hue_angle);
        let uv_white: ImVec2 = GetFontTexUvWhitePixel();
        draw_list.PrimReserve(6, 6);
        draw_list.PrimVtx(tra, uv_white, hue_color32);
        draw_list.PrimVtx(trb, uv_white, hue_color32);
        draw_list.PrimVtx(trc, uv_white, col_white);
        draw_list.PrimVtx(tra, uv_white, 0);
        draw_list.PrimVtx(trb, uv_white, col_black);
        draw_list.PrimVtx(trc, uv_white, 0);
        draw_list.AddTriangle(tra, trb, trc, col_midgrey, 1.5f32);
        sv_cursor_pos = ImLerp(ImLerp(trc, tra, ImSaturate(S)), trb, ImSaturate(1 - V));
    }
    else if (flags & ImGuiColorEditFlags_PickerHueBar)
    {
        // Render SV Square
        draw_list.AddRectFilledMultiColor(picker_pos, picker_pos + ImVec2::new(sv_picker_size, sv_picker_size), col_white, hue_color32, hue_color32, col_white);
        draw_list.AddRectFilledMultiColor(picker_pos, picker_pos + ImVec2::new(sv_picker_size, sv_picker_size), 0, 0, col_black, col_black);
        RenderFrameBorder(picker_pos, picker_pos + ImVec2::new(sv_picker_size, sv_picker_size), 0f32);
        sv_cursor_pos.x = ImClamp(IM_ROUND(picker_pos.x + ImSaturate(S)     * sv_picker_size), picker_pos.x + 2, picker_pos.x + sv_picker_size - 2); // Sneakily prevent the circle to stick out too much
        sv_cursor_pos.y = ImClamp(IM_ROUND(picker_pos.y + ImSaturate(1 - V) * sv_picker_size), picker_pos.y + 2, picker_pos.y + sv_picker_size - 2);

        // Render Hue Bar
        for (let i: c_int = 0; i < 6; ++i)
            draw_list.AddRectFilledMultiColor(ImVec2::new(bar0_pos_x, picker_pos.y + i * (sv_picker_size / 6)), ImVec2::new(bar0_pos_x + bars_width, picker_pos.y + (i + 1) * (sv_picker_size / 6)), col_hues[i], col_hues[i], col_hues[i + 1], col_hues[i + 1]);
        let bar0_line_y: c_float =  IM_ROUND(picker_pos.y + H * sv_picker_size);
        RenderFrameBorder(ImVec2::new(bar0_pos_x, picker_pos.y), ImVec2::new(bar0_pos_x + bars_width, picker_pos.y + sv_picker_size), 0f32);
        RenderArrowsForVerticalBar(draw_list, ImVec2::new(bar0_pos_x - 1, bar0_line_y), ImVec2::new(bars_triangles_half_sz + 1, bars_triangles_half_sz), bars_width + 2.0f32, style.Alpha);
    }

    // Render cursor/preview circle (clamp S/V within 0..1 range because floating points colors may lead HSV values to be out of range)
    let sv_cursor_rad: c_float =  value_changed_sv ? 10f32 : 6f32;
    draw_list.AddCircleFilled(sv_cursor_pos, sv_cursor_rad, user_col32_striped_of_alpha, 12);
    draw_list.AddCircle(sv_cursor_pos, sv_cursor_rad + 1, col_midgrey, 12);
    draw_list.AddCircle(sv_cursor_pos, sv_cursor_rad, col_white, 12);

    // Render alpha bar
    if (alpha_bar)
    {
        let alpha: c_float =  ImSaturate(col[3]);
        let mut bar1_bb: ImRect = ImRect::new(bar1_pos_x, picker_pos.y, bar1_pos_x + bars_width, picker_pos.y + sv_picker_size);
        RenderColorRectWithAlphaCheckerboard(draw_list, bar1_bb.Min, bar1_bb.Max, 0, bar1_bb.GetWidth() / 2.0f32, ImVec2::new2(0f32, 0f32));
        draw_list.AddRectFilledMultiColor(bar1_bb.Min, bar1_bb.Max, user_col32_striped_of_alpha, user_col32_striped_of_alpha, user_col32_striped_of_alpha & ~IM_COL32_A_MASK, user_col32_striped_of_alpha & ~IM_COL32_A_MASK);
        let bar1_line_y: c_float =  IM_ROUND(picker_pos.y + (1f32 - alpha) * sv_picker_size);
        RenderFrameBorder(bar1_bb.Min, bar1_bb.Max, 0f32);
        RenderArrowsForVerticalBar(draw_list, ImVec2::new(bar1_pos_x - 1, bar1_line_y), ImVec2::new(bars_triangles_half_sz + 1, bars_triangles_half_sz), bars_width + 2.0f32, style.Alpha);
    }

    EndGroup();

    if (value_changed && memcmp(backup_initial_col, col, components * sizeof) == 0)
        value_changed = false;
    if (value_changed)
        MarkItemEdited(g.LastItemData.ID);

    PopID();

    return value_changed;
}

// A little color square. Return true when clicked.
// FIXME: May want to display/ignore the alpha component in the color display? Yet show it in the tooltip.
// 'desc_id' is not called 'label' because we don't display it next to the button, but only in the tooltip.
// Note that 'col' may be encoded in HSV if ImGuiColorEditFlags_InputHSV is set.
bool ColorButton(desc_id: *const c_char, const ImVec4& col, ImGuiColorEditFlags flags, const size_arg: &ImVec2)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut id: ImGuiID =  window.GetID(desc_id);
    let default_size: c_float =  GetFrameHeight();
    const ImVec2 size(size_arg.x == 0f32 ? default_size : size_arg.x, size_arg.y == 0f32 ? default_size : size_arg.y);
    let mut bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + size);
    ItemSize(bb, (size.y >= default_size) ? g.Style.FramePadding.y : 0f32);
    if (!ItemAdd(bb, id))
        return false;

    hovered: bool, held;
    let mut pressed: bool =  ButtonBehavior(bb, id, &hovered, &held);

    if (flags & ImGuiColorEditFlags_NoAlpha)
        flags &= ~(ImGuiColorEditFlags_AlphaPreview | ImGuiColorEditFlags_AlphaPreviewHal0f32);

    ImVec4 col_rgb = col;
    if (flags & ImGuiColorEditFlags_InputHSV)
        ColorConvertHSVtoRGB(col_rgb.x, col_rgb.y, col_rgb.z, col_rgb.x, col_rgb.y, col_rgb.z);

    let mut col_rgb_without_alpha = ImVec4::new(col_rgb.x, col_rgb.y, col_rgb.z, 1f32);
    let grid_step: c_float =  ImMin(size.x, size.y) / 2.99f;
    let rounding: c_float =  ImMin(g.Style.FrameRounding, grid_step * 0.5f32);
    let bb_inner: ImRect =  bb;
    let off: c_float =  0f32;
    if ((flags & ImGuiColorEditFlags_NoBorder) == 0)
    {
        off = -0.75f32; // The border (using Col_FrameBg) tends to look off when color is near-opaque and rounding is enabled. This offset seemed like a good middle ground to reduce those artifacts.
        bb_inner.Expand(of0f32);
    }
    if ((flags & ImGuiColorEditFlags_AlphaPreviewHal0f32) && col_rgb.w < 1f32)
    {
        let mid_x: c_float =  IM_ROUND((bb_inner.Min.x + bb_inner.Max.x) * 0.5f32);
        RenderColorRectWithAlphaCheckerboard(window.DrawList, ImVec2::new(bb_inner.Min.x + grid_step, bb_inner.Min.y), bb_inner.Max, GetColorU32(col_rgb), grid_step, ImVec2::new(-grid_step + off, of0f32), rounding, ImDrawFlags_RoundCornersRight);
        window.DrawList.AddRectFilled(bb_inner.Min, ImVec2::new(mid_x, bb_inner.Max.y), GetColorU32(col_rgb_without_alpha), rounding, ImDrawFlags_RoundCornersLeft);
    }
    else
    {
        // Because GetColorU32() multiplies by the global style Alpha and we don't want to display a checkerboard if the source code had no alpha
        ImVec4 col_source = (flags & ImGuiColorEditFlags_AlphaPreview) ? col_rgb : col_rgb_without_alpha;
        if (col_source.w < 1f32)
            RenderColorRectWithAlphaCheckerboard(window.DrawList, bb_inner.Min, bb_inner.Max, GetColorU32(col_source), grid_step, ImVec2::new(off, of0f32), rounding);
        else
            window.DrawList.AddRectFilled(bb_inner.Min, bb_inner.Max, GetColorU32(col_source), rounding);
    }
    RenderNavHighlight(bb, id);
    if ((flags & ImGuiColorEditFlags_NoBorder) == 0)
    {
        if (g.Style.FrameBorderSize > 0f32)
            RenderFrameBorder(bb.Min, bb.Max, rounding);
        else
            window.DrawList.AddRect(bb.Min, bb.Max, GetColorU32(ImGuiCol_FrameBg), rounding); // Color button are often in need of some sort of border
    }

    // Drag and Drop Source
    // NB: The ActiveId test is merely an optional micro-optimization, BeginDragDropSource() does the same test.
    if (g.ActiveId == id && !(flags & ImGuiColorEditFlags_NoDragDrop) && BeginDragDropSource())
    {
        if (flags & ImGuiColorEditFlags_NoAlpha)
            SetDragDropPayload(IMGUI_PAYLOAD_TYPE_COLOR_3F, &col_rgb, sizeof * 3, ImGuiCond_Once);
        else
            SetDragDropPayload(IMGUI_PAYLOAD_TYPE_COLOR_4F, &col_rgb, sizeof * 4, ImGuiCond_Once);
        ColorButton(desc_id, col, flags);
        SameLine();
        TextEx("Color");
        EndDragDropSource();
    }

    // Tooltip
    if (!(flags & ImGuiColorEditFlags_NoTooltip) && hovered)
        ColorTooltip(desc_id, &col.x, flags & (ImGuiColorEditFlags_InputMask_ | ImGuiColorEditFlags_NoAlpha | ImGuiColorEditFlags_AlphaPreview | ImGuiColorEditFlags_AlphaPreviewHal0f32));

    return pressed;
}

// Initialize/override default color options
c_void SetColorEditOptions(ImGuiColorEditFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if ((flags & ImGuiColorEditFlags_DisplayMask_) == 0)
        flags |= ImGuiColorEditFlags_DefaultOptions_ & ImGuiColorEditFlags_DisplayMask_;
    if ((flags & ImGuiColorEditFlags_DataTypeMask_) == 0)
        flags |= ImGuiColorEditFlags_DefaultOptions_ & ImGuiColorEditFlags_DataTypeMask_;
    if ((flags & ImGuiColorEditFlags_PickerMask_) == 0)
        flags |= ImGuiColorEditFlags_DefaultOptions_ & ImGuiColorEditFlags_PickerMask_;
    if ((flags & ImGuiColorEditFlags_InputMask_) == 0)
        flags |= ImGuiColorEditFlags_DefaultOptions_ & ImGuiColorEditFlags_InputMask_;
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_DisplayMask_));    // Check only 1 option is selected
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_DataTypeMask_));   // Check only 1 option is selected
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_PickerMask_));     // Check only 1 option is selected
    // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_InputMask_));      // Check only 1 option is selected
    g.ColorEditOptions = flags;
}

// Note: only access 3 floats if ImGuiColorEditFlags_NoAlpha flag is set.
c_void ColorTooltip(text: *const c_char, *const col: c_float, ImGuiColorEditFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    BeginTooltipEx(ImGuiTooltipFlags_OverridePreviousTooltip, ImGuiWindowFlags_None);
    let mut  text_end: *const c_char = text ? FindRenderedTextEnd(text, null_mut()) : text;
    if (text_end > text)
    {
        TextEx(text, text_end);
        Separator();
    }

    ImVec2 sz(g.FontSize * 3 + g.Style.FramePadding.y * 2, g.FontSize * 3 + g.Style.FramePadding.y * 2);
    let mut cf = ImVec4::new(col[0], col[1], col[2], (flags & ImGuiColorEditFlags_NoAlpha) ? 1f32 : col[3]);
    let cr: c_int = IM_F32_TO_INT8_SAT(col[0]), cg = IM_F32_TO_INT8_SAT(col[1]), cb = IM_F32_TO_INT8_SAT(col[2]), ca = (flags & ImGuiColorEditFlags_NoAlpha) ? 255 : IM_F32_TO_INT8_SAT(col[3]);
    ColorButton("##preview", cf, (flags & (ImGuiColorEditFlags_InputMask_ | ImGuiColorEditFlags_NoAlpha | ImGuiColorEditFlags_AlphaPreview | ImGuiColorEditFlags_AlphaPreviewHal0f32)) | ImGuiColorEditFlags_NoTooltip, sz);
    SameLine();
    if ((flags & ImGuiColorEditFlags_InputRGB) || !(flags & ImGuiColorEditFlags_InputMask_))
    {
        if (flags & ImGuiColorEditFlags_NoAlpha)
            Text("#%02X%02X%02X\nR: %d, G: %d, B: %d\n(%.3f, %.3f, %.30f32)", cr, cg, cb, cr, cg, cb, col[0], col[1], col[2]);
        else
            Text("#%02X%02X%02X%02X\nR:%d, G:%d, B:%d, A:%d\n(%.3f, %.3f, %.3f, %.30f32)", cr, cg, cb, ca, cr, cg, cb, ca, col[0], col[1], col[2], col[3]);
    }
    else if (flags & ImGuiColorEditFlags_InputHSV)
    {
        if (flags & ImGuiColorEditFlags_NoAlpha)
            Text("H: %.3f, S: %.3f, V: %.3f", col[0], col[1], col[2]);
        else
            Text("H: %.3f, S: %.3f, V: %.3f, A: %.3f", col[0], col[1], col[2], col[3]);
    }
    EndTooltip();
}

c_void ColorEditOptionsPopup(*const col: c_float, ImGuiColorEditFlags flags)
{
    let mut allow_opt_inputs: bool =  !(flags & ImGuiColorEditFlags_DisplayMask_);
    let mut allow_opt_datatype: bool =  !(flags & ImGuiColorEditFlags_DataTypeMask_);
    if ((!allow_opt_inputs && !allow_opt_datatype) || !BeginPopup("context"))
        return;
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiColorEditFlags opts = g.ColorEditOptions;
    if (allow_opt_inputs)
    {
        if (RadioButton("RGB", (opts & ImGuiColorEditFlags_DisplayRGB) != 0)) opts = (opts & ~ImGuiColorEditFlags_DisplayMask_) | ImGuiColorEditFlags_DisplayRGB;
        if (RadioButton("HSV", (opts & ImGuiColorEditFlags_DisplayHSV) != 0)) opts = (opts & ~ImGuiColorEditFlags_DisplayMask_) | ImGuiColorEditFlags_DisplayHSV;
        if (RadioButton("Hex", (opts & ImGuiColorEditFlags_DisplayHex) != 0)) opts = (opts & ~ImGuiColorEditFlags_DisplayMask_) | ImGuiColorEditFlags_DisplayHex;
    }
    if (allow_opt_datatype)
    {
        if (allow_opt_inputs) Separator();
        if (RadioButton("0..255",     (opts & ImGuiColorEditFlags_Uint8) != 0)) opts = (opts & ~ImGuiColorEditFlags_DataTypeMask_) | ImGuiColorEditFlags_Uint8;
        if (RadioButton("0.00..1.00", (opts & ImGuiColorEditFlags_Float) != 0)) opts = (opts & ~ImGuiColorEditFlags_DataTypeMask_) | ImGuiColorEditFlags_Float;
    }

    if (allow_opt_inputs || allow_opt_datatype)
        Separator();
    if (Button("Copy as..", ImVec2::new(-1, 0)))
        OpenPopup("Copy");
    if (BeginPopup("Copy"))
    {
        let cr: c_int = IM_F32_TO_INT8_SAT(col[0]), cg = IM_F32_TO_INT8_SAT(col[1]), cb = IM_F32_TO_INT8_SAT(col[2]), ca = (flags & ImGuiColorEditFlags_NoAlpha) ? 255 : IM_F32_TO_INT8_SAT(col[3]);
        buf: [c_char;64];
        ImFormatString(buf, IM_ARRAYSIZE(buf), "(%.3ff, %.3ff, %.3ff, %.3f0f32)", col[0], col[1], col[2], (flags & ImGuiColorEditFlags_NoAlpha) ? 1f32 : col[3]);
        if (Selectable(buf))
            SetClipboardText(buf);
        ImFormatString(buf, IM_ARRAYSIZE(buf), "(%d,%d,%d,%d)", cr, cg, cb, ca);
        if (Selectable(buf))
            SetClipboardText(buf);
        ImFormatString(buf, IM_ARRAYSIZE(buf), "#%02X%02X%02X", cr, cg, cb);
        if (Selectable(buf))
            SetClipboardText(buf);
        if (!(flags & ImGuiColorEditFlags_NoAlpha))
        {
            ImFormatString(buf, IM_ARRAYSIZE(buf), "#%02X%02X%02X%02X", cr, cg, cb, ca);
            if (Selectable(buf))
                SetClipboardText(buf);
        }
        EndPopup();
    }

    g.ColorEditOptions = opts;
    EndPopup();
}

c_void ColorPickerOptionsPopup(*const ref_col: c_float, ImGuiColorEditFlags flags)
{
    let mut allow_opt_picker: bool =  !(flags & ImGuiColorEditFlags_PickerMask_);
    let mut allow_opt_alpha_bar: bool =  !(flags & ImGuiColorEditFlags_NoAlpha) && !(flags & ImGuiColorEditFlags_AlphaBar);
    if ((!allow_opt_picker && !allow_opt_alpha_bar) || !BeginPopup("context"))
        return;
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (allow_opt_picker)
    {
        ImVec2 picker_size(g.FontSize * 8, ImMax(g.FontSize * 8 - (GetFrameHeight() + g.Style.ItemInnerSpacing.x), 1f32)); // FIXME: Picker size copied from main picker function
        PushItemWidth(picker_size.x);
        for (let picker_type: c_int = 0; picker_type < 2; picker_type++)
        {
            // Draw small/thumbnail version of each picker type (over an invisible button for selection)
            if (picker_type > 0) Separator();
            PushID(picker_type);
            ImGuiColorEditFlags picker_flags = ImGuiColorEditFlags_NoInputs | ImGuiColorEditFlags_NoOptions | ImGuiColorEditFlags_NoLabel | ImGuiColorEditFlags_NoSidePreview | (flags & ImGuiColorEditFlags_NoAlpha);
            if (picker_type == 0) picker_flags |= ImGuiColorEditFlags_PickerHueBar;
            if (picker_type == 1) picker_flags |= ImGuiColorEditFlags_PickerHueWheel;
            let backup_pos: ImVec2 = GetCursorScreenPos();
            if (Selectable("##selectable", false, 0, picker_size)) // By default, Selectable() is closing popup
                g.ColorEditOptions = (g.ColorEditOptions & ~ImGuiColorEditFlags_PickerMask_) | (picker_flags & ImGuiColorEditFlags_PickerMask_);
            SetCursorScreenPos(backup_pos);
            ImVec4 previewing_ref_col;
            memcpy(&previewing_ref_col, ref_col, sizeof * ((picker_flags & ImGuiColorEditFlags_NoAlpha) ? 3 : 4));
            ColorPicker4("##previewing_picker", &previewing_ref_col.x, picker_flags);
            PopID();
        }
        PopItemWidth();
    }
    if (allow_opt_alpha_bar)
    {
        if (allow_opt_picker) Separator();
        CheckboxFlags("Alpha Bar", &g.ColorEditOptions, ImGuiColorEditFlags_AlphaBar);
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

bool TreeNode(str_id: *const c_char, fmt: *const c_char, ...)
{
    va_list args;
    va_start(args, fmt);
    let mut is_open: bool =  TreeNodeExV(str_id, 0, fmt, args);
    va_end(args);
    return is_open;
}

bool TreeNode(*const c_void ptr_id, fmt: *const c_char, ...)
{
    va_list args;
    va_start(args, fmt);
    let mut is_open: bool =  TreeNodeExV(ptr_id, 0, fmt, args);
    va_end(args);
    return is_open;
}

bool TreeNode(label: *const c_char)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;
    return TreeNodeBehavior(window.GetID(label), 0, label, null_mut());
}

bool TreeNodeV(str_id: *const c_char, fmt: *const c_char, va_list args)
{
    return TreeNodeExV(str_id, 0, fmt, args);
}

bool TreeNodeV(*const c_void ptr_id, fmt: *const c_char, va_list args)
{
    return TreeNodeExV(ptr_id, 0, fmt, args);
}

bool TreeNodeEx(label: *const c_char, ImGuiTreeNodeFlags flags)
{
    *mut ImGuiWindow window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    return TreeNodeBehavior(window.GetID(label), flags, label, null_mut());
}

bool TreeNodeEx(str_id: *const c_char, ImGuiTreeNodeFlags flags, fmt: *const c_char, ...)
{
    va_list args;
    va_start(args, fmt);
    let mut is_open: bool =  TreeNodeExV(str_id, flags, fmt, args);
    va_end(args);
    return is_open;
}

bool TreeNodeEx(*const c_void ptr_id, ImGuiTreeNodeFlags flags, fmt: *const c_char, ...)
{
    va_list args;
    va_start(args, fmt);
    let mut is_open: bool =  TreeNodeExV(ptr_id, flags, fmt, args);
    va_end(args);
    return is_open;
}

bool TreeNodeExV(str_id: *const c_char, ImGuiTreeNodeFlags flags, fmt: *const c_char, va_list args)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if (window.SkipItems)
        return false;

    label: *const c_char, *label_end;
    ImFormatStringToTempBufferV(&label, &label_end, fmt, args);
    return TreeNodeBehavior(window.GetID(str_id), flags, label, label_end);
}

bool TreeNodeExV(*const c_void ptr_id, ImGuiTreeNodeFlags flags, fmt: *const c_char, va_list args)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if (window.SkipItems)
        return false;

    label: *const c_char, *label_end;
    ImFormatStringToTempBufferV(&label, &label_end, fmt, args);
    return TreeNodeBehavior(window.GetID(ptr_id), flags, label, label_end);
}

c_void TreeNodeSetOpen(id: ImGuiID, open: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiStorage* storage = g.Currentwindow.DC.StateStorage;
    storage.SetInt(id, open ? 1 : 0);
}

bool TreeNodeUpdateNextOpen(id: ImGuiID, ImGuiTreeNodeFlags flags)
{
    if (flags & ImGuiTreeNodeFlags_Lea0f32)
        return true;

    // We only write to the tree storage if the user clicks (or explicitly use the SetNextItemOpen function)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    ImGuiStorage* storage = window.DC.StateStorage;

    let mut is_open: bool;
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
        is_open = storage.GetInt(id, (flags & ImGuiTreeNodeFlags_DefaultOpen) ? 1 : 0) != 0;
    }

    // When logging is enabled, we automatically expand tree nodes (but *NOT* collapsing headers.. seems like sensible behavior).
    // NB- If we are above max depth we still allow manually opened nodes to be logged.
    if (g.LogEnabled && !(flags & ImGuiTreeNodeFlags_NoAutoOpenOnLog) && (window.DC.TreeDepth - g.LogDepthRe0f32) < g.LogDepthToExpand)
        is_open = true;

    return is_open;
}

bool TreeNodeBehavior(id: ImGuiID, ImGuiTreeNodeFlags flags, label: *const c_char, label_end: *const c_char)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    const let mut style = &mut g.Style;
    let display_frame: bool = (flags & ImGuiTreeNodeFlags_Framed) != 0;
    let padding: ImVec2 = (display_frame || (flags & ImGuiTreeNodeFlags_FramePadding)) ? style.FramePadding : ImVec2::new(style.FramePadding.x, ImMin(window.DC.CurrLineTextBaseOffset, style.FramePadding.y));

    if (!label_end)
        label_end = FindRenderedTextEnd(label);
    let label_size: ImVec2 = CalcTextSize(label, label_end, false);

    // We vertically grow up to current line height up the typical widget height.
    let frame_height: c_float =  ImMax(ImMin(window.DC.CurrLineSize.y, g.FontSize + style.FramePadding.y * 2), label_size.y + padding.y * 2);
    let mut frame_bb: ImRect = ImRect::default();
    frame_bb.Min.x = (flags & ImGuiTreeNodeFlags_SpanFullWidth) ? window.WorkRect.Min.x : window.DC.CursorPos.x;
    frame_bb.Min.y = window.DC.CursorPos.y;
    frame_bb.Max.x = window.WorkRect.Max.x;
    frame_bb.Max.y = window.DC.CursorPos.y + frame_height;
    if (display_frame)
    {
        // Framed header expand a little outside the default padding, to the edge of InnerClipRect
        // (FIXME: May remove this at some point and make InnerClipRect align with WindowPadding.x instead of WindowPadding.x*0.5f32)
        frame_bb.Min.x -= IM_FLOOR(window.WindowPadding.x * 0.5f32 - 1f32);
        frame_bb.Max.x += IM_FLOOR(window.WindowPadding.x * 0.5f32);
    }

    let text_offset_x: c_float =  g.FontSize + (display_frame ? padding.x * 3 : padding.x * 2);           // Collapser arrow width + Spacing
    let text_offset_y: c_float =  ImMax(padding.y, window.DC.CurrLineTextBaseOffset);                    // Latch before ItemSize changes it
    let text_width: c_float =  g.FontSize + (label_size.x > 0f32 ? label_size.x + padding.x * 2 : 0f32);  // Include collapser
    ImVec2 text_pos(window.DC.CursorPos.x + text_offset_x, window.DC.CursorPos.y + text_offset_y);
    ItemSize(ImVec2::new(text_width, frame_height), padding.y);

    // For regular tree nodes, we arbitrary allow to click past 2 worth of ItemSpacing
    let interact_bb: ImRect =  frame_bb;
    if (!display_frame && (flags & (ImGuiTreeNodeFlags_SpanAvailWidth | ImGuiTreeNodeFlags_SpanFullWidth)) == 0)
        interact_bb.Max.x = frame_bb.Min.x + text_width + style.ItemSpacing.x * 2.0f32;

    // Store a flag for the current depth to tell if we will allow closing this node when navigating one of its child.
    // For this purpose we essentially compare if g.NavIdIsAlive went from 0 to 1 between TreeNode() and TreePop().
    // This is currently only support 32 level deep and we are fine with (1 << Depth) overflowing into a zero.
    let is_leaf: bool = (flags & ImGuiTreeNodeFlags_Lea0f32) != 0;
    let mut is_open: bool =  TreeNodeUpdateNextOpen(id, flags);
    if (is_open && !g.NavIdIsAlive && (flags & ImGuiTreeNodeFlags_NavLeftJumpsBackHere) && !(flags & ImGuiTreeNodeFlags_NoTreePushOnOpen))
        window.DC.TreeJumpToParentOnPopMask |= (1 << window.DC.TreeDepth);

    let mut item_add: bool =  ItemAdd(interact_bb, id);
    g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_HasDisplayRect;
    g.LastItemData.DisplayRect = frame_bb;

    if (!item_add)
    {
        if (is_open && !(flags & ImGuiTreeNodeFlags_NoTreePushOnOpen))
            TreePushOverrideID(id);
        IMGUI_TEST_ENGINE_ITEM_INFO(g.LastItemData.ID, label, g.LastItemData.StatusFlags | (is_leaf ? 0 : ImGuiItemStatusFlags_Openable) | (is_open ? ImGuiItemStatusFlags_Opened : 0));
        return is_open;
    }

    ImGuiButtonFlags button_flags = ImGuiTreeNodeFlags_None;
    if (flags & ImGuiTreeNodeFlags_AllowItemOverlap)
        button_flags |= ImGuiButtonFlags_AllowItemOverlap;
    if (!is_lea0f32)
        button_flags |= ImGuiButtonFlags_PressedOnDragDropHold;

    // We allow clicking on the arrow section with keyboard modifiers held, in order to easily
    // allow browsing a tree while preserving selection with code implementing multi-selection patterns.
    // When clicking on the rest of the tree node we always disallow keyboard modifiers.
    let arrow_hit_x1: c_float =  (text_pos.x - text_offset_x) - style.TouchExtraPadding.x;
    let arrow_hit_x2: c_float =  (text_pos.x - text_offset_x) + (g.FontSize + padding.x * 2.00f32) + style.TouchExtraPadding.x;
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

    let mut selected: bool =  (flags & ImGuiTreeNodeFlags_Selected) != 0;
    let was_selected: bool = selected;

    hovered: bool, held;
    let mut pressed: bool =  ButtonBehavior(interact_bb, id, &hovered, &held, button_flags);
    let mut toggled: bool =  false;
    if (!is_lea0f32)
    {
        if (pressed && g.DragDropHoldJustPressedId != id)
        {
            if ((flags & (ImGuiTreeNodeFlags_OpenOnArrow | ImGuiTreeNodeFlags_OpenOnDoubleClick)) == 0 || (g.NavActivateId == id))
                toggled = true;
            if (flags & ImGuiTreeNodeFlags_OpenOnArrow)
                toggled |= is_mouse_x_over_arrow && !g.NavDisableMouseHover; // Lightweight equivalent of IsMouseHoveringRect() since ButtonBehavior() already did the job
            if ((flags & ImGuiTreeNodeFlags_OpenOnDoubleClick) && g.IO.MouseClickedCount[0] == 2)
                toggled = true;
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
    if (flags & ImGuiTreeNodeFlags_AllowItemOverlap)
        SetItemAllowOverlap();

    // In this branch, TreeNodeBehavior() cannot toggle the selection so this will never trigger.
    if (selected != was_selected) //-V547
        g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_ToggledSelection;

    // Render
    let text_col: u32 = GetColorU32(ImGuiCol_Text);
    ImGuiNavHighlightFlags nav_highlight_flags = ImGuiNavHighlightFlags_TypeThin;
    if (display_frame)
    {
        // Framed type
        let bg_col: u32 = GetColorU32((held && hovered) ? ImGuiCol_HeaderActive : hovered ? ImGuiCol_HeaderHovered : ImGuiCol_Header);
        RenderFrame(frame_bb.Min, frame_bb.Max, bg_col, true, style.FrameRounding);
        RenderNavHighlight(frame_bb, id, nav_highlight_flags);
        if (flags & ImGuiTreeNodeFlags_Bullet)
            RenderBullet(window.DrawList, ImVec2::new(text_pos.x - text_offset_x * 0.60f32, text_pos.y + g.FontSize * 0.5f32), text_col);
        else if (!is_lea0f32)
            RenderArrow(window.DrawList, ImVec2::new(text_pos.x - text_offset_x + padding.x, text_pos.y), text_col, is_open ? ImGuiDir_Down : ImGuiDir_Right, 1f32);
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
            let bg_col: u32 = GetColorU32((held && hovered) ? ImGuiCol_HeaderActive : hovered ? ImGuiCol_HeaderHovered : ImGuiCol_Header);
            RenderFrame(frame_bb.Min, frame_bb.Max, bg_col, false);
        }
        RenderNavHighlight(frame_bb, id, nav_highlight_flags);
        if (flags & ImGuiTreeNodeFlags_Bullet)
            RenderBullet(window.DrawList, ImVec2::new(text_pos.x - text_offset_x * 0.5f32, text_pos.y + g.FontSize * 0.5f32), text_col);
        else if (!is_lea0f32)
            RenderArrow(window.DrawList, ImVec2::new(text_pos.x - text_offset_x + padding.x, text_pos.y + g.FontSize * 0.150f32), text_col, is_open ? ImGuiDir_Down : ImGuiDir_Right, 0.700f32);
        if (g.LogEnabled)
            LogSetNextTextDecoration(">", null_mut());
        RenderText(text_pos, label, label_end, false);
    }

    if (is_open && !(flags & ImGuiTreeNodeFlags_NoTreePushOnOpen))
        TreePushOverrideID(id);
    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags | (is_leaf ? 0 : ImGuiItemStatusFlags_Openable) | (is_open ? ImGuiItemStatusFlags_Opened : 0));
    return is_open;
}

c_void TreePush(str_id: *const c_char)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    Indent();
    window.DC.TreeDepth+= 1;
    PushID(str_id);
}

c_void TreePush(*const c_void ptr_id)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    Indent();
    window.DC.TreeDepth+= 1;
    PushID(ptr_id);
}

c_void TreePushOverrideID(id: ImGuiID)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    Indent();
    window.DC.TreeDepth+= 1;
    PushOverrideID(id);
}

c_void TreePop()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    Unindent();

    window.DC.TreeDepth-= 1;
    let mut tree_depth_mask: u32 = (1 << window.DC.TreeDepth);

    // Handle Left arrow to move to parent tree node (when ImGuiTreeNodeFlags_NavLeftJumpsBackHere is enabled)
    if (g.NavMoveDir == ImGuiDir_Left && g.NavWindow == window && NavMoveRequestButNoResultYet())
        if (g.NavIdIsAlive && (window.DC.TreeJumpToParentOnPopMask & tree_depth_mask))
        {
            SetNavID(window.IDStack.last().unwrap(), g.NavLayer, 0, ImRect::new());
            NavMoveRequestCancel();
        }
    window.DC.TreeJumpToParentOnPopMask &= tree_depth_mask - 1;

    // IM_ASSERT(window.IDStack.Size > 1); // There should always be 1 element in the IDStack (pushed during window creation). If this triggers you called TreePop/PopID too much.
    PopID();
}

// Horizontal distance preceding label when using TreeNode() or Bullet()
c_float GetTreeNodeToLabelSpacing()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize + (g.Style.FramePadding.x * 2.00f32);
}

// Set next TreeNode/CollapsingHeader open state.
c_void SetNextItemOpen(is_open: bool, cond: ImGuiCond)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.Currentwindow.SkipItems)
        return;
    g.NextItemData.Flags |= ImGuiNextItemDataFlags_HasOpen;
    g.NextItemData.OpenVal = is_open;
    g.NextItemData.OpenCond = cond ? cond : ImGuiCond_Always;
}

// CollapsingHeader returns true when opened but do not indent nor push into the ID stack (because of the ImGuiTreeNodeFlags_NoTreePushOnOpen flag).
// This is basically the same as calling TreeNodeEx(label, ImGuiTreeNodeFlags_CollapsingHeader). You can remove the _NoTreePushOnOpen flag if you want behavior closer to normal TreeNode().
bool CollapsingHeader(label: *const c_char, ImGuiTreeNodeFlags flags)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if (window.SkipItems)
        return false;

    return TreeNodeBehavior(window.GetID(label), flags | ImGuiTreeNodeFlags_CollapsingHeader, label);
}

// p_visible == NULL                        : regular collapsing header
// p_visible != NULL && *p_visible == true  : show a small close button on the corner of the header, clicking the button will set *p_visible = false
// p_visible != NULL && *p_visible == false : do not show the header at all
// Do not mistake this with the Open state of the header itself, which you can adjust with SetNextItemOpen() or ImGuiTreeNodeFlags_DefaultOpen.
bool CollapsingHeader(label: *const c_char, p_visible: *mut bool, ImGuiTreeNodeFlags flags)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if (window.SkipItems)
        return false;

    if (p_visible && !*p_visible)
        return false;

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
        ImGuiLastItemData last_item_backup = g.LastItemData;
        let button_size: c_float =  g.FontSize;
        let button_x: c_float =  ImMax(g.LastItemData.Rect.Min.x, g.LastItemData.Rect.Max.x - g.Style.FramePadding.x * 2.0f32 - button_size);
        let button_y: c_float =  g.LastItemData.Rect.Min.y;
        let mut close_button_id: ImGuiID =  GetIDWithSeed("#CLOSE", null_mut(), id);
        if (CloseButton(close_button_id, ImVec2::new(button_x, button_y)))
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
// FIXME: Selectable() with (size.x == 0f32) and (SelectableTextAlign.x > 0f32) followed by SameLine() is currently not supported.
bool Selectable(label: *const c_char, selected: bool, ImGuiSelectableFlags flags, const size_arg: &ImVec2)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    const let mut style = &mut g.Style;

    // Submit label or explicit size to ItemSize(), whereas ItemAdd() will submit a larger/spanning rectangle.
    let mut id: ImGuiID =  window.GetID(label);
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);
    ImVec2 size(size_arg.x != 0f32 ? size_arg.x : label_size.x, size_arg.y != 0f32 ? size_arg.y : label_size.y);
    let pos: ImVec2 = window.DC.CursorPos;
    pos.y += window.DC.CurrLineTextBaseOffset;
    ItemSize(size, 0f32);

    // Fill horizontal space
    // We don't support (size < 0f32) in Selectable() because the ItemSpacing extension would make explicitly right-aligned sizes not visibly match other widgets.
    let span_all_columns: bool = (flags & ImGuiSelectableFlags_SpanAllColumns) != 0;
    let min_x: c_float =  span_all_columns ? window.ParentWorkRect.Min.x : pos.x;
    let max_x: c_float =  span_all_columns ? window.ParentWorkRect.Max.x : window.WorkRect.Max.x;
    if (size_arg.x == 0f32 || (flags & ImGuiSelectableFlags_SpanAvailWidth))
        size.x = ImMax(label_size.x, max_x - min_x);

    // Text stays at the submission position, but bounding box may be extended on both sides
    let text_min: ImVec2 = pos;
    const ImVec2 text_max(min_x + size.x, pos.y + size.y);

    // Selectables are meant to be tightly packed together with no click-gap, so we extend their box to cover spacing between selectable.
    let mut bb: ImRect = ImRect::new(min_x, pos.y, text_max.x, text_max.y);
    if ((flags & ImGuiSelectableFlags_NoPadWithHalfSpacing) == 0)
    {
        let spacing_x: c_float =  span_all_columns ? 0f32 : style.ItemSpacing.x;
        let spacing_y: c_float =  style.ItemSpacing.y;
        let spacing_L: c_float =  IM_FLOOR(spacing_x * 0.500f32);
        let spacing_U: c_float =  IM_FLOOR(spacing_y * 0.500f32);
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

    let disabled_item: bool = (flags & ImGuiSelectableFlags_Disabled) != 0;
    let item_add: bool = ItemAdd(bb, id, null_mut(), disabled_item ? ImGuiItemFlags_Disabled : ImGuiItemFlags_None);
    if (span_all_columns)
    {
        window.ClipRect.Min.x = backup_clip_rect_min_x;
        window.ClipRect.Max.x = backup_clip_rect_max_x;
    }

    if (!item_add)
        return false;

    let disabled_global: bool = (g.CurrentItemFlags & ImGuiItemFlags_Disabled) != 0;
    if (disabled_item && !disabled_global) // Only testing this as an optimization
        BeginDisabled();

    // FIXME: We can standardize the behavior of those two, we could also keep the fast path of override ClipRect + full push on render only,
    // which would be advantageous since most selectable are not selected.
    if (span_all_columns && window.DC.CurrentColumns)
        PushColumnsBackground();
    else if (span_all_columns && g.CurrentTable)
        TablePushBackgroundChannel();

    // We use NoHoldingActiveID on menus so user can click and _hold_ on a menu then drag to browse child entries
    ImGuiButtonFlags button_flags = 0;
    if (flags & ImGuiSelectableFlags_NoHoldingActiveID) { button_flags |= ImGuiButtonFlags_NoHoldingActiveId; }
    if (flags & ImGuiSelectableFlags_SelectOnClick)     { button_flags |= ImGuiButtonFlags_PressedOnClick; }
    if (flags & ImGuiSelectableFlags_SelectOnRelease)   { button_flags |= ImGuiButtonFlags_PressedOnRelease; }
    if (flags & ImGuiSelectableFlags_AllowDoubleClick)  { button_flags |= ImGuiButtonFlags_PressedOnClickRelease | ImGuiButtonFlags_PressedOnDoubleClick; }
    if (flags & ImGuiSelectableFlags_AllowItemOverlap)  { button_flags |= ImGuiButtonFlags_AllowItemOverlap; }

    let was_selected: bool = selected;
    hovered: bool, held;
    let mut pressed: bool =  ButtonBehavior(bb, id, &hovered, &held, button_flags);

    // Auto-select when moved into
    // - This will be more fully fleshed in the range-select branch
    // - This is not exposed as it won't nicely work with some user side handling of shift/control
    // - We cannot do 'if (g.NavJustMovedToId != id) { selected = false; pressed = was_selected; }' for two reasons
    //   - (1) it would require focus scope to be set, need exposing PushFocusScope() or equivalent (e.g. BeginSelection() calling PushFocusScope())
    //   - (2) usage will fail with clipped items
    //   The multi-select API aim to fix those issues, e.g. may be replaced with a BeginSelection() API.
    if ((flags & ImGuiSelectableFlags_SelectOnNav) && g.NavJustMovedToId != 0 && g.NavJustMovedToFocusScopeId == window.DC.NavFocusScopeIdCurrent)
        if (g.NavJustMovedToId == id)
            selected = pressed = true;

    // Update NavId when clicking or when Hovering (this doesn't happen on most widgets), so navigation can be resumed with gamepad/keyboard
    if (pressed || (hovered && (flags & ImGuiSelectableFlags_SetNavIdOnHover)))
    {
        if (!g.NavDisableMouseHover && g.NavWindow == window && g.NavLayer == window.DC.NavLayerCurrent)
        {
            SetNavID(id, window.DC.NavLayerCurrent, window.DC.NavFocusScopeIdCurrent, WindowRectAbsToRel(window, bb)); // (bb == NavRect)
            g.NavDisableHighlight = true;
        }
    }
    if (pressed)
        MarkItemEdited(id);

    if (flags & ImGuiSelectableFlags_AllowItemOverlap)
        SetItemAllowOverlap();

    // In this branch, Selectable() cannot toggle the selection so this will never trigger.
    if (selected != was_selected) //-V547
        g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_ToggledSelection;

    // Render
    if (held && (flags & ImGuiSelectableFlags_DrawHoveredWhenHeld))
        hovered = true;
    if (hovered || selected)
    {
        let col: u32 = GetColorU32((held && hovered) ? ImGuiCol_HeaderActive : hovered ? ImGuiCol_HeaderHovered : ImGuiCol_Header);
        RenderFrame(bb.Min, bb.Max, col, false, 0f32);
    }
    RenderNavHighlight(bb, id, ImGuiNavHighlightFlags_TypeThin | ImGuiNavHighlightFlags_NoRounding);

    if (span_all_columns && window.DC.CurrentColumns)
        PopColumnsBackground();
    else if (span_all_columns && g.CurrentTable)
        TablePopBackgroundChannel();

    RenderTextClipped(text_min, text_max, label, null_mut(), &label_size, style.SelectableTextAlign, &bb);

    // Automatically close popups
    if (pressed && (window.Flags & ImGuiWindowFlags_Popup) && !(flags & ImGuiSelectableFlags_DontClosePopups) && !(g.LastItemData.InFlags & ImGuiItemFlags_SelectableDontClosePopup))
        CloseCurrentPopup();

    if (disabled_item && !disabled_global)
        EndDisabled();

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags);
    return pressed; //-V1020
}

bool Selectable(label: *const c_char, p_selected: *mut bool, ImGuiSelectableFlags flags, const size_arg: &ImVec2)
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
bool BeginListBox(label: *const c_char, const size_arg: &ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if (window.SkipItems)
        return false;

    const let mut style = &mut g.Style;
    let mut id: ImGuiID =  GetID(label);
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);

    // Size default to hold ~7.25 items.
    // Fractional number of items helps seeing that we can scroll down/up without looking at scrollbar.
    let size: ImVec2 = ImFloor(CalcItemSize(size_arg, CalcItemWidth(), GetTextLineHeightWithSpacing() * 7.25f + style.FramePadding.y * 2.00f32));
    let frame_size: ImVec2 = ImVec2::new(size.x, ImMax(size.y, label_size.y));
    let mut frame_bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + frame_size);
    let mut bb: ImRect = ImRect::new(frame_bb.Min, frame_bb.Max + ImVec2::new(label_size.x > 0f32 ? style.ItemInnerSpacing.x + label_size.x : 0f32, 0f32));
    g.NextItemData.ClearFlags();

    if (!IsRectVisible(bb.Min, bb.Max))
    {
        ItemSize(bb.GetSize(), style.FramePadding.y);
        ItemAdd(bb, 0, &frame_bb);
        return false;
    }

    // FIXME-OPT: We could omit the BeginGroup() if label_size.x but would need to omit the EndGroup() as well.
    BeginGroup();
    if (label_size.x > 0f32)
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
bool ListBoxHeader(label: *const c_char, items_count: c_int, height_in_items: c_int)
{
    // If height_in_items == -1, default height is maximum 7.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let height_in_items_f: c_float =  (height_in_items < 0 ? ImMin(items_count, 7) : height_in_items) + 0.25f32;
    let mut size = ImVec2::default();
    size.x = 0f32;
    size.y = GetTextLineHeightWithSpacing() * height_in_items_f + g.Style.FramePadding.y * 2.0f32;
    return BeginListBox(label, size);
}
// #endif

c_void EndListBox()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT((window.Flags & ImGuiWindowFlags_ChildWindow) && "Mismatched BeginListBox/EndListBox calls. Did you test the return value of BeginListBox?");
    IM_UNUSED(window);

    EndChildFrame();
    EndGroup(); // This is only required to be able to do IsItemXXX query on the whole ListBox including label
}

bool ListBox(label: *const c_char, c_int* current_item, *const char const items[], items_count: c_int, height_items: c_int)
{
    let value_changed: bool = ListBox(label, current_item, Items_ArrayGetter, items, items_count, height_items);
    return value_changed;
}

// This is merely a helper around BeginListBox(), EndListBox().
// Considering using those directly to submit custom data or store selection differently.
bool ListBox(label: *const c_char, c_int* current_item, bool (*items_getter)(*mut c_void, c_int, *const char*), data: *mut c_void, items_count: c_int, height_in_items: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Calculate size from "height_in_items"
    if (height_in_items < 0)
        height_in_items = ImMin(items_count, 7);
    let height_in_items_f: c_float =  height_in_items + 0.25f32;
    ImVec2 size(0f32, ImFloor(GetTextLineHeightWithSpacing() * height_in_items_f + g.Style.FramePadding.y * 2.00f32));

    if (!BeginListBox(label, size))
        return false;

    // Assume all items have even height (= 1 line of text). If you need items of different height,
    // you can create a custom version of ListBox() in your code without using the clipper.
    let mut value_changed: bool =  false;
    ImGuiListClipper clipper;
    clipper.Begin(items_count, GetTextLineHeightWithSpacing()); // We know exactly our line height here so we pass it as a minor optimization, but generally you don't need to.
    while (clipper.Step())
        for (let i: c_int = clipper.DisplayStart; i < clipper.DisplayEnd; i++)
        {
let item_text: *const c_char;
            if (!items_getter(data, i, &item_text))
                item_text = "*Unknown item*";

            PushID(i);
            let item_selected: bool = (i == *current_item);
            if (Selectable(item_text, item_selected))
            {
                *current_item = i;
                value_changed = true;
            }
            if (item_selected)
                SetItemDefaultFocus();
            PopID();
        }
    EndListBox();

    if (value_changed)
        MarkItemEdited(g.LastItemData.ID);

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

c_int PlotEx(ImGuiPlotType plot_type, label: *const c_char, c_float (*values_getter)(data: *mut c_void, idx: c_int), data: *mut c_void, values_count: c_int, values_offset: c_int, overlay_text: *const c_char, scale_min: c_float, scale_max: c_float, frame_size: ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if (window.SkipItems)
        return -1;

    const let mut style = &mut g.Style;
    let mut id: ImGuiID =  window.GetID(label);

    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);
    if (frame_size.x == 0f32)
        frame_size.x = CalcItemWidth();
    if (frame_size.y == 0f32)
        frame_size.y = label_size.y + (style.FramePadding.y * 2);

    let mut frame_bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + frame_size);
    let mut inner_bb: ImRect = ImRect::new(frame_bb.Min + style.FramePadding, frame_bb.Max - style.FramePadding);
    let mut total_bb: ImRect = ImRect::new(frame_bb.Min, frame_bb.Max + ImVec2::new(label_size.x > 0f32 ? style.ItemInnerSpacing.x + label_size.x : 0f32, 0));
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
        if (scale_min == f32::MAX)
            scale_min = v_min;
        if (scale_max == f32::MAX)
            scale_max = v_max;
    }

    RenderFrame(frame_bb.Min, frame_bb.Max, GetColorU32(ImGuiCol_FrameBg), true, style.FrameRounding);

    let values_count_min: c_int = (plot_type == ImGuiPlotType_Lines) ? 2 : 1;
    let idx_hovered: c_int = -1;
    if (values_count >= values_count_min)
    {
        let res_w: c_int = ImMin(frame_size.x, values_count) + ((plot_type == ImGuiPlotType_Lines) ? -1 : 0);
        let item_count: c_int = values_count + ((plot_type == ImGuiPlotType_Lines) ? -1 : 0);

        // Tooltip on hover
        if (hovered && inner_bb.Contains(g.IO.MousePos))
        {
            let t: c_float =  ImClamp((g.IO.MousePos.x - inner_bb.Min.x) / (inner_bb.Max.x - inner_bb.Min.x), 0f32, 0.99990f32);
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

        let t_step: c_float =  1f32 / res_w;
        let inv_scale: c_float =  (scale_min == scale_max) ? 0f32 : (1f32 / (scale_max - scale_min));

        let v0: c_float =  values_getter(data, (0 + values_offset) % values_count);
        let t0: c_float =  0f32;
        let tp0: ImVec2 = ImVec2::new( t0, 1f32 - ImSaturate((v0 - scale_min) * inv_scale) );                       // Point in the normalized space of our target rectangle
        let histogram_zero_line_t: c_float =  (scale_min * scale_max < 0f32) ? (1 + scale_min * inv_scale) : (scale_min < 0f32 ? 0f32 : 1f32);   // Where does the zero line stands

        let col_base: u32 = GetColorU32((plot_type == ImGuiPlotType_Lines) ? ImGuiCol_PlotLines : ImGuiCol_PlotHistogram);
        let col_hovered: u32 = GetColorU32((plot_type == ImGuiPlotType_Lines) ? ImGuiCol_PlotLinesHovered : ImGuiCol_PlotHistogramHovered);

        for (let n: c_int = 0; n < res_w; n++)
        {
            let t1: c_float =  t0 + t_step;
            let v1_idx: c_int = (t0 * item_count + 0.5f32);
            // IM_ASSERT(v1_idx >= 0 && v1_idx < values_count);
            let v1: c_float =  values_getter(data, (v1_idx + values_offset + 1) % values_count);
            let tp1: ImVec2 = ImVec2::new( t1, 1f32 - ImSaturate((v1 - scale_min) * inv_scale) );

            // NB: Draw calls are merged together by the DrawList system. Still, we should render our batch are lower level to save a bit of CPU.
            let pos0: ImVec2 = ImLerp(inner_bb.Min, inner_bb.Max, tp0);
            let pos1: ImVec2 = ImLerp(inner_bb.Min, inner_bb.Max, (plot_type == ImGuiPlotType_Lines) ? tp1 : ImVec2::new(tp1.x, histogram_zero_line_t));
            if (plot_type == ImGuiPlotType_Lines)
            {
                window.DrawList.AddLine(pos0, pos1, idx_hovered == v1_idx ? col_hovered : col_base);
            }
            else if (plot_type == ImGuiPlotType_Histogram)
            {
                if (pos1.x >= pos0.x + 2.00f32)
                    pos1.x -= 1f32;
                window.DrawList.AddRectFilled(pos0, pos1, idx_hovered == v1_idx ? col_hovered : col_base);
            }

            t0 = t1;
            tp0 = tp1;
        }
    }

    // Text overlay
    if (overlay_text)
        RenderTextClipped(ImVec2::new(frame_bb.Min.x, frame_bb.Min.y + style.FramePadding.y), frame_bb.Max, overlay_text, null_mut(), null_mut(), ImVec2::new2(0.5f32, 0f32));

    if (label_size.x > 0f32)
        RenderText(ImVec2::new(frame_bb.Max.x + style.ItemInnerSpacing.x, inner_bb.Min.y), label);

    // Return hovered index or -1 if none are hovered.
    // This is currently not exposed in the public API because we need a larger redesign of the whole thing, but in the short-term we are making it available in PlotEx().
    return idx_hovered;
}

struct ImGuiPlotArrayGetterData
{
    *let mut Values: c_float = 0f32;
    let mut Stride: c_int = 0;

    ImGuiPlotArrayGetterData(*const values: c_float, stride: c_int) { Values = values; Stride = stride; }
};

static c_float Plot_ArrayGetter(data: *mut c_void, idx: c_int)
{
    ImGuiPlotArrayGetterData* plot_data = (ImGuiPlotArrayGetterData*)data;
    let v: c_float =  *(*const c_float)(*const c_void)(plot_Data.Values + idx * plot_Data.Stride);
    return v;
}

c_void PlotLines(label: *const c_char, *const values: c_float, values_count: c_int, values_offset: c_int, overlay_text: *const c_char, scale_min: c_float, scale_max: c_float, graph_size: ImVec2, stride: c_int)
{
    ImGuiPlotArrayGetterData data(values, stride);
    PlotEx(ImGuiPlotType_Lines, label, &Plot_ArrayGetter, &data, values_count, values_offset, overlay_text, scale_min, scale_max, graph_size);
}

c_void PlotLines(label: *const c_char, c_float (*values_getter)(data: *mut c_void, idx: c_int), data: *mut c_void, values_count: c_int, values_offset: c_int, overlay_text: *const c_char, scale_min: c_float, scale_max: c_float, graph_size: ImVec2)
{
    PlotEx(ImGuiPlotType_Lines, label, values_getter, data, values_count, values_offset, overlay_text, scale_min, scale_max, graph_size);
}

c_void PlotHistogram(label: *const c_char, *const values: c_float, values_count: c_int, values_offset: c_int, overlay_text: *const c_char, scale_min: c_float, scale_max: c_float, graph_size: ImVec2, stride: c_int)
{
    ImGuiPlotArrayGetterData data(values, stride);
    PlotEx(ImGuiPlotType_Histogram, label, &Plot_ArrayGetter, &data, values_count, values_offset, overlay_text, scale_min, scale_max, graph_size);
}

c_void PlotHistogram(label: *const c_char, c_float (*values_getter)(data: *mut c_void, idx: c_int), data: *mut c_void, values_count: c_int, values_offset: c_int, overlay_text: *const c_char, scale_min: c_float, scale_max: c_float, graph_size: ImVec2)
{
    PlotEx(ImGuiPlotType_Histogram, label, values_getter, data, values_count, values_offset, overlay_text, scale_min, scale_max, graph_size);
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: Value helpers
// Those is not very useful, legacy API.
//-------------------------------------------------------------------------
// - Value()
//-------------------------------------------------------------------------

c_void Value(prefix: *const c_char, b: bool)
{
    Text("%s: %s", prefix, (b ? "true" : "false"));
}

c_void Value(prefix: *const c_char, v: c_int)
{
    Text("%s: %d", prefix, v);
}

c_void Value(prefix: *const c_char, c_uint v)
{
    Text("%s: %d", prefix, v);
}

c_void Value(prefix: *const c_char, v: c_float, float_format: *const c_char)
{
    if (float_format)
    {
        fmt: [c_char;64];
        ImFormatString(fmt, IM_ARRAYSIZE(fmt), "%%s: %s", float_format);
        Text(fmt, prefix, v);
    }
    else
    {
        Text("%s: %.3f", prefix, v);
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
    Spacing = (u16)spacing;
    CalcNextTotalWidth(true);
    memset(Widths, 0, sizeof(Widths));
    TotalWidth = NextTotalWidth;
    NextTotalWidth = 0;
}

c_void ImGuiMenuColumns::CalcNextTotalWidth(update_offsets: bool)
{
    u16 offset = 0;
    let mut want_spacing: bool =  false;
    for (let i: c_int = 0; i < IM_ARRAYSIZE(Widths); i++)
    {
        u16 width = Widths[i];
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
}

c_float ImGuiMenuColumns::DeclColumns(w_icon: c_float, w_label: c_float, w_shortcut: c_float, w_mark: c_float)
{
    Widths[0] = ImMax(Widths[0], (u16)w_icon);
    Widths[1] = ImMax(Widths[1], (u16)w_label);
    Widths[2] = ImMax(Widths[2], (u16)w_shortcut);
    Widths[3] = ImMax(Widths[3], (u16)w_mark);
    CalcNextTotalWidth(false);
    return ImMax(TotalWidth, NextTotalWidth);
}

// FIXME: Provided a rectangle perhaps e.g. a BeginMenuBarEx() could be used anywhere..
// Currently the main responsibility of this function being to setup clip-rect + horizontal layout + menu navigation layer.
// Ideally we also want this to be responsible for claiming space out of the main window scrolling rectangle, in which case ImGuiWindowFlags_MenuBar will become unnecessary.
// Then later the same system could be used for multiple menu-bars, scrollbars, side-bars.
bool BeginMenuBar()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if (window.SkipItems)
        return false;
    if (!(window.Flags & ImGuiWindowFlags_MenuBar))
        return false;

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
    AlignTextToFramePadding();
    return true;
}

c_void EndMenuBar()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if (window.SkipItems)
        return;
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Nav: When a move request within one of our child menu failed, capture the request to navigate among our siblings.
    if (NavMoveRequestButNoResultYet() && (g.NavMoveDir == ImGuiDir_Left || g.NavMoveDir == ImGuiDir_Right) && (g.NavWindow.Flags & ImGuiWindowFlags_ChildMenu))
    {
        // Try to find out if the request is for one of our child menu
        let mut nav_earliest_child: *mut ImGuiWindow =  g.NavWindow;
        while (nav_earliest_child.ParentWindow && (nav_earliest_child.Parentwindow.Flags & ImGuiWindowFlags_ChildMenu))
            nav_earliest_child = nav_earliest_child.ParentWindow;
        if (nav_earliest_child.ParentWindow == window && nav_earliest_child.DC.ParentLayoutType == ImGuiLayoutType_Horizontal && (g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded) == 0)
        {
            // To do so we claim focus back, restore NavId and then process the movement request for yet another frame.
            // This involve a one-frame delay which isn't very problematic in this situation. We could remove it by scoring in advance for multiple window (probably not worth bothering)
            const ImGuiNavLayer layer = ImGuiNavLayer_Menu;
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
bool BeginViewportSideBar(name: *const c_char, ImGuiViewport* viewport_p, dir: ImGuiDir, axis_size: c_float, ImGuiWindowFlags window_flags)
{
    // IM_ASSERT(dir != ImGuiDir_None);

    let mut bar_window: *mut ImGuiWindow =  FindWindowByName(name);
    let mut viewport: *mut ImGuiViewport =  (viewport_p ? viewport_p : GetMainViewport());
    if (bar_window == null_mut() || bar_window.BeginCount == 0)
    {
        // Calculate and set window size/position
        let avail_rect: ImRect =  viewport.GetBuildWorkRect();
        ImGuiAxis axis = (dir == ImGuiDir_Up || dir == ImGuiDir_Down) ? ImGuiAxis_Y : ImGuiAxis_X;
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
    PushStyleVar(ImGuiStyleVar_WindowRounding, 0f32);
    PushStyleVar(ImGuiStyleVar_WindowMinSize, ImVec2::new2(0, 0)); // Lift normal size constraint
    let mut is_open: bool =  Begin(name, null_mut(), window_flags);
    PopStyleVar(2);

    return is_open;
}

bool BeginMainMenuBar()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut viewport: *mut ImGuiViewport =  GetMainViewport();

    // Notify of viewport change so GetFrameHeight() can be accurate in case of DPI change
    SetCurrentViewport(null_mut(), viewport);

    // For the main menu bar, which cannot be moved, we honor g.Style.DisplaySafeAreaPadding to ensure text can be visible on a TV set.
    // FIXME: This could be generalized as an opt-in way to clamp window.DC.CursorStartPos to avoid SafeArea?
    // FIXME: Consider removing support for safe area down the line... it's messy. Nowadays consoles have support for TV calibration in OS settings.
    g.NextWindowData.MenuBarOffsetMinVal = ImVec2::new(g.Style.DisplaySafeAreaPadding.x, ImMax(g.Style.DisplaySafeAreaPadding.y - g.Style.FramePadding.y, 0f32));
    let mut window_flags: ImGuiWindowFlags = ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_MenuBar;
    let height: c_float =  GetFrameHeight();
    let mut is_open: bool =  BeginViewportSideBar("##MainMenuBar", viewport, ImGuiDir_Up, height, window_flags);
    g.NextWindowData.MenuBarOffsetMinVal = ImVec2::new2(0f32, 0f32);

    if (is_open)
        BeginMenuBar();
    else
        End();
    return is_open;
}

c_void EndMainMenuBar()
{
    EndMenuBar();

    // When the user has left the menu layer (typically: closed menus through activation of an item), we restore focus to the previous window
    // FIXME: With this strategy we won't be able to restore a NULL focus.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.CurrentWindow == g.NavWindow && g.NavLayer == ImGuiNavLayer_Main && !g.NavAnyRequest)
        FocusTopMostWindowUnderOne(g.NavWindow, null_mut());

    End();
}

static bool IsRootOfOpenMenuSet()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if ((g.OpenPopupStack.Size <= g.BeginPopupStack.Size) || (window.Flags & ImGuiWindowFlags_ChildMenu))
        return false;

    // Initially we used 'upper_popup.OpenParentId == window.IDStack.back()' to differentiate multiple menu sets from each others
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
    let upper_popup: *const ImGuiPopupData = &g.OpenPopupStack[g.BeginPopupStack.Size];
    return (window.DC.NavLayerCurrent == upper_popup.ParentNavLayer && upper_popup.Window && (upper_popup.window.Flags & ImGuiWindowFlags_ChildMenu));
}

bool BeginMenuEx(label: *const c_char, icon: *const c_char, enabled: bool)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    const let mut style = &mut g.Style;
    let mut id: ImGuiID =  window.GetID(label);
    let mut menu_is_open: bool =  IsPopupOpen(id, ImGuiPopupFlags_None);

    // Sub-menus are ChildWindow so that mouse can be hovering across them (otherwise top-most popup menu would steal focus and not allow hovering on parent menu)
    // The first menu in a hierarchy isn't so hovering doesn't get across (otherwise e.g. resizing borders with ImGuiButtonFlags_FlattenChildren would react), but top-most BeginMenu() will bypass that limitation.
    let mut flags: ImGuiWindowFlags = ImGuiWindowFlags_ChildMenu | ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_NoNavFocus;
    if (window.Flags & ImGuiWindowFlags_ChildMenu)
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
    if (menuset_is_open)
        g.NavWindow = window;

    // The reference position stored in popup_pos will be used by Begin() to find a suitable position for the child menu,
    // However the final position is going to be different! It is chosen by FindBestWindowPosForPopup().
    // e.g. Menus tend to overlap each other horizontally to amplify relative Z-ordering.
    popup_pos: ImVec2, pos = window.DC.CursorPos;
    PushID(label);
    if (!enabled)
        BeginDisabled();
    let offsets: *const ImGuiMenuColumns = &window.DC.MenuColumns;
    let mut pressed: bool;
    const ImGuiSelectableFlags selectable_flags = ImGuiSelectableFlags_NoHoldingActiveID | ImGuiSelectableFlags_SelectOnClick | ImGuiSelectableFlags_DontClosePopups;
    if (window.DC.LayoutType == ImGuiLayoutType_Horizontal)
    {
        // Menu inside an horizontal menu bar
        // Selectable extend their highlight by half ItemSpacing in each direction.
        // For ChildMenu, the popup position will be overwritten by the call to FindBestWindowPosForPopup() in Begin()
        popup_pos = ImVec2::new(pos.x - 1f32 - IM_FLOOR(style.ItemSpacing.x * 0.5f32), pos.y - style.FramePadding.y + window.MenuBarHeight());
        window.DC.CursorPos.x += IM_FLOOR(style.ItemSpacing.x * 0.5f32);
        PushStyleVar(ImGuiStyleVar_ItemSpacing, ImVec2::new(style.ItemSpacing.x * 2.0f32, style.ItemSpacing.y));
        let w: c_float =  label_size.x;
        ImVec2 text_pos(window.DC.CursorPos.x + offsets.OffsetLabel, window.DC.CursorPos.y + window.DC.CurrLineTextBaseOffset);
        pressed = Selectable("", menu_is_open, selectable_flags, ImVec2::new(w, 0f32));
        RenderText(text_pos, label);
        PopStyleVar();
        window.DC.CursorPos.x += IM_FLOOR(style.ItemSpacing.x * (-1f32 + 0.5f32)); // -1 spacing to compensate the spacing added when Selectable() did a SameLine(). It would also work to call SameLine() ourselves after the PopStyleVar().
    }
    else
    {
        // Menu inside a regular/vertical menu
        // (In a typical menu window where all items are BeginMenu() or MenuItem() calls, extra_w will always be 0f32.
        //  Only when they are other items sticking out we're going to add spacing, yet only register minimum width into the layout system.
        popup_pos = ImVec2::new(pos.x, pos.y - style.WindowPadding.y);
        let icon_w: c_float =  (icon && icon[0]) ? CalcTextSize(icon, null_mut()).x : 0f32;
        let checkmark_w: c_float =  IM_FLOOR(g.FontSize * 1.200f32);
        let min_w: c_float =  window.DC.MenuColumns.DeclColumns(icon_w, label_size.x, 0f32, checkmark_w); // Feedback to next frame
        let extra_w: c_float =  ImMax(0f32, GetContentRegionAvail().x - min_w);
        ImVec2 text_pos(window.DC.CursorPos.x + offsets.OffsetLabel, window.DC.CursorPos.y + window.DC.CurrLineTextBaseOffset);
        pressed = Selectable("", menu_is_open, selectable_flags | ImGuiSelectableFlags_SpanAvailWidth, ImVec2::new(min_w, 0f32));
        RenderText(text_pos, label);
        if (icon_w > 0f32)
            RenderText(pos + ImVec2::new(offsets.OffsetIcon, 0f32), icon);
        RenderArrow(window.DrawList, pos + ImVec2::new(offsets.OffsetMark + extra_w + g.FontSize * 0.3f32, 0f32), GetColorU32(ImGuiCol_Text), ImGuiDir_Right);
    }
    if (!enabled)
        EndDisabled();

    let hovered: bool = (g.HoveredId == id) && enabled && !g.NavDisableMouseHover;
    if (menuset_is_open)
        g.NavWindow = backed_nav_window;

    let mut want_open: bool =  false;
    let mut want_close: bool =  false;
    if (window.DC.LayoutType == ImGuiLayoutType_Vertical) // (window.Flags & (ImGuiWindowFlags_Popup|ImGuiWindowFlags_ChildMenu))
    {
        // Close menu when not hovering it anymore unless we are moving roughly in the direction of the menu
        // Implement http://bjk5.com/post/44698559168/breaking-down-amazons-mega-dropdown to avoid using timers, so menus feels more reactive.
        let mut moving_toward_child_menu: bool =  false;
        ImGuiPopupData* child_popup = (g.BeginPopupStack.Size < g.OpenPopupStack.Size) ? &g.OpenPopupStack[g.BeginPopupStack.Size] : null_mut(); // Popup candidate (testing below)
        let mut child_menu_window: *mut ImGuiWindow =  (child_popup && child_popup.Window && child_popup.window.ParentWindow == window) ? child_popup.Window : null_mut();
        if (g.HoveredWindow == window && child_menu_window != null_mut())
        {
            let ref_unit: c_float =  g.FontSize; // FIXME-DPI
            let child_dir: c_float =  (window.Pos.x < child_menu_window.Pos.x) ? 1f32 : -1f32;
            let next_window_rect: ImRect =  child_menu_window.Rect();
            let ta: ImVec2 = (g.IO.MousePos - g.IO.MouseDelta);
            let tb: ImVec2 = (child_dir > 0f32) ? next_window_rect.GetTL() : next_window_rect.GetTR();
            let tc: ImVec2 = (child_dir > 0f32) ? next_window_rect.GetBL() : next_window_rect.GetBR();
            let extra: c_float =  ImClamp(ImFabs(ta.x - tb.x) * 0.3f32, ref_unit * 0.5f32, ref_unit * 2.5f32);   // add a bit of extra slack.
            ta.x += child_dir * -0.5f32;
            tb.x += child_dir * ref_unit;
            tc.x += child_dir * ref_unit;
            tb.y = ta.y + ImMax((tb.y - extra) - ta.y, -ref_unit * 8.00f32);                           // triangle has maximum height to limit the slope and the bias toward large sub-menus
            tc.y = ta.y + ImMin((tc.y + extra) - ta.y, +ref_unit * 8.00f32);
            moving_toward_child_menu = ImTriangleContainsPoint(ta, tb, tc, g.IO.MousePos);
            //GetForegroundDrawList().AddTriangleFilled(ta, tb, tc, moving_toward_child_menu ? IM_COL32(0,128,0,128) : IM_COL32(128,0,0,128)); // [DEBUG]
        }

        // The 'HovereWindow == window' check creates an inconsistency (e.g. moving away from menu slowly tends to hit same window, whereas moving away fast does not)
        // But we also need to not close the top-menu menu when moving over void. Perhaps we should extend the triangle check to a larger polygon.
        // (Remember to test this on BeginPopup("A").BeginMenu("B") sequence which behaves slightly differently as B isn't a Child of A and hovering isn't shared.)
        if (menu_is_open && !hovered && g.HoveredWindow == window && !moving_toward_child_menu && !g.NavDisableMouseHover)
            want_close = true;

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
        ClosePopupToLevel(g.BeginPopupStack.Size, true);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags | ImGuiItemStatusFlags_Openable | (menu_is_open ? ImGuiItemStatusFlags_Opened : 0));
    PopID();

    if (!menu_is_open && want_open && g.OpenPopupStack.Size > g.BeginPopupStack.Size)
    {
        // Don't recycle same menu level in the same frame, first close the other menu and yield for a frame.
        OpenPopup(label);
        return false;
    }

    menu_is_open |= want_open;
    if (want_open)
        OpenPopup(label);

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

bool BeginMenu(label: *const c_char, enabled: bool)
{
    return BeginMenuEx(label, null_mut(), enabled);
}

c_void EndMenu()
{
    // Nav: When a left move request _within our child menu_ failed, close ourselves (the _parent_ menu).
    // A menu doesn't close itself because EndMenuBar() wants the catch the last Left<>Right inputs.
    // However, it means that with the current code, a BeginMenu() from outside another menu or a menu-bar won't be closable with the Left direction.
    // FIXME: This doesn't work if the parent BeginMenu() is not on a menu.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (g.NavMoveDir == ImGuiDir_Left && NavMoveRequestButNoResultYet() && window.DC.LayoutType == ImGuiLayoutType_Vertical)
        if (g.NavWindow && (g.NavWindow.RootWindowForNav.Flags & ImGuiWindowFlags_Popup) && g.NavWindow.RootWindowForNav.ParentWindow == window)
        {
            ClosePopupToLevel(g.BeginPopupStack.Size, true);
            NavMoveRequestCancel();
        }

    EndPopup();
}

bool MenuItemEx(label: *const c_char, icon: *const c_char, shortcut: *const c_char, selected: bool, enabled: bool)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if (window.SkipItems)
        return false;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut style = &mut g.Style;
    let pos: ImVec2 = window.DC.CursorPos;
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);

    let menuset_is_open: bool = IsRootOfOpenMenuSet();
    let mut backed_nav_window: *mut ImGuiWindow =  g.NavWindow;
    if (menuset_is_open)
        g.NavWindow = window;

    // We've been using the equivalent of ImGuiSelectableFlags_SetNavIdOnHover on all Selectable() since early Nav system days (commit 43ee5d73),
    // but I am unsure whether this should be kept at all. For now moved it to be an opt-in feature used by menus only.
    let mut pressed: bool;
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
        window.DC.CursorPos.x += IM_FLOOR(style.ItemSpacing.x * 0.5f32);
        ImVec2 text_pos(window.DC.CursorPos.x + offsets.OffsetLabel, window.DC.CursorPos.y + window.DC.CurrLineTextBaseOffset);
        PushStyleVar(ImGuiStyleVar_ItemSpacing, ImVec2::new(style.ItemSpacing.x * 2.0f32, style.ItemSpacing.y));
        pressed = Selectable("", selected, selectable_flags, ImVec2::new(w, 0f32));
        PopStyleVar();
        RenderText(text_pos, label);
        window.DC.CursorPos.x += IM_FLOOR(style.ItemSpacing.x * (-1f32 + 0.5f32)); // -1 spacing to compensate the spacing added when Selectable() did a SameLine(). It would also work to call SameLine() ourselves after the PopStyleVar().
    }
    else
    {
        // Menu item inside a vertical menu
        // (In a typical menu window where all items are BeginMenu() or MenuItem() calls, extra_w will always be 0f32.
        //  Only when they are other items sticking out we're going to add spacing, yet only register minimum width into the layout system.
        let icon_w: c_float =  (icon && icon[0]) ? CalcTextSize(icon, null_mut()).x : 0f32;
        let shortcut_w: c_float =  (shortcut && shortcut[0]) ? CalcTextSize(shortcut, null_mut()).x : 0f32;
        let checkmark_w: c_float =  IM_FLOOR(g.FontSize * 1.200f32);
        let min_w: c_float =  window.DC.MenuColumns.DeclColumns(icon_w, label_size.x, shortcut_w, checkmark_w); // Feedback for next frame
        let stretch_w: c_float =  ImMax(0f32, GetContentRegionAvail().x - min_w);
        pressed = Selectable("", false, selectable_flags | ImGuiSelectableFlags_SpanAvailWidth, ImVec2::new(min_w, 0f32));
        RenderText(pos + ImVec2::new(offsets.OffsetLabel, 0f32), label);
        if (icon_w > 0f32)
            RenderText(pos + ImVec2::new(offsets.OffsetIcon, 0f32), icon);
        if (shortcut_w > 0f32)
        {
            PushStyleColor(ImGuiCol_Text, style.Colors[ImGuiCol_TextDisabled]);
            RenderText(pos + ImVec2::new(offsets.OffsetShortcut + stretch_w, 0f32), shortcut, null_mut(), false);
            PopStyleColor();
        }
        if (selected)
            RenderCheckMark(window.DrawList, pos + ImVec2::new(offsets.OffsetMark + stretch_w + g.FontSize * 0.40f32, g.FontSize * 0.134f * 0.5f32), GetColorU32(ImGuiCol_Text), g.FontSize  * 0.8660f32);
    }
    IMGUI_TEST_ENGINE_ITEM_INFO(g.LastItemData.ID, label, g.LastItemData.StatusFlags | ImGuiItemStatusFlags_Checkable | (selected ? ImGuiItemStatusFlags_Checked : 0));
    if (!enabled)
        EndDisabled();
    PopID();
    if (menuset_is_open)
        g.NavWindow = backed_nav_window;

    return pressed;
}

bool MenuItem(label: *const c_char, shortcut: *const c_char, selected: bool, enabled: bool)
{
    return MenuItemEx(label, null_mut(), shortcut, selected, enabled);
}

bool MenuItem(label: *const c_char, shortcut: *const c_char, p_selected: *mut bool, enabled: bool)
{
    if (MenuItemEx(label, null_mut(), shortcut, p_selected ? *p_selected : false, enabled))
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
    c_int                 TabCount;               // Number of tabs in this section.
    c_float               Width;                  // Sum of width of tabs in this section (after shrinking down)
    c_float               Spacing;                // Horizontal spacing at the end of the section.

    ImGuiTabBarSection() { memset(this, 0, sizeof(*this)); }
};

namespace ImGui
{
    static c_void             TabBarLayout(tab_bar: *mut ImGuiTabBar);
    static u32            TabBarCalcTabID(tab_bar: *mut ImGuiTabBar, label: *const c_char, docked_window: *mut ImGuiWindow);
    static c_float            TabBarCalcMaxTabWidth();
    static c_float            TabBarScrollClamp(tab_bar: *mut ImGuiTabBar, scrolling: c_float);
    static c_void             TabBarScrollToTab(tab_bar: *mut ImGuiTabBar, tab_id: ImGuiID, ImGuiTabBarSection* sections);
    static ImGuiTabItem*    TabBarScrollingButtons(tab_bar: *mut ImGuiTabBar);
    static ImGuiTabItem*    TabBarTabListPopupButton(tab_bar: *mut ImGuiTabBar);
}

ImGuiTabBar::ImGuiTabBar()
{
    memset(this, 0, sizeof(*this));
    CurrFrameVisible = PrevFrameVisible = -1;
    LastTabItemIdx = -1;
}

static inline c_int TabItemGetSectionIdx(*const ImGuiTabItem tab)
{
    return (tab.Flags & ImGuiTabItemFlags_Leading) ? 0 : (tab.Flags & ImGuiTabItemFlags_Trailing) ? 2 : 1;
}

static c_int IMGUI_CDECL TabItemComparerBySection(*const c_void lhs, *const c_void rhs)
{
    let a: *const ImGuiTabItem = (*const ImGuiTabItem)lhs;
    let b: *const ImGuiTabItem = (*const ImGuiTabItem)rhs;
    let a_section: c_int = TabItemGetSectionIdx(a);
    let b_section: c_int = TabItemGetSectionIdx(b);
    if (a_section != b_section)
        return a_section - b_section;
    return (a.IndexDuringLayout - b.IndexDuringLayout);
}

static c_int IMGUI_CDECL TabItemComparerByBeginOrder(*const c_void lhs, *const c_void rhs)
{
    let a: *const ImGuiTabItem = (*const ImGuiTabItem)lhs;
    let b: *const ImGuiTabItem = (*const ImGuiTabItem)rhs;
    return (a.BeginOrder - b.BeginOrder);
}

static ImGuiTabBar* GetTabBarFromTabBarRef(const ImGuiPtrOrIndex& ref)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return ref.Ptr ? (ImGuiTabBar*)ref.Ptr : g.TabBars.GetByIndex(ref.Index);
}

static ImGuiPtrOrIndex GetTabBarRefFromTabBar(tab_bar: *mut ImGuiTabBar)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.TabBars.Contains(tab_bar))
        return ImGuiPtrOrIndex(g.TabBars.GetIndex(tab_bar));
    return ImGuiPtrOrIndex(tab_bar);
}

bool    BeginTabBar(str_id: *const c_char, ImGuiTabBarFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (window.SkipItems)
        return false;

    let mut id: ImGuiID =  window.GetID(str_id);
    let mut tab_bar: *mut ImGuiTabBar =  g.TabBars.GetOrAddByKey(id);
    let tab_bar_bb: ImRect =  ImRect::new(window.DC.CursorPos.x, window.DC.CursorPos.y, window.WorkRect.Max.x, window.DC.CursorPos.y + g.FontSize + g.Style.FramePadding.y * 2);
    tab_bar.ID = id;
    return BeginTabBarEx(tab_bar, tab_bar_bb, flags | ImGuiTabBarFlags_IsFocused, null_mut());
}

bool    BeginTabBarEx(tab_bar: *mut ImGuiTabBar, tab_bar_bb: &ImRect, ImGuiTabBarFlags flags, dock_node: *mut ImGuiDockNode)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (window.SkipItems)
        return false;

    if ((flags & ImGuiTabBarFlags_DockNode) == 0)
        PushOverrideID(tab_bar.ID);

    // Add to stack
    g.CurrentTabBarStack.push(GetTabBarRefFromTabBar(tab_bar));
    g.CurrentTabBar = tab_bar;

    // Append with multiple BeginTabBar()/EndTabBar() pairs.
    tab_bar.BackupCursorPos = window.DC.CursorPos;
    if (tab_bar.CurrFrameVisible == g.FrameCount)
    {
        window.DC.CursorPos = ImVec2::new(tab_bar.BarRect.Min.x, tab_bar.BarRect.Max.y + tab_bar.ItemSpacingY);
        tab_bar.BeginCount+= 1;
        return true;
    }

    // Ensure correct ordering when toggling ImGuiTabBarFlags_Reorderable flag, or when a new tab was added while being not reorderable
    if ((flags & ImGuiTabBarFlags_Reorderable) != (tab_bar.Flags & ImGuiTabBarFlags_Reorderable) || (tab_bar.TabsAddedNew && !(flags & ImGuiTabBarFlags_Reorderable)))
        if ((flags & ImGuiTabBarFlags_DockNode) == 0) // FIXME: TabBar with DockNode can now be hybrid
            ImQsort(tab_bar.Tabs.Data, tab_bar.Tabs.Size, sizeof(ImGuiTabItem), TabItemComparerByBeginOrder);
    tab_bar.TabsAddedNew = false;

    // Flags
    if ((flags & ImGuiTabBarFlags_FittingPolicyMask_) == 0)
        flags |= ImGuiTabBarFlags_FittingPolicyDefault_;

    tab_bar.Flags = flags;
    tab_bar.BarRect = tab_bar_bb;
    tab_bar.WantLayout = true; // Layout will be done on the first call to ItemTab()
    tab_bar.PrevFrameVisible = tab_bar.CurrFrameVisible;
    tab_bar.CurrFrameVisible = g.FrameCount;
    tab_bar.PrevTabsContentsHeight = tab_bar.CurrTabsContentsHeight;
    tab_bar.CurrTabsContentsHeight = 0f32;
    tab_bar.ItemSpacingY = g.Style.ItemSpacing.y;
    tab_bar.FramePadding = g.Style.FramePadding;
    tab_bar.TabsActiveCount = 0;
    tab_bar.BeginCount = 1;

    // Set cursor pos in a way which only be used in the off-chance the user erroneously submits item before BeginTabItem(): items will overlap
    window.DC.CursorPos = ImVec2::new(tab_bar.BarRect.Min.x, tab_bar.BarRect.Max.y + tab_bar.ItemSpacingY);

    // Draw separator
    let col: u32 = GetColorU32((flags & ImGuiTabBarFlags_IsFocused) ? ImGuiCol_TabActive : ImGuiCol_TabUnfocusedActive);
    let y: c_float =  tab_bar.BarRect.Max.y - 1f32;
    if (dock_node != null_mut())
    {
        let separator_min_x: c_float =  dock_node.Pos.x + window.WindowBorderSize;
        let separator_max_x: c_float =  dock_node.Pos.x + dock_node.Size.x - window.WindowBorderSize;
        window.DrawList.AddLine(ImVec2::new(separator_min_x, y), ImVec2::new(separator_max_x, y), col, 1f32);
    }
    else
    {
        let separator_min_x: c_float =  tab_bar.BarRect.Min.x - IM_FLOOR(window.WindowPadding.x * 0.5f32);
        let separator_max_x: c_float =  tab_bar.BarRect.Max.x + IM_FLOOR(window.WindowPadding.x * 0.5f32);
        window.DrawList.AddLine(ImVec2::new(separator_min_x, y), ImVec2::new(separator_max_x, y), col, 1f32);
    }
    return true;
}

c_void    EndTabBar()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (window.SkipItems)
        return;

    let mut tab_bar: *mut ImGuiTabBar =  g.CurrentTabBar;
    if (tab_bar == null_mut())
    {
        // IM_ASSERT_USER_ERROR(tab_bar != NULL, "Mismatched BeginTabBar()/EndTabBar()!");
        return;
    }

    // Fallback in case no TabItem have been submitted
    if (tab_bar.WantLayout)
        TabBarLayout(tab_bar);

    // Restore the last visible height if no tab is visible, this reduce vertical flicker/movement when a tabs gets removed without calling SetTabItemClosed().
    let tab_bar_appearing: bool = (tab_bar.PrevFrameVisible + 1 < g.FrameCount);
    if (tab_bar.VisibleTabWasSubmitted || tab_bar.VisibleTabId == 0 || tab_bar_appearing)
    {
        tab_bar.CurrTabsContentsHeight = ImMax(window.DC.CursorPos.y - tab_bar.BarRect.Max.y, tab_bar.CurrTabsContentsHeight);
        window.DC.CursorPos.y = tab_bar.BarRect.Max.y + tab_bar.CurrTabsContentsHeight;
    }
    else
    {
        window.DC.CursorPos.y = tab_bar.BarRect.Max.y + tab_bar.PrevTabsContentsHeight;
    }
    if (tab_bar.BeginCount > 1)
        window.DC.CursorPos = tab_bar.BackupCursorPos;

    if ((tab_bar.Flags & ImGuiTabBarFlags_DockNode) == 0)
        PopID();

    g.CurrentTabBarStack.pop_back();
    g.CurrentTabBar = g.CurrentTabBarStack.empty() ? null_mut() : GetTabBarFromTabBarRef(g.CurrentTabBarStack.last().unwrap());
}

// This is called only once a frame before by the first call to ItemTab()
// The reason we're not calling it in BeginTabBar() is to leave a chance to the user to call the SetTabItemClosed() functions.
static c_void TabBarLayout(tab_bar: *mut ImGuiTabBar)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    tab_bar.WantLayout = false;

    // Garbage collect by compacting list
    // Detect if we need to sort out tab list (e.g. in rare case where a tab changed section)
    let tab_dst_n: c_int = 0;
    let mut need_sort_by_section: bool =  false;
    ImGuiTabBarSection sections[3]; // Layout sections: Leading, Central, Trailing
    for (let tab_src_n: c_int = 0; tab_src_n < tab_bar.Tabs.Size; tab_src_n++)
    {
        let mut tab: *mut ImGuiTabItem = &tab_bar.Tabs[tab_src_n];
        if (tab.LastFrameVisible < tab_bar.PrevFrameVisible || tab.WantClose)
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
        tab.IndexDuringLayout = (i16)tab_dst_n;

        // We will need sorting if tabs have changed section (e.g. moved from one of Leading/Central/Trailing to another)
        let curr_tab_section_n: c_int = TabItemGetSectionIdx(tab);
        if (tab_dst_n > 0)
        {
            let mut prev_tab: *mut ImGuiTabItem = &tab_bar.Tabs[tab_dst_n - 1];
            let prev_tab_section_n: c_int = TabItemGetSectionIdx(prev_tab);
            if (curr_tab_section_n == 0 && prev_tab_section_n != 0)
                need_sort_by_section = true;
            if (prev_tab_section_n == 2 && curr_tab_section_n != 2)
                need_sort_by_section = true;
        }

        sections[curr_tab_section_n].TabCount+= 1;
        tab_dst_n+= 1;
    }
    if (tab_bar.Tabs.Size != tab_dst_n)
        tab_bar.Tabs.resize(tab_dst_n);

    if (need_sort_by_section)
        ImQsort(tab_bar.Tabs.Data, tab_bar.Tabs.Size, sizeof(ImGuiTabItem), TabItemComparerBySection);

    // Calculate spacing between sections
    sections[0].Spacing = sections[0].TabCount > 0 && (sections[1].TabCount + sections[2].TabCount) > 0 ? g.Style.ItemInnerSpacing.x : 0f32;
    sections[1].Spacing = sections[1].TabCount > 0 && sections[2].TabCount > 0 ? g.Style.ItemInnerSpacing.x : 0f32;

    // Setup next selected tab
    let mut scroll_to_tab_id: ImGuiID =  0;
    if (tab_bar.NextSelectedTabId)
    {
        tab_bar.SelectedTabId = tab_bar.NextSelectedTabId;
        tab_bar.NextSelectedTabId = 0;
        scroll_to_tab_id = tab_bar.SelectedTabId;
    }

    // Process order change request (we could probably process it when requested but it's just saner to do it in a single spot).
    if (tab_bar.ReorderRequestTabId != 0)
    {
        if (TabBarProcessReorder(tab_bar))
            if (tab_bar.ReorderRequestTabId == tab_bar.SelectedTabId)
                scroll_to_tab_id = tab_bar.ReorderRequestTabId;
        tab_bar.ReorderRequestTabId = 0;
    }

    // Tab List Popup (will alter tab_bar.BarRect and therefore the available width!)
    let tab_list_popup_button: bool = (tab_bar.Flags & ImGuiTabBarFlags_TabListPopupButton) != 0;
    if (tab_list_popup_button)
        if (let mut tab_to_select: *mut ImGuiTabItem = TabBarTabListPopupButton(tab_bar)) // NB: Will alter BarRect.Min.x!
            scroll_to_tab_id = tab_bar.SelectedTabId = tab_to_select.ID;

    // Leading/Trailing tabs will be shrink only if central one aren't visible anymore, so layout the shrink data as: leading, trailing, central
    // (whereas our tabs are stored as: leading, central, trailing)
    c_int shrink_buffer_indexes[3] = { 0, sections[0].TabCount + sections[2].TabCount, sections[0].TabCount };
    g.ShrinkWidthBuffer.resize(tab_bar.Tabs.Size);

    // Compute ideal tabs widths + store them into shrink buffer
    ImGuiTabItem* most_recently_selected_tab= null_mut();
    let curr_section_n: c_int = -1;
    let mut found_selected_tab_id: bool =  false;
    for (let tab_n: c_int = 0; tab_n < tab_bar.Tabs.Size; tab_n++)
    {
        let mut tab: *mut ImGuiTabItem = &tab_bar.Tabs[tab_n];
        // IM_ASSERT(tab.LastFrameVisible >= tab_bar.PrevFrameVisible);

        if ((most_recently_selected_tab == null_mut() || most_recently_selected_tab.LastFrameSelected < tab.LastFrameSelected) && !(tab.Flags & ImGuiTabItemFlags_Button))
            most_recently_selected_tab = tab;
        if (tab.ID == tab_bar.SelectedTabId)
            found_selected_tab_id = true;
        if (scroll_to_tab_id == 0 && g.NavJustMovedToId == tab.ID)
            scroll_to_tab_id = tab.ID;

        // Refresh tab width immediately, otherwise changes of style e.g. style.FramePadding.x would noticeably lag in the tab bar.
        // Additionally, when using TabBarAddTab() to manipulate tab bar order we occasionally insert new tabs that don't have a width yet,
        // and we cannot wait for the next BeginTabItem() call. We cannot compute this width within TabBarAddTab() because font size depends on the active window.
        let mut  tab_name: *const c_char = tab_bar.GetTabName(tab);
        let has_close_button: bool = (tab.Flags & ImGuiTabItemFlags_NoCloseButton) ? false : true;
        tab.ContentWidth = (tab.RequestedWidth >= 0f32) ? tab.RequestedWidth : TabItemCalcSize(tab_name, has_close_button).x;

        let section_n: c_int = TabItemGetSectionIdx(tab);
        ImGuiTabBarSection* section = &sections[section_n];
        section.Width += tab.ContentWidth + (section_n == curr_section_n ? g.Style.ItemInnerSpacing.x : 0f32);
        curr_section_n = section_n;

        // Store data so we can build an array sorted by width if we need to shrink tabs down
        IM_MSVC_WARNING_SUPPRESS(6385);
        ImGuiShrinkWidthItem* shrink_width_item = &g.ShrinkWidthBuffer[shrink_buffer_indexes[section_n]++];
        shrink_width_item.Index = tab_n;
        shrink_width_item.Width = shrink_width_item.InitialWidth = tab.ContentWidth;
        tab.Width = ImMax(tab.ContentWidth, 1f32);
    }

    // Compute total ideal width (used for e.g. auto-resizing a window)
    tab_bar.WidthAllTabsIdeal = 0f32;
    for (let section_n: c_int = 0; section_n < 3; section_n++)
        tab_bar.WidthAllTabsIdeal += sections[section_n].Width + sections[section_n].Spacing;

    // Horizontal scrolling buttons
    // (note that TabBarScrollButtons() will alter BarRect.Max.x)
    if ((tab_bar.WidthAllTabsIdeal > tab_bar.BarRect.GetWidth() && tab_bar.Tabs.Size > 1) && !(tab_bar.Flags & ImGuiTabBarFlags_NoTabListScrollingButtons) && (tab_bar.Flags & ImGuiTabBarFlags_FittingPolicyScroll))
        if (let mut scroll_and_select_tab: *mut ImGuiTabItem = TabBarScrollingButtons(tab_bar))
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
    let mut width_excess: c_float = 0f32;
    if (central_section_is_visible)
        width_excess = ImMax(section_1_w - (tab_bar.BarRect.GetWidth() - section_0_w - section_2_w), 0f32); // Excess used to shrink central section
    else
        width_excess = (section_0_w + section_2_w) - tab_bar.BarRect.GetWidth(); // Excess used to shrink leading/trailing section

    // With ImGuiTabBarFlags_FittingPolicyScroll policy, we will only shrink leading/trailing if the central section is not visible anymore
    if (width_excess >= 1f32 && ((tab_bar.Flags & ImGuiTabBarFlags_FittingPolicyResizeDown) || !central_section_is_visible))
    {
        let shrink_data_count: c_int = (central_section_is_visible ? sections[1].TabCount : sections[0].TabCount + sections[2].TabCount);
        let shrink_data_offset: c_int = (central_section_is_visible ? sections[0].TabCount + sections[2].TabCount : 0);
        ShrinkWidths(g.ShrinkWidthBuffer.Data + shrink_data_offset, shrink_data_count, width_excess);

        // Apply shrunk values into tabs and sections
        for (let tab_n: c_int = shrink_data_offset; tab_n < shrink_data_offset + shrink_data_count; tab_n++)
        {
            let mut tab: *mut ImGuiTabItem = &tab_bar.Tabs[g.ShrinkWidthBuffer[tab_n].Index];
            let shrinked_width: c_float =  IM_FLOOR(g.ShrinkWidthBuffer[tab_n].Width);
            if (shrinked_width < 0f32)
                continue;

            shrinked_width = ImMax(1f32, shrinked_width);
            let section_n: c_int = TabItemGetSectionIdx(tab);
            sections[section_n].Width -= (tab.Width - shrinked_width);
            tab.Width = shrinked_width;
        }
    }

    // Layout all active tabs
    let section_tab_index: c_int = 0;
    let tab_offset: c_float =  0f32;
    tab_bar.WidthAllTabs = 0f32;
    for (let section_n: c_int = 0; section_n < 3; section_n++)
    {
        ImGuiTabBarSection* section = &sections[section_n];
        if (section_n == 2)
            tab_offset = ImMin(ImMax(0f32, tab_bar.BarRect.GetWidth() - section.Width), tab_offset);

        for (let tab_n: c_int = 0; tab_n < section.TabCount; tab_n++)
        {
            let mut tab: *mut ImGuiTabItem = &tab_bar.Tabs[section_tab_index + tab_n];
            tab.Offset = tab_offset;
            tab.NameOffset = -1;
            tab_offset += tab.Width + (tab_n < section.TabCount - 1 ? g.Style.ItemInnerSpacing.x : 0f32);
        }
        tab_bar.WidthAllTabs += ImMax(section.Width + section.Spacing, 0f32);
        tab_offset += section.Spacing;
        section_tab_index += section.TabCount;
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
    tab_bar.VisibleTabWasSubmitted = false;

    // CTRL+TAB can override visible tab temporarily
    if (g.NavWindowingTarget != null_mut() && g.NavWindowingTarget.DockNode && g.NavWindowingTarget.DockNode.TabBar == tab_bar)
        tab_bar.VisibleTabId = scroll_to_tab_id = g.NavWindowingTarget.TabId;

    // Update scrolling
    if (scroll_to_tab_id != 0)
        TabBarScrollToTab(tab_bar, scroll_to_tab_id, sections);
    tab_bar.ScrollingAnim = TabBarScrollClamp(tab_bar, tab_bar.ScrollingAnim);
    tab_bar.ScrollingTarget = TabBarScrollClamp(tab_bar, tab_bar.ScrollingTarget);
    if (tab_bar.ScrollingAnim != tab_bar.ScrollingTarget)
    {
        // Scrolling speed adjust itself so we can always reach our target in 1/3 seconds.
        // Teleport if we are aiming far off the visible line
        tab_bar.ScrollingSpeed = ImMax(tab_bar.ScrollingSpeed, 70f32 * g.FontSize);
        tab_bar.ScrollingSpeed = ImMax(tab_bar.ScrollingSpeed, ImFabs(tab_bar.ScrollingTarget - tab_bar.ScrollingAnim) / 0.3f32);
        let teleport: bool = (tab_bar.PrevFrameVisible + 1 < g.FrameCount) || (tab_bar.ScrollingTargetDistToVisibility > 10f32 * g.FontSize);
        tab_bar.ScrollingAnim = teleport ? tab_bar.ScrollingTarget : ImLinearSweep(tab_bar.ScrollingAnim, tab_bar.ScrollingTarget, g.IO.DeltaTime * tab_bar.ScrollingSpeed);
    }
    else
    {
        tab_bar.ScrollingSpeed = 0f32;
    }
    tab_bar.ScrollingRectMinX = tab_bar.BarRect.Min.x + sections[0].Width + sections[0].Spacing;
    tab_bar.ScrollingRectMaxX = tab_bar.BarRect.Max.x - sections[2].Width - sections[1].Spacing;

    // Actual layout in host window (we don't do it in BeginTabBar() so as not to waste an extra frame)
    let mut window = g.CurrentWindow;
    window.DC.CursorPos = tab_bar.BarRect.Min;
    ItemSize(ImVec2::new(tab_bar.WidthAllTabs, tab_bar.BarRect.GetHeight()), tab_bar.FramePadding.y);
    window.DC.IdealMaxPos.x = ImMax(window.DC.IdealMaxPos.x, tab_bar.BarRect.Min.x + tab_bar.WidthAllTabsIdeal);
}

// Dockable uses Name/ID in the global namespace. Non-dockable items use the ID stack.
static u32   TabBarCalcTabID(tab_bar: *mut ImGuiTabBar, label: *const c_char, docked_window: *mut ImGuiWindow)
{
    if (docked_window != null_mut())
    {
        IM_UNUSED(tab_bar);
        // IM_ASSERT(tab_bar.Flags & ImGuiTabBarFlags_DockNode);
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

static c_float TabBarCalcMaxTabWidth()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize * 20f32;
}

ImGuiTabItem* TabBarFindTabByID(tab_bar: *mut ImGuiTabBar, tab_id: ImGuiID)
{
    if (tab_id != 0)
        for (let n: c_int = 0; n < tab_bar.Tabs.Size; n++)
            if (tab_bar.Tabs[n].ID == tab_id)
                return &tab_bar.Tabs[n];
    return null_mut();
}

// FIXME: See references to #2304 in TODO.txt
ImGuiTabItem* TabBarFindMostRecentlySelectedTabForActiveWindow(tab_bar: *mut ImGuiTabBar)
{
    ImGuiTabItem* most_recently_selected_tab= null_mut();
    for (let tab_n: c_int = 0; tab_n < tab_bar.Tabs.Size; tab_n++)
    {
        let mut tab: *mut ImGuiTabItem = &tab_bar.Tabs[tab_n];
        if (most_recently_selected_tab == null_mut() || most_recently_selected_tab.LastFrameSelected < tab.LastFrameSelected)
            if (tab.Window && tab.window.WasActive)
                most_recently_selected_tab = tab;
    }
    return most_recently_selected_tab;
}

// The purpose of this call is to register tab in advance so we can control their order at the time they appear.
// Otherwise calling this is unnecessary as tabs are appending as needed by the BeginTabItem() function.
c_void TabBarAddTab(tab_bar: *mut ImGuiTabBar, ImGuiTabItemFlags tab_flags, window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(TabBarFindTabByID(tab_bar, window.TabId) == NULL);
    // IM_ASSERT(g.CurrentTabBar != tab_bar);  // Can't work while the tab bar is active as our tab doesn't have an X offset yet, in theory we could/should test something like (tab_bar.CurrFrameVisible < g.FrameCount) but we'd need to solve why triggers the commented early-out assert in BeginTabBarEx() (probably dock node going from implicit to explicit in same frame)

    if (!window.HasCloseButton)
        tab_flags |= ImGuiTabItemFlags_NoCloseButton;       // Set _NoCloseButton immediately because it will be used for first-frame width calculation.

    ImGuiTabItem new_tab;
    new_tab.ID = window.TabId;
    new_tab.Flags = tab_flags;
    new_tab.LastFrameVisible = tab_bar.CurrFrameVisible;   // Required so BeginTabBar() doesn't ditch the tab
    if (new_tab.LastFrameVisible == -1)
        new_tab.LastFrameVisible = g.FrameCount - 1;
    new_tab.Window = window;                                // Required so tab bar layout can compute the tab width before tab submission
    tab_bar.Tabs.push(new_tab);
}

// The *TabId fields be already set by the docking system _before_ the actual TabItem was created, so we clear them regardless.
c_void TabBarRemoveTab(tab_bar: *mut ImGuiTabBar, tab_id: ImGuiID)
{
    if (let mut tab: *mut ImGuiTabItem = TabBarFindTabByID(tab_bar, tab_id))
        tab_bar.Tabs.erase(tab);
    if (tab_bar.VisibleTabId == tab_id)      { tab_bar.VisibleTabId = 0; }
    if (tab_bar.SelectedTabId == tab_id)     { tab_bar.SelectedTabId = 0; }
    if (tab_bar.NextSelectedTabId == tab_id) { tab_bar.NextSelectedTabId = 0; }
}

// Called on manual closure attempt
c_void TabBarCloseTab(tab_bar: *mut ImGuiTabBar, ImGuiTabItem* tab)
{
    if (tab.Flags & ImGuiTabItemFlags_Button)
        return; // A button appended with TabItemButton().

    if (!(tab.Flags & ImGuiTabItemFlags_UnsavedDocument))
    {
        // This will remove a frame of lag for selecting another tab on closure.
        // However we don't run it in the case where the 'Unsaved' flag is set, so user gets a chance to fully undo the closure
        tab.WantClose = true;
        if (tab_bar.VisibleTabId == tab.ID)
        {
            tab.LastFrameVisible = -1;
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

static c_float TabBarScrollClamp(tab_bar: *mut ImGuiTabBar, scrolling: c_float)
{
    scrolling = ImMin(scrolling, tab_bar.WidthAllTabs - tab_bar.BarRect.GetWidth());
    return ImMax(scrolling, 0f32);
}

// Note: we may scroll to tab that are not selected! e.g. using keyboard arrow keys
static c_void TabBarScrollToTab(tab_bar: *mut ImGuiTabBar, tab_id: ImGuiID, ImGuiTabBarSection* sections)
{
    let mut tab: *mut ImGuiTabItem = TabBarFindTabByID(tab_bar, tab_id);
    if (tab == null_mut())
        return;
    if (tab.Flags & ImGuiTabItemFlags_SectionMask_)
        return;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let margin: c_float =  g.FontSize * 1f32; // When to scroll to make Tab N+1 visible always make a bit of N visible to suggest more scrolling area (since we don't have a scrollbar)
    let order: c_int = tab_bar.GetTabOrder(tab);

    // Scrolling happens only in the central section (leading/trailing sections are not scrolling)
    // FIXME: This is all confusing.
    let scrollable_width: c_float =  tab_bar.BarRect.GetWidth() - sections[0].Width - sections[2].Width - sections[1].Spacing;

    // We make all tabs positions all relative Sections[0].Width to make code simpler
    let tab_x1: c_float =  tab.Offset - sections[0].Width + (order > sections[0].TabCount - 1 ? -margin : 0f32);
    let tab_x2: c_float =  tab.Offset - sections[0].Width + tab.Width + (order + 1 < tab_bar.Tabs.Size - sections[2].TabCount ? margin : 1f32);
    tab_bar.ScrollingTargetDistToVisibility = 0f32;
    if (tab_bar.ScrollingTarget > tab_x1 || (tab_x2 - tab_x1 >= scrollable_width))
    {
        // Scroll to the left
        tab_bar.ScrollingTargetDistToVisibility = ImMax(tab_bar.ScrollingAnim - tab_x2, 0f32);
        tab_bar.ScrollingTarget = tab_x1;
    }
    else if (tab_bar.ScrollingTarget < tab_x2 - scrollable_width)
    {
        // Scroll to the right
        tab_bar.ScrollingTargetDistToVisibility = ImMax((tab_x1 - scrollable_width) - tab_bar.ScrollingAnim, 0f32);
        tab_bar.ScrollingTarget = tab_x2 - scrollable_width;
    }
}

c_void TabBarQueueReorder(tab_bar: *mut ImGuiTabBar, *const ImGuiTabItem tab, offset: c_int)
{
    // IM_ASSERT(offset != 0);
    // IM_ASSERT(tab_bar.ReorderRequestTabId == 0);
    tab_bar.ReorderRequestTabId = tab.ID;
    tab_bar.ReorderRequestOffset = (i16)offset;
}

c_void TabBarQueueReorderFromMousePos(tab_bar: *mut ImGuiTabBar, *const ImGuiTabItem src_tab, mouse_pos: ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(tab_bar.ReorderRequestTabId == 0);
    if ((tab_bar.Flags & ImGuiTabBarFlags_Reorderable) == 0)
        return;

    let is_central_section: bool = (src_tab.Flags & ImGuiTabItemFlags_SectionMask_) == 0;
    let bar_offset: c_float =  tab_bar.BarRect.Min.x - (is_central_section ? tab_bar.ScrollingTarget : 0);

    // Count number of contiguous tabs we are crossing over
    let dir: c_int = (bar_offset + src_tab.Offset) > mouse_pos.x ? -1 : +1;
    let src_idx: c_int = tab_bar.Tabs.index_from_ptr(src_tab);
    let dst_idx: c_int = src_idx;
    for (let i: c_int = src_idx; i >= 0 && i < tab_bar.Tabs.Size; i += dir)
    {
        // Reordered tabs must share the same section
        let dst_tab: *const ImGuiTabItem = &tab_bar.Tabs[i];
        if (dst_tab.Flags & ImGuiTabItemFlags_NoReorder)
            break;
        if ((dst_tab.Flags & ImGuiTabItemFlags_SectionMask_) != (src_tab.Flags & ImGuiTabItemFlags_SectionMask_))
            break;
        dst_idx = i;

        // Include spacing after tab, so when mouse cursor is between tabs we would not continue checking further tabs that are not hovered.
        let x1: c_float =  bar_offset + dst_tab.Offset - g.Style.ItemInnerSpacing.x;
        let x2: c_float =  bar_offset + dst_tab.Offset + dst_tab.Width + g.Style.ItemInnerSpacing.x;
        //GetForegroundDrawList().AddRect(ImVec2::new(x1, tab_bar.BarRect.Min.y), ImVec2::new(x2, tab_bar.BarRect.Max.y), IM_COL32(255, 0, 0, 255));
        if ((dir < 0 && mouse_pos.x > x1) || (dir > 0 && mouse_pos.x < x2))
            break;
    }

    if (dst_idx != src_idx)
        TabBarQueueReorder(tab_bar, src_tab, dst_idx - src_idx);
}

bool TabBarProcessReorder(tab_bar: *mut ImGuiTabBar)
{
    let mut tab1: *mut ImGuiTabItem = TabBarFindTabByID(tab_bar, tab_bar.ReorderRequestTabId);
    if (tab1 == null_mut() || (tab1.Flags & ImGuiTabItemFlags_NoReorder))
        return false;

    //IM_ASSERT(tab_bar.Flags & ImGuiTabBarFlags_Reorderable); // <- this may happen when using debug tools
    let tab2_order: c_int = tab_bar.GetTabOrder(tab1) + tab_bar.ReorderRequestOffset;
    if (tab2_order < 0 || tab2_order >= tab_bar.Tabs.Size)
        return false;

    // Reordered tabs must share the same section
    // (Note: TabBarQueueReorderFromMousePos() also has a similar test but since we allow direct calls to TabBarQueueReorder() we do it here too)
    let mut tab2: *mut ImGuiTabItem = &tab_bar.Tabs[tab2_order];
    if (tab2.Flags & ImGuiTabItemFlags_NoReorder)
        return false;
    if ((tab1.Flags & ImGuiTabItemFlags_SectionMask_) != (tab2.Flags & ImGuiTabItemFlags_SectionMask_))
        return false;

    ImGuiTabItem item_tmp = *tab1;
    let mut src_tab: *mut ImGuiTabItem = (tab_bar.ReorderRequestOffset > 0) ? tab1 + 1 : tab2;
    let mut dst_tab: *mut ImGuiTabItem = (tab_bar.ReorderRequestOffset > 0) ? tab1 : tab2 + 1;
    let move_count: c_int = (tab_bar.ReorderRequestOffset > 0) ? tab_bar.ReorderRequestOffset : -tab_bar.ReorderRequestOffset;
    memmove(dst_tab, src_tab, move_count * sizeof(ImGuiTabItem));
    *tab2 = item_tmp;

    if (tab_bar.Flags & ImGuiTabBarFlags_SaveSettings)
        MarkIniSettingsDirty();
    return true;
}

static ImGuiTabItem* TabBarScrollingButtons(tab_bar: *mut ImGuiTabBar)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    const ImVec2 arrow_button_size(g.FontSize - 2.0f32, g.FontSize + g.Style.FramePadding.y * 2.00f32);
    let scrolling_buttons_width: c_float =  arrow_button_size.x * 2.0f32;

    let backup_cursor_pos: ImVec2 = window.DC.CursorPos;
    //window.DrawList.AddRect(ImVec2::new(tab_bar.BarRect.Max.x - scrolling_buttons_width, tab_bar.BarRect.Min.y), ImVec2::new(tab_bar.BarRect.Max.x, tab_bar.BarRect.Max.y), IM_COL32(255,0,0,255));

    let select_dir: c_int = 0;
    ImVec4 arrow_col = g.Style.Colors[ImGuiCol_Text];
    arrow_col.w *= 0.5f32;

    PushStyleColor(ImGuiCol_Text, arrow_col);
    PushStyleColor(ImGuiCol_Button, ImVec4(0, 0, 0, 0));
    let backup_repeat_delay: c_float =  g.IO.KeyRepeatDelay;
    let backup_repeat_rate: c_float =  g.IO.KeyRepeatRate;
    g.IO.KeyRepeatDelay = 0.250f32;
    g.IO.KeyRepeatRate = 0.200f32;
    let x: c_float =  ImMax(tab_bar.BarRect.Min.x, tab_bar.BarRect.Max.x - scrolling_buttons_width);
    window.DC.CursorPos = ImVec2::new(x, tab_bar.BarRect.Min.y);
    if (ArrowButtonEx("##<", ImGuiDir_Left, arrow_button_size, ImGuiButtonFlags_PressedOnClick | ImGuiButtonFlags_Repeat))
        select_dir = -1;
    window.DC.CursorPos = ImVec2::new(x + arrow_button_size.x, tab_bar.BarRect.Min.y);
    if (ArrowButtonEx("##>", ImGuiDir_Right, arrow_button_size, ImGuiButtonFlags_PressedOnClick | ImGuiButtonFlags_Repeat))
        select_dir = +1;
    PopStyleColor(2);
    g.IO.KeyRepeatRate = backup_repeat_rate;
    g.IO.KeyRepeatDelay = backup_repeat_delay;

    ImGuiTabItem* tab_to_scroll_to= null_mut();
    if (select_dir != 0)
        if (let mut tab_item: *mut ImGuiTabItem = TabBarFindTabByID(tab_bar, tab_bar.SelectedTabId))
        {
            let selected_order: c_int = tab_bar.GetTabOrder(tab_item);
            let target_order: c_int = selected_order + select_dir;

            // Skip tab item buttons until another tab item is found or end is reached
            while (tab_to_scroll_to == null_mut())
            {
                // If we are at the end of the list, still scroll to make our tab visible
                tab_to_scroll_to = &tab_bar.Tabs[(target_order >= 0 && target_order < tab_bar.Tabs.Size) ? target_order : selected_order];

                // Cross through buttons
                // (even if first/last item is a button, return it so we can update the scroll)
                if (tab_to_scroll_to.Flags & ImGuiTabItemFlags_Button)
                {
                    target_order += select_dir;
                    selected_order += select_dir;
                    tab_to_scroll_to = (target_order < 0 || target_order >= tab_bar.Tabs.Size) ? tab_to_scroll_to : null_mut();
                }
            }
        }
    window.DC.CursorPos = backup_cursor_pos;
    tab_bar.BarRect.Max.x -= scrolling_buttons_width + 1f32;

    return tab_to_scroll_to;
}

static ImGuiTabItem* TabBarTabListPopupButton(tab_bar: *mut ImGuiTabBar)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // We use g.Style.FramePadding.y to match the square ArrowButton size
    let tab_list_popup_button_width: c_float =  g.FontSize + g.Style.FramePadding.y;
    let backup_cursor_pos: ImVec2 = window.DC.CursorPos;
    window.DC.CursorPos = ImVec2::new(tab_bar.BarRect.Min.x - g.Style.FramePadding.y, tab_bar.BarRect.Min.y);
    tab_bar.BarRect.Min.x += tab_list_popup_button_width;

    ImVec4 arrow_col = g.Style.Colors[ImGuiCol_Text];
    arrow_col.w *= 0.5f32;
    PushStyleColor(ImGuiCol_Text, arrow_col);
    PushStyleColor(ImGuiCol_Button, ImVec4(0, 0, 0, 0));
    let mut open: bool =  BeginCombo("##v", null_mut(), ImGuiComboFlags_NoPreview | ImGuiComboFlags_HeightLargest);
    PopStyleColor(2);

    ImGuiTabItem* tab_to_select= null_mut();
    if (open)
    {
        for (let tab_n: c_int = 0; tab_n < tab_bar.Tabs.Size; tab_n++)
        {
            let mut tab: *mut ImGuiTabItem = &tab_bar.Tabs[tab_n];
            if (tab.Flags & ImGuiTabItemFlags_Button)
                continue;

            let mut  tab_name: *const c_char = tab_bar.GetTabName(tab);
            if (Selectable(tab_name, tab_bar.SelectedTabId == tab.ID))
                tab_to_select = tab;
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

bool    BeginTabItem(label: *const c_char, p_open: *mut bool, ImGuiTabItemFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (window.SkipItems)
        return false;

    let mut tab_bar: *mut ImGuiTabBar =  g.CurrentTabBar;
    if (tab_bar == null_mut())
    {
        // IM_ASSERT_USER_ERROR(tab_bar, "Needs to be called between BeginTabBar() and EndTabBar()!");
        return false;
    }
    // IM_ASSERT((flags & ImGuiTabItemFlags_Button) == 0);             // BeginTabItem() Can't be used with button flags, use TabItemButton() instead!

    let mut ret: bool =  TabItemEx(tab_bar, label, p_open, flags, null_mut());
    if (ret && !(flags & ImGuiTabItemFlags_NoPushId))
    {
        let mut tab: *mut ImGuiTabItem = &tab_bar.Tabs[tab_bar.LastTabItemIdx];
        PushOverrideID(tab.ID); // We already hashed 'label' so push into the ID stack directly instead of doing another hash through PushID(label)
    }
    return ret;
}

c_void    EndTabItem()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (window.SkipItems)
        return;

    let mut tab_bar: *mut ImGuiTabBar =  g.CurrentTabBar;
    if (tab_bar == null_mut())
    {
        // IM_ASSERT_USER_ERROR(tab_bar != NULL, "Needs to be called between BeginTabBar() and EndTabBar()!");
        return;
    }
    // IM_ASSERT(tab_bar.LastTabItemIdx >= 0);
    let mut tab: *mut ImGuiTabItem = &tab_bar.Tabs[tab_bar.LastTabItemIdx];
    if (!(tab.Flags & ImGuiTabItemFlags_NoPushId))
        PopID();
}

bool    TabItemButton(label: *const c_char, ImGuiTabItemFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (window.SkipItems)
        return false;

    let mut tab_bar: *mut ImGuiTabBar =  g.CurrentTabBar;
    if (tab_bar == null_mut())
    {
        // IM_ASSERT_USER_ERROR(tab_bar != NULL, "Needs to be called between BeginTabBar() and EndTabBar()!");
        return false;
    }
    return TabItemEx(tab_bar, label, null_mut(), flags | ImGuiTabItemFlags_Button | ImGuiTabItemFlags_NoReorder, null_mut());
}

bool    TabItemEx(tab_bar: *mut ImGuiTabBar, label: *const c_char, p_open: *mut bool, ImGuiTabItemFlags flags, docked_window: *mut ImGuiWindow)
{
    // Layout whole tab bar if not already done
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (tab_bar.WantLayout)
    {
        ImGuiNextItemData backup_next_item_data = g.NextItemData;
        TabBarLayout(tab_bar);
        g.NextItemData = backup_next_item_data;
    }
    let mut window = g.CurrentWindow;
    if (window.SkipItems)
        return false;

    const let mut style = &mut g.Style;
    let mut id: ImGuiID =  TabBarCalcTabID(tab_bar, label, docked_window);

    // If the user called us with *p_open == false, we early out and don't render.
    // We make a call to ItemAdd() so that attempts to use a contextual popup menu with an implicit ID won't use an older ID.
    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags);
    if (p_open && !*p_open)
    {
        ItemAdd(ImRect::new(), id, null_mut(), ImGuiItemFlags_NoNav | ImGuiItemFlags_NoNavDefaultFocus);
        return false;
    }

    // IM_ASSERT(!p_open || !(flags & ImGuiTabItemFlags_Button));
    // IM_ASSERT((flags & (ImGuiTabItemFlags_Leading | ImGuiTabItemFlags_Trailing)) != (ImGuiTabItemFlags_Leading | ImGuiTabItemFlags_Trailing)); // Can't use both Leading and Trailing

    // Store into ImGuiTabItemFlags_NoCloseButton, also honor ImGuiTabItemFlags_NoCloseButton passed by user (although not documented)
    if (flags & ImGuiTabItemFlags_NoCloseButton)
        p_open= null_mut();
    else if (p_open == null_mut())
        flags |= ImGuiTabItemFlags_NoCloseButton;

    // Acquire tab data
    let mut tab: *mut ImGuiTabItem = TabBarFindTabByID(tab_bar, id);
    let mut tab_is_new: bool =  false;
    if (tab == null_mut())
    {
        tab_bar.Tabs.push(ImGuiTabItem());
        tab = &tab_bar.Tabs.last().unwrap();
        tab.ID = id;
        tab_bar.TabsAddedNew = tab_is_new = true;
    }
    tab_bar.LastTabItemIdx = (i16)tab_bar.Tabs.index_from_ptr(tab);

    // Calculate tab contents size
    let size: ImVec2 = TabItemCalcSize(label, p_open != null_mut());
    tab.RequestedWidth = -1f32;
    if (g.NextItemData.Flags & ImGuiNextItemDataFlags_HasWidth)
        size.x = tab.RequestedWidth = g.NextItemData.Width;
    if (tab_is_new)
        tab.Width = ImMax(1f32, size.x);
    tab.ContentWidth = size.x;
    tab.BeginOrder = tab_bar.TabsActiveCount+= 1;

    let tab_bar_appearing: bool = (tab_bar.PrevFrameVisible + 1 < g.FrameCount);
    let tab_bar_focused: bool = (tab_bar.Flags & ImGuiTabBarFlags_IsFocused) != 0;
    let tab_appearing: bool = (tab.LastFrameVisible + 1 < g.FrameCount);
    let is_tab_button: bool = (flags & ImGuiTabItemFlags_Button) != 0;
    tab.LastFrameVisible = g.FrameCount;
    tab.Flags = flags;
    tab.Window = docked_window;

    // Append name with zero-terminator
    // (regular tabs are permitted in a DockNode tab bar, but window tabs not permitted in a non-DockNode tab bar)
    if (tab.Window != null_mut())
    {
        // IM_ASSERT(tab_bar.Flags & ImGuiTabBarFlags_DockNode);
        tab.NameOffset = -1;
    }
    else
    {
        // IM_ASSERT(tab.Window == NULL);
        tab.NameOffset = (i32)tab_bar.TabsNames.size();
        tab_bar.TabsNames.append(label, label + strlen(label) + 1); // Append name _with_ the zero-terminator.
    }

    // Update selected tab
    if (!is_tab_button)
    {
        if (tab_appearing && (tab_bar.Flags & ImGuiTabBarFlags_AutoSelectNewTabs) && tab_bar.NextSelectedTabId == 0)
            if (!tab_bar_appearing || tab_bar.SelectedTabId == 0)
                tab_bar.NextSelectedTabId = id;  // New tabs gets activated
        if ((flags & ImGuiTabItemFlags_SetSelected) && (tab_bar.SelectedTabId != id)) // _SetSelected can only be passed on explicit tab bar
            tab_bar.NextSelectedTabId = id;
    }

    // Lock visibility
    // (Note: tab_contents_visible != tab_selected... because CTRL+TAB operations may preview some tabs without selecting them!)
    let mut tab_contents_visible: bool =  (tab_bar.VisibleTabId == id);
    if (tab_contents_visible)
        tab_bar.VisibleTabWasSubmitted = true;

    // On the very first frame of a tab bar we let first tab contents be visible to minimize appearing glitches
    if (!tab_contents_visible && tab_bar.SelectedTabId == 0 && tab_bar_appearing && docked_window == null_mut())
        if (tab_bar.Tabs.Size == 1 && !(tab_bar.Flags & ImGuiTabBarFlags_AutoSelectNewTabs))
            tab_contents_visible = true;

    // Note that tab_is_new is not necessarily the same as tab_appearing! When a tab bar stops being submitted
    // and then gets submitted again, the tabs will have 'tab_appearing=true' but 'tab_is_new=false'.
    if (tab_appearing && (!tab_bar_appearing || tab_is_new))
    {
        ItemAdd(ImRect::new(), id, null_mut(), ImGuiItemFlags_NoNav | ImGuiItemFlags_NoNavDefaultFocus);
        if (is_tab_button)
            return false;
        return tab_contents_visible;
    }

    if (tab_bar.SelectedTabId == id)
        tab.LastFrameSelected = g.FrameCount;

    // Backup current layout position
    let backup_main_cursor_pos: ImVec2 = window.DC.CursorPos;

    // Layout
    let is_central_section: bool = (tab.Flags & ImGuiTabItemFlags_SectionMask_) == 0;
    size.x = tab.Width;
    if (is_central_section)
        window.DC.CursorPos = tab_bar.BarRect.Min + ImVec2::new(IM_FLOOR(tab.Offset - tab_bar.ScrollingAnim), 0f32);
    else
        window.DC.CursorPos = tab_bar.BarRect.Min + ImVec2::new(tab.Offset, 0f32);
    let pos: ImVec2 = window.DC.CursorPos;
    let mut bb: ImRect = ImRect::new(pos, pos + size);

    // We don't have CPU clipping primitives to clip the CloseButton (until it becomes a texture), so need to add an extra draw call (temporary in the case of vertical animation)
    let want_clip_rect: bool = is_central_section && (bb.Min.x < tab_bar.ScrollingRectMinX || bb.Max.x > tab_bar.ScrollingRectMaxX);
    if (want_clip_rect)
        PushClipRect(ImVec2::new(ImMax(bb.Min.x, tab_bar.ScrollingRectMinX), bb.Min.y - 1), ImVec2::new(tab_bar.ScrollingRectMaxX, bb.Max.y), true);

    let backup_cursor_max_pos: ImVec2 = window.DC.CursorMaxPos;
    ItemSize(bb.GetSize(), style.FramePadding.y);
    window.DC.CursorMaxPos = backup_cursor_max_pos;

    if (!ItemAdd(bb, id))
    {
        if (want_clip_rect)
            PopClipRect();
        window.DC.CursorPos = backup_main_cursor_pos;
        return tab_contents_visible;
    }

    // Click to Select a tab
    ImGuiButtonFlags button_flags = ((is_tab_button ? ImGuiButtonFlags_PressedOnClickRelease : ImGuiButtonFlags_PressedOnClick) | ImGuiButtonFlags_AllowItemOverlap);
    if (g.DragDropActive && !g.DragDropPayload.IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW)) // FIXME: May be an opt-in property of the payload to disable this
        button_flags |= ImGuiButtonFlags_PressedOnDragDropHold;
    hovered: bool, held;
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
    let node: *mut ImGuiDockNode = docked_window ? docked_window.DockNode : null_mut();
    let single_floating_window_node: bool = node && node.IsFloatingNode() && (node.Windows.len() == 1);
    if (held && single_floating_window_node && IsMouseDragging(0, 0f32))
    {
        // Move
        StartMouseMovingWindow(docked_window);
    }
    else if (held && !tab_appearing && IsMouseDragging(0))
    {
        // Drag and drop: re-order tabs
        let drag_dir: c_int = 0;
        let drag_distance_from_edge_x: c_float =  0f32;
        if (!g.DragDropActive && ((tab_bar.Flags & ImGuiTabBarFlags_Reorderable) || (docked_window != null_mut())))
        {
            // While moving a tab it will jump on the other side of the mouse, so we also test for MouseDelta.x
            if (g.IO.MouseDelta.x < 0f32 && g.IO.MousePos.x < bb.Min.x)
            {
                drag_dir = -1;
                drag_distance_from_edge_x = bb.Min.x - g.IO.MousePos.x;
                TabBarQueueReorderFromMousePos(tab_bar, tab, g.IO.MousePos);
            }
            else if (g.IO.MouseDelta.x > 0f32 && g.IO.MousePos.x > bb.Max.x)
            {
                drag_dir = +1;
                drag_distance_from_edge_x = g.IO.MousePos.x - bb.Max.x;
                TabBarQueueReorderFromMousePos(tab_bar, tab, g.IO.MousePos);
            }
        }

        // Extract a Dockable window out of it's tab bar
        if (docked_window != null_mut() && !(docked_window.Flags & ImGuiWindowFlags_NoMove))
        {
            // We use a variable threshold to distinguish dragging tabs within a tab bar and extracting them out of the tab bar
            let mut undocking_tab: bool =  (g.DragDropActive && g.DragDropPayload.SourceId == id);
            if (!undocking_tab) //&& (!g.IO.ConfigDockingWithShift || g.IO.KeyShift)
            {
                let threshold_base: c_float =  g.FontSize;
                let threshold_x: c_float =  (threshold_base * 2.20f32);
                let threshold_y: c_float =  (threshold_base * 1.5f32) + ImClamp((ImFabs(g.IO.MouseDragMaxDistanceAbs[0].x) - threshold_base * 2.00f32) * 0.20f32, 0f32, threshold_base * 4.00f32);
                //GetForegroundDrawList().AddRect(ImVec2::new(bb.Min.x - threshold_x, bb.Min.y - threshold_y), ImVec2::new(bb.Max.x + threshold_x, bb.Max.y + threshold_y), IM_COL32_WHITE); // [DEBUG]

                let distance_from_edge_y: c_float =  ImMax(bb.Min.y - g.IO.MousePos.y, g.IO.MousePos.y - bb.Max.y);
                if (distance_from_edge_y >= threshold_y)
                    undocking_tab = true;
                if (drag_distance_from_edge_x > threshold_x)
                    if ((drag_dir < 0 && tab_bar.GetTabOrder(tab) == 0) || (drag_dir > 0 && tab_bar.GetTabOrder(tab) == tab_bar.Tabs.Size - 1))
                        undocking_tab = true;
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
    if (hovered && g.HoveredIdNotActiveTimer > TOOLTIP_DELAY && bb.GetWidth() < tab.ContentWidth)
    {
        // Enlarge tab display when hovering
        bb.Max.x = bb.Min.x + IM_FLOOR(ImLerp(bb.GetWidth(), tab.ContentWidth, ImSaturate((g.HoveredIdNotActiveTimer - 0.400f32) * 6.00f32)));
        display_draw_list = GetForegroundDrawList(window);
        TabItemBackground(display_draw_list, bb, flags, GetColorU32(ImGuiCol_TitleBgActive));
    }
// #endif

    // Render tab shape
    let mut  display_draw_list: *mut ImDrawList =  window.DrawList;
    let tab_col: u32 = GetColorU32((held || hovered) ? ImGuiCol_TabHovered : tab_contents_visible ? (tab_bar_focused ? ImGuiCol_TabActive : ImGuiCol_TabUnfocusedActive) : (tab_bar_focused ? ImGuiCol_Tab : ImGuiCol_TabUnfocused));
    TabItemBackground(display_draw_list, bb, flags, tab_col);
    RenderNavHighlight(bb, id);

    // Select with right mouse button. This is so the common idiom for context menu automatically highlight the current widget.
    let hovered_unblocked: bool = IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup);
    if (hovered_unblocked && (IsMouseClicked(1) || IsMouseReleased(1)))
        if (!is_tab_button)
            tab_bar.NextSelectedTabId = id;

    if (tab_bar.Flags & ImGuiTabBarFlags_NoCloseWithMiddleMouseButton)
        flags |= ImGuiTabItemFlags_NoCloseWithMiddleMouseButton;

    // Render tab label, process close button
    let mut close_button_id: ImGuiID =  p_open ? GetIDWithSeed("#CLOSE", null_mut(), docked_window ? docked_window.ID : id) : 0;
    let mut just_closed: bool;
    let mut text_clipped: bool;
    TabItemLabelAndCloseButton(display_draw_list, bb, flags, tab_bar.FramePadding, label, id, close_button_id, tab_contents_visible, &just_closed, &text_clipped);
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
    if (want_clip_rect)
        PopClipRect();
    window.DC.CursorPos = backup_main_cursor_pos;

    // Tooltip
    // (Won't work over the close button because ItemOverlap systems messes up with HoveredIdTimer-> seems ok)
    // (We test IsItemHovered() to discard e.g. when another item is active or drag and drop over the tab bar, which g.HoveredId ignores)
    // FIXME: This is a mess.
    // FIXME: We may want disabled tab to still display the tooltip?
    if (text_clipped && g.HoveredId == id && !held)
        if (!(tab_bar.Flags & ImGuiTabBarFlags_NoTooltip) && !(tab.Flags & ImGuiTabItemFlags_NoTooltip))
            if (IsItemHovered(ImGuiHoveredFlags_DelayNormal))
                SetTooltip("%.*s", (FindRenderedTextEnd(label) - label), label);

    // IM_ASSERT(!is_tab_button || !(tab_bar.SelectedTabId == tab.ID && is_tab_button)); // TabItemButton should not be selected
    if (is_tab_button)
        return pressed;
    return tab_contents_visible;
}

// [Public] This is call is 100% optional but it allows to remove some one-frame glitches when a tab has been unexpectedly removed.
// To use it to need to call the function SetTabItemClosed() between BeginTabBar() and EndTabBar().
// Tabs closed by the close button will automatically be flagged to avoid this issue.
c_void    SetTabItemClosed(label: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut is_within_manual_tab_bar: bool =  g.CurrentTabBar && !(g.CurrentTabBar.Flags & ImGuiTabBarFlags_DockNode);
    if (is_within_manual_tab_bar)
    {
        let mut tab_bar: *mut ImGuiTabBar =  g.CurrentTabBar;
        let mut tab_id: ImGuiID =  TabBarCalcTabID(tab_bar, label, null_mut());
        if (let mut tab: *mut ImGuiTabItem = TabBarFindTabByID(tab_bar, tab_id))
            tab.WantClose = true; // Will be processed by next call to TabBarLayout()
    }
    else if (let mut window: *mut ImGuiWindow =  FindWindowByName(label))
    {
        if (window.DockIsActive)
            if (node: *mut ImGuiDockNode = window.DockNode)
            {
                let mut tab_id: ImGuiID =  TabBarCalcTabID(node.TabBar, label, window);
                TabBarRemoveTab(node.TabBar, tab_id);
                window.DockTabWantClose = true;
            }
    }
}

ImVec2 TabItemCalcSize(label: *const c_char, has_close_button: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);
    let size: ImVec2 = ImVec2::new(label_size.x + g.Style.FramePadding.x, label_size.y + g.Style.FramePadding.y * 2.00f32);
    if (has_close_button)
        size.x += g.Style.FramePadding.x + (g.Style.ItemInnerSpacing.x + g.FontSize); // We use Y intentionally to fit the close button circle.
    else
        size.x += g.Style.FramePadding.x + 1f32;
    return ImVec2::new(ImMin(size.x, TabBarCalcMaxTabWidth()), size.y);
}

c_void TabItemBackground(ImDrawList* draw_list, bb: &ImRect, ImGuiTabItemFlags flags, u32 col)
{
    // While rendering tabs, we trim 1 pixel off the top of our bounding box so they can fit within a regular frame height while looking "detached" from it.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let width: c_float =  bb.GetWidth();
    IM_UNUSED(flags);
    // IM_ASSERT(width > 0f32);
    let rounding: c_float =  ImMax(0f32, ImMin((flags & ImGuiTabItemFlags_Button) ? g.Style.FrameRounding : g.Style.TabRounding, width * 0.5f32 - 1f32));
    let y1: c_float =  bb.Min.y + 1f32;
    let y2: c_float =  bb.Max.y + ((flags & ImGuiTabItemFlags_Preview) ? 0f32 : -1f32);
    draw_list.PathLineTo(ImVec2::new(bb.Min.x, y2));
    draw_list.PathArcToFast(ImVec2::new(bb.Min.x + rounding, y1 + rounding), rounding, 6, 9);
    draw_list.PathArcToFast(ImVec2::new(bb.Max.x - rounding, y1 + rounding), rounding, 9, 12);
    draw_list.PathLineTo(ImVec2::new(bb.Max.x, y2));
    draw_list.PathFillConvex(col);
    if (g.Style.TabBorderSize > 0f32)
    {
        draw_list.PathLineTo(ImVec2::new(bb.Min.x + 0.5f32, y2));
        draw_list.PathArcToFast(ImVec2::new(bb.Min.x + rounding + 0.5f32, y1 + rounding + 0.5f32), rounding, 6, 9);
        draw_list.PathArcToFast(ImVec2::new(bb.Max.x - rounding - 0.5f32, y1 + rounding + 0.5f32), rounding, 9, 12);
        draw_list.PathLineTo(ImVec2::new(bb.Max.x - 0.5f32, y2));
        draw_list.PathStroke(GetColorU32(ImGuiCol_Border), 0, g.Style.TabBorderSize);
    }
}

// Render text label (with custom clipping) + Unsaved Document marker + Close Button logic
// We tend to lock style.FramePadding for a given tab-bar, hence the 'frame_padding' parameter.
c_void TabItemLabelAndCloseButton(ImDrawList* draw_list, bb: &ImRect, ImGuiTabItemFlags flags, frame_padding: ImVec2, label: *const c_char, tab_id: ImGuiID, close_button_id: ImGuiID, is_contents_visible: bool, out_just_closed: *mut bool, out_text_clipped: *mut bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let label_size: ImVec2 = CalcTextSize(label, null_mut(), true);

    if (out_just_closed)
        *out_just_closed = false;
    if (out_text_clipped)
        *out_text_clipped = false;

    if (bb.GetWidth() <= 1f32)
        return;

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
        //draw_list.AddCircle(text_ellipsis_clip_bb.Min, 3.0f32, *out_text_clipped ? IM_COL32(255, 0, 0, 255) : IM_COL32(0, 255, 0, 255));
    }

    let button_sz: c_float =  g.FontSize;
    const ImVec2 button_pos(ImMax(bb.Min.x, bb.Max.x - frame_padding.x * 2.0f32 - button_sz), bb.Min.y);

    // Close Button & Unsaved Marker
    // We are relying on a subtle and confusing distinction between 'hovered' and 'g.HoveredId' which happens because we are using ImGuiButtonFlags_AllowOverlapMode + SetItemAllowOverlap()
    //  'hovered' will be true when hovering the Tab but NOT when hovering the close button
    //  'g.HoveredId==id' will be true when hovering the Tab including when hovering the close button
    //  'g.ActiveId==close_button_id' will be true when we are holding on the close button, in which case both hovered booleans are false
    let mut close_button_pressed: bool =  false;
    let mut close_button_visible: bool =  false;
    if (close_button_id != 0)
        if (is_contents_visible || bb.GetWidth() >= ImMax(button_sz, g.Style.TabMinWidthForCloseButton))
            if (g.HoveredId == tab_id || g.HoveredId == close_button_id || g.ActiveId == tab_id || g.ActiveId == close_button_id)
                close_button_visible = true;
    let mut unsaved_marker_visible: bool =  (flags & ImGuiTabItemFlags_UnsavedDocument) != 0 && (button_pos.x + button_sz <= bb.Max.x);

    if (close_button_visible)
    {
        ImGuiLastItemData last_item_backup = g.LastItemData;
        PushStyleVar(ImGuiStyleVar_FramePadding, frame_padding);
        if (CloseButton(close_button_id, button_pos))
            close_button_pressed = true;
        PopStyleVar();
        g.LastItemData = last_item_backup;

        // Close with middle mouse button
        if (!(flags & ImGuiTabItemFlags_NoCloseWithMiddleMouseButton) && IsMouseClicked(2))
            close_button_pressed = true;
    }
    else if (unsaved_marker_visible)
    {
        let mut bullet_bb: ImRect = ImRect::new(button_pos, button_pos + ImVec2::new(button_sz, button_sz) + g.Style.FramePadding * 2.00f32);
        RenderBullet(draw_list, bullet_bb.GetCenter(), GetColorU32(ImGuiCol_Text));
    }

    // This is all rather complicated
    // (the main idea is that because the close button only appears on hover, we don't want it to alter the ellipsis position)
    // FIXME: if FramePadding is noticeably large, ellipsis_max_x will be wrong here (e.g. #3497), maybe for consistency that parameter of RenderTextEllipsis() shouldn't exist..
    let ellipsis_max_x: c_float =  close_button_visible ? text_pixel_clip_bb.Max.x : bb.Max.x - 1f32;
    if (close_button_visible || unsaved_marker_visible)
    {
        text_pixel_clip_bb.Max.x -= close_button_visible ? (button_sz) : (button_sz * 0.800f32);
        text_ellipsis_clip_bb.Max.x -= unsaved_marker_visible ? (button_sz * 0.800f32) : 0f32;
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
