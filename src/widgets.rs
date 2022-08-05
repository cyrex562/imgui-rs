// dear imgui, v1.88
// (widgets code)

/*

index of this file:

// [SECTION] Forward Declarations
// [SECTION] Widgets: Text, etc.
// [SECTION] Widgets: Main (Button, Image, Checkbox, RadioButton, ProgressBar, Bullet, etc.)
// [SECTION] Widgets: Low-level Layout helpers (Spacing, Dummy, NewLine, Separator, etc.)
// [SECTION] Widgets: ComboBox
// [SECTION] data Type and data Formatting Helpers
// [SECTION] Widgets: DragScalar, DragFloat, DragInt, etc.
// [SECTION] Widgets: SliderScalar, SliderFloat, SliderInt, etc.
// [SECTION] Widgets: InputScalar, InputFloat, InputInt, etc.
// [SECTION] Widgets: InputText, InputTextMultiline
// [SECTION] Widgets: ColorEdit, ColorPicker, ColorButton, etc.
// [SECTION] Widgets: TreeNode, CollapsingHeader, etc.
// [SECTION] Widgets: selectable
// [SECTION] Widgets: ListBox
// [SECTION] Widgets: PlotLines, PlotHistogram
// [SECTION] Widgets: value helpers
// [SECTION] Widgets: menu_item, BeginMenu, EndMenu, etc.
// [SECTION] Widgets: BeginTabBar, EndTabBar, etc.
// [SECTION] Widgets: BeginTabItem, EndTabItem, etc.
// [SECTION] Widgets: columns, BeginColumns, EndColumns, etc.

*/

#if defined(_MSC_VER) && !defined(_CRT_SECURE_NO_WARNINGS)
#define _CRT_SECURE_NO_WARNINGS
#endif

#include "defines.rs"

#ifndef IMGUI_DISABLE

#ifndef IMGUI_DEFINE_MATH_OPERATORS
#define IMGUI_DEFINE_MATH_OPERATORS
#endif
#include "internal_h.rs"

// System includes
#include <ctype.h>      // toupper
#if defined(_MSC_VER) && _MSC_VER <= 1500 // MSVC 2008 or earlier
#include <stddef.h>     // intptr_t
#else
#include <stdint.h>     // intptr_t
#endif

//-------------------------------------------------------------------------
// Warnings
//-------------------------------------------------------------------------

// Visual Studio warnings
#ifdef _MSC_VER
#pragma warning (disable: 4127)     // condition expression is constant
#pragma warning (disable: 4996)     // 'This function or variable may be unsafe': strcpy, strdup, sprintf, vsnprintf, sscanf, fopen
#if defined(_MSC_VER) && _MSC_VER >= 1922 // MSVC 2019 16.2 or later
#pragma warning (disable: 5054)     // operator '|': deprecated between enumerations of different types
#endif
#pragma warning (disable: 26451)    // [Static Analyzer] Arithmetic overflow : Using operator 'xxx' on a 4 byte value and then casting the result to a 8 byte value. Cast the value to the wider type before calling operator 'xxx' to avoid overflow(io.2).
#pragma warning (disable: 26812)    // [Static Analyzer] The enum type 'xxx' is unscoped. Prefer 'enum class' over 'enum' (Enum.3).
#endif

// Clang/GCC warnings with -Weverything
#if defined(__clang__)
#if __has_warning("-Wunknown-warning-option")
#pragma clang diagnostic ignored "-Wunknown-warning-option"         // warning: unknown warning group 'xxx'                      // not all warnings are known by all Clang versions and they tend to be rename-happy.. so ignoring warnings triggers new warnings on some configuration. Great!
#endif
#pragma clang diagnostic ignored "-Wunknown-pragmas"                // warning: unknown warning group 'xxx'
#pragma clang diagnostic ignored "-Wold-style-cast"                 // warning: use of old-style cast                            // yes, they are more terse.
#pragma clang diagnostic ignored "-Wfloat-equal"                    // warning: comparing floating point with == or != is unsafe // storing and comparing against same constants (typically 0.0) is ok.
#pragma clang diagnostic ignored "-Wformat-nonliteral"              // warning: format string is not a string literal            // passing non-literal to vsnformat(). yes, user passing incorrect format strings can crash the code.
#pragma clang diagnostic ignored "-Wsign-conversion"                // warning: implicit conversion changes signedness
#pragma clang diagnostic ignored "-Wzero-as-null-pointer-constant"  // warning: zero as null pointer constant                    // some standard header variations use #define None 0
#pragma clang diagnostic ignored "-Wdouble-promotion"               // warning: implicit conversion from 'float' to 'double' when passing argument to function  // using printf() is a misery with this as C++ va_arg ellipsis changes float to double.
#pragma clang diagnostic ignored "-Wenum-enum-conversion"           // warning: bitwise operation between different enumeration types ('XXXFlags_' and 'XXXFlagsPrivate_')
#pragma clang diagnostic ignored "-Wdeprecated-enum-enum-conversion"// warning: bitwise operation between different enumeration types ('XXXFlags_' and 'XXXFlagsPrivate_') is deprecated
#pragma clang diagnostic ignored "-Wimplicit-int-float-conversion"  // warning: implicit conversion from 'xxx' to 'float' may lose precision
#elif defined(__GNUC__)
#pragma GCC diagnostic ignored "-Wpragmas"                          // warning: unknown option after '#pragma GCC diagnostic' kind
#pragma GCC diagnostic ignored "-Wformat-nonliteral"                // warning: format not a string literal, format string not checked
#pragma GCC diagnostic ignored "-Wclass-memaccess"                  // [__GNUC__ >= 8] warning: 'memset/memcpy' clearing/writing an object of type 'xxxx' with no trivial copy-assignment; use assignment or value-initialization instead
#pragma GCC diagnostic ignored "-Wdeprecated-enum-enum-conversion"  // warning: bitwise operation between different enumeration types ('XXXFlags_' and 'XXXFlagsPrivate_') is deprecated
#endif

//-------------------------------------------------------------------------
// data
//-------------------------------------------------------------------------

// Widgets
static let          DRAGDROP_HOLD_TO_OPEN_TIMER = 0.70;    // time for drag-hold to activate items accepting the ImGuiButtonFlags_PressedOnDragDropHold button behavior.
static let          DRAG_MOUSE_THRESHOLD_FACTOR = 0.50;    // Multiplier for the default value of io.mouse_drag_threshold to make DragFloat/DragInt react faster to mouse drags.

// Those MIN/MAX values are not define because we need to point to them
static const signed char    IM_S8_MIN  = -128;
static const signed char    IM_S8_MAX  = 127;
static const unsigned char  IM_U8_MIN  = 0;
static const unsigned char  IM_U8_MAX  = 0xFF;
static const signed short   IM_S16_MIN = -32768;
static const signed short   IM_S16_MAX = 32767;
static const unsigned short IM_U16_MIN = 0;
static const unsigned short IM_U16_MAX = 0xFFFF;
static const ImS32          IM_S32_MIN = INT_MIN;    // (-2147483647 - 1), (0x80000000);
static const ImS32          IM_S32_MAX = INT_MAX;    // (2147483647), (0x7FFFFFFF)
static const ImU32          IM_U32_MIN = 0;
static const ImU32          IM_U32_MAX = UINT_MAX;   // (0xFFFFFFFF)
#ifdef LLONG_MIN
static const ImS64          IM_S64_MIN = LLONG_MIN;  // (-9223372036854775807ll - 1ll);
static const ImS64          IM_S64_MAX = LLONG_MAX;  // (9223372036854775807ll);
#else
static const ImS64          IM_S64_MIN = -9223372036854775807LL - 1;
static const ImS64          IM_S64_MAX = 9223372036854775807LL;
#endif
static const ImU64          IM_U64_MIN = 0;
#ifdef ULLONG_MAX
static const ImU64          IM_U64_MAX = ULLONG_MAX; // (0xFFFFFFFFFFFFFFFFull);
#else
static const ImU64          IM_U64_MAX = (2ULL * 9223372036854775807LL + 1);
#endif

//-------------------------------------------------------------------------
// [SECTION] Forward Declarations
//-------------------------------------------------------------------------

// For InputTextEx()
static bool             InputTextFilterCharacter(unsigned int* p_char, ImGuiInputTextFlags flags, ImGuiInputTextCallback callback, void* user_data, ImGuiInputSource input_source);
static int              InputTextCalcTextLenAndLineCount(const char* text_begin, const char** out_text_end);
static Vector2D           InputTextCalcTextSizeW(const ImWchar* text_begin, const ImWchar* text_end, const ImWchar** remaining = None, Vector2D* out_offset = None, bool stop_on_new_line = false);

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

void ImGui::TextEx(const char* text, const char* text_end, ImGuiTextFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return;
    // ImGuiContext& g = *GImGui;

    // Accept null ranges
    if (text == text_end)
        text = text_end = "";

    // Calculate length
    const char* text_begin = text;
    if (text_end == None)
        text_end = text + strlen(text); // FIXME-OPT

    const Vector2D text_pos(window.DC.CursorPos.x, window.DC.CursorPos.y + window.DC.curr_line_text_base_offset);
    let wrap_pos_x = window.DC.TextWrapPos;
    const bool wrap_enabled = (wrap_pos_x >= 0.0);
    if (text_end - text <= 2000 || wrap_enabled)
    {
        // Common case
        let wrap_width = wrap_enabled ? CalcWrapWidthForPos(window.DC.CursorPos, wrap_pos_x) : 0.0;
        const Vector2D text_size = CalcTextSize(text_begin, text_end, false, wrap_width);

        ImRect bb(text_pos, text_pos + text_size);
        ItemSize(text_size, 0.0);
        if (!ItemAdd(bb, 0))
            return;

        // Render (we don't hide text after ## in this end-user function)
        render_textWrapped(bb.Min, text_begin, text_end, wrap_width);
    }
    else
    {
        // Long text!
        // Perform manual coarse clipping to optimize for long multi-line text
        // - From this point we will only compute the width of lines that are visible. Optimization only available when word-wrapping is disabled.
        // - We also don't vertically center the text within the line full height, which is unlikely to matter because we are likely the biggest and only item on the line.
        // - We use memchr(), pay attention that well optimized versions of those str/mem functions are much faster than a casually written loop.
        const char* line = text;
        let line_height = GetTextLineHeight();
        Vector2D text_size(0, 0);

        // Lines to skip (can't skip when logging text)
        Vector2D pos = text_pos;
        if (!g.log_enabled)
        {
            int lines_skippable = ((window.ClipRect.Min.y - text_pos.y) / line_height);
            if (lines_skippable > 0)
            {
                int lines_skipped = 0;
                while (line < text_end && lines_skipped < lines_skippable)
                {
                    const char* line_end = (const char*)memchr(line, '\n', text_end - line);
                    if (!line_end)
                        line_end = text_end;
                    if ((flags & ImGuiTextFlags_NoWidthForLargeClippedText) == 0)
                        text_size.x = ImMax(text_size.x, CalcTextSize(line, line_end).x);
                    line = line_end + 1;
                    lines_skipped += 1;
                }
                pos.y += lines_skipped * line_height;
            }
        }

        // Lines to render
        if (line < text_end)
        {
            ImRect line_rect(pos, pos + DimgVec2D::new(FLT_MAX, line_height));
            while (line < text_end)
            {
                if (is_clipped_ex(line_rect, 0))
                    break;

                const char* line_end = (const char*)memchr(line, '\n', text_end - line);
                if (!line_end)
                    line_end = text_end;
                text_size.x = ImMax(text_size.x, CalcTextSize(line, line_end).x);
                render_text(pos, line, line_end, false);
                line = line_end + 1;
                line_rect.Min.y += line_height;
                line_rect.Max.y += line_height;
                pos.y += line_height;
            }

            // count remaining lines
            int lines_skipped = 0;
            while (line < text_end)
            {
                const char* line_end = (const char*)memchr(line, '\n', text_end - line);
                if (!line_end)
                    line_end = text_end;
                if ((flags & ImGuiTextFlags_NoWidthForLargeClippedText) == 0)
                    text_size.x = ImMax(text_size.x, CalcTextSize(line, line_end).x);
                line = line_end + 1;
                lines_skipped += 1;
            }
            pos.y += lines_skipped * line_height;
        }
        text_size.y = (pos - text_pos).y;

        ImRect bb(text_pos, text_pos + text_size);
        ItemSize(text_size, 0.0);
        ItemAdd(bb, 0);
    }
}

void ImGui::TextUnformatted(const char* text, const char* text_end)
{
    TextEx(text, text_end, ImGuiTextFlags_NoWidthForLargeClippedText);
}

void ImGui::text(const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    TextV(fmt, args);
    va_end(args);
}

void ImGui::TextV(const char* fmt, va_list args)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    // FIXME-OPT: Handle the %s shortcut?
    const char* text, *text_end;
    ImFormatStringToTempBufferV(&text, &text_end, fmt, args);
    TextEx(text, text_end, ImGuiTextFlags_NoWidthForLargeClippedText);
}

void ImGui::TextColored(const Vector4D& col, const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    TextColoredV(col, fmt, args);
    va_end(args);
}

void ImGui::TextColoredV(const Vector4D& col, const char* fmt, va_list args)
{
    PushStyleColor(ImGuiCol_Text, col);
    if (fmt[0] == '%' && fmt[1] == 's' && fmt[2] == 0)
        TextEx(va_arg(args, const char*), None, ImGuiTextFlags_NoWidthForLargeClippedText); // Skip formatting
    else
        TextV(fmt, args);
    PopStyleColor();
}

void ImGui::TextDisabled(const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    TextDisabledV(fmt, args);
    va_end(args);
}

void ImGui::TextDisabledV(const char* fmt, va_list args)
{
    // ImGuiContext& g = *GImGui;
    PushStyleColor(ImGuiCol_Text, g.Style.Colors[ImGuiCol_TextDisabled]);
    if (fmt[0] == '%' && fmt[1] == 's' && fmt[2] == 0)
        TextEx(va_arg(args, const char*), None, ImGuiTextFlags_NoWidthForLargeClippedText); // Skip formatting
    else
        TextV(fmt, args);
    PopStyleColor();
}

void ImGui::TextWrapped(const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    TextWrappedV(fmt, args);
    va_end(args);
}

void ImGui::TextWrappedV(const char* fmt, va_list args)
{
    // ImGuiContext& g = *GImGui;
    bool need_backup = (g.current_window_id->DC.TextWrapPos < 0.0);  // Keep existing wrap position if one is already set
    if (need_backup)
        PushTextWrapPos(0.0);
    if (fmt[0] == '%' && fmt[1] == 's' && fmt[2] == 0)
        TextEx(va_arg(args, const char*), None, ImGuiTextFlags_NoWidthForLargeClippedText); // Skip formatting
    else
        TextV(fmt, args);
    if (need_backup)
        PopTextWrapPos();
}

void ImGui::LabelText(const char* label, const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    LabelTextV(label, fmt, args);
    va_end(args);
}

// Add a label+text combo aligned to other label+value widgets
void ImGui::LabelTextV(const char* label, const char* fmt, va_list args)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;
    let w = CalcItemWidth();

    const char* value_text_begin, *value_text_end;
    ImFormatStringToTempBufferV(&value_text_begin, &value_text_end, fmt, args);
    const Vector2D value_size = CalcTextSize(value_text_begin, value_text_end, false);
    const Vector2D label_size = CalcTextSize(label, None, true);

    const Vector2D pos = window.DC.CursorPos;
    const ImRect value_bb(pos, pos + DimgVec2D::new(w, value_size.y + style.FramePadding.y * 2));
    const ImRect total_bb(pos, pos + DimgVec2D::new(w + (label_size.x > 0.0 ? style.ItemInnerSpacing.x + label_size.x : 0.0), ImMax(value_size.y, label_size.y) + style.FramePadding.y * 2));
    ItemSize(total_bb, style.FramePadding.y);
    if (!ItemAdd(total_bb, 0))
        return;

    // Render
    render_textClipped(value_bb.Min + style.FramePadding, value_bb.Max, value_text_begin, value_text_end, &value_size, DimgVec2D::new(0.0, 0.0));
    if (label_size.x > 0.0)
        render_text(DimgVec2D::new(value_bb.Max.x + style.ItemInnerSpacing.x, value_bb.Min.y + style.FramePadding.y), label);
}

void ImGui::BulletText(const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    BulletTextV(fmt, args);
    va_end(args);
}

// Text with a little bullet aligned to the typical tree node.
void ImGui::BulletTextV(const char* fmt, va_list args)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;

    const char* text_begin, *text_end;
    ImFormatStringToTempBufferV(&text_begin, &text_end, fmt, args);
    const Vector2D label_size = CalcTextSize(text_begin, text_end, false);
    const Vector2D total_size = DimgVec2D::new(g.FontSize + (label_size.x > 0.0 ? (label_size.x + style.FramePadding.x * 2) : 0.0), label_size.y);  // Empty text doesn't add padding
    Vector2D pos = window.DC.CursorPos;
    pos.y += window.DC.curr_line_text_base_offset;
    ItemSize(total_size, 0.0);
    const ImRect bb(pos, pos + total_size);
    if (!ItemAdd(bb, 0))
        return;

    // Render
    ImU32 text_col = GetColorU32(ImGuiCol_Text);
    RenderBullet(window.draw_list, bb.Min + DimgVec2D::new(style.FramePadding.x + g.FontSize * 0.5, g.FontSize * 0.5), text_col);
    render_text(bb.Min + DimgVec2D::new(g.FontSize + style.FramePadding.x * 2, 0.0), text_begin, text_end, false);
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

bool ImGui::ButtonBehavior(const ImRect& bb, Id32 id, bool* out_hovered, bool* out_held, ImGuiButtonFlags flags)
{
    // ImGuiContext& g = *GImGui;
    Window* window = GetCurrentWindow();

    // Default only reacts to left mouse button
    if ((flags & ImGuiButtonFlags_MouseButtonMask_) == 0)
        flags |= ImGuiButtonFlags_MouseButtonDefault_;

    // Default behavior requires click + release inside bounding box
    if ((flags & ImGuiButtonFlags_PressedOnMask_) == 0)
        flags |= ImGuiButtonFlags_PressedOnDefault_;

    Window* backup_hovered_window = g.HoveredWindow;
    const bool flatten_hovered_children = (flags & ImGuiButtonFlags_FlattenChildren) && g.HoveredWindow && g.HoveredWindow->RootWindowDockTree == window.RootWindowDockTree;
    if (flatten_hovered_children)
        g.HoveredWindow = window;

#ifdef IMGUI_ENABLE_TEST_ENGINE
    if (id != 0 && g.last_item_data.id != id)
        IMGUI_TEST_ENGINE_ITEM_ADD(bb, id);
#endif

    bool pressed = false;
    bool hovered = ItemHoverable(bb, id);

    // Drag source doesn't report as hovered
    if (hovered && g.DragDropActive && g.DragDropPayload.SourceId == id && !(g.DragDropSourceFlags & DragDropFlags::SourceNoDisableHover))
        hovered = false;

    // Special mode for Drag and Drop where holding button pressed for a long time while dragging another item triggers the button
    if (g.DragDropActive && (flags & ImGuiButtonFlags_PressedOnDragDropHold) && !(g.DragDropSourceFlags & DragDropFlags::SourceNoHoldToOpenOthers))
        if (IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByActiveItem))
        {
            hovered = true;
            set_hovered_id(id);
            if (g.HoveredIdTimer - g.IO.DeltaTime <= DRAGDROP_HOLD_TO_OPEN_TIMER && g.HoveredIdTimer >= DRAGDROP_HOLD_TO_OPEN_TIMER)
            {
                pressed = true;
                g.DragDropHoldJustPressedId = id;
                FocusWindow(window);
            }
        }

    if (flatten_hovered_children)
        g.HoveredWindow = backup_hovered_window;

    // AllowOverlap mode (rarely used) requires previous frame hovered_id to be null or to match. This allows using patterns where a later submitted widget overlaps a previous one.
    if (hovered && (flags & ImGuiButtonFlags_AllowItemOverlap) && (g.HoveredIdPreviousFrame != id && g.HoveredIdPreviousFrame != 0))
        hovered = false;

    // Mouse handling
    if (hovered)
    {
        if (!(flags & ImGuiButtonFlags_NoKeyModifiers) || (!g.IO.KeyCtrl && !g.IO.KeyShift && !g.IO.KeyAlt))
        {
            // Poll buttons
            int mouse_button_clicked = -1;
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
                        SetActiveID(id, window); // Hold on id
                    if (!(flags & ImGuiButtonFlags_NoNavFocus))
                        SetFocusID(id, window);
                    g.ActiveIdMouseButton = mouse_button_clicked;
                    FocusWindow(window);
                }
            }
            if (flags & ImGuiButtonFlags_PressedOnRelease)
            {
                int mouse_button_released = -1;
                if ((flags & ImGuiButtonFlags_MouseButtonLeft) && g.IO.MouseReleased[0])        { mouse_button_released = 0; }
                else if ((flags & ImGuiButtonFlags_MouseButtonRight) && g.IO.MouseReleased[1])  { mouse_button_released = 1; }
                else if ((flags & ImGuiButtonFlags_MouseButtonMiddle) && g.IO.MouseReleased[2]) { mouse_button_released = 2; }
                if (mouse_button_released != -1)
                {
                    const bool has_repeated_at_least_once = (flags & ImGuiButtonFlags_Repeat) && g.IO.MouseDownDurationPrev[mouse_button_released] >= g.IO.KeyRepeatDelay; // Repeat mode trumps on release behavior
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
                if (g.IO.MouseDownDuration[g.ActiveIdMouseButton] > 0.0 && is_mouse_clicked(g.ActiveIdMouseButton, true))
                    pressed = true;
        }

        if (pressed)
            g.NavDisableHighlight = true;
    }

    // Gamepad/Keyboard navigation
    // We report navigated item as hovered but we don't set g.hovered_id to not interfere with mouse.
    if (g.NavId == id && !g.NavDisableHighlight && g.NavDisableMouseHover && (g.ActiveId == 0 || g.ActiveId == id || g.ActiveId == window.MoveId))
        if (!(flags & ImGuiButtonFlags_NoHoveredOnFocus))
            hovered = true;
    if (g.NavActivateDownId == id)
    {
        bool nav_activated_by_code = (g.NavActivateId == id);
        bool nav_activated_by_inputs = IsNavInputTest(ImGuiNavInput_Activate, (flags & ImGuiButtonFlags_Repeat) ? ImGuiNavReadMode_Repeat : ImGuiNavReadMode_Pressed);
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
    bool held = false;
    if (g.ActiveId == id)
    {
        if (g.ActiveIdSource == ImGuiInputSource_Mouse)
        {
            if (g.ActiveIdIsJustActivated)
                g.active_id_click_offset = g.IO.MousePos - bb.Min;

            let mouse_button = g.ActiveIdMouseButton;
            IM_ASSERT(mouse_button >= 0 && mouse_button < MouseButton::COUNT);
            if (g.IO.MouseDown[mouse_button])
            {
                held = true;
            }
            else
            {
                bool release_in = hovered && (flags & ImGuiButtonFlags_PressedOnClickRelease) != 0;
                bool release_anywhere = (flags & ImGuiButtonFlags_PressedOnClickReleaseAnywhere) != 0;
                if ((release_in || release_anywhere) && !g.DragDropActive)
                {
                    // Report as pressed when releasing the mouse (this is the most common path)
                    bool is_double_click_release = (flags & ImGuiButtonFlags_PressedOnDoubleClick) && g.IO.MouseReleased[mouse_button] && g.IO.MouseClickedLastCount[mouse_button] == 2;
                    bool is_repeating_already = (flags & ImGuiButtonFlags_Repeat) && g.IO.MouseDownDurationPrev[mouse_button] >= g.IO.KeyRepeatDelay; // Repeat mode trumps <on release>
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

bool ImGui::ButtonEx(const char* label, const Vector2D& size_arg, ImGuiButtonFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;
    const Id32 id = window.GetID(label);
    const Vector2D label_size = CalcTextSize(label, None, true);

    Vector2D pos = window.DC.CursorPos;
    if ((flags & ImGuiButtonFlags_AlignTextBaseLine) && style.FramePadding.y < window.DC.curr_line_text_base_offset) // Try to vertically align buttons that are smaller/have no padding so that text baseline matches (bit hacky, since it shouldn't be a flag)
        pos.y += window.DC.curr_line_text_base_offset - style.FramePadding.y;
    Vector2D size = CalcItemSize(size_arg, label_size.x + style.FramePadding.x * 2.0, label_size.y + style.FramePadding.y * 2.0);

    const ImRect bb(pos, pos + size);
    ItemSize(size, style.FramePadding.y);
    if (!ItemAdd(bb, id))
        return false;

    if (g.last_item_data.in_flags & ItemFlags::ButtonRepeat)
        flags |= ImGuiButtonFlags_Repeat;

    bool hovered, held;
    bool pressed = ButtonBehavior(bb, id, &hovered, &held, flags);

    // Render
    const ImU32 col = GetColorU32((held && hovered) ? ImGuiCol_ButtonActive : hovered ? ImGuiCol_ButtonHovered : ImGuiCol_Button);
    RenderNavHighlight(bb, id);
    RenderFrame(bb.Min, bb.Max, col, true, style.frame_rounding);

    if (g.log_enabled)
        LogSetNextTextDecoration("[", "]");
    render_textClipped(bb.Min + style.FramePadding, bb.Max - style.FramePadding, label, None, &label_size, style.ButtonTextAlign, &bb);

    // Automatically close popups
    //if (pressed && !(flags & ImGuiButtonFlags_DontClosePopups) && (window->flags & WindowFlags_Popup))
    //    CloseCurrentPopup();

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.last_item_data.StatusFlags);
    return pressed;
}

bool ImGui::Button(const char* label, const Vector2D& size_arg)
{
    return ButtonEx(label, size_arg, ImGuiButtonFlags_None);
}

// Small buttons fits within text without additional vertical spacing.
bool ImGui::SmallButton(const char* label)
{
    // ImGuiContext& g = *GImGui;
    let backup_padding_y =  g.Style.FramePadding.y;
    g.Style.FramePadding.y = 0.0;
    bool pressed = ButtonEx(label, DimgVec2D::new(0, 0), ImGuiButtonFlags_AlignTextBaseLine);
    g.Style.FramePadding.y = backup_padding_y;
    return pressed;
}

// Tip: use ImGui::push_id()/PopID() to push indices or pointers in the id stack.
// Then you can keep 'str_id' empty or the same for all your buttons (instead of creating a string based on a non-string id)
bool ImGui::InvisibleButton(const char* str_id, const Vector2D& size_arg, ImGuiButtonFlags flags)
{
    // ImGuiContext& g = *GImGui;
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // Cannot use zero-size for InvisibleButton(). Unlike Button() there is not way to fallback using the label size.
    IM_ASSERT(size_arg.x != 0.0 && size_arg.y != 0.0);

    const Id32 id = window.GetID(str_id);
    Vector2D size = CalcItemSize(size_arg, 0.0, 0.0);
    const ImRect bb(window.DC.CursorPos, window.DC.CursorPos + size);
    ItemSize(size);
    if (!ItemAdd(bb, id))
        return false;

    bool hovered, held;
    bool pressed = ButtonBehavior(bb, id, &hovered, &held, flags);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, str_id, g.last_item_data.StatusFlags);
    return pressed;
}

bool ImGui::ArrowButtonEx(const char* str_id, ImGuiDir dir, Vector2D size, ImGuiButtonFlags flags)
{
    // ImGuiContext& g = *GImGui;
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    const Id32 id = window.GetID(str_id);
    const ImRect bb(window.DC.CursorPos, window.DC.CursorPos + size);
    let default_size = get_frame_height();
    ItemSize(size, (size.y >= default_size) ? g.Style.FramePadding.y : -1.0);
    if (!ItemAdd(bb, id))
        return false;

    if (g.last_item_data.in_flags & ItemFlags::ButtonRepeat)
        flags |= ImGuiButtonFlags_Repeat;

    bool hovered, held;
    bool pressed = ButtonBehavior(bb, id, &hovered, &held, flags);

    // Render
    const ImU32 bg_col = GetColorU32((held && hovered) ? ImGuiCol_ButtonActive : hovered ? ImGuiCol_ButtonHovered : ImGuiCol_Button);
    const ImU32 text_col = GetColorU32(ImGuiCol_Text);
    RenderNavHighlight(bb, id);
    RenderFrame(bb.Min, bb.Max, bg_col, true, g.Style.frame_rounding);
    RenderArrow(window.draw_list, bb.Min + DimgVec2D::new(ImMax(0.0, (size.x - g.FontSize) * 0.5), ImMax(0.0, (size.y - g.FontSize) * 0.5)), text_col, dir);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, str_id, g.last_item_data.StatusFlags);
    return pressed;
}

bool ImGui::ArrowButton(const char* str_id, ImGuiDir dir)
{
    let sz =  get_frame_height();
    return ArrowButtonEx(str_id, dir, DimgVec2D::new(sz, sz), ImGuiButtonFlags_None);
}

// Button to close a window
bool ImGui::CloseButton(Id32 id, const Vector2D& pos)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;

    // Tweak 1: Shrink hit-testing area if button covers an abnormally large proportion of the visible region. That's in order to facilitate moving the window away. (#3825)
    // This may better be applied as a general hit-rect reduction mechanism for all widgets to ensure the area to move window is always accessible?
    const ImRect bb(pos, pos + DimgVec2D::new(g.FontSize, g.FontSize) + g.Style.FramePadding * 2.0);
    ImRect bb_interact = bb;
    let area_to_visible_ratio = window.OuterRectClipped.GetArea() / bb.GetArea();
    if (area_to_visible_ratio < 1.5)
        bb_interact.Expand(ImFloor(bb_interact.GetSize() * -0.25));

    // Tweak 2: We intentionally allow interaction when clipped so that a mechanical Alt,Right,Activate sequence can always close a window.
    // (this isn't the regular behavior of buttons, but it doesn't affect the user much because navigation tends to keep items visible).
    bool is_clipped = !ItemAdd(bb_interact, id);

    bool hovered, held;
    bool pressed = ButtonBehavior(bb_interact, id, &hovered, &held);
    if (is_clipped)
        return pressed;

    // Render
    // FIXME: Clarify this mess
    ImU32 col = GetColorU32(held ? ImGuiCol_ButtonActive : ImGuiCol_ButtonHovered);
    Vector2D center = bb.get_center();
    if (hovered)
        window.draw_list->AddCircleFilled(center, ImMax(2.0, g.FontSize * 0.5 + 1.0), col, 12);

    let cross_extent =  g.FontSize * 0.5 * 0.7071 - 1.0;
    ImU32 cross_col = GetColorU32(ImGuiCol_Text);
    center -= DimgVec2D::new(0.5, 0.5);
    window.draw_list->AddLine(center + DimgVec2D::new(+cross_extent, +cross_extent), center + DimgVec2D::new(-cross_extent, -cross_extent), cross_col, 1.0);
    window.draw_list->AddLine(center + DimgVec2D::new(+cross_extent, -cross_extent), center + DimgVec2D::new(-cross_extent, +cross_extent), cross_col, 1.0);

    return pressed;
}

// The Collapse button also functions as a Dock Menu button.
bool ImGui::CollapseButton(Id32 id, const Vector2D& pos, ImGuiDockNode* dock_node)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;

    ImRect bb(pos, pos + DimgVec2D::new(g.FontSize, g.FontSize) + g.Style.FramePadding * 2.0);
    ItemAdd(bb, id);
    bool hovered, held;
    bool pressed = ButtonBehavior(bb, id, &hovered, &held, ImGuiButtonFlags_None);

    // Render
    //bool is_dock_menu = (window->dock_node_as_host && !window->collapsed);
    ImU32 bg_col = GetColorU32((held && hovered) ? ImGuiCol_ButtonActive : hovered ? ImGuiCol_ButtonHovered : ImGuiCol_Button);
    ImU32 text_col = GetColorU32(ImGuiCol_Text);
    if (hovered || held)
        window.draw_list->AddCircleFilled(bb.get_center() + DimgVec2D::new(0,-0.5), g.FontSize * 0.5 + 1.0, bg_col, 12);

    if (dock_node)
        RenderArrowDockMenu(window.draw_list, bb.Min + g.Style.FramePadding, g.FontSize, text_col);
    else
        RenderArrow(window.draw_list, bb.Min + g.Style.FramePadding, text_col, window.Collapsed ? ImGuiDir_Right : ImGuiDir_Down, 1.0);

    // Switch to moving the window after mouse is moved beyond the initial drag threshold
    if (is_item_active() && IsMouseDragging(0))
        StartMouseMovingWindowOrNode(window, dock_node, true);

    return pressed;
}

Id32 ImGui::GetWindowScrollbarID(Window* window, ImGuiAxis axis)
{
    return window.GetID(axis == ImGuiAxis_X ? "#SCROLLX" : "#SCROLLY");
}

// Return scrollbar rectangle, must only be called for corresponding axis if window->scrollbar_x/Y is set.
ImRect ImGui::GetWindowScrollbarRect(Window* window, ImGuiAxis axis)
{
    const ImRect outer_rect = window.Rect();
    const ImRect inner_rect = window.InnerRect;
    let border_size = window.WindowBorderSize;
    let scrollbar_size = window.ScrollbarSizes[axis ^ 1]; // (scrollbar_sizes.x = width of Y scrollbar; scrollbar_sizes.y = height of x scrollbar)
    IM_ASSERT(scrollbar_size > 0.0);
    if (axis == ImGuiAxis_X)
        return ImRect(inner_rect.Min.x, ImMax(outer_rect.Min.y, outer_rect.Max.y - border_size - scrollbar_size), inner_rect.Max.x, outer_rect.Max.y);
    else
        return ImRect(ImMax(outer_rect.Min.x, outer_rect.Max.x - border_size - scrollbar_size), inner_rect.Min.y, outer_rect.Max.x, inner_rect.Max.y);
}

void ImGui::Scrollbar(ImGuiAxis axis)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    const Id32 id = GetWindowScrollbarID(window, axis);

    // Calculate scrollbar bounding box
    ImRect bb = GetWindowScrollbarRect(window, axis);
    ImDrawFlags rounding_corners = ImDrawFlags_RoundCornersNone;
    if (axis == ImGuiAxis_X)
    {
        rounding_corners |= ImDrawFlags_RoundCornersBottomLeft;
        if (!window.ScrollbarY)
            rounding_corners |= ImDrawFlags_RoundCornersBottomRight;
    }
    else
    {
        if ((window.Flags & WindowFlags_NoTitleBar) && !(window.Flags & WindowFlags_MenuBar))
            rounding_corners |= ImDrawFlags_RoundCornersTopRight;
        if (!window.ScrollbarX)
            rounding_corners |= ImDrawFlags_RoundCornersBottomRight;
    }
    let size_avail =  window.InnerRect.Max[axis] - window.InnerRect.Min[axis];
    let size_contents =  window.ContentSize[axis] + window.WindowPadding[axis] * 2.0;
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
bool ImGui::ScrollbarEx(const ImRect& bb_frame, Id32 id, ImGuiAxis axis, ImS64* p_scroll_v, ImS64 size_avail_v, ImS64 size_contents_v, ImDrawFlags flags)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    if (window.SkipItems)
        return false;

    KeepAliveID(id);

    let bb_frame_width = bb_frame.GetWidth();
    let bb_frame_height = bb_frame.GetHeight();
    if (bb_frame_width <= 0.0 || bb_frame_height <= 0.0)
        return false;

    // When we are too small, start hiding and disabling the grab (this reduce visual noise on very small window and facilitate using the window resize grab)
    let alpha =  1.0;
    if ((axis == ImGuiAxis_Y) && bb_frame_height < g.FontSize + g.Style.FramePadding.y * 2.0)
        alpha = ImSaturate((bb_frame_height - g.FontSize) / (g.Style.FramePadding.y * 2.0));
    if (alpha <= 0.0)
        return false;

    const ImGuiStyle& style = g.Style;
    const bool allow_interaction = (alpha >= 1.0);

    ImRect bb = bb_frame;
    bb.Expand(DimgVec2D::new(-ImClamp(IM_FLOOR((bb_frame_width - 2.0) * 0.5), 0.0, 3.0), -ImClamp(IM_FLOOR((bb_frame_height - 2.0) * 0.5), 0.0, 3.0)));

    // V denote the main, longer axis of the scrollbar (= height for a vertical scrollbar)
    let scrollbar_size_v = (axis == ImGuiAxis_X) ? bb.GetWidth() : bb.GetHeight();

    // Calculate the height of our grabbable box. It generally represent the amount visible (vs the total scrollable amount)
    // But we maintain a minimum size in pixel to allow for the user to still aim inside.
    IM_ASSERT(ImMax(size_contents_v, size_avail_v) > 0.0); // Adding this assert to check if the ImMax(XXX,1.0) is still needed. PLEASE CONTACT ME if this triggers.
    const ImS64 win_size_v = ImMax(ImMax(size_contents_v, size_avail_v), (ImS64)1);
    let grab_h_pixels = ImClamp(scrollbar_size_v * (size_avail_v / win_size_v), style.GrabMinSize, scrollbar_size_v);
    let grab_h_norm = grab_h_pixels / scrollbar_size_v;

    // Handle input right away. None of the code of Begin() is relying on scrolling position before calling Scrollbar().
    bool held = false;
    bool hovered = false;
    ButtonBehavior(bb, id, &hovered, &held, ImGuiButtonFlags_NoNavFocus);

    const ImS64 scroll_max = ImMax((ImS64)1, size_contents_v - size_avail_v);
    let scroll_ratio =  ImSaturate(*p_scroll_v / scroll_max);
    let grab_v_norm =  scroll_ratio * (scrollbar_size_v - grab_h_pixels) / scrollbar_size_v; // Grab position in normalized space
    if (held && allow_interaction && grab_h_norm < 1.0)
    {
        let scrollbar_pos_v = bb.Min[axis];
        let mouse_pos_v = g.IO.MousePos[axis];

        // Click position in scrollbar normalized space (0.0->1.0)
        let clicked_v_norm = ImSaturate((mouse_pos_v - scrollbar_pos_v) / scrollbar_size_v);
        set_hovered_id(id);

        bool seek_absolute = false;
        if (g.ActiveIdIsJustActivated)
        {
            // On initial click calculate the distance between mouse and the center of the grab
            seek_absolute = (clicked_v_norm < grab_v_norm || clicked_v_norm > grab_v_norm + grab_h_norm);
            if (seek_absolute)
                g.ScrollbarClickDeltaToGrabCenter = 0.0;
            else
                g.ScrollbarClickDeltaToGrabCenter = clicked_v_norm - grab_v_norm - grab_h_norm * 0.5;
        }

        // Apply scroll (p_scroll_v will generally point on one member of window->scroll)
        // It is ok to modify scroll here because we are being called in Begin() after the calculation of content_size and before setting up our starting position
        let scroll_v_norm = ImSaturate((clicked_v_norm - g.ScrollbarClickDeltaToGrabCenter - grab_h_norm * 0.5) / (1.0 - grab_h_norm));
        *p_scroll_v = (ImS64)(scroll_v_norm * scroll_max);

        // Update values for rendering
        scroll_ratio = ImSaturate(*p_scroll_v / scroll_max);
        grab_v_norm = scroll_ratio * (scrollbar_size_v - grab_h_pixels) / scrollbar_size_v;

        // Update distance to grab now that we have seeked and saturated
        if (seek_absolute)
            g.ScrollbarClickDeltaToGrabCenter = clicked_v_norm - grab_v_norm - grab_h_norm * 0.5;
    }

    // Render
    const ImU32 bg_col = GetColorU32(ImGuiCol_ScrollbarBg);
    const ImU32 grab_col = GetColorU32(held ? ImGuiCol_ScrollbarGrabActive : hovered ? ImGuiCol_ScrollbarGrabHovered : ImGuiCol_ScrollbarGrab, alpha);
    window.draw_list->AddRectFilled(bb_frame.Min, bb_frame.Max, bg_col, window.WindowRounding, flags);
    ImRect grab_rect;
    if (axis == ImGuiAxis_X)
        grab_rect = ImRect(ImLerp(bb.Min.x, bb.Max.x, grab_v_norm), bb.Min.y, ImLerp(bb.Min.x, bb.Max.x, grab_v_norm) + grab_h_pixels, bb.Max.y);
    else
        grab_rect = ImRect(bb.Min.x, ImLerp(bb.Min.y, bb.Max.y, grab_v_norm), bb.Max.x, ImLerp(bb.Min.y, bb.Max.y, grab_v_norm) + grab_h_pixels);
    window.draw_list->AddRectFilled(grab_rect.Min, grab_rect.Max, grab_col, style.ScrollbarRounding);

    return held;
}

void ImGui::Image(ImTextureID user_texture_id, const Vector2D& size, const Vector2D& uv0, const Vector2D& uv1, const Vector4D& tint_col, const Vector4D& border_col)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    ImRect bb(window.DC.CursorPos, window.DC.CursorPos + size);
    if (border_col.w > 0.0)
        bb.Max += DimgVec2D::new(2, 2);
    ItemSize(bb);
    if (!ItemAdd(bb, 0))
        return;

    if (border_col.w > 0.0)
    {
        window.draw_list->AddRect(bb.Min, bb.Max, GetColorU32(border_col), 0.0);
        window.draw_list->AddImage(user_texture_id, bb.Min + DimgVec2D::new(1, 1), bb.Max - DimgVec2D::new(1, 1), uv0, uv1, GetColorU32(tint_col));
    }
    else
    {
        window.draw_list->AddImage(user_texture_id, bb.Min, bb.Max, uv0, uv1, GetColorU32(tint_col));
    }
}

// ImageButton() is flawed as 'id' is always derived from 'texture_id' (see #2464 #1390)
// We provide this internal helper to write your own variant while we figure out how to redesign the public ImageButton() API.
bool ImGui::ImageButtonEx(Id32 id, ImTextureID texture_id, const Vector2D& size, const Vector2D& uv0, const Vector2D& uv1, const Vector2D& padding, const Vector4D& bg_col, const Vector4D& tint_col)
{
    // ImGuiContext& g = *GImGui;
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    const ImRect bb(window.DC.CursorPos, window.DC.CursorPos + size + padding * 2);
    ItemSize(bb);
    if (!ItemAdd(bb, id))
        return false;

    bool hovered, held;
    bool pressed = ButtonBehavior(bb, id, &hovered, &held);

    // Render
    const ImU32 col = GetColorU32((held && hovered) ? ImGuiCol_ButtonActive : hovered ? ImGuiCol_ButtonHovered : ImGuiCol_Button);
    RenderNavHighlight(bb, id);
    RenderFrame(bb.Min, bb.Max, col, true, ImClamp(ImMin(padding.x, padding.y), 0.0, g.Style.frame_rounding));
    if (bg_col.w > 0.0)
        window.draw_list->AddRectFilled(bb.Min + padding, bb.Max - padding, GetColorU32(bg_col));
    window.draw_list->AddImage(texture_id, bb.Min + padding, bb.Max - padding, uv0, uv1, GetColorU32(tint_col));

    return pressed;
}

// frame_padding < 0: uses FramePadding from style (default)
// frame_padding = 0: no framing
// frame_padding > 0: set framing size
bool ImGui::ImageButton(ImTextureID user_texture_id, const Vector2D& size, const Vector2D& uv0, const Vector2D& uv1, int frame_padding, const Vector4D& bg_col, const Vector4D& tint_col)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    if (window.SkipItems)
        return false;

    // Default to using texture id as id. User can still push string/integer prefixes.
    push_id((void*)(intptr_t)user_texture_id);
    const Id32 id = window.GetID("#image");
    pop_id();

    const Vector2D padding = (frame_padding >= 0) ? DimgVec2D::new(frame_padding, frame_padding) : g.Style.FramePadding;
    return ImageButtonEx(id, user_texture_id, size, uv0, uv1, padding, bg_col, tint_col);
}

bool ImGui::Checkbox(const char* label, bool* v)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;
    const Id32 id = window.GetID(label);
    const Vector2D label_size = CalcTextSize(label, None, true);

    let square_sz = get_frame_height();
    const Vector2D pos = window.DC.CursorPos;
    const ImRect total_bb(pos, pos + DimgVec2D::new(square_sz + (label_size.x > 0.0 ? style.ItemInnerSpacing.x + label_size.x : 0.0), label_size.y + style.FramePadding.y * 2.0));
    ItemSize(total_bb, style.FramePadding.y);
    if (!ItemAdd(total_bb, id))
    {
        IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.last_item_data.StatusFlags | ItemStatusFlags::Checkable | (*v ? ItemStatusFlags::Checked : 0));
        return false;
    }

    bool hovered, held;
    bool pressed = ButtonBehavior(total_bb, id, &hovered, &held);
    if (pressed)
    {
        *v = !(*v);
        MarkItemEdited(id);
    }

    const ImRect check_bb(pos, pos + DimgVec2D::new(square_sz, square_sz));
    RenderNavHighlight(total_bb, id);
    RenderFrame(check_bb.Min, check_bb.Max, GetColorU32((held && hovered) ? ImGuiCol_FrameBgActive : hovered ? ImGuiCol_FrameBgHovered : ImGuiCol_FrameBg), true, style.frame_rounding);
    ImU32 check_col = GetColorU32(ImGuiCol_CheckMark);
    bool mixed_value = (g.last_item_data.in_flags & ItemFlags::MixedValue) != 0;
    if (mixed_value)
    {
        // Undocumented tristate/mixed/indeterminate checkbox (#2644)
        // This may seem awkwardly designed because the aim is to make ImGuiItemFlags_MixedValue supported by all widgets (not just checkbox)
        Vector2D pad(ImMax(1.0, IM_FLOOR(square_sz / 3.6)), ImMax(1.0, IM_FLOOR(square_sz / 3.6)));
        window.draw_list->AddRectFilled(check_bb.Min + pad, check_bb.Max - pad, check_col, style.frame_rounding);
    }
    else if (*v)
    {
        let pad = ImMax(1.0, IM_FLOOR(square_sz / 6.0));
        RenderCheckMark(window.draw_list, check_bb.Min + DimgVec2D::new(pad, pad), check_col, square_sz - pad * 2.0);
    }

    Vector2D label_pos = DimgVec2D::new(check_bb.Max.x + style.ItemInnerSpacing.x, check_bb.Min.y + style.FramePadding.y);
    if (g.log_enabled)
        LogRenderedText(&label_pos, mixed_value ? "[~]" : *v ? "[x]" : "[ ]");
    if (label_size.x > 0.0)
        render_text(label_pos, label);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.last_item_data.StatusFlags | ItemStatusFlags::Checkable | (*v ? ItemStatusFlags::Checked : 0));
    return pressed;
}

template<typename T>
bool ImGui::CheckboxFlagsT(const char* label, T* flags, T flags_value)
{
    bool all_on = (*flags & flags_value) == flags_value;
    bool any_on = (*flags & flags_value) != 0;
    bool pressed;
    if (!all_on && any_on)
    {
        // ImGuiContext& g = *GImGui;
        ImGuiItemFlags backup_item_flags = g.CurrentItemFlags;
        g.CurrentItemFlags |= ItemFlags::MixedValue;
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
            *flags &= ~flags_value;
    }
    return pressed;
}

bool ImGui::CheckboxFlags(const char* label, int* flags, int flags_value)
{
    return CheckboxFlagsT(label, flags, flags_value);
}

bool ImGui::CheckboxFlags(const char* label, unsigned int* flags, unsigned int flags_value)
{
    return CheckboxFlagsT(label, flags, flags_value);
}

bool ImGui::CheckboxFlags(const char* label, ImS64* flags, ImS64 flags_value)
{
    return CheckboxFlagsT(label, flags, flags_value);
}

bool ImGui::CheckboxFlags(const char* label, ImU64* flags, ImU64 flags_value)
{
    return CheckboxFlagsT(label, flags, flags_value);
}

bool ImGui::RadioButton(const char* label, bool active)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;
    const Id32 id = window.GetID(label);
    const Vector2D label_size = CalcTextSize(label, None, true);

    let square_sz = get_frame_height();
    const Vector2D pos = window.DC.CursorPos;
    const ImRect check_bb(pos, pos + DimgVec2D::new(square_sz, square_sz));
    const ImRect total_bb(pos, pos + DimgVec2D::new(square_sz + (label_size.x > 0.0 ? style.ItemInnerSpacing.x + label_size.x : 0.0), label_size.y + style.FramePadding.y * 2.0));
    ItemSize(total_bb, style.FramePadding.y);
    if (!ItemAdd(total_bb, id))
        return false;

    Vector2D center = check_bb.get_center();
    center.x = IM_ROUND(center.x);
    center.y = IM_ROUND(center.y);
    let radius = (square_sz - 1.0) * 0.5;

    bool hovered, held;
    bool pressed = ButtonBehavior(total_bb, id, &hovered, &held);
    if (pressed)
        MarkItemEdited(id);

    RenderNavHighlight(total_bb, id);
    window.draw_list->AddCircleFilled(center, radius, GetColorU32((held && hovered) ? ImGuiCol_FrameBgActive : hovered ? ImGuiCol_FrameBgHovered : ImGuiCol_FrameBg), 16);
    if (active)
    {
        let pad = ImMax(1.0, IM_FLOOR(square_sz / 6.0));
        window.draw_list->AddCircleFilled(center, radius - pad, GetColorU32(ImGuiCol_CheckMark), 16);
    }

    if (style.FrameBorderSize > 0.0)
    {
        window.draw_list->AddCircle(center + DimgVec2D::new(1, 1), radius, GetColorU32(ImGuiCol_BorderShadow), 16, style.FrameBorderSize);
        window.draw_list->AddCircle(center, radius, GetColorU32(ImGuiCol_Border), 16, style.FrameBorderSize);
    }

    Vector2D label_pos = DimgVec2D::new(check_bb.Max.x + style.ItemInnerSpacing.x, check_bb.Min.y + style.FramePadding.y);
    if (g.log_enabled)
        LogRenderedText(&label_pos, active ? "(x)" : "( )");
    if (label_size.x > 0.0)
        render_text(label_pos, label);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.last_item_data.StatusFlags);
    return pressed;
}

// FIXME: This would work nicely if it was a public template, e.g. 'template<T> RadioButton(const char* label, T* v, T v_button)', but I'm not sure how we would expose it..
bool ImGui::RadioButton(const char* label, int* v, int v_button)
{
    const bool pressed = RadioButton(label, *v == v_button);
    if (pressed)
        *v = v_button;
    return pressed;
}

// size_arg (for each axis) < 0.0: align to end, 0.0: auto, > 0.0: specified size
void ImGui::ProgressBar(float fraction, const Vector2D& size_arg, const char* overlay)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;

    Vector2D pos = window.DC.CursorPos;
    Vector2D size = CalcItemSize(size_arg, CalcItemWidth(), g.FontSize + style.FramePadding.y * 2.0);
    ImRect bb(pos, pos + size);
    ItemSize(size, style.FramePadding.y);
    if (!ItemAdd(bb, 0))
        return;

    // Render
    fraction = ImSaturate(fraction);
    RenderFrame(bb.Min, bb.Max, GetColorU32(ImGuiCol_FrameBg), true, style.frame_rounding);
    bb.Expand(DimgVec2D::new(-style.FrameBorderSize, -style.FrameBorderSize));
    const Vector2D fill_br = DimgVec2D::new(ImLerp(bb.Min.x, bb.Max.x, fraction), bb.Max.y);
    RenderRectFilledRangeH(window.draw_list, bb, GetColorU32(ImGuiCol_PlotHistogram), 0.0, fraction, style.frame_rounding);

    // Default displaying the fraction as percentage string, but user can override it
    char overlay_buf[32];
    if (!overlay)
    {
        ImFormatString(overlay_buf, IM_ARRAYSIZE(overlay_buf), "%.0%%", fraction * 100 + 0.01);
        overlay = overlay_buf;
    }

    Vector2D overlay_size = CalcTextSize(overlay, None);
    if (overlay_size.x > 0.0)
        render_textClipped(DimgVec2D::new(ImClamp(fill_br.x + style.item_spacing.x, bb.Min.x, bb.Max.x - overlay_size.x - style.ItemInnerSpacing.x), bb.Min.y), bb.Max, overlay, None, &overlay_size, DimgVec2D::new(0.0, 0.5), &bb);
}

void ImGui::Bullet()
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;
    let line_height = ImMax(ImMin(window.DC.curr_line_size.y, g.FontSize + style.FramePadding.y * 2), g.FontSize);
    const ImRect bb(window.DC.CursorPos, window.DC.CursorPos + DimgVec2D::new(g.FontSize, line_height));
    ItemSize(bb);
    if (!ItemAdd(bb, 0))
    {
        same_line(0, style.FramePadding.x * 2);
        return;
    }

    // Render and stay on same line
    ImU32 text_col = GetColorU32(ImGuiCol_Text);
    RenderBullet(window.draw_list, bb.Min + DimgVec2D::new(style.FramePadding.x + g.FontSize * 0.5, line_height * 0.5), text_col);
    same_line(0, style.FramePadding.x * 2.0);
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
// - splitter_behavior() [Internal]
// - ShrinkWidths() [Internal]
//-------------------------------------------------------------------------

void ImGui::Spacing()
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return;
    ItemSize(DimgVec2D::new(0, 0));
}

void ImGui::Dummy(const Vector2D& size)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    const ImRect bb(window.DC.CursorPos, window.DC.CursorPos + size);
    ItemSize(size);
    ItemAdd(bb, 0);
}

void ImGui::NewLine()
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    // ImGuiContext& g = *GImGui;
    const ImGuiLayoutType backup_layout_type = window.DC.layout_type;
    window.DC.layout_type = ImGuiLayoutType_Vertical;
    window.DC.Issame_line = false;
    if (window.DC.curr_line_size.y > 0.0)     // In the event that we are on a line with items that is smaller that font_size high, we will preserve its height.
        ItemSize(DimgVec2D::new(0, 0));
    else
        ItemSize(DimgVec2D::new(0.0, g.FontSize));
    window.DC.layout_type = backup_layout_type;
}

void ImGui::AlignTextToFramePadding()
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    // ImGuiContext& g = *GImGui;
    window.DC.curr_line_size.y = ImMax(window.DC.curr_line_size.y, g.FontSize + g.Style.FramePadding.y * 2);
    window.DC.curr_line_text_base_offset = ImMax(window.DC.curr_line_text_base_offset, g.Style.FramePadding.y);
}

// Horizontal/vertical separating line
void ImGui::SeparatorEx(ImGuiSeparatorFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return;

    // ImGuiContext& g = *GImGui;
    IM_ASSERT(ImIsPowerOfTwo(flags & (ImGuiSeparatorFlags_Horizontal | ImGuiSeparatorFlags_Vertical)));   // Check that only 1 option is selected

    let thickness_draw =  1.0;
    let thickness_layout =  0.0;
    if (flags & ImGuiSeparatorFlags_Vertical)
    {
        // Vertical separator, for menu bars (use current line height). Not exposed because it is misleading and it doesn't have an effect on regular layout.
        let y1 =  window.DC.CursorPos.y;
        let y2 =  window.DC.CursorPos.y + window.DC.curr_line_size.y;
        const ImRect bb(DimgVec2D::new(window.DC.CursorPos.x, y1), DimgVec2D::new(window.DC.CursorPos.x + thickness_draw, y2));
        ItemSize(DimgVec2D::new(thickness_layout, 0.0));
        if (!ItemAdd(bb, 0))
            return;

        // Draw
        window.draw_list->AddLine(DimgVec2D::new(bb.Min.x, bb.Min.y), DimgVec2D::new(bb.Min.x, bb.Max.y), GetColorU32(ImGuiCol_Separator));
        if (g.log_enabled)
            LogText(" |");
    }
    else if (flags & ImGuiSeparatorFlags_Horizontal)
    {
        // Horizontal Separator
        let x1 =  window.pos.x;
        let x2 =  window.pos.x + window.Size.x;

        // FIXME-WORKRECT: old hack (#205) until we decide of consistent behavior with work_rect/Indent and Separator
        if (g.GroupStack.Size > 0 && g.GroupStack.back().window_id == window.id)
            x1 += window.DC.indent.x;

        // FIXME-WORKRECT: In theory we should simply be using work_rect.min.x/max.x everywhere but it isn't aesthetically what we want,
        // need to introduce a variant of work_rect for that purpose. (#4787)
        if (ImGuiTable* table = g.current_table)
        {
            x1 = table->Columns[table->CurrentColumn].MinX;
            x2 = table->Columns[table->CurrentColumn].MaxX;
        }

        ImGuiOldColumns* columns = (flags & ImGuiSeparatorFlags_SpanAllColumns) ? window.DC.current_columns : None;
        if (columns)
            PushColumnsBackground();

        // We don't provide our width to the layout so that it doesn't get feed back into AutoFit
        // FIXME: This prevents ->CursorMaxPos based bounding box evaluation from working (e.g. TableEndCell)
        const ImRect bb(DimgVec2D::new(x1, window.DC.CursorPos.y), DimgVec2D::new(x2, window.DC.CursorPos.y + thickness_draw));
        ItemSize(DimgVec2D::new(0.0, thickness_layout));
        const bool item_visible = ItemAdd(bb, 0);
        if (item_visible)
        {
            // Draw
            window.draw_list->AddLine(bb.Min, DimgVec2D::new(bb.Max.x, bb.Min.y), GetColorU32(ImGuiCol_Separator));
            if (g.log_enabled)
                LogRenderedText(&bb.Min, "--------------------------------\n");

        }
        if (columns)
        {
            PopColumnsBackground();
            columns->LineMinY = window.DC.CursorPos.y;
        }
    }
}

void ImGui::Separator()
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    if (window.SkipItems)
        return;

    // Those flags should eventually be overridable by the user
    ImGuiSeparatorFlags flags = (window.DC.layout_type == LayoutType::Horizontal) ? ImGuiSeparatorFlags_Vertical : ImGuiSeparatorFlags_Horizontal;
    flags |= ImGuiSeparatorFlags_SpanAllColumns; // NB: this only applies to legacy columns() api as they relied on Separator() a lot.
    SeparatorEx(flags);
}

// Using 'hover_visibility_delay' allows us to hide the highlight and mouse cursor for a short time, which can be convenient to reduce visual noise.
bool ImGui::splitter_behavior(const ImRect& bb, Id32 id, ImGuiAxis axis, float* size1, float* size2, float min_size1, float min_size2, float hover_extend, float hover_visibility_delay, ImU32 bg_col)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;

    const ImGuiItemFlags item_flags_backup = g.CurrentItemFlags;
    g.CurrentItemFlags |= ItemFlags::NoNav | ItemFlags::NoNavDefaultFocus;
    bool item_add = ItemAdd(bb, id);
    g.CurrentItemFlags = item_flags_backup;
    if (!item_add)
        return false;

    bool hovered, held;
    ImRect bb_interact = bb;
    bb_interact.Expand(axis == ImGuiAxis_Y ? DimgVec2D::new(0.0, hover_extend) : DimgVec2D::new(hover_extend, 0.0));
    ButtonBehavior(bb_interact, id, &hovered, &held, ImGuiButtonFlags_FlattenChildren | ImGuiButtonFlags_AllowItemOverlap);
    if (hovered)
        g.last_item_data.StatusFlags |= ItemStatusFlags::HoveredRect; // for IsItemHovered(), because bb_interact is larger than bb
    if (g.ActiveId != id)
        SetItemAllowOverlap();

    if (held || (hovered && g.HoveredIdPreviousFrame == id && g.HoveredIdTimer >= hover_visibility_delay))
        SetMouseCursor(axis == ImGuiAxis_Y ? ImGuiMouseCursor_ResizeNS : ImGuiMouseCursor_ResizeEW);

    ImRect bb_render = bb;
    if (held)
    {
        Vector2D mouse_delta_2d = g.IO.MousePos - g.active_id_click_offset - bb_interact.Min;
        let mouse_delta =  (axis == ImGuiAxis_Y) ? mouse_delta_2d.y : mouse_delta_2d.x;

        // Minimum pane size
        let size_1_maximum_delta =  ImMax(0.0, *size1 - min_size1);
        let size_2_maximum_delta =  ImMax(0.0, *size2 - min_size2);
        if (mouse_delta < -size_1_maximum_delta)
            mouse_delta = -size_1_maximum_delta;
        if (mouse_delta > size_2_maximum_delta)
            mouse_delta = size_2_maximum_delta;

        // Apply resize
        if (mouse_delta != 0.0)
        {
            if (mouse_delta < 0.0)
                IM_ASSERT(*size1 + mouse_delta >= min_size1);
            if (mouse_delta > 0.0)
                IM_ASSERT(*size2 - mouse_delta >= min_size2);
            *size1 += mouse_delta;
            *size2 -= mouse_delta;
            bb_render.Translate((axis == ImGuiAxis_X) ? DimgVec2D::new(mouse_delta, 0.0) : DimgVec2D::new(0.0, mouse_delta));
            MarkItemEdited(id);
        }
    }

    // Render at new position
    if (bg_col & COLOR32_A_MASK)
        window.draw_list->AddRectFilled(bb_render.Min, bb_render.Max, bg_col, 0.0);
    const ImU32 col = GetColorU32(held ? ImGuiCol_SeparatorActive : (hovered && g.HoveredIdTimer >= hover_visibility_delay) ? ImGuiCol_SeparatorHovered : ImGuiCol_Separator);
    window.draw_list->AddRectFilled(bb_render.Min, bb_render.Max, col, 0.0);

    return held;
}

static int IMGUI_CDECL ShrinkWidthItemComparer(const void* lhs, const void* rhs)
{
    const ImGuiShrinkWidthItem* a = (const ImGuiShrinkWidthItem*)lhs;
    const ImGuiShrinkWidthItem* b = (const ImGuiShrinkWidthItem*)rhs;
    if (int d = (b->Width - a->Width))
        return d;
    return (b->Index - a->Index);
}

// Shrink excess width from a set of item, by removing width from the larger items first.
// Set items width to -1.0 to disable shrinking this item.
void ImGui::ShrinkWidths(ImGuiShrinkWidthItem* items, int count, float width_excess)
{
    if (count == 1)
    {
        if (items[0].width >= 0.0)
            items[0].width = ImMax(items[0].width - width_excess, 1.0);
        return;
    }
    ImQsort(items, count, sizeof(ImGuiShrinkWidthItem), ShrinkWidthItemComparer);
    int count_same_width = 1;
    while (width_excess > 0.0 && count_same_width < count)
    {
        while (count_same_width < count && items[0].width <= items[count_same_width].width)
            count_same_width += 1;
        let max_width_to_remove_per_item =  (count_same_width < count && items[count_same_width].width >= 0.0) ? (items[0].width - items[count_same_width].width) : (items[0].width - 1.0);
        if (max_width_to_remove_per_item <= 0.0)
            break;
        let width_to_remove_per_item =  ImMin(width_excess / count_same_width, max_width_to_remove_per_item);
        for (int item_n = 0; item_n < count_same_width; item_n += 1)
            items[item_n].width -= width_to_remove_per_item;
        width_excess -= width_to_remove_per_item * count_same_width;
    }

    // Round width and redistribute remainder
    // Ensure that e.g. the right-most tab of a shrunk tab-bar always reaches exactly at the same distance from the right-most edge of the tab bar separator.
    width_excess = 0.0;
    for (int n = 0; n < count; n += 1)
    {
        let width_rounded =  ImFloor(items[n].width);
        width_excess += items[n].width - width_rounded;
        items[n].width = width_rounded;
    }
    while (width_excess > 0.0)
        for (int n = 0; n < count; n += 1)
            if (items[n].width + 1.0 <= items[n].InitialWidth)
            {
                items[n].width += 1.0;
                width_excess -= 1.0;
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

static float CalcMaxPopupHeightFromItemCount(int items_count)
{
    // ImGuiContext& g = *GImGui;
    if (items_count <= 0)
        return FLT_MAX;
    return (g.FontSize + g.Style.item_spacing.y) * items_count - g.Style.item_spacing.y + (g.Style.WindowPadding.y * 2);
}

bool ImGui::BeginCombo(const char* label, const char* preview_value, ImGuiComboFlags flags)
{
    // ImGuiContext& g = *GImGui;
    Window* window = GetCurrentWindow();

    ImGuiNextWindowDataFlags backup_next_window_data_flags = g.NextWindowData.Flags;
    g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
    if (window.SkipItems)
        return false;

    const ImGuiStyle& style = g.Style;
    const Id32 id = window.GetID(label);
    IM_ASSERT((flags & (ImGuiComboFlags_NoArrowButton | ImGuiComboFlags_NoPreview)) != (ImGuiComboFlags_NoArrowButton | ImGuiComboFlags_NoPreview)); // Can't use both flags together

    let arrow_size = (flags & ImGuiComboFlags_NoArrowButton) ? 0.0 : get_frame_height();
    const Vector2D label_size = CalcTextSize(label, None, true);
    let w = (flags & ImGuiComboFlags_NoPreview) ? arrow_size : CalcItemWidth();
    const ImRect bb(window.DC.CursorPos, window.DC.CursorPos + DimgVec2D::new(w, label_size.y + style.FramePadding.y * 2.0));
    const ImRect total_bb(bb.Min, bb.Max + DimgVec2D::new(label_size.x > 0.0 ? style.ItemInnerSpacing.x + label_size.x : 0.0, 0.0));
    ItemSize(total_bb, style.FramePadding.y);
    if (!ItemAdd(total_bb, id, &bb))
        return false;

    // Open on click
    bool hovered, held;
    bool pressed = ButtonBehavior(bb, id, &hovered, &held);
    const Id32 popup_id = hash_string("##ComboPopup", 0, id);
    bool popup_open = is_popup_open(popup_id, ImGuiPopupFlags_None);
    if (pressed && !popup_open)
    {
        open_popupEx(popup_id, ImGuiPopupFlags_None);
        popup_open = true;
    }

    // Render shape
    const ImU32 frame_col = GetColorU32(hovered ? ImGuiCol_FrameBgHovered : ImGuiCol_FrameBg);
    let value_x2 = ImMax(bb.Min.x, bb.Max.x - arrow_size);
    RenderNavHighlight(bb, id);
    if (!(flags & ImGuiComboFlags_NoPreview))
        window.draw_list->AddRectFilled(bb.Min, DimgVec2D::new(value_x2, bb.Max.y), frame_col, style.frame_rounding, (flags & ImGuiComboFlags_NoArrowButton) ? ImDrawFlags_RoundCornersAll : ImDrawFlags_RoundCornersLeft);
    if (!(flags & ImGuiComboFlags_NoArrowButton))
    {
        ImU32 bg_col = GetColorU32((popup_open || hovered) ? ImGuiCol_ButtonHovered : ImGuiCol_Button);
        ImU32 text_col = GetColorU32(ImGuiCol_Text);
        window.draw_list->AddRectFilled(DimgVec2D::new(value_x2, bb.Min.y), bb.Max, bg_col, style.frame_rounding, (w <= arrow_size) ? ImDrawFlags_RoundCornersAll : ImDrawFlags_RoundCornersRight);
        if (value_x2 + arrow_size - style.FramePadding.x <= bb.Max.x)
            RenderArrow(window.draw_list, DimgVec2D::new(value_x2 + style.FramePadding.y, bb.Min.y + style.FramePadding.y), text_col, ImGuiDir_Down, 1.0);
    }
    RenderFrameBorder(bb.Min, bb.Max, style.frame_rounding);

    // Custom preview
    if (flags & ImGuiComboFlags_CustomPreview)
    {
        g.ComboPreviewData.PreviewRect = ImRect(bb.Min.x, bb.Min.y, value_x2, bb.Max.y);
        IM_ASSERT(preview_value == None || preview_value[0] == 0);
        preview_value = None;
    }

    // Render preview and label
    if (preview_value != None && !(flags & ImGuiComboFlags_NoPreview))
    {
        if (g.log_enabled)
            LogSetNextTextDecoration("{", "}");
        render_textClipped(bb.Min + style.FramePadding, DimgVec2D::new(value_x2, bb.Max.y), preview_value, None, None);
    }
    if (label_size.x > 0)
        render_text(DimgVec2D::new(bb.Max.x + style.ItemInnerSpacing.x, bb.Min.y + style.FramePadding.y), label);

    if (!popup_open)
        return false;

    g.NextWindowData.Flags = backup_next_window_data_flags;
    return BeginComboPopup(popup_id, bb, flags);
}

bool ImGui::BeginComboPopup(Id32 popup_id, const ImRect& bb, ImGuiComboFlags flags)
{
    // ImGuiContext& g = *GImGui;
    if (!is_popup_open(popup_id, ImGuiPopupFlags_None))
    {
        g.NextWindowData.ClearFlags();
        return false;
    }

    // Set popup size
    let w =  bb.GetWidth();
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSizeConstraint)
    {
        g.NextWindowData.SizeConstraintRect.Min.x = ImMax(g.NextWindowData.SizeConstraintRect.Min.x, w);
    }
    else
    {
        if ((flags & ImGuiComboFlags_HeightMask_) == 0)
            flags |= ImGuiComboFlags_HeightRegular;
        IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiComboFlags_HeightMask_)); // Only one
        int popup_max_height_in_items = -1;
        if (flags & ImGuiComboFlags_HeightRegular)     popup_max_height_in_items = 8;
        else if (flags & ImGuiComboFlags_HeightSmall)  popup_max_height_in_items = 4;
        else if (flags & ImGuiComboFlags_HeightLarge)  popup_max_height_in_items = 20;
        SetNextWindowSizeConstraints(DimgVec2D::new(w, 0.0), DimgVec2D::new(FLT_MAX, CalcMaxPopupHeightFromItemCount(popup_max_height_in_items)));
    }

    // This is essentially a specialized version of begin_popupEx()
    char name[16];
    ImFormatString(name, IM_ARRAYSIZE(name), "##Combo_%02d", g.begin_popupStack.Size); // Recycle windows based on depth

    // Set position given a custom constraint (peak into expected window size so we can position it)
    // FIXME: This might be easier to express with an hypothetical set_next_window_posConstraints() function?
    // FIXME: This might be moved to Begin() or at least around the same spot where Tooltips and other Popups are calling FindBestWindowPosForPopupEx()?
    if (Window* popup_window = find_window_by_name(name))
        if (popup_window.WasActive)
        {
            // Always override 'auto_pos_last_direction' to not leave a chance for a past value to affect us.
            Vector2D size_expected = CalcWindowNextAutoFitSize(popup_window);
            popup_window.AutoPosLastDirection = (flags & ImGuiComboFlags_PopupAlignLeft) ? ImGuiDir_Left : ImGuiDir_Down; // Left = "Below, Toward Left", down = "Below, Toward Right (default)"
            ImRect r_outer = GetPopupAllowedExtentRect(popup_window);
            Vector2D pos = FindBestWindowPosForPopupEx(bb.GetBL(), size_expected, &popup_window.AutoPosLastDirection, r_outer, bb, ImGuiPopupPositionPolicy_ComboBox);
            set_next_window_pos(pos);
        }

    // We don't use begin_popupEx() solely because we have a custom name string, which we could make an argument to begin_popupEx()
    WindowFlags window_flags = WindowFlags_AlwaysAutoResize | WindowFlags_Popup | WindowFlags_NoTitleBar | WindowFlags_NoResize | WindowFlags_NoSavedSettings | WindowFlags_NoMove;
    PushStyleVar(ImGuiStyleVar_WindowPadding, DimgVec2D::new(g.Style.FramePadding.x, g.Style.WindowPadding.y)); // Horizontally align ourselves with the framed text
    bool ret = Begin(name, None, window_flags);
    PopStyleVar();
    if (!ret)
    {
        end_popup();
        IM_ASSERT(0);   // This should never happen as we tested for IsPopupOpen() above
        return false;
    }
    return true;
}

void ImGui::EndCombo()
{
    end_popup();
}

// Call directly after the BeginCombo/EndCombo block. The preview is designed to only host non-interactive elements
// (Experimental, see GitHub issues: #1658, #4168)
bool ImGui::BeginComboPreview()
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    ImGuiComboPreviewData* preview_data = &g.ComboPreviewData;

    if (window.SkipItems || !window.ClipRect.Overlaps(g.last_item_data.Rect)) // FIXME: Because we don't have a ItemStatusFlags::Visible flag to test last ItemAdd() result
        return false;
    IM_ASSERT(g.last_item_data.Rect.Min.x == preview_data->PreviewRect.Min.x && g.last_item_data.Rect.Min.y == preview_data->PreviewRect.Min.y); // Didn't call after BeginCombo/EndCombo block or forgot to pass ImGuiComboFlags_CustomPreview flag?
    if (!window.ClipRect.contains(preview_data->PreviewRect)) // Narrower test (optional)
        return false;

    // FIXME: This could be contained in a PushWorkRect() api
    preview_data->backup_cursor_pos = window.DC.CursorPos;
    preview_data->backup_cursor_max_pos = window.DC.CursorMaxPos;
    preview_data->backup_cursor_posPrevLine = window.DC.cursor_pos_prev_line;
    preview_data->BackupPrevLineTextBaseOffset = window.DC.PrevLineTextBaseOffset;
    preview_data->BackupLayout = window.DC.layout_type;
    window.DC.CursorPos = preview_data->PreviewRect.Min + g.Style.FramePadding;
    window.DC.CursorMaxPos = window.DC.CursorPos;
    window.DC.layout_type = LayoutType::Horizontal;
    window.DC.Issame_line = false;
    push_clip_rect(preview_data->PreviewRect.Min, preview_data->PreviewRect.Max, true);

    return true;
}

void ImGui::EndComboPreview()
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    ImGuiComboPreviewData* preview_data = &g.ComboPreviewData;

    // FIXME: Using CursorMaxPos approximation instead of correct AABB which we will store in ImDrawCmd in the future
    ImDrawList* draw_list = window.draw_list;
    if (window.DC.CursorMaxPos.x < preview_data->PreviewRect.Max.x && window.DC.CursorMaxPos.y < preview_data->PreviewRect.Max.y)
        if (draw_list.cmd_buffer.Size > 1) // Unlikely case that the push_clip_rect() didn't create a command
        {
            draw_list->command_header.ClipRect = draw_list.cmd_buffer[draw_list.cmd_buffer.Size - 1].ClipRect = draw_list.cmd_buffer[draw_list.cmd_buffer.Size - 2].ClipRect;
            draw_list->_TryMergeDrawCmds();
        }
    PopClipRect();
    window.DC.CursorPos = preview_data->backup_cursor_pos;
    window.DC.CursorMaxPos = ImMax(window.DC.CursorMaxPos, preview_data->backup_cursor_max_pos);
    window.DC.cursor_pos_prev_line = preview_data->backup_cursor_posPrevLine;
    window.DC.PrevLineTextBaseOffset = preview_data->BackupPrevLineTextBaseOffset;
    window.DC.layout_type = preview_data->BackupLayout;
    window.DC.Issame_line = false;
    preview_data->PreviewRect = ImRect();
}

// Getter for the old Combo() API: const char*[]
static bool Items_ArrayGetter(void* data, int idx, const char** out_text)
{
    const char* const* items = (const char* const*)data;
    if (out_text)
        *out_text = items[idx];
    return true;
}

// Getter for the old Combo() API: "item1\0item2\0item3\0"
static bool Items_SingleStringGetter(void* data, int idx, const char** out_text)
{
    // FIXME-OPT: we could pre-compute the indices to fasten this. But only 1 active combo means the waste is limited.
    const char* items_separated_by_zeros = (const char*)data;
    int items_count = 0;
    const char* p = items_separated_by_zeros;
    while (*p)
    {
        if (idx == items_count)
            break;
        p += strlen(p) + 1;
        items_count += 1;
    }
    if (!*p)
        return false;
    if (out_text)
        *out_text = p;
    return true;
}

// Old API, prefer using BeginCombo() nowadays if you can.
bool ImGui::Combo(const char* label, int* current_item, bool (*items_getter)(void*, int, const char**), void* data, int items_count, int popup_max_height_in_items)
{
    // ImGuiContext& g = *GImGui;

    // Call the getter to obtain the preview string which is a parameter to BeginCombo()
    const char* preview_value = None;
    if (*current_item >= 0 && *current_item < items_count)
        items_getter(data, *current_item, &preview_value);

    // The old Combo() API exposed "popup_max_height_in_items". The new more general BeginCombo() API doesn't have/need it, but we emulate it here.
    if (popup_max_height_in_items != -1 && !(g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSizeConstraint))
        SetNextWindowSizeConstraints(DimgVec2D::new(0, 0), DimgVec2D::new(FLT_MAX, CalcMaxPopupHeightFromItemCount(popup_max_height_in_items)));

    if (!BeginCombo(label, preview_value, ImGuiComboFlags_None))
        return false;

    // Display items
    // FIXME-OPT: Use clipper (but we need to disable it on the appearing frame to make sure our call to SetItemDefaultFocus() is processed)
    bool value_changed = false;
    for (int i = 0; i < items_count; i += 1)
    {
        push_id(i);
        const bool item_selected = (i == *current_item);
        const char* item_text;
        if (!items_getter(data, i, &item_text))
            item_text = "*Unknown item*";
        if (selectable(item_text, item_selected))
        {
            value_changed = true;
            *current_item = i;
        }
        if (item_selected)
            SetItemDefaultFocus();
        pop_id();
    }

    EndCombo();

    if (value_changed)
        MarkItemEdited(g.last_item_data.id);

    return value_changed;
}

// Combo box helper allowing to pass an array of strings.
bool ImGui::Combo(const char* label, int* current_item, const char* const items[], int items_count, int height_in_items)
{
    const bool value_changed = Combo(label, current_item, Items_ArrayGetter, (void*)items, items_count, height_in_items);
    return value_changed;
}

// Combo box helper allowing to pass all items in a single string literal holding multiple zero-terminated items "item1\0item2\0"
bool ImGui::Combo(const char* label, int* current_item, const char* items_separated_by_zeros, int height_in_items)
{
    int items_count = 0;
    const char* p = items_separated_by_zeros;       // FIXME-OPT: Avoid computing this, or at least only when combo is open
    while (*p)
    {
        p += strlen(p) + 1;
        items_count += 1;
    }
    bool value_changed = Combo(label, current_item, Items_SingleStringGetter, (void*)items_separated_by_zeros, items_count, height_in_items);
    return value_changed;
}

//-------------------------------------------------------------------------
// [SECTION] data Type and data Formatting Helpers [Internal]
//-------------------------------------------------------------------------
// - PatchFormatStringFloatToInt()
// - DataTypeGetInfo()
// - DataTypeFormatString()
// - DataTypeApplyOp()
// - DataTypeApplyOpFromText()
// - DataTypeClamp()
// - GetMinimumStepAtDecimalPrecision
// - RoundScalarWithFormat<>()
//-------------------------------------------------------------------------

static const DataTypeInfo GDataTypeInfo[] =
{
    { sizeof(char),             "S8",   "%d",   "%d"    },  // DataType::S8
    { sizeof(unsigned char),    "U8",   "%u",   "%u"    },
    { sizeof(short),            "S16",  "%d",   "%d"    },  // DataType::S16
    { sizeof(unsigned short),   "U16",  "%u",   "%u"    },
    { sizeof,              "S32",  "%d",   "%d"    },  // DataType::S32
    { sizeof(unsigned int),     "U32",  "%u",   "%u"    },
#ifdef _MSC_VER
    { sizeof(ImS64),            "S64",  "%I64d","%I64d" },  // DataType::S64
    { sizeof,            "U64",  "%I64u","%I64u" },
#else
    { sizeof(ImS64),            "S64",  "%lld", "%lld"  },  // DataType::S64
    { sizeof,            "U64",  "%llu", "%llu"  },
#endif
    { sizeof,            "float", "%.3","%f"    },  // DataType::Float (float are promoted to double in va_arg)
    { sizeof(double),           "double","%f",  "%lf"   },  // DataType::Double
};
IM_STATIC_ASSERT(IM_ARRAYSIZE(GDataTypeInfo) == DataType::COUNT);

// FIXME-LEGACY: Prior to 1.61 our DragInt() function internally used floats and because of this the compile-time default value for format was "%.0".
// Even though we changed the compile-time default, we expect users to have carried %f around, which would break the display of DragInt() calls.
// To honor backward compatibility we are rewriting the format string, unless IMGUI_DISABLE_OBSOLETE_FUNCTIONS is enabled. What could possibly go wrong?!
static const char* PatchFormatStringFloatToInt(const char* fmt)
{
    if (fmt[0] == '%' && fmt[1] == '.' && fmt[2] == '0' && fmt[3] == 'f' && fmt[4] == 0) // Fast legacy path for "%.0" which is expected to be the most common case.
        return "%d";
    const char* fmt_start = ImParseFormatFindStart(fmt);    // Find % (if any, and ignore %%)
    const char* fmt_end = ImParseFormatFindEnd(fmt_start);  // Find end of format specifier, which itself is an exercise of confidence/recklessness (because snprintf is dependent on libc or user).
    if (fmt_end > fmt_start && fmt_end[-1] == 'f')
    {
#ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
        if (fmt_start == fmt && fmt_end[0] == 0)
            return "%d";
        const char* tmp_format;
        ImFormatStringToTempBuffer(&tmp_format, None, "%.*s%%d%s", (fmt_start - fmt), fmt, fmt_end); // Honor leading and trailing decorations, but lose alignment/precision.
        return tmp_format;
#else
        IM_ASSERT(0 && "DragInt(): Invalid format string!"); // Old versions used a default parameter of "%.0", please replace with e.g. "%d"
#endif
    }
    return fmt;
}

const DataTypeInfo* ImGui::DataTypeGetInfo(DataType data_type)
{
    IM_ASSERT(data_type >= 0 && data_type < DataType::COUNT);
    return &GDataTypeInfo[data_type];
}

int ImGui::DataTypeFormatString(char* buf, int buf_size, DataType data_type, const void* p_data, const char* format)
{
    // Signedness doesn't matter when pushing integer arguments
    if (data_type == DataType::S32 || data_type == DataType::U32)
        return ImFormatString(buf, buf_size, format, *(const ImU32*)p_data);
    if (data_type == DataType::S64 || data_type == DataType::U64)
        return ImFormatString(buf, buf_size, format, *(const ImU64*)p_data);
    if (data_type == DataType::Float)
        return ImFormatString(buf, buf_size, format, *(let*)p_data);
    if (data_type == DataType::Double)
        return ImFormatString(buf, buf_size, format, *(const double*)p_data);
    if (data_type == DataType::S8)
        return ImFormatString(buf, buf_size, format, *(const ImS8*)p_data);
    if (data_type == DataType::U8)
        return ImFormatString(buf, buf_size, format, *(const ImU8*)p_data);
    if (data_type == DataType::S16)
        return ImFormatString(buf, buf_size, format, *(const ImS16*)p_data);
    if (data_type == DataType::U16)
        return ImFormatString(buf, buf_size, format, *(const ImU16*)p_data);
    IM_ASSERT(0);
    return 0;
}

void ImGui::DataTypeApplyOp(DataType data_type, int op, void* output, const void* arg1, const void* arg2)
{
    IM_ASSERT(op == '+' || op == '-');
    switch (data_type)
    {
        case DataType::S8:
            if (op == '+') { *(ImS8*)output  = ImAddClampOverflow(*(const ImS8*)arg1,  *(const ImS8*)arg2,  IM_S8_MIN,  IM_S8_MAX); }
            if (op == '-') { *(ImS8*)output  = ImSubClampOverflow(*(const ImS8*)arg1,  *(const ImS8*)arg2,  IM_S8_MIN,  IM_S8_MAX); }
            return;
        case DataType::U8:
            if (op == '+') { *(ImU8*)output  = ImAddClampOverflow(*(const ImU8*)arg1,  *(const ImU8*)arg2,  IM_U8_MIN,  IM_U8_MAX); }
            if (op == '-') { *(ImU8*)output  = ImSubClampOverflow(*(const ImU8*)arg1,  *(const ImU8*)arg2,  IM_U8_MIN,  IM_U8_MAX); }
            return;
        case DataType::S16:
            if (op == '+') { *(ImS16*)output = ImAddClampOverflow(*(const ImS16*)arg1, *(const ImS16*)arg2, IM_S16_MIN, IM_S16_MAX); }
            if (op == '-') { *(ImS16*)output = ImSubClampOverflow(*(const ImS16*)arg1, *(const ImS16*)arg2, IM_S16_MIN, IM_S16_MAX); }
            return;
        case DataType::U16:
            if (op == '+') { *(ImU16*)output = ImAddClampOverflow(*(const ImU16*)arg1, *(const ImU16*)arg2, IM_U16_MIN, IM_U16_MAX); }
            if (op == '-') { *(ImU16*)output = ImSubClampOverflow(*(const ImU16*)arg1, *(const ImU16*)arg2, IM_U16_MIN, IM_U16_MAX); }
            return;
        case DataType::S32:
            if (op == '+') { *(ImS32*)output = ImAddClampOverflow(*(const ImS32*)arg1, *(const ImS32*)arg2, IM_S32_MIN, IM_S32_MAX); }
            if (op == '-') { *(ImS32*)output = ImSubClampOverflow(*(const ImS32*)arg1, *(const ImS32*)arg2, IM_S32_MIN, IM_S32_MAX); }
            return;
        case DataType::U32:
            if (op == '+') { *(ImU32*)output = ImAddClampOverflow(*(const ImU32*)arg1, *(const ImU32*)arg2, IM_U32_MIN, IM_U32_MAX); }
            if (op == '-') { *(ImU32*)output = ImSubClampOverflow(*(const ImU32*)arg1, *(const ImU32*)arg2, IM_U32_MIN, IM_U32_MAX); }
            return;
        case DataType::S64:
            if (op == '+') { *(ImS64*)output = ImAddClampOverflow(*(const ImS64*)arg1, *(const ImS64*)arg2, IM_S64_MIN, IM_S64_MAX); }
            if (op == '-') { *(ImS64*)output = ImSubClampOverflow(*(const ImS64*)arg1, *(const ImS64*)arg2, IM_S64_MIN, IM_S64_MAX); }
            return;
        case DataType::U64:
            if (op == '+') { *(ImU64*)output = ImAddClampOverflow(*(const ImU64*)arg1, *(const ImU64*)arg2, IM_U64_MIN, IM_U64_MAX); }
            if (op == '-') { *(ImU64*)output = ImSubClampOverflow(*(const ImU64*)arg1, *(const ImU64*)arg2, IM_U64_MIN, IM_U64_MAX); }
            return;
        case DataType::Float:
            if (op == '+') { *(float*)output = *(let*)arg1 + *(let*)arg2; }
            if (op == '-') { *(float*)output = *(let*)arg1 - *(let*)arg2; }
            return;
        case DataType::Double:
            if (op == '+') { *(double*)output = *(const double*)arg1 + *(const double*)arg2; }
            if (op == '-') { *(double*)output = *(const double*)arg1 - *(const double*)arg2; }
            return;
        case DataType::COUNT: break;
    }
    IM_ASSERT(0);
}

// User can input math operators (e.g. +100) to edit a numerical values.
// NB: This is _not_ a full expression evaluator. We should probably add one and replace this dumb mess..
bool ImGui::DataTypeApplyFromText(const char* buf, DataType data_type, void* p_data, const char* format)
{
    while (ImCharIsBlankA(*buf))
        buf += 1;
    if (!buf[0])
        return false;

    // Copy the value in an opaque buffer so we can compare at the end of the function if it changed at all.
    const DataTypeInfo* type_info = DataTypeGetInfo(data_type);
    DataTypeTempStorage data_backup;
    memcpy(&data_backup, p_data, type_info->Size);

    // Sanitize format
    // For float/double we have to ignore format with precision (e.g. "%.2") because sscanf doesn't take them in, so force them into %f and %lf
    char format_sanitized[32];
    if (data_type == DataType::Float || data_type == DataType::Double)
        format = type_info->ScanFmt;
    else
        format = ImParseFormatSanitizeForScanning(format, format_sanitized, IM_ARRAYSIZE(format_sanitized));

    // Small types need a 32-bit buffer to receive the result from scanf()
    int v32 = 0;
    if (sscanf(buf, format, type_info->Size >= 4 ? p_data : &v32) < 1)
        return false;
    if (type_info->Size < 4)
    {
        if (data_type == DataType::S8)
            *(ImS8*)p_data = (ImS8)ImClamp(v32, IM_S8_MIN, IM_S8_MAX);
        else if (data_type == DataType::U8)
            *(ImU8*)p_data = (ImU8)ImClamp(v32, IM_U8_MIN, IM_U8_MAX);
        else if (data_type == DataType::S16)
            *(ImS16*)p_data = (ImS16)ImClamp(v32, IM_S16_MIN, IM_S16_MAX);
        else if (data_type == DataType::U16)
            *(ImU16*)p_data = (ImU16)ImClamp(v32, IM_U16_MIN, IM_U16_MAX);
        else
            IM_ASSERT(0);
    }

    return memcmp(&data_backup, p_data, type_info->Size) != 0;
}

template<typename T>
static int DataTypeCompareT(const T* lhs, const T* rhs)
{
    if (*lhs < *rhs) return -1;
    if (*lhs > *rhs) return +1;
    return 0;
}

int ImGui::DataTypeCompare(DataType data_type, const void* arg_1, const void* arg_2)
{
    switch (data_type)
    {
    case DataType::S8:     return DataTypeCompareT<ImS8  >((const ImS8*  )arg_1, (const ImS8*  )arg_2);
    case DataType::U8:     return DataTypeCompareT<ImU8  >((const ImU8*  )arg_1, (const ImU8*  )arg_2);
    case DataType::S16:    return DataTypeCompareT<ImS16 >((const ImS16* )arg_1, (const ImS16* )arg_2);
    case DataType::U16:    return DataTypeCompareT<ImU16 >((const ImU16* )arg_1, (const ImU16* )arg_2);
    case DataType::S32:    return DataTypeCompareT<ImS32 >((const ImS32* )arg_1, (const ImS32* )arg_2);
    case DataType::U32:    return DataTypeCompareT<ImU32 >((const ImU32* )arg_1, (const ImU32* )arg_2);
    case DataType::S64:    return DataTypeCompareT<ImS64 >((const ImS64* )arg_1, (const ImS64* )arg_2);
    case DataType::U64:    return DataTypeCompareT<ImU64 >((const ImU64* )arg_1, (const ImU64* )arg_2);
    case DataType::Float:  return DataTypeCompareT<float >((let* )arg_1, (let* )arg_2);
    case DataType::Double: return DataTypeCompareT<double>((const double*)arg_1, (const double*)arg_2);
    case DataType::COUNT:  break;
    }
    IM_ASSERT(0);
    return 0;
}

template<typename T>
static bool DataTypeClampT(T* v, const T* v_min, const T* v_max)
{
    // Clamp, both sides are optional, return true if modified
    if (v_min && *v < *v_min) { *v = *v_min; return true; }
    if (v_max && *v > *v_max) { *v = *v_max; return true; }
    return false;
}

bool ImGui::DataTypeClamp(DataType data_type, void* p_data, const void* p_min, const void* p_max)
{
    switch (data_type)
    {
    case DataType::S8:     return DataTypeClampT<ImS8  >((ImS8*  )p_data, (const ImS8*  )p_min, (const ImS8*  )p_max);
    case DataType::U8:     return DataTypeClampT<ImU8  >((ImU8*  )p_data, (const ImU8*  )p_min, (const ImU8*  )p_max);
    case DataType::S16:    return DataTypeClampT<ImS16 >((ImS16* )p_data, (const ImS16* )p_min, (const ImS16* )p_max);
    case DataType::U16:    return DataTypeClampT<ImU16 >((ImU16* )p_data, (const ImU16* )p_min, (const ImU16* )p_max);
    case DataType::S32:    return DataTypeClampT<ImS32 >((ImS32* )p_data, (const ImS32* )p_min, (const ImS32* )p_max);
    case DataType::U32:    return DataTypeClampT<ImU32 >((ImU32* )p_data, (const ImU32* )p_min, (const ImU32* )p_max);
    case DataType::S64:    return DataTypeClampT<ImS64 >((ImS64* )p_data, (const ImS64* )p_min, (const ImS64* )p_max);
    case DataType::U64:    return DataTypeClampT<ImU64 >((ImU64* )p_data, (const ImU64* )p_min, (const ImU64* )p_max);
    case DataType::Float:  return DataTypeClampT<float >((float* )p_data, (let* )p_min, (let* )p_max);
    case DataType::Double: return DataTypeClampT<double>((double*)p_data, (const double*)p_min, (const double*)p_max);
    case DataType::COUNT:  break;
    }
    IM_ASSERT(0);
    return false;
}

static float GetMinimumStepAtDecimalPrecision(int decimal_precision)
{
    static let min_steps[10] = { 1.0, 0.1, 0.01, 0.001, 0.0001, 0.00001, 0.000001, 0.0000001, 0.00000001, 0.000000001 };
    if (decimal_precision < 0)
        return FLT_MIN;
    return (decimal_precision < IM_ARRAYSIZE(min_steps)) ? min_steps[decimal_precision] : ImPow(10.0, -decimal_precision);
}

template<typename TYPE>
TYPE ImGui::RoundScalarWithFormatT(const char* format, DataType data_type, TYPE v)
{
    IM_UNUSED(data_type);
    IM_ASSERT(data_type == DataType::Float || data_type == DataType::Double);
    const char* fmt_start = ImParseFormatFindStart(format);
    if (fmt_start[0] != '%' || fmt_start[1] == '%') // Don't apply if the value is not visible in the format string
        return v;

    // Sanitize format
    char fmt_sanitized[32];
    ImParseFormatSanitizeForPrinting(fmt_start, fmt_sanitized, IM_ARRAYSIZE(fmt_sanitized));
    fmt_start = fmt_sanitized;

    // Format value with our rounding, and read back
    char v_str[64];
    ImFormatString(v_str, IM_ARRAYSIZE(v_str), fmt_start, v);
    const char* p = v_str;
    while (*p == ' ')
        p += 1;
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
bool ImGui::DragBehaviorT(DataType data_type, TYPE* v, float v_speed, const TYPE v_min, const TYPE v_max, const char* format, ImGuiSliderFlags flags)
{
    // ImGuiContext& g = *GImGui;
    const ImGuiAxis axis = (flags & ImGuiSliderFlags_Vertical) ? ImGuiAxis_Y : ImGuiAxis_X;
    const bool is_clamped = (v_min < v_max);
    const bool is_logarithmic = (flags & ImGuiSliderFlags_Logarithmic) != 0;
    const bool is_floating_point = (data_type == DataType::Float) || (data_type == DataType::Double);

    // Default tweak speed
    if (v_speed == 0.0 && is_clamped && (v_max - v_min < FLT_MAX))
        v_speed = ((v_max - v_min) * g.DragSpeedDefaultRatio);

    // Inputs accumulates into g.drag_current_accum, which is flushed into the current value as soon as it makes a difference with our precision settings
    let adjust_delta =  0.0;
    if (g.ActiveIdSource == ImGuiInputSource_Mouse && IsMousePosValid() && IsMouseDragPastThreshold(0, g.IO.MouseDragThreshold * DRAG_MOUSE_THRESHOLD_FACTOR))
    {
        adjust_delta = g.IO.MouseDelta[axis];
        if (g.IO.KeyAlt)
            adjust_delta *= 1.0 / 100.0;
        if (g.IO.KeyShift)
            adjust_delta *= 10.0;
    }
    else if (g.ActiveIdSource == ImGuiInputSource_Nav)
    {
        let decimal_precision = is_floating_point ? ImParseFormatPrecision(format, 3) : 0;
        adjust_delta = GetNavInputAmount2d(ImGuiNavDirSourceFlags_Keyboard | ImGuiNavDirSourceFlags_PadDPad, ImGuiNavReadMode_RepeatFast, 1.0 / 10.0, 10.0)[axis];
        v_speed = ImMax(v_speed, GetMinimumStepAtDecimalPrecision(decimal_precision));
    }
    adjust_delta *= v_speed;

    // For vertical drag we currently assume that Up=higher value (like we do with vertical sliders). This may become a parameter.
    if (axis == ImGuiAxis_Y)
        adjust_delta = -adjust_delta;

    // For logarithmic use our range is effectively 0..1 so scale the delta into that range
    if (is_logarithmic && (v_max - v_min < FLT_MAX) && ((v_max - v_min) > 0.000001)) // Epsilon to avoid /0
        adjust_delta /= (v_max - v_min);

    // clear current value on activation
    // Avoid altering values and clamping when we are _already_ past the limits and heading in the same direction, so e.g. if range is 0..255, current value is 300 and we are pushing to the right side, keep the 300.
    bool is_just_activated = g.ActiveIdIsJustActivated;
    bool is_already_past_limits_and_pushing_outward = is_clamped && ((*v >= v_max && adjust_delta > 0.0) || (*v <= v_min && adjust_delta < 0.0));
    if (is_just_activated || is_already_past_limits_and_pushing_outward)
    {
        g.DragCurrentAccum = 0.0;
        g.DragCurrentAccumDirty = false;
    }
    else if (adjust_delta != 0.0)
    {
        g.DragCurrentAccum += adjust_delta;
        g.DragCurrentAccumDirty = true;
    }

    if (!g.DragCurrentAccumDirty)
        return false;

    TYPE v_cur = *v;
    FLOATTYPE v_old_ref_for_accum_remainder = (FLOATTYPE)0.0;

    let logarithmic_zero_epsilon =  0.0; // Only valid when is_logarithmic is true
    let zero_deadzone_halfsize = 0.0; // Drag widgets have no deadzone (as it doesn't make sense)
    if (is_logarithmic)
    {
        // When using logarithmic sliders, we need to clamp to avoid hitting zero, but our choice of clamp value greatly affects slider precision. We attempt to use the specified precision to estimate a good lower bound.
        let decimal_precision = is_floating_point ? ImParseFormatPrecision(format, 3) : 1;
        logarithmic_zero_epsilon = ImPow(0.1, decimal_precision);

        // Convert to parametric space, apply delta, convert back
        let v_old_parametric =  ScaleRatioFromValueT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, v_cur, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);
        let v_new_parametric =  v_old_parametric + g.DragCurrentAccum;
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
        let v_new_parametric =  ScaleRatioFromValueT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, v_cur, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);
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
        if (v_cur < v_min || (v_cur > *v && adjust_delta < 0.0 && !is_floating_point))
            v_cur = v_min;
        if (v_cur > v_max || (v_cur < *v && adjust_delta > 0.0 && !is_floating_point))
            v_cur = v_max;
    }

    // Apply result
    if (*v == v_cur)
        return false;
    *v = v_cur;
    return true;
}

bool ImGui::DragBehavior(Id32 id, DataType data_type, void* p_v, float v_speed, const void* p_min, const void* p_max, const char* format, ImGuiSliderFlags flags)
{
    // Read imgui.cpp "API BREAKING CHANGES" section for 1.78 if you hit this assert.
    IM_ASSERT((flags == 1 || (flags & ImGuiSliderFlags_InvalidMask_) == 0) && "Invalid ImGuiSliderFlags flags! Has the 'float power' argument been mistakenly cast to flags? Call function with ImGuiSliderFlags_Logarithmic flags instead.");

    // ImGuiContext& g = *GImGui;
    if (g.ActiveId == id)
    {
        if (g.ActiveIdSource == ImGuiInputSource_Mouse && !g.IO.MouseDown[0])
            ClearActiveID();
        else if (g.ActiveIdSource == ImGuiInputSource_Nav && g.NavActivatePressedId == id && !g.ActiveIdIsJustActivated)
            ClearActiveID();
    }
    if (g.ActiveId != id)
        return false;
    if ((g.last_item_data.in_flags & ItemFlags::ReadOnly) || (flags & ImGuiSliderFlags_ReadOnly))
        return false;

    switch (data_type)
    {
    case DataType::S8:     { ImS32 v32 = (ImS32)*(ImS8*)p_v;  bool r = DragBehaviorT<ImS32, ImS32, float>(DataType::S32, &v32, v_speed, p_min ? *(const ImS8*) p_min : IM_S8_MIN,  p_max ? *(const ImS8*)p_max  : IM_S8_MAX,  format, flags); if (r) *(ImS8*)p_v = (ImS8)v32; return r; }
    case DataType::U8:     { ImU32 v32 = *(ImU8*)p_v;  bool r = DragBehaviorT<ImU32, ImS32, float>(DataType::U32, &v32, v_speed, p_min ? *(const ImU8*) p_min : IM_U8_MIN,  p_max ? *(const ImU8*)p_max  : IM_U8_MAX,  format, flags); if (r) *(ImU8*)p_v = (ImU8)v32; return r; }
    case DataType::S16:    { ImS32 v32 = (ImS32)*(ImS16*)p_v; bool r = DragBehaviorT<ImS32, ImS32, float>(DataType::S32, &v32, v_speed, p_min ? *(const ImS16*)p_min : IM_S16_MIN, p_max ? *(const ImS16*)p_max : IM_S16_MAX, format, flags); if (r) *(ImS16*)p_v = (ImS16)v32; return r; }
    case DataType::U16:    { ImU32 v32 = *(ImU16*)p_v; bool r = DragBehaviorT<ImU32, ImS32, float>(DataType::U32, &v32, v_speed, p_min ? *(const ImU16*)p_min : IM_U16_MIN, p_max ? *(const ImU16*)p_max : IM_U16_MAX, format, flags); if (r) *(ImU16*)p_v = (ImU16)v32; return r; }
    case DataType::S32:    return DragBehaviorT<ImS32, ImS32, float >(data_type, (ImS32*)p_v,  v_speed, p_min ? *(const ImS32* )p_min : IM_S32_MIN, p_max ? *(const ImS32* )p_max : IM_S32_MAX, format, flags);
    case DataType::U32:    return DragBehaviorT<ImU32, ImS32, float >(data_type, (ImU32*)p_v,  v_speed, p_min ? *(const ImU32* )p_min : IM_U32_MIN, p_max ? *(const ImU32* )p_max : IM_U32_MAX, format, flags);
    case DataType::S64:    return DragBehaviorT<ImS64, ImS64, double>(data_type, (ImS64*)p_v,  v_speed, p_min ? *(const ImS64* )p_min : IM_S64_MIN, p_max ? *(const ImS64* )p_max : IM_S64_MAX, format, flags);
    case DataType::U64:    return DragBehaviorT<ImU64, ImS64, double>(data_type, (ImU64*)p_v,  v_speed, p_min ? *(const ImU64* )p_min : IM_U64_MIN, p_max ? *(const ImU64* )p_max : IM_U64_MAX, format, flags);
    case DataType::Float:  return DragBehaviorT<float, float, float >(data_type, (float*)p_v,  v_speed, p_min ? *(let* )p_min : -FLT_MAX,   p_max ? *(let* )p_max : FLT_MAX,    format, flags);
    case DataType::Double: return DragBehaviorT<double,double,double>(data_type, (double*)p_v, v_speed, p_min ? *(const double*)p_min : -DBL_MAX,   p_max ? *(const double*)p_max : DBL_MAX,    format, flags);
    case DataType::COUNT:  break;
    }
    IM_ASSERT(0);
    return false;
}

// Note: p_data, p_min and p_max are _pointers_ to a memory address holding the data. For a Drag widget, p_min and p_max are optional.
// Read code of e.g. DragFloat(), DragInt() etc. or examples in 'Demo->Widgets->data Types' to understand how to use this function directly.
bool ImGui::DragScalar(const char* label, DataType data_type, void* p_data, float v_speed, const void* p_min, const void* p_max, const char* format, ImGuiSliderFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;
    const Id32 id = window.GetID(label);
    let w = CalcItemWidth();

    const Vector2D label_size = CalcTextSize(label, None, true);
    const ImRect frame_bb(window.DC.CursorPos, window.DC.CursorPos + DimgVec2D::new(w, label_size.y + style.FramePadding.y * 2.0));
    const ImRect total_bb(frame_bb.Min, frame_bb.Max + DimgVec2D::new(label_size.x > 0.0 ? style.ItemInnerSpacing.x + label_size.x : 0.0, 0.0));

    const bool temp_input_allowed = (flags & ImGuiSliderFlags_NoInput) == 0;
    ItemSize(total_bb, style.FramePadding.y);
    if (!ItemAdd(total_bb, id, &frame_bb, temp_input_allowed ? ItemFlags::Inputable : 0))
        return false;

    // Default format string when passing None
    if (format == None)
        format = DataTypeGetInfo(data_type)->PrintFmt;
    else if (data_type == DataType::S32 && strcmp(format, "%d") != 0) // (FIXME-LEGACY: Patch old "%.0" format string to use "%d", read function more details.)
        format = PatchFormatStringFloatToInt(format);

    const bool hovered = ItemHoverable(frame_bb, id);
    bool temp_input_is_active = temp_input_allowed && TempInputIsActive(id);
    if (!temp_input_is_active)
    {
        // Tabbing or CTRL-clicking on Drag turns it into an InputText
        const bool input_requested_by_tabbing = temp_input_allowed && (g.last_item_data.StatusFlags & ItemStatusFlags::FocusedByTabbing) != 0;
        const bool clicked = (hovered && g.IO.MouseClicked[0]);
        const bool double_clicked = (hovered && g.IO.MouseClickedCount[0] == 2);
        const bool make_active = (input_requested_by_tabbing || clicked || double_clicked || g.NavActivateId == id || g.NavActivateInputId == id);
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
        const bool is_clamp_input = (flags & ImGuiSliderFlags_AlwaysClamp) != 0 && (p_min == None || p_max == None || DataTypeCompare(data_type, p_min, p_max) < 0);
        return TempInputScalar(frame_bb, id, label, data_type, p_data, format, is_clamp_input ? p_min : None, is_clamp_input ? p_max : None);
    }

    // Draw frame
    const ImU32 frame_col = GetColorU32(g.ActiveId == id ? ImGuiCol_FrameBgActive : hovered ? ImGuiCol_FrameBgHovered : ImGuiCol_FrameBg);
    RenderNavHighlight(frame_bb, id);
    RenderFrame(frame_bb.Min, frame_bb.Max, frame_col, true, style.frame_rounding);

    // Drag behavior
    const bool value_changed = DragBehavior(id, data_type, p_data, v_speed, p_min, p_max, format, flags);
    if (value_changed)
        MarkItemEdited(id);

    // Display value using user-provided display format so user can add prefix/suffix/decorations to the value.
    char value_buf[64];
    const char* value_buf_end = value_buf + DataTypeFormatString(value_buf, IM_ARRAYSIZE(value_buf), data_type, p_data, format);
    if (g.log_enabled)
        LogSetNextTextDecoration("{", "}");
    render_textClipped(frame_bb.Min, frame_bb.Max, value_buf, value_buf_end, None, DimgVec2D::new(0.5, 0.5));

    if (label_size.x > 0.0)
        render_text(DimgVec2D::new(frame_bb.Max.x + style.ItemInnerSpacing.x, frame_bb.Min.y + style.FramePadding.y), label);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.last_item_data.StatusFlags);
    return value_changed;
}

bool ImGui::DragScalarN(const char* label, DataType data_type, void* p_data, int components, float v_speed, const void* p_min, const void* p_max, const char* format, ImGuiSliderFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    bool value_changed = false;
    BeginGroup();
    push_id(label);
    PushMultiItemsWidths(components, CalcItemWidth());
    size_t type_size = GDataTypeInfo[data_type].Size;
    for (int i = 0; i < components; i += 1)
    {
        push_id(i);
        if (i > 0)
            same_line(0, g.Style.ItemInnerSpacing.x);
        value_changed |= DragScalar("", data_type, p_data, v_speed, p_min, p_max, format, flags);
        pop_id();
        PopItemWidth();
        p_data = (void*)((char*)p_data + type_size);
    }
    pop_id();

    const char* label_end = FindRenderedTextEnd(label);
    if (label != label_end)
    {
        same_line(0, g.Style.ItemInnerSpacing.x);
        TextEx(label, label_end);
    }

    EndGroup();
    return value_changed;
}

bool ImGui::DragFloat(const char* label, float* v, float v_speed, float v_min, float v_max, const char* format, ImGuiSliderFlags flags)
{
    return DragScalar(label, DataType::Float, v, v_speed, &v_min, &v_max, format, flags);
}

bool ImGui::DragFloat2(const char* label, float v[2], float v_speed, float v_min, float v_max, const char* format, ImGuiSliderFlags flags)
{
    return DragScalarN(label, DataType::Float, v, 2, v_speed, &v_min, &v_max, format, flags);
}

bool ImGui::DragFloat3(const char* label, float v[3], float v_speed, float v_min, float v_max, const char* format, ImGuiSliderFlags flags)
{
    return DragScalarN(label, DataType::Float, v, 3, v_speed, &v_min, &v_max, format, flags);
}

bool ImGui::DragFloat4(const char* label, float v[4], float v_speed, float v_min, float v_max, const char* format, ImGuiSliderFlags flags)
{
    return DragScalarN(label, DataType::Float, v, 4, v_speed, &v_min, &v_max, format, flags);
}

// NB: You likely want to specify the ImGuiSliderFlags_AlwaysClamp when using this.
bool ImGui::DragFloatRange2(const char* label, float* v_current_min, float* v_current_max, float v_speed, float v_min, float v_max, const char* format, const char* format_max, ImGuiSliderFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    push_id(label);
    BeginGroup();
    PushMultiItemsWidths(2, CalcItemWidth());

    let min_min =  (v_min >= v_max) ? -FLT_MAX : v_min;
    let min_max =  (v_min >= v_max) ? *v_current_max : ImMin(v_max, *v_current_max);
    ImGuiSliderFlags min_flags = flags | ((min_min == min_max) ? ImGuiSliderFlags_ReadOnly : 0);
    bool value_changed = DragScalar("##min", DataType::Float, v_current_min, v_speed, &min_min, &min_max, format, min_flags);
    PopItemWidth();
    same_line(0, g.Style.ItemInnerSpacing.x);

    let max_min =  (v_min >= v_max) ? *v_current_min : ImMax(v_min, *v_current_min);
    let max_max =  (v_min >= v_max) ? FLT_MAX : v_max;
    ImGuiSliderFlags max_flags = flags | ((max_min == max_max) ? ImGuiSliderFlags_ReadOnly : 0);
    value_changed |= DragScalar("##max", DataType::Float, v_current_max, v_speed, &max_min, &max_max, format_max ? format_max : format, max_flags);
    PopItemWidth();
    same_line(0, g.Style.ItemInnerSpacing.x);

    TextEx(label, FindRenderedTextEnd(label));
    EndGroup();
    pop_id();

    return value_changed;
}

// NB: v_speed is float to allow adjusting the drag speed with more precision
bool ImGui::DragInt(const char* label, int* v, float v_speed, int v_min, int v_max, const char* format, ImGuiSliderFlags flags)
{
    return DragScalar(label, DataType::S32, v, v_speed, &v_min, &v_max, format, flags);
}

bool ImGui::DragInt2(const char* label, int v[2], float v_speed, int v_min, int v_max, const char* format, ImGuiSliderFlags flags)
{
    return DragScalarN(label, DataType::S32, v, 2, v_speed, &v_min, &v_max, format, flags);
}

bool ImGui::DragInt3(const char* label, int v[3], float v_speed, int v_min, int v_max, const char* format, ImGuiSliderFlags flags)
{
    return DragScalarN(label, DataType::S32, v, 3, v_speed, &v_min, &v_max, format, flags);
}

bool ImGui::DragInt4(const char* label, int v[4], float v_speed, int v_min, int v_max, const char* format, ImGuiSliderFlags flags)
{
    return DragScalarN(label, DataType::S32, v, 4, v_speed, &v_min, &v_max, format, flags);
}

// NB: You likely want to specify the ImGuiSliderFlags_AlwaysClamp when using this.
bool ImGui::DragIntRange2(const char* label, int* v_current_min, int* v_current_max, float v_speed, int v_min, int v_max, const char* format, const char* format_max, ImGuiSliderFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    push_id(label);
    BeginGroup();
    PushMultiItemsWidths(2, CalcItemWidth());

    int min_min = (v_min >= v_max) ? INT_MIN : v_min;
    int min_max = (v_min >= v_max) ? *v_current_max : ImMin(v_max, *v_current_max);
    ImGuiSliderFlags min_flags = flags | ((min_min == min_max) ? ImGuiSliderFlags_ReadOnly : 0);
    bool value_changed = DragInt("##min", v_current_min, v_speed, min_min, min_max, format, min_flags);
    PopItemWidth();
    same_line(0, g.Style.ItemInnerSpacing.x);

    int max_min = (v_min >= v_max) ? *v_current_min : ImMax(v_min, *v_current_min);
    int max_max = (v_min >= v_max) ? INT_MAX : v_max;
    ImGuiSliderFlags max_flags = flags | ((max_min == max_max) ? ImGuiSliderFlags_ReadOnly : 0);
    value_changed |= DragInt("##max", v_current_max, v_speed, max_min, max_max, format_max ? format_max : format, max_flags);
    PopItemWidth();
    same_line(0, g.Style.ItemInnerSpacing.x);

    TextEx(label, FindRenderedTextEnd(label));
    EndGroup();
    pop_id();

    return value_changed;
}

#ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS

// Obsolete versions with power parameter. See https://github.com/ocornut/imgui/issues/3361 for details.
bool ImGui::DragScalar(const char* label, DataType data_type, void* p_data, float v_speed, const void* p_min, const void* p_max, const char* format, float power)
{
    ImGuiSliderFlags drag_flags = ImGuiSliderFlags_None;
    if (power != 1.0)
    {
        IM_ASSERT(power == 1.0 && "Call function with ImGuiSliderFlags_Logarithmic flags instead of using the old 'float power' function!");
        IM_ASSERT(p_min != None && p_max != None);  // When using a power curve the drag needs to have known bounds
        drag_flags |= ImGuiSliderFlags_Logarithmic;   // Fallback for non-asserting paths
    }
    return DragScalar(label, data_type, p_data, v_speed, p_min, p_max, format, drag_flags);
}

bool ImGui::DragScalarN(const char* label, DataType data_type, void* p_data, int components, float v_speed, const void* p_min, const void* p_max, const char* format, float power)
{
    ImGuiSliderFlags drag_flags = ImGuiSliderFlags_None;
    if (power != 1.0)
    {
        IM_ASSERT(power == 1.0 && "Call function with ImGuiSliderFlags_Logarithmic flags instead of using the old 'float power' function!");
        IM_ASSERT(p_min != None && p_max != None);  // When using a power curve the drag needs to have known bounds
        drag_flags |= ImGuiSliderFlags_Logarithmic;   // Fallback for non-asserting paths
    }
    return DragScalarN(label, data_type, p_data, components, v_speed, p_min, p_max, format, drag_flags);
}

#endif // IMGUI_DISABLE_OBSOLETE_FUNCTIONS

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
float ImGui::ScaleRatioFromValueT(DataType data_type, TYPE v, TYPE v_min, TYPE v_max, bool is_logarithmic, float logarithmic_zero_epsilon, float zero_deadzone_halfsize)
{
    if (v_min == v_max)
        return 0.0;
    IM_UNUSED(data_type);

    const TYPE v_clamped = (v_min < v_max) ? ImClamp(v, v_min, v_max) : ImClamp(v, v_max, v_min);
    if (is_logarithmic)
    {
        bool flipped = v_max < v_min;

        if (flipped) // Handle the case where the range is backwards
            ImSwap(v_min, v_max);

        // Fudge min/max to avoid getting close to log(0)
        FLOATTYPE v_min_fudged = (ImAbs((FLOATTYPE)v_min) < logarithmic_zero_epsilon) ? ((v_min < 0.0) ? -logarithmic_zero_epsilon : logarithmic_zero_epsilon) : (FLOATTYPE)v_min;
        FLOATTYPE v_max_fudged = (ImAbs((FLOATTYPE)v_max) < logarithmic_zero_epsilon) ? ((v_max < 0.0) ? -logarithmic_zero_epsilon : logarithmic_zero_epsilon) : (FLOATTYPE)v_max;

        // Awkward special cases - we need ranges of the form (-100 .. 0) to convert to (-100 .. -epsilon), not (-100 .. epsilon)
        if ((v_min == 0.0) && (v_max < 0.0))
            v_min_fudged = -logarithmic_zero_epsilon;
        else if ((v_max == 0.0) && (v_min < 0.0))
            v_max_fudged = -logarithmic_zero_epsilon;

        float result;
        if (v_clamped <= v_min_fudged)
            result = 0.0; // Workaround for values that are in-range but below our fudge
        else if (v_clamped >= v_max_fudged)
            result = 1.0; // Workaround for values that are in-range but above our fudge
        else if ((v_min * v_max) < 0.0) // Range crosses zero, so split into two portions
        {
            let zero_point_center =  (-v_min) / (v_max - v_min); // The zero point in parametric space.  There's an argument we should take the logarithmic nature into account when calculating this, but for now this should do (and the most common case of a symmetrical range works fine)
            let zero_point_snap_L =  zero_point_center - zero_deadzone_halfsize;
            let zero_point_snap_R =  zero_point_center + zero_deadzone_halfsize;
            if (v == 0.0)
                result = zero_point_center; // Special case for exactly zero
            else if (v < 0.0)
                result = (1.0 - (ImLog(-(FLOATTYPE)v_clamped / logarithmic_zero_epsilon) / ImLog(-v_min_fudged / logarithmic_zero_epsilon))) * zero_point_snap_L;
            else
                result = zero_point_snap_R + ((ImLog((FLOATTYPE)v_clamped / logarithmic_zero_epsilon) / ImLog(v_max_fudged / logarithmic_zero_epsilon)) * (1.0 - zero_point_snap_R));
        }
        else if ((v_min < 0.0) || (v_max < 0.0)) // Entirely negative slider
            result = 1.0 - (ImLog(-(FLOATTYPE)v_clamped / -v_max_fudged) / ImLog(-v_min_fudged / -v_max_fudged));
        else
            result = (ImLog((FLOATTYPE)v_clamped / v_min_fudged) / ImLog(v_max_fudged / v_min_fudged));

        return flipped ? (1.0 - result) : result;
    }
    else
    {
        // Linear slider
        return ((FLOATTYPE)(SIGNEDTYPE)(v_clamped - v_min) / (FLOATTYPE)(SIGNEDTYPE)(v_max - v_min));
    }
}

// Convert a parametric position on a slider into a value v in the output space (the logical opposite of ScaleRatioFromValueT)
template<typename TYPE, typename SIGNEDTYPE, typename FLOATTYPE>
TYPE ImGui::ScaleValueFromRatioT(DataType data_type, float t, TYPE v_min, TYPE v_max, bool is_logarithmic, float logarithmic_zero_epsilon, float zero_deadzone_halfsize)
{
    // We special-case the extents because otherwise our logarithmic fudging can lead to "mathematically correct"
    // but non-intuitive behaviors like a fully-left slider not actually reaching the minimum value. Also generally simpler.
    if (t <= 0.0 || v_min == v_max)
        return v_min;
    if (t >= 1.0)
        return v_max;

    TYPE result = (TYPE)0;
    if (is_logarithmic)
    {
        // Fudge min/max to avoid getting silly results close to zero
        FLOATTYPE v_min_fudged = (ImAbs((FLOATTYPE)v_min) < logarithmic_zero_epsilon) ? ((v_min < 0.0) ? -logarithmic_zero_epsilon : logarithmic_zero_epsilon) : (FLOATTYPE)v_min;
        FLOATTYPE v_max_fudged = (ImAbs((FLOATTYPE)v_max) < logarithmic_zero_epsilon) ? ((v_max < 0.0) ? -logarithmic_zero_epsilon : logarithmic_zero_epsilon) : (FLOATTYPE)v_max;

        const bool flipped = v_max < v_min; // Check if range is "backwards"
        if (flipped)
            ImSwap(v_min_fudged, v_max_fudged);

        // Awkward special case - we need ranges of the form (-100 .. 0) to convert to (-100 .. -epsilon), not (-100 .. epsilon)
        if ((v_max == 0.0) && (v_min < 0.0))
            v_max_fudged = -logarithmic_zero_epsilon;

        let t_with_flip =  flipped ? (1.0 - t) : t; // t, but flipped if necessary to account for us flipping the range

        if ((v_min * v_max) < 0.0) // Range crosses zero, so we have to do this in two parts
        {
            let zero_point_center =  (-ImMin(v_min, v_max)) / ImAbs(v_max - v_min); // The zero point in parametric space
            let zero_point_snap_L =  zero_point_center - zero_deadzone_halfsize;
            let zero_point_snap_R =  zero_point_center + zero_deadzone_halfsize;
            if (t_with_flip >= zero_point_snap_L && t_with_flip <= zero_point_snap_R)
                result = (TYPE)0.0; // Special case to make getting exactly zero possible (the epsilon prevents it otherwise)
            else if (t_with_flip < zero_point_center)
                result = (TYPE)-(logarithmic_zero_epsilon * ImPow(-v_min_fudged / logarithmic_zero_epsilon, (FLOATTYPE)(1.0 - (t_with_flip / zero_point_snap_L))));
            else
                result = (TYPE)(logarithmic_zero_epsilon * ImPow(v_max_fudged / logarithmic_zero_epsilon, (FLOATTYPE)((t_with_flip - zero_point_snap_R) / (1.0 - zero_point_snap_R))));
        }
        else if ((v_min < 0.0) || (v_max < 0.0)) // Entirely negative slider
            result = (TYPE)-(-v_max_fudged * ImPow(-v_min_fudged / -v_max_fudged, (FLOATTYPE)(1.0 - t_with_flip)));
        else
            result = (TYPE)(v_min_fudged * ImPow(v_max_fudged / v_min_fudged, (FLOATTYPE)t_with_flip));
    }
    else
    {
        // Linear slider
        const bool is_floating_point = (data_type == DataType::Float) || (data_type == DataType::Double);
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
bool ImGui::SliderBehaviorT(const ImRect& bb, Id32 id, DataType data_type, TYPE* v, const TYPE v_min, const TYPE v_max, const char* format, ImGuiSliderFlags flags, ImRect* out_grab_bb)
{
    // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;

    const ImGuiAxis axis = (flags & ImGuiSliderFlags_Vertical) ? ImGuiAxis_Y : ImGuiAxis_X;
    const bool is_logarithmic = (flags & ImGuiSliderFlags_Logarithmic) != 0;
    const bool is_floating_point = (data_type == DataType::Float) || (data_type == DataType::Double);
    const SIGNEDTYPE v_range = (v_min < v_max ? v_max - v_min : v_min - v_max);

    // Calculate bounds
    let grab_padding = 2.0; // FIXME: Should be part of style.
    let slider_sz = (bb.Max[axis] - bb.Min[axis]) - grab_padding * 2.0;
    let grab_sz =  style.GrabMinSize;
    if (!is_floating_point && v_range >= 0)                                     // v_range < 0 may happen on integer overflows
        grab_sz = ImMax((slider_sz / (v_range + 1)), style.GrabMinSize); // For integer sliders: if possible have the grab size represent 1 unit
    grab_sz = ImMin(grab_sz, slider_sz);
    let slider_usable_sz = slider_sz - grab_sz;
    let slider_usable_pos_min = bb.Min[axis] + grab_padding + grab_sz * 0.5;
    let slider_usable_pos_max = bb.Max[axis] - grab_padding - grab_sz * 0.5;

    let logarithmic_zero_epsilon =  0.0; // Only valid when is_logarithmic is true
    let zero_deadzone_halfsize =  0.0; // Only valid when is_logarithmic is true
    if (is_logarithmic)
    {
        // When using logarithmic sliders, we need to clamp to avoid hitting zero, but our choice of clamp value greatly affects slider precision. We attempt to use the specified precision to estimate a good lower bound.
        let decimal_precision = is_floating_point ? ImParseFormatPrecision(format, 3) : 1;
        logarithmic_zero_epsilon = ImPow(0.1, decimal_precision);
        zero_deadzone_halfsize = (style.LogSliderDeadzone * 0.5) / ImMax(slider_usable_sz, 1.0);
    }

    // Process interacting with the slider
    bool value_changed = false;
    if (g.ActiveId == id)
    {
        bool set_new_value = false;
        let clicked_t =  0.0;
        if (g.ActiveIdSource == ImGuiInputSource_Mouse)
        {
            if (!g.IO.MouseDown[0])
            {
                ClearActiveID();
            }
            else
            {
                let mouse_abs_pos = g.IO.MousePos[axis];
                if (g.ActiveIdIsJustActivated)
                {
                    let grab_t =  ScaleRatioFromValueT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, *v, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);
                    if (axis == ImGuiAxis_Y)
                        grab_t = 1.0 - grab_t;
                    let grab_pos = ImLerp(slider_usable_pos_min, slider_usable_pos_max, grab_t);
                    const bool clicked_around_grab = (mouse_abs_pos >= grab_pos - grab_sz * 0.5 - 1.0) && (mouse_abs_pos <= grab_pos + grab_sz * 0.5 + 1.0); // No harm being extra generous here.
                    g.SliderGrabClickOffset = (clicked_around_grab && is_floating_point) ? mouse_abs_pos - grab_pos : 0.0;
                }
                if (slider_usable_sz > 0.0)
                    clicked_t = ImSaturate((mouse_abs_pos - g.SliderGrabClickOffset - slider_usable_pos_min) / slider_usable_sz);
                if (axis == ImGuiAxis_Y)
                    clicked_t = 1.0 - clicked_t;
                set_new_value = true;
            }
        }
        else if (g.ActiveIdSource == ImGuiInputSource_Nav)
        {
            if (g.ActiveIdIsJustActivated)
            {
                g.SliderCurrentAccum = 0.0; // Reset any stored nav delta upon activation
                g.SliderCurrentAccumDirty = false;
            }

            const Vector2D input_delta2 = GetNavInputAmount2d(ImGuiNavDirSourceFlags_Keyboard | ImGuiNavDirSourceFlags_PadDPad, ImGuiNavReadMode_RepeatFast, 0.0, 0.0);
            let input_delta =  (axis == ImGuiAxis_X) ? input_delta2.x : -input_delta2.y;
            if (input_delta != 0.0)
            {
                let decimal_precision = is_floating_point ? ImParseFormatPrecision(format, 3) : 0;
                if (decimal_precision > 0)
                {
                    input_delta /= 100.0;    // Gamepad/keyboard tweak speeds in % of slider bounds
                    if (IsNavInputDown(ImGuiNavInput_TweakSlow))
                        input_delta /= 10.0;
                }
                else
                {
                    if ((v_range >= -100.0 && v_range <= 100.0) || IsNavInputDown(ImGuiNavInput_TweakSlow))
                        input_delta = ((input_delta < 0.0) ? -1.0 : +1.0) / v_range; // Gamepad/keyboard tweak speeds in integer steps
                    else
                        input_delta /= 100.0;
                }
                if (IsNavInputDown(ImGuiNavInput_TweakFast))
                    input_delta *= 10.0;

                g.SliderCurrentAccum += input_delta;
                g.SliderCurrentAccumDirty = true;
            }

            let delta =  g.SliderCurrentAccum;
            if (g.NavActivatePressedId == id && !g.ActiveIdIsJustActivated)
            {
                ClearActiveID();
            }
            else if (g.SliderCurrentAccumDirty)
            {
                clicked_t = ScaleRatioFromValueT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, *v, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);

                if ((clicked_t >= 1.0 && delta > 0.0) || (clicked_t <= 0.0 && delta < 0.0)) // This is to avoid applying the saturation when already past the limits
                {
                    set_new_value = false;
                    g.SliderCurrentAccum = 0.0; // If pushing up against the limits, don't continue to accumulate
                }
                else
                {
                    set_new_value = true;
                    let old_clicked_t =  clicked_t;
                    clicked_t = ImSaturate(clicked_t + delta);

                    // Calculate what our "new" clicked_t will be, and thus how far we actually moved the slider, and subtract this from the accumulator
                    TYPE v_new = ScaleValueFromRatioT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, clicked_t, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);
                    if (is_floating_point && !(flags & ImGuiSliderFlags_NoRoundToFormat))
                        v_new = RoundScalarWithFormatT<TYPE>(format, data_type, v_new);
                    let new_clicked_t =  ScaleRatioFromValueT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, v_new, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);

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

    if (slider_sz < 1.0)
    {
        *out_grab_bb = ImRect(bb.Min, bb.Min);
    }
    else
    {
        // Output grab position so it can be displayed by the caller
        let grab_t =  ScaleRatioFromValueT<TYPE, SIGNEDTYPE, FLOATTYPE>(data_type, *v, v_min, v_max, is_logarithmic, logarithmic_zero_epsilon, zero_deadzone_halfsize);
        if (axis == ImGuiAxis_Y)
            grab_t = 1.0 - grab_t;
        let grab_pos = ImLerp(slider_usable_pos_min, slider_usable_pos_max, grab_t);
        if (axis == ImGuiAxis_X)
            *out_grab_bb = ImRect(grab_pos - grab_sz * 0.5, bb.Min.y + grab_padding, grab_pos + grab_sz * 0.5, bb.Max.y - grab_padding);
        else
            *out_grab_bb = ImRect(bb.Min.x + grab_padding, grab_pos - grab_sz * 0.5, bb.Max.x - grab_padding, grab_pos + grab_sz * 0.5);
    }

    return value_changed;
}

// For 32-bit and larger types, slider bounds are limited to half the natural type range.
// So e.g. an integer Slider between INT_MAX-10 and INT_MAX will fail, but an integer Slider between INT_MAX/2-10 and INT_MAX/2 will be ok.
// It would be possible to lift that limitation with some work but it doesn't seem to be worth it for sliders.
bool ImGui::SliderBehavior(const ImRect& bb, Id32 id, DataType data_type, void* p_v, const void* p_min, const void* p_max, const char* format, ImGuiSliderFlags flags, ImRect* out_grab_bb)
{
    // Read imgui.cpp "API BREAKING CHANGES" section for 1.78 if you hit this assert.
    IM_ASSERT((flags == 1 || (flags & ImGuiSliderFlags_InvalidMask_) == 0) && "Invalid ImGuiSliderFlags flag!  Has the 'float power' argument been mistakenly cast to flags? Call function with ImGuiSliderFlags_Logarithmic flags instead.");

    // ImGuiContext& g = *GImGui;
    if ((g.last_item_data.in_flags & ItemFlags::ReadOnly) || (flags & ImGuiSliderFlags_ReadOnly))
        return false;

    switch (data_type)
    {
    case DataType::S8:  { ImS32 v32 = (ImS32)*(ImS8*)p_v;  bool r = SliderBehaviorT<ImS32, ImS32, float>(bb, id, DataType::S32, &v32, *(const ImS8*)p_min,  *(const ImS8*)p_max,  format, flags, out_grab_bb); if (r) *(ImS8*)p_v  = (ImS8)v32;  return r; }
    case DataType::U8:  { ImU32 v32 = *(ImU8*)p_v;  bool r = SliderBehaviorT<ImU32, ImS32, float>(bb, id, DataType::U32, &v32, *(const ImU8*)p_min,  *(const ImU8*)p_max,  format, flags, out_grab_bb); if (r) *(ImU8*)p_v  = (ImU8)v32;  return r; }
    case DataType::S16: { ImS32 v32 = (ImS32)*(ImS16*)p_v; bool r = SliderBehaviorT<ImS32, ImS32, float>(bb, id, DataType::S32, &v32, *(const ImS16*)p_min, *(const ImS16*)p_max, format, flags, out_grab_bb); if (r) *(ImS16*)p_v = (ImS16)v32; return r; }
    case DataType::U16: { ImU32 v32 = *(ImU16*)p_v; bool r = SliderBehaviorT<ImU32, ImS32, float>(bb, id, DataType::U32, &v32, *(const ImU16*)p_min, *(const ImU16*)p_max, format, flags, out_grab_bb); if (r) *(ImU16*)p_v = (ImU16)v32; return r; }
    case DataType::S32:
        IM_ASSERT(*(const ImS32*)p_min >= IM_S32_MIN / 2 && *(const ImS32*)p_max <= IM_S32_MAX / 2);
        return SliderBehaviorT<ImS32, ImS32, float >(bb, id, data_type, (ImS32*)p_v,  *(const ImS32*)p_min,  *(const ImS32*)p_max,  format, flags, out_grab_bb);
    case DataType::U32:
        IM_ASSERT(*(const ImU32*)p_max <= IM_U32_MAX / 2);
        return SliderBehaviorT<ImU32, ImS32, float >(bb, id, data_type, (ImU32*)p_v,  *(const ImU32*)p_min,  *(const ImU32*)p_max,  format, flags, out_grab_bb);
    case DataType::S64:
        IM_ASSERT(*(const ImS64*)p_min >= IM_S64_MIN / 2 && *(const ImS64*)p_max <= IM_S64_MAX / 2);
        return SliderBehaviorT<ImS64, ImS64, double>(bb, id, data_type, (ImS64*)p_v,  *(const ImS64*)p_min,  *(const ImS64*)p_max,  format, flags, out_grab_bb);
    case DataType::U64:
        IM_ASSERT(*(const ImU64*)p_max <= IM_U64_MAX / 2);
        return SliderBehaviorT<ImU64, ImS64, double>(bb, id, data_type, (ImU64*)p_v,  *(const ImU64*)p_min,  *(const ImU64*)p_max,  format, flags, out_grab_bb);
    case DataType::Float:
        IM_ASSERT(*(let*)p_min >= -FLT_MAX / 2.0 && *(let*)p_max <= FLT_MAX / 2.0);
        return SliderBehaviorT<float, float, float >(bb, id, data_type, (float*)p_v,  *(let*)p_min,  *(let*)p_max,  format, flags, out_grab_bb);
    case DataType::Double:
        IM_ASSERT(*(const double*)p_min >= -DBL_MAX / 2.0 && *(const double*)p_max <= DBL_MAX / 2.0);
        return SliderBehaviorT<double, double, double>(bb, id, data_type, (double*)p_v, *(const double*)p_min, *(const double*)p_max, format, flags, out_grab_bb);
    case DataType::COUNT: break;
    }
    IM_ASSERT(0);
    return false;
}

// Note: p_data, p_min and p_max are _pointers_ to a memory address holding the data. For a slider, they are all required.
// Read code of e.g. SliderFloat(), SliderInt() etc. or examples in 'Demo->Widgets->data Types' to understand how to use this function directly.
bool ImGui::SliderScalar(const char* label, DataType data_type, void* p_data, const void* p_min, const void* p_max, const char* format, ImGuiSliderFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;
    const Id32 id = window.GetID(label);
    let w = CalcItemWidth();

    const Vector2D label_size = CalcTextSize(label, None, true);
    const ImRect frame_bb(window.DC.CursorPos, window.DC.CursorPos + DimgVec2D::new(w, label_size.y + style.FramePadding.y * 2.0));
    const ImRect total_bb(frame_bb.Min, frame_bb.Max + DimgVec2D::new(label_size.x > 0.0 ? style.ItemInnerSpacing.x + label_size.x : 0.0, 0.0));

    const bool temp_input_allowed = (flags & ImGuiSliderFlags_NoInput) == 0;
    ItemSize(total_bb, style.FramePadding.y);
    if (!ItemAdd(total_bb, id, &frame_bb, temp_input_allowed ? ItemFlags::Inputable : 0))
        return false;

    // Default format string when passing None
    if (format == None)
        format = DataTypeGetInfo(data_type)->PrintFmt;
    else if (data_type == DataType::S32 && strcmp(format, "%d") != 0) // (FIXME-LEGACY: Patch old "%.0" format string to use "%d", read function more details.)
        format = PatchFormatStringFloatToInt(format);

    const bool hovered = ItemHoverable(frame_bb, id);
    bool temp_input_is_active = temp_input_allowed && TempInputIsActive(id);
    if (!temp_input_is_active)
    {
        // Tabbing or CTRL-clicking on Slider turns it into an input box
        const bool input_requested_by_tabbing = temp_input_allowed && (g.last_item_data.StatusFlags & ItemStatusFlags::FocusedByTabbing) != 0;
        const bool clicked = (hovered && g.IO.MouseClicked[0]);
        const bool make_active = (input_requested_by_tabbing || clicked || g.NavActivateId == id || g.NavActivateInputId == id);
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
        const bool is_clamp_input = (flags & ImGuiSliderFlags_AlwaysClamp) != 0;
        return TempInputScalar(frame_bb, id, label, data_type, p_data, format, is_clamp_input ? p_min : None, is_clamp_input ? p_max : None);
    }

    // Draw frame
    const ImU32 frame_col = GetColorU32(g.ActiveId == id ? ImGuiCol_FrameBgActive : hovered ? ImGuiCol_FrameBgHovered : ImGuiCol_FrameBg);
    RenderNavHighlight(frame_bb, id);
    RenderFrame(frame_bb.Min, frame_bb.Max, frame_col, true, g.Style.frame_rounding);

    // Slider behavior
    ImRect grab_bb;
    const bool value_changed = SliderBehavior(frame_bb, id, data_type, p_data, p_min, p_max, format, flags, &grab_bb);
    if (value_changed)
        MarkItemEdited(id);

    // Render grab
    if (grab_bb.Max.x > grab_bb.Min.x)
        window.draw_list->AddRectFilled(grab_bb.Min, grab_bb.Max, GetColorU32(g.ActiveId == id ? ImGuiCol_SliderGrabActive : ImGuiCol_SliderGrab), style.GrabRounding);

    // Display value using user-provided display format so user can add prefix/suffix/decorations to the value.
    char value_buf[64];
    const char* value_buf_end = value_buf + DataTypeFormatString(value_buf, IM_ARRAYSIZE(value_buf), data_type, p_data, format);
    if (g.log_enabled)
        LogSetNextTextDecoration("{", "}");
    render_textClipped(frame_bb.Min, frame_bb.Max, value_buf, value_buf_end, None, DimgVec2D::new(0.5, 0.5));

    if (label_size.x > 0.0)
        render_text(DimgVec2D::new(frame_bb.Max.x + style.ItemInnerSpacing.x, frame_bb.Min.y + style.FramePadding.y), label);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.last_item_data.StatusFlags);
    return value_changed;
}

// Add multiple sliders on 1 line for compact edition of multiple components
bool ImGui::SliderScalarN(const char* label, DataType data_type, void* v, int components, const void* v_min, const void* v_max, const char* format, ImGuiSliderFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    bool value_changed = false;
    BeginGroup();
    push_id(label);
    PushMultiItemsWidths(components, CalcItemWidth());
    size_t type_size = GDataTypeInfo[data_type].Size;
    for (int i = 0; i < components; i += 1)
    {
        push_id(i);
        if (i > 0)
            same_line(0, g.Style.ItemInnerSpacing.x);
        value_changed |= SliderScalar("", data_type, v, v_min, v_max, format, flags);
        pop_id();
        PopItemWidth();
        v = (void*)((char*)v + type_size);
    }
    pop_id();

    const char* label_end = FindRenderedTextEnd(label);
    if (label != label_end)
    {
        same_line(0, g.Style.ItemInnerSpacing.x);
        TextEx(label, label_end);
    }

    EndGroup();
    return value_changed;
}

bool ImGui::SliderFloat(const char* label, float* v, float v_min, float v_max, const char* format, ImGuiSliderFlags flags)
{
    return SliderScalar(label, DataType::Float, v, &v_min, &v_max, format, flags);
}

bool ImGui::SliderFloat2(const char* label, float v[2], float v_min, float v_max, const char* format, ImGuiSliderFlags flags)
{
    return SliderScalarN(label, DataType::Float, v, 2, &v_min, &v_max, format, flags);
}

bool ImGui::SliderFloat3(const char* label, float v[3], float v_min, float v_max, const char* format, ImGuiSliderFlags flags)
{
    return SliderScalarN(label, DataType::Float, v, 3, &v_min, &v_max, format, flags);
}

bool ImGui::SliderFloat4(const char* label, float v[4], float v_min, float v_max, const char* format, ImGuiSliderFlags flags)
{
    return SliderScalarN(label, DataType::Float, v, 4, &v_min, &v_max, format, flags);
}

bool ImGui::SliderAngle(const char* label, float* v_rad, float v_degrees_min, float v_degrees_max, const char* format, ImGuiSliderFlags flags)
{
    if (format == None)
        format = "%.0 deg";
    let v_deg =  (*v_rad) * 360.0 / (2 * IM_PI);
    bool value_changed = SliderFloat(label, &v_deg, v_degrees_min, v_degrees_max, format, flags);
    *v_rad = v_deg * (2 * IM_PI) / 360.0;
    return value_changed;
}

bool ImGui::SliderInt(const char* label, int* v, int v_min, int v_max, const char* format, ImGuiSliderFlags flags)
{
    return SliderScalar(label, DataType::S32, v, &v_min, &v_max, format, flags);
}

bool ImGui::SliderInt2(const char* label, int v[2], int v_min, int v_max, const char* format, ImGuiSliderFlags flags)
{
    return SliderScalarN(label, DataType::S32, v, 2, &v_min, &v_max, format, flags);
}

bool ImGui::SliderInt3(const char* label, int v[3], int v_min, int v_max, const char* format, ImGuiSliderFlags flags)
{
    return SliderScalarN(label, DataType::S32, v, 3, &v_min, &v_max, format, flags);
}

bool ImGui::SliderInt4(const char* label, int v[4], int v_min, int v_max, const char* format, ImGuiSliderFlags flags)
{
    return SliderScalarN(label, DataType::S32, v, 4, &v_min, &v_max, format, flags);
}

bool ImGui::VSliderScalar(const char* label, const Vector2D& size, DataType data_type, void* p_data, const void* p_min, const void* p_max, const char* format, ImGuiSliderFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;
    const Id32 id = window.GetID(label);

    const Vector2D label_size = CalcTextSize(label, None, true);
    const ImRect frame_bb(window.DC.CursorPos, window.DC.CursorPos + size);
    const ImRect bb(frame_bb.Min, frame_bb.Max + DimgVec2D::new(label_size.x > 0.0 ? style.ItemInnerSpacing.x + label_size.x : 0.0, 0.0));

    ItemSize(bb, style.FramePadding.y);
    if (!ItemAdd(frame_bb, id))
        return false;

    // Default format string when passing None
    if (format == None)
        format = DataTypeGetInfo(data_type)->PrintFmt;
    else if (data_type == DataType::S32 && strcmp(format, "%d") != 0) // (FIXME-LEGACY: Patch old "%.0" format string to use "%d", read function more details.)
        format = PatchFormatStringFloatToInt(format);

    const bool hovered = ItemHoverable(frame_bb, id);
    if ((hovered && g.IO.MouseClicked[0]) || g.NavActivateId == id || g.NavActivateInputId == id)
    {
        SetActiveID(id, window);
        SetFocusID(id, window);
        FocusWindow(window);
        g.ActiveIdUsingNavDirMask |= (1 << ImGuiDir_Up) | (1 << ImGuiDir_Down);
    }

    // Draw frame
    const ImU32 frame_col = GetColorU32(g.ActiveId == id ? ImGuiCol_FrameBgActive : hovered ? ImGuiCol_FrameBgHovered : ImGuiCol_FrameBg);
    RenderNavHighlight(frame_bb, id);
    RenderFrame(frame_bb.Min, frame_bb.Max, frame_col, true, g.Style.frame_rounding);

    // Slider behavior
    ImRect grab_bb;
    const bool value_changed = SliderBehavior(frame_bb, id, data_type, p_data, p_min, p_max, format, flags | ImGuiSliderFlags_Vertical, &grab_bb);
    if (value_changed)
        MarkItemEdited(id);

    // Render grab
    if (grab_bb.Max.y > grab_bb.Min.y)
        window.draw_list->AddRectFilled(grab_bb.Min, grab_bb.Max, GetColorU32(g.ActiveId == id ? ImGuiCol_SliderGrabActive : ImGuiCol_SliderGrab), style.GrabRounding);

    // Display value using user-provided display format so user can add prefix/suffix/decorations to the value.
    // For the vertical slider we allow centered text to overlap the frame padding
    char value_buf[64];
    const char* value_buf_end = value_buf + DataTypeFormatString(value_buf, IM_ARRAYSIZE(value_buf), data_type, p_data, format);
    render_textClipped(DimgVec2D::new(frame_bb.Min.x, frame_bb.Min.y + style.FramePadding.y), frame_bb.Max, value_buf, value_buf_end, None, DimgVec2D::new(0.5, 0.0));
    if (label_size.x > 0.0)
        render_text(DimgVec2D::new(frame_bb.Max.x + style.ItemInnerSpacing.x, frame_bb.Min.y + style.FramePadding.y), label);

    return value_changed;
}

bool ImGui::VSliderFloat(const char* label, const Vector2D& size, float* v, float v_min, float v_max, const char* format, ImGuiSliderFlags flags)
{
    return VSliderScalar(label, size, DataType::Float, v, &v_min, &v_max, format, flags);
}

bool ImGui::VSliderInt(const char* label, const Vector2D& size, int* v, int v_min, int v_max, const char* format, ImGuiSliderFlags flags)
{
    return VSliderScalar(label, size, DataType::S32, v, &v_min, &v_max, format, flags);
}

#ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS

// Obsolete versions with power parameter. See https://github.com/ocornut/imgui/issues/3361 for details.
bool ImGui::SliderScalar(const char* label, DataType data_type, void* p_data, const void* p_min, const void* p_max, const char* format, float power)
{
    ImGuiSliderFlags slider_flags = ImGuiSliderFlags_None;
    if (power != 1.0)
    {
        IM_ASSERT(power == 1.0 && "Call function with ImGuiSliderFlags_Logarithmic flags instead of using the old 'float power' function!");
        slider_flags |= ImGuiSliderFlags_Logarithmic;   // Fallback for non-asserting paths
    }
    return SliderScalar(label, data_type, p_data, p_min, p_max, format, slider_flags);
}

bool ImGui::SliderScalarN(const char* label, DataType data_type, void* v, int components, const void* v_min, const void* v_max, const char* format, float power)
{
    ImGuiSliderFlags slider_flags = ImGuiSliderFlags_None;
    if (power != 1.0)
    {
        IM_ASSERT(power == 1.0 && "Call function with ImGuiSliderFlags_Logarithmic flags instead of using the old 'float power' function!");
        slider_flags |= ImGuiSliderFlags_Logarithmic;   // Fallback for non-asserting paths
    }
    return SliderScalarN(label, data_type, v, components, v_min, v_max, format, slider_flags);
}

#endif // IMGUI_DISABLE_OBSOLETE_FUNCTIONS

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
const char* ImParseFormatFindStart(const char* fmt)
{
    while (char c = fmt[0])
    {
        if (c == '%' && fmt[1] != '%')
            return fmt;
        else if (c == '%')
            fmt += 1;
        fmt += 1;
    }
    return fmt;
}

const char* ImParseFormatFindEnd(const char* fmt)
{
    // Printf/scanf types modifiers: I/L/h/j/l/t/w/z. Other uppercase letters qualify as types aka end of the format.
    if (fmt[0] != '%')
        return fmt;
    const unsigned int ignored_uppercase_mask = (1 << ('I'-'A')) | (1 << ('L'-'A'));
    const unsigned int ignored_lowercase_mask = (1 << ('h'-'a')) | (1 << ('j'-'a')) | (1 << ('l'-'a')) | (1 << ('t'-'a')) | (1 << ('w'-'a')) | (1 << ('z'-'a'));
    for (char c; (c = *fmt) != 0; fmt += 1)
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
//  fmt = "%.3"       -> return fmt
//  fmt = "hello %.3" -> return fmt + 6
//  fmt = "%.3 hello" -> return buf written with "%.3"
const char* ImParseFormatTrimDecorations(const char* fmt, char* buf, size_t buf_size)
{
    const char* fmt_start = ImParseFormatFindStart(fmt);
    if (fmt_start[0] != '%')
        return fmt;
    const char* fmt_end = ImParseFormatFindEnd(fmt_start);
    if (fmt_end[0] == 0) // If we only have leading decoration, we don't need to copy the data.
        return fmt_start;
    ImStrncpy(buf, fmt_start, ImMin((fmt_end - fmt_start) + 1, buf_size));
    return buf;
}

// Sanitize format
// - Zero terminate so extra characters after format (e.g. "%f123") don't confuse atof/atoi
// - stb_sprintf.h supports several new modifiers which format numbers in a way that also makes them incompatible atof/atoi.
void ImParseFormatSanitizeForPrinting(const char* fmt_in, char* fmt_out, size_t fmt_out_size)
{
    const char* fmt_end = ImParseFormatFindEnd(fmt_in);
    IM_UNUSED(fmt_out_size);
    IM_ASSERT((fmt_end - fmt_in + 1) < fmt_out_size); // Format is too long, let us know if this happens to you!
    while (fmt_in < fmt_end)
    {
        char c = *fmt_in += 1;
        if (c != '\'' && c != '$' && c != '_') // Custom flags provided by stb_sprintf.h. POSIX 2008 also supports '.
            *(fmt_out += 1) = c;
    }
    *fmt_out = 0; // Zero-terminate
}

// - For scanning we need to remove all width and precision fields "%3.7" -> "%f". BUT don't strip types like "%I64d" which includes digits. ! "%07I64d" -> "%I64d"
const char* ImParseFormatSanitizeForScanning(const char* fmt_in, char* fmt_out, size_t fmt_out_size)
{
    const char* fmt_end = ImParseFormatFindEnd(fmt_in);
    const char* fmt_out_begin = fmt_out;
    IM_UNUSED(fmt_out_size);
    IM_ASSERT((fmt_end - fmt_in + 1) < fmt_out_size); // Format is too long, let us know if this happens to you!
    bool has_type = false;
    while (fmt_in < fmt_end)
    {
        char c = *fmt_in += 1;
        if (!has_type && ((c >= '0' && c <= '9') || c == '.'))
            continue;
        has_type |= ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')); // Stop skipping digits
        if (c != '\'' && c != '$' && c != '_') // Custom flags provided by stb_sprintf.h. POSIX 2008 also supports '.
            *(fmt_out += 1) = c;
    }
    *fmt_out = 0; // Zero-terminate
    return fmt_out_begin;
}

template<typename TYPE>
static const char* ImAtoi(const char* src, TYPE* output)
{
    int negative = 0;
    if (*src == '-') { negative = 1; src += 1; }
    if (*src == '+') { src += 1; }
    TYPE v = 0;
    while (*src >= '0' && *src <= '9')
        v = (v * 10) + (*src += 1 - '0');
    *output = negative ? -v : v;
    return src;
}

// Parse display precision back from the display format string
// FIXME: This is still used by some navigation code path to infer a minimum tweak step, but we should aim to rework widgets so it isn't needed.
int ImParseFormatPrecision(const char* fmt, int default_precision)
{
    fmt = ImParseFormatFindStart(fmt);
    if (fmt[0] != '%')
        return default_precision;
    fmt += 1;
    while (*fmt >= '0' && *fmt <= '9')
        fmt += 1;
    int precision = INT_MAX;
    if (*fmt == '.')
    {
        fmt = ImAtoi<int>(fmt + 1, &precision);
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
bool ImGui::TempInputText(const ImRect& bb, Id32 id, const char* label, char* buf, int buf_size, ImGuiInputTextFlags flags)
{
    // On the first frame, g.TempInputTextId == 0, then on subsequent frames it becomes == id.
    // We clear ActiveID on the first frame to allow the InputText() taking it back.
    // ImGuiContext& g = *GImGui;
    const bool init = (g.TempInputId != id);
    if (init)
        ClearActiveID();

    g.current_window_id->DC.CursorPos = bb.Min;
    bool value_changed = InputTextEx(label, None, buf, buf_size, bb.GetSize(), flags | ImGuiInputTextFlags_MergedItem);
    if (init)
    {
        // First frame we started displaying the InputText widget, we expect it to take the active id.
        IM_ASSERT(g.ActiveId == id);
        g.TempInputId = g.ActiveId;
    }
    return value_changed;
}

static inline ImGuiInputTextFlags InputScalar_DefaultCharsFilter(DataType data_type, const char* format)
{
    if (data_type == DataType::Float || data_type == DataType::Double)
        return ImGuiInputTextFlags_CharsScientific;
    const char format_last_char = format[0] ? format[strlen(format) - 1] : 0;
    return (format_last_char == 'x' || format_last_char == 'X') ? ImGuiInputTextFlags_CharsHexadecimal : ImGuiInputTextFlags_CharsDecimal;
}

// Note that Drag/Slider functions are only forwarding the min/max values clamping values if the ImGuiSliderFlags_AlwaysClamp flag is set!
// This is intended: this way we allow CTRL+Click manual input to set a value out of bounds, for maximum flexibility.
// However this may not be ideal for all uses, as some user code may break on out of bound values.
bool ImGui::TempInputScalar(const ImRect& bb, Id32 id, const char* label, DataType data_type, void* p_data, const char* format, const void* p_clamp_min, const void* p_clamp_max)
{
    char fmt_buf[32];
    char data_buf[32];
    format = ImParseFormatTrimDecorations(format, fmt_buf, IM_ARRAYSIZE(fmt_buf));
    DataTypeFormatString(data_buf, IM_ARRAYSIZE(data_buf), data_type, p_data, format);
    ImStrTrimBlanks(data_buf);

    ImGuiInputTextFlags flags = ImGuiInputTextFlags_AutoSelectAll | ImGuiInputTextFlags_NoMarkEdited;
    flags |= InputScalar_DefaultCharsFilter(data_type, format);

    bool value_changed = false;
    if (TempInputText(bb, id, label, data_buf, IM_ARRAYSIZE(data_buf), flags))
    {
        // Backup old value
        size_t data_type_size = DataTypeGetInfo(data_type)->Size;
        DataTypeTempStorage data_backup;
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
// Read code of e.g. InputFloat(), InputInt() etc. or examples in 'Demo->Widgets->data Types' to understand how to use this function directly.
bool ImGui::InputScalar(const char* label, DataType data_type, void* p_data, const void* p_step, const void* p_step_fast, const char* format, ImGuiInputTextFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;

    if (format == None)
        format = DataTypeGetInfo(data_type)->PrintFmt;

    char buf[64];
    DataTypeFormatString(buf, IM_ARRAYSIZE(buf), data_type, p_data, format);

    // Testing active_id as a minor optimization as filtering is not needed until active
    if (g.ActiveId == 0 && (flags & (ImGuiInputTextFlags_CharsDecimal | ImGuiInputTextFlags_CharsHexadecimal | ImGuiInputTextFlags_CharsScientific)) == 0)
        flags |= InputScalar_DefaultCharsFilter(data_type, format);
    flags |= ImGuiInputTextFlags_AutoSelectAll | ImGuiInputTextFlags_NoMarkEdited; // We call MarkItemEdited() ourselves by comparing the actual data rather than the string.

    bool value_changed = false;
    if (p_step != None)
    {
        let button_size = get_frame_height();

        BeginGroup(); // The only purpose of the group here is to allow the caller to query item data e.g. IsItemActive()
        push_id(label);
        SetNextItemWidth(ImMax(1.0, CalcItemWidth() - (button_size + style.ItemInnerSpacing.x) * 2));
        if (InputText("", buf, IM_ARRAYSIZE(buf), flags)) // PushId(label) + "" gives us the expected id from outside point of view
            value_changed = DataTypeApplyFromText(buf, data_type, p_data, format);

        // step buttons
        const Vector2D backup_frame_padding = style.FramePadding;
        style.FramePadding.x = style.FramePadding.y;
        ImGuiButtonFlags button_flags = ImGuiButtonFlags_Repeat | ImGuiButtonFlags_DontClosePopups;
        if (flags & ImGuiInputTextFlags_ReadOnly)
            BeginDisabled();
        same_line(0, style.ItemInnerSpacing.x);
        if (ButtonEx("-", DimgVec2D::new(button_size, button_size), button_flags))
        {
            DataTypeApplyOp(data_type, '-', p_data, p_data, g.IO.KeyCtrl && p_step_fast ? p_step_fast : p_step);
            value_changed = true;
        }
        same_line(0, style.ItemInnerSpacing.x);
        if (ButtonEx("+", DimgVec2D::new(button_size, button_size), button_flags))
        {
            DataTypeApplyOp(data_type, '+', p_data, p_data, g.IO.KeyCtrl && p_step_fast ? p_step_fast : p_step);
            value_changed = true;
        }
        if (flags & ImGuiInputTextFlags_ReadOnly)
            EndDisabled();

        const char* label_end = FindRenderedTextEnd(label);
        if (label != label_end)
        {
            same_line(0, style.ItemInnerSpacing.x);
            TextEx(label, label_end);
        }
        style.FramePadding = backup_frame_padding;

        pop_id();
        EndGroup();
    }
    else
    {
        if (InputText(label, buf, IM_ARRAYSIZE(buf), flags))
            value_changed = DataTypeApplyFromText(buf, data_type, p_data, format);
    }
    if (value_changed)
        MarkItemEdited(g.last_item_data.id);

    return value_changed;
}

bool ImGui::InputScalarN(const char* label, DataType data_type, void* p_data, int components, const void* p_step, const void* p_step_fast, const char* format, ImGuiInputTextFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    bool value_changed = false;
    BeginGroup();
    push_id(label);
    PushMultiItemsWidths(components, CalcItemWidth());
    size_t type_size = GDataTypeInfo[data_type].Size;
    for (int i = 0; i < components; i += 1)
    {
        push_id(i);
        if (i > 0)
            same_line(0, g.Style.ItemInnerSpacing.x);
        value_changed |= InputScalar("", data_type, p_data, p_step, p_step_fast, format, flags);
        pop_id();
        PopItemWidth();
        p_data = (void*)((char*)p_data + type_size);
    }
    pop_id();

    const char* label_end = FindRenderedTextEnd(label);
    if (label != label_end)
    {
        same_line(0.0, g.Style.ItemInnerSpacing.x);
        TextEx(label, label_end);
    }

    EndGroup();
    return value_changed;
}

bool ImGui::InputFloat(const char* label, float* v, float step, float step_fast, const char* format, ImGuiInputTextFlags flags)
{
    flags |= ImGuiInputTextFlags_CharsScientific;
    return InputScalar(label, DataType::Float, (void*)v, (void*)(step > 0.0 ? &step : None), (void*)(step_fast > 0.0 ? &step_fast : None), format, flags);
}

bool ImGui::InputFloat2(const char* label, float v[2], const char* format, ImGuiInputTextFlags flags)
{
    return InputScalarN(label, DataType::Float, v, 2, None, None, format, flags);
}

bool ImGui::InputFloat3(const char* label, float v[3], const char* format, ImGuiInputTextFlags flags)
{
    return InputScalarN(label, DataType::Float, v, 3, None, None, format, flags);
}

bool ImGui::InputFloat4(const char* label, float v[4], const char* format, ImGuiInputTextFlags flags)
{
    return InputScalarN(label, DataType::Float, v, 4, None, None, format, flags);
}

bool ImGui::InputInt(const char* label, int* v, int step, int step_fast, ImGuiInputTextFlags flags)
{
    // Hexadecimal input provided as a convenience but the flag name is awkward. Typically you'd use InputText() to parse your own data, if you want to handle prefixes.
    const char* format = (flags & ImGuiInputTextFlags_CharsHexadecimal) ? "%08X" : "%d";
    return InputScalar(label, DataType::S32, (void*)v, (void*)(step > 0 ? &step : None), (void*)(step_fast > 0 ? &step_fast : None), format, flags);
}

bool ImGui::InputInt2(const char* label, int v[2], ImGuiInputTextFlags flags)
{
    return InputScalarN(label, DataType::S32, v, 2, None, None, "%d", flags);
}

bool ImGui::InputInt3(const char* label, int v[3], ImGuiInputTextFlags flags)
{
    return InputScalarN(label, DataType::S32, v, 3, None, None, "%d", flags);
}

bool ImGui::InputInt4(const char* label, int v[4], ImGuiInputTextFlags flags)
{
    return InputScalarN(label, DataType::S32, v, 4, None, None, "%d", flags);
}

bool ImGui::InputDouble(const char* label, double* v, double step, double step_fast, const char* format, ImGuiInputTextFlags flags)
{
    flags |= ImGuiInputTextFlags_CharsScientific;
    return InputScalar(label, DataType::Double, (void*)v, (void*)(step > 0.0 ? &step : None), (void*)(step_fast > 0.0 ? &step_fast : None), format, flags);
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

bool ImGui::InputText(const char* label, char* buf, size_t buf_size, ImGuiInputTextFlags flags, ImGuiInputTextCallback callback, void* user_data)
{
    IM_ASSERT(!(flags & ImGuiInputTextFlags_Multiline)); // call InputTextMultiline()
    return InputTextEx(label, None, buf, buf_size, DimgVec2D::new(0, 0), flags, callback, user_data);
}

bool ImGui::InputTextMultiline(const char* label, char* buf, size_t buf_size, const Vector2D& size, ImGuiInputTextFlags flags, ImGuiInputTextCallback callback, void* user_data)
{
    return InputTextEx(label, None, buf, buf_size, size, flags | ImGuiInputTextFlags_Multiline, callback, user_data);
}

bool ImGui::InputTextWithHint(const char* label, const char* hint, char* buf, size_t buf_size, ImGuiInputTextFlags flags, ImGuiInputTextCallback callback, void* user_data)
{
    IM_ASSERT(!(flags & ImGuiInputTextFlags_Multiline)); // call InputTextMultiline()
    return InputTextEx(label, hint, buf, buf_size, DimgVec2D::new(0, 0), flags, callback, user_data);
}

static int InputTextCalcTextLenAndLineCount(const char* text_begin, const char** out_text_end)
{
    int line_count = 0;
    const char* s = text_begin;
    while (char c = *s += 1) // We are only matching for \n so we can ignore UTF-8 decoding
        if (c == '\n')
            line_count += 1;
    s--;
    if (s[0] != '\n' && s[0] != '\r')
        line_count += 1;
    *out_text_end = s;
    return line_count;
}

// Wrapper for stb_textedit.h to edit text (our wrapper is for: statically sized buffer, single-line, wchar characters. InputText converts between UTF-8 and wchar)
namespace ImStb
{

} // namespace ImStb

void ImGuiInputTextState::OnKeyPressed(int key)
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
void ImGuiInputTextCallbackData::DeleteChars(int pos, int bytes_count)
{
    IM_ASSERT(pos + bytes_count <= BufTextLen);
    char* dst = Buf + pos;
    const char* src = Buf + pos + bytes_count;
    while (char c = *src += 1)
        *dst += 1 = c;
    *dst = '\0';

    if (CursorPos >= pos + bytes_count)
        CursorPos -= bytes_count;
    else if (CursorPos >= pos)
        CursorPos = pos;
    SelectionStart = SelectionEnd = CursorPos;
    BufDirty = true;
    BufTextLen -= bytes_count;
}

void ImGuiInputTextCallbackData::InsertChars(int pos, const char* new_text, const char* new_text_end)
{
    const bool is_resizable = (Flags & ImGuiInputTextFlags_CallbackResize) != 0;
    let new_text_len = new_text_end ? (new_text_end - new_text) : strlen(new_text);
    if (new_text_len + BufTextLen >= BufSize)
    {
        if (!is_resizable)
            return;

        // Contrary to STB_TEXTEDIT_INSERTCHARS() this is working in the UTF8 buffer, hence the mildly similar code (until we remove the U16 buffer altogether!)
        // ImGuiContext& g = *GImGui;
        ImGuiInputTextState* edit_state = &g.InputTextState;
        IM_ASSERT(edit_state->ID != 0 && g.ActiveId == edit_state->ID);
        IM_ASSERT(Buf == edit_state->TextA.Data);
        int new_buf_size = BufTextLen + ImClamp(new_text_len * 4, 32, ImMax(256, new_text_len)) + 1;
        edit_state->TextA.reserve(new_buf_size + 1);
        Buf = edit_state->TextA.Data;
        BufSize = edit_state->BufCapacityA = new_buf_size;
    }

    if (BufTextLen != pos)
        memmove(Buf + pos + new_text_len, Buf + pos, (BufTextLen - pos));
    memcpy(Buf + pos, new_text, new_text_len * sizeof(char));
    Buf[BufTextLen + new_text_len] = '\0';

    if (CursorPos >= pos)
        CursorPos += new_text_len;
    SelectionStart = SelectionEnd = CursorPos;
    BufDirty = true;
    BufTextLen += new_text_len;
}

// Return false to discard a character.
static bool InputTextFilterCharacter(unsigned int* p_char, ImGuiInputTextFlags flags, ImGuiInputTextCallback callback, void* user_data, ImGuiInputSource input_source)
{
    IM_ASSERT(input_source == ImGuiInputSource_Keyboard || input_source == ImGuiInputSource_Clipboard);
    unsigned int c = *p_char;

    // Filter non-printable (NB: isprint is unreliable! see #2467)
    bool apply_named_filters = true;
    if (c < 0x20)
    {
        bool pass = false;
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
        if (c >= 0xE000 && c <= 0xF8FF)
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
        //   ImGui::GetCurrentContext()->PlatformLocaleDecimalPoint = *localeconv()->decimal_point;
        // Users of non-default decimal point (in particular ',') may be affected by word-selection logic (is_word_boundary_from_right/is_word_boundary_from_left) functions.
        // ImGuiContext& g = *GImGui;
        const unsigned c_decimal_point = (unsigned int)g.PlatformLocaleDecimalPoint;

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
                *p_char = (c += (unsigned int)('A' - 'a'));

        if (flags & ImGuiInputTextFlags_CharsNoBlank)
            if (ImCharIsBlankW(c))
                return false;
    }

    // Custom callback filter
    if (flags & ImGuiInputTextFlags_CallbackCharFilter)
    {
        ImGuiInputTextCallbackData callback_data;
        memset(&callback_data, 0, sizeof(ImGuiInputTextCallbackData));
        callback_data.EventFlag = ImGuiInputTextFlags_CallbackCharFilter;
        callback_data.EventChar = (ImWchar)c;
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
static void InputTextReconcileUndoStateAfterUserCallback(ImGuiInputTextState* state, const char* new_buf_a, int new_length_a)
{
    // ImGuiContext& g = *GImGui;
    const ImWchar* old_buf = state->TextW.Data;
    let old_length = state->CurLenW;
    let new_length = ImTextCountCharsFromUtf8(new_buf_a, new_buf_a + new_length_a);
    g.TempBuffer.reserve_discard((new_length + 1) * sizeof(ImWchar));
    ImWchar* new_buf = (ImWchar*)(void*)g.TempBuffer.Data;
    ImTextStrFromUtf8(new_buf, new_length + 1, new_buf_a, new_buf_a + new_length_a);

    let shorter_length = ImMin(old_length, new_length);
    int first_diff;
    for (first_diff = 0; first_diff < shorter_length; first_diff += 1)
        if (old_buf[first_diff] != new_buf[first_diff])
            break;
    if (first_diff == old_length && first_diff == new_length)
        return;

    int old_last_diff = old_length - 1;
    int new_last_diff = new_length - 1;
    for (; old_last_diff >= first_diff && new_last_diff >= first_diff; old_last_diff--, new_last_diff--)
        if (old_buf[old_last_diff] != new_buf[new_last_diff])
            break;

    let insert_len = new_last_diff - first_diff + 1;
    let delete_len = old_last_diff - first_diff + 1;
    if (insert_len > 0 || delete_len > 0)
        if (STB_TEXTEDIT_CHARTYPE* p = stb_text_createundo(&state->Stb.undostate, first_diff, delete_len, insert_len))
            for (int i = 0; i < delete_len; i += 1)
                p[i] = ImStb::STB_TEXTEDIT_GETCHAR(state, first_diff + i);
}

// Edit a string of text
// - buf_size account for the zero-terminator, so a buf_size of 6 can hold "Hello" but not "Hello!".
//   This is so we can easily call InputText() on static arrays using ARRAYSIZE() and to match
//   Note that in std::string world, capacity() would omit 1 byte used by the zero-terminator.
// - When active, hold on a privately held copy of the text (and apply back to 'buf'). So changing 'buf' while the InputText is active has no effect.
// - If you want to use ImGui::InputText() with std::string, see misc/cpp/imgui_stdlib.h
// (FIXME: Rather confusing and messy function, among the worse part of our codebase, expecting to rewrite a V2 at some point.. Partly because we are
//  doing UTF8 > U16 > UTF8 conversions on the go to easily interface with stb_textedit. Ideally should stay in UTF-8 all the time. See https://github.com/nothings/stb/issues/188)
bool ImGui::InputTextEx(const char* label, const char* hint, char* buf, int buf_size, const Vector2D& size_arg, ImGuiInputTextFlags flags, ImGuiInputTextCallback callback, void* callback_user_data)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    IM_ASSERT(buf != None && buf_size >= 0);
    IM_ASSERT(!((flags & ImGuiInputTextFlags_CallbackHistory) && (flags & ImGuiInputTextFlags_Multiline)));        // Can't use both together (they both use up/down keys)
    IM_ASSERT(!((flags & ImGuiInputTextFlags_CallbackCompletion) && (flags & ImGuiInputTextFlags_AllowTabInput))); // Can't use both together (they both use tab key)

    // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.IO;
    const ImGuiStyle& style = g.Style;

    const bool RENDER_SELECTION_WHEN_INACTIVE = false;
    const bool is_multiline = (flags & ImGuiInputTextFlags_Multiline) != 0;
    const bool is_readonly = (flags & ImGuiInputTextFlags_ReadOnly) != 0;
    const bool is_password = (flags & ImGuiInputTextFlags_Password) != 0;
    const bool is_undoable = (flags & ImGuiInputTextFlags_NoUndoRedo) == 0;
    const bool is_resizable = (flags & ImGuiInputTextFlags_CallbackResize) != 0;
    if (is_resizable)
        IM_ASSERT(callback != None); // Must provide a callback if you set the ImGuiInputTextFlags_CallbackResize flag!

    if (is_multiline) // Open group before calling GetID() because groups tracks id created within their scope (including the scrollbar)
        BeginGroup();
    const Id32 id = window.GetID(label);
    const Vector2D label_size = CalcTextSize(label, None, true);
    const Vector2D frame_size = CalcItemSize(size_arg, CalcItemWidth(), (is_multiline ? g.FontSize * 8.0 : label_size.y) + style.FramePadding.y * 2.0); // Arbitrary default of 8 lines high for multi-line
    const Vector2D total_size = DimgVec2D::new(frame_size.x + (label_size.x > 0.0 ? style.ItemInnerSpacing.x + label_size.x : 0.0), frame_size.y);

    const ImRect frame_bb(window.DC.CursorPos, window.DC.CursorPos + frame_size);
    const ImRect total_bb(frame_bb.Min, frame_bb.Min + total_size);

    Window* draw_window = window;
    Vector2D inner_size = frame_size;
    ImGuiItemStatusFlags item_status_flags = 0;
    ImGuiLastItemData item_data_backup;
    if (is_multiline)
    {
        Vector2D backup_pos = window.DC.CursorPos;
        ItemSize(total_bb, style.FramePadding.y);
        if (!ItemAdd(total_bb, id, &frame_bb, ItemFlags::Inputable))
        {
            EndGroup();
            return false;
        }
        item_status_flags = g.last_item_data.StatusFlags;
        item_data_backup = g.last_item_data;
        window.DC.CursorPos = backup_pos;

        // We reproduce the contents of BeginChildFrame() in order to provide 'label' so our window internal data are easier to read/debug.
        // FIXME-NAV: Pressing NavActivate will trigger general child activation right before triggering our own below. Harmless but bizarre.
        PushStyleColor(ImGuiCol_ChildBg, style.Colors[ImGuiCol_FrameBg]);
        PushStyleVar(ImGuiStyleVar_ChildRounding, style.frame_rounding);
        PushStyleVar(ImGuiStyleVar_ChildBorderSize, style.FrameBorderSize);
        PushStyleVar(ImGuiStyleVar_WindowPadding, DimgVec2D::new(0, 0)); // Ensure no clip rect so mouse hover can reach FramePadding edges
        bool child_visible = BeginChildEx(label, id, frame_bb.GetSize(), true, WindowFlags_NoMove);
        PopStyleVar(3);
        PopStyleColor();
        if (!child_visible)
        {
            EndChild();
            EndGroup();
            return false;
        }
        draw_window = g.current_window_id; // Child window
        draw_window.DC.nav_layers_active_mask_next |= (1 << draw_window.DC.NavLayerCurrent); // This is to ensure that EndChild() will display a navigation highlight so we can "enter" into it.
        draw_window.DC.CursorPos += style.FramePadding;
        inner_size.x -= draw_window.ScrollbarSizes.x;
    }
    else
    {
        // Support for internal ImGuiInputTextFlags_MergedItem flag, which could be redesigned as an ItemFlags if needed (with test performed in ItemAdd)
        ItemSize(total_bb, style.FramePadding.y);
        if (!(flags & ImGuiInputTextFlags_MergedItem))
            if (!ItemAdd(total_bb, id, &frame_bb, ItemFlags::Inputable))
                return false;
        item_status_flags = g.last_item_data.StatusFlags;
    }
    const bool hovered = ItemHoverable(frame_bb, id);
    if (hovered)
        g.MouseCursor = ImGuiMouseCursor_TextInput;

    // We are only allowed to access the state if we are already the active widget.
    ImGuiInputTextState* state = GetInputTextState(id);

    const bool input_requested_by_tabbing = (item_status_flags & ItemStatusFlags::FocusedByTabbing) != 0;
    const bool input_requested_by_nav = (g.ActiveId != id) && ((g.NavActivateInputId == id) || (g.NavActivateId == id && g.NavInputSource == ImGuiInputSource_Keyboard));

    const bool user_clicked = hovered && io.MouseClicked[0];
    const bool user_scroll_finish = is_multiline && state != None && g.ActiveId == 0 && g.ActiveIdPreviousFrame == GetWindowScrollbarID(draw_window, ImGuiAxis_Y);
    const bool user_scroll_active = is_multiline && state != None && g.ActiveId == GetWindowScrollbarID(draw_window, ImGuiAxis_Y);
    bool clear_active_id = false;
    bool select_all = false;

    let scroll_y =  is_multiline ? draw_window.Scroll.y : FLT_MAX;

    const bool init_changed_specs = (state != None && state->Stb.single_line != !is_multiline);
    const bool init_make_active = (user_clicked || user_scroll_finish || input_requested_by_nav || input_requested_by_tabbing);
    const bool init_state = (init_make_active || user_scroll_active);
    if ((init_state && g.ActiveId != id) || init_changed_specs)
    {
        // Access state even if we don't own it yet.
        state = &g.InputTextState;
        state->CursorAnimReset();

        // Take a copy of the initial buffer value (both in original UTF-8 format and converted to wchar)
        // From the moment we focused we are ignoring the content of 'buf' (unless we are in read-only mode)
        let buf_len = strlen(buf);
        state->InitialTextA.resize(buf_len + 1);    // UTF-8. we use +1 to make sure that .data is always pointing to at least an empty string.
        memcpy(state->InitialTextA.Data, buf, buf_len + 1);

        // Preserve cursor position and undo/redo stack if we come back to same widget
        // FIXME: Since we reworked this on 2022/06, may want to differenciate recycle_cursor vs recycle_undostate?
        bool recycle_state = (state->ID == id && !init_changed_specs);
        if (recycle_state && (state->CurLenA != buf_len || (state->TextAIsValid && strncmp(state->TextA.Data, buf, buf_len) != 0)))
            recycle_state = false;

        // Start edition
        const char* buf_end = None;
        state->ID = id;
        state->TextW.resize(buf_size + 1);          // wchar count <= UTF-8 count. we use +1 to make sure that .data is always pointing to at least an empty string.
        state->TextA.resize(0);
        state->TextAIsValid = false;                // TextA is not valid yet (we will display buf until then)
        state->CurLenW = ImTextStrFromUtf8(state->TextW.Data, buf_size, buf, None, &buf_end);
        state->CurLenA = (buf_end - buf);      // We can't get the result from ImStrncpy() above because it is not UTF-8 aware. Here we'll cut off malformed UTF-8.

        if (recycle_state)
        {
            // Recycle existing cursor/selection/undo stack but clamp position
            // Note a single mouse click will override the cursor/position immediately by calling stb_textedit_click handler.
            state->CursorClamp();
        }
        else
        {
            state->ScrollX = 0.0;
            stb_textedit_initialize_state(&state->Stb, !is_multiline);
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
            state->Stb.insert_mode = 1; // stb field name is indeed incorrect (see #2863)
    }

    if (g.ActiveId != id && init_make_active)
    {
        IM_ASSERT(state && state->ID == id);
        SetActiveID(id, window);
        SetFocusID(id, window);
        FocusWindow(window);

        // Declare our inputs
        IM_ASSERT(ImGuiNavInput_COUNT < 32);
        g.ActiveIdUsingNavDirMask |= (1 << ImGuiDir_Left) | (1 << ImGuiDir_Right);
        if (is_multiline || (flags & ImGuiInputTextFlags_CallbackHistory))
            g.ActiveIdUsingNavDirMask |= (1 << ImGuiDir_Up) | (1 << ImGuiDir_Down);
        g.ActiveIdUsingNavInputMask |= (1 << ImGuiNavInput_Cancel);
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

    // We have an edge case if active_id was set through another widget (e.g. widget being swapped), clear id immediately (don't wait until the end of the function)
    if (g.ActiveId == id && state == None)
        ClearActiveID();

    // Release focus when we click outside
    if (g.ActiveId == id && io.MouseClicked[0] && !init_state && !init_make_active) //-V560
        clear_active_id = true;

    // Lock the decision of whether we are going to take the path displaying the cursor or selection
    const bool render_cursor = (g.ActiveId == id) || (state && user_scroll_active);
    bool render_selection = state && (state->HasSelection() || select_all) && (RENDER_SELECTION_WHEN_INACTIVE || render_cursor);
    bool value_changed = false;
    bool enter_pressed = false;

    // When read-only we always use the live data passed to the function
    // FIXME-OPT: Because our selection/cursor code currently needs the wide text we need to convert it when active, which is not ideal :(
    if (is_readonly && state != None && (render_cursor || render_selection))
    {
        const char* buf_end = None;
        state->TextW.resize(buf_size + 1);
        state->CurLenW = ImTextStrFromUtf8(state->TextW.Data, state->TextW.Size, buf, None, &buf_end);
        state->CurLenA = (buf_end - buf);
        state->CursorClamp();
        render_selection &= state->HasSelection();
    }

    // Select the buffer to render.
    const bool buf_display_from_state = (render_cursor || render_selection || g.ActiveId == id) && !is_readonly && state && state->TextAIsValid;
    const bool is_displaying_hint = (hint != None && (buf_display_from_state ? state->TextA.Data : buf)[0] == 0);

    // Password pushes a temporary font with only a fallback glyph
    if (is_password && !is_displaying_hint)
    {
        const ImFontGlyph* glyph = g.Font->FindGlyph('*');
        ImFont* password_font = &g.InputTextPasswordFont;
        password_font->FontSize = g.Font->FontSize;
        password_font->Scale = g.Font->Scale;
        password_font->Ascent = g.Font->Ascent;
        password_font->Descent = g.Font->Descent;
        password_font->ContainerAtlas = g.Font->ContainerAtlas;
        password_font->FallbackGlyph = glyph;
        password_font->FallbackAdvanceX = glyph->AdvanceX;
        IM_ASSERT(password_font->Glyphs.empty() && password_font->IndexAdvanceX.empty() && password_font->IndexLookup.empty());
        PushFont(password_font);
    }

    // Process mouse inputs and character inputs
    int backup_current_text_length = 0;
    if (g.ActiveId == id)
    {
        IM_ASSERT(state != None);
        backup_current_text_length = state->CurLenA;
        state->Edited = false;
        state->BufCapacityA = buf_size;
        state->Flags = flags;

        // Although we are active we don't prevent mouse from hovering other elements unless we are interacting right now with the widget.
        // down the line we should have a cleaner library-wide concept of Selected vs active.
        g.active_id_allow_overlap = !io.MouseDown[0];
        g.WantTextInputNextFrame = 1;

        // Edit in progress
        let mouse_x = (io.MousePos.x - frame_bb.Min.x - style.FramePadding.x) + state->ScrollX;
        let mouse_y = (is_multiline ? (io.MousePos.y - draw_window.DC.CursorPos.y) : (g.FontSize * 0.5));

        const bool is_osx = io.ConfigMacOSXBehaviors;
        if (select_all)
        {
            state->SelectAll();
            state->SelectedAllMouseLock = true;
        }
        else if (hovered && io.MouseClickedCount[0] >= 2 && !io.KeyShift)
        {
            stb_textedit_click(state, &state->Stb, mouse_x, mouse_y);
            let multiclick_count = (io.MouseClickedCount[0] - 2);
            if ((multiclick_count % 2) == 0)
            {
                // Double-click: Select word
                // We always use the "Mac" word advance for double-click select vs CTRL+Right which use the platform dependent variant:
                // FIXME: There are likely many ways to improve this behavior, but there's no "right" behavior (depends on use-case, software, OS)
                const bool is_bol = (state->Stb.cursor == 0) || ImStb::STB_TEXTEDIT_GETCHAR(state, state->Stb.cursor - 1) == '\n';
                if (STB_TEXT_HAS_SELECTION(&state->Stb) || !is_bol)
                    state->OnKeyPressed(STB_TEXTEDIT_K_WORDLEFT);
                //state->OnKeyPressed(STB_TEXTEDIT_K_WORDRIGHT | STB_TEXTEDIT_K_SHIFT);
                if (!STB_TEXT_HAS_SELECTION(&state->Stb))
                    ImStb::stb_textedit_prep_selection_at_cursor(&state->Stb);
                state->Stb.cursor = ImStb::STB_TEXTEDIT_MOVEWORDRIGHT_MAC(state, state->Stb.cursor);
                state->Stb.select_end = state->Stb.cursor;
                ImStb::stb_textedit_clamp(state, &state->Stb);
            }
            else
            {
                // Triple-click: Select line
                const bool is_eol = ImStb::STB_TEXTEDIT_GETCHAR(state, state->Stb.cursor) == '\n';
                state->OnKeyPressed(STB_TEXTEDIT_K_LINESTART);
                state->OnKeyPressed(STB_TEXTEDIT_K_LINEEND | STB_TEXTEDIT_K_SHIFT);
                state->OnKeyPressed(STB_TEXTEDIT_K_RIGHT | STB_TEXTEDIT_K_SHIFT);
                if (!is_eol && is_multiline)
                {
                    ImSwap(state->Stb.select_start, state->Stb.select_end);
                    state->Stb.cursor = state->Stb.select_end;
                }
                state->CursorFollow = false;
            }
            state->CursorAnimReset();
        }
        else if (io.MouseClicked[0] && !state->SelectedAllMouseLock)
        {
            // FIXME: unselect on late click could be done release?
            if (hovered)
            {
                stb_textedit_click(state, &state->Stb, mouse_x, mouse_y);
                state->CursorAnimReset();
            }
        }
        else if (io.MouseDown[0] && !state->SelectedAllMouseLock && (io.MouseDelta.x != 0.0 || io.MouseDelta.y != 0.0))
        {
            stb_textedit_drag(state, &state->Stb, mouse_x, mouse_y);
            state->CursorAnimReset();
            state->CursorFollow = true;
        }
        if (state->SelectedAllMouseLock && !io.MouseDown[0])
            state->SelectedAllMouseLock = false;

        // We except backends to emit a Tab key but some also emit a Tab character which we ignore (#2467, #1336)
        // (For Tab and Enter: Win32/SFML/Allegro are sending both keys and chars, GLFW and SDL are only sending keys. For Space they all send all threes)
        const bool ignore_char_inputs = (io.KeyCtrl && !io.KeyAlt) || (is_osx && io.KeySuper);
        if ((flags & ImGuiInputTextFlags_AllowTabInput) && IsKeyPressed(ImGuiKey_Tab) && !ignore_char_inputs && !io.KeyShift && !is_readonly)
        {
            unsigned int c = '\t'; // Insert TAB
            if (InputTextFilterCharacter(&c, flags, callback, callback_user_data, ImGuiInputSource_Keyboard))
                state->OnKeyPressed(c);
        }

        // Process regular text input (before we check for Return because using some IME will effectively send a Return?)
        // We ignore CTRL inputs, but need to allow ALT+CTRL as some keyboards (e.g. German) use AltGR (which _is_ Alt+Ctrl) to input certain characters.
        if (io.InputQueueCharacters.Size > 0)
        {
            if (!ignore_char_inputs && !is_readonly && !input_requested_by_nav)
                for (int n = 0; n < io.InputQueueCharacters.Size; n += 1)
                {
                    // Insert character if they pass filtering
                    unsigned int c = (unsigned int)io.InputQueueCharacters[n];
                    if (c == '\t') // Skip Tab, see above.
                        continue;
                    if (InputTextFilterCharacter(&c, flags, callback, callback_user_data, ImGuiInputSource_Keyboard))
                        state->OnKeyPressed(c);
                }

            // Consume characters
            io.InputQueueCharacters.resize(0);
        }
    }

    // Process other shortcuts/key-presses
    bool cancel_edit = false;
    if (g.ActiveId == id && !g.ActiveIdIsJustActivated && !clear_active_id)
    {
        IM_ASSERT(state != None);

        let row_count_per_page = ImMax(((inner_size.y - style.FramePadding.y) / g.FontSize), 1);
        state->Stb.row_count_per_page = row_count_per_page;

        let k_mask = (io.KeyShift ? STB_TEXTEDIT_K_SHIFT : 0);
        const bool is_osx = io.ConfigMacOSXBehaviors;
        const bool is_osx_shift_shortcut = is_osx && (io.KeyMods == (ImGuiModFlags_Super | ImGuiModFlags_Shift));
        const bool is_wordmove_key_down = is_osx ? io.KeyAlt : io.KeyCtrl;                     // OS x style: Text editing cursor movement using Alt instead of Ctrl
        const bool is_startend_key_down = is_osx && io.KeySuper && !io.KeyCtrl && !io.KeyAlt;  // OS x style: Line/Text Start and End using Cmd+Arrows instead of Home/End
        const bool is_ctrl_key_only = (io.KeyMods == ImGuiModFlags_Ctrl);
        const bool is_shift_key_only = (io.KeyMods == ImGuiModFlags_Shift);
        const bool is_shortcut_key = g.IO.ConfigMacOSXBehaviors ? (io.KeyMods == ImGuiModFlags_Super) : (io.KeyMods == ImGuiModFlags_Ctrl);

        const bool is_cut   = ((is_shortcut_key && IsKeyPressed(ImGuiKey_X)) || (is_shift_key_only && IsKeyPressed(ImGuiKey_Delete))) && !is_readonly && !is_password && (!is_multiline || state->HasSelection());
        const bool is_copy  = ((is_shortcut_key && IsKeyPressed(ImGuiKey_C)) || (is_ctrl_key_only  && IsKeyPressed(ImGuiKey_Insert))) && !is_password && (!is_multiline || state->HasSelection());
        const bool is_paste = ((is_shortcut_key && IsKeyPressed(ImGuiKey_V)) || (is_shift_key_only && IsKeyPressed(ImGuiKey_Insert))) && !is_readonly;
        const bool is_undo  = ((is_shortcut_key && IsKeyPressed(ImGuiKey_Z)) && !is_readonly && is_undoable);
        const bool is_redo  = ((is_shortcut_key && IsKeyPressed(ImGuiKey_Y)) || (is_osx_shift_shortcut && IsKeyPressed(ImGuiKey_Z))) && !is_readonly && is_undoable;

        // We allow validate/cancel with Nav source (gamepad) to makes it easier to undo an accidental NavInput press with no keyboard wired, but otherwise it isn't very useful.
        const bool is_validate_enter = IsKeyPressed(ImGuiKey_Enter) || IsKeyPressed(ImGuiKey_KeypadEnter);
        const bool is_validate_nav = (IsNavInputTest(ImGuiNavInput_Activate, ImGuiNavReadMode_Pressed) && !IsKeyPressed(ImGuiKey_Space)) || IsNavInputTest(ImGuiNavInput_Input, ImGuiNavReadMode_Pressed);
        const bool is_cancel   = IsKeyPressed(ImGuiKey_Escape) || IsNavInputTest(ImGuiNavInput_Cancel, ImGuiNavReadMode_Pressed);

        if (IsKeyPressed(ImGuiKey_LeftArrow))                        { state->OnKeyPressed((is_startend_key_down ? STB_TEXTEDIT_K_LINESTART : is_wordmove_key_down ? STB_TEXTEDIT_K_WORDLEFT : STB_TEXTEDIT_K_LEFT) | k_mask); }
        else if (IsKeyPressed(ImGuiKey_RightArrow))                  { state->OnKeyPressed((is_startend_key_down ? STB_TEXTEDIT_K_LINEEND : is_wordmove_key_down ? STB_TEXTEDIT_K_WORDRIGHT : STB_TEXTEDIT_K_RIGHT) | k_mask); }
        else if (IsKeyPressed(ImGuiKey_UpArrow) && is_multiline)     { if (io.KeyCtrl) SetScrollY(draw_window, ImMax(draw_window.Scroll.y - g.FontSize, 0.0)); else state->OnKeyPressed((is_startend_key_down ? STB_TEXTEDIT_K_TEXTSTART : STB_TEXTEDIT_K_UP) | k_mask); }
        else if (IsKeyPressed(ImGuiKey_DownArrow) && is_multiline)   { if (io.KeyCtrl) SetScrollY(draw_window, ImMin(draw_window.Scroll.y + g.FontSize, GetScrollMaxY())); else state->OnKeyPressed((is_startend_key_down ? STB_TEXTEDIT_K_TEXTEND : STB_TEXTEDIT_K_DOWN) | k_mask); }
        else if (IsKeyPressed(ImGuiKey_PageUp) && is_multiline)      { state->OnKeyPressed(STB_TEXTEDIT_K_PGUP | k_mask); scroll_y -= row_count_per_page * g.FontSize; }
        else if (IsKeyPressed(ImGuiKey_PageDown) && is_multiline)    { state->OnKeyPressed(STB_TEXTEDIT_K_PGDOWN | k_mask); scroll_y += row_count_per_page * g.FontSize; }
        else if (IsKeyPressed(ImGuiKey_Home))                        { state->OnKeyPressed(io.KeyCtrl ? STB_TEXTEDIT_K_TEXTSTART | k_mask : STB_TEXTEDIT_K_LINESTART | k_mask); }
        else if (IsKeyPressed(ImGuiKey_End))                         { state->OnKeyPressed(io.KeyCtrl ? STB_TEXTEDIT_K_TEXTEND | k_mask : STB_TEXTEDIT_K_LINEEND | k_mask); }
        else if (IsKeyPressed(ImGuiKey_Delete) && !is_readonly && !is_cut) { state->OnKeyPressed(STB_TEXTEDIT_K_DELETE | k_mask); }
        else if (IsKeyPressed(ImGuiKey_Backspace) && !is_readonly)
        {
            if (!state->HasSelection())
            {
                if (is_wordmove_key_down)
                    state->OnKeyPressed(STB_TEXTEDIT_K_WORDLEFT | STB_TEXTEDIT_K_SHIFT);
                else if (is_osx && io.KeySuper && !io.KeyAlt && !io.KeyCtrl)
                    state->OnKeyPressed(STB_TEXTEDIT_K_LINESTART | STB_TEXTEDIT_K_SHIFT);
            }
            state->OnKeyPressed(STB_TEXTEDIT_K_BACKSPACE | k_mask);
        }
        else if (is_validate_enter)
        {
            bool ctrl_enter_for_new_line = (flags & ImGuiInputTextFlags_CtrlEnterForNewLine) != 0;
            if (!is_multiline || (ctrl_enter_for_new_line && !io.KeyCtrl) || (!ctrl_enter_for_new_line && io.KeyCtrl))
            {
                enter_pressed = clear_active_id = true;
            }
            else if (!is_readonly)
            {
                unsigned int c = '\n'; // Insert new line
                if (InputTextFilterCharacter(&c, flags, callback, callback_user_data, ImGuiInputSource_Keyboard))
                    state->OnKeyPressed(c);
            }
        }
        else if (is_validate_nav)
        {
            IM_ASSERT(!is_validate_enter);
            enter_pressed = clear_active_id = true;
        }
        else if (is_cancel)
        {
            clear_active_id = cancel_edit = true;
        }
        else if (is_undo || is_redo)
        {
            state->OnKeyPressed(is_undo ? STB_TEXTEDIT_K_UNDO : STB_TEXTEDIT_K_REDO);
            state->ClearSelection();
        }
        else if (is_shortcut_key && IsKeyPressed(ImGuiKey_A))
        {
            state->SelectAll();
            state->CursorFollow = true;
        }
        else if (is_cut || is_copy)
        {
            // Cut, Copy
            if (io.SetClipboardTextFn)
            {
                let ib = state->HasSelection() ? ImMin(state->Stb.select_start, state->Stb.select_end) : 0;
                let ie = state->HasSelection() ? ImMax(state->Stb.select_start, state->Stb.select_end) : state->CurLenW;
                let clipboard_data_len = ImTextCountUtf8BytesFromStr(state->TextW.Data + ib, state->TextW.Data + ie) + 1;
                char* clipboard_data = (char*)IM_ALLOC(clipboard_data_len * sizeof(char));
                ImTextStrToUtf8(clipboard_data, clipboard_data_len, state->TextW.Data + ib, state->TextW.Data + ie);
                SetClipboardText(clipboard_data);
                MemFree(clipboard_data);
            }
            if (is_cut)
            {
                if (!state->HasSelection())
                    state->SelectAll();
                state->CursorFollow = true;
                stb_textedit_cut(state, &state->Stb);
            }
        }
        else if (is_paste)
        {
            if (const char* clipboard = GetClipboardText())
            {
                // Filter pasted buffer
                let clipboard_len = strlen(clipboard);
                ImWchar* clipboard_filtered = (ImWchar*)IM_ALLOC((clipboard_len + 1) * sizeof(ImWchar));
                int clipboard_filtered_len = 0;
                for (const char* s = clipboard; *s; )
                {
                    unsigned int c;
                    s += ImTextCharFromUtf8(&c, s, None);
                    if (c == 0)
                        break;
                    if (!InputTextFilterCharacter(&c, flags, callback, callback_user_data, ImGuiInputSource_Clipboard))
                        continue;
                    clipboard_filtered[clipboard_filtered_len += 1] = (ImWchar)c;
                }
                clipboard_filtered[clipboard_filtered_len] = 0;
                if (clipboard_filtered_len > 0) // If everything was filtered, ignore the pasting operation
                {
                    stb_textedit_paste(state, &state->Stb, clipboard_filtered, clipboard_filtered_len);
                    state->CursorFollow = true;
                }
                MemFree(clipboard_filtered);
            }
        }

        // Update render selection flag after events have been handled, so selection highlight can be displayed during the same frame.
        render_selection |= state->HasSelection() && (RENDER_SELECTION_WHEN_INACTIVE || render_cursor);
    }

    // Process callbacks and apply result back to user's buffer.
    const char* apply_new_text = None;
    int apply_new_text_length = 0;
    if (g.ActiveId == id)
    {
        IM_ASSERT(state != None);
        if (cancel_edit)
        {
            // Restore initial value. Only return true if restoring to the initial value changes the current buffer contents.
            if (!is_readonly && strcmp(buf, state->InitialTextA.Data) != 0)
            {
                // Push records into the undo stack so we can CTRL+Z the revert operation itself
                apply_new_text = state->InitialTextA.Data;
                apply_new_text_length = state->InitialTextA.Size - 1;
                ImVector<ImWchar> w_text;
                if (apply_new_text_length > 0)
                {
                    w_text.resize(ImTextCountCharsFromUtf8(apply_new_text, apply_new_text + apply_new_text_length) + 1);
                    ImTextStrFromUtf8(w_text.Data, w_text.Size, apply_new_text, apply_new_text + apply_new_text_length);
                }
                stb_textedit_replace(state, &state->Stb, w_text.Data, (apply_new_text_length > 0) ? (w_text.Size - 1) : 0);
            }
        }

        // Apply ASCII value
        if (!is_readonly)
        {
            state->TextAIsValid = true;
            state->TextA.resize(state->TextW.Size * 4 + 1);
            ImTextStrToUtf8(state->TextA.Data, state->TextA.Size, state->TextW.Data, None);
        }

        // When using 'ImGuiInputTextFlags_EnterReturnsTrue' as a special case we reapply the live buffer back to the input buffer before clearing active_id, even though strictly speaking it wasn't modified on this frame.
        // If we didn't do that, code like InputInt() with ImGuiInputTextFlags_EnterReturnsTrue would fail.
        // This also allows the user to use InputText() with ImGuiInputTextFlags_EnterReturnsTrue without maintaining any user-side storage (please note that if you use this property along ImGuiInputTextFlags_CallbackResize you can end up with your temporary string object unnecessarily allocating once a frame, either store your string data, either if you don't then don't use ImGuiInputTextFlags_CallbackResize).
        const bool apply_edit_back_to_user_buffer = !cancel_edit || (enter_pressed && (flags & ImGuiInputTextFlags_EnterReturnsTrue) != 0);
        if (apply_edit_back_to_user_buffer)
        {
            // Apply new value immediately - copy modified buffer back
            // Note that as soon as the input box is active, the in-widget value gets priority over any underlying modification of the input buffer
            // FIXME: We actually always render 'buf' when calling draw_list->add_text, making the comment above incorrect.
            // FIXME-OPT: CPU waste to do this every time the widget is active, should mark dirty state from the stb_textedit callbacks.

            // User callback
            if ((flags & (ImGuiInputTextFlags_CallbackCompletion | ImGuiInputTextFlags_CallbackHistory | ImGuiInputTextFlags_CallbackEdit | ImGuiInputTextFlags_CallbackAlways)) != 0)
            {
                IM_ASSERT(callback != None);

                // The reason we specify the usage semantic (Completion/History) is that Completion needs to disable keyboard TABBING at the moment.
                ImGuiInputTextFlags event_flag = 0;
                ImGuiKey event_key = ImGuiKey_None;
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
                else if ((flags & ImGuiInputTextFlags_CallbackEdit) && state->Edited)
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

                    char* callback_buf = is_readonly ? buf : state->TextA.Data;
                    callback_data.EventKey = event_key;
                    callback_data.Buf = callback_buf;
                    callback_data.BufTextLen = state->CurLenA;
                    callback_data.BufSize = state->BufCapacityA;
                    callback_data.BufDirty = false;

                    // We have to convert from wchar-positions to UTF-8-positions, which can be pretty slow (an incentive to ditch the ImWchar buffer, see https://github.com/nothings/stb/issues/188)
                    ImWchar* text = state->TextW.Data;
                    let utf8_cursor_pos = callback_data.CursorPos = ImTextCountUtf8BytesFromStr(text, text + state->Stb.cursor);
                    let utf8_selection_start = callback_data.SelectionStart = ImTextCountUtf8BytesFromStr(text, text + state->Stb.select_start);
                    let utf8_selection_end = callback_data.SelectionEnd = ImTextCountUtf8BytesFromStr(text, text + state->Stb.select_end);

                    // Call user code
                    callback(&callback_data);

                    // Read back what user may have modified
                    callback_buf = is_readonly ? buf : state->TextA.Data; // Pointer may have been invalidated by a resize callback
                    IM_ASSERT(callback_data.Buf == callback_buf);         // Invalid to modify those fields
                    IM_ASSERT(callback_data.BufSize == state->BufCapacityA);
                    IM_ASSERT(callback_data.Flags == flags);
                    const bool buf_dirty = callback_data.BufDirty;
                    if (callback_data.CursorPos != utf8_cursor_pos || buf_dirty)            { state->Stb.cursor = ImTextCountCharsFromUtf8(callback_data.Buf, callback_data.Buf + callback_data.CursorPos); state->CursorFollow = true; }
                    if (callback_data.SelectionStart != utf8_selection_start || buf_dirty)  { state->Stb.select_start = (callback_data.SelectionStart == callback_data.CursorPos) ? state->Stb.cursor : ImTextCountCharsFromUtf8(callback_data.Buf, callback_data.Buf + callback_data.SelectionStart); }
                    if (callback_data.SelectionEnd != utf8_selection_end || buf_dirty)      { state->Stb.select_end = (callback_data.SelectionEnd == callback_data.SelectionStart) ? state->Stb.select_start : ImTextCountCharsFromUtf8(callback_data.Buf, callback_data.Buf + callback_data.SelectionEnd); }
                    if (buf_dirty)
                    {
                        IM_ASSERT(callback_data.BufTextLen == strlen(callback_data.Buf)); // You need to maintain BufTextLen if you change the text!
                        InputTextReconcileUndoStateAfterUserCallback(state, callback_data.Buf, callback_data.BufTextLen); // FIXME: Move the rest of this block inside function and rename to InputTextReconcileStateAfterUserCallback() ?
                        if (callback_data.BufTextLen > backup_current_text_length && is_resizable)
                            state->TextW.resize(state->TextW.Size + (callback_data.BufTextLen - backup_current_text_length)); // Worse case scenario resize
                        state->CurLenW = ImTextStrFromUtf8(state->TextW.Data, state->TextW.Size, callback_data.Buf, None);
                        state->CurLenA = callback_data.BufTextLen;  // Assume correct length and valid UTF-8 from user, saves us an extra strlen()
                        state->CursorAnimReset();
                    }
                }
            }

            // Will copy result string if modified
            if (!is_readonly && strcmp(state->TextA.Data, buf) != 0)
            {
                apply_new_text = state->TextA.Data;
                apply_new_text_length = state->CurLenA;
            }
        }

        // clear temporary user storage
        state->Flags = ImGuiInputTextFlags_None;
    }

    // Copy result to user buffer. This can currently only happen when (g.active_id == id)
    if (apply_new_text != None)
    {
        // We cannot test for 'backup_current_text_length != apply_new_text_length' here because we have no guarantee that the size
        // of our owned buffer matches the size of the string object held by the user, and by design we allow InputText() to be used
        // without any storage on user's side.
        IM_ASSERT(apply_new_text_length >= 0);
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
            IM_ASSERT(apply_new_text_length <= buf_size);
        }
        //IMGUI_DEBUG_PRINT("InputText(\"%s\"): apply_new_text length %d\n", label, apply_new_text_length);

        // If the underlying buffer resize was denied or not carried to the next frame, apply_new_text_length+1 may be >= buf_size.
        ImStrncpy(buf, apply_new_text, ImMin(apply_new_text_length + 1, buf_size));
        value_changed = true;
    }

    // Release active id at the end of the function (so e.g. pressing Return still does a final application of the value)
    if (clear_active_id && g.ActiveId == id)
        ClearActiveID();

    // Render frame
    if (!is_multiline)
    {
        RenderNavHighlight(frame_bb, id);
        RenderFrame(frame_bb.Min, frame_bb.Max, GetColorU32(ImGuiCol_FrameBg), true, style.frame_rounding);
    }

    const Vector4D clip_rect(frame_bb.Min.x, frame_bb.Min.y, frame_bb.Min.x + inner_size.x, frame_bb.Min.y + inner_size.y); // Not using frame_bb.max because we have adjusted size
    Vector2D draw_pos = is_multiline ? draw_window.DC.CursorPos : frame_bb.Min + style.FramePadding;
    Vector2D text_size(0.0, 0.0);

    // Set upper limit of single-line InputTextEx() at 2 million characters strings. The current pathological worst case is a long line
    // without any carriage return, which would makes ImFont::render_text() reserve too many vertices and probably crash. Avoid it altogether.
    // Note that we only use this limit on single-line InputText(), so a pathologically large line on a InputTextMultiline() would still crash.
    let buf_display_max_length = 2 * 1024 * 1024;
    const char* buf_display = buf_display_from_state ? state->TextA.Data : buf; //-V595
    const char* buf_display_end = None; // We have specialized paths below for setting the length
    if (is_displaying_hint)
    {
        buf_display = hint;
        buf_display_end = hint + strlen(hint);
    }

    // Render text. We currently only render selection when the widget is active or while scrolling.
    // FIXME: We could remove the '&& render_cursor' to keep rendering selection when inactive.
    if (render_cursor || render_selection)
    {
        IM_ASSERT(state != None);
        if (!is_displaying_hint)
            buf_display_end = buf_display + state->CurLenA;

        // Render text (with cursor and selection)
        // This is going to be messy. We need to:
        // - Display the text (this alone can be more easily clipped)
        // - Handle scrolling, highlight selection, display cursor (those all requires some form of 1d->2d cursor position calculation)
        // - Measure text height (for scrollbar)
        // We are attempting to do most of that in **one main pass** to minimize the computation cost (non-negligible for large amount of text) + 2nd pass for selection rendering (we could merge them by an extra refactoring effort)
        // FIXME: This should occur on buf_display but we'd need to maintain cursor/select_start/select_end for UTF-8.
        const ImWchar* text_begin = state->TextW.Data;
        Vector2D cursor_offset, select_start_offset;

        {
            // Find lines numbers straddling 'cursor' (slot 0) and 'select_start' (slot 1) positions.
            const ImWchar* searches_input_ptr[2] = { None, None };
            int searches_result_line_no[2] = { -1000, -1000 };
            int searches_remaining = 0;
            if (render_cursor)
            {
                searches_input_ptr[0] = text_begin + state->Stb.cursor;
                searches_result_line_no[0] = -1;
                searches_remaining += 1;
            }
            if (render_selection)
            {
                searches_input_ptr[1] = text_begin + ImMin(state->Stb.select_start, state->Stb.select_end);
                searches_result_line_no[1] = -1;
                searches_remaining += 1;
            }

            // Iterate all lines to find our line numbers
            // In multi-line mode, we never exit the loop until all lines are counted, so add one extra to the searches_remaining counter.
            searches_remaining += is_multiline ? 1 : 0;
            int line_count = 0;
            //for (const ImWchar* s = text_begin; (s = (const ImWchar*)wcschr((const wchar_t*)s, (wchar_t)'\n')) != None; s++)  // FIXME-OPT: Could use this when wchar_t are 16-bit
            for (const ImWchar* s = text_begin; *s != 0; s += 1)
                if (*s == '\n')
                {
                    line_count += 1;
                    if (searches_result_line_no[0] == -1 && s >= searches_input_ptr[0]) { searches_result_line_no[0] = line_count; if (--searches_remaining <= 0) break; }
                    if (searches_result_line_no[1] == -1 && s >= searches_input_ptr[1]) { searches_result_line_no[1] = line_count; if (--searches_remaining <= 0) break; }
                }
            line_count += 1;
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
                text_size = DimgVec2D::new(inner_size.x, line_count * g.FontSize);
        }

        // scroll
        if (render_cursor && state->CursorFollow)
        {
            // Horizontal scroll in chunks of quarter width
            if (!(flags & ImGuiInputTextFlags_NoHorizontalScroll))
            {
                let scroll_increment_x = inner_size.x * 0.25;
                let visible_width = inner_size.x - style.FramePadding.x;
                if (cursor_offset.x < state->ScrollX)
                    state->ScrollX = IM_FLOOR(ImMax(0.0, cursor_offset.x - scroll_increment_x));
                else if (cursor_offset.x - visible_width >= state->ScrollX)
                    state->ScrollX = IM_FLOOR(cursor_offset.x - visible_width + scroll_increment_x);
            }
            else
            {
                state->ScrollX = 0.0;
            }

            // Vertical scroll
            if (is_multiline)
            {
                // Test if cursor is vertically visible
                if (cursor_offset.y - g.FontSize < scroll_y)
                    scroll_y = ImMax(0.0, cursor_offset.y - g.FontSize);
                else if (cursor_offset.y - (inner_size.y - style.FramePadding.y * 2.0) >= scroll_y)
                    scroll_y = cursor_offset.y - inner_size.y + style.FramePadding.y * 2.0;
                let scroll_max_y = ImMax((text_size.y + style.FramePadding.y * 2.0) - inner_size.y, 0.0);
                scroll_y = ImClamp(scroll_y, 0.0, scroll_max_y);
                draw_pos.y += (draw_window.Scroll.y - scroll_y);   // Manipulate cursor pos immediately avoid a frame of lag
                draw_window.Scroll.y = scroll_y;
            }

            state->CursorFollow = false;
        }

        // Draw selection
        const Vector2D draw_scroll = DimgVec2D::new(state->ScrollX, 0.0);
        if (render_selection)
        {
            const ImWchar* text_selected_begin = text_begin + ImMin(state->Stb.select_start, state->Stb.select_end);
            const ImWchar* text_selected_end = text_begin + ImMax(state->Stb.select_start, state->Stb.select_end);

            ImU32 bg_color = GetColorU32(ImGuiCol_TextSelectedBg, render_cursor ? 1.0 : 0.6); // FIXME: current code flow mandate that render_cursor is always true here, we are leaving the transparent one for tests.
            let bg_offy_up =  is_multiline ? 0.0 : -1.0;    // FIXME: those offsets should be part of the style? they don't play so well with multi-line selection.
            let bg_offy_dn =  is_multiline ? 0.0 : 2.0;
            Vector2D rect_pos = draw_pos + select_start_offset - draw_scroll;
            for (const ImWchar* p = text_selected_begin; p < text_selected_end; )
            {
                if (rect_pos.y > clip_rect.w + g.FontSize)
                    break;
                if (rect_pos.y < clip_rect.y)
                {
                    //p = (const ImWchar*)wmemchr((const wchar_t*)p, '\n', text_selected_end - p);  // FIXME-OPT: Could use this when wchar_t are 16-bit
                    //p = p ? p + 1 : text_selected_end;
                    while (p < text_selected_end)
                        if (*p += 1 == '\n')
                            break;
                }
                else
                {
                    Vector2D rect_size = InputTextCalcTextSizeW(p, text_selected_end, &p, None, true);
                    if (rect_size.x <= 0.0) rect_size.x = IM_FLOOR(g.Font->GetCharAdvance((ImWchar)' ') * 0.50); // So we can see selected empty lines
                    ImRect rect(rect_pos + DimgVec2D::new(0.0, bg_offy_up - g.FontSize), rect_pos + DimgVec2D::new(rect_size.x, bg_offy_dn));
                    rect.ClipWith(clip_rect);
                    if (rect.Overlaps(clip_rect))
                        draw_window.draw_list->AddRectFilled(rect.Min, rect.Max, bg_color);
                }
                rect_pos.x = draw_pos.x - draw_scroll.x;
                rect_pos.y += g.FontSize;
            }
        }

        // We test for 'buf_display_max_length' as a way to avoid some pathological cases (e.g. single-line 1 MB string) which would make ImDrawList crash.
        if (is_multiline || (buf_display_end - buf_display) < buf_display_max_length)
        {
            ImU32 col = GetColorU32(is_displaying_hint ? ImGuiCol_TextDisabled : ImGuiCol_Text);
            draw_window.draw_list->AddText(g.Font, g.FontSize, draw_pos - draw_scroll, col, buf_display, buf_display_end, 0.0, is_multiline ? None : &clip_rect);
        }

        // Draw blinking cursor
        if (render_cursor)
        {
            state->CursorAnim += io.DeltaTime;
            bool cursor_is_visible = (!g.IO.ConfigInputTextCursorBlink) || (state->CursorAnim <= 0.0) || ImFmod(state->CursorAnim, 1.20) <= 0.80;
            Vector2D cursor_screen_pos = ImFloor(draw_pos + cursor_offset - draw_scroll);
            ImRect cursor_screen_rect(cursor_screen_pos.x, cursor_screen_pos.y - g.FontSize + 0.5, cursor_screen_pos.x + 1.0, cursor_screen_pos.y - 1.5);
            if (cursor_is_visible && cursor_screen_rect.Overlaps(clip_rect))
                draw_window.draw_list->AddLine(cursor_screen_rect.Min, cursor_screen_rect.GetBL(), GetColorU32(ImGuiCol_Text));

            // Notify OS of text input position for advanced IME (-1 x offset so that windows IME can cover our cursor. Bit of an extra nicety.)
            if (!is_readonly)
            {
                g.PlatformImeData.WantVisible = true;
                g.PlatformImeData.InputPos = DimgVec2D::new(cursor_screen_pos.x - 1.0, cursor_screen_pos.y - g.FontSize);
                g.PlatformImeData.InputLineHeight = g.FontSize;
                g.PlatformImeViewport = window.viewport->ID;
            }
        }
    }
    else
    {
        // Render text only (no selection, no cursor)
        if (is_multiline)
            text_size = DimgVec2D::new(inner_size.x, InputTextCalcTextLenAndLineCount(buf_display, &buf_display_end) * g.FontSize); // We don't need width
        else if (!is_displaying_hint && g.ActiveId == id)
            buf_display_end = buf_display + state->CurLenA;
        else if (!is_displaying_hint)
            buf_display_end = buf_display + strlen(buf_display);

        if (is_multiline || (buf_display_end - buf_display) < buf_display_max_length)
        {
            ImU32 col = GetColorU32(is_displaying_hint ? ImGuiCol_TextDisabled : ImGuiCol_Text);
            draw_window.draw_list->AddText(g.Font, g.FontSize, draw_pos, col, buf_display, buf_display_end, 0.0, is_multiline ? None : &clip_rect);
        }
    }

    if (is_password && !is_displaying_hint)
        PopFont();

    if (is_multiline)
    {
        // For focus requests to work on our multiline we need to ensure our child ItemAdd() call specifies the ImGuiItemFlags_Inputable (ref issue #4761)...
        Dummy(DimgVec2D::new(text_size.x, text_size.y + style.FramePadding.y));
        ImGuiItemFlags backup_item_flags = g.CurrentItemFlags;
        g.CurrentItemFlags |= ItemFlags::Inputable | ItemFlags::NoTabStop;
        EndChild();
        item_data_backup.StatusFlags |= (g.last_item_data.StatusFlags & ItemStatusFlags::HoveredWindow);
        g.CurrentItemFlags = backup_item_flags;

        // ...and then we need to undo the group overriding last item data, which gets a bit messy as EndGroup() tries to forward scrollbar being active...
        // FIXME: This quite messy/tricky, should attempt to get rid of the child window.
        EndGroup();
        if (g.last_item_data.id == 0)
        {
            g.last_item_data.id = id;
            g.last_item_data.in_flags = item_data_backup.in_flags;
            g.last_item_data.StatusFlags = item_data_backup.StatusFlags;
        }
    }

    // Log as text
    if (g.log_enabled && (!is_password || is_displaying_hint))
    {
        LogSetNextTextDecoration("{", "}");
        LogRenderedText(&draw_pos, buf_display, buf_display_end);
    }

    if (label_size.x > 0)
        render_text(DimgVec2D::new(frame_bb.Max.x + style.ItemInnerSpacing.x, frame_bb.Min.y + style.FramePadding.y), label);

    if (value_changed && !(flags & ImGuiInputTextFlags_NoMarkEdited))
        MarkItemEdited(id);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.last_item_data.StatusFlags);
    if ((flags & ImGuiInputTextFlags_EnterReturnsTrue) != 0)
        return enter_pressed;
    else
        return value_changed;
}

void ImGui::DebugNodeInputTextState(ImGuiInputTextState* state)
{
#ifndef IMGUI_DISABLE_DEBUG_TOOLS
    // ImGuiContext& g = *GImGui;
    ImStb::STB_TexteditState* stb_state = &state->Stb;
    ImStb::StbUndoState* undo_state = &stb_state->undostate;
    text("id: 0x%08X, ActiveID: 0x%08X", state->ID, g.ActiveId);
    text("CurLenW: %d, CurLenA: %d, Cursor: %d, Selection: %d..%d", state->CurLenA, state->CurLenW, stb_state->cursor, stb_state->select_start, stb_state->select_end);
    text("undo_point: %d, redo_point: %d, undo_char_point: %d, redo_char_point: %d", undo_state->undo_point, undo_state->redo_point, undo_state->undo_char_point, undo_state->redo_char_point);
    if (BeginChild("undopoints", DimgVec2D::new(0.0, GetTextLineHeight() * 15), true)) // Visualize undo state
    {
        PushStyleVar(ImGuiStyleVar_ItemSpacing, DimgVec2D::new(0, 0));
        for (int n = 0; n < STB_TEXTEDIT_UNDOSTATECOUNT; n += 1)
        {
            ImStb::StbUndoRecord* undo_rec = &undo_state->undo_rec[n];
            const char undo_rec_type = (n < undo_state->undo_point) ? 'u' : (n >= undo_state->redo_point) ? 'r' : ' ';
            if (undo_rec_type == ' ')
                BeginDisabled();
            char buf[64] = "";
            if (undo_rec_type != ' ' && undo_rec->char_storage != -1)
                ImTextStrToUtf8(buf, IM_ARRAYSIZE(buf), undo_state->undo_char + undo_rec->char_storage, undo_state->undo_char + undo_rec->char_storage + undo_rec->insert_length);
            text("%c [%02d] where %03d, insert %03d, delete %03d, char_storage %03d \"%s\"",
                undo_rec_type, n, undo_rec->where, undo_rec->insert_length, undo_rec->delete_length, undo_rec->char_storage, buf);
            if (undo_rec_type == ' ')
                EndDisabled();
        }
        PopStyleVar();
    }
    EndChild();
#else
    IM_UNUSED(state);
#endif
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

bool ImGui::ColorEdit3(const char* label, float col[3], ImGuiColorEditFlags flags)
{
    return ColorEdit4(label, col, flags | ImGuiColorEditFlags_NoAlpha);
}

// ColorEdit supports RGB and HSV inputs. In case of RGB input resulting color may have undefined hue and/or saturation.
// Since widget displays both RGB and HSV values we must preserve hue and saturation to prevent these values resetting.
static void ColorEditRestoreHS(let* col, float* H, float* S, float* V)
{
    // This check is optional. Suppose we have two color widgets side by side, both widgets display different colors, but both colors have hue and/or saturation undefined.
    // With color check: hue/saturation is preserved in one widget. Editing color in one widget would reset hue/saturation in another one.
    // Without color check: common hue/saturation would be displayed in all widgets that have hue/saturation undefined.
    // g.color_edit_last_color is stored as ImU32 RGB value: this essentially gives us color equality check with reduced precision.
    // Tiny external color changes would not be detected and this check would still pass. This is OK, since we only restore hue/saturation _only_ if they are undefined,
    // therefore this change flipping hue/saturation from undefined to a very tiny value would still be represented in color picker.
    // ImGuiContext& g = *GImGui;
    if (g.ColorEditLastColor != ImGui::color_convert_float4_to_u32(Vector4D(col[0], col[1], col[2], 0)))
        return;

    // When S == 0, H is undefined.
    // When H == 1 it wraps around to 0.
    if (*S == 0.0 || (*H == 0.0 && g.ColorEditLastHue == 1))
        *H = g.ColorEditLastHue;

    // When V == 0, S is undefined.
    if (*V == 0.0)
        *S = g.ColorEditLastSat;
}

// Edit colors components (each component in 0.0..1.0 range).
// See enum ImGuiColorEditFlags_ for available options. e.g. Only access 3 floats if ImGuiColorEditFlags_NoAlpha flag is set.
// With typical options: Left-click on color square to open color picker. Right-click to open option menu. CTRL-Click over input fields to edit them and TAB to go to next item.
bool ImGui::ColorEdit4(const char* label, float col[4], ImGuiColorEditFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;
    let square_sz = get_frame_height();
    let w_full = CalcItemWidth();
    let w_button = (flags & ImGuiColorEditFlags_NoSmallPreview) ? 0.0 : (square_sz + style.ItemInnerSpacing.x);
    let w_inputs = w_full - w_button;
    const char* label_display_end = FindRenderedTextEnd(label);
    g.next_item_data.ClearFlags();

    BeginGroup();
    push_id(label);

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
    IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_DisplayMask_)); // Check that only 1 is selected
    IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_InputMask_));   // Check that only 1 is selected

    const bool alpha = (flags & ImGuiColorEditFlags_NoAlpha) == 0;
    const bool hdr = (flags & ImGuiColorEditFlags_HDR) != 0;
    let components = alpha ? 4 : 3;

    // Convert to the formats we need
    float f[4] = { col[0], col[1], col[2], alpha ? col[3] : 1.0 };
    if ((flags & ImGuiColorEditFlags_InputHSV) && (flags & ImGuiColorEditFlags_DisplayRGB))
        ColorConvertHSVtoRGB(f[0], f[1], f[2], f[0], f[1], f[2]);
    else if ((flags & ImGuiColorEditFlags_InputRGB) && (flags & ImGuiColorEditFlags_DisplayHSV))
    {
        // Hue is lost when converting from greyscale rgb (saturation=0). Restore it.
        ColorConvertRGBtoHSV(f[0], f[1], f[2], f[0], f[1], f[2]);
        ColorEditRestoreHS(col, &f[0], &f[1], &f[2]);
    }
    int i[4] = { IM_F32_TO_INT8_UNBOUND(f[0]), IM_F32_TO_INT8_UNBOUND(f[1]), IM_F32_TO_INT8_UNBOUND(f[2]), IM_F32_TO_INT8_UNBOUND(f[3]) };

    bool value_changed = false;
    bool value_changed_as_float = false;

    const Vector2D pos = window.DC.CursorPos;
    let inputs_offset_x = (style.ColorButtonPosition == ImGuiDir_Left) ? w_button : 0.0;
    window.DC.CursorPos.x = pos.x + inputs_offset_x;

    if ((flags & (ImGuiColorEditFlags_DisplayRGB | ImGuiColorEditFlags_DisplayHSV)) != 0 && (flags & ImGuiColorEditFlags_NoInputs) == 0)
    {
        // RGB/HSV 0..255 Sliders
        let w_item_one  = ImMax(1.0, IM_FLOOR((w_inputs - (style.ItemInnerSpacing.x) * (components - 1)) / components));
        let w_item_last = ImMax(1.0, IM_FLOOR(w_inputs - (w_item_one + style.ItemInnerSpacing.x) * (components - 1)));

        const bool hide_prefix = (w_item_one <= CalcTextSize((flags & ImGuiColorEditFlags_Float) ? "M:0.000" : "M:000").x);
        static const char* ids[4] = { "##x", "##Y", "##Z", "##W" };
        static const char* fmt_table_int[3][4] =
        {
            {   "%3d",   "%3d",   "%3d",   "%3d" }, // Short display
            { "R:%3d", "G:%3d", "B:%3d", "A:%3d" }, // Long display for RGBA
            { "H:%3d", "S:%3d", "V:%3d", "A:%3d" }  // Long display for HSVA
        };
        static const char* fmt_table_float[3][4] =
        {
            {   "%0.3",   "%0.3",   "%0.3",   "%0.3" }, // Short display
            { "R:%0.3", "G:%0.3", "B:%0.3", "A:%0.3" }, // Long display for RGBA
            { "H:%0.3", "S:%0.3", "V:%0.3", "A:%0.3" }  // Long display for HSVA
        };
        let fmt_idx = hide_prefix ? 0 : (flags & ImGuiColorEditFlags_DisplayHSV) ? 2 : 1;

        for (int n = 0; n < components; n += 1)
        {
            if (n > 0)
                same_line(0, style.ItemInnerSpacing.x);
            SetNextItemWidth((n + 1 < components) ? w_item_one : w_item_last);

            // FIXME: When ImGuiColorEditFlags_HDR flag is passed HS values snap in weird ways when SV values go below 0.
            if (flags & ImGuiColorEditFlags_Float)
            {
                value_changed |= DragFloat(ids[n], &f[n], 1.0 / 255.0, 0.0, hdr ? 0.0 : 1.0, fmt_table_float[fmt_idx][n]);
                value_changed_as_float |= value_changed;
            }
            else
            {
                value_changed |= DragInt(ids[n], &i[n], 1.0, 0, hdr ? 0 : 255, fmt_table_int[fmt_idx][n]);
            }
            if (!(flags & ImGuiColorEditFlags_NoOptions))
                open_popupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);
        }
    }
    else if ((flags & ImGuiColorEditFlags_DisplayHex) != 0 && (flags & ImGuiColorEditFlags_NoInputs) == 0)
    {
        // RGB Hexadecimal Input
        char buf[64];
        if (alpha)
            ImFormatString(buf, IM_ARRAYSIZE(buf), "#%02X%02X%02X%02X", ImClamp(i[0], 0, 255), ImClamp(i[1], 0, 255), ImClamp(i[2], 0, 255), ImClamp(i[3], 0, 255));
        else
            ImFormatString(buf, IM_ARRAYSIZE(buf), "#%02X%02X%02X", ImClamp(i[0], 0, 255), ImClamp(i[1], 0, 255), ImClamp(i[2], 0, 255));
        SetNextItemWidth(w_inputs);
        if (InputText("##Text", buf, IM_ARRAYSIZE(buf), ImGuiInputTextFlags_CharsHexadecimal | ImGuiInputTextFlags_CharsUppercase))
        {
            value_changed = true;
            char* p = buf;
            while (*p == '#' || ImCharIsBlankA(*p))
                p += 1;
            i[0] = i[1] = i[2] = 0;
            i[3] = 0xFF; // alpha default to 255 is not parsed by scanf (e.g. inputting #FFFFFF omitting alpha)
            int r;
            if (alpha)
                r = sscanf(p, "%02X%02X%02X%02X", (unsigned int*)&i[0], (unsigned int*)&i[1], (unsigned int*)&i[2], (unsigned int*)&i[3]); // Treat at unsigned (%x is unsigned)
            else
                r = sscanf(p, "%02X%02X%02X", (unsigned int*)&i[0], (unsigned int*)&i[1], (unsigned int*)&i[2]);
            IM_UNUSED(r); // Fixes C6031: Return value ignored: 'sscanf'.
        }
        if (!(flags & ImGuiColorEditFlags_NoOptions))
            open_popupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);
    }

    Window* picker_active_window = None;
    if (!(flags & ImGuiColorEditFlags_NoSmallPreview))
    {
        let button_offset_x = ((flags & ImGuiColorEditFlags_NoInputs) || (style.ColorButtonPosition == ImGuiDir_Left)) ? 0.0 : w_inputs + style.ItemInnerSpacing.x;
        window.DC.CursorPos = DimgVec2D::new(pos.x + button_offset_x, pos.y);

        const Vector4D col_v4(col[0], col[1], col[2], alpha ? col[3] : 1.0);
        if (ColorButton("##ColorButton", col_v4, flags))
        {
            if (!(flags & ImGuiColorEditFlags_NoPicker))
            {
                // Store current color and open a picker
                g.ColorPickerRef = col_v4;
                open_popup("picker");
                set_next_window_pos(g.last_item_data.Rect.GetBL() + DimgVec2D::new(0.0, style.item_spacing.y));
            }
        }
        if (!(flags & ImGuiColorEditFlags_NoOptions))
            open_popupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);

        if (begin_popup("picker"))
        {
            picker_active_window = g.current_window_id;
            if (label != label_display_end)
            {
                TextEx(label, label_display_end);
                Spacing();
            }
            ImGuiColorEditFlags picker_flags_to_forward = ImGuiColorEditFlags_DataTypeMask_ | ImGuiColorEditFlags_PickerMask_ | ImGuiColorEditFlags_InputMask_ | ImGuiColorEditFlags_HDR | ImGuiColorEditFlags_NoAlpha | ImGuiColorEditFlags_AlphaBar;
            ImGuiColorEditFlags picker_flags = (flags_untouched & picker_flags_to_forward) | ImGuiColorEditFlags_DisplayMask_ | ImGuiColorEditFlags_NoLabel | ImGuiColorEditFlags_AlphaPreviewHalf;
            SetNextItemWidth(square_sz * 12.0); // Use 256 + bar sizes?
            value_changed |= ColorPicker4("##picker", col, picker_flags, &g.ColorPickerRef.x);
            end_popup();
        }
    }

    if (label != label_display_end && !(flags & ImGuiColorEditFlags_NoLabel))
    {
        same_line(0.0, style.ItemInnerSpacing.x);
        TextEx(label, label_display_end);
    }

    // Convert back
    if (value_changed && picker_active_window == None)
    {
        if (!value_changed_as_float)
            for (int n = 0; n < 4; n += 1)
                f[n] = i[n] / 255.0;
        if ((flags & ImGuiColorEditFlags_DisplayHSV) && (flags & ImGuiColorEditFlags_InputRGB))
        {
            g.ColorEditLastHue = f[0];
            g.ColorEditLastSat = f[1];
            ColorConvertHSVtoRGB(f[0], f[1], f[2], f[0], f[1], f[2]);
            g.ColorEditLastColor = color_convert_float4_to_u32(Vector4D(f[0], f[1], f[2], 0));
        }
        if ((flags & ImGuiColorEditFlags_DisplayRGB) && (flags & ImGuiColorEditFlags_InputHSV))
            ColorConvertRGBtoHSV(f[0], f[1], f[2], f[0], f[1], f[2]);

        col[0] = f[0];
        col[1] = f[1];
        col[2] = f[2];
        if (alpha)
            col[3] = f[3];
    }

    pop_id();
    EndGroup();

    // Drag and Drop Target
    // NB: The flag test is merely an optional micro-optimization, BeginDragDropTarget() does the same test.
    if ((g.last_item_data.StatusFlags & ItemStatusFlags::HoveredRect) && !(flags & ImGuiColorEditFlags_NoDragDrop) && BeginDragDropTarget())
    {
        bool accepted_drag_drop = false;
        if (const ImGuiPayload* payload = accept_drag_drop_payload(IMGUI_PAYLOAD_TYPE_COLOR_3F))
        {
            memcpy((float*)col, payload->Data, sizeof * 3); // Preserve alpha if any //-V512
            value_changed = accepted_drag_drop = true;
        }
        if (const ImGuiPayload* payload = accept_drag_drop_payload(IMGUI_PAYLOAD_TYPE_COLOR_4F))
        {
            memcpy((float*)col, payload->Data, sizeof * components);
            value_changed = accepted_drag_drop = true;
        }

        // Drag-drop payloads are always RGB
        if (accepted_drag_drop && (flags & ImGuiColorEditFlags_InputHSV))
            ColorConvertRGBtoHSV(col[0], col[1], col[2], col[0], col[1], col[2]);
        end_drag_drop_target();
    }

    // When picker is being actively used, use its active id so IsItemActive() will function on ColorEdit4().
    if (picker_active_window && g.ActiveId != 0 && g.ActiveIdWindow == picker_active_window)
        g.last_item_data.id = g.ActiveId;

    if (value_changed)
        MarkItemEdited(g.last_item_data.id);

    return value_changed;
}

bool ImGui::ColorPicker3(const char* label, float col[3], ImGuiColorEditFlags flags)
{
    float col4[4] = { col[0], col[1], col[2], 1.0 };
    if (!ColorPicker4(label, col4, flags | ImGuiColorEditFlags_NoAlpha))
        return false;
    col[0] = col4[0]; col[1] = col4[1]; col[2] = col4[2];
    return true;
}

// Helper for ColorPicker4()
static void RenderArrowsForVerticalBar(ImDrawList* draw_list, Vector2D pos, Vector2D half_sz, float bar_w, float alpha)
{
    ImU32 alpha8 = IM_F32_TO_INT8_SAT(alpha);
    ImGui::RenderArrowPointingAt(draw_list, DimgVec2D::new(pos.x + half_sz.x + 1,         pos.y), DimgVec2D::new(half_sz.x + 2, half_sz.y + 1), ImGuiDir_Right, IM_COL32(0,0,0,alpha8));
    ImGui::RenderArrowPointingAt(draw_list, DimgVec2D::new(pos.x + half_sz.x,             pos.y), half_sz,                              ImGuiDir_Right, IM_COL32(255,255,255,alpha8));
    ImGui::RenderArrowPointingAt(draw_list, DimgVec2D::new(pos.x + bar_w - half_sz.x - 1, pos.y), DimgVec2D::new(half_sz.x + 2, half_sz.y + 1), ImGuiDir_Left,  IM_COL32(0,0,0,alpha8));
    ImGui::RenderArrowPointingAt(draw_list, DimgVec2D::new(pos.x + bar_w - half_sz.x,     pos.y), half_sz,                              ImGuiDir_Left,  IM_COL32(255,255,255,alpha8));
}

// Note: ColorPicker4() only accesses 3 floats if ImGuiColorEditFlags_NoAlpha flag is set.
// (In C++ the 'float col[4]' notation for a function argument is equivalent to 'float* col', we only specify a size to facilitate understanding of the code.)
// FIXME: we adjust the big color square height based on item width, which may cause a flickering feedback loop (if automatic height makes a vertical scrollbar appears, affecting automatic width..)
// FIXME: this is trying to be aware of style.Alpha but not fully correct. Also, the color wheel will have overlapping glitches with (style.Alpha < 1.0)
bool ImGui::ColorPicker4(const char* label, float col[4], ImGuiColorEditFlags flags, let* ref_col)
{
    // ImGuiContext& g = *GImGui;
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    ImDrawList* draw_list = window.draw_list;
    ImGuiStyle& style = g.Style;
    ImGuiIO& io = g.IO;

    let width = CalcItemWidth();
    g.next_item_data.ClearFlags();

    push_id(label);
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
    IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_PickerMask_)); // Check that only 1 is selected
    IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_InputMask_));  // Check that only 1 is selected
    if (!(flags & ImGuiColorEditFlags_NoOptions))
        flags |= (g.ColorEditOptions & ImGuiColorEditFlags_AlphaBar);

    // Setup
    int components = (flags & ImGuiColorEditFlags_NoAlpha) ? 3 : 4;
    bool alpha_bar = (flags & ImGuiColorEditFlags_AlphaBar) && !(flags & ImGuiColorEditFlags_NoAlpha);
    Vector2D picker_pos = window.DC.CursorPos;
    let square_sz =  get_frame_height();
    let bars_width =  square_sz; // Arbitrary smallish width of Hue/Alpha picking bars
    let sv_picker_size =  ImMax(bars_width * 1, width - (alpha_bar ? 2 : 1) * (bars_width + style.ItemInnerSpacing.x)); // Saturation/value picking box
    let bar0_pos_x =  picker_pos.x + sv_picker_size + style.ItemInnerSpacing.x;
    let bar1_pos_x =  bar0_pos_x + bars_width + style.ItemInnerSpacing.x;
    let bars_triangles_half_sz =  IM_FLOOR(bars_width * 0.20);

    float backup_initial_col[4];
    memcpy(backup_initial_col, col, components * sizeof);

    let wheel_thickness =  sv_picker_size * 0.08;
    let wheel_r_outer =  sv_picker_size * 0.50;
    let wheel_r_inner =  wheel_r_outer - wheel_thickness;
    Vector2D wheel_center(picker_pos.x + (sv_picker_size + bars_width)*0.5, picker_pos.y + sv_picker_size * 0.5);

    // Note: the triangle is displayed rotated with triangle_pa pointing to Hue, but most coordinates stays unrotated for logic.
    let triangle_r =  wheel_r_inner - (sv_picker_size * 0.027);
    Vector2D triangle_pa = DimgVec2D::new(triangle_r, 0.0); // Hue point.
    Vector2D triangle_pb = DimgVec2D::new(triangle_r * -0.5, triangle_r * -0.866025); // Black point.
    Vector2D triangle_pc = DimgVec2D::new(triangle_r * -0.5, triangle_r * +0.866025); // White point.

    let H =  col[0], S = col[1], V = col[2];
    let R =  col[0], G = col[1], B = col[2];
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

    bool value_changed = false, value_changed_h = false, value_changed_sv = false;

    push_item_flag(ItemFlags::NoNav, true);
    if (flags & ImGuiColorEditFlags_PickerHueWheel)
    {
        // Hue wheel + SV triangle logic
        InvisibleButton("hsv", DimgVec2D::new(sv_picker_size + style.ItemInnerSpacing.x + bars_width, sv_picker_size));
        if (is_item_active())
        {
            Vector2D initial_off = g.IO.MouseClickedPos[0] - wheel_center;
            Vector2D current_off = g.IO.MousePos - wheel_center;
            let initial_dist2 =  ImLengthSqr(initial_off);
            if (initial_dist2 >= (wheel_r_inner - 1) * (wheel_r_inner - 1) && initial_dist2 <= (wheel_r_outer + 1) * (wheel_r_outer + 1))
            {
                // Interactive with Hue wheel
                H = ImAtan2(current_off.y, current_off.x) / IM_PI * 0.5;
                if (H < 0.0)
                    H += 1.0;
                value_changed = value_changed_h = true;
            }
            let cos_hue_angle =  ImCos(-H * 2.0 * IM_PI);
            let sin_hue_angle =  ImSin(-H * 2.0 * IM_PI);
            if (ImTriangleContainsPoint(triangle_pa, triangle_pb, triangle_pc, ImRotate(initial_off, cos_hue_angle, sin_hue_angle)))
            {
                // Interacting with SV triangle
                Vector2D current_off_unrotated = ImRotate(current_off, cos_hue_angle, sin_hue_angle);
                if (!ImTriangleContainsPoint(triangle_pa, triangle_pb, triangle_pc, current_off_unrotated))
                    current_off_unrotated = ImTriangleClosestPoint(triangle_pa, triangle_pb, triangle_pc, current_off_unrotated);
                float uu, vv, ww;
                ImTriangleBarycentricCoords(triangle_pa, triangle_pb, triangle_pc, current_off_unrotated, uu, vv, ww);
                V = ImClamp(1.0 - vv, 0.0001, 1.0);
                S = ImClamp(uu / V, 0.0001, 1.0);
                value_changed = value_changed_sv = true;
            }
        }
        if (!(flags & ImGuiColorEditFlags_NoOptions))
            open_popupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);
    }
    else if (flags & ImGuiColorEditFlags_PickerHueBar)
    {
        // SV rectangle logic
        InvisibleButton("sv", DimgVec2D::new(sv_picker_size, sv_picker_size));
        if (is_item_active())
        {
            S = ImSaturate((io.MousePos.x - picker_pos.x) / (sv_picker_size - 1));
            V = 1.0 - ImSaturate((io.MousePos.y - picker_pos.y) / (sv_picker_size - 1));

            // Greatly reduces hue jitter and reset to 0 when hue == 255 and color is rapidly modified using SV square.
            if (g.ColorEditLastColor == color_convert_float4_to_u32(Vector4D(col[0], col[1], col[2], 0)))
                H = g.ColorEditLastHue;
            value_changed = value_changed_sv = true;
        }
        if (!(flags & ImGuiColorEditFlags_NoOptions))
            open_popupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);

        // Hue bar logic
        SetCursorScreenPos(DimgVec2D::new(bar0_pos_x, picker_pos.y));
        InvisibleButton("hue", DimgVec2D::new(bars_width, sv_picker_size));
        if (is_item_active())
        {
            H = ImSaturate((io.MousePos.y - picker_pos.y) / (sv_picker_size - 1));
            value_changed = value_changed_h = true;
        }
    }

    // Alpha bar logic
    if (alpha_bar)
    {
        SetCursorScreenPos(DimgVec2D::new(bar1_pos_x, picker_pos.y));
        InvisibleButton("alpha", DimgVec2D::new(bars_width, sv_picker_size));
        if (is_item_active())
        {
            col[3] = 1.0 - ImSaturate((io.MousePos.y - picker_pos.y) / (sv_picker_size - 1));
            value_changed = true;
        }
    }
    pop_item_flag(); // ImGuiItemFlags_NoNav

    if (!(flags & ImGuiColorEditFlags_NoSidePreview))
    {
        same_line(0, style.ItemInnerSpacing.x);
        BeginGroup();
    }

    if (!(flags & ImGuiColorEditFlags_NoLabel))
    {
        const char* label_display_end = FindRenderedTextEnd(label);
        if (label != label_display_end)
        {
            if ((flags & ImGuiColorEditFlags_NoSidePreview))
                same_line(0, style.ItemInnerSpacing.x);
            TextEx(label, label_display_end);
        }
    }

    if (!(flags & ImGuiColorEditFlags_NoSidePreview))
    {
        push_item_flag(ItemFlags::NoNavDefaultFocus, true);
        Vector4D col_v4(col[0], col[1], col[2], (flags & ImGuiColorEditFlags_NoAlpha) ? 1.0 : col[3]);
        if ((flags & ImGuiColorEditFlags_NoLabel))
            text("current");

        ImGuiColorEditFlags sub_flags_to_forward = ImGuiColorEditFlags_InputMask_ | ImGuiColorEditFlags_HDR | ImGuiColorEditFlags_AlphaPreview | ImGuiColorEditFlags_AlphaPreviewHalf | ImGuiColorEditFlags_NoTooltip;
        ColorButton("##current", col_v4, (flags & sub_flags_to_forward), DimgVec2D::new(square_sz * 3, square_sz * 2));
        if (ref_col != None)
        {
            text("Original");
            Vector4D ref_col_v4(ref_col[0], ref_col[1], ref_col[2], (flags & ImGuiColorEditFlags_NoAlpha) ? 1.0 : ref_col[3]);
            if (ColorButton("##original", ref_col_v4, (flags & sub_flags_to_forward), DimgVec2D::new(square_sz * 3, square_sz * 2)))
            {
                memcpy(col, ref_col, components * sizeof);
                value_changed = true;
            }
        }
        pop_item_flag();
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
            g.ColorEditLastColor = color_convert_float4_to_u32(Vector4D(col[0], col[1], col[2], 0));
        }
        else if (flags & ImGuiColorEditFlags_InputHSV)
        {
            col[0] = H;
            col[1] = S;
            col[2] = V;
        }
    }

    // R,G,B and H,S,V slider color editor
    bool value_changed_fix_hue_wrap = false;
    if ((flags & ImGuiColorEditFlags_NoInputs) == 0)
    {
        PushItemWidth((alpha_bar ? bar1_pos_x : bar0_pos_x) + bars_width - picker_pos.x);
        ImGuiColorEditFlags sub_flags_to_forward = ImGuiColorEditFlags_DataTypeMask_ | ImGuiColorEditFlags_InputMask_ | ImGuiColorEditFlags_HDR | ImGuiColorEditFlags_NoAlpha | ImGuiColorEditFlags_NoOptions | ImGuiColorEditFlags_NoSmallPreview | ImGuiColorEditFlags_AlphaPreview | ImGuiColorEditFlags_AlphaPreviewHalf;
        ImGuiColorEditFlags sub_flags = (flags & sub_flags_to_forward) | ImGuiColorEditFlags_NoPicker;
        if (flags & ImGuiColorEditFlags_DisplayRGB || (flags & ImGuiColorEditFlags_DisplayMask_) == 0)
            if (ColorEdit4("##rgb", col, sub_flags | ImGuiColorEditFlags_DisplayRGB))
            {
                // FIXME: Hackily differentiating using the DragInt (active_id != 0 && !active_id_allow_overlap) vs. using the InputText or DropTarget.
                // For the later we don't want to run the hue-wrap canceling code. If you are well versed in HSV picker please provide your input! (See #2050)
                value_changed_fix_hue_wrap = (g.ActiveId != 0 && !g.active_id_allow_overlap);
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
        float new_H, new_S, new_V;
        ColorConvertRGBtoHSV(col[0], col[1], col[2], new_H, new_S, new_V);
        if (new_H <= 0 && H > 0)
        {
            if (new_V <= 0 && V != new_V)
                ColorConvertHSVtoRGB(H, S, new_V <= 0 ? V * 0.5 : new_V, col[0], col[1], col[2]);
            else if (new_S <= 0)
                ColorConvertHSVtoRGB(H, new_S <= 0 ? S * 0.5 : new_S, new_V, col[0], col[1], col[2]);
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

    let style_alpha8 = IM_F32_TO_INT8_SAT(style.Alpha);
    const ImU32 col_black = IM_COL32(0,0,0,style_alpha8);
    const ImU32 col_white = IM_COL32(255,255,255,style_alpha8);
    const ImU32 col_midgrey = IM_COL32(128,128,128,style_alpha8);
    const ImU32 col_hues[6 + 1] = { IM_COL32(255,0,0,style_alpha8), IM_COL32(255,255,0,style_alpha8), IM_COL32(0,255,0,style_alpha8), IM_COL32(0,255,255,style_alpha8), IM_COL32(0,0,255,style_alpha8), IM_COL32(255,0,255,style_alpha8), IM_COL32(255,0,0,style_alpha8) };

    Vector4D hue_color_f(1, 1, 1, style.Alpha); ColorConvertHSVtoRGB(H, 1, 1, hue_color_f.x, hue_color_f.y, hue_color_f.z);
    ImU32 hue_color32 = color_convert_float4_to_u32(hue_color_f);
    ImU32 user_col32_striped_of_alpha = color_convert_float4_to_u32(Vector4D(R, G, B, style.Alpha)); // Important: this is still including the main rendering/style alpha!!

    Vector2D sv_cursor_pos;

    if (flags & ImGuiColorEditFlags_PickerHueWheel)
    {
        // Render Hue Wheel
        let aeps = 0.5 / wheel_r_outer; // Half a pixel arc length in radians (2pi cancels out).
        let segment_per_arc = ImMax(4, wheel_r_outer / 12);
        for (int n = 0; n < 6; n += 1)
        {
            let a0 = (n)     /6.0 * 2.0 * IM_PI - aeps;
            let a1 = (n+1.0)/6.0 * 2.0 * IM_PI + aeps;
            let vert_start_idx = draw_list->vtx_buffer.Size;
            draw_list->path_arc_to(wheel_center, (wheel_r_inner + wheel_r_outer)*0.5, a0, a1, segment_per_arc);
            draw_list->path_stroke(col_white, 0, wheel_thickness);
            let vert_end_idx = draw_list->vtx_buffer.Size;

            // Paint colors over existing vertices
            Vector2D gradient_p0(wheel_center.x + ImCos(a0) * wheel_r_inner, wheel_center.y + ImSin(a0) * wheel_r_inner);
            Vector2D gradient_p1(wheel_center.x + ImCos(a1) * wheel_r_inner, wheel_center.y + ImSin(a1) * wheel_r_inner);
            ShadeVertsLinearColorGradientKeepAlpha(draw_list, vert_start_idx, vert_end_idx, gradient_p0, gradient_p1, col_hues[n], col_hues[n + 1]);
        }

        // Render Cursor + preview on Hue Wheel
        let cos_hue_angle =  ImCos(H * 2.0 * IM_PI);
        let sin_hue_angle =  ImSin(H * 2.0 * IM_PI);
        Vector2D hue_cursor_pos(wheel_center.x + cos_hue_angle * (wheel_r_inner + wheel_r_outer) * 0.5, wheel_center.y + sin_hue_angle * (wheel_r_inner + wheel_r_outer) * 0.5);
        let hue_cursor_rad =  value_changed_h ? wheel_thickness * 0.65 : wheel_thickness * 0.55;
        int hue_cursor_segments = ImClamp((hue_cursor_rad / 1.4), 9, 32);
        draw_list->AddCircleFilled(hue_cursor_pos, hue_cursor_rad, hue_color32, hue_cursor_segments);
        draw_list->AddCircle(hue_cursor_pos, hue_cursor_rad + 1, col_midgrey, hue_cursor_segments);
        draw_list->AddCircle(hue_cursor_pos, hue_cursor_rad, col_white, hue_cursor_segments);

        // Render SV triangle (rotated according to hue)
        Vector2D tra = wheel_center + ImRotate(triangle_pa, cos_hue_angle, sin_hue_angle);
        Vector2D trb = wheel_center + ImRotate(triangle_pb, cos_hue_angle, sin_hue_angle);
        Vector2D trc = wheel_center + ImRotate(triangle_pc, cos_hue_angle, sin_hue_angle);
        Vector2D uv_white = GetFontTexUvWhitePixel();
        draw_list->prim_reserve(6, 6);
        draw_list->PrimVtx(tra, uv_white, hue_color32);
        draw_list->PrimVtx(trb, uv_white, hue_color32);
        draw_list->PrimVtx(trc, uv_white, col_white);
        draw_list->PrimVtx(tra, uv_white, 0);
        draw_list->PrimVtx(trb, uv_white, col_black);
        draw_list->PrimVtx(trc, uv_white, 0);
        draw_list->AddTriangle(tra, trb, trc, col_midgrey, 1.5);
        sv_cursor_pos = ImLerp(ImLerp(trc, tra, ImSaturate(S)), trb, ImSaturate(1 - V));
    }
    else if (flags & ImGuiColorEditFlags_PickerHueBar)
    {
        // Render SV Square
        draw_list->AddRectFilledMultiColor(picker_pos, picker_pos + DimgVec2D::new(sv_picker_size, sv_picker_size), col_white, hue_color32, hue_color32, col_white);
        draw_list->AddRectFilledMultiColor(picker_pos, picker_pos + DimgVec2D::new(sv_picker_size, sv_picker_size), 0, 0, col_black, col_black);
        RenderFrameBorder(picker_pos, picker_pos + DimgVec2D::new(sv_picker_size, sv_picker_size), 0.0);
        sv_cursor_pos.x = ImClamp(IM_ROUND(picker_pos.x + ImSaturate(S)     * sv_picker_size), picker_pos.x + 2, picker_pos.x + sv_picker_size - 2); // Sneakily prevent the circle to stick out too much
        sv_cursor_pos.y = ImClamp(IM_ROUND(picker_pos.y + ImSaturate(1 - V) * sv_picker_size), picker_pos.y + 2, picker_pos.y + sv_picker_size - 2);

        // Render Hue Bar
        for (int i = 0; i < 6;  += 1i)
            draw_list->AddRectFilledMultiColor(DimgVec2D::new(bar0_pos_x, picker_pos.y + i * (sv_picker_size / 6)), DimgVec2D::new(bar0_pos_x + bars_width, picker_pos.y + (i + 1) * (sv_picker_size / 6)), col_hues[i], col_hues[i], col_hues[i + 1], col_hues[i + 1]);
        let bar0_line_y =  IM_ROUND(picker_pos.y + H * sv_picker_size);
        RenderFrameBorder(DimgVec2D::new(bar0_pos_x, picker_pos.y), DimgVec2D::new(bar0_pos_x + bars_width, picker_pos.y + sv_picker_size), 0.0);
        RenderArrowsForVerticalBar(draw_list, DimgVec2D::new(bar0_pos_x - 1, bar0_line_y), DimgVec2D::new(bars_triangles_half_sz + 1, bars_triangles_half_sz), bars_width + 2.0, style.Alpha);
    }

    // Render cursor/preview circle (clamp S/V within 0..1 range because floating points colors may lead HSV values to be out of range)
    let sv_cursor_rad =  value_changed_sv ? 10.0 : 6.0;
    draw_list->AddCircleFilled(sv_cursor_pos, sv_cursor_rad, user_col32_striped_of_alpha, 12);
    draw_list->AddCircle(sv_cursor_pos, sv_cursor_rad + 1, col_midgrey, 12);
    draw_list->AddCircle(sv_cursor_pos, sv_cursor_rad, col_white, 12);

    // Render alpha bar
    if (alpha_bar)
    {
        let alpha =  ImSaturate(col[3]);
        ImRect bar1_bb(bar1_pos_x, picker_pos.y, bar1_pos_x + bars_width, picker_pos.y + sv_picker_size);
        RenderColorRectWithAlphaCheckerboard(draw_list, bar1_bb.Min, bar1_bb.Max, 0, bar1_bb.GetWidth() / 2.0, DimgVec2D::new(0.0, 0.0));
        draw_list->AddRectFilledMultiColor(bar1_bb.Min, bar1_bb.Max, user_col32_striped_of_alpha, user_col32_striped_of_alpha, user_col32_striped_of_alpha & ~COLOR32_A_MASK, user_col32_striped_of_alpha & ~COLOR32_A_MASK);
        let bar1_line_y =  IM_ROUND(picker_pos.y + (1.0 - alpha) * sv_picker_size);
        RenderFrameBorder(bar1_bb.Min, bar1_bb.Max, 0.0);
        RenderArrowsForVerticalBar(draw_list, DimgVec2D::new(bar1_pos_x - 1, bar1_line_y), DimgVec2D::new(bars_triangles_half_sz + 1, bars_triangles_half_sz), bars_width + 2.0, style.Alpha);
    }

    EndGroup();

    if (value_changed && memcmp(backup_initial_col, col, components * sizeof) == 0)
        value_changed = false;
    if (value_changed)
        MarkItemEdited(g.last_item_data.id);

    pop_id();

    return value_changed;
}

// A little color square. Return true when clicked.
// FIXME: May want to display/ignore the alpha component in the color display? Yet show it in the tooltip.
// 'desc_id' is not called 'label' because we don't display it next to the button, but only in the tooltip.
// Note that 'col' may be encoded in HSV if ImGuiColorEditFlags_InputHSV is set.
bool ImGui::ColorButton(const char* desc_id, const Vector4D& col, ImGuiColorEditFlags flags, const Vector2D& size_arg)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    const Id32 id = window.GetID(desc_id);
    let default_size = get_frame_height();
    const Vector2D size(size_arg.x == 0.0 ? default_size : size_arg.x, size_arg.y == 0.0 ? default_size : size_arg.y);
    const ImRect bb(window.DC.CursorPos, window.DC.CursorPos + size);
    ItemSize(bb, (size.y >= default_size) ? g.Style.FramePadding.y : 0.0);
    if (!ItemAdd(bb, id))
        return false;

    bool hovered, held;
    bool pressed = ButtonBehavior(bb, id, &hovered, &held);

    if (flags & ImGuiColorEditFlags_NoAlpha)
        flags &= ~(ImGuiColorEditFlags_AlphaPreview | ImGuiColorEditFlags_AlphaPreviewHalf);

    Vector4D col_rgb = col;
    if (flags & ImGuiColorEditFlags_InputHSV)
        ColorConvertHSVtoRGB(col_rgb.x, col_rgb.y, col_rgb.z, col_rgb.x, col_rgb.y, col_rgb.z);

    Vector4D col_rgb_without_alpha(col_rgb.x, col_rgb.y, col_rgb.z, 1.0);
    let grid_step =  ImMin(size.x, size.y) / 2.99;
    let rounding =  ImMin(g.Style.frame_rounding, grid_step * 0.5);
    ImRect bb_inner = bb;
    let off =  0.0;
    if ((flags & ImGuiColorEditFlags_NoBorder) == 0)
    {
        off = -0.75; // The border (using Col_FrameBg) tends to look off when color is near-opaque and rounding is enabled. This offset seemed like a good middle ground to reduce those artifacts.
        bb_inner.Expand(off);
    }
    if ((flags & ImGuiColorEditFlags_AlphaPreviewHalf) && col_rgb.w < 1.0)
    {
        let mid_x =  IM_ROUND((bb_inner.Min.x + bb_inner.Max.x) * 0.5);
        RenderColorRectWithAlphaCheckerboard(window.draw_list, DimgVec2D::new(bb_inner.Min.x + grid_step, bb_inner.Min.y), bb_inner.Max, GetColorU32(col_rgb), grid_step, DimgVec2D::new(-grid_step + off, off), rounding, ImDrawFlags_RoundCornersRight);
        window.draw_list->AddRectFilled(bb_inner.Min, DimgVec2D::new(mid_x, bb_inner.Max.y), GetColorU32(col_rgb_without_alpha), rounding, ImDrawFlags_RoundCornersLeft);
    }
    else
    {
        // Because GetColorU32() multiplies by the global style Alpha and we don't want to display a checkerboard if the source code had no alpha
        Vector4D col_source = (flags & ImGuiColorEditFlags_AlphaPreview) ? col_rgb : col_rgb_without_alpha;
        if (col_source.w < 1.0)
            RenderColorRectWithAlphaCheckerboard(window.draw_list, bb_inner.Min, bb_inner.Max, GetColorU32(col_source), grid_step, DimgVec2D::new(off, off), rounding);
        else
            window.draw_list->AddRectFilled(bb_inner.Min, bb_inner.Max, GetColorU32(col_source), rounding);
    }
    RenderNavHighlight(bb, id);
    if ((flags & ImGuiColorEditFlags_NoBorder) == 0)
    {
        if (g.Style.FrameBorderSize > 0.0)
            RenderFrameBorder(bb.Min, bb.Max, rounding);
        else
            window.draw_list->AddRect(bb.Min, bb.Max, GetColorU32(ImGuiCol_FrameBg), rounding); // Color button are often in need of some sort of border
    }

    // Drag and Drop Source
    // NB: The active_id test is merely an optional micro-optimization, begin_drag_drop_source() does the same test.
    if (g.ActiveId == id && !(flags & ImGuiColorEditFlags_NoDragDrop) && begin_drag_drop_source())
    {
        if (flags & ImGuiColorEditFlags_NoAlpha)
            set_drag_drop_payload(IMGUI_PAYLOAD_TYPE_COLOR_3F, &col_rgb, sizeof * 3, ImGuiCond_Once);
        else
            set_drag_drop_payload(IMGUI_PAYLOAD_TYPE_COLOR_4F, &col_rgb, sizeof * 4, ImGuiCond_Once);
        ColorButton(desc_id, col, flags);
        same_line();
        TextEx("Color");
        end_drag_drop_source();
    }

    // Tooltip
    if (!(flags & ImGuiColorEditFlags_NoTooltip) && hovered)
        ColorTooltip(desc_id, &col.x, flags & (ImGuiColorEditFlags_InputMask_ | ImGuiColorEditFlags_NoAlpha | ImGuiColorEditFlags_AlphaPreview | ImGuiColorEditFlags_AlphaPreviewHalf));

    return pressed;
}

// Initialize/override default color options
void ImGui::SetColorEditOptions(ImGuiColorEditFlags flags)
{
    // ImGuiContext& g = *GImGui;
    if ((flags & ImGuiColorEditFlags_DisplayMask_) == 0)
        flags |= ImGuiColorEditFlags_DefaultOptions_ & ImGuiColorEditFlags_DisplayMask_;
    if ((flags & ImGuiColorEditFlags_DataTypeMask_) == 0)
        flags |= ImGuiColorEditFlags_DefaultOptions_ & ImGuiColorEditFlags_DataTypeMask_;
    if ((flags & ImGuiColorEditFlags_PickerMask_) == 0)
        flags |= ImGuiColorEditFlags_DefaultOptions_ & ImGuiColorEditFlags_PickerMask_;
    if ((flags & ImGuiColorEditFlags_InputMask_) == 0)
        flags |= ImGuiColorEditFlags_DefaultOptions_ & ImGuiColorEditFlags_InputMask_;
    IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_DisplayMask_));    // Check only 1 option is selected
    IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_DataTypeMask_));   // Check only 1 option is selected
    IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_PickerMask_));     // Check only 1 option is selected
    IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiColorEditFlags_InputMask_));      // Check only 1 option is selected
    g.ColorEditOptions = flags;
}

// Note: only access 3 floats if ImGuiColorEditFlags_NoAlpha flag is set.
void ImGui::ColorTooltip(const char* text, let* col, ImGuiColorEditFlags flags)
{
    // ImGuiContext& g = *GImGui;

    BeginTooltipEx(ImGuiTooltipFlags_OverridePreviousTooltip, WindowFlags_None);
    const char* text_end = text ? FindRenderedTextEnd(text, None) : text;
    if (text_end > text)
    {
        TextEx(text, text_end);
        Separator();
    }

    Vector2D sz(g.FontSize * 3 + g.Style.FramePadding.y * 2, g.FontSize * 3 + g.Style.FramePadding.y * 2);
    Vector4D cf(col[0], col[1], col[2], (flags & ImGuiColorEditFlags_NoAlpha) ? 1.0 : col[3]);
    int cr = IM_F32_TO_INT8_SAT(col[0]), cg = IM_F32_TO_INT8_SAT(col[1]), cb = IM_F32_TO_INT8_SAT(col[2]), ca = (flags & ImGuiColorEditFlags_NoAlpha) ? 255 : IM_F32_TO_INT8_SAT(col[3]);
    ColorButton("##preview", cf, (flags & (ImGuiColorEditFlags_InputMask_ | ImGuiColorEditFlags_NoAlpha | ImGuiColorEditFlags_AlphaPreview | ImGuiColorEditFlags_AlphaPreviewHalf)) | ImGuiColorEditFlags_NoTooltip, sz);
    same_line();
    if ((flags & ImGuiColorEditFlags_InputRGB) || !(flags & ImGuiColorEditFlags_InputMask_))
    {
        if (flags & ImGuiColorEditFlags_NoAlpha)
            text("#%02X%02X%02X\nR: %d, G: %d, B: %d\n(%.3, %.3, %.3)", cr, cg, cb, cr, cg, cb, col[0], col[1], col[2]);
        else
            text("#%02X%02X%02X%02X\nR:%d, G:%d, B:%d, A:%d\n(%.3, %.3, %.3, %.3)", cr, cg, cb, ca, cr, cg, cb, ca, col[0], col[1], col[2], col[3]);
    }
    else if (flags & ImGuiColorEditFlags_InputHSV)
    {
        if (flags & ImGuiColorEditFlags_NoAlpha)
            text("H: %.3, S: %.3, V: %.3", col[0], col[1], col[2]);
        else
            text("H: %.3, S: %.3, V: %.3, A: %.3", col[0], col[1], col[2], col[3]);
    }
    EndTooltip();
}

void ImGui::ColorEditOptionsPopup(let* col, ImGuiColorEditFlags flags)
{
    bool allow_opt_inputs = !(flags & ImGuiColorEditFlags_DisplayMask_);
    bool allow_opt_datatype = !(flags & ImGuiColorEditFlags_DataTypeMask_);
    if ((!allow_opt_inputs && !allow_opt_datatype) || !begin_popup("context"))
        return;
    // ImGuiContext& g = *GImGui;
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
    if (Button("Copy as..", DimgVec2D::new(-1, 0)))
        open_popup("Copy");
    if (begin_popup("Copy"))
    {
        int cr = IM_F32_TO_INT8_SAT(col[0]), cg = IM_F32_TO_INT8_SAT(col[1]), cb = IM_F32_TO_INT8_SAT(col[2]), ca = (flags & ImGuiColorEditFlags_NoAlpha) ? 255 : IM_F32_TO_INT8_SAT(col[3]);
        char buf[64];
        ImFormatString(buf, IM_ARRAYSIZE(buf), "(%.3f, %.3f, %.3f, %.3f)", col[0], col[1], col[2], (flags & ImGuiColorEditFlags_NoAlpha) ? 1.0 : col[3]);
        if (selectable(buf))
            SetClipboardText(buf);
        ImFormatString(buf, IM_ARRAYSIZE(buf), "(%d,%d,%d,%d)", cr, cg, cb, ca);
        if (selectable(buf))
            SetClipboardText(buf);
        ImFormatString(buf, IM_ARRAYSIZE(buf), "#%02X%02X%02X", cr, cg, cb);
        if (selectable(buf))
            SetClipboardText(buf);
        if (!(flags & ImGuiColorEditFlags_NoAlpha))
        {
            ImFormatString(buf, IM_ARRAYSIZE(buf), "#%02X%02X%02X%02X", cr, cg, cb, ca);
            if (selectable(buf))
                SetClipboardText(buf);
        }
        end_popup();
    }

    g.ColorEditOptions = opts;
    end_popup();
}

void ImGui::ColorPickerOptionsPopup(let* ref_col, ImGuiColorEditFlags flags)
{
    bool allow_opt_picker = !(flags & ImGuiColorEditFlags_PickerMask_);
    bool allow_opt_alpha_bar = !(flags & ImGuiColorEditFlags_NoAlpha) && !(flags & ImGuiColorEditFlags_AlphaBar);
    if ((!allow_opt_picker && !allow_opt_alpha_bar) || !begin_popup("context"))
        return;
    // ImGuiContext& g = *GImGui;
    if (allow_opt_picker)
    {
        Vector2D picker_size(g.FontSize * 8, ImMax(g.FontSize * 8 - (get_frame_height() + g.Style.ItemInnerSpacing.x), 1.0)); // FIXME: Picker size copied from main picker function
        PushItemWidth(picker_size.x);
        for (int picker_type = 0; picker_type < 2; picker_type += 1)
        {
            // Draw small/thumbnail version of each picker type (over an invisible button for selection)
            if (picker_type > 0) Separator();
            push_id(picker_type);
            ImGuiColorEditFlags picker_flags = ImGuiColorEditFlags_NoInputs | ImGuiColorEditFlags_NoOptions | ImGuiColorEditFlags_NoLabel | ImGuiColorEditFlags_NoSidePreview | (flags & ImGuiColorEditFlags_NoAlpha);
            if (picker_type == 0) picker_flags |= ImGuiColorEditFlags_PickerHueBar;
            if (picker_type == 1) picker_flags |= ImGuiColorEditFlags_PickerHueWheel;
            Vector2D backup_pos = GetCursorScreenPos();
            if (selectable("##selectable", false, 0, picker_size)) // By default, selectable() is closing popup
                g.ColorEditOptions = (g.ColorEditOptions & ~ImGuiColorEditFlags_PickerMask_) | (picker_flags & ImGuiColorEditFlags_PickerMask_);
            SetCursorScreenPos(backup_pos);
            Vector4D previewing_ref_col;
            memcpy(&previewing_ref_col, ref_col, sizeof * ((picker_flags & ImGuiColorEditFlags_NoAlpha) ? 3 : 4));
            ColorPicker4("##previewing_picker", &previewing_ref_col.x, picker_flags);
            pop_id();
        }
        PopItemWidth();
    }
    if (allow_opt_alpha_bar)
    {
        if (allow_opt_picker) Separator();
        CheckboxFlags("Alpha Bar", &g.ColorEditOptions, ImGuiColorEditFlags_AlphaBar);
    }
    end_popup();
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

bool ImGui::TreeNode(const char* str_id, const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    bool is_open = TreeNodeExV(str_id, 0, fmt, args);
    va_end(args);
    return is_open;
}

bool ImGui::TreeNode(const void* ptr_id, const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    bool is_open = TreeNodeExV(ptr_id, 0, fmt, args);
    va_end(args);
    return is_open;
}

bool ImGui::TreeNode(const char* label)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;
    return TreeNodeBehavior(window.GetID(label), 0, label, None);
}

bool ImGui::TreeNodeV(const char* str_id, const char* fmt, va_list args)
{
    return TreeNodeExV(str_id, 0, fmt, args);
}

bool ImGui::TreeNodeV(const void* ptr_id, const char* fmt, va_list args)
{
    return TreeNodeExV(ptr_id, 0, fmt, args);
}

bool ImGui::TreeNodeEx(const char* label, ImGuiTreeNodeFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    return TreeNodeBehavior(window.GetID(label), flags, label, None);
}

bool ImGui::TreeNodeEx(const char* str_id, ImGuiTreeNodeFlags flags, const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    bool is_open = TreeNodeExV(str_id, flags, fmt, args);
    va_end(args);
    return is_open;
}

bool ImGui::TreeNodeEx(const void* ptr_id, ImGuiTreeNodeFlags flags, const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    bool is_open = TreeNodeExV(ptr_id, flags, fmt, args);
    va_end(args);
    return is_open;
}

bool ImGui::TreeNodeExV(const char* str_id, ImGuiTreeNodeFlags flags, const char* fmt, va_list args)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    const char* label, *label_end;
    ImFormatStringToTempBufferV(&label, &label_end, fmt, args);
    return TreeNodeBehavior(window.GetID(str_id), flags, label, label_end);
}

bool ImGui::TreeNodeExV(const void* ptr_id, ImGuiTreeNodeFlags flags, const char* fmt, va_list args)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    const char* label, *label_end;
    ImFormatStringToTempBufferV(&label, &label_end, fmt, args);
    return TreeNodeBehavior(window.GetID(ptr_id), flags, label, label_end);
}

bool ImGui::TreeNodeBehaviorIsOpen(Id32 id, ImGuiTreeNodeFlags flags)
{
    if (flags & ImGuiTreeNodeFlags_Leaf)
        return true;

    // We only write to the tree storage if the user clicks (or explicitly use the SetNextItemOpen function)
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    ImGuiStorage* storage = window.DC.StateStorage;

    bool is_open;
    if (g.next_item_data.Flags & NextItemDataFlags::HasOpen)
    {
        if (g.next_item_data.OpenCond & ImGuiCond_Always)
        {
            is_open = g.next_item_data.OpenVal;
            storage->SetInt(id, is_open);
        }
        else
        {
            // We treat ImGuiCond_Once and ImGuiCond_FirstUseEver the same because tree node state are not saved persistently.
            let stored_value = storage->GetInt(id, -1);
            if (stored_value == -1)
            {
                is_open = g.next_item_data.OpenVal;
                storage->SetInt(id, is_open);
            }
            else
            {
                is_open = stored_value != 0;
            }
        }
    }
    else
    {
        is_open = storage->GetInt(id, (flags & ImGuiTreeNodeFlags_DefaultOpen) ? 1 : 0) != 0;
    }

    // When logging is enabled, we automatically expand tree nodes (but *NOT* collapsing headers.. seems like sensible behavior).
    // NB- If we are above max depth we still allow manually opened nodes to be logged.
    if (g.log_enabled && !(flags & ImGuiTreeNodeFlags_NoAutoOpenOnLog) && (window.DC.TreeDepth - g.LogDepthRef) < g.LogDepthToExpand)
        is_open = true;

    return is_open;
}

bool ImGui::TreeNodeBehavior(Id32 id, ImGuiTreeNodeFlags flags, const char* label, const char* label_end)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;
    const bool display_frame = (flags & ImGuiTreeNodeFlags_Framed) != 0;
    const Vector2D padding = (display_frame || (flags & ImGuiTreeNodeFlags_FramePadding)) ? style.FramePadding : DimgVec2D::new(style.FramePadding.x, ImMin(window.DC.curr_line_text_base_offset, style.FramePadding.y));

    if (!label_end)
        label_end = FindRenderedTextEnd(label);
    const Vector2D label_size = CalcTextSize(label, label_end, false);

    // We vertically grow up to current line height up the typical widget height.
    let frame_height = ImMax(ImMin(window.DC.curr_line_size.y, g.FontSize + style.FramePadding.y * 2), label_size.y + padding.y * 2);
    ImRect frame_bb;
    frame_bb.Min.x = (flags & ImGuiTreeNodeFlags_SpanFullWidth) ? window.work_rect.Min.x : window.DC.CursorPos.x;
    frame_bb.Min.y = window.DC.CursorPos.y;
    frame_bb.Max.x = window.work_rect.Max.x;
    frame_bb.Max.y = window.DC.CursorPos.y + frame_height;
    if (display_frame)
    {
        // Framed header expand a little outside the default padding, to the edge of inner_clip_rect
        // (FIXME: May remove this at some point and make inner_clip_rect align with window_padding.x instead of window_padding.x*0.5)
        frame_bb.Min.x -= IM_FLOOR(window.WindowPadding.x * 0.5 - 1.0);
        frame_bb.Max.x += IM_FLOOR(window.WindowPadding.x * 0.5);
    }

    let text_offset_x = g.FontSize + (display_frame ? padding.x * 3 : padding.x * 2);           // Collapser arrow width + Spacing
    let text_offset_y = ImMax(padding.y, window.DC.curr_line_text_base_offset);                    // Latch before ItemSize changes it
    let text_width = g.FontSize + (label_size.x > 0.0 ? label_size.x + padding.x * 2 : 0.0);  // Include collapser
    Vector2D text_pos(window.DC.CursorPos.x + text_offset_x, window.DC.CursorPos.y + text_offset_y);
    ItemSize(DimgVec2D::new(text_width, frame_height), padding.y);

    // For regular tree nodes, we arbitrary allow to click past 2 worth of ItemSpacing
    ImRect interact_bb = frame_bb;
    if (!display_frame && (flags & (ImGuiTreeNodeFlags_SpanAvailWidth | ImGuiTreeNodeFlags_SpanFullWidth)) == 0)
        interact_bb.Max.x = frame_bb.Min.x + text_width + style.item_spacing.x * 2.0;

    // Store a flag for the current depth to tell if we will allow closing this node when navigating one of its child.
    // For this purpose we essentially compare if g.nav_id_is_alive went from 0 to 1 between TreeNode() and TreePop().
    // This is currently only support 32 level deep and we are fine with (1 << Depth) overflowing into a zero.
    const bool is_leaf = (flags & ImGuiTreeNodeFlags_Leaf) != 0;
    bool is_open = TreeNodeBehaviorIsOpen(id, flags);
    if (is_open && !g.NavIdIsAlive && (flags & ImGuiTreeNodeFlags_NavLeftJumpsBackHere) && !(flags & ImGuiTreeNodeFlags_NoTreePushOnOpen))
        window.DC.TreeJumpToParentOnPopMask |= (1 << window.DC.TreeDepth);

    bool item_add = ItemAdd(interact_bb, id);
    g.last_item_data.StatusFlags |= ItemStatusFlags::HasDisplayRect;
    g.last_item_data.display_rect = frame_bb;

    if (!item_add)
    {
        if (is_open && !(flags & ImGuiTreeNodeFlags_NoTreePushOnOpen))
            Treepush_override_id(id);
        IMGUI_TEST_ENGINE_ITEM_INFO(g.last_item_data.id, label, g.last_item_data.StatusFlags | (is_leaf ? 0 : ItemStatusFlags::Openable) | (is_open ? ItemStatusFlags::Opened : 0));
        return is_open;
    }

    ImGuiButtonFlags button_flags = ImGuiTreeNodeFlags_None;
    if (flags & ImGuiTreeNodeFlags_AllowItemOverlap)
        button_flags |= ImGuiButtonFlags_AllowItemOverlap;
    if (!is_leaf)
        button_flags |= ImGuiButtonFlags_PressedOnDragDropHold;

    // We allow clicking on the arrow section with keyboard modifiers held, in order to easily
    // allow browsing a tree while preserving selection with code implementing multi-selection patterns.
    // When clicking on the rest of the tree node we always disallow keyboard modifiers.
    let arrow_hit_x1 = (text_pos.x - text_offset_x) - style.TouchExtraPadding.x;
    let arrow_hit_x2 = (text_pos.x - text_offset_x) + (g.FontSize + padding.x * 2.0) + style.TouchExtraPadding.x;
    const bool is_mouse_x_over_arrow = (g.IO.MousePos.x >= arrow_hit_x1 && g.IO.MousePos.x < arrow_hit_x2);
    if (window != g.HoveredWindow || !is_mouse_x_over_arrow)
        button_flags |= ImGuiButtonFlags_NoKeyModifiers;

    // Open behaviors can be altered with the _OpenOnArrow and _OnOnDoubleClick flags.
    // Some alteration have subtle effects (e.g. toggle on MouseUp vs mouse_down events) due to requirements for multi-selection and drag and drop support.
    // - Single-click on label = Toggle on MouseUp (default, when _OpenOnArrow=0)
    // - Single-click on arrow = Toggle on mouse_down (when _OpenOnArrow=0)
    // - Single-click on arrow = Toggle on mouse_down (when _OpenOnArrow=1)
    // - Double-click on label = Toggle on MouseDoubleClick (when _OpenOnDoubleClick=1)
    // - Double-click on arrow = Toggle on MouseDoubleClick (when _OpenOnDoubleClick=1 and _OpenOnArrow=0)
    // It is rather standard that arrow click react on down rather than Up.
    // We set ImGuiButtonFlags_PressedOnClickRelease on OpenOnDoubleClick because we want the item to be active on the initial mouse_down in order for drag and drop to work.
    if (is_mouse_x_over_arrow)
        button_flags |= ImGuiButtonFlags_PressedOnClick;
    else if (flags & ImGuiTreeNodeFlags_OpenOnDoubleClick)
        button_flags |= ImGuiButtonFlags_PressedOnClickRelease | ImGuiButtonFlags_PressedOnDoubleClick;
    else
        button_flags |= ImGuiButtonFlags_PressedOnClickRelease;

    bool selected = (flags & ImGuiTreeNodeFlags_Selected) != 0;
    const bool was_selected = selected;

    bool hovered, held;
    bool pressed = ButtonBehavior(interact_bb, id, &hovered, &held, button_flags);
    bool toggled = false;
    if (!is_leaf)
    {
        if (pressed && g.DragDropHoldJustPressedId != id)
        {
            if ((flags & (ImGuiTreeNodeFlags_OpenOnArrow | ImGuiTreeNodeFlags_OpenOnDoubleClick)) == 0 || (g.NavActivateId == id))
                toggled = true;
            if (flags & ImGuiTreeNodeFlags_OpenOnArrow)
                toggled |= is_mouse_x_over_arrow && !g.NavDisableMouseHover; // Lightweight equivalent of is_mouse_hovering_rect() since ButtonBehavior() already did the job
            if ((flags & ImGuiTreeNodeFlags_OpenOnDoubleClick) && g.IO.MouseClickedCount[0] == 2)
                toggled = true;
        }
        else if (pressed && g.DragDropHoldJustPressedId == id)
        {
            IM_ASSERT(button_flags & ImGuiButtonFlags_PressedOnDragDropHold);
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
            window.DC.StateStorage->SetInt(id, is_open);
            g.last_item_data.StatusFlags |= ItemStatusFlags::ToggledOpen;
        }
    }
    if (flags & ImGuiTreeNodeFlags_AllowItemOverlap)
        SetItemAllowOverlap();

    // In this branch, TreeNodeBehavior() cannot toggle the selection so this will never trigger.
    if (selected != was_selected) //-V547
        g.last_item_data.StatusFlags |= ItemStatusFlags::ToggledSelection;

    // Render
    const ImU32 text_col = GetColorU32(ImGuiCol_Text);
    ImGuiNavHighlightFlags nav_highlight_flags = ImGuiNavHighlightFlags_TypeThin;
    if (display_frame)
    {
        // Framed type
        const ImU32 bg_col = GetColorU32((held && hovered) ? ImGuiCol_HeaderActive : hovered ? ImGuiCol_HeaderHovered : ImGuiCol_Header);
        RenderFrame(frame_bb.Min, frame_bb.Max, bg_col, true, style.frame_rounding);
        RenderNavHighlight(frame_bb, id, nav_highlight_flags);
        if (flags & ImGuiTreeNodeFlags_Bullet)
            RenderBullet(window.draw_list, DimgVec2D::new(text_pos.x - text_offset_x * 0.60, text_pos.y + g.FontSize * 0.5), text_col);
        else if (!is_leaf)
            RenderArrow(window.draw_list, DimgVec2D::new(text_pos.x - text_offset_x + padding.x, text_pos.y), text_col, is_open ? ImGuiDir_Down : ImGuiDir_Right, 1.0);
        else // Leaf without bullet, left-adjusted text
            text_pos.x -= text_offset_x;
        if (flags & ImGuiTreeNodeFlags_ClipLabelForTrailingButton)
            frame_bb.Max.x -= g.FontSize + style.FramePadding.x;

        if (g.log_enabled)
            LogSetNextTextDecoration("###", "###");
        render_textClipped(text_pos, frame_bb.Max, label, label_end, &label_size);
    }
    else
    {
        // Unframed typed for tree nodes
        if (hovered || selected)
        {
            const ImU32 bg_col = GetColorU32((held && hovered) ? ImGuiCol_HeaderActive : hovered ? ImGuiCol_HeaderHovered : ImGuiCol_Header);
            RenderFrame(frame_bb.Min, frame_bb.Max, bg_col, false);
        }
        RenderNavHighlight(frame_bb, id, nav_highlight_flags);
        if (flags & ImGuiTreeNodeFlags_Bullet)
            RenderBullet(window.draw_list, DimgVec2D::new(text_pos.x - text_offset_x * 0.5, text_pos.y + g.FontSize * 0.5), text_col);
        else if (!is_leaf)
            RenderArrow(window.draw_list, DimgVec2D::new(text_pos.x - text_offset_x + padding.x, text_pos.y + g.FontSize * 0.15), text_col, is_open ? ImGuiDir_Down : ImGuiDir_Right, 0.70);
        if (g.log_enabled)
            LogSetNextTextDecoration(">", None);
        render_text(text_pos, label, label_end, false);
    }

    if (is_open && !(flags & ImGuiTreeNodeFlags_NoTreePushOnOpen))
        Treepush_override_id(id);
    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.last_item_data.StatusFlags | (is_leaf ? 0 : ItemStatusFlags::Openable) | (is_open ? ItemStatusFlags::Opened : 0));
    return is_open;
}

void ImGui::TreePush(const char* str_id)
{
    Window* window = GetCurrentWindow();
    Indent();
    window.DC.TreeDepth += 1;
    push_id(str_id);
}

void ImGui::TreePush(const void* ptr_id)
{
    Window* window = GetCurrentWindow();
    Indent();
    window.DC.TreeDepth += 1;
    push_id(ptr_id);
}

void ImGui::Treepush_override_id(Id32 id)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    Indent();
    window.DC.TreeDepth += 1;
    push_override_id(id);
}

void ImGui::TreePop()
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    Unindent();

    window.DC.TreeDepth--;
    ImU32 tree_depth_mask = (1 << window.DC.TreeDepth);

    // Handle Left arrow to move to parent tree node (when ImGuiTreeNodeFlags_NavLeftJumpsBackHere is enabled)
    if (g.NavMoveDir == ImGuiDir_Left && g.NavWindow == window && NavMoveRequestButNoResultYet())
        if (g.NavIdIsAlive && (window.DC.TreeJumpToParentOnPopMask & tree_depth_mask))
        {
            SetNavID(window.idStack.back(), g.NavLayer, 0, ImRect());
            NavMoveRequestCancel();
        }
    window.DC.TreeJumpToParentOnPopMask &= tree_depth_mask - 1;

    IM_ASSERT(window.idStack.Size > 1); // There should always be 1 element in the IDStack (pushed during window creation). If this triggers you called TreePop/PopID too much.
    pop_id();
}

// Horizontal distance preceding label when using TreeNode() or Bullet()
float ImGui::GetTreeNodeToLabelSpacing()
{
    // ImGuiContext& g = *GImGui;
    return g.FontSize + (g.Style.FramePadding.x * 2.0);
}

// Set next TreeNode/CollapsingHeader open state.
void ImGui::SetNextItemOpen(bool is_open, ImGuiCond cond)
{
    // ImGuiContext& g = *GImGui;
    if (g.current_window_id->SkipItems)
        return;
    g.next_item_data.Flags |= NextItemDataFlags::HasOpen;
    g.next_item_data.OpenVal = is_open;
    g.next_item_data.OpenCond = cond ? cond : ImGuiCond_Always;
}

// CollapsingHeader returns true when opened but do not indent nor push into the id stack (because of the ImGuiTreeNodeFlags_NoTreePushOnOpen flag).
// This is basically the same as calling TreeNodeEx(label, ImGuiTreeNodeFlags_CollapsingHeader). You can remove the _NoTreePushOnOpen flag if you want behavior closer to normal TreeNode().
bool ImGui::CollapsingHeader(const char* label, ImGuiTreeNodeFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    return TreeNodeBehavior(window.GetID(label), flags | ImGuiTreeNodeFlags_CollapsingHeader, label);
}

// p_visible == None                        : regular collapsing header
// p_visible != None && *p_visible == true  : show a small close button on the corner of the header, clicking the button will set *p_visible = false
// p_visible != None && *p_visible == false : do not show the header at all
// Do not mistake this with the Open state of the header itself, which you can adjust with SetNextItemOpen() or ImGuiTreeNodeFlags_DefaultOpen.
bool ImGui::CollapsingHeader(const char* label, bool* p_visible, ImGuiTreeNodeFlags flags)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    if (p_visible && !*p_visible)
        return false;

    Id32 id = window.GetID(label);
    flags |= ImGuiTreeNodeFlags_CollapsingHeader;
    if (p_visible)
        flags |= ImGuiTreeNodeFlags_AllowItemOverlap | ImGuiTreeNodeFlags_ClipLabelForTrailingButton;
    bool is_open = TreeNodeBehavior(id, flags, label);
    if (p_visible != None)
    {
        // Create a small overlapping close button
        // FIXME: We can evolve this into user accessible helpers to add extra buttons on title bars, headers, etc.
        // FIXME: CloseButton can overlap into text, need find a way to clip the text somehow.
        // ImGuiContext& g = *GImGui;
        ImGuiLastItemData last_item_backup = g.last_item_data;
        let button_size =  g.FontSize;
        let button_x =  ImMax(g.last_item_data.Rect.Min.x, g.last_item_data.Rect.Max.x - g.Style.FramePadding.x * 2.0 - button_size);
        let button_y =  g.last_item_data.Rect.Min.y;
        Id32 close_button_id = GetIDWithSeed("#CLOSE", None, id);
        if (CloseButton(close_button_id, DimgVec2D::new(button_x, button_y)))
            *p_visible = false;
        g.last_item_data = last_item_backup;
    }

    return is_open;
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: selectable
//-------------------------------------------------------------------------
// - selectable()
//-------------------------------------------------------------------------

// Tip: pass a non-visible label (e.g. "##hello") then you can use the space to draw other text or image.
// But you need to make sure the id is unique, e.g. enclose calls in push_id/PopID or use ##unique_id.
// With this scheme, ImGuiselectableFlags_SpanAllColumns and ImGuiselectableFlags_AllowItemOverlap are also frequently used flags.
// FIXME: selectable() with (size.x == 0.0) and (selectableTextAlign.x > 0.0) followed by same_line() is currently not supported.
bool ImGui::selectable(const char* label, bool selected, ImGuiselectableFlags flags, const Vector2D& size_arg)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;

    // Submit label or explicit size to ItemSize(), whereas ItemAdd() will submit a larger/spanning rectangle.
    Id32 id = window.GetID(label);
    Vector2D label_size = CalcTextSize(label, None, true);
    Vector2D size(size_arg.x != 0.0 ? size_arg.x : label_size.x, size_arg.y != 0.0 ? size_arg.y : label_size.y);
    Vector2D pos = window.DC.CursorPos;
    pos.y += window.DC.curr_line_text_base_offset;
    ItemSize(size, 0.0);

    // Fill horizontal space
    // We don't support (size < 0.0) in selectable() because the ItemSpacing extension would make explicitly right-aligned sizes not visibly match other widgets.
    const bool span_all_columns = (flags & ImGuiselectableFlags_SpanAllColumns) != 0;
    let min_x = span_all_columns ? window.ParentWorkRect.Min.x : pos.x;
    let max_x = span_all_columns ? window.ParentWorkRect.Max.x : window.work_rect.Max.x;
    if (size_arg.x == 0.0 || (flags & ImGuiselectableFlags_SpanAvailWidth))
        size.x = ImMax(label_size.x, max_x - min_x);

    // Text stays at the submission position, but bounding box may be extended on both sides
    const Vector2D text_min = pos;
    const Vector2D text_max(min_x + size.x, pos.y + size.y);

    // selectables are meant to be tightly packed together with no click-gap, so we extend their box to cover spacing between selectable.
    ImRect bb(min_x, pos.y, text_max.x, text_max.y);
    if ((flags & ImGuiselectableFlags_NoPadWithHalfSpacing) == 0)
    {
        let spacing_x = span_all_columns ? 0.0 : style.item_spacing.x;
        let spacing_y = style.item_spacing.y;
        let spacing_L = IM_FLOOR(spacing_x * 0.50);
        let spacing_U = IM_FLOOR(spacing_y * 0.50);
        bb.Min.x -= spacing_L;
        bb.Min.y -= spacing_U;
        bb.Max.x += (spacing_x - spacing_L);
        bb.Max.y += (spacing_y - spacing_U);
    }
    //if (g.io.key_ctrl) { GetForegroundDrawList()->add_rect(bb.min, bb.max, IM_COL32(0, 255, 0, 255)); }

    // Modify clip_rect for the ItemAdd(), faster than doing a PushColumnsBackground/PushTableBackground for every selectable..
    let backup_clip_rect_min_x = window.ClipRect.Min.x;
    let backup_clip_rect_max_x = window.ClipRect.Max.x;
    if (span_all_columns)
    {
        window.ClipRect.Min.x = window.ParentWorkRect.Min.x;
        window.ClipRect.Max.x = window.ParentWorkRect.Max.x;
    }

    const bool disabled_item = (flags & ImGuiselectableFlags_Disabled) != 0;
    const bool item_add = ItemAdd(bb, id, None, disabled_item ? ItemFlags::Disabled : ItemFlags::None);
    if (span_all_columns)
    {
        window.ClipRect.Min.x = backup_clip_rect_min_x;
        window.ClipRect.Max.x = backup_clip_rect_max_x;
    }

    if (!item_add)
        return false;

    const bool disabled_global = (g.CurrentItemFlags & ItemFlags::Disabled) != 0;
    if (disabled_item && !disabled_global) // Only testing this as an optimization
        BeginDisabled();

    // FIXME: We can standardize the behavior of those two, we could also keep the fast path of override clip_rect + full push on render only,
    // which would be advantageous since most selectable are not selected.
    if (span_all_columns && window.DC.current_columns)
        PushColumnsBackground();
    else if (span_all_columns && g.current_table)
        TablePushBackgroundChannel();

    // We use NoHoldingActiveID on menus so user can click and _hold_ on a menu then drag to browse child entries
    ImGuiButtonFlags button_flags = 0;
    if (flags & ImGuiselectableFlags_NoHoldingActiveID) { button_flags |= ImGuiButtonFlags_NoHoldingActiveId; }
    if (flags & ImGuiselectableFlags_SelectOnClick)     { button_flags |= ImGuiButtonFlags_PressedOnClick; }
    if (flags & ImGuiselectableFlags_SelectOnRelease)   { button_flags |= ImGuiButtonFlags_PressedOnRelease; }
    if (flags & ImGuiselectableFlags_AllowDoubleClick)  { button_flags |= ImGuiButtonFlags_PressedOnClickRelease | ImGuiButtonFlags_PressedOnDoubleClick; }
    if (flags & ImGuiselectableFlags_AllowItemOverlap)  { button_flags |= ImGuiButtonFlags_AllowItemOverlap; }

    const bool was_selected = selected;
    bool hovered, held;
    bool pressed = ButtonBehavior(bb, id, &hovered, &held, button_flags);

    // Auto-select when moved into
    // - This will be more fully fleshed in the range-select branch
    // - This is not exposed as it won't nicely work with some user side handling of shift/control
    // - We cannot do 'if (g.nav_just_moved_to_id != id) { selected = false; pressed = was_selected; }' for two reasons
    //   - (1) it would require focus scope to be set, need exposing PushFocusScope() or equivalent (e.g. BeginSelection() calling PushFocusScope())
    //   - (2) usage will fail with clipped items
    //   The multi-select API aim to fix those issues, e.g. may be replaced with a BeginSelection() API.
    if ((flags & ImGuiselectableFlags_SelectOnNav) && g.NavJustMovedToId != 0 && g.NavJustMovedToFocusScopeId == window.DC.NavFocusScopeIdCurrent)
        if (g.NavJustMovedToId == id)
            selected = pressed = true;

    // Update nav_id when clicking or when Hovering (this doesn't happen on most widgets), so navigation can be resumed with gamepad/keyboard
    if (pressed || (hovered && (flags & ImGuiselectableFlags_SetNavIdOnHover)))
    {
        if (!g.NavDisableMouseHover && g.NavWindow == window && g.NavLayer == window.DC.NavLayerCurrent)
        {
            SetNavID(id, window.DC.NavLayerCurrent, window.DC.NavFocusScopeIdCurrent, WindowRectAbsToRel(window, bb)); // (bb == nav_rect)
            g.NavDisableHighlight = true;
        }
    }
    if (pressed)
        MarkItemEdited(id);

    if (flags & ImGuiselectableFlags_AllowItemOverlap)
        SetItemAllowOverlap();

    // In this branch, selectable() cannot toggle the selection so this will never trigger.
    if (selected != was_selected) //-V547
        g.last_item_data.StatusFlags |= ItemStatusFlags::ToggledSelection;

    // Render
    if (held && (flags & ImGuiselectableFlags_DrawHoveredWhenHeld))
        hovered = true;
    if (hovered || selected)
    {
        const ImU32 col = GetColorU32((held && hovered) ? ImGuiCol_HeaderActive : hovered ? ImGuiCol_HeaderHovered : ImGuiCol_Header);
        RenderFrame(bb.Min, bb.Max, col, false, 0.0);
    }
    RenderNavHighlight(bb, id, ImGuiNavHighlightFlags_TypeThin | ImGuiNavHighlightFlags_NoRounding);

    if (span_all_columns && window.DC.current_columns)
        PopColumnsBackground();
    else if (span_all_columns && g.current_table)
        TablePopBackgroundChannel();

    render_textClipped(text_min, text_max, label, None, &label_size, style.selectableTextAlign, &bb);

    // Automatically close popups
    if (pressed && (window.Flags & WindowFlags_Popup) && !(flags & ImGuiselectableFlags_DontClosePopups) && !(g.last_item_data.in_flags & ItemFlags::selectableDontClosePopup))
        CloseCurrentPopup();

    if (disabled_item && !disabled_global)
        EndDisabled();

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.last_item_data.StatusFlags);
    return pressed; //-V1020
}

bool ImGui::selectable(const char* label, bool* p_selected, ImGuiselectableFlags flags, const Vector2D& size_arg)
{
    if (selectable(label, *p_selected, flags, size_arg))
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
bool ImGui::BeginListBox(const char* label, const Vector2D& size_arg)
{
    // ImGuiContext& g = *GImGui;
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    const ImGuiStyle& style = g.Style;
    const Id32 id = GetID(label);
    const Vector2D label_size = CalcTextSize(label, None, true);

    // size default to hold ~7.25 items.
    // Fractional number of items helps seeing that we can scroll down/up without looking at scrollbar.
    Vector2D size = ImFloor(CalcItemSize(size_arg, CalcItemWidth(), GetTextLineHeightWithSpacing() * 7.25 + style.FramePadding.y * 2.0));
    Vector2D frame_size = DimgVec2D::new(size.x, ImMax(size.y, label_size.y));
    ImRect frame_bb(window.DC.CursorPos, window.DC.CursorPos + frame_size);
    ImRect bb(frame_bb.Min, frame_bb.Max + DimgVec2D::new(label_size.x > 0.0 ? style.ItemInnerSpacing.x + label_size.x : 0.0, 0.0));
    g.next_item_data.ClearFlags();

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
        Vector2D label_pos = DimgVec2D::new(frame_bb.Max.x + style.ItemInnerSpacing.x, frame_bb.Min.y + style.FramePadding.y);
        render_text(label_pos, label);
        window.DC.CursorMaxPos = ImMax(window.DC.CursorMaxPos, label_pos + label_size);
    }

    BeginChildFrame(id, frame_bb.GetSize());
    return true;
}

#ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
// OBSOLETED in 1.81 (from February 2021)
bool ImGui::ListBoxHeader(const char* label, int items_count, int height_in_items)
{
    // If height_in_items == -1, default height is maximum 7.
    // ImGuiContext& g = *GImGui;
    let height_in_items_f =  (height_in_items < 0 ? ImMin(items_count, 7) : height_in_items) + 0.25;
    Vector2D size;
    size.x = 0.0;
    size.y = GetTextLineHeightWithSpacing() * height_in_items_f + g.Style.FramePadding.y * 2.0;
    return BeginListBox(label, size);
}
#endif

void ImGui::EndListBox()
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    IM_ASSERT((window.Flags & WindowFlags_ChildWindow) && "Mismatched BeginListBox/EndListBox calls. Did you test the return value of BeginListBox?");
    IM_UNUSED(window);

    EndChildFrame();
    EndGroup(); // This is only required to be able to do IsItemXXX query on the whole ListBox including label
}

bool ImGui::ListBox(const char* label, int* current_item, const char* const items[], int items_count, int height_items)
{
    const bool value_changed = ListBox(label, current_item, Items_ArrayGetter, (void*)items, items_count, height_items);
    return value_changed;
}

// This is merely a helper around BeginListBox(), EndListBox().
// Considering using those directly to submit custom data or store selection differently.
bool ImGui::ListBox(const char* label, int* current_item, bool (*items_getter)(void*, int, const char**), void* data, int items_count, int height_in_items)
{
    // ImGuiContext& g = *GImGui;

    // Calculate size from "height_in_items"
    if (height_in_items < 0)
        height_in_items = ImMin(items_count, 7);
    let height_in_items_f =  height_in_items + 0.25;
    Vector2D size(0.0, ImFloor(GetTextLineHeightWithSpacing() * height_in_items_f + g.Style.FramePadding.y * 2.0));

    if (!BeginListBox(label, size))
        return false;

    // Assume all items have even height (= 1 line of text). If you need items of different height,
    // you can create a custom version of ListBox() in your code without using the clipper.
    bool value_changed = false;
    ImGuiListClipper clipper;
    clipper.Begin(items_count, GetTextLineHeightWithSpacing()); // We know exactly our line height here so we pass it as a minor optimization, but generally you don't need to.
    while (clipper.Step())
        for (int i = clipper.DisplayStart; i < clipper.DisplayEnd; i += 1)
        {
            const char* item_text;
            if (!items_getter(data, i, &item_text))
                item_text = "*Unknown item*";

            push_id(i);
            const bool item_selected = (i == *current_item);
            if (selectable(item_text, item_selected))
            {
                *current_item = i;
                value_changed = true;
            }
            if (item_selected)
                SetItemDefaultFocus();
            pop_id();
        }
    EndListBox();

    if (value_changed)
        MarkItemEdited(g.last_item_data.id);

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

int ImGui::PlotEx(ImGuiPlotType plot_type, const char* label, float (*values_getter)(void* data, int idx), void* data, int values_count, int values_offset, const char* overlay_text, float scale_min, float scale_max, Vector2D frame_size)
{
    // ImGuiContext& g = *GImGui;
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return -1;

    const ImGuiStyle& style = g.Style;
    const Id32 id = window.GetID(label);

    const Vector2D label_size = CalcTextSize(label, None, true);
    if (frame_size.x == 0.0)
        frame_size.x = CalcItemWidth();
    if (frame_size.y == 0.0)
        frame_size.y = label_size.y + (style.FramePadding.y * 2);

    const ImRect frame_bb(window.DC.CursorPos, window.DC.CursorPos + frame_size);
    const ImRect inner_bb(frame_bb.Min + style.FramePadding, frame_bb.Max - style.FramePadding);
    const ImRect total_bb(frame_bb.Min, frame_bb.Max + DimgVec2D::new(label_size.x > 0.0 ? style.ItemInnerSpacing.x + label_size.x : 0.0, 0));
    ItemSize(total_bb, style.FramePadding.y);
    if (!ItemAdd(total_bb, 0, &frame_bb))
        return -1;
    const bool hovered = ItemHoverable(frame_bb, id);

    // Determine scale from values if not specified
    if (scale_min == FLT_MAX || scale_max == FLT_MAX)
    {
        let v_min =  FLT_MAX;
        let v_max =  -FLT_MAX;
        for (int i = 0; i < values_count; i += 1)
        {
            let v = values_getter(data, i);
            if (v != v) // Ignore NaN values
                continue;
            v_min = ImMin(v_min, v);
            v_max = ImMax(v_max, v);
        }
        if (scale_min == FLT_MAX)
            scale_min = v_min;
        if (scale_max == FLT_MAX)
            scale_max = v_max;
    }

    RenderFrame(frame_bb.Min, frame_bb.Max, GetColorU32(ImGuiCol_FrameBg), true, style.frame_rounding);

    let values_count_min = (plot_type == ImGuiPlotType_Lines) ? 2 : 1;
    int idx_hovered = -1;
    if (values_count >= values_count_min)
    {
        int res_w = ImMin(frame_size.x, values_count) + ((plot_type == ImGuiPlotType_Lines) ? -1 : 0);
        int item_count = values_count + ((plot_type == ImGuiPlotType_Lines) ? -1 : 0);

        // Tooltip on hover
        if (hovered && inner_bb.contains(g.IO.MousePos))
        {
            let t = ImClamp((g.IO.MousePos.x - inner_bb.Min.x) / (inner_bb.Max.x - inner_bb.Min.x), 0.0, 0.9999);
            let v_idx = (t * item_count);
            IM_ASSERT(v_idx >= 0 && v_idx < values_count);

            let v0 = values_getter(data, (v_idx + values_offset) % values_count);
            let v1 = values_getter(data, (v_idx + 1 + values_offset) % values_count);
            if (plot_type == ImGuiPlotType_Lines)
                SetTooltip("%d: %8.4g\n%d: %8.4g", v_idx, v0, v_idx + 1, v1);
            else if (plot_type == ImGuiPlotType_Histogram)
                SetTooltip("%d: %8.4g", v_idx, v0);
            idx_hovered = v_idx;
        }

        let t_step = 1.0 / res_w;
        let inv_scale = (scale_min == scale_max) ? 0.0 : (1.0 / (scale_max - scale_min));

        let v0 =  values_getter(data, (0 + values_offset) % values_count);
        let t0 =  0.0;
        Vector2D tp0 = DimgVec2D::new( t0, 1.0 - ImSaturate((v0 - scale_min) * inv_scale) );                       // Point in the normalized space of our target rectangle
        let histogram_zero_line_t =  (scale_min * scale_max < 0.0) ? (1 + scale_min * inv_scale) : (scale_min < 0.0 ? 0.0 : 1.0);   // Where does the zero line stands

        const ImU32 col_base = GetColorU32((plot_type == ImGuiPlotType_Lines) ? ImGuiCol_PlotLines : ImGuiCol_PlotHistogram);
        const ImU32 col_hovered = GetColorU32((plot_type == ImGuiPlotType_Lines) ? ImGuiCol_PlotLinesHovered : ImGuiCol_PlotHistogramHovered);

        for (int n = 0; n < res_w; n += 1)
        {
            let t1 = t0 + t_step;
            let v1_idx = (t0 * item_count + 0.5);
            IM_ASSERT(v1_idx >= 0 && v1_idx < values_count);
            let v1 = values_getter(data, (v1_idx + values_offset + 1) % values_count);
            const Vector2D tp1 = DimgVec2D::new( t1, 1.0 - ImSaturate((v1 - scale_min) * inv_scale) );

            // NB: Draw calls are merged together by the draw_list system. Still, we should render our batch are lower level to save a bit of CPU.
            Vector2D pos0 = ImLerp(inner_bb.Min, inner_bb.Max, tp0);
            Vector2D pos1 = ImLerp(inner_bb.Min, inner_bb.Max, (plot_type == ImGuiPlotType_Lines) ? tp1 : DimgVec2D::new(tp1.x, histogram_zero_line_t));
            if (plot_type == ImGuiPlotType_Lines)
            {
                window.draw_list->AddLine(pos0, pos1, idx_hovered == v1_idx ? col_hovered : col_base);
            }
            else if (plot_type == ImGuiPlotType_Histogram)
            {
                if (pos1.x >= pos0.x + 2.0)
                    pos1.x -= 1.0;
                window.draw_list->AddRectFilled(pos0, pos1, idx_hovered == v1_idx ? col_hovered : col_base);
            }

            t0 = t1;
            tp0 = tp1;
        }
    }

    // Text overlay
    if (overlay_text)
        render_textClipped(DimgVec2D::new(frame_bb.Min.x, frame_bb.Min.y + style.FramePadding.y), frame_bb.Max, overlay_text, None, None, DimgVec2D::new(0.5, 0.0));

    if (label_size.x > 0.0)
        render_text(DimgVec2D::new(frame_bb.Max.x + style.ItemInnerSpacing.x, inner_bb.Min.y), label);

    // Return hovered index or -1 if none are hovered.
    // This is currently not exposed in the public API because we need a larger redesign of the whole thing, but in the short-term we are making it available in PlotEx().
    return idx_hovered;
}

struct ImGuiPlotArrayGetterData
{
    let* Values;
    int Stride;

    ImGuiPlotArrayGetterData(let* values, int stride) { Values = values; Stride = stride; }
};

static float Plot_ArrayGetter(void* data, int idx)
{
    ImGuiPlotArrayGetterData* plot_data = (ImGuiPlotArrayGetterData*)data;
    let v = *(let*)(const void*)((const unsigned char*)plot_data->Values + idx * plot_data->Stride);
    return v;
}

void ImGui::PlotLines(const char* label, let* values, int values_count, int values_offset, const char* overlay_text, float scale_min, float scale_max, Vector2D graph_size, int stride)
{
    ImGuiPlotArrayGetterData data(values, stride);
    PlotEx(ImGuiPlotType_Lines, label, &Plot_ArrayGetter, (void*)&data, values_count, values_offset, overlay_text, scale_min, scale_max, graph_size);
}

void ImGui::PlotLines(const char* label, float (*values_getter)(void* data, int idx), void* data, int values_count, int values_offset, const char* overlay_text, float scale_min, float scale_max, Vector2D graph_size)
{
    PlotEx(ImGuiPlotType_Lines, label, values_getter, data, values_count, values_offset, overlay_text, scale_min, scale_max, graph_size);
}

void ImGui::PlotHistogram(const char* label, let* values, int values_count, int values_offset, const char* overlay_text, float scale_min, float scale_max, Vector2D graph_size, int stride)
{
    ImGuiPlotArrayGetterData data(values, stride);
    PlotEx(ImGuiPlotType_Histogram, label, &Plot_ArrayGetter, (void*)&data, values_count, values_offset, overlay_text, scale_min, scale_max, graph_size);
}

void ImGui::PlotHistogram(const char* label, float (*values_getter)(void* data, int idx), void* data, int values_count, int values_offset, const char* overlay_text, float scale_min, float scale_max, Vector2D graph_size)
{
    PlotEx(ImGuiPlotType_Histogram, label, values_getter, data, values_count, values_offset, overlay_text, scale_min, scale_max, graph_size);
}

//-------------------------------------------------------------------------
// [SECTION] Widgets: value helpers
// Those is not very useful, legacy API.
//-------------------------------------------------------------------------
// - value()
//-------------------------------------------------------------------------

void ImGui::Value(const char* prefix, bool b)
{
    text("%s: %s", prefix, (b ? "true" : "false"));
}

void ImGui::Value(const char* prefix, int v)
{
    text("%s: %d", prefix, v);
}

void ImGui::Value(const char* prefix, unsigned int v)
{
    text("%s: %d", prefix, v);
}

void ImGui::Value(const char* prefix, float v, const char* float_format)
{
    if (float_format)
    {
        char fmt[64];
        ImFormatString(fmt, IM_ARRAYSIZE(fmt), "%%s: %s", float_format);
        text(fmt, prefix, v);
    }
    else
    {
        text("%s: %.3", prefix, v);
    }
}

//-------------------------------------------------------------------------
// [SECTION] menu_item, BeginMenu, EndMenu, etc.
//-------------------------------------------------------------------------
// - ImGuiMenuColumns [Internal]
// - BeginMenuBar()
// - EndMenuBar()
// - BeginMainMenuBar()
// - EndMainMenuBar()
// - BeginMenu()
// - EndMenu()
// - menu_itemEx() [Internal]
// - menu_item()
//-------------------------------------------------------------------------

// Helpers for internal use
void ImGuiMenuColumns::Update(float spacing, bool window_reappearing)
{
    if (window_reappearing)
        memset(Widths, 0, sizeof(Widths));
    Spacing = (ImU16)spacing;
    CalcNextTotalWidth(true);
    memset(Widths, 0, sizeof(Widths));
    TotalWidth = NextTotalWidth;
    NextTotalWidth = 0;
}

void ImGuiMenuColumns::CalcNextTotalWidth(bool update_offsets)
{
    ImU16 offset = 0;
    bool want_spacing = false;
    for (int i = 0; i < IM_ARRAYSIZE(Widths); i += 1)
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
}

float ImGuiMenuColumns::DeclColumns(float w_icon, float w_label, float w_shortcut, float w_mark)
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
// Ideally we also want this to be responsible for claiming space out of the main window scrolling rectangle, in which case WindowFlags_MenuBar will become unnecessary.
// Then later the same system could be used for multiple menu-bars, scrollbars, side-bars.
bool ImGui::BeginMenuBar()
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;
    if (!(window.Flags & WindowFlags_MenuBar))
        return false;

    IM_ASSERT(!window.DC.MenuBarAppending);
    BeginGroup(); // Backup position on layer 0 // FIXME: Misleading to use a group for that backup/restore
    push_id("##menubar");

    // We don't clip with current window clipping rectangle as it is already set to the area below. However we clip with window full rect.
    // We remove 1 worth of rounding to max.x to that text in long menus and small windows don't tend to display over the lower-right rounded area, which looks particularly glitchy.
    ImRect bar_rect = window.MenuBarRect();
    ImRect clip_rect(IM_ROUND(bar_rect.Min.x + window.WindowBorderSize), IM_ROUND(bar_rect.Min.y + window.WindowBorderSize), IM_ROUND(ImMax(bar_rect.Min.x, bar_rect.Max.x - ImMax(window.WindowRounding, window.WindowBorderSize))), IM_ROUND(bar_rect.Max.y));
    clip_rect.ClipWith(window.OuterRectClipped);
    push_clip_rect(clip_rect.Min, clip_rect.Max, false);

    // We overwrite CursorMaxPos because BeginGroup sets it to CursorPos (essentially the .emit_item hack in EndMenuBar() would need something analogous here, maybe a BeginGroupEx() with flags).
    window.DC.CursorPos = window.DC.CursorMaxPos = DimgVec2D::new(bar_rect.Min.x + window.DC.MenuBarOffset.x, bar_rect.Min.y + window.DC.MenuBarOffset.y);
    window.DC.layout_type = LayoutType::Horizontal;
    window.DC.Issame_line = false;
    window.DC.NavLayerCurrent = ImGuiNavLayer_Menu;
    window.DC.MenuBarAppending = true;
    AlignTextToFramePadding();
    return true;
}

void ImGui::EndMenuBar()
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return;
    // ImGuiContext& g = *GImGui;

    // Nav: When a move request within one of our child menu failed, capture the request to navigate among our siblings.
    if (NavMoveRequestButNoResultYet() && (g.NavMoveDir == ImGuiDir_Left || g.NavMoveDir == ImGuiDir_Right) && (g.NavWindow->Flags & WindowFlags_ChildMenu))
    {
        // Try to find out if the request is for one of our child menu
        Window* nav_earliest_child = g.NavWindow;
        while (nav_earliest_child->ParentWindow && (nav_earliest_child->ParentWindow->Flags & WindowFlags_ChildMenu))
            nav_earliest_child = nav_earliest_child->ParentWindow;
        if (nav_earliest_child->ParentWindow == window && nav_earliest_child->DC.ParentLayoutType == LayoutType::Horizontal && (g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded) == 0)
        {
            // To do so we claim focus back, restore nav_id and then process the movement request for yet another frame.
            // This involve a one-frame delay which isn't very problematic in this situation. We could remove it by scoring in advance for multiple window (probably not worth bothering)
            const ImGuiNavLayer layer = ImGuiNavLayer_Menu;
            IM_ASSERT(window.DC.nav_layers_active_mask_next & (1 << layer)); // Sanity check
            FocusWindow(window);
            SetNavID(window.nav_last_ids[layer], layer, 0, window.nav_rectRel[layer]);
            g.NavDisableHighlight = true; // Hide highlight for the current frame so we don't see the intermediary selection.
            g.NavDisableMouseHover = g.NavMousePosDirty = true;
            NavMoveRequestForward(g.NavMoveDir, g.NavMoveClipDir, g.NavMoveFlags, g.NavMoveScrollFlags); // Repeat
        }
    }

    IM_MSVC_WARNING_SUPPRESS(6011); // Static Analysis false positive "warning C6011: Dereferencing None pointer 'window'"
    IM_ASSERT(window.Flags & WindowFlags_MenuBar);
    IM_ASSERT(window.DC.MenuBarAppending);
    PopClipRect();
    pop_id();
    window.DC.MenuBarOffset.x = window.DC.CursorPos.x - window.pos.x; // Save horizontal position so next append can reuse it. This is kinda equivalent to a per-layer CursorPos.
    g.GroupStack.back().emit_item = false;
    EndGroup(); // Restore position on layer 0
    window.DC.layout_type = ImGuiLayoutType_Vertical;
    window.DC.Issame_line = false;
    window.DC.NavLayerCurrent = ImGuiNavLayer_Main;
    window.DC.MenuBarAppending = false;
}

// Important: calling order matters!
// FIXME: Somehow overlapping with docking tech.
// FIXME: The "rect-cut" aspect of this could be formalized into a lower-level helper (rect-cut: https://halt.software/dead-simple-layouts)
bool ImGui::BeginViewportSideBar(const char* name, ImGuiViewport* viewport_p, ImGuiDir dir, float axis_size, WindowFlags window_flags)
{
    IM_ASSERT(dir != ImGuiDir_None);

    Window* bar_window = find_window_by_name(name);
    ImGuiViewportP* viewport = (ImGuiViewportP*)(void*)(viewport_p ? viewport_p : get_main_viewport());
    if (bar_window == None || bar_window.BeginCount == 0)
    {
        // Calculate and set window size/position
        ImRect avail_rect = viewport->GetBuildWorkRect();
        ImGuiAxis axis = (dir == ImGuiDir_Up || dir == ImGuiDir_Down) ? ImGuiAxis_Y : ImGuiAxis_X;
        Vector2D pos = avail_rect.Min;
        if (dir == ImGuiDir_Right || dir == ImGuiDir_Down)
            pos[axis] = avail_rect.Max[axis] - axis_size;
        Vector2D size = avail_rect.GetSize();
        size[axis] = axis_size;
        set_next_window_pos(pos);
        SetNextWindowSize(size);

        // Report our size into work area (for next frame) using actual window size
        if (dir == ImGuiDir_Up || dir == ImGuiDir_Left)
            viewport->BuildWorkOffsetMin[axis] += axis_size;
        else if (dir == ImGuiDir_Down || dir == ImGuiDir_Right)
            viewport->BuildWorkOffsetMax[axis] -= axis_size;
    }

    window_flags |= WindowFlags_NoTitleBar | WindowFlags_NoResize | WindowFlags_NoMove | WindowFlags_NoDocking;
    set_next_window_viewport(viewport->ID); // Enforce viewport so we don't create our own viewport when ImGuiConfigFlags_ViewportsNoMerge is set.
    PushStyleVar(ImGuiStyleVar_WindowRounding, 0.0);
    PushStyleVar(ImGuiStyleVar_WindowMinSize, DimgVec2D::new(0, 0)); // Lift normal size constraint
    bool is_open = Begin(name, None, window_flags);
    PopStyleVar(2);

    return is_open;
}

bool ImGui::BeginMainMenuBar()
{
    // ImGuiContext& g = *GImGui;
    ImGuiViewportP* viewport = (ImGuiViewportP*)(void*)get_main_viewport();

    // Notify of viewport change so GetFrameHeight() can be accurate in case of DPI change
    SetCurrentViewport(None, viewport);

    // For the main menu bar, which cannot be moved, we honor g.style.DisplaySafeAreaPadding to ensure text can be visible on a TV set.
    // FIXME: This could be generalized as an opt-in way to clamp window->dc.CursorStartPos to avoid SafeArea?
    // FIXME: Consider removing support for safe area down the line... it's messy. Nowadays consoles have support for TV calibration in OS settings.
    g.NextWindowData.MenuBarOffsetMinVal = DimgVec2D::new(g.Style.DisplaySafeAreaPadding.x, ImMax(g.Style.DisplaySafeAreaPadding.y - g.Style.FramePadding.y, 0.0));
    WindowFlags window_flags = WindowFlags_NoScrollbar | WindowFlags_NoSavedSettings | WindowFlags_MenuBar;
    let height =  get_frame_height();
    bool is_open = BeginViewportSideBar("##MainMenuBar", viewport, ImGuiDir_Up, height, window_flags);
    g.NextWindowData.MenuBarOffsetMinVal = DimgVec2D::new(0.0, 0.0);

    if (is_open)
        BeginMenuBar();
    else
        End();
    return is_open;
}

void ImGui::EndMainMenuBar()
{
    EndMenuBar();

    // When the user has left the menu layer (typically: closed menus through activation of an item), we restore focus to the previous window
    // FIXME: With this strategy we won't be able to restore a None focus.
    // ImGuiContext& g = *GImGui;
    if (g.current_window_id == g.NavWindow && g.NavLayer == ImGuiNavLayer_Main && !g.nav_any_request)
        focus_topmost_window_under_one(g.NavWindow, None);

    End();
}

static bool IsRootOfOpenMenuSet()
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    if ((g.open_popupStack.Size <= g.begin_popupStack.Size) || (window.Flags & WindowFlags_ChildMenu))
        return false;

    // Initially we used 'upper_popup->open_parent_id == window->IDStack.back()' to differentiate multiple menu sets from each others
    // (e.g. inside menu bar vs loose menu items) based on parent id.
    // This would however prevent the use of e.g. PuhsID() user code submitting menus.
    // Previously this worked between popup and a first child menu because the first child menu always had the _ChildWindow flag,
    // making  hovering on parent popup possible while first child menu was focused - but this was generally a bug with other side effects.
    // Instead we don't treat Popup specifically (in order to consistently support menu features in them), maybe the first child menu of a Popup
    // doesn't have the _ChildWindow flag, and we rely on this IsRootOfOpenMenuSet() check to allow hovering between root window/popup and first child menu.
    // In the end, lack of id check made it so we could no longer differentiate between separate menu sets. To compensate for that, we at least check parent window nav layer.
    // This fixes the most common case of menu opening on hover when moving between window content and menu bar. Multiple different menu sets in same nav layer would still
    // open on hover, but that should be a lesser problem, because if such menus are close in proximity in window content then it won't feel weird and if they are far apart
    // it likely won't be a problem anyone runs into.
    const ImGuiPopupData* upper_popup = &g.open_popupStack[g.begin_popupStack.Size];
    return (window.DC.NavLayerCurrent == upper_popup->ParentNavLayer && upper_popup->Window && (upper_popup->Window->Flags & WindowFlags_ChildMenu));
}

bool ImGui::BeginMenuEx(const char* label, const char* icon, bool enabled)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;
    const Id32 id = window.GetID(label);
    bool menu_is_open = is_popup_open(id, ImGuiPopupFlags_None);

    // Sub-menus are ChildWindow so that mouse can be hovering across them (otherwise top-most popup menu would steal focus and not allow hovering on parent menu)
    // The first menu in a hierarchy isn't so hovering doesn't get across (otherwise e.g. resizing borders with ImGuiButtonFlags_FlattenChildren would react), but top-most BeginMenu() will bypass that limitation.
    WindowFlags flags = WindowFlags_ChildMenu | WindowFlags_AlwaysAutoResize | WindowFlags_NoMove | WindowFlags_NoTitleBar | WindowFlags_NoSavedSettings | WindowFlags_NoNavFocus;
    if (window.Flags & WindowFlags_ChildMenu)
        flags |= WindowFlags_ChildWindow;

    // If a menu with same the id was already submitted, we will append to it, matching the behavior of Begin().
    // We are relying on a O(N) search - so O(N log N) over the frame - which seems like the most efficient for the expected small amount of BeginMenu() calls per frame.
    // If somehow this is ever becoming a problem we can switch to use e.g. ImGuiStorage mapping key to last frame used.
    if (g.MenusIdSubmittedThisFrame.contains(id))
    {
        if (menu_is_open)
            menu_is_open = begin_popupEx(id, flags); // menu_is_open can be 'false' when the popup is completely clipped (e.g. zero size display)
        else
            g.NextWindowData.ClearFlags();          // we behave like Begin() and need to consume those values
        return menu_is_open;
    }

    // Tag menu as used. Next time BeginMenu() with same id is called it will append to existing menu
    g.MenusIdSubmittedThisFrame.push_back(id);

    Vector2D label_size = CalcTextSize(label, None, true);

    // Odd hack to allow hovering across menus of a same menu-set (otherwise we wouldn't be able to hover parent without always being a Child window)
    const bool menuset_is_open = IsRootOfOpenMenuSet();
    Window* backed_nav_window = g.NavWindow;
    if (menuset_is_open)
        g.NavWindow = window;

    // The reference position stored in popup_pos will be used by Begin() to find a suitable position for the child menu,
    // However the final position is going to be different! It is chosen by FindBestWindowPosForPopup().
    // e.g. Menus tend to overlap each other horizontally to amplify relative Z-ordering.
    Vector2D popup_pos, pos = window.DC.CursorPos;
    push_id(label);
    if (!enabled)
        BeginDisabled();
    const ImGuiMenuColumns* offsets = &window.DC.MenuColumns;
    bool pressed;
    const ImGuiselectableFlags selectable_flags = ImGuiselectableFlags_NoHoldingActiveID | ImGuiselectableFlags_SelectOnClick | ImGuiselectableFlags_DontClosePopups;
    if (window.DC.layout_type == LayoutType::Horizontal)
    {
        // Menu inside an horizontal menu bar
        // selectable extend their highlight by half ItemSpacing in each direction.
        // For ChildMenu, the popup position will be overwritten by the call to FindBestWindowPosForPopup() in Begin()
        popup_pos = DimgVec2D::new(pos.x - 1.0 - IM_FLOOR(style.item_spacing.x * 0.5), pos.y - style.FramePadding.y + window.MenuBarHeight());
        window.DC.CursorPos.x += IM_FLOOR(style.item_spacing.x * 0.5);
        PushStyleVar(ImGuiStyleVar_ItemSpacing, DimgVec2D::new(style.item_spacing.x * 2.0, style.item_spacing.y));
        let w =  label_size.x;
        Vector2D text_pos(window.DC.CursorPos.x + offsets->OffsetLabel, window.DC.CursorPos.y + window.DC.curr_line_text_base_offset);
        pressed = selectable("", menu_is_open, selectable_flags, DimgVec2D::new(w, 0.0));
        render_text(text_pos, label);
        PopStyleVar();
        window.DC.CursorPos.x += IM_FLOOR(style.item_spacing.x * (-1.0 + 0.5)); // -1 spacing to compensate the spacing added when selectable() did a same_line(). It would also work to call same_line() ourselves after the PopStyleVar().
    }
    else
    {
        // Menu inside a regular/vertical menu
        // (In a typical menu window where all items are BeginMenu() or menu_item() calls, extra_w will always be 0.0.
        //  Only when they are other items sticking out we're going to add spacing, yet only register minimum width into the layout system.
        popup_pos = DimgVec2D::new(pos.x, pos.y - style.WindowPadding.y);
        let icon_w =  (icon && icon[0]) ? CalcTextSize(icon, None).x : 0.0;
        let checkmark_w =  IM_FLOOR(g.FontSize * 1.20);
        let min_w =  window.DC.MenuColumns.DeclColumns(icon_w, label_size.x, 0.0, checkmark_w); // Feedback to next frame
        let extra_w =  ImMax(0.0, get_content_region_avail().x - min_w);
        Vector2D text_pos(window.DC.CursorPos.x + offsets->OffsetLabel, window.DC.CursorPos.y + window.DC.curr_line_text_base_offset);
        pressed = selectable("", menu_is_open, selectable_flags | ImGuiselectableFlags_SpanAvailWidth, DimgVec2D::new(min_w, 0.0));
        render_text(text_pos, label);
        if (icon_w > 0.0)
            render_text(pos + DimgVec2D::new(offsets->OffsetIcon, 0.0), icon);
        RenderArrow(window.draw_list, pos + DimgVec2D::new(offsets->OffsetMark + extra_w + g.FontSize * 0.30, 0.0), GetColorU32(ImGuiCol_Text), ImGuiDir_Right);
    }
    if (!enabled)
        EndDisabled();

    const bool hovered = (g.HoveredId == id) && enabled && !g.NavDisableMouseHover;
    if (menuset_is_open)
        g.NavWindow = backed_nav_window;

    bool want_open = false;
    bool want_close = false;
    if (window.DC.layout_type == ImGuiLayoutType_Vertical) // (window->flags & (WindowFlags_Popup|WindowFlags_ChildMenu))
    {
        // Close menu when not hovering it anymore unless we are moving roughly in the direction of the menu
        // Implement http://bjk5.com/post/44698559168/breaking-down-amazons-mega-dropdown to avoid using timers, so menus feels more reactive.
        bool moving_toward_child_menu = false;
        Window* child_menu_window = (g.begin_popupStack.Size < g.open_popupStack.Size && g.open_popupStack[g.begin_popupStack.Size].SourceWindow == window) ? g.open_popupStack[g.begin_popupStack.Size].Window : None;
        if (g.HoveredWindow == window && child_menu_window != None && !(window.Flags & WindowFlags_MenuBar))
        {
            let ref_unit =  g.FontSize; // FIXME-DPI
            ImRect next_window_rect = child_menu_window.Rect();
            Vector2D ta = (g.IO.MousePos - g.IO.MouseDelta);
            Vector2D tb = (window.pos.x < child_menu_window.pos.x) ? next_window_rect.GetTL() : next_window_rect.GetTR();
            Vector2D tc = (window.pos.x < child_menu_window.pos.x) ? next_window_rect.GetBL() : next_window_rect.GetBR();
            let extra =  ImClamp(ImFabs(ta.x - tb.x) * 0.30, ref_unit * 0.5, ref_unit * 2.5);   // add a bit of extra slack.
            ta.x += (window.pos.x < child_menu_window.pos.x) ? -0.5 : +0.5;                     // to avoid numerical issues (FIXME: ??)
            tb.y = ta.y + ImMax((tb.y - extra) - ta.y, -ref_unit * 8.0);                           // triangle has maximum height to limit the slope and the bias toward large sub-menus
            tc.y = ta.y + ImMin((tc.y + extra) - ta.y, +ref_unit * 8.0);
            moving_toward_child_menu = ImTriangleContainsPoint(ta, tb, tc, g.IO.MousePos);
            //GetForegroundDrawList()->add_triangle_filled(ta, tb, tc, moving_toward_other_child_menu ? IM_COL32(0,128,0,128) : IM_COL32(128,0,0,128)); // [DEBUG]
        }

        // The 'HovereWindow == window' check creates an inconsistency (e.g. moving away from menu slowly tends to hit same window, whereas moving away fast does not)
        // But we also need to not close the top-menu menu when moving over void. Perhaps we should extend the triangle check to a larger polygon.
        // (Remember to test this on begin_popup("A")->BeginMenu("B") sequence which behaves slightly differently as B isn't a Child of A and hovering isn't shared.)
        if (menu_is_open && !hovered && g.HoveredWindow == window && !moving_toward_child_menu)
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
        else if (g.NavId == id && g.NavMoveDir == ImGuiDir_Down) // Nav-down to open
        {
            want_open = true;
            NavMoveRequestCancel();
        }
    }

    if (!enabled) // explicitly close if an open menu becomes disabled, facilitate users code a lot in pattern such as 'if (BeginMenu("options", has_object)) { ..use object.. }'
        want_close = true;
    if (want_close && is_popup_open(id, ImGuiPopupFlags_None))
        ClosePopupToLevel(g.begin_popupStack.Size, true);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.last_item_data.StatusFlags | ItemStatusFlags::Openable | (menu_is_open ? ItemStatusFlags::Opened : 0));
    pop_id();

    if (!menu_is_open && want_open && g.open_popupStack.Size > g.begin_popupStack.Size)
    {
        // Don't recycle same menu level in the same frame, first close the other menu and yield for a frame.
        open_popup(label);
        return false;
    }

    menu_is_open |= want_open;
    if (want_open)
        open_popup(label);

    if (menu_is_open)
    {
        set_next_window_pos(popup_pos, ImGuiCond_Always); // Note: this is super misleading! The value will serve as reference for FindBestWindowPosForPopup(), not actual pos.
        PushStyleVar(ImGuiStyleVar_ChildRounding, style.PopupRounding); // First level will use _PopupRounding, subsequent will use _ChildRounding
        menu_is_open = begin_popupEx(id, flags); // menu_is_open can be 'false' when the popup is completely clipped (e.g. zero size display)
        PopStyleVar();
    }
    else
    {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
    }

    return menu_is_open;
}

bool ImGui::BeginMenu(const char* label, bool enabled)
{
    return BeginMenuEx(label, None, enabled);
}

void ImGui::EndMenu()
{
    // Nav: When a left move request _within our child menu_ failed, close ourselves (the _parent_ menu).
    // A menu doesn't close itself because EndMenuBar() wants the catch the last Left<>Right inputs.
    // However, it means that with the current code, a BeginMenu() from outside another menu or a menu-bar won't be closable with the Left direction.
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    if (g.NavMoveDir == ImGuiDir_Left && NavMoveRequestButNoResultYet() && window.DC.layout_type == ImGuiLayoutType_Vertical)
        if (g.NavWindow && (g.NavWindow->RootWindowForNav->Flags & WindowFlags_Popup) && g.NavWindow->RootWindowForNav->ParentWindow == window)
        {
            ClosePopupToLevel(g.begin_popupStack.Size, true);
            NavMoveRequestCancel();
        }

    end_popup();
}

bool ImGui::menu_itemEx(const char* label, const char* icon, const char* shortcut, bool selected, bool enabled)
{
    Window* window = GetCurrentWindow();
    if (window.SkipItems)
        return false;

    // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;
    Vector2D pos = window.DC.CursorPos;
    Vector2D label_size = CalcTextSize(label, None, true);

    const bool menuset_is_open = IsRootOfOpenMenuSet();
    Window* backed_nav_window = g.NavWindow;
    if (menuset_is_open)
        g.NavWindow = window;

    // We've been using the equivalent of ImGuiselectableFlags_SetNavIdOnHover on all selectable() since early Nav system days (commit 43ee5d73),
    // but I am unsure whether this should be kept at all. For now moved it to be an opt-in feature used by menus only.
    bool pressed;
    push_id(label);
    if (!enabled)
        BeginDisabled();

    const ImGuiselectableFlags selectable_flags = ImGuiselectableFlags_SelectOnRelease | ImGuiselectableFlags_SetNavIdOnHover;
    const ImGuiMenuColumns* offsets = &window.DC.MenuColumns;
    if (window.DC.layout_type == LayoutType::Horizontal)
    {
        // Mimic the exact layout spacing of BeginMenu() to allow menu_item() inside a menu bar, which is a little misleading but may be useful
        // Note that in this situation: we don't render the shortcut, we render a highlight instead of the selected tick mark.
        let w =  label_size.x;
        window.DC.CursorPos.x += IM_FLOOR(style.item_spacing.x * 0.5);
        Vector2D text_pos(window.DC.CursorPos.x + offsets->OffsetLabel, window.DC.CursorPos.y + window.DC.curr_line_text_base_offset);
        PushStyleVar(ImGuiStyleVar_ItemSpacing, DimgVec2D::new(style.item_spacing.x * 2.0, style.item_spacing.y));
        pressed = selectable("", selected, selectable_flags, DimgVec2D::new(w, 0.0));
        PopStyleVar();
        render_text(text_pos, label);
        window.DC.CursorPos.x += IM_FLOOR(style.item_spacing.x * (-1.0 + 0.5)); // -1 spacing to compensate the spacing added when selectable() did a same_line(). It would also work to call same_line() ourselves after the PopStyleVar().
    }
    else
    {
        // Menu item inside a vertical menu
        // (In a typical menu window where all items are BeginMenu() or menu_item() calls, extra_w will always be 0.0.
        //  Only when they are other items sticking out we're going to add spacing, yet only register minimum width into the layout system.
        let icon_w =  (icon && icon[0]) ? CalcTextSize(icon, None).x : 0.0;
        let shortcut_w =  (shortcut && shortcut[0]) ? CalcTextSize(shortcut, None).x : 0.0;
        let checkmark_w =  IM_FLOOR(g.FontSize * 1.20);
        let min_w =  window.DC.MenuColumns.DeclColumns(icon_w, label_size.x, shortcut_w, checkmark_w); // Feedback for next frame
        let stretch_w =  ImMax(0.0, get_content_region_avail().x - min_w);
        pressed = selectable("", false, selectable_flags | ImGuiselectableFlags_SpanAvailWidth, DimgVec2D::new(min_w, 0.0));
        render_text(pos + DimgVec2D::new(offsets->OffsetLabel, 0.0), label);
        if (icon_w > 0.0)
            render_text(pos + DimgVec2D::new(offsets->OffsetIcon, 0.0), icon);
        if (shortcut_w > 0.0)
        {
            PushStyleColor(ImGuiCol_Text, style.Colors[ImGuiCol_TextDisabled]);
            render_text(pos + DimgVec2D::new(offsets->OffsetShortcut + stretch_w, 0.0), shortcut, None, false);
            PopStyleColor();
        }
        if (selected)
            RenderCheckMark(window.draw_list, pos + DimgVec2D::new(offsets->OffsetMark + stretch_w + g.FontSize * 0.40, g.FontSize * 0.134 * 0.5), GetColorU32(ImGuiCol_Text), g.FontSize  * 0.866);
    }
    IMGUI_TEST_ENGINE_ITEM_INFO(g.last_item_data.id, label, g.last_item_data.StatusFlags | ItemStatusFlags::Checkable | (selected ? ItemStatusFlags::Checked : 0));
    if (!enabled)
        EndDisabled();
    pop_id();
    if (menuset_is_open)
        g.NavWindow = backed_nav_window;

    return pressed;
}

bool ImGui::menu_item(const char* label, const char* shortcut, bool selected, bool enabled)
{
    return menu_itemEx(label, None, shortcut, selected, enabled);
}

bool ImGui::menu_item(const char* label, const char* shortcut, bool* p_selected, bool enabled)
{
    if (menu_itemEx(label, None, shortcut, p_selected ? *p_selected : false, enabled))
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
// - tab_bar_remove_tab() [Internal]
// - TabBarCloseTab() [Internal]
// - TabBarScrollClamp() [Internal]
// - TabBarScrollToTab() [Internal]
// - TabBarQueueChangeTabOrder() [Internal]
// - TabBarScrollingButtons() [Internal]
// - TabBarTabListPopupButton() [Internal]
//-------------------------------------------------------------------------

struct ImGuiTabBarSection
{
    int                 TabCount;               // Number of tabs in this section.
    float               Width;                  // Sum of width of tabs in this section (after shrinking down)
    float               Spacing;                // Horizontal spacing at the end of the section.

    ImGuiTabBarSection() { memset(this, 0, sizeof(*this)); }
};

namespace ImGui
{
    static void             TabBarLayout(ImGuiTabBar* tab_bar);
    static ImU32            TabBarCalcTabID(ImGuiTabBar* tab_bar, const char* label, Window* docked_window);
    static float            TabBarCalcMaxTabWidth();
    static float            TabBarScrollClamp(ImGuiTabBar* tab_bar, float scrolling);
    static void             TabBarScrollToTab(ImGuiTabBar* tab_bar, Id32 tab_id, ImGuiTabBarSection* sections);
    static ImGuiTabItem*    TabBarScrollingButtons(ImGuiTabBar* tab_bar);
    static ImGuiTabItem*    TabBarTabListPopupButton(ImGuiTabBar* tab_bar);
}

ImGuiTabBar::ImGuiTabBar()
{
    memset(this, 0, sizeof(*this));
    CurrFrameVisible = PrevFrameVisible = -1;
    LastTabItemIdx = -1;
}

static inline int TabItemGetSectionIdx(const ImGuiTabItem* tab)
{
    return (tab->Flags & TabItemFlags::Leading) ? 0 : (tab->Flags & TabItemFlags::Trailing) ? 2 : 1;
}

static int IMGUI_CDECL TabItemComparerBySection(const void* lhs, const void* rhs)
{
    const ImGuiTabItem* a = lhs;
    const ImGuiTabItem* b = rhs;
    let a_section = TabItemGetSectionIdx(a);
    let b_section = TabItemGetSectionIdx(b);
    if (a_section != b_section)
        return a_section - b_section;
    return (a->IndexDuringLayout - b->IndexDuringLayout);
}

static int IMGUI_CDECL TabItemComparerByBeginOrder(const void* lhs, const void* rhs)
{
    const ImGuiTabItem* a = lhs;
    const ImGuiTabItem* b = rhs;
    return (a->BeginOrder - b->BeginOrder);
}

static ImGuiTabBar* GetTabBarFromTabBarRef(const ImGuiPtrOrIndex& ref)
{
    // ImGuiContext& g = *GImGui;
    return ref.Ptr ? (ImGuiTabBar*)ref.Ptr : g.tab_bars.GetByIndex(ref.Index);
}

static ImGuiPtrOrIndex GetTabBarRefFromTabBar(ImGuiTabBar* tab_bar)
{
    // ImGuiContext& g = *GImGui;
    if (g.tab_bars.contains(tab_bar))
        return ImGuiPtrOrIndex(g.tab_bars.GetIndex(tab_bar));
    return ImGuiPtrOrIndex(tab_bar);
}

bool    ImGui::BeginTabBar(const char* str_id, ImGuiTabBarFlags flags)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    if (window.SkipItems)
        return false;

    Id32 id = window.GetID(str_id);
    ImGuiTabBar* tab_bar = g.tab_bars.GetOrAddByKey(id);
    ImRect tab_bar_bb = ImRect(window.DC.CursorPos.x, window.DC.CursorPos.y, window.work_rect.Max.x, window.DC.CursorPos.y + g.FontSize + g.Style.FramePadding.y * 2);
    tab_bar->ID = id;
    return begin_tab_bar_ex(tab_bar, tab_bar_bb, flags | TabBarFlags::IsFocused, None);
}

bool    ImGui::begin_tab_bar_ex(ImGuiTabBar* tab_bar, const ImRect& tab_bar_bb, ImGuiTabBarFlags flags, ImGuiDockNode* dock_node)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    if (window.SkipItems)
        return false;

    if ((flags & TabBarFlags::DockNode) == 0)
        push_override_id(tab_bar->ID);

    // Add to stack
    g.CurrentTabBarStack.push_back(GetTabBarRefFromTabBar(tab_bar));
    g.CurrentTabBar = tab_bar;

    // Append with multiple BeginTabBar()/EndTabBar() pairs.
    tab_bar->backup_cursor_pos = window.DC.CursorPos;
    if (tab_bar->CurrFrameVisible == g.FrameCount)
    {
        window.DC.CursorPos = DimgVec2D::new(tab_bar->BarRect.Min.x, tab_bar->BarRect.Max.y + tab_bar->ItemSpacingY);
        tab_bar->BeginCount += 1;
        return true;
    }

    // Ensure correct ordering when toggling ImGuiTabBarFlags_Reorderable flag, or when a new tab was added while being not reorderable
    if ((flags & TabBarFlags::Reorderable) != (tab_bar->Flags & TabBarFlags::Reorderable) || (tab_bar->TabsAddedNew && !(flags & TabBarFlags::Reorderable)))
        if ((flags & TabBarFlags::DockNode) == 0) // FIXME: tab_bar with dock_node can now be hybrid
            ImQsort(tab_bar->Tabs.Data, tab_bar->Tabs.Size, sizeof(ImGuiTabItem), TabItemComparerByBeginOrder);
    tab_bar->TabsAddedNew = false;

    // flags
    if ((flags & TabBarFlags::FittingPolicyMask_) == 0)
        flags |= TabBarFlags::FittingPolicyDefault_;

    tab_bar->Flags = flags;
    tab_bar->BarRect = tab_bar_bb;
    tab_bar->WantLayout = true; // Layout will be done on the first call to ItemTab()
    tab_bar->PrevFrameVisible = tab_bar->CurrFrameVisible;
    tab_bar->CurrFrameVisible = g.FrameCount;
    tab_bar->PrevTabsContentsHeight = tab_bar->CurrTabsContentsHeight;
    tab_bar->CurrTabsContentsHeight = 0.0;
    tab_bar->ItemSpacingY = g.Style.item_spacing.y;
    tab_bar->FramePadding = g.Style.FramePadding;
    tab_bar->TabsActiveCount = 0;
    tab_bar->BeginCount = 1;

    // Set cursor pos in a way which only be used in the off-chance the user erroneously submits item before BeginTabItem(): items will overlap
    window.DC.CursorPos = DimgVec2D::new(tab_bar->BarRect.Min.x, tab_bar->BarRect.Max.y + tab_bar->ItemSpacingY);

    // Draw separator
    const ImU32 col = GetColorU32((flags & TabBarFlags::IsFocused) ? ImGuiCol_TabActive : ImGuiCol_TabUnfocusedActive);
    let y = tab_bar->BarRect.Max.y - 1.0;
    if (dock_node != None)
    {
        let separator_min_x = dock_node->Pos.x + window.WindowBorderSize;
        let separator_max_x = dock_node->Pos.x + dock_node->Size.x - window.WindowBorderSize;
        window.draw_list->AddLine(DimgVec2D::new(separator_min_x, y), DimgVec2D::new(separator_max_x, y), col, 1.0);
    }
    else
    {
        let separator_min_x = tab_bar->BarRect.Min.x - IM_FLOOR(window.WindowPadding.x * 0.5);
        let separator_max_x = tab_bar->BarRect.Max.x + IM_FLOOR(window.WindowPadding.x * 0.5);
        window.draw_list->AddLine(DimgVec2D::new(separator_min_x, y), DimgVec2D::new(separator_max_x, y), col, 1.0);
    }
    return true;
}

void    ImGui::end_tab_bar()
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    if (window.SkipItems)
        return;

    ImGuiTabBar* tab_bar = g.CurrentTabBar;
    if (tab_bar == None)
    {
        IM_ASSERT_USER_ERROR(tab_bar != None, "Mismatched BeginTabBar()/EndTabBar()!");
        return;
    }

    // Fallback in case no TabItem have been submitted
    if (tab_bar->WantLayout)
        TabBarLayout(tab_bar);

    // Restore the last visible height if no tab is visible, this reduce vertical flicker/movement when a tabs gets removed without calling SetTabItemClosed().
    const bool tab_bar_appearing = (tab_bar->PrevFrameVisible + 1 < g.FrameCount);
    if (tab_bar->VisibleTabWasSubmitted || tab_bar->visible_tab_id == 0 || tab_bar_appearing)
    {
        tab_bar->CurrTabsContentsHeight = ImMax(window.DC.CursorPos.y - tab_bar->BarRect.Max.y, tab_bar->CurrTabsContentsHeight);
        window.DC.CursorPos.y = tab_bar->BarRect.Max.y + tab_bar->CurrTabsContentsHeight;
    }
    else
    {
        window.DC.CursorPos.y = tab_bar->BarRect.Max.y + tab_bar->PrevTabsContentsHeight;
    }
    if (tab_bar->BeginCount > 1)
        window.DC.CursorPos = tab_bar->backup_cursor_pos;

    if ((tab_bar->Flags & TabBarFlags::DockNode) == 0)
        pop_id();

    g.CurrentTabBarStack.pop_back();
    g.CurrentTabBar = g.CurrentTabBarStack.empty() ? None : GetTabBarFromTabBarRef(g.CurrentTabBarStack.back());
}

// This is called only once a frame before by the first call to ItemTab()
// The reason we're not calling it in BeginTabBar() is to leave a chance to the user to call the SetTabItemClosed() functions.
static void ImGui::TabBarLayout(ImGuiTabBar* tab_bar)
{
    // ImGuiContext& g = *GImGui;
    tab_bar->WantLayout = false;

    // Garbage collect by compacting list
    // Detect if we need to sort out tab list (e.g. in rare case where a tab changed section)
    int tab_dst_n = 0;
    bool need_sort_by_section = false;
    ImGuiTabBarSection sections[3]; // Layout sections: Leading, Central, Trailing
    for (int tab_src_n = 0; tab_src_n < tab_bar->Tabs.Size; tab_src_n += 1)
    {
        ImGuiTabItem* tab = &tab_bar->Tabs[tab_src_n];
        if (tab->LastFrameVisible < tab_bar->PrevFrameVisible || tab->WantClose)
        {
            // Remove tab
            if (tab_bar->visible_tab_id == tab->ID) { tab_bar->visible_tab_id = 0; }
            if (tab_bar->SelectedTabId == tab->ID) { tab_bar->SelectedTabId = 0; }
            if (tab_bar->NextSelectedTabId == tab->ID) { tab_bar->NextSelectedTabId = 0; }
            continue;
        }
        if (tab_dst_n != tab_src_n)
            tab_bar->Tabs[tab_dst_n] = tab_bar->Tabs[tab_src_n];

        tab = &tab_bar->Tabs[tab_dst_n];
        tab->IndexDuringLayout = (ImS16)tab_dst_n;

        // We will need sorting if tabs have changed section (e.g. moved from one of Leading/Central/Trailing to another)
        int curr_tab_section_n = TabItemGetSectionIdx(tab);
        if (tab_dst_n > 0)
        {
            ImGuiTabItem* prev_tab = &tab_bar->Tabs[tab_dst_n - 1];
            int prev_tab_section_n = TabItemGetSectionIdx(prev_tab);
            if (curr_tab_section_n == 0 && prev_tab_section_n != 0)
                need_sort_by_section = true;
            if (prev_tab_section_n == 2 && curr_tab_section_n != 2)
                need_sort_by_section = true;
        }

        sections[curr_tab_section_n].TabCount += 1;
        tab_dst_n += 1;
    }
    if (tab_bar->Tabs.Size != tab_dst_n)
        tab_bar->Tabs.resize(tab_dst_n);

    if (need_sort_by_section)
        ImQsort(tab_bar->Tabs.Data, tab_bar->Tabs.Size, sizeof(ImGuiTabItem), TabItemComparerBySection);

    // Calculate spacing between sections
    sections[0].Spacing = sections[0].TabCount > 0 && (sections[1].TabCount + sections[2].TabCount) > 0 ? g.Style.ItemInnerSpacing.x : 0.0;
    sections[1].Spacing = sections[1].TabCount > 0 && sections[2].TabCount > 0 ? g.Style.ItemInnerSpacing.x : 0.0;

    // Setup next selected tab
    Id32 scroll_to_tab_id = INVALID_ID;
    if (tab_bar->NextSelectedTabId)
    {
        tab_bar->SelectedTabId = tab_bar->NextSelectedTabId;
        tab_bar->NextSelectedTabId = 0;
        scroll_to_tab_id = tab_bar->SelectedTabId;
    }

    // Process order change request (we could probably process it when requested but it's just saner to do it in a single spot).
    if (tab_bar->ReorderRequestTabId != 0)
    {
        if (TabBarProcessReorder(tab_bar))
            if (tab_bar->ReorderRequestTabId == tab_bar->SelectedTabId)
                scroll_to_tab_id = tab_bar->ReorderRequestTabId;
        tab_bar->ReorderRequestTabId = 0;
    }

    // Tab List Popup (will alter tab_bar->BarRect and therefore the available width!)
    const bool tab_list_popup_button = (tab_bar->Flags & TabBarFlags::TabListPopupButton) != 0;
    if (tab_list_popup_button)
        if (ImGuiTabItem* tab_to_select = TabBarTabListPopupButton(tab_bar)) // NB: Will alter BarRect.min.x!
            scroll_to_tab_id = tab_bar->SelectedTabId = tab_to_select->ID;

    // Leading/Trailing tabs will be shrink only if central one aren't visible anymore, so layout the shrink data as: leading, trailing, central
    // (whereas our tabs are stored as: leading, central, trailing)
    int shrink_buffer_indexes[3] = { 0, sections[0].TabCount + sections[2].TabCount, sections[0].TabCount };
    g.ShrinkWidthBuffer.resize(tab_bar->Tabs.Size);

    // Compute ideal tabs widths + store them into shrink buffer
    ImGuiTabItem* most_recently_selected_tab = None;
    int curr_section_n = -1;
    bool found_selected_tab_id = false;
    for (int tab_n = 0; tab_n < tab_bar->Tabs.Size; tab_n += 1)
    {
        ImGuiTabItem* tab = &tab_bar->Tabs[tab_n];
        IM_ASSERT(tab->LastFrameVisible >= tab_bar->PrevFrameVisible);

        if ((most_recently_selected_tab == None || most_recently_selected_tab->LastFrameSelected < tab->LastFrameSelected) && !(tab->Flags & TabItemFlags::Button))
            most_recently_selected_tab = tab;
        if (tab->ID == tab_bar->SelectedTabId)
            found_selected_tab_id = true;
        if (scroll_to_tab_id == 0 && g.NavJustMovedToId == tab->ID)
            scroll_to_tab_id = tab->ID;

        // Refresh tab width immediately, otherwise changes of style e.g. style.FramePadding.x would noticeably lag in the tab bar.
        // Additionally, when using TabBarAddTab() to manipulate tab bar order we occasionally insert new tabs that don't have a width yet,
        // and we cannot wait for the next BeginTabItem() call. We cannot compute this width within TabBarAddTab() because font size depends on the active window.
        const char* tab_name = tab_bar->get_tab_name(tab);
        const bool has_close_button = (tab->Flags & TabItemFlags::NoCloseButton) ? false : true;
        tab->ContentWidth = (tab->RequestedWidth > 0.0) ? tab->RequestedWidth : tab_item_calc_size(tab_name, has_close_button).x;

        int section_n = TabItemGetSectionIdx(tab);
        ImGuiTabBarSection* section = &sections[section_n];
        section->Width += tab->ContentWidth + (section_n == curr_section_n ? g.Style.ItemInnerSpacing.x : 0.0);
        curr_section_n = section_n;

        // Store data so we can build an array sorted by width if we need to shrink tabs down
        IM_MSVC_WARNING_SUPPRESS(6385);
        ImGuiShrinkWidthItem* shrink_width_item = &g.ShrinkWidthBuffer[shrink_buffer_indexes[section_n] += 1];
        shrink_width_item->Index = tab_n;
        shrink_width_item->Width = shrink_width_item->InitialWidth = tab->ContentWidth;

        IM_ASSERT(tab->ContentWidth > 0.0);
        tab->Width = tab->ContentWidth;
    }

    // Compute total ideal width (used for e.g. auto-resizing a window)
    tab_bar->WidthAllTabsIdeal = 0.0;
    for (int section_n = 0; section_n < 3; section_n += 1)
        tab_bar->WidthAllTabsIdeal += sections[section_n].width + sections[section_n].Spacing;

    // Horizontal scrolling buttons
    // (note that TabBarScrollButtons() will alter BarRect.max.x)
    if ((tab_bar->WidthAllTabsIdeal > tab_bar->BarRect.GetWidth() && tab_bar->Tabs.Size > 1) && !(tab_bar->Flags & TabBarFlags::NoTabListScrollingButtons) && (tab_bar->Flags & TabBarFlags::FittingPolicyScroll))
        if (ImGuiTabItem* scroll_and_select_tab = TabBarScrollingButtons(tab_bar))
        {
            scroll_to_tab_id = scroll_and_select_tab->ID;
            if ((scroll_and_select_tab->Flags & TabItemFlags::Button) == 0)
                tab_bar->SelectedTabId = scroll_to_tab_id;
        }

    // Shrink widths if full tabs don't fit in their allocated space
    let section_0_w =  sections[0].width + sections[0].Spacing;
    let section_1_w =  sections[1].width + sections[1].Spacing;
    let section_2_w =  sections[2].width + sections[2].Spacing;
    bool central_section_is_visible = (section_0_w + section_2_w) < tab_bar->BarRect.GetWidth();
    float width_excess;
    if (central_section_is_visible)
        width_excess = ImMax(section_1_w - (tab_bar->BarRect.GetWidth() - section_0_w - section_2_w), 0.0); // Excess used to shrink central section
    else
        width_excess = (section_0_w + section_2_w) - tab_bar->BarRect.GetWidth(); // Excess used to shrink leading/trailing section

    // With ImGuiTabBarFlags_FittingPolicyScroll policy, we will only shrink leading/trailing if the central section is not visible anymore
    if (width_excess > 0.0 && ((tab_bar->Flags & TabBarFlags::FittingPolicyResizeDown) || !central_section_is_visible))
    {
        int shrink_data_count = (central_section_is_visible ? sections[1].TabCount : sections[0].TabCount + sections[2].TabCount);
        int shrink_data_offset = (central_section_is_visible ? sections[0].TabCount + sections[2].TabCount : 0);
        ShrinkWidths(g.ShrinkWidthBuffer.Data + shrink_data_offset, shrink_data_count, width_excess);

        // Apply shrunk values into tabs and sections
        for (int tab_n = shrink_data_offset; tab_n < shrink_data_offset + shrink_data_count; tab_n += 1)
        {
            ImGuiTabItem* tab = &tab_bar->Tabs[g.ShrinkWidthBuffer[tab_n].Index];
            let shrinked_width =  IM_FLOOR(g.ShrinkWidthBuffer[tab_n].width);
            if (shrinked_width < 0.0)
                continue;

            int section_n = TabItemGetSectionIdx(tab);
            sections[section_n].width -= (tab->Width - shrinked_width);
            tab->Width = shrinked_width;
        }
    }

    // Layout all active tabs
    int section_tab_index = 0;
    let tab_offset =  0.0;
    tab_bar->WidthAllTabs = 0.0;
    for (int section_n = 0; section_n < 3; section_n += 1)
    {
        ImGuiTabBarSection* section = &sections[section_n];
        if (section_n == 2)
            tab_offset = ImMin(ImMax(0.0, tab_bar->BarRect.GetWidth() - section->Width), tab_offset);

        for (int tab_n = 0; tab_n < section->TabCount; tab_n += 1)
        {
            ImGuiTabItem* tab = &tab_bar->Tabs[section_tab_index + tab_n];
            tab->Offset = tab_offset;
            tab->NameOffset = -1;
            tab_offset += tab->Width + (tab_n < section->TabCount - 1 ? g.Style.ItemInnerSpacing.x : 0.0);
        }
        tab_bar->WidthAllTabs += ImMax(section->Width + section->Spacing, 0.0);
        tab_offset += section->Spacing;
        section_tab_index += section->TabCount;
    }

    // clear name buffers
    tab_bar->TabsNames.Buf.resize(0);

    // If we have lost the selected tab, select the next most recently active one
    if (found_selected_tab_id == false)
        tab_bar->SelectedTabId = 0;
    if (tab_bar->SelectedTabId == 0 && tab_bar->NextSelectedTabId == 0 && most_recently_selected_tab != None)
        scroll_to_tab_id = tab_bar->SelectedTabId = most_recently_selected_tab->ID;

    // Lock in visible tab
    tab_bar->visible_tab_id = tab_bar->SelectedTabId;
    tab_bar->VisibleTabWasSubmitted = false;

    // CTRL+TAB can override visible tab temporarily
    if (g.NavWindowingTarget != None && g.NavWindowingTarget->DockNode && g.NavWindowingTarget->DockNode->TabBar == tab_bar)
        tab_bar->visible_tab_id = scroll_to_tab_id = g.NavWindowingTarget->ID;

    // Update scrolling
    if (scroll_to_tab_id != 0)
        TabBarScrollToTab(tab_bar, scroll_to_tab_id, sections);
    tab_bar->ScrollingAnim = TabBarScrollClamp(tab_bar, tab_bar->ScrollingAnim);
    tab_bar->ScrollingTarget = TabBarScrollClamp(tab_bar, tab_bar->ScrollingTarget);
    if (tab_bar->ScrollingAnim != tab_bar->ScrollingTarget)
    {
        // Scrolling speed adjust itself so we can always reach our target in 1/3 seconds.
        // Teleport if we are aiming far off the visible line
        tab_bar->ScrollingSpeed = ImMax(tab_bar->ScrollingSpeed, 70.0 * g.FontSize);
        tab_bar->ScrollingSpeed = ImMax(tab_bar->ScrollingSpeed, ImFabs(tab_bar->ScrollingTarget - tab_bar->ScrollingAnim) / 0.3);
        const bool teleport = (tab_bar->PrevFrameVisible + 1 < g.FrameCount) || (tab_bar->ScrollingTargetDistToVisibility > 10.0 * g.FontSize);
        tab_bar->ScrollingAnim = teleport ? tab_bar->ScrollingTarget : ImLinearSweep(tab_bar->ScrollingAnim, tab_bar->ScrollingTarget, g.IO.DeltaTime * tab_bar->ScrollingSpeed);
    }
    else
    {
        tab_bar->ScrollingSpeed = 0.0;
    }
    tab_bar->ScrollingRectMinX = tab_bar->BarRect.Min.x + sections[0].width + sections[0].Spacing;
    tab_bar->ScrollingRectMaxX = tab_bar->BarRect.Max.x - sections[2].width - sections[1].Spacing;

    // Actual layout in host window (we don't do it in BeginTabBar() so as not to waste an extra frame)
    Window* window = g.current_window_id;
    window.DC.CursorPos = tab_bar->BarRect.Min;
    ItemSize(DimgVec2D::new(tab_bar->WidthAllTabs, tab_bar->BarRect.GetHeight()), tab_bar->FramePadding.y);
    window.DC.IdealMaxPos.x = ImMax(window.DC.IdealMaxPos.x, tab_bar->BarRect.Min.x + tab_bar->WidthAllTabsIdeal);
}

// Dockable uses name/id in the global namespace. Non-dockable items use the id stack.
static ImU32   ImGui::TabBarCalcTabID(ImGuiTabBar* tab_bar, const char* label, Window* docked_window)
{
    if (docked_window != None)
    {
        IM_UNUSED(tab_bar);
        IM_ASSERT(tab_bar->Flags & TabBarFlags::DockNode);
        Id32 id = docked_window.tab_id;
        KeepAliveID(id);
        return id;
    }
    else
    {
        Window* window = GImGui->CurrentWindow;
        return window.GetID(label);
    }
}

static float ImGui::TabBarCalcMaxTabWidth()
{
    // ImGuiContext& g = *GImGui;
    return g.FontSize * 20.0;
}

ImGuiTabItem* ImGui::tab_bar_find_tab_by_id(ImGuiTabBar* tab_bar, Id32 tab_id)
{
    if (tab_id != 0)
        for (int n = 0; n < tab_bar->Tabs.Size; n += 1)
            if (tab_bar->Tabs[n].id == tab_id)
                return &tab_bar->Tabs[n];
    return None;
}

// FIXME: See references to #2304 in TODO.txt
ImGuiTabItem* ImGui::TabBarFindMostRecentlySelectedTabForActiveWindow(ImGuiTabBar* tab_bar)
{
    ImGuiTabItem* most_recently_selected_tab = None;
    for (int tab_n = 0; tab_n < tab_bar->Tabs.Size; tab_n += 1)
    {
        ImGuiTabItem* tab = &tab_bar->Tabs[tab_n];
        if (most_recently_selected_tab == None || most_recently_selected_tab->LastFrameSelected < tab->LastFrameSelected)
            if (tab->Window && tab->Window->WasActive)
                most_recently_selected_tab = tab;
    }
    return most_recently_selected_tab;
}

// The purpose of this call is to register tab in advance so we can control their order at the time they appear.
// Otherwise calling this is unnecessary as tabs are appending as needed by the BeginTabItem() function.
void ImGui::tab_bar_add_tab(ImGuiTabBar* tab_bar, ImGuiTabItemFlags tab_flags, Window* window)
{
    // ImGuiContext& g = *GImGui;
    IM_ASSERT(tab_bar_find_tab_by_id(tab_bar, window.tab_id) == None);
    IM_ASSERT(g.CurrentTabBar != tab_bar);  // Can't work while the tab bar is active as our tab doesn't have an x offset yet, in theory we could/should test something like (tab_bar->CurrFrameVisible < g.frame_count) but we'd need to solve why triggers the commented early-out assert in BeginTabBarEx() (probably dock node going from implicit to explicit in same frame)

    if (!window.has_close_button)
        tab_flags |= TabItemFlags::NoCloseButton;       // Set _NoCloseButton immediately because it will be used for first-frame width calculation.

    ImGuiTabItem new_tab;
    new_tab.id = window.tab_id;
    new_tab.Flags = tab_flags;
    new_tab.LastFrameVisible = tab_bar->CurrFrameVisible;   // Required so BeginTabBar() doesn't ditch the tab
    if (new_tab.LastFrameVisible == -1)
        new_tab.LastFrameVisible = g.FrameCount - 1;
    new_tab.Window = window;                                // Required so tab bar layout can compute the tab width before tab submission
    tab_bar->Tabs.push_back(new_tab);
}

// The *tab_id fields be already set by the docking system _before_ the actual TabItem was created, so we clear them regardless.
void ImGui::tab_bar_remove_tab(ImGuiTabBar* tab_bar, Id32 tab_id)
{
    if (ImGuiTabItem* tab = tab_bar_find_tab_by_id(tab_bar, tab_id))
        tab_bar->Tabs.erase(tab);
    if (tab_bar->visible_tab_id == tab_id)      { tab_bar->visible_tab_id = 0; }
    if (tab_bar->SelectedTabId == tab_id)     { tab_bar->SelectedTabId = 0; }
    if (tab_bar->NextSelectedTabId == tab_id) { tab_bar->NextSelectedTabId = 0; }
}

// Called on manual closure attempt
void ImGui::tab_bar_close_tab(ImGuiTabBar* tab_bar, ImGuiTabItem* tab)
{
    IM_ASSERT(!(tab->Flags & TabItemFlags::Button));
    if (!(tab->Flags & TabItemFlags::UnsavedDocument))
    {
        // This will remove a frame of lag for selecting another tab on closure.
        // However we don't run it in the case where the 'Unsaved' flag is set, so user gets a chance to fully undo the closure
        tab->WantClose = true;
        if (tab_bar->visible_tab_id == tab->ID)
        {
            tab->LastFrameVisible = -1;
            tab_bar->SelectedTabId = tab_bar->NextSelectedTabId = 0;
        }
    }
    else
    {
        // Actually select before expecting closure attempt (on an UnsavedDocument tab user is expect to e.g. show a popup)
        if (tab_bar->visible_tab_id != tab->ID)
            tab_bar->NextSelectedTabId = tab->ID;
    }
}

static float ImGui::TabBarScrollClamp(ImGuiTabBar* tab_bar, float scrolling)
{
    scrolling = ImMin(scrolling, tab_bar->WidthAllTabs - tab_bar->BarRect.GetWidth());
    return ImMax(scrolling, 0.0);
}

// Note: we may scroll to tab that are not selected! e.g. using keyboard arrow keys
static void ImGui::TabBarScrollToTab(ImGuiTabBar* tab_bar, Id32 tab_id, ImGuiTabBarSection* sections)
{
    ImGuiTabItem* tab = tab_bar_find_tab_by_id(tab_bar, tab_id);
    if (tab == None)
        return;
    if (tab->Flags & TabItemFlags::SectionMask_)
        return;

    // ImGuiContext& g = *GImGui;
    let margin =  g.FontSize * 1.0; // When to scroll to make Tab N+1 visible always make a bit of N visible to suggest more scrolling area (since we don't have a scrollbar)
    int order = tab_bar->GetTabOrder(tab);

    // Scrolling happens only in the central section (leading/trailing sections are not scrolling)
    // FIXME: This is all confusing.
    let scrollable_width =  tab_bar->BarRect.GetWidth() - sections[0].width - sections[2].width - sections[1].Spacing;

    // We make all tabs positions all relative Sections[0].width to make code simpler
    let tab_x1 =  tab->Offset - sections[0].width + (order > sections[0].TabCount - 1 ? -margin : 0.0);
    let tab_x2 =  tab->Offset - sections[0].width + tab->Width + (order + 1 < tab_bar->Tabs.Size - sections[2].TabCount ? margin : 1.0);
    tab_bar->ScrollingTargetDistToVisibility = 0.0;
    if (tab_bar->ScrollingTarget > tab_x1 || (tab_x2 - tab_x1 >= scrollable_width))
    {
        // scroll to the left
        tab_bar->ScrollingTargetDistToVisibility = ImMax(tab_bar->ScrollingAnim - tab_x2, 0.0);
        tab_bar->ScrollingTarget = tab_x1;
    }
    else if (tab_bar->ScrollingTarget < tab_x2 - scrollable_width)
    {
        // scroll to the right
        tab_bar->ScrollingTargetDistToVisibility = ImMax((tab_x1 - scrollable_width) - tab_bar->ScrollingAnim, 0.0);
        tab_bar->ScrollingTarget = tab_x2 - scrollable_width;
    }
}

void ImGui::TabBarQueueReorder(ImGuiTabBar* tab_bar, const ImGuiTabItem* tab, int offset)
{
    IM_ASSERT(offset != 0);
    IM_ASSERT(tab_bar->ReorderRequestTabId == 0);
    tab_bar->ReorderRequestTabId = tab->ID;
    tab_bar->ReorderRequestOffset = (ImS16)offset;
}

void ImGui::TabBarQueueReorderFromMousePos(ImGuiTabBar* tab_bar, const ImGuiTabItem* src_tab, Vector2D mouse_pos)
{
    // ImGuiContext& g = *GImGui;
    IM_ASSERT(tab_bar->ReorderRequestTabId == 0);
    if ((tab_bar->Flags & TabBarFlags::Reorderable) == 0)
        return;

    const bool is_central_section = (src_tab->Flags & TabItemFlags::SectionMask_) == 0;
    let bar_offset = tab_bar->BarRect.Min.x - (is_central_section ? tab_bar->ScrollingTarget : 0);

    // count number of contiguous tabs we are crossing over
    let dir = (bar_offset + src_tab->Offset) > mouse_pos.x ? -1 : +1;
    let src_idx = tab_bar->Tabs.index_from_ptr(src_tab);
    int dst_idx = src_idx;
    for (int i = src_idx; i >= 0 && i < tab_bar->Tabs.Size; i += dir)
    {
        // Reordered tabs must share the same section
        const ImGuiTabItem* dst_tab = &tab_bar->Tabs[i];
        if (dst_tab->Flags & TabItemFlags::NoReorder)
            break;
        if ((dst_tab->Flags & TabItemFlags::SectionMask_) != (src_tab->Flags & TabItemFlags::SectionMask_))
            break;
        dst_idx = i;

        // Include spacing after tab, so when mouse cursor is between tabs we would not continue checking further tabs that are not hovered.
        let x1 = bar_offset + dst_tab->Offset - g.Style.ItemInnerSpacing.x;
        let x2 = bar_offset + dst_tab->Offset + dst_tab->Width + g.Style.ItemInnerSpacing.x;
        //GetForegroundDrawList()->add_rect(Vector2D(x1, tab_bar->BarRect.min.y), Vector2D(x2, tab_bar->BarRect.max.y), IM_COL32(255, 0, 0, 255));
        if ((dir < 0 && mouse_pos.x > x1) || (dir > 0 && mouse_pos.x < x2))
            break;
    }

    if (dst_idx != src_idx)
        TabBarQueueReorder(tab_bar, src_tab, dst_idx - src_idx);
}

bool ImGui::TabBarProcessReorder(ImGuiTabBar* tab_bar)
{
    ImGuiTabItem* tab1 = tab_bar_find_tab_by_id(tab_bar, tab_bar->ReorderRequestTabId);
    if (tab1 == None || (tab1->Flags & TabItemFlags::NoReorder))
        return false;

    //IM_ASSERT(tab_bar->flags & ImGuiTabBarFlags_Reorderable); // <- this may happen when using debug tools
    int tab2_order = tab_bar->GetTabOrder(tab1) + tab_bar->ReorderRequestOffset;
    if (tab2_order < 0 || tab2_order >= tab_bar->Tabs.Size)
        return false;

    // Reordered tabs must share the same section
    // (Note: TabBarQueueReorderFromMousePos() also has a similar test but since we allow direct calls to TabBarQueueReorder() we do it here too)
    ImGuiTabItem* tab2 = &tab_bar->Tabs[tab2_order];
    if (tab2->Flags & TabItemFlags::NoReorder)
        return false;
    if ((tab1->Flags & TabItemFlags::SectionMask_) != (tab2->Flags & TabItemFlags::SectionMask_))
        return false;

    ImGuiTabItem item_tmp = *tab1;
    ImGuiTabItem* src_tab = (tab_bar->ReorderRequestOffset > 0) ? tab1 + 1 : tab2;
    ImGuiTabItem* dst_tab = (tab_bar->ReorderRequestOffset > 0) ? tab1 : tab2 + 1;
    let move_count = (tab_bar->ReorderRequestOffset > 0) ? tab_bar->ReorderRequestOffset : -tab_bar->ReorderRequestOffset;
    memmove(dst_tab, src_tab, move_count * sizeof(ImGuiTabItem));
    *tab2 = item_tmp;

    if (tab_bar->Flags & TabBarFlags::SaveSettings)
        mark_ini_settings_dirty();
    return true;
}

static ImGuiTabItem* ImGui::TabBarScrollingButtons(ImGuiTabBar* tab_bar)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;

    const Vector2D arrow_button_size(g.FontSize - 2.0, g.FontSize + g.Style.FramePadding.y * 2.0);
    let scrolling_buttons_width = arrow_button_size.x * 2.0;

    const Vector2D backup_cursor_pos = window.DC.CursorPos;
    //window->draw_list->add_rect(Vector2D(tab_bar->BarRect.max.x - scrolling_buttons_width, tab_bar->BarRect.min.y), Vector2D(tab_bar->BarRect.max.x, tab_bar->BarRect.max.y), IM_COL32(255,0,0,255));

    int select_dir = 0;
    Vector4D arrow_col = g.Style.Colors[ImGuiCol_Text];
    arrow_col.w *= 0.5;

    PushStyleColor(ImGuiCol_Text, arrow_col);
    PushStyleColor(ImGuiCol_Button, Vector4D(0, 0, 0, 0));
    let backup_repeat_delay = g.IO.KeyRepeatDelay;
    let backup_repeat_rate = g.IO.KeyRepeatRate;
    g.IO.KeyRepeatDelay = 0.250;
    g.IO.KeyRepeatRate = 0.200;
    let x =  ImMax(tab_bar->BarRect.Min.x, tab_bar->BarRect.Max.x - scrolling_buttons_width);
    window.DC.CursorPos = DimgVec2D::new(x, tab_bar->BarRect.Min.y);
    if (ArrowButtonEx("##<", ImGuiDir_Left, arrow_button_size, ImGuiButtonFlags_PressedOnClick | ImGuiButtonFlags_Repeat))
        select_dir = -1;
    window.DC.CursorPos = DimgVec2D::new(x + arrow_button_size.x, tab_bar->BarRect.Min.y);
    if (ArrowButtonEx("##>", ImGuiDir_Right, arrow_button_size, ImGuiButtonFlags_PressedOnClick | ImGuiButtonFlags_Repeat))
        select_dir = +1;
    PopStyleColor(2);
    g.IO.KeyRepeatRate = backup_repeat_rate;
    g.IO.KeyRepeatDelay = backup_repeat_delay;

    ImGuiTabItem* tab_to_scroll_to = None;
    if (select_dir != 0)
        if (ImGuiTabItem* tab_item = tab_bar_find_tab_by_id(tab_bar, tab_bar->SelectedTabId))
        {
            int selected_order = tab_bar->GetTabOrder(tab_item);
            int target_order = selected_order + select_dir;

            // Skip tab item buttons until another tab item is found or end is reached
            while (tab_to_scroll_to == None)
            {
                // If we are at the end of the list, still scroll to make our tab visible
                tab_to_scroll_to = &tab_bar->Tabs[(target_order >= 0 && target_order < tab_bar->Tabs.Size) ? target_order : selected_order];

                // Cross through buttons
                // (even if first/last item is a button, return it so we can update the scroll)
                if (tab_to_scroll_to->Flags & TabItemFlags::Button)
                {
                    target_order += select_dir;
                    selected_order += select_dir;
                    tab_to_scroll_to = (target_order < 0 || target_order >= tab_bar->Tabs.Size) ? tab_to_scroll_to : None;
                }
            }
        }
    window.DC.CursorPos = backup_cursor_pos;
    tab_bar->BarRect.Max.x -= scrolling_buttons_width + 1.0;

    return tab_to_scroll_to;
}

static ImGuiTabItem* ImGui::TabBarTabListPopupButton(ImGuiTabBar* tab_bar)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;

    // We use g.style.FramePadding.y to match the square ArrowButton size
    let tab_list_popup_button_width = g.FontSize + g.Style.FramePadding.y;
    const Vector2D backup_cursor_pos = window.DC.CursorPos;
    window.DC.CursorPos = DimgVec2D::new(tab_bar->BarRect.Min.x - g.Style.FramePadding.y, tab_bar->BarRect.Min.y);
    tab_bar->BarRect.Min.x += tab_list_popup_button_width;

    Vector4D arrow_col = g.Style.Colors[ImGuiCol_Text];
    arrow_col.w *= 0.5;
    PushStyleColor(ImGuiCol_Text, arrow_col);
    PushStyleColor(ImGuiCol_Button, Vector4D(0, 0, 0, 0));
    bool open = BeginCombo("##v", None, ImGuiComboFlags_NoPreview | ImGuiComboFlags_HeightLargest);
    PopStyleColor(2);

    ImGuiTabItem* tab_to_select = None;
    if (open)
    {
        for (int tab_n = 0; tab_n < tab_bar->Tabs.Size; tab_n += 1)
        {
            ImGuiTabItem* tab = &tab_bar->Tabs[tab_n];
            if (tab->Flags & TabItemFlags::Button)
                continue;

            const char* tab_name = tab_bar->get_tab_name(tab);
            if (selectable(tab_name, tab_bar->SelectedTabId == tab->ID))
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
// - tab_item_calc_size() [Internal]
// - tab_item_background() [Internal]
// - tab_item_label_and_close_button() [Internal]
//-------------------------------------------------------------------------

bool    ImGui::BeginTabItem(const char* label, bool* p_open, ImGuiTabItemFlags flags)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    if (window.SkipItems)
        return false;

    ImGuiTabBar* tab_bar = g.CurrentTabBar;
    if (tab_bar == None)
    {
        IM_ASSERT_USER_ERROR(tab_bar, "Needs to be called between BeginTabBar() and EndTabBar()!");
        return false;
    }
    IM_ASSERT((flags & TabItemFlags::Button) == 0);             // BeginTabItem() Can't be used with button flags, use TabItemButton() instead!

    bool ret = tab_item_ex(tab_bar, label, p_open, flags, None);
    if (ret && !(flags & TabItemFlags::NoPushId))
    {
        ImGuiTabItem* tab = &tab_bar->Tabs[tab_bar->LastTabItemIdx];
        push_override_id(tab->ID); // We already hashed 'label' so push into the id stack directly instead of doing another hash through push_id(label)
    }
    return ret;
}

void    ImGui::EndTabItem()
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    if (window.SkipItems)
        return;

    ImGuiTabBar* tab_bar = g.CurrentTabBar;
    if (tab_bar == None)
    {
        IM_ASSERT_USER_ERROR(tab_bar != None, "Needs to be called between BeginTabBar() and EndTabBar()!");
        return;
    }
    IM_ASSERT(tab_bar->LastTabItemIdx >= 0);
    ImGuiTabItem* tab = &tab_bar->Tabs[tab_bar->LastTabItemIdx];
    if (!(tab->Flags & TabItemFlags::NoPushId))
        pop_id();
}

bool    ImGui::TabItemButton(const char* label, ImGuiTabItemFlags flags)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window_id;
    if (window.SkipItems)
        return false;

    ImGuiTabBar* tab_bar = g.CurrentTabBar;
    if (tab_bar == None)
    {
        IM_ASSERT_USER_ERROR(tab_bar != None, "Needs to be called between BeginTabBar() and EndTabBar()!");
        return false;
    }
    return tab_item_ex(tab_bar, label, None, flags | TabItemFlags::Button | TabItemFlags::NoReorder, None);
}

bool    ImGui::tab_item_ex(ImGuiTabBar* tab_bar, const char* label, bool* p_open, ImGuiTabItemFlags flags, Window* docked_window)
{
    // Layout whole tab bar if not already done
    // ImGuiContext& g = *GImGui;
    if (tab_bar->WantLayout)
    {
        ImGuiNextItemData backup_next_item_data = g.next_item_data;
        TabBarLayout(tab_bar);
        g.next_item_data = backup_next_item_data;
    }
    Window* window = g.current_window_id;
    if (window.SkipItems)
        return false;

    const ImGuiStyle& style = g.Style;
    const Id32 id = TabBarCalcTabID(tab_bar, label, docked_window);

    // If the user called us with *p_open == false, we early out and don't render.
    // We make a call to ItemAdd() so that attempts to use a contextual popup menu with an implicit id won't use an older id.
    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.last_item_data.StatusFlags);
    if (p_open && !*p_open)
    {
        ItemAdd(ImRect(), id, None, ItemFlags::NoNav | ItemFlags::NoNavDefaultFocus);
        return false;
    }

    IM_ASSERT(!p_open || !(flags & TabItemFlags::Button));
    IM_ASSERT((flags & (TabItemFlags::Leading | TabItemFlags::Trailing)) != (TabItemFlags::Leading | TabItemFlags::Trailing)); // Can't use both Leading and Trailing

    // Store into ImGuiTabItemFlags_NoCloseButton, also honor ImGuiTabItemFlags_NoCloseButton passed by user (although not documented)
    if (flags & TabItemFlags::NoCloseButton)
        p_open = None;
    else if (p_open == None)
        flags |= TabItemFlags::NoCloseButton;

    // Acquire tab data
    ImGuiTabItem* tab = tab_bar_find_tab_by_id(tab_bar, id);
    bool tab_is_new = false;
    if (tab == None)
    {
        tab_bar->Tabs.push_back(ImGuiTabItem());
        tab = &tab_bar->Tabs.back();
        tab->ID = id;
        tab_bar->TabsAddedNew = tab_is_new = true;
    }
    tab_bar->LastTabItemIdx = (ImS16)tab_bar->Tabs.index_from_ptr(tab);

    // Calculate tab contents size
    Vector2D size = tab_item_calc_size(label, p_open != None);
    tab->RequestedWidth = -1.0;
    if (g.next_item_data.Flags & NextItemDataFlags::HasWidth)
        size.x = tab->RequestedWidth = g.next_item_data.width;
    if (tab_is_new)
        tab->Width = size.x;
    tab->ContentWidth = size.x;
    tab->BeginOrder = tab_bar->TabsActiveCount += 1;

    const bool tab_bar_appearing = (tab_bar->PrevFrameVisible + 1 < g.FrameCount);
    const bool tab_bar_focused = (tab_bar->Flags & TabBarFlags::IsFocused) != 0;
    const bool tab_appearing = (tab->LastFrameVisible + 1 < g.FrameCount);
    const bool is_tab_button = (flags & TabItemFlags::Button) != 0;
    tab->LastFrameVisible = g.FrameCount;
    tab->Flags = flags;
    tab->Window = docked_window;

    // Append name with zero-terminator
    // (regular tabs are permitted in a dock_node tab bar, but window tabs not permitted in a non-dock_node tab bar)
    if (tab->Window != None)
    {
        IM_ASSERT(tab_bar->Flags & TabBarFlags::DockNode);
        tab->NameOffset = -1;
    }
    else
    {
        IM_ASSERT(tab->Window == None);
        tab->NameOffset = (ImS32)tab_bar->TabsNames.size();
        tab_bar->TabsNames.append(label, label + strlen(label) + 1); // Append name _with_ the zero-terminator.
    }

    // Update selected tab
    if (!is_tab_button)
    {
        if (tab_appearing && (tab_bar->Flags & TabBarFlags::AutoSelectNewTabs) && tab_bar->NextSelectedTabId == 0)
            if (!tab_bar_appearing || tab_bar->SelectedTabId == 0)
                tab_bar->NextSelectedTabId = id;  // New tabs gets activated
        if ((flags & TabItemFlags::SetSelected) && (tab_bar->SelectedTabId != id)) // _SetSelected can only be passed on explicit tab bar
            tab_bar->NextSelectedTabId = id;
    }

    // Lock visibility
    // (Note: tab_contents_visible != tab_selected... because CTRL+TAB operations may preview some tabs without selecting them!)
    bool tab_contents_visible = (tab_bar->visible_tab_id == id);
    if (tab_contents_visible)
        tab_bar->VisibleTabWasSubmitted = true;

    // On the very first frame of a tab bar we let first tab contents be visible to minimize appearing glitches
    if (!tab_contents_visible && tab_bar->SelectedTabId == 0 && tab_bar_appearing && docked_window == None)
        if (tab_bar->Tabs.Size == 1 && !(tab_bar->Flags & TabBarFlags::AutoSelectNewTabs))
            tab_contents_visible = true;

    // Note that tab_is_new is not necessarily the same as tab_appearing! When a tab bar stops being submitted
    // and then gets submitted again, the tabs will have 'tab_appearing=true' but 'tab_is_new=false'.
    if (tab_appearing && (!tab_bar_appearing || tab_is_new))
    {
        ItemAdd(ImRect(), id, None, ItemFlags::NoNav | ItemFlags::NoNavDefaultFocus);
        if (is_tab_button)
            return false;
        return tab_contents_visible;
    }

    if (tab_bar->SelectedTabId == id)
        tab->LastFrameSelected = g.FrameCount;

    // Backup current layout position
    const Vector2D backup_main_cursor_pos = window.DC.CursorPos;

    // Layout
    const bool is_central_section = (tab->Flags & TabItemFlags::SectionMask_) == 0;
    size.x = tab->Width;
    if (is_central_section)
        window.DC.CursorPos = tab_bar->BarRect.Min + DimgVec2D::new(IM_FLOOR(tab->Offset - tab_bar->ScrollingAnim), 0.0);
    else
        window.DC.CursorPos = tab_bar->BarRect.Min + DimgVec2D::new(tab->Offset, 0.0);
    Vector2D pos = window.DC.CursorPos;
    ImRect bb(pos, pos + size);

    // We don't have CPU clipping primitives to clip the CloseButton (until it becomes a texture), so need to add an extra draw call (temporary in the case of vertical animation)
    const bool want_clip_rect = is_central_section && (bb.Min.x < tab_bar->ScrollingRectMinX || bb.Max.x > tab_bar->ScrollingRectMaxX);
    if (want_clip_rect)
        push_clip_rect(DimgVec2D::new(ImMax(bb.Min.x, tab_bar->ScrollingRectMinX), bb.Min.y - 1), DimgVec2D::new(tab_bar->ScrollingRectMaxX, bb.Max.y), true);

    Vector2D backup_cursor_max_pos = window.DC.CursorMaxPos;
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
    if (g.DragDropActive && !g.DragDropPayload.is_data_type(IMGUI_PAYLOAD_TYPE_WINDOW)) // FIXME: May be an opt-in property of the payload to disable this
        button_flags |= ImGuiButtonFlags_PressedOnDragDropHold;
    bool hovered, held;
    bool pressed = ButtonBehavior(bb, id, &hovered, &held, button_flags);
    if (pressed && !is_tab_button)
        tab_bar->NextSelectedTabId = id;

    // Transfer active id window so the active id is not owned by the dock host (as StartMouseMovingWindow()
    // will only do it on the drag). This allows FocusWindow() to be more conservative in how it clears active id.
    if (held && docked_window && g.ActiveId == id && g.ActiveIdIsJustActivated)
        g.ActiveIdWindow = docked_window;

    // Allow the close button to overlap unless we are dragging (in which case we don't want any overlapping tabs to be hovered)
    if (g.ActiveId != id)
        SetItemAllowOverlap();

    // Drag and drop a single floating window node moves it
    ImGuiDockNode* node = docked_window ? docked_window.DockNode : None;
    const bool single_floating_window_node = node && node->is_floating_node() && (node->Windows.Size == 1);
    if (held && single_floating_window_node && IsMouseDragging(0, 0.0))
    {
        // Move
        StartMouseMovingWindow(docked_window);
    }
    else if (held && !tab_appearing && IsMouseDragging(0))
    {
        // Drag and drop: re-order tabs
        int drag_dir = 0;
        let drag_distance_from_edge_x =  0.0;
        if (!g.DragDropActive && ((tab_bar->Flags & TabBarFlags::Reorderable) || (docked_window != None)))
        {
            // While moving a tab it will jump on the other side of the mouse, so we also test for mouse_delta.x
            if (g.IO.MouseDelta.x < 0.0 && g.IO.MousePos.x < bb.Min.x)
            {
                drag_dir = -1;
                drag_distance_from_edge_x = bb.Min.x - g.IO.MousePos.x;
                TabBarQueueReorderFromMousePos(tab_bar, tab, g.IO.MousePos);
            }
            else if (g.IO.MouseDelta.x > 0.0 && g.IO.MousePos.x > bb.Max.x)
            {
                drag_dir = +1;
                drag_distance_from_edge_x = g.IO.MousePos.x - bb.Max.x;
                TabBarQueueReorderFromMousePos(tab_bar, tab, g.IO.MousePos);
            }
        }

        // Extract a Dockable window out of it's tab bar
        if (docked_window != None && !(docked_window.Flags & WindowFlags_NoMove))
        {
            // We use a variable threshold to distinguish dragging tabs within a tab bar and extracting them out of the tab bar
            bool undocking_tab = (g.DragDropActive && g.DragDropPayload.SourceId == id);
            if (!undocking_tab) //&& (!g.io.config_docking_with_shift || g.io.key_shift)
            {
                let threshold_base =  g.FontSize;
                let threshold_x =  (threshold_base * 2.2);
                let threshold_y =  (threshold_base * 1.5) + ImClamp((ImFabs(g.IO.MouseDragMaxDistanceAbs[0].x) - threshold_base * 2.0) * 0.20, 0.0, threshold_base * 4.0);
                //GetForegroundDrawList()->add_rect(Vector2D(bb.min.x - threshold_x, bb.min.y - threshold_y), Vector2D(bb.max.x + threshold_x, bb.max.y + threshold_y), IM_COL32_WHITE); // [DEBUG]

                let distance_from_edge_y =  ImMax(bb.Min.y - g.IO.MousePos.y, g.IO.MousePos.y - bb.Max.y);
                if (distance_from_edge_y >= threshold_y)
                    undocking_tab = true;
                if (drag_distance_from_edge_x > threshold_x)
                    if ((drag_dir < 0 && tab_bar->GetTabOrder(tab) == 0) || (drag_dir > 0 && tab_bar->GetTabOrder(tab) == tab_bar->Tabs.Size - 1))
                        undocking_tab = true;
            }

            if (undocking_tab)
            {
                // Undock
                // FIXME: refactor to share more code with e.g. StartMouseMovingWindow
                DockContextQueueUndockWindow(&g, docked_window);
                g.MovingWindow = docked_window;
                SetActiveID(g.MovingWindow->MoveId, g.MovingWindow);
                g.active_id_click_offset -= g.MovingWindow->Pos - bb.Min;
                g.active_id_no_clear_on_focus_loss = true;
                set_active_id_using_nav_and_keys();
            }
        }
    }

#if 0
    if (hovered && g.HoveredIdNotActiveTimer > TOOLTIP_DELAY && bb.GetWidth() < tab->ContentWidth)
    {
        // Enlarge tab display when hovering
        bb.max.x = bb.min.x + IM_FLOOR(ImLerp(bb.GetWidth(), tab->ContentWidth, ImSaturate((g.HoveredIdNotActiveTimer - 0.40) * 6.0)));
        display_draw_list = GetForegroundDrawList(window);
        tab_item_background(display_draw_list, bb, flags, GetColorU32(ImGuiCol_TitleBgActive));
    }
#endif

    // Render tab shape
    ImDrawList* display_draw_list = window.draw_list;
    const ImU32 tab_col = GetColorU32((held || hovered) ? ImGuiCol_TabHovered : tab_contents_visible ? (tab_bar_focused ? ImGuiCol_TabActive : ImGuiCol_TabUnfocusedActive) : (tab_bar_focused ? ImGuiCol_Tab : ImGuiCol_TabUnfocused));
    tab_item_background(display_draw_list, bb, flags, tab_col);
    RenderNavHighlight(bb, id);

    // Select with right mouse button. This is so the common idiom for context menu automatically highlight the current widget.
    const bool hovered_unblocked = IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup);
    if (hovered_unblocked && (is_mouse_clicked(1) || IsMouseReleased(1)))
        if (!is_tab_button)
            tab_bar->NextSelectedTabId = id;

    if (tab_bar->Flags & TabBarFlags::NoCloseWithMiddleMouseButton)
        flags |= TabItemFlags::NoCloseWithMiddleMouseButton;

    // Render tab label, process close button
    const Id32 close_button_id = p_open ? GetIDWithSeed("#CLOSE", None, docked_window ? docked_window.id : id) : 0;
    bool just_closed;
    bool text_clipped;
    tab_item_label_and_close_button(display_draw_list, bb, flags, tab_bar->FramePadding, label, id, close_button_id, tab_contents_visible, &just_closed, &text_clipped);
    if (just_closed && p_open != None)
    {
        *p_open = false;
        tab_bar_close_tab(tab_bar, tab);
    }

    // Forward Hovered state so IsItemHovered() after Begin() can work (even though we are technically hovering our parent)
    // That state is copied to window->dock_tab_item_status_flags by our caller.
    if (docked_window && (hovered || g.HoveredId == close_button_id))
        g.last_item_data.StatusFlags |= ItemStatusFlags::HoveredWindow;

    // Restore main window position so user can draw there
    if (want_clip_rect)
        PopClipRect();
    window.DC.CursorPos = backup_main_cursor_pos;

    // Tooltip
    // (Won't work over the close button because ItemOverlap systems messes up with hovered_id_timer-> seems ok)
    // (We test IsItemHovered() to discard e.g. when another item is active or drag and drop over the tab bar, which g.hovered_id ignores)
    // FIXME: This is a mess.
    // FIXME: We may want disabled tab to still display the tooltip?
    if (text_clipped && g.HoveredId == id && !held && g.HoveredIdNotActiveTimer > g.TooltipSlowDelay && IsItemHovered())
        if (!(tab_bar->Flags & TabBarFlags::NoTooltip) && !(tab->Flags & TabItemFlags::NoTooltip))
            SetTooltip("%.*s", (FindRenderedTextEnd(label) - label), label);

    IM_ASSERT(!is_tab_button || !(tab_bar->SelectedTabId == tab->ID && is_tab_button)); // TabItemButton should not be selected
    if (is_tab_button)
        return pressed;
    return tab_contents_visible;
}

// [Public] This is call is 100% optional but it allows to remove some one-frame glitches when a tab has been unexpectedly removed.
// To use it to need to call the function SetTabItemClosed() between BeginTabBar() and EndTabBar().
// Tabs closed by the close button will automatically be flagged to avoid this issue.
void    ImGui::SetTabItemClosed(const char* label)
{
    // ImGuiContext& g = *GImGui;
    bool is_within_manual_tab_bar = g.CurrentTabBar && !(g.CurrentTabBar->Flags & TabBarFlags::DockNode);
    if (is_within_manual_tab_bar)
    {
        ImGuiTabBar* tab_bar = g.CurrentTabBar;
        Id32 tab_id = TabBarCalcTabID(tab_bar, label, None);
        if (ImGuiTabItem* tab = tab_bar_find_tab_by_id(tab_bar, tab_id))
            tab->WantClose = true; // Will be processed by next call to TabBarLayout()
    }
    else if (Window* window = find_window_by_name(label))
    {
        if (window.DockIsActive)
            if (ImGuiDockNode* node = window.DockNode)
            {
                Id32 tab_id = TabBarCalcTabID(node->TabBar, label, window);
                tab_bar_remove_tab(node->TabBar, tab_id);
                window.dock_tab_want_close = true;
            }
    }
}

Vector2D ImGui::tab_item_calc_size(const char* label, bool has_close_button)
{
    // ImGuiContext& g = *GImGui;
    Vector2D label_size = CalcTextSize(label, None, true);
    Vector2D size = DimgVec2D::new(label_size.x + g.Style.FramePadding.x, label_size.y + g.Style.FramePadding.y * 2.0);
    if (has_close_button)
        size.x += g.Style.FramePadding.x + (g.Style.ItemInnerSpacing.x + g.FontSize); // We use Y intentionally to fit the close button circle.
    else
        size.x += g.Style.FramePadding.x + 1.0;
    return DimgVec2D::new(ImMin(size.x, TabBarCalcMaxTabWidth()), size.y);
}

void ImGui::tab_item_background(ImDrawList* draw_list, const ImRect& bb, ImGuiTabItemFlags flags, ImU32 col)
{
    // While rendering tabs, we trim 1 pixel off the top of our bounding box so they can fit within a regular frame height while looking "detached" from it.
    // ImGuiContext& g = *GImGui;
    let width = bb.GetWidth();
    IM_UNUSED(flags);
    IM_ASSERT(width > 0.0);
    let rounding = ImMax(0.0, ImMin((flags & TabItemFlags::Button) ? g.Style.frame_rounding : g.Style.TabRounding, width * 0.5 - 1.0));
    let y1 = bb.Min.y + 1.0;
    let y2 = bb.Max.y + ((flags & TabItemFlags::Preview) ? 0.0 : -1.0);
    draw_list->path_line_to(DimgVec2D::new(bb.Min.x, y2));
    draw_list->path_arc_toFast(DimgVec2D::new(bb.Min.x + rounding, y1 + rounding), rounding, 6, 9);
    draw_list->path_arc_toFast(DimgVec2D::new(bb.Max.x - rounding, y1 + rounding), rounding, 9, 12);
    draw_list->path_line_to(DimgVec2D::new(bb.Max.x, y2));
    draw_list->PathFillConvex(col);
    if (g.Style.TabBorderSize > 0.0)
    {
        draw_list->path_line_to(DimgVec2D::new(bb.Min.x + 0.5, y2));
        draw_list->path_arc_toFast(DimgVec2D::new(bb.Min.x + rounding + 0.5, y1 + rounding + 0.5), rounding, 6, 9);
        draw_list->path_arc_toFast(DimgVec2D::new(bb.Max.x - rounding - 0.5, y1 + rounding + 0.5), rounding, 9, 12);
        draw_list->path_line_to(DimgVec2D::new(bb.Max.x - 0.5, y2));
        draw_list->path_stroke(GetColorU32(ImGuiCol_Border), 0, g.Style.TabBorderSize);
    }
}

// Render text label (with custom clipping) + Unsaved Document marker + Close Button logic
// We tend to lock style.FramePadding for a given tab-bar, hence the 'frame_padding' parameter.
void ImGui::tab_item_label_and_close_button(ImDrawList* draw_list, const ImRect& bb, ImGuiTabItemFlags flags, Vector2D frame_padding, const char* label, Id32 tab_id, Id32 close_button_id, bool is_contents_visible, bool* out_just_closed, bool* out_text_clipped)
{
    // ImGuiContext& g = *GImGui;
    Vector2D label_size = CalcTextSize(label, None, true);

    if (out_just_closed)
        *out_just_closed = false;
    if (out_text_clipped)
        *out_text_clipped = false;

    if (bb.GetWidth() <= 1.0)
        return;

    // In style V2 we'll have full override of all colors per state (e.g. focused, selected)
    // But right now if you want to alter text color of tabs this is what you need to do.
#if 0
    let backup_alpha = g.style.Alpha;
    if (!is_contents_visible)
        g.style.Alpha *= 0.7;
#endif

    // Render text label (with clipping + alpha gradient) + unsaved marker
    ImRect text_pixel_clip_bb(bb.Min.x + frame_padding.x, bb.Min.y + frame_padding.y, bb.Max.x - frame_padding.x, bb.Max.y);
    ImRect text_ellipsis_clip_bb = text_pixel_clip_bb;

    // Return clipped state ignoring the close button
    if (out_text_clipped)
    {
        *out_text_clipped = (text_ellipsis_clip_bb.Min.x + label_size.x) > text_pixel_clip_bb.Max.x;
        //draw_list->add_circle(text_ellipsis_clip_bb.min, 3.0, *out_text_clipped ? IM_COL32(255, 0, 0, 255) : IM_COL32(0, 255, 0, 255));
    }

    let button_sz = g.FontSize;
    const Vector2D button_pos(ImMax(bb.Min.x, bb.Max.x - frame_padding.x * 2.0 - button_sz), bb.Min.y);

    // Close Button & Unsaved Marker
    // We are relying on a subtle and confusing distinction between 'hovered' and 'g.hovered_id' which happens because we are using ImGuiButtonFlags_AllowOverlapMode + SetItemAllowOverlap()
    //  'hovered' will be true when hovering the Tab but NOT when hovering the close button
    //  'g.hovered_id==id' will be true when hovering the Tab including when hovering the close button
    //  'g.active_id==close_button_id' will be true when we are holding on the close button, in which case both hovered booleans are false
    bool close_button_pressed = false;
    bool close_button_visible = false;
    if (close_button_id != 0)
        if (is_contents_visible || bb.GetWidth() >= ImMax(button_sz, g.Style.TabMinWidthForCloseButton))
            if (g.HoveredId == tab_id || g.HoveredId == close_button_id || g.ActiveId == tab_id || g.ActiveId == close_button_id)
                close_button_visible = true;
    bool unsaved_marker_visible = (flags & TabItemFlags::UnsavedDocument) != 0 && (button_pos.x + button_sz <= bb.Max.x);

    if (close_button_visible)
    {
        ImGuiLastItemData last_item_backup = g.last_item_data;
        PushStyleVar(ImGuiStyleVar_FramePadding, frame_padding);
        if (CloseButton(close_button_id, button_pos))
            close_button_pressed = true;
        PopStyleVar();
        g.last_item_data = last_item_backup;

        // Close with middle mouse button
        if (!(flags & TabItemFlags::NoCloseWithMiddleMouseButton) && is_mouse_clicked(2))
            close_button_pressed = true;
    }
    else if (unsaved_marker_visible)
    {
        const ImRect bullet_bb(button_pos, button_pos + DimgVec2D::new(button_sz, button_sz) + g.Style.FramePadding * 2.0);
        RenderBullet(draw_list, bullet_bb.get_center(), GetColorU32(ImGuiCol_Text));
    }

    // This is all rather complicated
    // (the main idea is that because the close button only appears on hover, we don't want it to alter the ellipsis position)
    // FIXME: if FramePadding is noticeably large, ellipsis_max_x will be wrong here (e.g. #3497), maybe for consistency that parameter of RenderTextEllipsis() shouldn't exist..
    let ellipsis_max_x =  close_button_visible ? text_pixel_clip_bb.Max.x : bb.Max.x - 1.0;
    if (close_button_visible || unsaved_marker_visible)
    {
        text_pixel_clip_bb.Max.x -= close_button_visible ? (button_sz) : (button_sz * 0.80);
        text_ellipsis_clip_bb.Max.x -= unsaved_marker_visible ? (button_sz * 0.80) : 0.0;
        ellipsis_max_x = text_pixel_clip_bb.Max.x;
    }
    render_textEllipsis(draw_list, text_ellipsis_clip_bb.Min, text_ellipsis_clip_bb.Max, text_pixel_clip_bb.Max.x, ellipsis_max_x, label, None, &label_size);

#if 0
    if (!is_contents_visible)
        g.style.Alpha = backup_alpha;
#endif

    if (out_just_closed)
        *out_just_closed = close_button_pressed;
}


#endif // #ifndef IMGUI_DISABLE
