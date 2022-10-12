// dear imgui, v1.89 WIP
// (drawing and font code)

use std::ptr::{null, null_mut};
use libc::{c_char, c_float, c_int, c_uchar, c_uint};
use crate::bit_vector::ImBitVector;
use crate::color::{IM_COL32_A_MASK, IM_COL32_B_SHIFT, IM_COL32_G_SHIFT, IM_COL32_R_SHIFT, ImGuiCol_Border, ImGuiCol_BorderShadow, ImGuiCol_Button, ImGuiCol_ButtonActive, ImGuiCol_ButtonHovered, ImGuiCol_CheckMark, ImGuiCol_ChildBg, ImGuiCol_DockingEmptyBg, ImGuiCol_DockingPreview, ImGuiCol_DragDropTarget, ImGuiCol_FrameBg, ImGuiCol_FrameBgActive, ImGuiCol_FrameBgHovered, ImGuiCol_Header, ImGuiCol_HeaderActive, ImGuiCol_HeaderHovered, ImGuiCol_MenuBarBg, ImGuiCol_ModalWindowDimBg, ImGuiCol_NavHighlight, ImGuiCol_NavWindowingDimBg, ImGuiCol_NavWindowingHighlight, ImGuiCol_PlotHistogram, ImGuiCol_PlotHistogramHovered, ImGuiCol_PlotLines, ImGuiCol_PlotLinesHovered, ImGuiCol_PopupBg, ImGuiCol_ResizeGrip, ImGuiCol_ResizeGripActive, ImGuiCol_ResizeGripHovered, ImGuiCol_ScrollbarBg, ImGuiCol_ScrollbarGrab, ImGuiCol_ScrollbarGrabActive, ImGuiCol_ScrollbarGrabHovered, ImGuiCol_Separator, ImGuiCol_SeparatorActive, ImGuiCol_SeparatorHovered, ImGuiCol_SliderGrab, ImGuiCol_SliderGrabActive, ImGuiCol_Tab, ImGuiCol_TabActive, ImGuiCol_TabHovered, ImGuiCol_TableBorderLight, ImGuiCol_TableBorderStrong, ImGuiCol_TableHeaderBg, ImGuiCol_TableRowBg, ImGuiCol_TableRowBgAlt, ImGuiCol_TabUnfocused, ImGuiCol_TabUnfocusedActive, ImGuiCol_Text, ImGuiCol_TextDisabled, ImGuiCol_TextSelectedBg, ImGuiCol_TitleBg, ImGuiCol_TitleBgActive, ImGuiCol_TitleBgCollapsed, ImGuiCol_WindowBg};
use crate::draw_cmd::ImDrawCmd;
use crate::draw_flags::{ImDrawFlags, ImDrawFlags_RoundCornersAll, ImDrawFlags_RoundCornersMask_};
use crate::draw_list::ImDrawList;
use crate::draw_vert::ImDrawVert;
use crate::font_atlas::ImFontAtlas;
use crate::font_build_dst_data::ImFontBuildDstData;
use crate::font_build_src_data::ImFontBuildSrcData;
use crate::math::{ImClamp, ImDot, ImLengthSqr, ImLerp, ImMax, ImMin, ImMul};
use crate::mouse_cursor::ImGuiMouseCursor_COUNT;
use crate::style::ImGuiStyle;
use crate::style_ops::GetStyle;
use crate::type_defs::{ImTextureID, ImWchar};
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;

// ImDrawCallback: Draw callbacks for advanced uses [configurable type: override in imconfig.h]
// NB: You most likely do NOT need to use draw callbacks just to create your own widget or customized UI rendering,
// you can poke into the draw list for that! Draw callback may be useful for example to:
//  A) Change your GPU render state,
//  B) render a complex 3D scene inside a UI element without an intermediate texture/render target, etc.
// The expected behavior from your rendering function is 'if (cmd.UserCallback != NULL) { cmd.UserCallback(parent_list, cmd); } else { RenderTriangles() }'
// If you want to override the signature of ImDrawCallback, you can simply use e.g. '#define ImDrawCallback MyDrawCallback' (in imconfig.h) + update rendering backend accordingly.
// #ifndef ImDrawCallback
// typedef c_void (*ImDrawCallback)(*const ImDrawList parent_list, *const ImDrawCmd cmd);
pub type ImDrawCallback = fn(parent_list: *const ImDrawList, cmd: *const ImDrawCmd);
// #endif


/*

Index of this file:

// [SECTION] STB libraries implementation
// [SECTION] Style functions
// [SECTION] ImDrawList
// [SECTION] ImDrawListSplitter
// [SECTION] ImDrawData
// [SECTION] Helpers ShadeVertsXXX functions
// [SECTION] ImFontConfig
// [SECTION] ImFontAtlas
// [SECTION] ImFontAtlas glyph ranges helpers
// [SECTION] ImFontGlyphRangesBuilder
// [SECTION] ImFont
// [SECTION] ImGui Internal Render Helpers
// [SECTION] Decompression code
// [SECTION] Default font data (ProggyClean.tt0f32)

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
// #ifdef IMGUI_ENABLE_FREETYPE
// #include "misc/freetype/imgui_freetype.h"
// #endif

// #include <stdio.h>      // vsnprintf, sscanf, printf
// #if !defined(alloca)
// #if defined(__GLIBC__) || defined(__sun) || defined(__APPLE__) || defined(__NEWLIB__)
// #include <alloca.h>     // alloca (glibc uses <alloca.h>. Note that Cygwin may have _WIN32 defined, so the order matters here)
// #elif defined(_WIN32)
// #include <malloc.h>     // alloca
// #if !defined(alloca)
// #define alloca _alloca  // for clang with MS Codegen
// #endif
// #else
// #include <stdlib.h>     // alloca
// #endif
// #endif

// Visual Studio warnings
// #ifdef _MSC_VER
// #pragma warning (disable: 4127)     // condition expression is constant
// #pragma warning (disable: 4505)     // unreferenced local function has been removed (stb stuf0f32)
// #pragma warning (disable: 4996)     // 'This function or variable may be unsafe': strcpy, strdup, sprintf, vsnprintf, sscanf, fopen
// #pragma warning (disable: 6255)     // [Static Analyzer] _alloca indicates failure by raising a stack overflow exception.  Consider using _malloca instead.
// #pragma warning (disable: 26451)    // [Static Analyzer] Arithmetic overflow : Using operator 'xxx' on a 4 byte value and then casting the result to a 8 byte value. Cast the value to the wider type before calling operator 'xxx' to avoid overflow(io.2).
// #pragma warning (disable: 26812)    // [Static Analyzer] The enum type 'xxx' is unscoped. Prefer 'enum class' over 'enum' (Enum.3). [MSVC Static Analyzer)
// #endif

// Clang/GCC warnings with -Weverything
// #if defined(__clang__)
// #if __has_warning("-Wunknown-warning-option")
// #pragma clang diagnostic ignored "-Wunknown-warning-option"         // warning: unknown warning group 'xxx'                      // not all warnings are known by all Clang versions and they tend to be rename-happy.. so ignoring warnings triggers new warnings on some configuration. Great!
// #endif
// #if __has_warning("-Walloca")
// #pragma clang diagnostic ignored "-Walloca"                         // warning: use of function '__builtin_alloca' is discouraged
// #endif
// #pragma clang diagnostic ignored "-Wunknown-pragmas"                // warning: unknown warning group 'xxx'
// #pragma clang diagnostic ignored "-Wold-style-cast"                 // warning: use of old-style cast                            // yes, they are more terse.
// #pragma clang diagnostic ignored "-Wfloat-equal"                    // warning: comparing floating point with == or != is unsafe // storing and comparing against same constants ok.
// #pragma clang diagnostic ignored "-Wglobal-constructors"            // warning: declaration requires a global destructor         // similar to above, not sure what the exact difference is.
// #pragma clang diagnostic ignored "-Wsign-conversion"                // warning: implicit conversion changes signedness
// #pragma clang diagnostic ignored "-Wzero-as-null-pointer-constant"  // warning: zero as null pointer constant                    // some standard header variations use #define NULL 0
// #pragma clang diagnostic ignored "-Wcomma"                          // warning: possible misuse of comma operator here
// #pragma clang diagnostic ignored "-Wreserved-id-macro"              // warning: macro name is a reserved identifier
// #pragma clang diagnostic ignored "-Wdouble-promotion"               // warning: implicit conversion from 'float' to 'double' when passing argument to function  // using printf() is a misery with this as C++ va_arg ellipsis changes float to double.
// #pragma clang diagnostic ignored "-Wimplicit-int-float-conversion"  // warning: implicit conversion from 'xxx' to 'float' may lose precision
// #elif defined(__GNUC__)
// #pragma GCC diagnostic ignored "-Wpragmas"                  // warning: unknown option after '#pragma GCC diagnostic' kind
// #pragma GCC diagnostic ignored "-Wunused-function"          // warning: 'xxxx' defined but not used
// #pragma GCC diagnostic ignored "-Wdouble-promotion"         // warning: implicit conversion from 'float' to 'double' when passing argument to function
// #pragma GCC diagnostic ignored "-Wconversion"               // warning: conversion to 'xxxx' from 'xxxx' may alter its value
// #pragma GCC diagnostic ignored "-Wstack-protector"          // warning: stack protector not protecting local variables: variable length buffer
// #pragma GCC diagnostic ignored "-Wclass-memaccess"          // [__GNUC__ >= 8] warning: 'memset/memcpy' clearing/writing an object of type 'xxxx' with no trivial copy-assignment; use assignment or value-initialization instead
// #endif

//-------------------------------------------------------------------------
// [SECTION] STB libraries implementation (for stb_truetype and stb_rect_pack)
//-------------------------------------------------------------------------

// Compile time options:
//#define IMGUI_STB_NAMESPACE           ImStb
//#define IMGUI_STB_TRUETYPE_FILENAME   "my_folder/stb_truetype.h"
//#define IMGUI_STB_RECT_PACK_FILENAME  "my_folder/stb_rect_pack.h"
//#define IMGUI_DISABLE_STB_TRUETYPE_IMPLEMENTATION
//#define IMGUI_DISABLE_STB_RECT_PACK_IMPLEMENTATION

// #ifdef IMGUI_STB_NAMESPACE
namespace IMGUI_STB_NAMESPACE
{
// #endif

// #ifdef _MSC_VER
// #pragma warning (push)
// #pragma warning (disable: 4456)                             // declaration of 'xx' hides previous local declaration
// #pragma warning (disable: 6011)                             // (stb_rectpack) Dereferencing NULL pointer 'cur->next'.
// #pragma warning (disable: 6385)                             // (stb_truetype) Reading invalid data from 'buffer':  the readable size is '_Old_3`kernel_width' bytes, but '3' bytes may be read.
// #pragma warning (disable: 28182)                            // (stb_rectpack) Dereferencing NULL pointer. 'cur' contains the same NULL value as 'cur->next' did.
// #endif

// #if defined(__clang__)
// #pragma clang diagnostic push
// #pragma clang diagnostic ignored "-Wunused-function"
// #pragma clang diagnostic ignored "-Wmissing-prototypes"
// #pragma clang diagnostic ignored "-Wimplicit-fallthrough"
// #pragma clang diagnostic ignored "-Wcast-qual"              // warning: cast from 'const xxxx *' to 'xxx *' drops const qualifier
// #endif

// #if defined(__GNUC__)
// #pragma GCC diagnostic push
// #pragma GCC diagnostic ignored "-Wtype-limits"              // warning: comparison is always true due to limited range of data type [-Wtype-limits]
// #pragma GCC diagnostic ignored "-Wcast-qual"                // warning: cast from type 'const xxxx *' to type 'xxxx *' casts away qualifiers
// #endif

// #ifndef STB_RECT_PACK_IMPLEMENTATION                        // in case the user already have an implementation in the _same_ compilation unit (e.g. unity builds)
// #ifndef IMGUI_DISABLE_STB_RECT_PACK_IMPLEMENTATION          // in case the user already have an implementation in another compilation unit
// #define STBRP_STATIC
// #define STBRP_ASSERT(x)     do { IM_ASSERT(x); } while (0)
// #define STBRP_SORT          ImQsort
// #define STB_RECT_PACK_IMPLEMENTATION
// #endif
// #ifdef IMGUI_STB_RECT_PACK_FILENAME
// #include IMGUI_STB_RECT_PACK_FILENAME
// #else
// #include "imstb_rectpack.h"
// #endif
// #endif

// #ifdef  IMGUI_ENABLE_STB_TRUETYPE
// #ifndef STB_TRUETYPE_IMPLEMENTATION                         // in case the user already have an implementation in the _same_ compilation unit (e.g. unity builds)
// #ifndef IMGUI_DISABLE_STB_TRUETYPE_IMPLEMENTATION           // in case the user already have an implementation in another compilation unit
// #define STBTT_malloc(x,u)   ((void)(u), IM_ALLOC(x))
// #define STBTT_free(x,u)     ((void)(u), IM_FREE(x))
// #define STBTT_assert(x)     do { IM_ASSERT(x); } while(0)
// #define STBTT_fmod(x,y)     ImFmod(x,y)
// #define STBTT_sqrt(x)       ImSqrt(x)
// #define STBTT_pow(x,y)      ImPow(x,y)
// #define STBTT_fabs(x)       ImFabs(x)
// #define STBTT_ifloor(x)     (ImFloorSigned(x))
// #define STBTT_iceil(x)      (ImCeil(x))
// #define STBTT_STATIC
// #define STB_TRUETYPE_IMPLEMENTATION
// #else
// #define STBTT_DEF extern
// #endif
// #ifdef IMGUI_STB_TRUETYPE_FILENAME
// #include IMGUI_STB_TRUETYPE_FILENAME
// #else
// #include "imstb_truetype.h"
// #endif
// #endif
// #endif // IMGUI_ENABLE_STB_TRUETYPE

// #if defined(__GNUC__)
// #pragma GCC diagnostic pop
// #endif

// #if defined(__clang__)
// #pragma clang diagnostic pop
// #endif

// #if defined(_MSC_VER)
// #pragma warning (pop)
// #endif

// #ifdef IMGUI_STB_NAMESPACE
} // namespace ImStb
using namespace IMGUI_STB_NAMESPACE;
// #endif

//-----------------------------------------------------------------------------
// [SECTION] Style functions
//-----------------------------------------------------------------------------

pub fn StyleColorsDark(dst: *ImGuiStyle)
{
    let style = if dst { dst } else { &GetStyle() };
    let colors = style.Colors;

    colors[ImGuiCol_Text]                   = ImVec4::new2(1f32, 1f32, 1f32, 1.000f32);
    colors[ImGuiCol_TextDisabled]           = ImVec4::new2(0.50f32, 0.50f32, 0.50f32, 1.000f32);
    colors[ImGuiCol_WindowBg]               = ImVec4::new2(0.06, 0.06, 0.06, 0.940f32);
    colors[ImGuiCol_ChildBg]                = ImVec4::new2(0.00f32, 0.00f32, 0.00f32, 0.000f32);
    colors[ImGuiCol_PopupBg]                = ImVec4::new2(0.08, 0.08, 0.08, 0.940f32);
    colors[ImGuiCol_Border]                 = ImVec4::new2(0.43, 0.43, 0.50f32, 0.500f32);
    colors[ImGuiCol_BorderShadow]           = ImVec4::new2(0.00f32, 0.00f32, 0.00f32, 0.000f32);
    colors[ImGuiCol_FrameBg]                = ImVec4::new2(0.16, 0.29, 0.48, 0.540f32);
    colors[ImGuiCol_FrameBgHovered]         = ImVec4::new2(0.26, 0.59, 0.98, 0.400f32);
    colors[ImGuiCol_FrameBgActive]          = ImVec4::new2(0.26, 0.59, 0.98, 0.670f32);
    colors[ImGuiCol_TitleBg]                = ImVec4::new2(0.04, 0.04, 0.04, 1.000f32);
    colors[ImGuiCol_TitleBgActive]          = ImVec4::new2(0.16, 0.29, 0.48, 1.000f32);
    colors[ImGuiCol_TitleBgCollapsed]       = ImVec4::new2(0.00f32, 0.00f32, 0.00f32, 0.510f32);
    colors[ImGuiCol_MenuBarBg]              = ImVec4::new2(0.14, 0.14, 0.14, 1.000f32);
    colors[ImGuiCol_ScrollbarBg]            = ImVec4::new2(0.02, 0.02, 0.02, 0.530f32);
    colors[ImGuiCol_ScrollbarGrab]          = ImVec4::new2(0.31, 0.31, 0.31, 1.000f32);
    colors[ImGuiCol_ScrollbarGrabHovered]   = ImVec4::new2(0.41, 0.41, 0.41, 1.000f32);
    colors[ImGuiCol_ScrollbarGrabActive]    = ImVec4::new2(0.51, 0.51, 0.51, 1.000f32);
    colors[ImGuiCol_CheckMark]              = ImVec4::new2(0.26, 0.59, 0.98, 1.000f32);
    colors[ImGuiCol_SliderGrab]             = ImVec4::new2(0.24, 0.52, 0.88, 1.000f32);
    colors[ImGuiCol_SliderGrabActive]       = ImVec4::new2(0.26, 0.59, 0.98, 1.000f32);
    colors[ImGuiCol_Button]                 = ImVec4::new2(0.26, 0.59, 0.98, 0.400f32);
    colors[ImGuiCol_ButtonHovered]          = ImVec4::new2(0.26, 0.59, 0.98, 1.000f32);
    colors[ImGuiCol_ButtonActive]           = ImVec4::new2(0.06, 0.53, 0.98, 1.000f32);
    colors[ImGuiCol_Header]                 = ImVec4::new2(0.26, 0.59, 0.98, 0.310f32);
    colors[ImGuiCol_HeaderHovered]          = ImVec4::new2(0.26, 0.59, 0.98, 0.800f32);
    colors[ImGuiCol_HeaderActive]           = ImVec4::new2(0.26, 0.59, 0.98, 1.000f32);
    colors[ImGuiCol_Separator]              = colors[ImGuiCol_Border];
    colors[ImGuiCol_SeparatorHovered]       = ImVec4::new2(0.1f32, 0.40f32, 0.75, 0.780f32);
    colors[ImGuiCol_SeparatorActive]        = ImVec4::new2(0.1f32, 0.40f32, 0.75, 1.000f32);
    colors[ImGuiCol_ResizeGrip]             = ImVec4::new2(0.26, 0.59, 0.98, 0.200f32);
    colors[ImGuiCol_ResizeGripHovered]      = ImVec4::new2(0.26, 0.59, 0.98, 0.670f32);
    colors[ImGuiCol_ResizeGripActive]       = ImVec4::new2(0.26, 0.59, 0.98, 0.950f32);
    colors[ImGuiCol_Tab]                    = ImLerp(colors[ImGuiCol_Header],       colors[ImGuiCol_TitleBgActive], 0.800f32);
    colors[ImGuiCol_TabHovered]             = colors[ImGuiCol_HeaderHovered];
    colors[ImGuiCol_TabActive]              = ImLerp(colors[ImGuiCol_HeaderActive], colors[ImGuiCol_TitleBgActive], 0.600f32);
    colors[ImGuiCol_TabUnfocused]           = ImLerp(colors[ImGuiCol_Tab],          colors[ImGuiCol_TitleBg], 0.800f32);
    colors[ImGuiCol_TabUnfocusedActive]     = ImLerp(colors[ImGuiCol_TabActive],    colors[ImGuiCol_TitleBg], 0.400f32);
    colors[ImGuiCol_DockingPreview]         = colors[ImGuiCol_HeaderActive] * ImVec4::new2(1f32, 1f32, 1f32, 0.70f32);
    colors[ImGuiCol_DockingEmptyBg]         = ImVec4::new2(0.20f32, 0.20f32, 0.20f32, 1.000f32);
    colors[ImGuiCol_PlotLines]              = ImVec4::new2(0.61f, 0.61f, 0.61f, 1.000f32);
    colors[ImGuiCol_PlotLinesHovered]       = ImVec4::new2(1f32, 0.43f, 0.35f, 1.000f32);
    colors[ImGuiCol_PlotHistogram]          = ImVec4::new2(0.90f32, 0.70f32, 0.00f32, 1.000f32);
    colors[ImGuiCol_PlotHistogramHovered]   = ImVec4::new2(1f32, 0.60f32, 0.00f32, 1.000f32);
    colors[ImGuiCol_TableHeaderBg]          = ImVec4::new2(0.19f, 0.19f, 0.20f32, 1.000f32);
    colors[ImGuiCol_TableBorderStrong]      = ImVec4::new2(0.31f, 0.31f, 0.35f, 1.000f32);   // Prefer using Alpha=1.0 here
    colors[ImGuiCol_TableBorderLight]       = ImVec4::new2(0.23f, 0.23f, 0.25f, 1.000f32);   // Prefer using Alpha=1.0 here
    colors[ImGuiCol_TableRowBg]             = ImVec4::new2(0.00f32, 0.00f32, 0.00f32, 0.000f32);
    colors[ImGuiCol_TableRowBgAlt]          = ImVec4::new2(1f32, 1f32, 1f32, 0.060f32);
    colors[ImGuiCol_TextSelectedBg]         = ImVec4::new2(0.26f, 0.59f, 0.98f, 0.350f32);
    colors[ImGuiCol_DragDropTarget]         = ImVec4::new2(1f32, 1f32, 0.00f32, 0.900f32);
    colors[ImGuiCol_NavHighlight]           = ImVec4::new2(0.26f, 0.59f, 0.98f, 1.000f32);
    colors[ImGuiCol_NavWindowingHighlight]  = ImVec4::new2(1f32, 1f32, 1f32, 0.700f32);
    colors[ImGuiCol_NavWindowingDimBg]      = ImVec4::new2(0.80f32, 0.80f32, 0.80f32, 0.200f32);
    colors[ImGuiCol_ModalWindowDimBg]       = ImVec4::new2(0.80f32, 0.80f32, 0.80f32, 0.350f32);
}

pub fn StyleColorsClassic(dst: *mut ImGuiStyle)
{
    let style = if dst { dst } else { &GetStyle() };
    let colors = style.Colors;

    colors[ImGuiCol_Text]                   = ImVec4::new2(0.90f32, 0.90f32, 0.90f32, 1.000f32);
    colors[ImGuiCol_TextDisabled]           = ImVec4::new2(0.60f32, 0.60f32, 0.60f32, 1.000f32);
    colors[ImGuiCol_WindowBg]               = ImVec4::new2(0.00f32, 0.00f32, 0.00f32, 0.850f32);
    colors[ImGuiCol_ChildBg]                = ImVec4::new2(0.00f32, 0.00f32, 0.00f32, 0.000f32);
    colors[ImGuiCol_PopupBg]                = ImVec4::new2(0.11, 0.11, 0.14, 0.920f32);
    colors[ImGuiCol_Border]                 = ImVec4::new2(0.50f32, 0.50f32, 0.50f32, 0.500f32);
    colors[ImGuiCol_BorderShadow]           = ImVec4::new2(0.00f32, 0.00f32, 0.00f32, 0.000f32);
    colors[ImGuiCol_FrameBg]                = ImVec4::new2(0.43, 0.43, 0.43, 0.390f32);
    colors[ImGuiCol_FrameBgHovered]         = ImVec4::new2(0.47, 0.47, 0.69, 0.400f32);
    colors[ImGuiCol_FrameBgActive]          = ImVec4::new2(0.42, 0.41, 0.64, 0.690f32);
    colors[ImGuiCol_TitleBg]                = ImVec4::new2(0.27, 0.27, 0.54, 0.830f32);
    colors[ImGuiCol_TitleBgActive]          = ImVec4::new2(0.32, 0.32, 0.63, 0.870f32);
    colors[ImGuiCol_TitleBgCollapsed]       = ImVec4::new2(0.40f32, 0.40f32, 0.80f32, 0.200f32);
    colors[ImGuiCol_MenuBarBg]              = ImVec4::new2(0.40f32, 0.40f32, 0.55, 0.800f32);
    colors[ImGuiCol_ScrollbarBg]            = ImVec4::new2(0.20f32, 0.25, 0.3f32, 0.600f32);
    colors[ImGuiCol_ScrollbarGrab]          = ImVec4::new2(0.40f32, 0.40f32, 0.80f32, 0.300f32);
    colors[ImGuiCol_ScrollbarGrabHovered]   = ImVec4::new2(0.40f32, 0.40f32, 0.80f32, 0.400f32);
    colors[ImGuiCol_ScrollbarGrabActive]    = ImVec4::new2(0.41, 0.39, 0.80f32, 0.600f32);
    colors[ImGuiCol_CheckMark]              = ImVec4::new2(0.90f32, 0.90f32, 0.90f32, 0.500f32);
    colors[ImGuiCol_SliderGrab]             = ImVec4::new2(1f32, 1f32, 1f32, 0.300f32);
    colors[ImGuiCol_SliderGrabActive]       = ImVec4::new2(0.41, 0.39, 0.80f32, 0.600f32);
    colors[ImGuiCol_Button]                 = ImVec4::new2(0.35, 0.40f32, 0.61, 0.620f32);
    colors[ImGuiCol_ButtonHovered]          = ImVec4::new2(0.40f32, 0.48, 0.71, 0.790f32);
    colors[ImGuiCol_ButtonActive]           = ImVec4::new2(0.46, 0.54, 0.80f32, 1.000f32);
    colors[ImGuiCol_Header]                 = ImVec4::new2(0.40f32, 0.40f32, 0.90f32, 0.450f32);
    colors[ImGuiCol_HeaderHovered]          = ImVec4::new2(0.45, 0.45, 0.90f32, 0.800f32);
    colors[ImGuiCol_HeaderActive]           = ImVec4::new2(0.53, 0.53, 0.87, 0.800f32);
    colors[ImGuiCol_Separator]              = ImVec4::new2(0.50f32, 0.50f32, 0.50f32, 0.600f32);
    colors[ImGuiCol_SeparatorHovered]       = ImVec4::new2(0.60f32, 0.60f32, 0.70f32, 1.000f32);
    colors[ImGuiCol_SeparatorActive]        = ImVec4::new2(0.70f32, 0.70f32, 0.90f32, 1.000f32);
    colors[ImGuiCol_ResizeGrip]             = ImVec4::new2(1f32, 1f32, 1f32, 0.100f32);
    colors[ImGuiCol_ResizeGripHovered]      = ImVec4::new2(0.78, 0.82, 1f32, 0.600f32);
    colors[ImGuiCol_ResizeGripActive]       = ImVec4::new2(0.78, 0.82, 1f32, 0.900f32);
    colors[ImGuiCol_Tab]                    = ImLerp(colors[ImGuiCol_Header],       colors[ImGuiCol_TitleBgActive], 0.800f32);
    colors[ImGuiCol_TabHovered]             = colors[ImGuiCol_HeaderHovered];
    colors[ImGuiCol_TabActive]              = ImLerp(colors[ImGuiCol_HeaderActive], colors[ImGuiCol_TitleBgActive], 0.600f32);
    colors[ImGuiCol_TabUnfocused]           = ImLerp(colors[ImGuiCol_Tab],          colors[ImGuiCol_TitleBg], 0.800f32);
    colors[ImGuiCol_TabUnfocusedActive]     = ImLerp(colors[ImGuiCol_TabActive],    colors[ImGuiCol_TitleBg], 0.400f32);
    colors[ImGuiCol_DockingPreview]         = colors[ImGuiCol_Header] * ImVec4::new2(1f32, 1f32, 1f32, 0.70f32);
    colors[ImGuiCol_DockingEmptyBg]         = ImVec4::new2(0.20f32, 0.20f32, 0.20f32, 1.000f32);
    colors[ImGuiCol_PlotLines]              = ImVec4::new2(1f32, 1f32, 1f32, 1.000f32);
    colors[ImGuiCol_PlotLinesHovered]       = ImVec4::new2(0.90f32, 0.70f32, 0.00f32, 1.000f32);
    colors[ImGuiCol_PlotHistogram]          = ImVec4::new2(0.90f32, 0.70f32, 0.00f32, 1.000f32);
    colors[ImGuiCol_PlotHistogramHovered]   = ImVec4::new2(1f32, 0.60f32, 0.00f32, 1.000f32);
    colors[ImGuiCol_TableHeaderBg]          = ImVec4::new2(0.27, 0.27, 0.38, 1.000f32);
    colors[ImGuiCol_TableBorderStrong]      = ImVec4::new2(0.31, 0.31, 0.45, 1.000f32);   // Prefer using Alpha=1.0 here
    colors[ImGuiCol_TableBorderLight]       = ImVec4::new2(0.26, 0.26, 0.28, 1.000f32);   // Prefer using Alpha=1.0 here
    colors[ImGuiCol_TableRowBg]             = ImVec4::new2(0.00f32, 0.00f32, 0.00f32, 0.000f32);
    colors[ImGuiCol_TableRowBgAlt]          = ImVec4::new2(1f32, 1f32, 1f32, 0.070f32);
    colors[ImGuiCol_TextSelectedBg]         = ImVec4::new2(0.00f32, 0.00f32, 1f32, 0.350f32);
    colors[ImGuiCol_DragDropTarget]         = ImVec4::new2(1f32, 1f32, 0.00f32, 0.900f32);
    colors[ImGuiCol_NavHighlight]           = colors[ImGuiCol_HeaderHovered];
    colors[ImGuiCol_NavWindowingHighlight]  = ImVec4::new2(1f32, 1f32, 1f32, 0.700f32);
    colors[ImGuiCol_NavWindowingDimBg]      = ImVec4::new2(0.80f32, 0.80f32, 0.80f32, 0.200f32);
    colors[ImGuiCol_ModalWindowDimBg]       = ImVec4::new2(0.20f32, 0.20f32, 0.20f32, 0.350f32);
}

// Those light colors are better suited with a thicker font than the default one + FrameBorder
pub fn StyleColorsLight(dst: *mut ImGuiStyle)
{
    let style = if dst { dst } else { &GetStyle() };
    let colors = style.Colors;

    colors[ImGuiCol_Text]                   = ImVec4::new2(0.00f32, 0.00f32, 0.00f32, 1.000f32);
    colors[ImGuiCol_TextDisabled]           = ImVec4::new2(0.60f32, 0.60f32, 0.60f32, 1.000f32);
    colors[ImGuiCol_WindowBg]               = ImVec4::new2(0.94, 0.94, 0.94, 1.000f32);
    colors[ImGuiCol_ChildBg]                = ImVec4::new2(0.00f32, 0.00f32, 0.00f32, 0.000f32);
    colors[ImGuiCol_PopupBg]                = ImVec4::new2(1f32, 1f32, 1f32, 0.980f32);
    colors[ImGuiCol_Border]                 = ImVec4::new2(0.00f32, 0.00f32, 0.00f32, 0.300f32);
    colors[ImGuiCol_BorderShadow]           = ImVec4::new2(0.00f32, 0.00f32, 0.00f32, 0.000f32);
    colors[ImGuiCol_FrameBg]                = ImVec4::new2(1f32, 1f32, 1f32, 1.000f32);
    colors[ImGuiCol_FrameBgHovered]         = ImVec4::new2(0.26, 0.59, 0.98, 0.400f32);
    colors[ImGuiCol_FrameBgActive]          = ImVec4::new2(0.26, 0.59, 0.98, 0.670f32);
    colors[ImGuiCol_TitleBg]                = ImVec4::new2(0.96, 0.96, 0.96, 1.000f32);
    colors[ImGuiCol_TitleBgActive]          = ImVec4::new2(0.82, 0.82, 0.82, 1.000f32);
    colors[ImGuiCol_TitleBgCollapsed]       = ImVec4::new2(1f32, 1f32, 1f32, 0.510f32);
    colors[ImGuiCol_MenuBarBg]              = ImVec4::new2(0.86, 0.86, 0.86, 1.000f32);
    colors[ImGuiCol_ScrollbarBg]            = ImVec4::new2(0.98, 0.98, 0.98, 0.530f32);
    colors[ImGuiCol_ScrollbarGrab]          = ImVec4::new2(0.69, 0.69, 0.69, 0.800f32);
    colors[ImGuiCol_ScrollbarGrabHovered]   = ImVec4::new2(0.49, 0.49, 0.49, 0.800f32);
    colors[ImGuiCol_ScrollbarGrabActive]    = ImVec4::new2(0.49, 0.49, 0.49, 1.000f32);
    colors[ImGuiCol_CheckMark]              = ImVec4::new2(0.26, 0.59, 0.98, 1.000f32);
    colors[ImGuiCol_SliderGrab]             = ImVec4::new2(0.26, 0.59, 0.98, 0.780f32);
    colors[ImGuiCol_SliderGrabActive]       = ImVec4::new2(0.46, 0.54, 0.80f32, 0.600f32);
    colors[ImGuiCol_Button]                 = ImVec4::new2(0.26, 0.59, 0.98, 0.400f32);
    colors[ImGuiCol_ButtonHovered]          = ImVec4::new2(0.26, 0.59, 0.98, 1.000f32);
    colors[ImGuiCol_ButtonActive]           = ImVec4::new2(0.06, 0.53, 0.98, 1.000f32);
    colors[ImGuiCol_Header]                 = ImVec4::new2(0.26, 0.59, 0.98, 0.310f32);
    colors[ImGuiCol_HeaderHovered]          = ImVec4::new2(0.26, 0.59, 0.98, 0.800f32);
    colors[ImGuiCol_HeaderActive]           = ImVec4::new2(0.26, 0.59, 0.98, 1.000f32);
    colors[ImGuiCol_Separator]              = ImVec4::new2(0.39, 0.39, 0.39, 0.620f32);
    colors[ImGuiCol_SeparatorHovered]       = ImVec4::new2(0.14, 0.44, 0.80f32, 0.780f32);
    colors[ImGuiCol_SeparatorActive]        = ImVec4::new2(0.14, 0.44, 0.80f32, 1.000f32);
    colors[ImGuiCol_ResizeGrip]             = ImVec4::new2(0.35, 0.35, 0.35, 0.170f32);
    colors[ImGuiCol_ResizeGripHovered]      = ImVec4::new2(0.26, 0.59, 0.98, 0.670f32);
    colors[ImGuiCol_ResizeGripActive]       = ImVec4::new2(0.26, 0.59, 0.98, 0.950f32);
    colors[ImGuiCol_Tab]                    = ImLerp(colors[ImGuiCol_Header],       colors[ImGuiCol_TitleBgActive], 0.900f32);
    colors[ImGuiCol_TabHovered]             = colors[ImGuiCol_HeaderHovered];
    colors[ImGuiCol_TabActive]              = ImLerp(colors[ImGuiCol_HeaderActive], colors[ImGuiCol_TitleBgActive], 0.600f32);
    colors[ImGuiCol_TabUnfocused]           = ImLerp(colors[ImGuiCol_Tab],          colors[ImGuiCol_TitleBg], 0.800f32);
    colors[ImGuiCol_TabUnfocusedActive]     = ImLerp(colors[ImGuiCol_TabActive],    colors[ImGuiCol_TitleBg], 0.400f32);
    colors[ImGuiCol_DockingPreview]         = colors[ImGuiCol_Header] * ImVec4::new2(1f32, 1f32, 1f32, 0.70f32);
    colors[ImGuiCol_DockingEmptyBg]         = ImVec4::new2(0.20f32, 0.20f32, 0.20f32, 1.000f32);
    colors[ImGuiCol_PlotLines]              = ImVec4::new2(0.39, 0.39, 0.39, 1.000f32);
    colors[ImGuiCol_PlotLinesHovered]       = ImVec4::new2(1f32, 0.43, 0.35, 1.000f32);
    colors[ImGuiCol_PlotHistogram]          = ImVec4::new2(0.90f32, 0.70f32, 0.00f32, 1.000f32);
    colors[ImGuiCol_PlotHistogramHovered]   = ImVec4::new2(1f32, 0.45, 0.00f32, 1.000f32);
    colors[ImGuiCol_TableHeaderBg]          = ImVec4::new2(0.78, 0.87, 0.98, 1.000f32);
    colors[ImGuiCol_TableBorderStrong]      = ImVec4::new2(0.57, 0.57, 0.64, 1.000f32);   // Prefer using Alpha=1.0 here
    colors[ImGuiCol_TableBorderLight]       = ImVec4::new2(0.68, 0.68, 0.74, 1.000f32);   // Prefer using Alpha=1.0 here
    colors[ImGuiCol_TableRowBg]             = ImVec4::new2(0.00f32, 0.00f32, 0.00f32, 0.000f32);
    colors[ImGuiCol_TableRowBgAlt]          = ImVec4::new2(0.3f32, 0.3f32, 0.3f32, 0.090f32);
    colors[ImGuiCol_TextSelectedBg]         = ImVec4::new2(0.26, 0.59, 0.98, 0.350f32);
    colors[ImGuiCol_DragDropTarget]         = ImVec4::new2(0.26, 0.59, 0.98, 0.950f32);
    colors[ImGuiCol_NavHighlight]           = colors[ImGuiCol_HeaderHovered];
    colors[ImGuiCol_NavWindowingHighlight]  = ImVec4::new2(0.70f32, 0.70f32, 0.70f32, 0.700f32);
    colors[ImGuiCol_NavWindowingDimBg]      = ImVec4::new2(0.20f32, 0.20f32, 0.20f32, 0.200f32);
    colors[ImGuiCol_ModalWindowDimBg]       = ImVec4::new2(0.20f32, 0.20f32, 0.20f32, 0.350f32);
}




// On AddPolyline() and AddConvexPolyFilled() we intentionally avoid using ImVec2 and superfluous function calls to optimize debug/non-inlined builds.
// - Those macros expects l-values and need to be used as their own statement.
// - Those macros are intentionally not surrounded by the 'do {} while (0)' idiom because even that translates to runtime with de *= inv_len2; VY *= inv_len2; } } (void)0





// #define IM_NORMALIZE2F_OVER_ZERO(VX,VY)     { float d2 = VX*VX + VY*VY; if (d2 > 0f32) { float inv_len = ImRsqrt(d2); VX *= inv_len; VY *= inv_len; } } (void)0
// #define IM_FIXNORMAL2F_MAX_INVLEN2          100f32 // 500f32 (see #4053, #3366)
// #define IM_FIXNORMAL2F(VX,VY)               { float d2 = VX*VX + VY*VY; if (d2 > 0.0000010f32) { float inv_len2 = 1f32 / d2; if (inv_len2 > IM_FIXNORMAL2F_MAX_INVLEN2) inv_len2 = IM_FIXNORMAL2F_MAX_INVLEN2; VXonst center: &ImVec2, radius: c_float, a_min: c_float, a_max: c_float, num_segments: c_int)







// Closely mimics ImBezierCubicClosestPointCasteljau() in imgui.cpp
pub fn PathBezierCubicCurveToCasteljau(path: &mut Vec<ImVec2>, x1: c_float, y1: c_float, x2: c_float, y2: c_float, x3: c_float, y3: c_float, x4: c_float, y4: c_float, tess_tol: c_float, level: c_int) {
    let dx: c_float = x4 - x1;
    let dy: c_float = y4 - y1;
    let mut d2: c_float = (x2 - x4) * dy - (y2 - y4) * dx;
    let mut d3: c_float = (x3 - x4) * dy - (y3 - y4) * dx;
    d2 = if (d2 >= 0) { d2 } else { -d2 };
    d3 = if (d3 >= 0) { d3 } else { -d3 };
    if (d2 + d3) * (d2 + d3) < tess_tol * (dx * dx + dy * dy) {
        path.push(ImVec2::new2(x4, y4));
    } else if (level < 10) {
        let x12: c_float = (x1 + x2) * 0.5f32;
        let y12 = (y1 + y2) * 0.5f32;
        let x23: c_float = (x2 + x3) * 0.5f32;
        let y23 = (y2 + y3) * 0.5f32;
        let x34: c_float = (x3 + x4) * 0.5f32;
        let y34 = (y3 + y4) * 0.5f32;
        let x123: c_float = (x12 + x23) * 0.5f32;
        let y123 = (y12 + y23) * 0.5f32;
        let x234: c_float = (x23 + x34) * 0.5f32;
        let y234 = (y23 + y34) * 0.5f32;
        let x1234: c_float = (x123 + x234) * 0.5f32;
        let y1234 = (y123 + y234) * 0.5f32;
        PathBezierCubicCurveToCasteljau(path, x1, y1, x12, y12, x123, y123, x1234, y1234, tess_tol, level + 1);
        PathBezierCubicCurveToCasteljau(path, x1234, y1234, x234, y234, x34, y34, x4, y4, tess_tol, level + 1);
    }
}

pub fn PathBezierQuadraticCurveToCasteljau(path: &mut Vec<ImVec2>, x1: c_float, y1: c_float, x2: c_float, y2: c_float, x3: c_float, y3: c_float, tess_tol: c_float, level: c_int) {
    let dx: c_float = x3 - x1;
    let dy = y3 - y1;
    let det: c_float = (x2 - x3) * dy - (y2 - y3) * dx;
    if det * det * 4.0f32 < tess_tol * (dx * dx + dy * dy) {
        path.push(ImVec2::new2(x3, y3));
    } else if (level < 10) {
        let x12: c_float = (x1 + x2) * 0.5f32;
        let y12 = (y1 + y2) * 0.5f32;
        let x23: c_float = (x2 + x3) * 0.5f32;
        let y23 = (y2 + y3) * 0.5f32;
        let x123: c_float = (x12 + x23) * 0.5f32;
        let y123 = (y12 + y23) * 0.5f32;
        PathBezierQuadraticCurveToCasteljau(path, x1, y1, x12, y12, x123, y123, tess_tol, level + 1);
        PathBezierQuadraticCurveToCasteljau(path, x123, y123, x23, y23, x3, y3, tess_tol, level + 1);
    }
}


// IM_STATIC_ASSERT(ImDrawFlags_RoundCornersTopLeft == (1 << 4));
pub fn FixRectCornerFlags(mut flags: ImDrawFlags) -> ImDrawFlags {
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    // Legacy Support for hard coded !0 (used to be a suggested equivalent to ImDrawCornerFlags_All)
    //   !0   --> ImDrawFlags_RoundCornersAll or 0
    if flags == !0 {
        return ImDrawFlags_RoundCornersAll;
    }

    // Legacy Support for hard coded 0x01 to 0x0f32 (matching 15 out of 16 old flags combinations)
    //   0x01 --> ImDrawFlags_RoundCornersTopLeft (VALUE 0x01 OVERLAPS ImDrawFlags_Closed but ImDrawFlags_Closed is never valid in this path!)
    //   0x02 --> ImDrawFlags_RoundCornersTopRight
    //   0x03 --> ImDrawFlags_RoundCornersTopLeft | ImDrawFlags_RoundCornersTopRight
    //   0x04 --> ImDrawFlags_RoundCornersBotLeft
    //   0x05 --> ImDrawFlags_RoundCornersTopLeft | ImDrawFlags_RoundCornersBotLeft
    //   ...
    //   0x0f32 --> ImDrawFlags_RoundCornersAll or 0
    // (See all values in ImDrawCornerFlags_)
    if flags >= 0x01 && flags <= 0x00f32 {
        return flags << 4;
    }

    // We cannot support hard coded 0x00 with 'float rounding > 0f32' --> replace with ImDrawFlags_RoundCornersNone or use 'float rounding = 0f32'
// #endif

    // If this triggers, please update your code replacing hardcoded values with new ImDrawFlags_RoundCorners* values.
    // Note that ImDrawFlags_Closed (== 0x01) is an invalid flag for AddRect(), AddRectFilled(), PathRect() etc...
    // IM_ASSERT((flags & 0x00f32) == 0 && "Misuse of legacy hardcoded ImDrawCornerFlags values!");

    if (flags & ImDrawFlags_RoundCornersMask_) == 0 {
        flags |= ImDrawFlags_RoundCornersAll;
    }

    return flags;
}

//-----------------------------------------------------------------------------
// [SECTION] Helpers ShadeVertsXXX functions
//-----------------------------------------------------------------------------

// Generic linear color gradient, write to RGB fields, leave A untouched.
pub fn ShadeVertsLinearColorGradientKeepAlpha(draw_list: *mut ImDrawList, vert_start_idx: c_int, vert_end_idx: c_int, gradient_p0: ImVec2, gradient_p1: ImVec2, col0: u32, col1: u32)
{
    let gradient_extent: ImVec2 = gradient_p1 - gradient_p0;
    let gradient_inv_length2: c_float =  1f32 / ImLengthSqr(&gradient_extent);
    let mut vert_start: *mut ImDrawVert = &mut draw_list.VtxBuffer + vert_start_idx;
    let mut vert_end: *mut ImDrawVert = &mut draw_list.VtxBuffer + vert_end_idx;
    let col0_r: c_int = (col0 >> IM_COL32_R_SHIFT) & 0xFF;
    let col0_g: c_int = (col0 >> IM_COL32_G_SHIFT) & 0xFF;
    let col0_b: c_int = ((col0 >> IM_COL32_B_SHIFT) & 0xFF) as c_int;
    let col_delta_r: c_int = ((col1 >> IM_COL32_R_SHIFT) & 0xF0f32) - col0_r;
    let col_delta_g: c_int = ((col1 >> IM_COL32_G_SHIFT) & 0xF0f32) - col0_g;
    let col_delta_b: c_int = (((col1 >> IM_COL32_B_SHIFT) & 0xF0f32) - col0_b) as c_int;
    // for (let mut vert: *mut ImDrawVert vert_start; vert < vert_end; vert++)
    for vert in vert_start .. vert_end
    {
        let d: c_float =  ImDot2( - gradient_p0, gradient_extent);
        let t: c_float =  ImClamp(d * gradient_inv_length2, 0f32, 1f32);
        let r: c_int = (col0_r + col_delta_r * t);
        let g: c_int = (col0_g + col_delta_g * t);
        let b: c_int = (col0_b + col_delta_b * t);
        // TODO: find  missing element 
        // = (r << IM_COL32_R_SHIFT) | (g << IM_COL32_G_SHIFT) | (b << IM_COL32_B_SHIFT) | ( & IM_COL32_A_MASK);
    }
}

// Distribute UV over (a, b) rectangle
pub fn ShadeVertsLinearUV(draw_list: *mut ImDrawList, vert_start_idx: c_int, vert_end_idx: c_int, a: &ImVec2, b: &ImVec2, uv_a: &ImVec2, uv_b: &ImVec2, clamp: bool)
{
    let size: ImVec2 = b - a;
    let uv_size: ImVec2 = uv_b - uv_a;
    let scale: ImVec2 = ImVec2::new2(
        if size.x != 0f32 { (uv_size.x / size.x) } else { 0f32 },
        if size.y != 0f32 { (uv_size.y / size.y) } else { 0f32 });

    let vert_start: *mut ImDrawVert = &mut draw_list.VtxBuffer + vert_start_idx;
    let vert_end: *mut ImDrawVert = &mut draw_list.VtxBuffer + vert_end_idx;
    if clamp
    {
        let min: ImVec2 = ImMin(uv_a.clone(), uv_b.clone());
        let max: ImVec2 = ImMax(uv_a.clone(), uv_b.clone());
        // for (ImDrawVert* vertex = vert_start; vertex < vert_end; ++vertex)
        //      = ImClamp(uv_a + ImMul(ImVec2::new2(.x, .y) - a, scale), min, max);
    }
    else
    {
        // for (ImDrawVert* vertex = vert_start; vertex < vert_end; ++vertex)
        //      = uv_a + ImMul(ImVec2::new2(.x, .y) - a, scale);
    }
}



// Default font TTF is compressed with stb_compress then base85 encoded (see misc/fonts/binary_to_compressed_c.cpp for encoder)
// static c_uint stb_decompress_length(const c_uchar* input);
// static c_uint stb_decompress(c_uchar* output, const c_uchar* input, c_uint length);
// static *const char  GetDefaultCompressedFontDataTTFBase85();
pub fn Decode85Byte(c: c_char)  -> c_uint                                  {
    return if c >= '\\' as c_char { c - 36 } else { c - 35 } as c_uint; }

pub unsafe fn Decode85(mut src: *const c_uchar, mut dst: *const c_uchar) {
    while (*src) {
        let mut tmp: c_uint = Decode85Byte(src[0]) + 85 * (Decode85Byte(src[1]) + 85 * (Decode85Byte(src[2]) + 85 * (Decode85Byte(src[3]) + 85 * Decode85Byte(src[4]))));
        dst[0] = ((tmp >> 0) & 0xF0F);
        dst[1] = ((tmp >> 8) & 0xF0F);
        dst[2] = ((tmp >> 16) & 0xF0F);
        dst[3] = ((tmp >> 24) & 0xF0F);   // We can't assume little-endianness.
        src += 5;
        dst += 4;
    }
}


pub fn   ImFontAtlasBuildMultiplyCalcLookupTable(mut out_table: [c_uchar;256], in_brighten_factor: c_float)
{
    // for (let mut i: c_uint =  0; i < 256; i++)
    for i in 0 .. 256
    {
        let mut value: c_uint =  (i * in_brighten_factor);
        out_table[i] = if value > 255 { 255 } else { value & 0xF0F };
    }
}

pub fn   ImFontAtlasBuildMultiplyRectAlpha8(table: [c_uchar;256],pixels: *mut c_uchar, x: c_int, y: c_int, w: c_int, h: c_int, stride: c_int)
{
    led data: *mut c_uchar = pixels + x + y * stride;
    // for (let j: c_int = h; j > 0; j--, data += stride)
    for j in h .. 0
    {
        // for (let i: c_int = 0; i < w; i++)
        for i in 0..w {
            data[i] = table[data[i]];
        }
    }
}

pub fn UnpackBitVectorToFlatIndexList(in_list: *mut ImBitVector, out_list: &mut Vec<c_int>) {
    // IM_ASSERT(sizeof(in.Storage.Data[0]) == sizeof);
    // TODO: find missing elements
    // let it_begin: *const u32 = .begin();
    // let it_end: *const u32 = .end();
    // for (* let it: u32 = it_begin; it < it_end; it+ +)
    // {
    //     if let mut entries_32: u32 = *it
    //     {
    //         // for ( let mut bit_n: u32 = 0;bit_n < 32; bit_n++ )
    //     for bit_n in 0 .. 32
    //     {
    //         if entries_32 & (1 < < bit_n) {
    //         out.push((((it - it_begin) < < 5) + bit_n));
    //         }
    //         }
    //     }
    // }
}

pub unsafe fn ImFontAtlasBuildWithStbTruetype(mut atlas: *mut ImFontAtlas) -> bool
{
    // IM_ASSERT(atlas->ConfigData.Size > 0);

    ImFontAtlasBuildInit(atlas);

    // Clear atlas
    atlas.TexID = null_mut();
    atlas.TexWidth = 0;
    atlas.TexHeight = 0;
    atlas.TexUvScale = ImVec2::new2(0.0, 0.0);
    atlas.TexUvWhitePixel = ImVec2(0.0, 0.0);
    atlas.ClearTexData();

    // Temporary storage for building
    let mut src_tmp_array: Vec<ImFontBuildSrcData> = vec![];
    let mut dst_tmp_array: Vec<ImFontBuildDstData> = vec![];
    // TODO: find missing element
    // src_tmp_array.resize(.Size);
    src_tmp_array.reserve(atlas.ConfigData.len());
    // dst_tmp_array.resize(.Size);
    dst_tmp_array.reserve(atlas.Fonts.len());
    libc::memset(src_tmp_array.Data, 0, src_tmp_array.size_in_bytes());
    libc::memset(dst_tmp_array.Data, 0, dst_tmp_array.size_in_bytes());

    // 1. Initialize font loading structure, check font data validity
    // for (let src_i: c_int = 0; src_i < atlas.Confi.Size; src_i++)
    for src_i in 0 .. atlas.ConfigData.len()
    {
        ImFontBuildSrcData& src_tmp = src_tmp_array[src_i];
        ImFontConfig& cfg = atlas->ConfigData[src_i];
        // IM_ASSERT(cfg.DstFont && (!cfg.DstFont->IsLoaded() || cfg.DstFont->ContainerAtlas == atlas));

        // Find index from cfg.DstFont (we allow the user to set cfg.DstFont. Also it makes casual debugging nicer than when storing indices)
        src_tmp.DstIndex = -1;
        // for (let output_i: c_int = 0; output_i < .Size && src_tmp.DstIndex == -1; output_i++)
        for output_i in 0 .. atlas.Fonts.len()
        {
            if cfg.DstFont == atlas.Fonts[output_i] {
                src_tmp.DstIndex = output_i;
            }
            if src_tmp.DstIndex != -1 {
                break;
            }
        }
        if src_tmp.DstIndex == -1
        {
            // IM_ASSERT(src_tmp.DstIndex != -1); // cfg.DstFont not pointing within atlas->Fonts[] array?
            return false;
        }
        // Initialize helper structure for font loading and verify that the TTF/OTF data is correct
        let font_offset: c_int = stbtt_GetFontOffsetForIndex(cfg.FontData, cfg.FontNo);
        // IM_ASSERT(font_offset >= 0 && "FontData is incorrect, or FontNo cannot be found.");
        if !stbtt_InitFont(&src_tmp.FontInfo, cfg.FontData, font_offset)
        {
            return false;
        }

        // Measure highest codepoints
        ImFontBuildDstData& dst_tmp = dst_tmp_array[src_tmp.DstIndex];
        src_tmp.SrcRanges = if cfg.GlyphRanges { cfg.GlyphRanges } : atlas.GetGlyphRangesDefault();
        // for (src_range: ImWchar = src_tmp.SrcRanges; src_range[0] && src_range[1]; src_range += 2)
        let mut src_range = src_tmp.SrcRanges;
        while src_range[0] && src_range[1]
        {
            src_tmp.GlyphsHighest = ImMax(src_tmp.GlyphsHighest, src_range[1]);
            src_range += 2;
        }
        dst_tmp.SrcCount+= 1;
        dst_tmp.GlyphsHighest = ImMax(dst_tmp.GlyphsHighest, src_tmp.GlyphsHighest);
    }

    // 2. For every requested codepoint, check for their presence in the font data, and handle redundancy or overlaps between source fonts to avoid unused glyphs.
    let total_glyphs_count: c_int = 0;
    // for (let src_i: c_int = 0; src_i < src_tmp_array.Size; src_i++)
    for src_i in 0 .. src_tmp_array.len()
    {
        let src_tmp: *mut ImFontBuildSrcData = &mut src_tmp_array[src_i];
        let dst_tmp: *mut ImFontBuildSrcData = &mut dst_tmp_array[src_tmp.DstIndex.clone()];
        src_tmp.GlyphsSet.Create(&src_tmp.GlyphsHighest + 1);
        if dst_tmp.GlyphsSet.Storage.empty()
        {
            dst_tmp.GlyphsSet.Create(&dst_tmp.GlyphsHighest + 1);
        }

        for (*const let src_range: ImWchar = src_tmp.SrcRanges; src_range[0] && src_range[1]; src_range += 2)
        for (let mut codepoint: c_uint =  src_range[0]; codepoint <= src_range[1]; codepoint++)
        {
            if dst_tmp.GlyphsSet.TestBit(codepoint))    // Don't overwrite existing glyphs. We could make this an option for MergeMode (e.g. MergeOverwrite==true
            {
                continue;
            }
            if (!stbtt_FindGlyphIndex(&src_tmp.FontInfo, codepoint))    // It is actually in the font?
            continue;

            // Add to avail set/counters
            src_tmp.GlyphsCount+= 1;
            dst_tmp.GlyphsCount+= 1;
            src_tmp.GlyphsSet.SetBit(codepoint);
            dst_tmp.GlyphsSet.SetBit(codepoint);
            total_glyphs_count+= 1;
        }
    }

    // 3. Unpack our bit map into a flat list (we now have all the Unicode points that we know are requested _and_ available _and_ not overlapping another)
    for (c_int src_i = 0; src_i < src_tmp_array.Size; src_i++)
    {
        ImFontBuildSrcData& src_tmp = src_tmp_array[src_i];
        src_tmp.GlyphsList.reserve(src_tmp.GlyphsCount);
        UnpackBitVectorToFlatIndexList(&src_tmp.GlyphsSet, &src_tmp.GlyphsList);
        src_tmp.GlyphsSet.Clear();
        // IM_ASSERT(src_tmp.GlyphsList.Size == src_tmp.GlyphsCount);
    }
    for (let dst_i: c_int = 0; dst_i < dst_tmp_array.Size; dst_i++)
    dst_tmp_array[dst_i].GlyphsSet.Clear();
    dst_tmp_array.clear();

    // Allocate packing character data and flag packed characters buffer as non-packed (x0=y0=x1=y1=0)
    // (We technically don't need to zero-clear buf_rects, but let's do it for the sake of sanity)
    // Vec<stbrp_rect> buf_rects;
    let mut buf_rects: Vec<stbrp_rect> = vec![];
    // Vec<stbtt_packedchar> buf_packedchars;
    let mut buf_packedchars: Vec< >
    buf_rects.resize(total_glyphs_count);
    buf_packedchars.resize(total_glyphs_count);
    memset(buf_rects.Data, 0, buf_rects.size_in_bytes());
    memset(buf_packedchars.Data, 0, buf_packedchars.size_in_bytes());

    // 4. Gather glyphs sizes so we can pack them in our virtual canvas.
    c_int total_surface = 0;
    c_int buf_rects_out_n = 0;
    c_int buf_packedchars_out_n = 0;
    for (c_int src_i = 0; src_i < src_tmp_array.Size; src_i++)
    {
        ImFontBuildSrcData& src_tmp = src_tmp_array[src_i];
        if src_tmp.GlyphsCount == 0
        {
            continue;
        }

        src_tmp.Rects = &buf_rects[buf_rects_out_n];
        src_tmp.PackedChars = &buf_packedchars[buf_packedchars_out_n];
        buf_rects_out_n += src_tmp.GlyphsCount;
        buf_packedchars_out_n += src_tmp.GlyphsCount;

        // Convert our ranges in the format stb_truetype wants
        ImFontConfig& cfg = atlas->ConfigData[src_i];
        src_tmp.PackRange.font_size = cfg.SizePixels;
        src_tmp.PackRange.first_unicode_codepoint_in_range = 0;
        src_tmp.PackRange.array_of_unicode_codepoints = src_tmp.GlyphsList.Data;
        src_tmp.PackRange.num_chars = src_tmp.GlyphsList.Size;
        src_tmp.PackRange.chardata_for_range = src_tmp.PackedChars;
        src_tmp.PackRange.h_oversample = (c_uchar)cfg.OversampleH;
        src_tmp.PackRange.v_oversample = (c_uchar)cfg.OversampleV;

        // Gather the sizes of all rectangles we will need to pack (this loop is based on stbtt_PackFontRangesGatherRects)
        let         : c_float =  (cfg.SizePixels > 0) ? stbtt_ScaleForPixelHeight(&src_tmp.FontInfo, cfg.SizePixels) : stbtt_ScaleForMappingEmToPixels(&src_tmp.FontInfo, -cfg.SizePixels);
        let padding: c_int = atlas->TexGlyphPadding;
        for (c_int glyph_i = 0; glyph_i < src_tmp.GlyphsList.Size; glyph_i++)
        {
            c_int x0, y0, x1, y1;
            let glyph_index_in_font: c_int = stbtt_FindGlyphIndex(&src_tmp.FontInfo, src_tmp.GlyphsList[glyph_i]);
            // IM_ASSERT(glyph_index_in_font != 0);
            stbtt_GetGlyphBitmapBoxSubpixel(&src_tmp.FontInfo, glyph_index_in_font, scale * cfg.OversampleH, scale * cfg.OversampleV, 0, 0, &x0, &y0, &x1, &y1);
            src_tmp.Rects[glyph_i].w = (stbrp_coord)(x1 - x0 + padding + cfg.OversampleH - 1);
            src_tmp.Rects[glyph_i].h = (stbrp_coord)(y1 - y0 + padding + cfg.OversampleV - 1);
            total_surface += src_tmp.Rects[glyph_i].w * src_tmp.Rects[glyph_i].h;
        }
    }

    // We need a width for the skyline algorithm, any width!
    // The exact width doesn't really matter much, but some API/GPU have texture size limitations and increasing width can decrease height.
    // User can override TexDesiredWidth and TexGlyphPadding if they wish, otherwise we use a simple heuristic to select the width based on expected surface.
    let surface_sqrt: c_int = ImSqrt(total_surface) + 1;
    = 0;
    if  > 0
    {
        = ;
    }


    // 5. Start packing
    // Pack our extra data rectangles first, so it will be on the upper-left corner of our texture (UV will have small values).
    let TEX_HEIGHT_MAX: c_int = 1024 * 32;
    stbtt_pack_context spc = {};
    stbtt_PackBegin(&spc, NULL, atlas->TexWidth, TEX_HEIGHT_MAX, 0, atlas->TexGlyphPadding, NULL);
    ImFontAtlasBuildPackCustomRects(atlas, spc.pack_info);

    // 6. Pack each source font. No rendering yet, we are working with rectangles in an infinitely tall texture at this point.
    for (c_int src_i = 0; src_i < src_tmp_array.Size; src_i++)
    {
        ImFontBuildSrcData& src_tmp = src_tmp_array[src_i];
        if src_tmp.GlyphsCount == 0
        {
            continue;
        }

        stbrp_pack_rects((stbrp_context*)spc.pack_info, src_tmp.Rects, src_tmp.GlyphsCount);

        // Extend texture height and mark missing glyphs as non-packed so we won't render them.
        // FIXME: We are not handling packing failure here (would happen if we got off TEX_HEIGHT_MAX or if a single if larger than TexWidth?)
        for (let glyph_i: c_int = 0; glyph_i < src_tmp.GlyphsCount; glyph_i++)
        if src_tmp.Rects[glyph_i].was_packed
        {
            = ImMax(, src_tmp.Rects[glyph_i].y + src_tmp.Rects[glyph_i].h);
        }
    }

        // 7. Allocate texture
        = ( & ImFontAtlasFlags_NoPowerOfTwoHeight) ? ( + 1) : ImUpperPowerOfTwo();
    = ImVec2::new2(1f32 / , 1f32 / );
    = IM_ALLOC( * );
    memset(, 0,  * );
    spc.pixels = ;
    spc.height = ;

    // 8. Render/rasterize font characters into the texture
    for (c_int src_i = 0; src_i < src_tmp_array.Size; src_i++)
    {
        ImFontConfig& cfg = atlas->ConfigData[src_i];
        ImFontBuildSrcData& src_tmp = src_tmp_array[src_i];
        if src_tmp.GlyphsCount == 0
        {
            continue;
        }

        stbtt_PackFontRangesRenderIntoRects(&spc, &src_tmp.FontInfo, &src_tmp.PackRange, 1, src_tmp.Rects);

        // Apply multiply operator
        if (cfg.RasterizerMultiply != 1f32)
        {
            unsigned multiply_table: [c_char;256];
            ImFontAtlasBuildMultiplyCalcLookupTable(multiply_table, cfg.RasterizerMultiply);
            stbrp_rect* r = &src_tmp.Rects[0];
            for (let glyph_i: c_int = 0; glyph_i < src_tmp.GlyphsCount; glyph_i++, r++)
            if
            {
                ImFontAtlasBuildMultiplyRectAlpha8(multiply_table, , , , , ,  * 1);
            }
        }
        src_tmp.Rects = None;
    }

    // End packing
    stbtt_PackEnd(&spc);
    buf_rects.clear();

    // 9. Setup ImFont and glyphs for runtime
    for (c_int src_i = 0; src_i < src_tmp_array.Size; src_i++)
    {
        ImFontBuildSrcData& src_tmp = src_tmp_array[src_i];
        if src_tmp.GlyphsCount == 0
        {
            continue;
        }

        // When merging fonts with MergeMode=true:
        // - We can have multiple input fonts writing into a same destination font.
        // - dst_font->ConfigData is != from cfg which is our source configuration.
        ImFontConfig& cfg = atlas->ConfigData[src_i];
        ImFont* dst_font = cfg.DstFont;

        let
        : c_float =  stbtt_ScaleForPixelHeight(&src_tmp.FontInfo, cfg.SizePixels);
        c_int unscaled_ascent, unscaled_descent, unscaled_line_gap;
        stbtt_GetFontVMetrics(&src_tmp.FontInfo, &unscaled_ascent, &unscaled_descent, &unscaled_line_gap);

        let
        : c_float =  ImFloor(unscaled_ascent * font_scale + ((unscaled_ascent > 0f32) ? +1 : -1));
        let         : c_float =  ImFloor(unscaled_descent * font_scale + ((unscaled_descent > 0f32) ? +1 : -1));
        ImFontAtlasBuildSetupFont(atlas, dst_font, &cfg, ascent, descent);
        let         : c_float =  cfg.GlyphOffset.x;
        let         : c_float =  cfg.GlyphOffset.y + IM_ROUND(dst_font->Ascent);

        for (c_int glyph_i = 0; glyph_i < src_tmp.GlyphsCount; glyph_i++)
        {
            // Register glyph
            let codepoint: c_int = src_tmp.GlyphsList[glyph_i];
            const stbtt_packedchar& pc = src_tmp.PackedChars[glyph_i];
            stbtt_aligned_quad q;
            c_float unused_x = 0f32, unused_y = 0f32;
            stbtt_GetPackedQuad(src_tmp.PackedChars, atlas->TexWidth, atlas->TexHeight, glyph_i, &unused_x, &unused_y, &q, 0);
            dst_font->AddGlyph(&cfg, codepoint, q.x0 + font_off_x, q.y0 + font_off_y, q.x1 + font_off_x, q.y1 + font_off_y, q.s0, q.t0, q.s1, q.t1, pc.xadvance);
        }
    }

    // Cleanup
    src_tmp_array.clear_destruct();

    ImFontAtlasBuildFinish(atlas);
    return true;
}

*const ImFontBuilderIO ImFontAtlasGetBuilderForStbTruetype()
{
    static ImFontBuilderIO io;
    io.FontBuilder_Build = ImFontAtlasBuildWithStbTruetype;
    return &io;
}

// #endif // IMGUI_ENABLE_STB_TRUETYPE

c_void ImFontAtlasBuildSetupFont(ImFontAtlas* atlas, ImFont* font, ImFontConfig* font_config, c_float ascent, c_float descent)
{
    if (!font_config->MergeMode)
    {
        font->ClearOutputData();
        font->FontSize = font_config->SizePixels;
        font->ConfigData = font_config;
        font->ConfigDataCount = 0;
        font->ContainerAtlas = atlas;
        font->Ascent = ascent;
        font->Descent = descent;
    }
    font->ConfigDataCount+= 1;
}

c_void ImFontAtlasBuildPackCustomRects(ImFontAtlas* atlas, stbrp_context_opaque: *mut c_void)
{
    stbrp_context* pack_context = (stbrp_context*)stbrp_context_opaque;
    // IM_ASSERT(pack_context != NULL);

    Vec<ImFontAtlasCustomRect>& user_rects = atlas->CustomRects;
    // IM_ASSERT(user_rects.Size >= 1); // We expect at least the default custom rects to be registered, else something went wrong.

    Vec<stbrp_rect> pack_rects;
    pack_rects.resize(user_rects.Size);
    memset(pack_rects.Data, 0, pack_rects.size_in_bytes());
    for (c_int i = 0; i < user_rects.Size; i++)
    {
        pack_rects[i].w = user_rects[i].Width;
        pack_rects[i].h = user_rects[i].Height;
    }
    stbrp_pack_rects(pack_context, &pack_rects[0], pack_rects.Size);
    for (c_int i = 0; i < pack_rects.Size; i++)
        if (pack_rects[i].was_packed)
        {
            user_rects[i].X = pack_rects[i].x;
            user_rects[i].Y = pack_rects[i].y;
            // IM_ASSERT(pack_rects[i].w == user_rects[i].Width && pack_rects[i].h == user_rects[i].Height);
            atlas->TexHeight = ImMax(atlas->TexHeight, pack_rects[i].y + pack_rects[i].h);
        }
}

c_void ImFontAtlasBuildRender8bppRectFromString(ImFontAtlas* atlas, c_int x, c_int y, c_int w, c_int h, *const char in_str, char in_marker_char, c_uchar in_marker_pixel_value)
{
    // IM_ASSERT(x >= 0 && x + w <= atlas->TexWidth);
    // IM_ASSERT(y >= 0 && y + h <= atlas->TexHeight);
    c_uchar* out_pixel = atlas->TexPixelsAlpha8 + x + (y * atlas->TexWidth);
    for (c_int off_y = 0; off_y < h; off_y++, out_pixel += atlas->TexWidth, in_str += w)
        for (c_int off_x = 0; off_x < w; off_x++)
            out_pixel[off_x] = (in_str[off_x] == in_marker_char) ? in_marker_pixel_value : 0x00;
}

c_void ImFontAtlasBuildRender32bppRectFromString(ImFontAtlas* atlas, c_int x, c_int y, c_int w, c_int h, *const char in_str, char in_marker_char, c_uint in_marker_pixel_value)
{
    // IM_ASSERT(x >= 0 && x + w <= atlas->TexWidth);
    // IM_ASSERT(y >= 0 && y + h <= atlas->TexHeight);
    c_uint* out_pixel = atlas->TexPixelsRGBA32 + x + (y * atlas->TexWidth);
    for (c_int off_y = 0; off_y < h; off_y++, out_pixel += atlas->TexWidth, in_str += w)
        for (c_int off_x = 0; off_x < w; off_x++)
            out_pixel[off_x] = (in_str[off_x] == in_marker_char) ? in_marker_pixel_value : IM_COL32_BLACK_TRANS;
}

static c_void ImFontAtlasBuildRenderDefaultTexData(ImFontAtlas* atlas)
{
    ImFontAtlasCustomRect* r = atlas->GetCustomRectByIndex(atlas->PackIdMouseCursors);
    // IM_ASSERT(r->IsPacked());

    let w: c_int = atlas->TexWidth;
    if (!(atlas->Flags & ImFontAtlasFlags_NoMouseCursors))
    {
        // Render/copy pixels
        // IM_ASSERT(r->Width == FONT_ATLAS_DEFAULT_TEX_DATA_W * 2 + 1 && r->Height == FONT_ATLAS_DEFAULT_TEX_DATA_H);
        let x_for_white: c_int = r->X;
        let x_for_black: c_int = r->X + FONT_ATLAS_DEFAULT_TEX_DATA_W + 1;
        if (atlas->TexPixelsAlpha8 != NULL)
        {
            ImFontAtlasBuildRender8bppRectFromString(atlas, x_for_white, r->Y, FONT_ATLAS_DEFAULT_TEX_DATA_W, FONT_ATLAS_DEFAULT_TEX_DATA_H, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS, '.', 0xF0f32);
            ImFontAtlasBuildRender8bppRectFromString(atlas, x_for_black, r->Y, FONT_ATLAS_DEFAULT_TEX_DATA_W, FONT_ATLAS_DEFAULT_TEX_DATA_H, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS, 'X', 0xF0f32);
        }
        else
        {
            ImFontAtlasBuildRender32bppRectFromString(atlas, x_for_white, r->Y, FONT_ATLAS_DEFAULT_TEX_DATA_W, FONT_ATLAS_DEFAULT_TEX_DATA_H, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS, '.', IM_COL32_WHITE);
            ImFontAtlasBuildRender32bppRectFromString(atlas, x_for_black, r->Y, FONT_ATLAS_DEFAULT_TEX_DATA_W, FONT_ATLAS_DEFAULT_TEX_DATA_H, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS, 'X', IM_COL32_WHITE);
        }
    }
    else
    {
        // Render 4 white pixels
        // IM_ASSERT(r->Width == 2 && r->Height == 2);
        let offset: c_int = r->X + r->Y * w;
        if (atlas->TexPixelsAlpha8 != NULL)
        {
            atlas->TexPixelsAlpha8[offset] = atlas->TexPixelsAlpha8[offset + 1] = atlas->TexPixelsAlpha8[offset + w] = atlas->TexPixelsAlpha8[offset + w + 1] = 0xFF;
        }
        else
        {
            atlas->TexPixelsRGBA32[offset] = atlas->TexPixelsRGBA32[offset + 1] = atlas->TexPixelsRGBA32[offset + w] = atlas->TexPixelsRGBA32[offset + w + 1] = IM_COL32_WHITE;
        }
    }
     = ImVec2::new2(( + 0.5f32) * .x, ( + 0.5f32) * .y);
}

static c_void ImFontAtlasBuildRenderLinesTexData(ImFontAtlas* atlas)
{
    if  & ImFontAtlasFlags_NoBakedLines
{
        return;
}

    // This generates a triangular shape in the texture, with the various line widths stacked on top of each other to allow interpolation between them
    ImFontAtlasCustomRect* r = atlas->GetCustomRectByIndex(atlas->PackIdLines);
    // IM_ASSERT(r->IsPacked());
    for (c_uint n = 0; n < IM_DRAWLIST_TEX_LINES_WIDTH_MAX + 1; n++) // +1 because of the zero-width row
    {
        // Each line consists of at least two empty pixels at the ends, with a line of solid pixels in the middle
        c_uint y = n;
        c_uint line_width = n;
        c_uint pad_left = (r->Width - line_width) / 2;
        c_uint pad_right = r->Width - (pad_left + line_width);

        // Write each slice
        // IM_ASSERT(pad_left + line_width + pad_right == r->Width && y < r->Height); // Make sure we're inside the texture bounds before we start writing pixels
        if (atlas->TexPixelsAlpha8 != NULL)
        {
            c_uchar* write_ptr = &atlas->TexPixelsAlpha8[r->X + ((r->Y + y) * atlas->TexWidth)];
            for (c_uint i = 0; i < pad_left; i++)
                *(write_ptr + i) = 0x00;

            for (c_uint i = 0; i < line_width; i++)
                *(write_ptr + pad_left + i) = 0xFF;

            for (c_uint i = 0; i < pad_right; i++)
                *(write_ptr + pad_left + line_width + i) = 0x00;
        }
        else
        {
            c_uint* write_ptr = &atlas->TexPixelsRGBA32[r->X + ((r->Y + y) * atlas->TexWidth)];
            for (c_uint i = 0; i < pad_left; i++)
                *(write_ptr + i) = IM_COL32(255, 255, 255, 0);

            for (c_uint i = 0; i < line_width; i++)
                *(write_ptr + pad_left + i) = IM_COL32_WHITE;

            for (c_uint i = 0; i < pad_right; i++)
                *(write_ptr + pad_left + line_width + i) = IM_COL32(255, 255, 255, 0);
        }

        // Calculate UVs for this line
        let uv0: ImVec2 = ImVec2::new2(( + pad_left - 1), ( + y)) * ;
        let uv1: ImVec2 = ImVec2::new2(( + pad_left + line_width + 1), ( + y + 1)) * ;
        let half_v: c_float =  (uv0.y + uv1.y) * 0.5f32; // Calculate a constant V in the middle of the row to avoid sampling artifacts
        [n] = ImVec4::new2(uv0.x, half_v, uv1.x, half_v);
    }
}

// Note: this is called / shared by both the stb_truetype and the FreeType builder
c_void ImFontAtlasBuildInit(ImFontAtlas* atlas)
{
    // Register texture region for mouse cursors or standard white pixels
    if (atlas->PackIdMouseCursors < 0)
    {
        if !( & ImFontAtlasFlags_NoMouseCursors)
{
             = (FONT_ATLAS_DEFAULT_TEX_DATA_W * 2 + 1, FONT_ATLAS_DEFAULT_TEX_DATA_H);
}

    }

    // Register texture region for thick lines
    // The +2 here is to give space for the end caps, whilst height +1 is to accommodate the fact we have a zero-width row
    if (atlas->PackIdLines < 0)
    {
        if !( & ImFontAtlasFlags_NoBakedLines)
{
             = (IM_DRAWLIST_TEX_LINES_WIDTH_MAX + 2, IM_DRAWLIST_TEX_LINES_WIDTH_MAX + 1);
}
    }
}

// This is called/shared by both the stb_truetype and the FreeType builder.
c_void ImFontAtlasBuildFinish(ImFontAtlas* atlas)
{
    // Render into our custom data blocks
    // IM_ASSERT(atlas->TexPixelsAlpha8 != NULL || atlas->TexPixelsRGBA32 != NULL);
    ImFontAtlasBuildRenderDefaultTexData(atlas);
    ImFontAtlasBuildRenderLinesTexData(atlas);

    // Register custom rectangle glyphs
    for (c_int i = 0; i < atlas->CustomRects.Size; i++)
    {
        let r: *const ImFontAtlasCustomRect = &[i];
        if  == null_mut() ||  == 0
{
            continue;
}

        // Will ignore ImFontConfig settings: GlyphMinAdvanceX, GlyphMinAdvanceY, GlyphExtraSpacing, PixelSnapH
        // IM_ASSERT(r->Font->ContainerAtlas == atlas);
        ImVec2 uv0, uv1;
        atlas->CalcCustomRectUV(r, &uv0, &uv1);
        r->Font->AddGlyph(NULL, r->GlyphID, r->GlyphOffset.x, r->GlyphOffset.y, r->GlyphOffset.x + r->Width, r->GlyphOffset.y + r->Height, uv0.x, uv0.y, uv1.x, uv1.y, r->GlyphAdvanceX);
    }

    // Build all fonts lookup tables
    for (let i: c_int = 0; i < .Size; i++)
        if [i].DirtyLookupTables
{
            [i].BuildLookupTable();
}

    atlas->TexReady = true;
}

// Retrieve list of range (2 int per range, values are inclusive)
*const ImWchar   ImFontAtlas::GetGlyphRangesDefault()
{
    static const ImWchar ranges[] =
    {
        0x0020, 0x00FF, // Basic Latin + Latin Supplement
        0,
    };
    return &ranges[0];
}

*const ImWchar  ImFontAtlas::GetGlyphRangesKorean()
{
    static const ImWchar ranges[] =
    {
        0x0020, 0x00FF, // Basic Latin + Latin Supplement
        0x3131, 0x3163, // Korean alphabets
        0xAC00, 0xD7A3, // Korean characters
        0xFFFD, 0xFFFD, // Invalid
        0,
    };
    return &ranges[0];
}

*const ImWchar  ImFontAtlas::GetGlyphRangesChineseFull()
{
    static const ImWchar ranges[] =
    {
        0x0020, 0x00FF, // Basic Latin + Latin Supplement
        0x2000, 0x206F, // General Punctuation
        0x3000, 0x30FF, // CJK Symbols and Punctuations, Hiragana, Katakana
        0x31F0, 0x31FF, // Katakana Phonetic Extensions
        0xFF00, 0xFFEF, // Half-width characters
        0xFFFD, 0xFFFD, // Invalid
        0x4e00, 0x9FAF, // CJK Ideograms
        0,
    };
    return &ranges[0];
}

static c_void UnpackAccumulativeOffsetsIntoRanges(c_int base_codepoint, *const c_short accumulative_offsets, c_int accumulative_offsets_count, ImWchar* out_ranges)
{
    for (c_int n = 0; n < accumulative_offsets_count; n++, out_ranges += 2)
    {
        out_ranges[0] = out_ranges[1] = (base_codepoint + accumulative_offsets[n]);
        base_codepoint += accumulative_offsets[n];
    }
    out_ranges[0] = 0;
}

//-------------------------------------------------------------------------
// [SECTION] ImFontAtlas glyph ranges helpers
//-------------------------------------------------------------------------

*const ImWchar  ImFontAtlas::GetGlyphRangesChineseSimplifiedCommon()
{
    // Store 2500 regularly used characters for Simplified Chinese.
    // Sourced from https://zh.wiktionary.org/wiki/%E9%99%84%E5%BD%95:%E7%8E%B0%E4%BB%A3%E6%B1%89%E8%AF%AD%E5%B8%B8%E7%94%A8%E5%AD%97%E8%A1%A8
    // This table covers 97.97% of all characters used during the month in July, 1987.
    // You can use ImFontGlyphRangesBuilder to create your own ranges derived from this, by merging existing ranges or adding new characters.
    // (Stored as accumulative offsets from the initial unicode codepoint 0x4E00. This encoding is designed to helps us compact the source code size.)
    static const c_short accumulative_offsets_from_0x4E00[] =
    {
        0,1,2,4,1,1,1,1,2,1,3,2,1,2,2,1,1,1,1,1,5,2,1,2,3,3,3,2,2,4,1,1,1,2,1,5,2,3,1,2,1,2,1,1,2,1,1,2,2,1,4,1,1,1,1,5,10,1,2,19,2,1,2,1,2,1,2,1,2,
        1,5,1,6,3,2,1,2,2,1,1,1,4,8,5,1,1,4,1,1,3,1,2,1,5,1,2,1,1,1,10,1,1,5,2,4,6,1,4,2,2,2,12,2,1,1,6,1,1,1,4,1,1,4,6,5,1,4,2,2,4,10,7,1,1,4,2,4,
        2,1,4,3,6,10,12,5,7,2,14,2,9,1,1,6,7,10,4,7,13,1,5,4,8,4,1,1,2,28,5,6,1,1,5,2,5,20,2,2,9,8,11,2,9,17,1,8,6,8,27,4,6,9,20,11,27,6,68,2,2,1,1,
        1,2,1,2,2,7,6,11,3,3,1,1,3,1,2,1,1,1,1,1,3,1,1,8,3,4,1,5,7,2,1,4,4,8,4,2,1,2,1,1,4,5,6,3,6,2,12,3,1,3,9,2,4,3,4,1,5,3,3,1,3,7,1,5,1,1,1,1,2,
        3,4,5,2,3,2,6,1,1,2,1,7,1,7,3,4,5,15,2,2,1,5,3,22,19,2,1,1,1,1,2,5,1,1,1,6,1,1,12,8,2,9,18,22,4,1,1,5,1,16,1,2,7,10,15,1,1,6,2,4,1,2,4,1,6,
        1,1,3,2,4,1,6,4,5,1,2,1,1,2,1,10,3,1,3,2,1,9,3,2,5,7,2,19,4,3,6,1,1,1,1,1,4,3,2,1,1,1,2,5,3,1,1,1,2,2,1,1,2,1,1,2,1,3,1,1,1,3,7,1,4,1,1,2,1,
        1,2,1,2,4,4,3,8,1,1,1,2,1,3,5,1,3,1,3,4,6,2,2,14,4,6,6,11,9,1,15,3,1,28,5,2,5,5,3,1,3,4,5,4,6,14,3,2,3,5,21,2,7,20,10,1,2,19,2,4,28,28,2,3,
        2,1,14,4,1,26,28,42,12,40,3,52,79,5,14,17,3,2,2,11,3,4,6,3,1,8,2,23,4,5,8,10,4,2,7,3,5,1,1,6,3,1,2,2,2,5,28,1,1,7,7,20,5,3,29,3,17,26,1,8,4,
        27,3,6,11,23,5,3,4,6,13,24,16,6,5,10,25,35,7,3,2,3,3,14,3,6,2,6,1,4,2,3,8,2,1,1,3,3,3,4,1,1,13,2,2,4,5,2,1,14,14,1,2,2,1,4,5,2,3,1,14,3,12,
        3,17,2,16,5,1,2,1,8,9,3,19,4,2,2,4,17,25,21,20,28,75,1,10,29,103,4,1,2,1,1,4,2,4,1,2,3,24,2,2,2,1,1,2,1,3,8,1,1,1,2,1,1,3,1,1,1,6,1,5,3,1,1,
        1,3,4,1,1,5,2,1,5,6,13,9,16,1,1,1,1,3,2,3,2,4,5,2,5,2,2,3,7,13,7,2,2,1,1,1,1,2,3,3,2,1,6,4,9,2,1,14,2,14,2,1,18,3,4,14,4,11,41,15,23,15,23,
        176,1,3,4,1,1,1,1,5,3,1,2,3,7,3,1,1,2,1,2,4,4,6,2,4,1,9,7,1,10,5,8,16,29,1,1,2,2,3,1,3,5,2,4,5,4,1,1,2,2,3,3,7,1,6,10,1,17,1,44,4,6,2,1,1,6,
        5,4,2,10,1,6,9,2,8,1,24,1,2,13,7,8,8,2,1,4,1,3,1,3,3,5,2,5,10,9,4,9,12,2,1,6,1,10,1,1,7,7,4,10,8,3,1,13,4,3,1,6,1,3,5,2,1,2,17,16,5,2,16,6,
        1,4,2,1,3,3,6,8,5,11,11,1,3,3,2,4,6,10,9,5,7,4,7,4,7,1,1,4,2,1,3,6,8,7,1,6,11,5,5,3,24,9,4,2,7,13,5,1,8,82,16,61,1,1,1,4,2,2,16,10,3,8,1,1,
        6,4,2,1,3,1,1,1,4,3,8,4,2,2,1,1,1,1,1,6,3,5,1,1,4,6,9,2,1,1,1,2,1,7,2,1,6,1,5,4,4,3,1,8,1,3,3,1,3,2,2,2,2,3,1,6,1,2,1,2,1,3,7,1,8,2,1,2,1,5,
        2,5,3,5,10,1,2,1,1,3,2,5,11,3,9,3,5,1,1,5,9,1,2,1,5,7,9,9,8,1,3,3,3,6,8,2,3,2,1,1,32,6,1,2,15,9,3,7,13,1,3,10,13,2,14,1,13,10,2,1,3,10,4,15,
        2,15,15,10,1,3,9,6,9,32,25,26,47,7,3,2,3,1,6,3,4,3,2,8,5,4,1,9,4,2,2,19,10,6,2,3,8,1,2,2,4,2,1,9,4,4,4,6,4,8,9,2,3,1,1,1,1,3,5,5,1,3,8,4,6,
        2,1,4,12,1,5,3,7,13,2,5,8,1,6,1,2,5,14,6,1,5,2,4,8,15,5,1,23,6,62,2,10,1,1,8,1,2,2,10,4,2,2,9,2,1,1,3,2,3,1,5,3,3,2,1,3,8,1,1,1,11,3,1,1,4,
        3,7,1,14,1,2,3,12,5,2,5,1,6,7,5,7,14,11,1,3,1,8,9,12,2,1,11,8,4,4,2,6,10,9,13,1,1,3,1,5,1,3,2,4,4,1,18,2,3,14,11,4,29,4,2,7,1,3,13,9,2,2,5,
        3,5,20,7,16,8,5,72,34,6,4,22,12,12,28,45,36,9,7,39,9,191,1,1,1,4,11,8,4,9,2,3,22,1,1,1,1,4,17,1,7,7,1,11,31,10,2,4,8,2,3,2,1,4,2,16,4,32,2,
        3,19,13,4,9,1,5,2,14,8,1,1,3,6,19,6,5,1,16,6,2,10,8,5,1,2,3,1,5,5,1,11,6,6,1,3,3,2,6,3,8,1,1,4,10,7,5,7,7,5,8,9,2,1,3,4,1,1,3,1,3,3,2,6,16,
        1,4,6,3,1,10,6,1,3,15,2,9,2,10,25,13,9,16,6,2,2,10,11,4,3,9,1,2,6,6,5,4,30,40,1,10,7,12,14,33,6,3,6,7,3,1,3,1,11,14,4,9,5,12,11,49,18,51,31,
        140,31,2,2,1,5,1,8,1,10,1,4,4,3,24,1,10,1,3,6,6,16,3,4,5,2,1,4,2,57,10,6,22,2,22,3,7,22,6,10,11,36,18,16,33,36,2,5,5,1,1,1,4,10,1,4,13,2,7,
        5,2,9,3,4,1,7,43,3,7,3,9,14,7,9,1,11,1,1,3,7,4,18,13,1,14,1,3,6,10,73,2,2,30,6,1,11,18,19,13,22,3,46,42,37,89,7,3,16,34,2,2,3,9,1,7,1,1,1,2,
        2,4,10,7,3,10,3,9,5,28,9,2,6,13,7,3,1,3,10,2,7,2,11,3,6,21,54,85,2,1,4,2,2,1,39,3,21,2,2,5,1,1,1,4,1,1,3,4,15,1,3,2,4,4,2,3,8,2,20,1,8,7,13,
        4,1,26,6,2,9,34,4,21,52,10,4,4,1,5,12,2,11,1,7,2,30,12,44,2,30,1,1,3,6,16,9,17,39,82,2,2,24,7,1,7,3,16,9,14,44,2,1,2,1,2,3,5,2,4,1,6,7,5,3,
        2,6,1,11,5,11,2,1,18,19,8,1,3,24,29,2,1,3,5,2,2,1,13,6,5,1,46,11,3,5,1,1,5,8,2,10,6,12,6,3,7,11,2,4,16,13,2,5,1,1,2,2,5,2,28,5,2,23,10,8,4,
        4,22,39,95,38,8,14,9,5,1,13,5,4,3,13,12,11,1,9,1,27,37,2,5,4,4,63,211,95,2,2,2,1,3,5,2,1,1,2,2,1,1,1,3,2,4,1,2,1,1,5,2,2,1,1,2,3,1,3,1,1,1,
        3,1,4,2,1,3,6,1,1,3,7,15,5,3,2,5,3,9,11,4,2,22,1,6,3,8,7,1,4,28,4,16,3,3,25,4,4,27,27,1,4,1,2,2,7,1,3,5,2,28,8,2,14,1,8,6,16,25,3,3,3,14,3,
        3,1,1,2,1,4,6,3,8,4,1,1,1,2,3,6,10,6,2,3,18,3,2,5,5,4,3,1,5,2,5,4,23,7,6,12,6,4,17,11,9,5,1,1,10,5,12,1,1,11,26,33,7,3,6,1,17,7,1,5,12,1,11,
        2,4,1,8,14,17,23,1,2,1,7,8,16,11,9,6,5,2,6,4,16,2,8,14,1,11,8,9,1,1,1,9,25,4,11,19,7,2,15,2,12,8,52,7,5,19,2,16,4,36,8,1,16,8,24,26,4,6,2,9,
        5,4,36,3,28,12,25,15,37,27,17,12,59,38,5,32,127,1,2,9,17,14,4,1,2,1,1,8,11,50,4,14,2,19,16,4,17,5,4,5,26,12,45,2,23,45,104,30,12,8,3,10,2,2,
        3,3,1,4,20,7,2,9,6,15,2,20,1,3,16,4,11,15,6,134,2,5,59,1,2,2,2,1,9,17,3,26,137,10,211,59,1,2,4,1,4,1,1,1,2,6,2,3,1,1,2,3,2,3,1,3,4,4,2,3,3,
        1,4,3,1,7,2,2,3,1,2,1,3,3,3,2,2,3,2,1,3,14,6,1,3,2,9,6,15,27,9,34,145,1,1,2,1,1,1,1,2,1,1,1,1,2,2,2,3,1,2,1,1,1,2,3,5,8,3,5,2,4,1,3,2,2,2,12,
        4,1,1,1,10,4,5,1,20,4,16,1,15,9,5,12,2,9,2,5,4,2,26,19,7,1,26,4,30,12,15,42,1,6,8,172,1,1,4,2,1,1,11,2,2,4,2,1,2,1,10,8,1,2,1,4,5,1,2,5,1,8,
        4,1,3,4,2,1,6,2,1,3,4,1,2,1,1,1,1,12,5,7,2,4,3,1,1,1,3,3,6,1,2,2,3,3,3,2,1,2,12,14,11,6,6,4,12,2,8,1,7,10,1,35,7,4,13,15,4,3,23,21,28,52,5,
        26,5,6,1,7,10,2,7,53,3,2,1,1,1,2,163,532,1,10,11,1,3,3,4,8,2,8,6,2,2,23,22,4,2,2,4,2,1,3,1,3,3,5,9,8,2,1,2,8,1,10,2,12,21,20,15,105,2,3,1,1,
        3,2,3,1,1,2,5,1,4,15,11,19,1,1,1,1,5,4,5,1,1,2,5,3,5,12,1,2,5,1,11,1,1,15,9,1,4,5,3,26,8,2,1,3,1,1,15,19,2,12,1,2,5,2,7,2,19,2,20,6,26,7,5,
        2,2,7,34,21,13,70,2,128,1,1,2,1,1,2,1,1,3,2,2,2,15,1,4,1,3,4,42,10,6,1,49,85,8,1,2,1,1,4,4,2,3,6,1,5,7,4,3,211,4,1,2,1,2,5,1,2,4,2,2,6,5,6,
        10,3,4,48,100,6,2,16,296,5,27,387,2,2,3,7,16,8,5,38,15,39,21,9,10,3,7,59,13,27,21,47,5,21,6
    };
    static ImWchar base_ranges[] = // not zero-terminated
    {
        0x0020, 0x00FF, // Basic Latin + Latin Supplement
        0x2000, 0x206F, // General Punctuation
        0x3000, 0x30FF, // CJK Symbols and Punctuations, Hiragana, Katakana
        0x31F0, 0x31FF, // Katakana Phonetic Extensions
        0xFF00, 0xFFEF, // Half-width characters
        0xFFFD, 0xFFFD  // Invalid
    };
    static ImWchar full_ranges[IM_ARRAYSIZE(base_ranges) + IM_ARRAYSIZE(accumulative_offsets_from_0x4E00) * 2 + 1] = { 0 };
    if (!full_ranges[0])
    {
        memcpy(full_ranges, base_ranges, sizeof(base_ranges));
        UnpackAccumulativeOffsetsIntoRanges(0x4E00, accumulative_offsets_from_0x4E00, IM_ARRAYSIZE(accumulative_offsets_from_0x4E00), full_ranges + IM_ARRAYSIZE(base_ranges));
    }
    return &full_ranges[0];
}

*const ImWchar  ImFontAtlas::GetGlyphRangesJapanese()
{
    // 2999 ideograms code points for Japanese
    // - 2136 Joyo (meaning "for regular use" or "for common use") Kanji code points
    // - 863 Jinmeiyo (meaning "for personal name") Kanji code points
    // - Sourced from the character information database of the Information-technology Promotion Agency, Japan
    //   - https://mojikiban.ipa.go.jp/mji/
    //   - Available under the terms of the Creative Commons Attribution-ShareAlike 2.1 Japan (CC BY-SA 2.1 JP).
    //     - https://creativecommons.org/licenses/by-sa/2.1/jp/deed.en
    //     - https://creativecommons.org/licenses/by-sa/2.1/jp/legalcode
    //   - You can generate this code by the script at:
    //     - https://github.com/vaiorabbit/everyday_use_kanji
    // - References:
    //   - List of Joyo Kanji
    //     - (Official list by the Agency for Cultural Affairs) https://www.bunka.go.jp/kokugo_nihongo/sisaku/joho/joho/kakuki/14/tosin02/index.html
    //     - (Wikipedia) https://en.wikipedia.org/wiki/List_of_j%C5%8Dy%C5%8D_kanji
    //   - List of Jinmeiyo Kanji
    //     - (Official list by the Ministry of Justice) http://www.moj.go.jp/MINJI/minji86.html
    //     - (Wikipedia) https://en.wikipedia.org/wiki/Jinmeiy%C5%8D_kanji
    // - Missing 1 Joyo Kanji: U+20B9F (Kun'yomi: Shikaru, On'yomi: Shitsu,shichi), see https://github.com/ocornut/imgui/pull/3627 for details.
    // You can use ImFontGlyphRangesBuilder to create your own ranges derived from this, by merging existing ranges or adding new characters.
    // (Stored as accumulative offsets from the initial unicode codepoint 0x4E00. This encoding is designed to helps us compact the source code size.)
    static const c_short accumulative_offsets_from_0x4E00[] =
    {
        0,1,2,4,1,1,1,1,2,1,3,3,2,2,1,5,3,5,7,5,6,1,2,1,7,2,6,3,1,8,1,1,4,1,1,18,2,11,2,6,2,1,2,1,5,1,2,1,3,1,2,1,2,3,3,1,1,2,3,1,1,1,12,7,9,1,4,5,1,
        1,2,1,10,1,1,9,2,2,4,5,6,9,3,1,1,1,1,9,3,18,5,2,2,2,2,1,6,3,7,1,1,1,1,2,2,4,2,1,23,2,10,4,3,5,2,4,10,2,4,13,1,6,1,9,3,1,1,6,6,7,6,3,1,2,11,3,
        2,2,3,2,15,2,2,5,4,3,6,4,1,2,5,2,12,16,6,13,9,13,2,1,1,7,16,4,7,1,19,1,5,1,2,2,7,7,8,2,6,5,4,9,18,7,4,5,9,13,11,8,15,2,1,1,1,2,1,2,2,1,2,2,8,
        2,9,3,3,1,1,4,4,1,1,1,4,9,1,4,3,5,5,2,7,5,3,4,8,2,1,13,2,3,3,1,14,1,1,4,5,1,3,6,1,5,2,1,1,3,3,3,3,1,1,2,7,6,6,7,1,4,7,6,1,1,1,1,1,12,3,3,9,5,
        2,6,1,5,6,1,2,3,18,2,4,14,4,1,3,6,1,1,6,3,5,5,3,2,2,2,2,12,3,1,4,2,3,2,3,11,1,7,4,1,2,1,3,17,1,9,1,24,1,1,4,2,2,4,1,2,7,1,1,1,3,1,2,2,4,15,1,
        1,2,1,1,2,1,5,2,5,20,2,5,9,1,10,8,7,6,1,1,1,1,1,1,6,2,1,2,8,1,1,1,1,5,1,1,3,1,1,1,1,3,1,1,12,4,1,3,1,1,1,1,1,10,3,1,7,5,13,1,2,3,4,6,1,1,30,
        2,9,9,1,15,38,11,3,1,8,24,7,1,9,8,10,2,1,9,31,2,13,6,2,9,4,49,5,2,15,2,1,10,2,1,1,1,2,2,6,15,30,35,3,14,18,8,1,16,10,28,12,19,45,38,1,3,2,3,
        13,2,1,7,3,6,5,3,4,3,1,5,7,8,1,5,3,18,5,3,6,1,21,4,24,9,24,40,3,14,3,21,3,2,1,2,4,2,3,1,15,15,6,5,1,1,3,1,5,6,1,9,7,3,3,2,1,4,3,8,21,5,16,4,
        5,2,10,11,11,3,6,3,2,9,3,6,13,1,2,1,1,1,1,11,12,6,6,1,4,2,6,5,2,1,1,3,3,6,13,3,1,1,5,1,2,3,3,14,2,1,2,2,2,5,1,9,5,1,1,6,12,3,12,3,4,13,2,14,
        2,8,1,17,5,1,16,4,2,2,21,8,9,6,23,20,12,25,19,9,38,8,3,21,40,25,33,13,4,3,1,4,1,2,4,1,2,5,26,2,1,1,2,1,3,6,2,1,1,1,1,1,1,2,3,1,1,1,9,2,3,1,1,
        1,3,6,3,2,1,1,6,6,1,8,2,2,2,1,4,1,2,3,2,7,3,2,4,1,2,1,2,2,1,1,1,1,1,3,1,2,5,4,10,9,4,9,1,1,1,1,1,1,5,3,2,1,6,4,9,6,1,10,2,31,17,8,3,7,5,40,1,
        7,7,1,6,5,2,10,7,8,4,15,39,25,6,28,47,18,10,7,1,3,1,1,2,1,1,1,3,3,3,1,1,1,3,4,2,1,4,1,3,6,10,7,8,6,2,2,1,3,3,2,5,8,7,9,12,2,15,1,1,4,1,2,1,1,
        1,3,2,1,3,3,5,6,2,3,2,10,1,4,2,8,1,1,1,11,6,1,21,4,16,3,1,3,1,4,2,3,6,5,1,3,1,1,3,3,4,6,1,1,10,4,2,7,10,4,7,4,2,9,4,3,1,1,1,4,1,8,3,4,1,3,1,
        6,1,4,2,1,4,7,2,1,8,1,4,5,1,1,2,2,4,6,2,7,1,10,1,1,3,4,11,10,8,21,4,6,1,3,5,2,1,2,28,5,5,2,3,13,1,2,3,1,4,2,1,5,20,3,8,11,1,3,3,3,1,8,10,9,2,
        10,9,2,3,1,1,2,4,1,8,3,6,1,7,8,6,11,1,4,29,8,4,3,1,2,7,13,1,4,1,6,2,6,12,12,2,20,3,2,3,6,4,8,9,2,7,34,5,1,18,6,1,1,4,4,5,7,9,1,2,2,4,3,4,1,7,
        2,2,2,6,2,3,25,5,3,6,1,4,6,7,4,2,1,4,2,13,6,4,4,3,1,5,3,4,4,3,2,1,1,4,1,2,1,1,3,1,11,1,6,3,1,7,3,6,2,8,8,6,9,3,4,11,3,2,10,12,2,5,11,1,6,4,5,
        3,1,8,5,4,6,6,3,5,1,1,3,2,1,2,2,6,17,12,1,10,1,6,12,1,6,6,19,9,6,16,1,13,4,4,15,7,17,6,11,9,15,12,6,7,2,1,2,2,15,9,3,21,4,6,49,18,7,3,2,3,1,
        6,8,2,2,6,2,9,1,3,6,4,4,1,2,16,2,5,2,1,6,2,3,5,3,1,2,5,1,2,1,9,3,1,8,6,4,8,11,3,1,1,1,1,3,1,13,8,4,1,3,2,2,1,4,1,11,1,5,2,1,5,2,5,8,6,1,1,7,
        4,3,8,3,2,7,2,1,5,1,5,2,4,7,6,2,8,5,1,11,4,5,3,6,18,1,2,13,3,3,1,21,1,1,4,1,4,1,1,1,8,1,2,2,7,1,2,4,2,2,9,2,1,1,1,4,3,6,3,12,5,1,1,1,5,6,3,2,
        4,8,2,2,4,2,7,1,8,9,5,2,3,2,1,3,2,13,7,14,6,5,1,1,2,1,4,2,23,2,1,1,6,3,1,4,1,15,3,1,7,3,9,14,1,3,1,4,1,1,5,8,1,3,8,3,8,15,11,4,14,4,4,2,5,5,
        1,7,1,6,14,7,7,8,5,15,4,8,6,5,6,2,1,13,1,20,15,11,9,2,5,6,2,11,2,6,2,5,1,5,8,4,13,19,25,4,1,1,11,1,34,2,5,9,14,6,2,2,6,1,1,14,1,3,14,13,1,6,
        12,21,14,14,6,32,17,8,32,9,28,1,2,4,11,8,3,1,14,2,5,15,1,1,1,1,3,6,4,1,3,4,11,3,1,1,11,30,1,5,1,4,1,5,8,1,1,3,2,4,3,17,35,2,6,12,17,3,1,6,2,
        1,1,12,2,7,3,3,2,1,16,2,8,3,6,5,4,7,3,3,8,1,9,8,5,1,2,1,3,2,8,1,2,9,12,1,1,2,3,8,3,24,12,4,3,7,5,8,3,3,3,3,3,3,1,23,10,3,1,2,2,6,3,1,16,1,16,
        22,3,10,4,11,6,9,7,7,3,6,2,2,2,4,10,2,1,1,2,8,7,1,6,4,1,3,3,3,5,10,12,12,2,3,12,8,15,1,1,16,6,6,1,5,9,11,4,11,4,2,6,12,1,17,5,13,1,4,9,5,1,11,
        2,1,8,1,5,7,28,8,3,5,10,2,17,3,38,22,1,2,18,12,10,4,38,18,1,4,44,19,4,1,8,4,1,12,1,4,31,12,1,14,7,75,7,5,10,6,6,13,3,2,11,11,3,2,5,28,15,6,18,
        18,5,6,4,3,16,1,7,18,7,36,3,5,3,1,7,1,9,1,10,7,2,4,2,6,2,9,7,4,3,32,12,3,7,10,2,23,16,3,1,12,3,31,4,11,1,3,8,9,5,1,30,15,6,12,3,2,2,11,19,9,
        14,2,6,2,3,19,13,17,5,3,3,25,3,14,1,1,1,36,1,3,2,19,3,13,36,9,13,31,6,4,16,34,2,5,4,2,3,3,5,1,1,1,4,3,1,17,3,2,3,5,3,1,3,2,3,5,6,3,12,11,1,3,
        1,2,26,7,12,7,2,14,3,3,7,7,11,25,25,28,16,4,36,1,2,1,6,2,1,9,3,27,17,4,3,4,13,4,1,3,2,2,1,10,4,2,4,6,3,8,2,1,18,1,1,24,2,2,4,33,2,3,63,7,1,6,
        40,7,3,4,4,2,4,15,18,1,16,1,1,11,2,41,14,1,3,18,13,3,2,4,16,2,17,7,15,24,7,18,13,44,2,2,3,6,1,1,7,5,1,7,1,4,3,3,5,10,8,2,3,1,8,1,1,27,4,2,1,
        12,1,2,1,10,6,1,6,7,5,2,3,7,11,5,11,3,6,6,2,3,15,4,9,1,1,2,1,2,11,2,8,12,8,5,4,2,3,1,5,2,2,1,14,1,12,11,4,1,11,17,17,4,3,2,5,5,7,3,1,5,9,9,8,
        2,5,6,6,13,13,2,1,2,6,1,2,2,49,4,9,1,2,10,16,7,8,4,3,2,23,4,58,3,29,1,14,19,19,11,11,2,7,5,1,3,4,6,2,18,5,12,12,17,17,3,3,2,4,1,6,2,3,4,3,1,
        1,1,1,5,1,1,9,1,3,1,3,6,1,8,1,1,2,6,4,14,3,1,4,11,4,1,3,32,1,2,4,13,4,1,2,4,2,1,3,1,11,1,4,2,1,4,4,6,3,5,1,6,5,7,6,3,23,3,5,3,5,3,3,13,3,9,10,
        1,12,10,2,3,18,13,7,160,52,4,2,2,3,2,14,5,4,12,4,6,4,1,20,4,11,6,2,12,27,1,4,1,2,2,7,4,5,2,28,3,7,25,8,3,19,3,6,10,2,2,1,10,2,5,4,1,3,4,1,5,
        3,2,6,9,3,6,2,16,3,3,16,4,5,5,3,2,1,2,16,15,8,2,6,21,2,4,1,22,5,8,1,1,21,11,2,1,11,11,19,13,12,4,2,3,2,3,6,1,8,11,1,4,2,9,5,2,1,11,2,9,1,1,2,
        14,31,9,3,4,21,14,4,8,1,7,2,2,2,5,1,4,20,3,3,4,10,1,11,9,8,2,1,4,5,14,12,14,2,17,9,6,31,4,14,1,20,13,26,5,2,7,3,6,13,2,4,2,19,6,2,2,18,9,3,5,
        12,12,14,4,6,2,3,6,9,5,22,4,5,25,6,4,8,5,2,6,27,2,35,2,16,3,7,8,8,6,6,5,9,17,2,20,6,19,2,13,3,1,1,1,4,17,12,2,14,7,1,4,18,12,38,33,2,10,1,1,
        2,13,14,17,11,50,6,33,20,26,74,16,23,45,50,13,38,33,6,6,7,4,4,2,1,3,2,5,8,7,8,9,3,11,21,9,13,1,3,10,6,7,1,2,2,18,5,5,1,9,9,2,68,9,19,13,2,5,
        1,4,4,7,4,13,3,9,10,21,17,3,26,2,1,5,2,4,5,4,1,7,4,7,3,4,2,1,6,1,1,20,4,1,9,2,2,1,3,3,2,3,2,1,1,1,20,2,3,1,6,2,3,6,2,4,8,1,3,2,10,3,5,3,4,4,
        3,4,16,1,6,1,10,2,4,2,1,1,2,10,11,2,2,3,1,24,31,4,10,10,2,5,12,16,164,15,4,16,7,9,15,19,17,1,2,1,1,5,1,1,1,1,1,3,1,4,3,1,3,1,3,1,2,1,1,3,3,7,
        2,8,1,2,2,2,1,3,4,3,7,8,12,92,2,10,3,1,3,14,5,25,16,42,4,7,7,4,2,21,5,27,26,27,21,25,30,31,2,1,5,13,3,22,5,6,6,11,9,12,1,5,9,7,5,5,22,60,3,5,
        13,1,1,8,1,1,3,3,2,1,9,3,3,18,4,1,2,3,7,6,3,1,2,3,9,1,3,1,3,2,1,3,1,1,1,2,1,11,3,1,6,9,1,3,2,3,1,2,1,5,1,1,4,3,4,1,2,2,4,4,1,7,2,1,2,2,3,5,13,
        18,3,4,14,9,9,4,16,3,7,5,8,2,6,48,28,3,1,1,4,2,14,8,2,9,2,1,15,2,4,3,2,10,16,12,8,7,1,1,3,1,1,1,2,7,4,1,6,4,38,39,16,23,7,15,15,3,2,12,7,21,
        37,27,6,5,4,8,2,10,8,8,6,5,1,2,1,3,24,1,16,17,9,23,10,17,6,1,51,55,44,13,294,9,3,6,2,4,2,2,15,1,1,1,13,21,17,68,14,8,9,4,1,4,9,3,11,7,1,1,1,
        5,6,3,2,1,1,1,2,3,8,1,2,2,4,1,5,5,2,1,4,3,7,13,4,1,4,1,3,1,1,1,5,5,10,1,6,1,5,2,1,5,2,4,1,4,5,7,3,18,2,9,11,32,4,3,3,2,4,7,11,16,9,11,8,13,38,
        32,8,4,2,1,1,2,1,2,4,4,1,1,1,4,1,21,3,11,1,16,1,1,6,1,3,2,4,9,8,57,7,44,1,3,3,13,3,10,1,1,7,5,2,7,21,47,63,3,15,4,7,1,16,1,1,2,8,2,3,42,15,4,
        1,29,7,22,10,3,78,16,12,20,18,4,67,11,5,1,3,15,6,21,31,32,27,18,13,71,35,5,142,4,10,1,2,50,19,33,16,35,37,16,19,27,7,1,133,19,1,4,8,7,20,1,4,
        4,1,10,3,1,6,1,2,51,5,40,15,24,43,22928,11,1,13,154,70,3,1,1,7,4,10,1,2,1,1,2,1,2,1,2,2,1,1,2,1,1,1,1,1,2,1,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,1,
        3,2,1,1,1,1,2,1,1,
    };
    static ImWchar base_ranges[] = // not zero-terminated
    {
        0x0020, 0x00FF, // Basic Latin + Latin Supplement
        0x3000, 0x30FF, // CJK Symbols and Punctuations, Hiragana, Katakana
        0x31F0, 0x31FF, // Katakana Phonetic Extensions
        0xFF00, 0xFFEF, // Half-width characters
        0xFFFD, 0xFFFD  // Invalid
    };
    static ImWchar full_ranges[IM_ARRAYSIZE(base_ranges) + IM_ARRAYSIZE(accumulative_offsets_from_0x4E00)*2 + 1] = { 0 };
    if (!full_ranges[0])
    {
        memcpy(full_ranges, base_ranges, sizeof(base_ranges));
        UnpackAccumulativeOffsetsIntoRanges(0x4E00, accumulative_offsets_from_0x4E00, IM_ARRAYSIZE(accumulative_offsets_from_0x4E00), full_ranges + IM_ARRAYSIZE(base_ranges));
    }
    return &full_ranges[0];
}

*const ImWchar  ImFontAtlas::GetGlyphRangesCyrillic()
{
    static const ImWchar ranges[] =
    {
        0x0020, 0x00FF, // Basic Latin + Latin Supplement
        0x0400, 0x052F, // Cyrillic + Cyrillic Supplement
        0x2DE0, 0x2DFF, // Cyrillic Extended-A
        0xA640, 0xA69F, // Cyrillic Extended-B
        0,
    };
    return &ranges[0];
}

*const ImWchar  ImFontAtlas::GetGlyphRangesThai()
{
    static const ImWchar ranges[] =
    {
        0x0020, 0x00FF, // Basic Latin
        0x2010, 0x205E, // Punctuations
        0x0E00, 0x0E7F, // Thai
        0,
    };
    return &ranges[0];
}

*const ImWchar  ImFontAtlas::GetGlyphRangesVietnamese()
{
    static const ImWchar ranges[] =
    {
        0x0020, 0x00FF, // Basic Latin
        0x0102, 0x0103,
        0x0110, 0x0111,
        0x0128, 0x0129,
        0x0168, 0x0169,
        0x01A0, 0x01A1,
        0x01AF, 0x01B0,
        0x1EA0, 0x1EF9,
        0,
    };
    return &ranges[0];
}

//-----------------------------------------------------------------------------
// [SECTION] ImFontGlyphRangesBuilder
//-----------------------------------------------------------------------------

c_void ImFontGlyphRangesBuilder::AddText(*const char text, *const char text_end)
{
    while (text_end ? (text < text_end) : *text)
    {
        c_uint c = 0;
        c_int c_len = ImTextCharFromUtf8(&c, text, text_end);
        text += c_len;
        if c_len == 0
{
            break;
}
        AddChar(c);
    }
}

c_void ImFontGlyphRangesBuilder::AddRanges(*const ImWchar ranges)
{
    for (; ranges[0]; ranges += 2)
        for (c_uint c = ranges[0]; c <= ranges[1] && c <= IM_UNICODE_CODEPOINT_MAX; c++) //-V560
            AddChar(c);
}

c_void ImFontGlyphRangesBuilder::BuildRanges(Vec<ImWchar>* out_ranges)
{
    let max_codepoint: c_int = IM_UNICODE_CODEPOINT_MAX;
    for (c_int n = 0; n <= max_codepoint; n++)
        if (GetBit(n))
        {
            out_ranges.push(n);
            while (n < max_codepoint && GetBit(n + 1))
                n+= 1;
            out_ranges.push(n);
        }
    out_ranges.push(0);
}

//-----------------------------------------------------------------------------
// [SECTION] ImFont
//-----------------------------------------------------------------------------

ImFont::ImFont()
{
    FontSize = 0f32;
    FallbackAdvanceX = 0f32;
    FallbackChar = -1;
    EllipsisChar = -1;
    DotChar = -1;
    FallbackGlyph = None;
    ContainerAtlas = None;
    ConfigData = None;
    ConfigDataCount = 0;
    DirtyLookupTables = false;
    Scale = 1f32;
    Ascent = Descent = 0f32;
    MetricsTotalSurface = 0;
    memset(Used4kPagesMap, 0, sizeof(Used4kPagesMap));
}

c_void    ImFont::ClearOutputData()
{
    FontSize = 0f32;
    FallbackAdvanceX = 0f32;
    Glyphs.clear();
    IndexAdvanceX.clear();
    IndexLookup.clear();
    FallbackGlyph = None;
    ContainerAtlas = None;
    DirtyLookupTables = true;
    Ascent = Descent = 0f32;
    MetricsTotalSurface = 0;
}

static ImWchar FindFirstExistingGlyph(ImFont* font, *const ImWchar candidate_chars, c_int candidate_chars_count)
{
    for (let n: c_int = 0; n < candidate_chars_count; n++)
        if (candidate_chars[n]) != null_mut()
{
            return candidate_chars[n];
}
    return -1;
}

c_void ImFont::BuildLookupTable()
{
    c_int max_codepoint = 0;
    for (c_int i = 0; i != Glyphs.Size; i++)
        max_codepoint = ImMax(max_codepoint, Glyphs[i].Codepoint);

    // Build lookup table
    // IM_ASSERT(Glyphs.Size < 0xFFF0f32); // -1 is reserved
    IndexAdvanceX.clear();
    IndexLookup.clear();
    DirtyLookupTables = false;
    memset(Used4kPagesMap, 0, sizeof(Used4kPagesMap));
    GrowIndex(max_codepoint + 1);
    for (c_int i = 0; i < Glyphs.Size; i++)
    {
        c_int codepoint = Glyphs[i].Codepoint;
        IndexAdvanceX[codepoint] = Glyphs[i].AdvanceX;
        IndexLookup[codepoint] = i;

        // Mark 4K page as used
        let page_n: c_int = codepoint / 4096;
        Used4kPagesMap[page_n >> 3] |= 1 << (page_n & 7);
    }

    // Create a glyph to handle TAB
    // FIXME: Needs proper TAB handling but it needs to be contextualized (or we could arbitrary say that each string starts at "column 0" ?)
    if (FindGlyph(' '))
    {
        if Glyphs.last().unwrap().Codepoint != '\t')   // So we can call this function multiple times (FIXME: Flaky
{
            Glyphs.resize(Glyphs.Size + 1);
}
        ImFontGlyph& tab_glyph = Glyphs.last().unwrap();
        tab_glyph = *FindGlyph(' ');
        tab_glyph.Codepoint = '\t';
        tab_glyph.AdvanceX *= IM_TABSIZE;
        IndexAdvanceX[tab_glyph.Codepoint] = tab_glyph.AdvanceX;
        IndexLookup[tab_glyph.Codepoint] = (Glyphs.Size - 1);
    }

    // Mark special glyphs as not visible (note that AddGlyph already mark as non-visible glyphs with zero-size polygons)
    SetGlyphVisible(' ', false);
    SetGlyphVisible('\t', false);

    // Ellipsis character is required for rendering elided text. We prefer using U+2026 (horizontal ellipsis).
    // However some old fonts may contain ellipsis at U+0085. Here we auto-detect most suitable ellipsis character.
    // FIXME: Note that 0x2026 is rarely included in our font ranges. Because of this we are more likely to use three individual dots.
    const ImWchar ellipsis_chars[] = { 0x2026, 0x0085 };
    const ImWchar dots_chars[] = { '.', 0xFF0E };
    if EllipsisChar == -1
{
        EllipsisChar = FindFirstExistingGlyph(this, ellipsis_chars, IM_ARRAYSIZE(ellipsis_chars));
}
    if DotChar == -1
{
        DotChar = FindFirstExistingGlyph(this, dots_chars, IM_ARRAYSIZE(dots_chars));
}

    // Setup fallback character
    const ImWchar fallback_chars[] = { IM_UNICODE_CODEPOINT_INVALID, '?', ' ' };
    FallbackGlyph = FindGlyphNoFallback(FallbackChar);
    if (FallbackGlyph == NULL)
    {
        FallbackChar = FindFirstExistingGlyph(this, fallback_chars, IM_ARRAYSIZE(fallback_chars));
        FallbackGlyph = FindGlyphNoFallback(FallbackChar);
        if (FallbackGlyph == NULL)
        {
            FallbackGlyph = &Glyphs.back();
            FallbackChar = FallbackGlyph->Codepoint;
        }
    }

    FallbackAdvanceX = ;
    for (let i: c_int = 0; i < max_codepoint + 1; i++)
        if IndexAdvanceX[i] < 0f32
{
            IndexAdvanceX[i] = FallbackAdvanceX;
}
}

// API is designed this way to avoid exposing the 4K page size
// e.g. use with IsGlyphRangeUnused(0, 255)
bool ImFont::IsGlyphRangeUnused(c_uint c_begin, c_uint c_last)
{
    let mut page_begin: c_uint =  (c_begin / 4096);
    let mut page_last: c_uint =  (c_last / 4096);
    for (let mut page_n: c_uint =  page_begin; page_n <= page_last; page_n++)
        if (page_n >> 3) < sizeof(Used4kPagesMap)
{
            if (Used4kPagesMap[page_n >> 3] & (1 << (page_n & 7)))
                return false;
}
    return true;
}

c_void ImFont::SetGlyphVisible(ImWchar c, bool visible)
{
    if ImFontGlyph* glyph = (ImFontGlyph*)FindGlyph(c)
{
         = visible ? 1 : 0;
}
}

c_void ImFont::GrowIndex(c_int new_size)
{
    // IM_ASSERT(IndexAdvanceX.Size == IndexLookup.Size);
    if new_size <= IndexLookup.Size
{
        return;
}
    IndexAdvanceX.resize(new_size, -1f32);
    IndexLookup.resize(new_size, -1);
}

// x0/y0/x1/y1 are offset from the character upper-left layout position, in pixels. Therefore x0/y0 are often fairly close to zero.
// Not to be mistaken with texture coordinates, which are held by u0/v0/u1/v1 in normalized format (0.0..1.0 on each texture axis).
// 'cfg' is not necessarily == 'this->ConfigData' because multiple source fonts+configs can be used to build one target font.
c_void ImFont::AddGlyph(*const ImFontConfig cfg, ImWchar codepoint, c_float x0, c_float y0, c_float x1, c_float y1, c_float u0, c_float v0, c_float u1, c_float v1, c_float advance_x)
{
    if (cfg != NULL)
    {
        // Clamp & recenter if needed
        let         : c_float =  advance_x;
        advance_x = ImClamp(advance_x, cfg->GlyphMinAdvanceX, cfg->GlyphMaxAdvanceX);
        if (advance_x != advance_x_original)
        {
            c_float char_off_x = cfg->PixelSnapH ? ImFloor((advance_x - advance_x_original) * 0.5f32) : (advance_x - advance_x_original) * 0.5f32;
            x0 += char_off_x;
            x1 += char_off_x;
        }

        // Snap to pixel
        if
{
            advance_x = IM_ROUND(advance_x);
}

        // Bake spacing
        advance_x += cfg->GlyphExtraSpacing.x;
    }

    Glyphs.resize(Glyphs.Size + 1);
    ImFontGlyph& glyph = Glyphs.back();
    glyph.Codepoint = codepoint;
    glyph.Visible = (x0 != x1) && (y0 != y1);
    glyph.Colored = false;
    glyph.X0 = x0;
    glyph.Y0 = y0;
    glyph.X1 = x1;
    glyph.Y1 = y1;
    glyph.U0 = u0;
    glyph.V0 = v0;
    glyph.U1 = u1;
    glyph.V1 = v1;
    glyph.AdvanceX = advance_x;

    // Compute rough surface usage metrics (+1 to account for average padding, +0.99 to round)
    // We use (U1-U0)*TexWidth instead of X1-X0 to account for oversampling.
    c_float pad = ContainerAtlas->TexGlyphPadding + 0.99f;
    DirtyLookupTables = true;
    MetricsTotalSurface += ((glyph.U1 - glyph.U0) * ContainerAtlas->TexWidth + pad) * ((glyph.V1 - glyph.V0) * ContainerAtlas->TexHeight + pad);
}

c_void ImFont::AddRemapChar(ImWchar dst, ImWchar src, bool overwrite_dst)
{
    // IM_ASSERT(IndexLookup.Size > 0);    // Currently this can only be called AFTER the font has been built, aka after calling ImFontAtlas::GetTexDataAs*() function.
    c_uint index_size = IndexLookup.Size;

    if (dst < index_size && IndexLookup.Data[dst] == -1 && !overwrite_dst) // 'dst' already exists
        return;
    if (src >= index_size && dst >= index_size) // both 'dst' and 'src' don't exist -> no-op
        return;

    GrowIndex(dst + 1);
    IndexLookup[dst] = (src < index_size) ? IndexLookup.Data[src] : -1;
    IndexAdvanceX[dst] = (src < index_size) ? IndexAdvanceX.Data[src] : 1f32;
}

*const ImFontGlyph ImFont::FindGlyph(ImWchar c) const
{
    if c >= IndexLookup.Size
{
        return FallbackGlyph;
}
    const let i: ImWchar = IndexLookup.Data[c];
    if i == -1
{
        return FallbackGlyph;
}
    return &Glyphs.Data[i];
}

*const ImFontGlyph ImFont::FindGlyphNoFallback(ImWchar c) const
{
    if c >= IndexLookup.Size
{
        return null_mut();
}
    const let i: ImWchar = IndexLookup.Data[c];
    if i == -1
{
        return null_mut();
}
    return &Glyphs.Data[i];
}

*const char ImFont::CalcWordWrapPositionA(c_float scale, *const char text, *const char text_end, c_float wrap_width) const
{
    // Simple word-wrapping for English, not full-featured. Please submit failing cases!
    // FIXME: Much possible improvements (don't cut things like "word !", "word!!!" but cut within "word,,,,", more sensible support for punctuations, support for Unicode punctuations, etc.)

    // For references, possible wrap point marked with ^
    //  "aaa bbb, ccc,ddd. eee   fff. ggg!"
    //      ^    ^    ^   ^   ^__    ^    ^

    // List of hardcoded separators: .,;!?'"

    // Skip extra blanks after a line returns (that includes not counting them in width computation)
    // e.g. "Hello    world" --> "Hello" "World"

    // Cut words that cannot possibly fit within one line.
    // e.g.: "The tropical fish" with ~5 characters worth of width --> "The tr" "opical" "fish"

    c_float line_width = 0f32;
    c_float word_width = 0f32;
    c_float blank_width = 0f32;
    wrap_width /= scale; // We work with unscaled widths to avoid scaling every characters

    let mut  word_end: *const c_char = text;
    let mut  prev_word_end: *const c_char = None;
    let mut inside_word: bool =  true;

    let mut  s: *const c_char = text;
    while (s < text_end)
    {
        c_uint c = *s;
let next_s: *const c_char;
        if c < 0x80
{
            next_s = s + 1;
}

        if c == 0
{
            break;
}

        if (c < 32)
        {
            if (c == '\n')
            {
                line_width = word_width = blank_width = 0f32;
                inside_word = true;
                s = next_s;
                continue;
            }
            if (c == '\r')
            {
                s = next_s;
                continue;
            }
        }

        let
        : c_float =  (c < IndexAdvanceX.Size ? IndexAdvanceX.Data[c] : FallbackAdvanceX);
        if (ImCharIsBlankW(c))
        {
            if (inside_word)
            {
                line_width += blank_width;
                blank_width = 0f32;
                word_end = s;
            }
            blank_width += char_width;
            inside_word = false;
        }
        else
        {
            word_width += char_width;
            if (inside_word)
            {
                word_end = next_s;
            }
            else
            {
                prev_word_end = word_end;
                line_width += word_width + blank_width;
                word_width = blank_width = 0f32;
            }

            // Allow wrapping after punctuation.
            inside_word = (c != '.' && c != ',' && c != ';' && c != '!' && c != '?' && c != '\"');
        }

        // We ignore blank width at the end of the line (they can be skipped)
        if (line_width + word_width > wrap_width)
        {
            // Words that cannot possibly fit within an entire line will be cut anywhere.
            if word_width < wrap_width
{
                s = prev_word_end ? prev_word_end : word_end;
}
            break;
        }

        s = next_s;
    }

    return s;
}

ImVec2 ImFont::CalcTextSizeA(c_float size, c_float max_width, c_float wrap_width, *const char text_begin, *const char text_end, *const char* remaining) const
{
    if !text_end
{
        text_end = text_begin + strlen(text_begin);
} // FIXME-OPT: Need to avoid this.

    let
    : c_float =  size;
    let     : c_float =  size / FontSize;

    let text_size: ImVec2 = ImVec2(0, 0);
    c_float line_width = 0f32;

    let word_wrap_enabled: bool = (wrap_width > 0f32);
    let mut  word_wrap_eol: *const c_char = None;

    let mut  s: *const c_char = text_begin;
    while (s < text_end)
    {
        if (word_wrap_enabled)
        {
            // Calculate how far we can render. Requires two passes on the string data but keeps the code simple and not intrusive for what's essentially an uncommon feature.
            if (!word_wrap_eol)
            {
                word_wrap_eol = CalcWordWrapPositionA(scale, s, text_end, wrap_width - line_width);
                if (word_wrap_eol == s) // Wrap_width is too small to fit anything. Force displaying 1 character to minimize the height discontinuity.
                    word_wrap_eol+= 1;    // +1 may not be a character start point in UTF-8 but it's ok because we use s >= word_wrap_eol below
            }

            if (s >= word_wrap_eol)
            {
                if text_size.x < line_width
{
                    text_size.x = line_width;
}
                text_size.y += line_height;
                line_width = 0f32;
                word_wrap_eol = None;

                // Wrapping skips upcoming blanks
                while (s < text_end)
                {
                    const char c = *s;
                    if (ImCharIsBlankA(c)) { s+= 1; } else if (c == '\n') { s+= 1; break; } else { break; }
                }
                continue;
            }
        }

        // Decode and advance source
        let mut  prev_s: *const c_char = s;
        c_uint c = *s;
        if (c < 0x80)
        {
            s += 1;
        }
        else
        {
            s += ImTextCharFromUtf8(&c, s, text_end);
            if (c == 0) // Malformed UTF-8?
                break;
        }

        if (c < 32)
        {
            if (c == '\n')
            {
                text_size.x = ImMax(text_size.x, line_width);
                text_size.y += line_height;
                line_width = 0f32;
                continue;
            }
            if c == '\r'
{
                continue;
}
        }

        let
        : c_float =  (c < IndexAdvanceX.Size ? IndexAdvanceX.Data[c] : FallbackAdvanceX) * scale;
        if (line_width + char_width >= max_width)
        {
            s = prev_s;
            break;
        }

        line_width += char_width;
    }

    if text_size.x < line_width
{
        text_size.x = line_width;
}

    if line_width > 0 || text_size.y == 0f32
{
        text_size.y += line_height;
}

    if remaining
{
        *remaining = s;
}

    return text_size;
}


//-----------------------------------------------------------------------------
// [SECTION] ImGui Internal Render Helpers
//-----------------------------------------------------------------------------
// Vaguely redesigned to stop accessing ImGui global state:
// - RenderArrow()
// - RenderBullet()
// - RenderCheckMark()
// - RenderArrowDockMenu()
// - RenderArrowPointingAt()
// - RenderRectFilledRangeH()
// - RenderRectFilledWithHole()
//-----------------------------------------------------------------------------
// Function in need of a redesign (legacy mess)
// - RenderColorRectWithAlphaCheckerboard()
//-----------------------------------------------------------------------------

// Render an arrow aimed to be aligned with text (p_min is a position in the same space text would be positioned). To e.g. denote expanded/collapsed state
c_void RenderArrow(draw_list: *mut ImDrawList, pos: ImVec2, col: u32, dir: ImGuiDir, scale: c_float)
{
    let h: c_float =  draw_list._Data.FontSize * 1f32;
    let r: c_float =  h * 0.40f32 * scale;
    let center: ImVec2 = pos + ImVec2::new2(h * 0.50f32, h * 0.50f32 * scale);

    ImVec2 a, b, c;
    switch (dir)
    {
    case ImGuiDir_Up:
    case ImGuiDir_Down:
        if (dir == ImGuiDir_Up) r = -r;
        a = ImVec2::new2(+0.000f32, +0.7500f32) * r;
        b = ImVec2::new2(-0.866f, -0.7500f32) * r;
        c = ImVec2::new2(+0.866f, -0.7500f32) * r;
        break;
    case ImGuiDir_Left:
    case ImGuiDir_Right:
        if (dir == ImGuiDir_Left) r = -r;
        a = ImVec2::new2(+0.750f32, +0.0000f32) * r;
        b = ImVec2::new2(-0.750f32, +0.8660f32) * r;
        c = ImVec2::new2(-0.750f32, -0.8660f32) * r;
        break;
    case ImGuiDir_None:
    case ImGuiDir_COUNT:
        // IM_ASSERT(0);
        break;
    }
    draw_list->AddTriangleFilled(center + a, center + b, center + c, col);
}

c_void RenderBullet(draw_list: *mut ImDrawList, pos: ImVec2, col: u32)
{
    draw_list->AddCircleFilled(pos, draw_list->_Data.FontSize * 0.20f32, col, 8);
}

c_void RenderCheckMark(draw_list: *mut ImDrawList, pos: ImVec2, col: u32, sz: c_float)
{
    c_float thickness = ImMax(sz / 5f32, 1f32);
    sz -= thickness * 0.5f32;
    pos += ImVec2::new2(thickness * 0.25f, thickness * 0.250f32);

    let third: c_float =  sz / 3.0f32;
    let bx: c_float =  pos.x + third;
    let by: c_float =  pos.y + sz - third * 0.5f32;
    draw_list.PathLineTo(ImVec2::new2(bx - third, by - third));
    draw_list.PathLineTo(ImVec2::new2(bx, by));
    draw_list.PathLineTo(ImVec2::new2(bx + third * 2.0f32, by - third * 2.00f32));
    draw_list.PathStroke(col, 0, thickness);
}

// Render an arrow. 'pos' is position of the arrow tip. half_sz.x is length from base to tip. half_sz.y is length on each side.
c_void RenderArrowPointingAt(draw_list: *mut ImDrawList, pos: ImVec2, half_sz: ImVec2, direction: ImGuiDir, col: u32)
{
    switch (direction)
    {
    case ImGuiDir_Left:  draw_list.AddTriangleFilled(ImVec2::new2(pos.x + half_sz.x, pos.y - half_sz.y), ImVec2::new(pos.x + half_sz.x, pos.y + half_sz.y), pos, col); return;
    case ImGuiDir_Right: draw_list.AddTriangleFilled(ImVec2::new2(pos.x - half_sz.x, pos.y + half_sz.y), ImVec2::new(pos.x - half_sz.x, pos.y - half_sz.y), pos, col); return;
    case ImGuiDir_Up:    draw_list.AddTriangleFilled(ImVec2::new2(pos.x + half_sz.x, pos.y + half_sz.y), ImVec2::new(pos.x - half_sz.x, pos.y + half_sz.y), pos, col); return;
    case ImGuiDir_Down:  draw_list.AddTriangleFilled(ImVec2::new2(pos.x - half_sz.x, pos.y - half_sz.y), ImVec2::new(pos.x + half_sz.x, pos.y - half_sz.y), pos, col); return;
    case ImGuiDir_None: case ImGuiDir_COUNT: break; // Fix warnings
    }
}

// This is less wide than RenderArrow() and we use in dock nodes instead of the regular RenderArrow() to denote a change of functionality,
// and because the saved space means that the left-most tab label can stay at exactly the same position as the label of a loose window.
c_void RenderArrowDockMenu(draw_list: *mut ImDrawList, p_min: ImVec2, sz: c_float, col: u32)
{
    draw_list.AddRectFilled(p_min + ImVec2::new2(sz * 0.20f32, sz * 0.150f32), p_min + ImVec2::new(sz * 0.80f32, sz * 0.300f32), col);
    RenderArrowPointingAt(draw_list, p_min + ImVec2::new2(sz * 0.50f32, sz * 0.850f32), ImVec2::new(sz * 0.3f32, sz * 0.400f32), ImGuiDir_Down, col);
}

static inline c_float ImAcos01(c_float x)
{
    if (x <= 0f32) return IM_PI * 0.5f32;
    if (x >= 1f32) return 0f32;
    return ImAcos(x);
    //return (-0.69813170079773212f * x * x - 0.872664625997164770f32) * x + 1.5707963267948966f; // Cheap approximation, may be enough for what we do.
}

// FIXME: Cleanup and move code to ImDrawList.
c_void RenderRectFilledRangeH(draw_list: *mut ImDrawList, rect: &ImRect, col: u32, x_start_norm: c_float, x_end_norm: c_float, rounding: c_float)
{
    if x_end_norm == x_start_norm
{
        return;
}
    if x_start_norm > x_end_norm
{
        ImSwap(x_start_norm, x_end_norm);
}

    let p0: ImVec2 = ImVec2::new2(ImLerp(rect.Min.x, rect.Max.x, x_start_norm), rect.Min.y);
    let p1: ImVec2 = ImVec2::new2(ImLerp(rect.Min.x, rect.Max.x, x_end_norm), rect.Max.y);
    if (rounding == 0f32)
    {
        draw_list->AddRectFilled(p0, p1, col, 0f32);
        return;
    }

    rounding = ImClamp(ImMin((rect.Max.x - rect.Min.x) * 0.5f32, (rect.Max.y - rect.Min.y) * 0.5f32) - 1f32, 0f32, rounding);
    let     : c_float =  1f32 / rounding;
    let     : c_float =  ImAcos01(1f32 - (p0.x - rect.Min.x) * inv_rounding);
    let     : c_float =  ImAcos01(1f32 - (p1.x - rect.Min.x) * inv_rounding);
    let     : c_float =  IM_PI * 0.5f32; // We will == compare to this because we know this is the exact value ImAcos01 can return.
    let     : c_float =  ImMax(p0.x, rect.Min.x + rounding);
    if (arc0_b == arc0_e)
    {
        draw_list.PathLineTo(ImVec2::new2(x0, p1.y));
        draw_list.PathLineTo(ImVec2::new2(x0, p0.y));
    }
    else if (arc0_b == 0f32 && arc0_e == half_pi)
    {
        draw_list.PathArcToFast(ImVec2::new2(x0, p1.y - rounding), rounding, 3, 6); // BL
        draw_list.PathArcToFast(ImVec2::new2(x0, p0.y + rounding), rounding, 6, 9); // TR
    }
    else
    {
        draw_list.PathArcTo(ImVec2::new2(x0, p1.y - rounding), rounding, IM_PI - arc0_e, IM_PI - arc0_b, 3); // BL
        draw_list.PathArcTo(ImVec2::new2(x0, p0.y + rounding), rounding, IM_PI + arc0_b, IM_PI + arc0_e, 3); // TR
    }
    if (p1.x > rect.Min.x + rounding)
    {
        let         : c_float =  ImAcos01(1f32 - (rect.Max.x - p1.x) * inv_rounding);
        let         : c_float =  ImAcos01(1f32 - (rect.Max.x - p0.x) * inv_rounding);
        let         : c_float =  ImMin(p1.x, rect.Max.x - rounding);
        if (arc1_b == arc1_e)
        {
            draw_list.PathLineTo(ImVec2::new2(x1, p0.y));
            draw_list.PathLineTo(ImVec2::new2(x1, p1.y));
        }
        else if (arc1_b == 0f32 && arc1_e == half_pi)
        {
            draw_list.PathArcToFast(ImVec2::new2(x1, p0.y + rounding), rounding, 9, 12); // TR
            draw_list.PathArcToFast(ImVec2::new2(x1, p1.y - rounding), rounding, 0, 3);  // BR
        }
        else
        {
            draw_list.PathArcTo(ImVec2::new2(x1, p0.y + rounding), rounding, -arc1_e, -arc1_b, 3); // TR
            draw_list.PathArcTo(ImVec2::new2(x1, p1.y - rounding), rounding, +arc1_b, +arc1_e, 3); // BR
        }
    }
    draw_list->PathFillConvex(col);
}

c_void RenderRectFilledWithHole(draw_list: *mut ImDrawList, outer: &ImRect, inner: &ImRect, col: u32, rounding: c_float)
{
    let fill_L: bool = (inner.Min.x > outer.Min.x);
    let fill_R: bool = (inner.Max.x < outer.Max.x);
    let fill_U: bool = (inner.Min.y > outer.Min.y);
    let fill_D: bool = (inner.Max.y < outer.Max.y);
    if (fill_L) draw_list.AddRectFilled(ImVec2::new2(outer.Min.x, inner.Min.y), ImVec2::new(inner.Min.x, inner.Max.y), col, rounding, ImDrawFlags_RoundCornersNone | (fill_U ? 0 : ImDrawFlags_RoundCornersTopLeft)    | (fill_D ? 0 : ImDrawFlags_RoundCornersBottomLeft));
    if (fill_R) draw_list.AddRectFilled(ImVec2::new2(inner.Max.x, inner.Min.y), ImVec2::new(outer.Max.x, inner.Max.y), col, rounding, ImDrawFlags_RoundCornersNone | (fill_U ? 0 : ImDrawFlags_RoundCornersTopRight)   | (fill_D ? 0 : ImDrawFlags_RoundCornersBottomRight));
    if (fill_U) draw_list.AddRectFilled(ImVec2::new2(inner.Min.x, outer.Min.y), ImVec2::new(inner.Max.x, inner.Min.y), col, rounding, ImDrawFlags_RoundCornersNone | (fill_L ? 0 : ImDrawFlags_RoundCornersTopLeft)    | (fill_R ? 0 : ImDrawFlags_RoundCornersTopRight));
    if (fill_D) draw_list.AddRectFilled(ImVec2::new2(inner.Min.x, inner.Max.y), ImVec2::new(inner.Max.x, outer.Max.y), col, rounding, ImDrawFlags_RoundCornersNone | (fill_L ? 0 : ImDrawFlags_RoundCornersBottomLeft) | (fill_R ? 0 : ImDrawFlags_RoundCornersBottomRight));
    if (fill_L && fill_U) draw_list.AddRectFilled(ImVec2::new2(outer.Min.x, outer.Min.y), ImVec2::new(inner.Min.x, inner.Min.y), col, rounding, ImDrawFlags_RoundCornersTopLeft);
    if (fill_R && fill_U) draw_list.AddRectFilled(ImVec2::new2(inner.Max.x, outer.Min.y), ImVec2::new(outer.Max.x, inner.Min.y), col, rounding, ImDrawFlags_RoundCornersTopRight);
    if (fill_L && fill_D) draw_list.AddRectFilled(ImVec2::new2(outer.Min.x, inner.Max.y), ImVec2::new(inner.Min.x, outer.Max.y), col, rounding, ImDrawFlags_RoundCornersBottomLeft);
    if (fill_R && fill_D) draw_list.AddRectFilled(ImVec2::new2(inner.Max.x, inner.Max.y), ImVec2::new(outer.Max.x, outer.Max.y), col, rounding, ImDrawFlags_RoundCornersBottomRight);
}

ImDrawFlags ImGui::CalcRoundingFlagsForRectInRect(const ImRect& r_in, const ImRect& r_outer, c_float threshold)
{
    let mut round_l: bool =  r_in.Min.x <= r_outer.Min.x + threshold;
    let mut round_r: bool =  r_in.Max.x >= r_outer.Max.x - threshold;
    let mut round_t: bool =  r_in.Min.y <= r_outer.Min.y + threshold;
    let mut round_b: bool =  r_in.Max.y >= r_outer.Max.y - threshold;
    return ImDrawFlags_RoundCornersNone
        | ((round_t && round_l) ? ImDrawFlags_RoundCornersTopLeft : 0) | ((round_t && round_r) ? ImDrawFlags_RoundCornersTopRight : 0)
        | ((round_b && round_l) ? ImDrawFlags_RoundCornersBottomLeft : 0) | ((round_b && round_r) ? ImDrawFlags_RoundCornersBottomRight : 0);
}

// Helper for ColorPicker4()
// NB: This is rather brittle and will show artifact when rounding this enabled if rounded corners overlap multiple cells. Caller currently responsible for avoiding that.
// Spent a non reasonable amount of time trying to getting this right for ColorButton with rounding+anti-aliasing+ImGuiColorEditFlags_HalfAlphaPreview flag + various grid sizes and offsets, and eventually gave up... probably more reasonable to disable rounding altogether.
// FIXME: uses GetColorU32
c_void RenderColorRectWithAlphaCheckerboard(draw_list: *mut ImDrawList, p_min: ImVec2, p_max: ImVec2, col: u32, grid_step: c_float, grid_off: ImVec2, rounding: c_float, ImDrawFlags flags)
{
    if (flags & ImDrawFlags_RoundCornersMask_) == 0
{
        flags = ImDrawFlags_RoundCornersDefault_;
}
    if (((col & IM_COL32_A_MASK) >> IM_COL32_A_SHIFT) < 0xF0f32)
    {
        u32 col_bg1 = GetColorU32(ImAlphaBlendColors(IM_COL32(204, 204, 204, 255), col));
        u32 col_bg2 = GetColorU32(ImAlphaBlendColors(IM_COL32(128, 128, 128, 255), col));
        draw_list->AddRectFilled(p_min, p_max, col_bg1, rounding, flags);

        c_int yi = 0;
        for (c_float y = p_min.y + grid_off.y; y < p_max.y; y += grid_step, yi++)
        {
            let y1: c_float =  ImClamp(y, p_min.y, p_max.y), y2 = ImMin(y + grid_step, p_max.y);
            if y2 <= y1
{
                continue;
}
            for (let x: c_float =  p_min.x + grid_off.x + (yi & 1) * grid_step; x < p_max.x; x += grid_step * 2.00f32)
            {
                let x1: c_float =  ImClamp(x, p_min.x, p_max.x), x2 = ImMin(x + grid_step, p_max.x);
                if x2 <= x1
{
                    continue;
}
                ImDrawFlags cell_flags = ImDrawFlags_RoundCornersNone;
                if (y1 <= p_min.y) { if (x1 <= p_min.x) cell_flags |= ImDrawFlags_RoundCornersTopLeft; if (x2 >= p_max.x) cell_flags |= ImDrawFlags_RoundCornersTopRight; }
                if (y2 >= p_max.y) { if (x1 <= p_min.x) cell_flags |= ImDrawFlags_RoundCornersBottomLeft; if (x2 >= p_max.x) cell_flags |= ImDrawFlags_RoundCornersBottomRight; }

                // Combine flags
                cell_flags = (flags == ImDrawFlags_RoundCornersNone || cell_flags == ImDrawFlags_RoundCornersNone) ? ImDrawFlags_RoundCornersNone : (cell_flags & flags);
                draw_list.AddRectFilled(ImVec2::new2(x1, y1), ImVec2::new(x2, y2), col_bg2, rounding, cell_flags);
            }
        }
    }
    else
    {
        draw_list->AddRectFilled(p_min, p_max, col, rounding, flags);
    }
}

//-----------------------------------------------------------------------------
// [SECTION] Decompression code
//-----------------------------------------------------------------------------
// Compressed with stb_compress() then converted to a C array and encoded as base85.
// Use the program in misc/fonts/binary_to_compressed_c.cpp to create the array from a TTF file.
// The purpose of encoding as base85 instead of "0x00,0x01,..." style is only save on _source code_ size.
// Decompression from stb.h (public domain) by Sean Barrett https://github.com/nothings/stb/blob/master/stb.h
//-----------------------------------------------------------------------------

static c_uint stb_decompress_length(const c_uchar *input)
{
    return (input[8] << 24) + (input[9] << 16) + (input[10] << 8) + input[11];
}

static c_uchar *stb__barrier_out_e, *stb__barrier_out_b;
static const c_uchar *stb__barrier_in_b;
static c_uchar *stb__dout;
static c_void stb__match(const c_uchar *data, c_uint length)
{
    // INVERSE of memmove... write each byte before copying the next...
    // IM_ASSERT(stb__dout + length <= stb__barrier_out_e);
    if (stb__dout + length > stb__barrier_out_e) { stb__dout += length; return; }
    if (data < stb__barrier_out_b) { stb__dout = stb__barrier_out_e+1; return; }
    while (length--) *stb__dout++ = *data+= 1;
}

static c_void stb__lit(const c_uchar *data, c_uint length)
{
    // IM_ASSERT(stb__dout + length <= stb__barrier_out_e);
    if (stb__dout + length > stb__barrier_out_e) { stb__dout += length; return; }
    if (data < stb__barrier_in_b) { stb__dout = stb__barrier_out_e+1; return; }
    memcpy(stb__dout, data, length);
    stb__dout += length;
}

// #define stb__in2(x)   ((i[x] << 8) + i[(x)+1])
// #define stb__in3(x)   ((i[x] << 16) + stb__in2((x)+1))
// #define stb__in4(x)   ((i[x] << 24) + stb__in3((x)+1))

static const c_uchar *stb_decompress_token(const c_uchar *i)
{
    if (*i >= 0x20) { // use fewer if's for cases that expand small
        if (*i >= 0x80)       stb__match(stb__dout-i[1]-1, i[0] - 0x80 + 1), i += 2;
        else if (*i >= 0x40)  stb__match(stb__dout-(stb__in2(0) - 0x4000 + 1), i[2]+1), i += 3;
        else /* *i >= 0x20 */ stb__lit(i+1, i[0] - 0x20 + 1), i += 1 + (i[0] - 0x20 + 1);
    } else { // more ifs for cases that expand large, since overhead is amortized
        if (*i >= 0x18)       stb__match(stb__dout-(stb__in3(0) - 0x180000 + 1), i[3]+1), i += 4;
        else if (*i >= 0x10)  stb__match(stb__dout-(stb__in3(0) - 0x100000 + 1), stb__in2(3)+1), i += 5;
        else if (*i >= 0x08)  stb__lit(i+2, stb__in2(0) - 0x0800 + 1), i += 2 + (stb__in2(0) - 0x0800 + 1);
        else if (*i == 0x07)  stb__lit(i+3, stb__in2(1) + 1), i += 3 + (stb__in2(1) + 1);
        else if (*i == 0x06)  stb__match(stb__dout-(stb__in3(1)+1), i[4]+1), i += 5;
        else if (*i == 0x04)  stb__match(stb__dout-(stb__in3(1)+1), stb__in2(4)+1), i += 6;
    }
    return i;
}

static c_uint stb_adler32(c_uint adler32, c_uchar *buffer, c_uint buflen)
{
    const unsigned long ADLER_MOD = 65521;
    unsigned long s1 = adler32 & 0xffff, s2 = adler32 >> 16;
    unsigned long blocklen = buflen % 5552;

    unsigned long i;
    while (buflen) {
        for (i=0; i + 7 < blocklen; i += 8) {
            s1 += buffer[0], s2 += s1;
            s1 += buffer[1], s2 += s1;
            s1 += buffer[2], s2 += s1;
            s1 += buffer[3], s2 += s1;
            s1 += buffer[4], s2 += s1;
            s1 += buffer[5], s2 += s1;
            s1 += buffer[6], s2 += s1;
            s1 += buffer[7], s2 += s1;

            buffer += 8;
        }

        for (; i < blocklen; ++i)
            s1 += *buffer++, s2 += s1;

        s1 %= ADLER_MOD, s2 %= ADLER_MOD;
        buflen -= blocklen;
        blocklen = 5552;
    }
    return (s2 << 16) + s1;
}

static c_uint stb_decompress(c_uchar *output, const c_uchar *i, c_uint /*length*/)
{
    if (stb__in4(0) != 0x57bC0000) return 0;
    if (stb__in4(4) != 0)          return 0; // error! stream is > 4GB
    const c_uint olen = stb_decompress_length(i);
    stb__barrier_in_b = i;
    stb__barrier_out_e = output + olen;
    stb__barrier_out_b = output;
    i += 16;

    stb__dout = output;
    for (;;) {
        const c_uchar *old_i = i;
        i = stb_decompress_token(i);
        if (i == old_i) {
            if (*i == 0x05 && i[1] == 0xfa) {
                // IM_ASSERT(stb__dout == output + olen);
                if (stb__dout != output + olen) return 0;
                if stb_adler32(1, output, olen) !=  stb__in4(2)
{
                    return 0;
}
                return olen;
            } else {
                // IM_ASSERT(0); /* NOTREACHED */
                return 0;
            }
        }
        // IM_ASSERT(stb__dout <= output + olen);
        if stb__dout > output + olen
{
            return 0;
}
    }
}

//-----------------------------------------------------------------------------
// [SECTION] Default font data (ProggyClean.tt0f32)
//-----------------------------------------------------------------------------
// ProggyClean.ttf
// Copyright (c) 2004, 2005 Tristan Grimmer
// MIT license (see License.txt in http://www.upperbounds.net/download/ProggyClean.ttf.zip)
// Download and more information at http://upperbounds.net
//-----------------------------------------------------------------------------
// File: 'ProggyClean.ttf' (41208 bytes)
// Exported using misc/fonts/binary_to_compressed_c.cpp (with compression + base85 string encoding).
// The purpose of encoding as base85 instead of "0x00,0x01,..." style is only save on _source code_ size.
//-----------------------------------------------------------------------------
static const char proggy_clean_ttf_compressed_data_base85[11980 + 1] =
    "7])#######hV0qs'/###[),##/l:$#Q6>##5[n42>c-TH`->>#/e>11NNV=Bv(*:.F?uu#(gRU.o0XGH`$vhLG1hxt9?W`#,5LsCp#-i>.r$<$6pD>Lb';9Crc6tgXmKVeU2cD4Eo3R/"
    "2*>]b(MC;$jPfY.;h^`IWM9<Lh2TlS+f-s$o6Q<BWH`YiU.xfLq$N;$0iR/GX:U(jcW2p/W*q?-qmnUCI;jHSAiFWM.R*kU@C=GH?a9wp8f$e.-4^Qg1)Q-GL(lf(r/7GrRgwV%MS=C#"
    "`8ND>Qo#t'X#(v#Y9w0#1D$CIf;W'#pWUPXOuxXuU(H9M(1<q-UE31#^-V'8IRUo7Qf./L>=Ke$$'5F%)]0^#0X@U.a<r:QLtFsLcL6##lOj)#.Y5<-R&KgLwqJfLgN&;Q?gI^#DY2uL"
    "i@^rMl9t=cWq6##weg>$FBjVQTSDgEKnIS7EM9>ZY9w0#L;>>#Mx&4Mvt//L[MkA#W@lK.N'[0#7RL_&#w+F%HtG9M#XL`N&.,GM4Pg;-<nLENhvx>-VsM.M0rJfLH2eTM`*oJMHRC`N"
    "kfimM2J,W-jXS:)r0wK#@Fge$U>`w'N7G#$#fB#$E^$#:9:hk+eOe--6x)F7*E%?76%^GMHePW-Z5l'&GiF#$956:rS?dA#fiK:)Yr+`&#0j@'DbG&#^$PG.Ll+DNa<XCMKEV*N)LN/N"
    "*b=%Q6pia-Xg8I$<MR&,VdJe$<(7G;Ckl'&hF;;$<_=X(b.RS%%)###MPBuuE1V:v&cX&#2m#(&cV]`k9OhLMbn%s$G2,B$BfD3X*sp5#l,$R#]x_X1xKX%b5U*[r5iMfUo9U`N99hG)"
    "tm+/Us9pG)XPu`<0s-)WTt(gCRxIg(%6sfh=ktMKn3j)<6<b5Sk_/0(^]AaN#(p/L>&VZ>1i%h1S9u5o@YaaW$e+b<TWFn/Z:Oh(Cx2$lNEoN^e)#CFY@@I;BOQ*sRwZtZxRcU7uW6CX"
    "ow0i(?$Q[cjOd[P4d)]>ROPOpxTO7Stwi1::iB1q)C_=dV26J;2,]7op$]uQr@_V7$q^%lQwtuHY]=DX,n3L#0PHDO4f9>dC@O>HBuKPpP*E,N+b3L#lpR/MrTEH.IAQk.a>D[.e;mc."
    "x]Ip.PH^'/aqUO/$1WxLoW0[iLA<QT;5HKD+@qQ'NQ(3_PLhE48R.qAPSwQ0/WK?Z,[x?-J;jQTWA0X@KJ(_Y8N-:/M74:/-ZpKrUss?d#dZq]DAbkU*JqkL+nwX@@47`5>w=4h(9.`G"
    "CRUxHPeR`5Mjol(dUWxZa(>STrPkrJiWx`5U7F#.g*jrohGg`cg:lSTvEY/EV_7H4Q9[Z%cnv;JQYZ5q.l7Zeas:HOIZOB?G<Nald$qs]@]L<J7bR*>gv:[7MI2k).'2($5FNP&EQ(,)"
    "U]W]+fh18.vsai00);D3@4ku5P?DP8aJt+;qUM]=+b'8@;mViBKx0DE[-auGl8:PJ&Dj+M6OC]O^((##]`0i)drT;-7X`=-H3[igUnPG-NZlo.#k@h#=Ork$m>a>$-?Tm$UV(?#P6YY#"
    "'/###xe7q.73rI3*pP/$1>s9)W,JrM7SN]'/4C#v$U`0#V.[0>xQsH$fEmPMgY2u7Kh(G%siIfLSoS+MK2eTM$=5,M8p`A.;_R%#u[K#$x4AG8.kK/HSB==-'Ie/QTtG?-.*^N-4B/ZM"
    "_3YlQC7(p7q)&](`6_c)$/*JL(L-^(]$wIM`dPtOdGA,U3:w2M-0<q-]L_?^)1vw'.,MRsqVr.L;aN&#/EgJ)PBc[-f>+WomX2u7lqM2iEumMTcsF?-aT=Z-97UEnXglEn1K-bnEO`gu"
    "Ft(c%=;Am_Qs@jLooI&NX;]0#j4#F14;gl8-GQpgwhrq8'=l_f-b49'UOqkLu7-##oDY2L(te+Mch&gLYtJ,MEtJfLh'x'M=$CS-ZZ%P]8bZ>#S?YY#%Q&q'3^Fw&?D)UDNrocM3A76/"
    "/oL?#h7gl85[qW/NDOk%16ij;+:1a'iNIdb-ou8.P*w,v5#EI$TWS>Pot-R*H'-SEpA:g)f+O$%%`kA#G=8RMmG1&O`>to8bC]T&$,n.LoO>29sp3dt-52U%VM#q7'DHpg+#Z9%H[K<L"
    "%a2E-grWVM3@2=-k22tL]4$##6We'8UJCKE[d_=%wI;'6X-GsLX4j^SgJ$##R*w,vP3wK#iiW&#*h^D&R?jp7+/u&#(AP##XU8c$fSYW-J95_-Dp[g9wcO&#M-h1OcJlc-*vpw0xUX&#"
    "OQFKNX@QI'IoPp7nb,QU//MQ&ZDkKP)X<WSVL(68uVl&#c'[0#(s1X&xm$Y%B7*K:eDA323j998GXbA#pwMs-jgD$9QISB-A_(aN4xoFM^@C58D0+Q+q3n0#3U1InDjF682-SjMXJK)("
    "h$hxua_K]ul92%'BOU&#BRRh-slg8KDlr:%L71Ka:.A;%YULjDPmL<LYs8i#XwJOYaKPKc1h:'9Ke,g)b),78=I39B;xiY$bgGw-&.Zi9InXDuYa%G*f2Bq7mn9^#p1vv%#(Wi-;/Z5h"
    "o;#2:;%d&#x9v68C5g?ntX0X)pT`;%pB3q7mgGN)3%(P8nTd5L7GeA-GL@+%J3u2:(Yf>et`e;)f#Km8&+DC$I46>#Kr]]u-[=99tts1.qb#q72g1WJO81q+eN'03'eM>&1XxY-caEnO"
    "j%2n8)),?ILR5^.Ibn<-X-Mq7[a82Lq:F&#ce+S9wsCK*x`569E8ew'He]h:sI[2LM$[guka3ZRd6:t%IG:;$%YiJ:Nq=?eAw;/:nnDq0(CYcMpG)qLN4$##&J<j$UpK<Q4a1]MupW^-"
    "sj_$%[HK%'F####QRZJ::Y3EGl4'@%FkiAOg#p[##O`gukTfBHagL<LHw%q&OV0##F=6/:chIm0@eCP8X]:kFI%hl8hgO@RcBhS-@Qb$%+m=hPDLg*%K8ln(wcf3/'DW-$.lR?n[nCH-"
    "eXOONTJlh:.RYF%3'p6sq:UIMA945&^HFS87@$EP2iG<-lCO$%c`uKGD3rC$x0BL8aFn--`ke%#HMP'vh1/R&O_J9'um,.<tx[@%wsJk&bUT2`0uMv7gg#qp/ij.L56'hl;.s5CUrxjO"
    "M7-##.l+Au'A&O:-T72L]P`&=;ctp'XScX*rU.>-XTt,%OVU4)S1+R-#dg0/Nn?Ku1^0f$B*P:Rowwm-`0PKjYDDM'3]d39VZHEl4,.j']Pk-M.h^&:0FACm$maq-&sgw0t7/6(^xtk%"
    "LuH88Fj-ekm>GA#_>568x6(OFRl-IZp`&b,_P'$M<Jnq79VsJW/mWS*PUiq76;]/NM_>hLbxfc$mj`,O;&%W2m`Zh:/)Uetw:aJ%]K9h:TcF]u_-Sj9,VK3M.*'&0D[Ca]J9gp8,kAW]"
    "%(?A%R$f<->Zts'^kn=-^@c4%-pY6qI%J%1IGxfLU9CP8cbPlXv);C=b),<2mOvP8up,UVf3839acAWAW-W?#ao/^#%KYo8fRULNd2.>%m]UK:n%r$'sw]J;5pAoO_#2mO3n,'=H5(et"
    "Hg*`+RLgv>=4U8guD$I%D:W>-r5V*%j*W:Kvej.Lp$<M-SGZ':+Q_k+uvOSLiEo(<aD/K<CCc`'Lx>'?;++O'>()jLR-^u68PHm8ZFWe+ej8h:9r6L*0//c&iH&R8pRbA#Kjm%upV1g:"
    "a_#Ur7FuA#(tRh#.Y5K+@?3<-8m0$PEn;J:rh6?I6uG<-`wMU'ircp0LaE_OtlMb&1#6T.#FDKu#1Lw%u%+GM+X'e?YLfjM[VO0MbuFp7;>Q&#WIo)0@F%q7c#4XAXN-U&VB<HFF*qL("
    "$/V,;(kXZejWO`<[5?\?ewY(*9=%wDc;,u<'9t3W-(H1th3+G]ucQ]kLs7df($/*JL]@*t7Bu_G3_7mp7<iaQjO@.kLg;x3B0lqp7Hf,^Ze7-##@/c58Mo(3;knp0%)A7?-W+eI'o8)b<"
    "nKnw'Ho8C=Y>pqB>0ie&jhZ[?iLR@@_AvA-iQC(=ksRZRVp7`.=+NpBC%rh&3]R:8XDmE5^V8O(x<<aG/1N$#FX$0V5Y6x'aErI3I$7x%E`v<-BY,)%-?Psf*l?%C3.mM(=/M0:JxG'?"
    "7WhH%o'a<-80g0NBxoO(GH<dM]n.+%q@jH?f.UsJ2Ggs&4<-e47&Kl+f//9@`b+?.TeN_&B8Ss?v;^Trk;f#YvJkl&w$]>-+k?'(<S:68tq*WoDfZu';mM?8X[ma8W%*`-=;D.(nc7/;"
    ")g:T1=^J$&BRV(-lTmNB6xqB[@0*o.erM*<SWF]u2=st-*(6v>^](H.aREZSi,#1:[IXaZFOm<-ui#qUq2$##Ri;u75OK#(RtaW-K-F`S+cF]uN`-KMQ%rP/Xri.LRcB##=YL3BgM/3M"
    "D?@f&1'BW-)Ju<L25gl8uhVm1hL$##*8###'A3/LkKW+(^rWX?5W_8g)a(m&K8P>#bmmWCMkk&#TR`C,5d>g)F;t,4:@_l8G/5h4vUd%&%950:VXD'QdWoY-F$BtUwmfe$YqL'8(PWX("
    "P?^@Po3$##`MSs?DWBZ/S>+4%>fX,VWv/w'KD`LP5IbH;rTV>n3cEK8U#bX]l-/V+^lj3;vlMb&[5YQ8#pekX9JP3XUC72L,,?+Ni&co7ApnO*5NK,((W-i:$,kp'UDAO(G0Sq7MVjJs"
    "bIu)'Z,*[>br5fX^:FPAWr-m2KgL<LUN098kTF&#lvo58=/vjDo;.;)Ka*hLR#/k=rKbxuV`>Q_nN6'8uTG&#1T5g)uLv:873UpTLgH+#FgpH'_o1780Ph8KmxQJ8#H72L4@768@Tm&Q"
    "h4CB/5OvmA&,Q&QbUoi$a_%3M01H)4x7I^&KQVgtFnV+;[Pc>[m4k//,]1?#`VY[Jr*3&&slRfLiVZJ:]?=K3Sw=[$=uRB?3xk48@aeg<Z'<$#4H)6,>e0jT6'N#(q%.O=?2S]u*(m<-"
    "V8J'(1)G][68hW$5'q[GC&5j`TE?m'esFGNRM)j,ffZ?-qx8;->g4t*:CIP/[Qap7/9'#(1sao7w-.qNUdkJ)tCF&#B^;xGvn2r9FEPFFFcL@.iFNkTve$m%#QvQS8U@)2Z+3K:AKM5i"
    "sZ88+dKQ)W6>J%CL<KE>`.d*(B`-n8D9oK<Up]c$X$(,)M8Zt7/[rdkqTgl-0cuGMv'?>-XV1q['-5k'cAZ69e;D_?$ZPP&s^+7])$*$#@QYi9,5P&#9r+$%CE=68>K8r0=dSC%%(@p7"
    ".m7jilQ02'0-VWAg<a/''3u.=4L$Y)6k/K:_[3=&jvL<L0C/2'v:^;-DIBW,B4E68:kZ;%?8(Q8BH=kO65BW?xSG&#@uU,DS*,?.+(o(#1vCS8#CHF>TlGW'b)Tq7VT9q^*^$$.:&N@@"
    "$&)WHtPm*5_rO0&e%K&#-30j(E4#'Zb.o/(Tpm$>K'f@[PvFl,hfINTNU6u'0pao7%XUp9]5.>%h`8_=VYbxuel.NTSsJfLacFu3B'lQSu/m6-Oqem8T+oE--$0a/k]uj9EwsG>%veR*"
    "hv^BFpQj:K'#SJ,sB-'#](j.Lg92rTw-*n%@/;39rrJF,l#qV%OrtBeC6/,;qB3ebNW[?,Hqj2L.1NP&GjUR=1D8QaS3Up&@*9wP?+lo7b?@%'k4`p0Z$22%K3+iCZj?XJN4Nm&+YF]u"
    "@-W$U%VEQ/,,>>#)D<h#`)h0:<Q6909ua+&VU%n2:cG3FJ-%@Bj-DgLr`Hw&HAKjKjseK</xKT*)B,N9X3]krc12t'pgTV(Lv-tL[xg_%=M_q7a^x?7Ubd>#%8cY#YZ?=,`Wdxu/ae&#"
    "w6)R89tI#6@s'(6Bf7a&?S=^ZI_kS&ai`&=tE72L_D,;^R)7[$s<Eh#c&)q.MXI%#v9ROa5FZO%sF7q7Nwb&#ptUJ:aqJe$Sl68%.D###EC><?-aF&#RNQv>o8lKN%5/$(vdfq7+ebA#"
    "u1p]ovUKW&Y%q]'>$1@-[xfn$7ZTp7mM,G,Ko7a&Gu%G[RMxJs[0MM%wci.LFDK)(<c`Q8N)jEIF*+?P2a8g%)$q]o2aH8C&<SibC/q,(e:v;-b#6[$NtDZ84Je2KNvB#$P5?tQ3nt(0"
    "d=j.LQf./Ll33+(;q3L-w=8dX$#WF&uIJ@-bfI>%:_i2B5CsR8&9Z&#=mPEnm0f`<&c)QL5uJ#%u%lJj+D-r;BoF&#4DoS97h5g)E#o:&S4weDF,9^Hoe`h*L+_a*NrLW-1pG_&2UdB8"
    "6e%B/:=>)N4xeW.*wft-;$'58-ESqr<b?UI(_%@[P46>#U`'6AQ]m&6/`Z>#S?YY#Vc;r7U2&326d=w&H####?TZ`*4?&.MK?LP8Vxg>$[QXc%QJv92.(Db*B)gb*BM9dM*hJMAo*c&#"
    "b0v=Pjer]$gG&JXDf->'StvU7505l9$AFvgYRI^&<^b68?j#q9QX4SM'RO#&sL1IM.rJfLUAj221]d##DW=m83u5;'bYx,*Sl0hL(W;;$doB&O/TQ:(Z^xBdLjL<Lni;''X.`$#8+1GD"
    ":k$YUWsbn8ogh6rxZ2Z9]%nd+>V#*8U_72Lh+2Q8Cj0i:6hp&$C/:p(HK>T8Y[gHQ4`4)'$Ab(Nof%V'8hL&#<NEdtg(n'=S1A(Q1/I&4([%dM`,Iu'1:_hL>SfD07&6D<fp8dHM7/g+"
    "tlPN9J*rKaPct&?'uBCem^jn%9_K)<,C5K3s=5g&GmJb*[SYq7K;TRLGCsM-$$;S%:Y@r7AK0pprpL<Lrh,q7e/%KWK:50I^+m'vi`3?%Zp+<-d+$L-Sv:@.o19n$s0&39;kn;S%BSq*"
    "$3WoJSCLweV[aZ'MQIjO<7;X-X;&+dMLvu#^UsGEC9WEc[X(wI7#2.(F0jV*eZf<-Qv3J-c+J5AlrB#$p(H68LvEA'q3n0#m,[`*8Ft)FcYgEud]CWfm68,(aLA$@EFTgLXoBq/UPlp7"
    ":d[/;r_ix=:TF`S5H-b<LI&HY(K=h#)]Lk$K14lVfm:x$H<3^Ql<M`$OhapBnkup'D#L$Pb_`N*g]2e;X/Dtg,bsj&K#2[-:iYr'_wgH)NUIR8a1n#S?Yej'h8^58UbZd+^FKD*T@;6A"
    "7aQC[K8d-(v6GI$x:T<&'Gp5Uf>@M.*J:;$-rv29'M]8qMv-tLp,'886iaC=Hb*YJoKJ,(j%K=H`K.v9HggqBIiZu'QvBT.#=)0ukruV&.)3=(^1`o*Pj4<-<aN((^7('#Z0wK#5GX@7"
    "u][`*S^43933A4rl][`*O4CgLEl]v$1Q3AeF37dbXk,.)vj#x'd`;qgbQR%FW,2(?LO=s%Sc68%NP'##Aotl8x=BE#j1UD([3$M(]UI2LX3RpKN@;/#f'f/&_mt&0f32)XdF<9t4)Qa.*kT"
    "LwQ'(TTB9.xH'>#MJ+gLq9-##@HuZPN0]u:h7.T..G:;$/Usj(T7`Q8tT72LnYl<-qx8;-HV7Q-&Xdx%1a,hC=0u+HlsV>nuIQL-5<N?)NBS)QN*_I,?&)2'IM%L3I)X((e/dl2&8'<M"
    ":^#M*Q+[T.Xri.LYS3v%fF`68h;b-X[/En'CR.q7E)p'/kle2HM,u;^%OKC-N+Ll%F9CF<Nf'^#t2L,;27W:0O@6##U6W7:$rJfLWHj$#)woqBefIZ.PK<b*t7ed;p*_m;4ExK#h@&]>"
    "_>@kXQtMacfD.m-VAb8;IReM3$wf0''hra*so568'Ip&vRs849'MRYSp%:t:h5qSgwpEr$B>Q,;s(C#$)`svQuF$##-D,##,g68@2[T;.XSdN9Qe)rpt._K-#5w0f32)sP'##p#C0c%-Gb%"
    "hd+<-j'Ai*x&&HMkT]C'OSl##5RG[JXaHN;d'uA#x._U;.`PU@(Z3dt4r152@:v,'R.Sj'w#0<-;kPI)FfJ&#AYJ&#//)>-k=m=*XnK$>=)72L]0I%>.G690a:$##<,);?;72#?x9+d;"
    "^V'9;jY@;)br#q^YQpx:X#Te$Z^'=-=bGhLf:D6&bNwZ9-ZD#n^9HhLMr5G;']d&6'wYmTFmL<LD)F^%[tC'8;+9E#C$g%#5Y>q9wI>P(9mI[>kC-ekLC/R&CH+s'B;K-M6$EB%is00:"
    "+A4[7xks.LrNk0&E)wILYF@2L'0Nb$+pv<(2.768/FrY&h$^3i&@+G%JT'<-,v`3;_)I9M^AE]CN?Cl2AZg+%4iTpT3<n-&%H%b<FDj2M<hH=&Eh<2Len$b*aTX=-8QxN)k11IM1c^j%"
    "9s<L<NFSo)B?+<-(GxsF,^-Eh@$4dXhN$+#rxK8'je'D7k`e;)2pYwPA'_p9&@^18ml1^[@g4t*[JOa*[=Qp7(qJ_oOL^('7fB&Hq-:sf,sNj8xq^>$U4O]GKx'm9)b@p7YsvK3w^YR-"
    "CdQ*:Ir<($u&)#(&?L9Rg3H)4fiEp^iI9O8KnTj,]H?D*r7'M;PwZ9K0E^k&-cpI;.p/6_vwoFMV<->#%Xi.LxVnrU(4&8/P+:hLSKj$#U%]49t'I:rgMi'FL@a:0Y-uA[39',(vbma*"
    "hU%<-SRF`Tt:542R_VV$p@[p8DV[A,?1839FWdF<TddF<9Ah-6&9tWoDlh]&1SpGMq>Ti1O*H&#(AL8[_P%.M>v^-))qOT*F5Cq0`Ye%+$B6i:7@0IX<N+T+0MlMBPQ*Vj>SsD<U4JHY"
    "8kD2)2fU/M#$e.)T4,_=8hLim[&);?UkK'-x?'(:siIfL<$pFM`i<?%W(mGDHM%>iWP,##P`%/L<eXi:@Z9C.7o=@(pXdAO/NLQ8lPl+HPOQa8wD8=^GlPa8TKI1CjhsCTSLJM'/Wl>-"
    "S(qw%sf/@%#B6;/U7K]uZbi^Oc^2n<bhPmUkMw>%t<)'mEVE''n`WnJra$^TKvX5B>;_aSEK',(hwa0:i4G?.Bci.(X[?b*($,=-n<.Q%`(X=?+@Am*Js0&=3bh8K]mL<LoNs'6,'85`"
    "0?t/'_U59@]ddF<#LdF<eWdF<OuN/45rY<-L@&#+fm>69=Lb,OcZV/);TTm8VI;?%OtJ<(b4mq7M6:u?KRdF<gR@2L=FNU-<b[(9c/ML3m;Z[$oF3g)GAWqpARc=<ROu7cL5l;-[A]%/"
    "+fsd;l#SafT/f*W]0=O'$(Tb<[)*@e775R-:Yob%g*>l*:xP?Yb.5)%w_I?7uk5JC+FS(m#i'k.'a0i)9<7b'fs'59hq$*5Uhv##pi^8+hIEBF`nvo`;'l0.^S1<-wUK2/Coh58KKhLj"
    "M=SO*rfO`+qC`W-On.=AJ56>>i2@2LH6A:&5q`?9I3@@'04&p2/LVa*T-4<-i3;M9UvZd+N7>b*eIwg:CC)c<>nO&#<IGe;__.thjZl<%w(Wk2xmp4Q@I#I9,DF]u7-P=.-_:YJ]aS@V"
    "?6*C()dOp7:WL,b&3Rg/.cmM9&r^>$(>.Z-I&J(Q0Hd5Q%7Co-b`-c<N(6r@ip+AurK<m86QIth*#v;-OBqi+L7wDE-Ir8K['m+DDSLwK&/.?-V%U_%3:qKNu$_b*B-kp7NaD'QdWQPK"
    "Yq[@>P)hI;*_F]u`Rb[.j8_Q/<&>uu+VsH$sM9TA%?)(vmJ80),P7E>)tjD%2L=-t#fK[%`v=Q8<FfNkgg^oIbah*#8/Qt$F&:K*-(N/'+1vMB,u()-a.VUU*#[e%gAAO(S>WlA2);Sa"
    ">gXm8YB`1d@K#n]76-a$U,mF<fX]idqd)<3,]J7JmW4`6]uks=4-72L(jEk+:bJ0M^q-8Dm_Z?0olP1C9Sa&H[d&c$ooQUj]Exd*3ZM@-WGW2%s',B-_M%>%Ul:#/'xoFM9QX-$.QN'>"
    "[%$Z$uF6pA6Ki2O5:8w*vP1<-1`[G,)-m#>0`P&#eb#.3i)rtB61(o'$?X3B</R90;eZ]%Ncq;-Tl]#F>2Qft^ae_5tKL9MUe9b*sLEQ95C&`=G?@Mj=wh*'3E>=-<)Gt*Iw)'QG:`@I"
    "wOf7&]1i'S01B+Ev/Nac#9S;=;YQpg_6U`*kVY39xK,[/6Aj7:'1Bm-_1EYfa1+o&o4hp7KN_Q(OlIo@S%;jVdn0'1<Vc52=u`3^o-n1'g4v58Hj&6_t7$##?M)c<$bgQ_'SY((-xkA#"
    "Y(,p'H9rIVY-b,'%bCPF7.J<Up^,(dU1VY*5#WkTU>h19w,WQhLI)3S#f$2(eb,jr*b;3Vw]*7NH%$c4Vs,eD9>XW8?N]o+(*pgC%/72LV-u<Hp,3@e^9UB1J+ak9-TN/mhKPg+AJYd$"
    "MlvAF_jCK*.O-^(63adMT->W%iewS8W6m2rtCpo'RS1R84=@paTKt)>=%&1[)*vp'u+x,VrwN;&]kuO9JDbg=pO$J*.jVe;u'm0dr9l,<*wMK*Oe=g8lV_KEBFkO'oU]^=[-792#ok,)"
    "i]lR8qQ2oA8wcRCZ^7w/Njh;?.stX?Q1>S1q4Bn$)K1<-rGdO'$Wr.Lc.CG)$/*JL4tNR/,SVO3,aUw'DJN:)Ss;wGn9A32ijw%FL+Z0Fn.U9;reSq)bmI32U==5ALuG&#Vf1398/pVo"
    "1*c-(aY168o<`JsSbk-,1N;$>0:OUas(3:8Z972LSfF8eb=c-;>SPw7.6hn3m`9^Xkn(r.qS[0;T%&Qc=+STRxX'q1BNk3&*eu2;&8q$&x>Q#Q7^Tf+6<(d%ZVmj2bDi%.3L2n+4W'$P"
    "iDDG)g,r%+?,$@?uou5tSe2aN_AQU*<h`e-GI7)?OK2A.d7_c)?wQ5AS@DL3r#7fSkgl6-++D:'A,uq7SvlB$pcpH'q3n0#_%dY#xCpr-l<F0NR@-##FEV6NTF6##$l84N1w?AO>'IAO"
    "URQ##V^Fv-XFbGM7Fl(N<3DhLGF%q.1rC$#:T__&Pi68%0xi_&[qFJ(77j_&JWoF.V735&T,[R*:xFR*K5>>#`bW-?4Ne_&6Ne_&6Ne_&n`kr-#GJcM6X;uM6X;uM(.a..^2TkL%oR(#"
    ";u.T%fAr%4tJ8&><1=GHZ_+m9/#H1F^R#SC#*N=BA9(D?v[UiFY>>^8p,KKF.W]L29uLkLlu/+4T<XoIB&hx=T1PcDaB&;HH+-AFr?(m9HZV)FKS8JCw;SD=6[^/DZUL`EUDf]GGlG&>"
    "w$)F./^n3+rlo+DB;5sIYGNk+i1t-69Jg--0pao7Sm#K)pdHW&;LuDNH@H>#/X-TI(;P>#,Gc>#0Su>#4`1?#8lC?#<xU?#@.i?#D:%@#HF7@#LRI@#P_[@#Tkn@#Xw*A#]-=A#a9OA#"
    "d<F&#*;G##.GY##2Sl##6`($#:l:$#>xL$#B.`$#F:r$#JF.%#NR@%#R_R%#Vke%#Zww%#_-4&#3^Rh%Sflr-k'MS.o?.5/sWel/wpEM0%3'/1)K^f1-d>G21&v(35>V`39V7A4=onx4"
    "A1OY5EI0;6Ibgr6M$HS7Q<)58C5w,;WoA*#[%T*#`1g*#d=#+#hI5+#lUG+#pbY+#tnl+#x$),#&1;,#*=M,#.I`,#2Ur,#6b.-#;w[H#iQtA#m^0B#qjBB#uvTB##-hB#'9$C#+E6C#"
    "/QHC#3^ZC#7jmC#;v)D#?,<D#C8ND#GDaD#KPsD#O]/E#g1A5#KA*1#gC17#MGd;#8(02#L-d3#rWM4#Hga1#,<w0#T.j<#O#'2#CYN1#qa^:#_4m3#o@/=#eG8=#t8J5#`+78#4uI-#"
    "m3B2#SB[8#Q0@8#i[*9#iOn8#1Nm;#^sN9#qh<9#:=x-#P;K2#$%X9#bC+.#Rg;<#mN=.#MTF.#RZO.#2?)4#Y#(/#[)1/#b;L/#dAU/#0Sv;#lY$0#n`-0#sf60#(F24#wrH0#%/e0#"
    "TmD<#%JSMFove:CTBEXI:<eh2g)B,3h2^G3i;#d3jD>)4kMYD4lVu`4m`:&5niUA5@(A5BA1]PBB:xlBCC=2CDLXMCEUtiCf&0g2'tN?PGT4CPGT4CPGT4CPGT4CPGT4CPGT4CPGT4CP"
    "GT4CPGT4CPGT4CPGT4CPGT4CPGT4CP-qekC`.9kEg^+F$kwViFJTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5o,^<-28ZI'O?;xp"
    "O?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xp;7q-#lLYI:xvD=#";

static *const char GetDefaultCompressedFontDataTTFBase85()
{
    return proggy_clean_ttf_compressed_data_base85;
}

// #endif // #ifndef IMGUI_DISABLE
