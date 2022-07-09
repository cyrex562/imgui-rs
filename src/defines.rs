// dear imgui, v1.88

use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use crate::button::DimgButtonFlags;
use crate::combo::DimgComboFlags;
use crate::draw_channel::DimgDrawChannel;
use crate::draw_cmd::DimgDrawCmd;
use crate::draw_list::DimgDrawList;
use crate::font::{DimgFont, ImFontBuilderIO};
use crate::font_atlas::DimgFontAtlas;
use crate::vec_nd::{DimgVec2D, DimgVec4};
use crate::input::{DimgKey, ImGuiInputCallbackData};
use crate::table::{BORDERS_OUTER, DimgTableFlags};
use crate::table_column::DimgTableColumnFlags;
use crate::types::ImWchar32;
use crate::viewport::DimgViewportFlags;
use crate::window::DimgHoveredFlags;

// (headers)

// Help:
// - Read FAQ at http://dearimgui.org/faq
// - Newcomers, read 'Programmer guide' in imgui.cpp for notes on how to setup Dear ImGui in your codebase.
// - Call and read ImGui::ShowDemoWindow() in imgui_demo.cpp. All applications in examples/ are doing that.
// Read imgui.cpp for details, links and comments.

// Resources:
// - FAQ                   http://dearimgui.org/faq
// - Homepage & latest     https://github.com/ocornut/imgui
// - Releases & changelog  https://github.com/ocornut/imgui/releases
// - Gallery               https://github.com/ocornut/imgui/issues/5243 (please post your screenshots/video there!)
// - Wiki                  https://github.com/ocornut/imgui/wiki (lots of good stuff there)
// - Glossary              https://github.com/ocornut/imgui/wiki/Glossary
// - Issues & support      https://github.com/ocornut/imgui/issues

// Getting Started?
// - For first-time users having issues compiling/linking/running or issues loading fonts:
//   please post in https://github.com/ocornut/imgui/discussions if you cannot find a solution in resources above.

/*

Index of this file:
// [SECTION] Header mess
// [SECTION] Forward declarations and basic types
// [SECTION] Dear ImGui end-user API functions
// [SECTION] flags & Enumerations
// [SECTION] Helpers: Memory allocations macros, ImVector<>
// [SECTION] ImGuiStyle
// [SECTION] ImGuiIO
// [SECTION] Misc data structures (ImGuiInputTextCallbackData, ImGuiSizeCallbackData, ImGuiWindowClass, ImGuiPayload, ImGuiTableSortSpecs, ImGuiTableColumnSortSpecs)
// [SECTION] Helpers (ImGuiOnceUponAFrame, ImGuiTextFilter, ImGuiTextBuffer, ImGuiStorage, ImGuiListClipper, ImColor)
// [SECTION] Drawing API (ImDrawCallback, ImDrawCmd, ImDrawIdx, ImDrawVert, ImDrawChannel, ImDrawListSplitter, ImDrawFlags, ImDrawListFlags, ImDrawList, ImDrawData)
// [SECTION] font API (ImFontConfig, ImFontGlyph, ImFontGlyphRangesBuilder, ImFontAtlasFlags, ImFontAtlas, ImFont)
// [SECTION] viewports (ImGuiViewportFlags, ImGuiViewport)
// [SECTION] Platform Dependent Interfaces (ImGuiPlatformIO, ImGuiPlatformMonitor, ImGuiPlatformImeData)
// [SECTION] Obsolete functions and types

*/

// #pragma once

// Configuration file with compile-time options (edit imconfig.h or '#define IMGUI_USER_CONFIG "myfilename.h" from your build system')
// #ifdef IMGUI_USER_CONFIG
// #include IMGUI_USER_CONFIG
// #endif
// #if !defined(IMGUI_DISABLE_INCLUDE_IMCONFIG_H) || defined(IMGUI_INCLUDE_IMCONFIG_H)
// #include "imconfig.rs"
// #endif

// #ifndef IMGUI_DISABLE

//-----------------------------------------------------------------------------
// [SECTION] Header mess
//-----------------------------------------------------------------------------

// Includes
// #include <float.h>                  // FLT_MIN, FLT_MAX
// #include <stdarg.h>                 // va_list, va_start, va_end
// #include <stddef.h>                 // ptrdiff_t, NULL
// #include <string.h>                 // memset, memmove, memcpy, strlen, strchr, strcpy, strcmp

// Version
// (Integer encoded as XYYZZ for use in #if preprocessor conditionals. Work in progress versions typically starts at XYY99 then bounce up to XYY00, XYY01 etc. when release tagging happens)
// #define IMGUI_VERSION               "1.88"
// #define IMGUI_VERSION_NUM           18800
// #define IMGUI_CHECKVERSION()        ImGui::DebugCheckVersionAndDataLayout(IMGUI_VERSION, sizeof(ImGuiIO), sizeof(ImGuiStyle), sizeof(ImVec2), sizeof(ImVec4), sizeof(ImDrawVert), sizeof(ImDrawIdx))
// #define IMGUI_HAS_TABLE
// #define IMGUI_HAS_VIEWPORT          // viewport WIP branch
// #define IMGUI_HAS_DOCK              // Docking WIP branch

// Define attributes of all API symbols declarations (e.g. for DLL under windows)
//  is used for core imgui functions, IMGUI_IMPL_API is used for the default backends files (imgui_impl_xxx.h)
// Using dear imgui via a shared library is not recommended, because we don't guarantee backward nor forward ABI compatibility (also function call overhead, as dear imgui is a call-heavy API)
// #ifndef
// #define
// #endif
// #ifndef IMGUI_IMPL_API
// #define IMGUI_IMPL_API
// #endif

// Helper Macros
// #ifndef IM_ASSERT
// #include <assert.h>
// #define IM_ASSERT(_EXPR)            assert(_EXPR)                               // You can override the default assert handler by editing imconfig.h
// #endif
// #define IM_ARRAYSIZE(_ARR)          ((sizeof(_ARR) / sizeof(*(_ARR))))     // size of a static C-style array. Don't use on pointers!
// #define IM_UNUSED(_VAR)             ((void)(_VAR))                              // Used to silence "unused variable warnings". Often useful as asserts may be stripped out from final builds.
// #define IM_OFFSETOF(_TYPE,_MEMBER)  offsetof(_TYPE, _MEMBER)                    // Offset of _MEMBER within _TYPE. Standardized as offsetof() in C++11

// Helper Macros - IM_FMTARGS, IM_FMTLIST: Apply printf-style warnings to our formatting functions.
// #if !defined(IMGUI_USE_STB_SPRINTF) && defined(__MINGW32__) && !defined(__clang__)
// #define IM_FMTARGS(FMT)             __attribute__((format(gnu_printf, FMT, FMT+1)))
// #define IM_FMTLIST(FMT)             __attribute__((format(gnu_printf, FMT, 0)))
// #elif !defined(IMGUI_USE_STB_SPRINTF) && (defined(__clang__) || defined(__GNUC__))
// #define IM_FMTARGS(FMT)             __attribute__((format(printf, FMT, FMT+1)))
// #define IM_FMTLIST(FMT)             __attribute__((format(printf, FMT, 0)))
// #else
// #define IM_FMTARGS(FMT)
// #define IM_FMTLIST(FMT)
// #endif

// Disable some of MSVC most aggressive Debug runtime checks in function header/footer (used in some simple/low-level functions)
// #if defined(_MSC_VER) && !defined(__clang__)  && !defined(__INTEL_COMPILER) && !defined(IMGUI_DEBUG_PARANOID)
// #define IM_MSVC_RUNTIME_CHECKS_OFF      __pragma(runtime_checks("",off))     __pragma(check_stack(off)) __pragma(strict_gs_check(push,off))
// #define IM_MSVC_RUNTIME_CHECKS_RESTORE  __pragma(runtime_checks("",restore)) __pragma(check_stack())    __pragma(strict_gs_check(pop))
// #else
// #define IM_MSVC_RUNTIME_CHECKS_OFF
// #define IM_MSVC_RUNTIME_CHECKS_RESTORE
// #endif

// Warnings
// #ifdef _MSC_VER
// #pragma warning (push)
// #pragma warning (disable: 26495)    // [Static Analyzer] Variable 'XXX' is uninitialized. Always initialize a member variable (type.6).
// #endif
// #if defined(__clang__)
// #pragma clang diagnostic push
// #pragma clang diagnostic ignored "-Wold-style-cast"
// #if __has_warning("-Wzero-as-null-pointer-constant")
// #pragma clang diagnostic ignored "-Wzero-as-null-pointer-constant"
// #endif
// #elif defined(__GNUC__)
// #pragma GCC diagnostic push
// #pragma GCC diagnostic ignored "-Wpragmas"          // warning: unknown option after '#pragma GCC diagnostic' kind
// #pragma GCC diagnostic ignored "-Wclass-memaccess"  // [__GNUC__ >= 8] warning: 'memset/memcpy' clearing/writing an object of type 'xxxx' with no trivial copy-assignment; use assignment or value-initialization instead
// #endif

//-----------------------------------------------------------------------------
// [SECTION] Forward declarations and basic types
//-----------------------------------------------------------------------------

// Forward declarations
// struct ImDrawChannel;               // Temporary storage to output draw commands out of order, used by ImDrawListSplitter and ImDrawList::ChannelsSplit()
// struct ImDrawCmd;                   // A single draw command within a parent ImDrawList (generally maps to 1 GPU draw call, unless it is a callback)
// struct ImDrawData;                  // All draw command lists required to render the frame + pos/size coordinates to use for the projection matrix.
// struct ImDrawList;                  // A single draw command list (generally one per window, conceptually you may see this as a dynamic "mesh" builder)
// struct ImDrawListSharedData;        // data shared among multiple draw lists (typically owned by parent ImGui context, but you may create one yourself)
// struct ImDrawListSplitter;          // Helper to split a draw list into different layers which can be drawn into out of order, then flattened back.
// struct ImDrawVert;                  // A single vertex (pos + uv + col = 20 bytes by default. Override layout with IMGUI_OVERRIDE_DRAWVERT_STRUCT_LAYOUT)
// struct ImFont;                      // Runtime data for a single font within a parent ImFontAtlas
// struct ImFontAtlas;                 // Runtime data for multiple fonts, bake multiple fonts into a single texture, TTF/OTF font loader
// struct ImFontBuilderIO;             // Opaque interface to a font builder (stb_truetype or FreeType).
// struct ImFontConfig;                // Configuration data when adding a font or merging fonts
// struct ImFontGlyph;                 // A single font glyph (code point + coordinates within in ImFontAtlas + offset)
// struct ImFontGlyphRangesBuilder;    // Helper to build glyph ranges from text/string data
// struct ImColor;                     // Helper functions to create a color that can be converted to either u32 or float4 (*OBSOLETE* please avoid using)
// struct ImGuiContext;                // Dear ImGui context (opaque structure, unless including imgui_internal.h)
// struct ImGuiIO;                     // Main configuration and I/O between your application and ImGui
// struct ImGuiInputTextCallbackData;  // Shared state of InputText() when using custom ImGuiInputTextCallback (rare/advanced use)
// struct ImGuiKeyData;                // Storage for ImGuiIO and IsKeyDown(), IsKeyPressed() etc functions.
// struct ImGuiListClipper;            // Helper to manually clip large list of items
// struct ImGuiOnceUponAFrame;         // Helper for running a block of code not more than once a frame
// struct ImGuiPayload;                // User data payload for drag and drop operations
// struct ImGuiPlatformIO;             // Multi-viewport support: interface for Platform/Renderer backends + viewports to render
// struct ImGuiPlatformMonitor;        // Multi-viewport support: user-provided bounds for each connected monitor/display. Used when positioning popups and tooltips to avoid them straddling monitors
// struct ImGuiPlatformImeData;        // Platform IME data for io.SetPlatformImeDataFn() function.
// struct ImGuiSizeCallbackData;       // Callback data when using SetNextWindowSizeConstraints() (rare/advanced use)
// struct ImGuiStorage;                // Helper for key->value storage
// struct ImGuiStyle;                  // Runtime data for styling/colors
// struct ImGuiTableSortSpecs;         // Sorting specifications for a table (often handling sort specs for a single column, occasionally more)
// struct ImGuiTableColumnSortSpecs;   // Sorting specification for one column of a table
// struct ImGuiTextBuffer;             // Helper to hold and append into a text buffer (~string builder)
// struct ImGuiTextFilter;             // Helper to parse and apply text filters (e.g. "aaaaa[,bbbbb][,ccccc]")
// struct ImGuiViewport;               // A Platform Window (always 1 unless multi-viewport are enabled. One per platform window to output to). In the future may represent Platform Monitor
// struct ImGuiWindowClass;            // Window class (rare/advanced uses: provide hints to the platform backend via altered viewport flags and parent/child info)

// Enums/flags (declared as int for compatibility with old C++, to allow using as flags without overhead, and to not pollute the top of this file)
// - Tip: Use your programming IDE navigation facilities on the names in the _central column_ below to find the actual flags/enum lists!
//   In Visual Studio IDE: CTRL+comma ("Edit.GoToAll") can follow symbols in comments, whereas CTRL+F12 ("Edit.GoToImplementation") cannot.
//   With Visual Assist installed: ALT+G ("VAssistX.GoToImplementation") can also follow symbols in comments.
// typedef pub ImGuiCol: i32,             // -> enum ImGuiCol_             // Enum: A color identifier for styling
// typedef pub ImGuiCond: i32,            // -> enum ImGuiCond_            // Enum: A condition for many Set*() functions
// typedef pub ImGuiDataType: i32,        // -> enum ImGuiDataType_        // Enum: A primary data type
// typedef pub ImGuiDir: i32,             // -> enum ImGuiDir_             // Enum: A cardinal direction
// typedef pub ImGuiKey: i32,             // -> enum ImGuiKey_             // Enum: A key identifier
// typedef pub ImGuiNavInput: i32,        // -> enum ImGuiNavInput_        // Enum: An input identifier for navigation
// typedef pub ImGuiMouseButton: i32,     // -> enum ImGuiMouseButton_     // Enum: A mouse button identifier (0=left, 1=right, 2=middle)
// typedef pub ImGuiMouseCursor: i32,     // -> enum ImGuiMouseCursor_     // Enum: A mouse cursor identifier
// pub type ImGuiMouseCursor = i32;
// typedef pub ImGuiSortDirection: i32,   // -> enum ImGuiSortDirection_   // Enum: A sorting direction (ascending or descending)
// typedef pub ImGuiStyleVar: i32,        // -> enum ImGuiStyleVar_        // Enum: A variable identifier for styling
// typedef pub ImGuiTableBgTarget: i32,   // -> enum ImGuiTableBgTarget_   // Enum: A color target for TableSetBgColor()
// typedef pub ImDrawFlags: i32,          // -> enum ImDrawFlags_          // flags: for ImDrawList functions
// typedef pub ImDrawListFlags: i32,      // -> enum       // flags: for ImDrawList instance
// typedef pub ImFontAtlasFlags: i32,     // -> enum      // flags: for ImFontAtlas build
// typedef pub ImGuiBackendFlags: i32,    // -> enum ImGuiBackendFlags_    // flags: for io.backend_flags
// typedef pub ImGuiButtonFlags: i32,     // -> enum ImGuiButtonFlags_     // flags: for InvisibleButton()
// typedef pub ImGuiColorEditFlags: i32,  // -> enum ImGuiColorEditFlags_  // flags: for ColorEdit4(), ColorPicker4() etc.
// typedef pub ImGuiConfigFlags: i32,     // -> enum ImGuiConfigFlags_     // flags: for io.config_flags
// typedef pub ImGuiComboFlags: i32,      // -> enum ImGuiComboFlags_      // flags: for BeginCombo()
// typedef pub ImGuiDockNodeFlags: i32,   // -> enum ImGuiDockNodeFlags_   // flags: for DockSpace()
// typedef pub ImGuiDragDropFlags: i32,   // -> enum ImGuiDragDropFlags_   // flags: for BeginDragDropSource(), AcceptDragDropPayload()
// typedef pub ImGuiFocusedFlags: i32,    // -> enum ImGuiFocusedFlags_    // flags: for IsWindowFocused()
// typedef pub ImGuiHoveredFlags: i32,    // -> enum ImGuiHoveredFlags_    // flags: for IsItemHovered(), IsWindowHovered() etc.
// typedef pub ImGuiInputTextFlags: i32,  // -> enum ImGuiInputTextFlags_  // flags: for InputText(), InputTextMultiline()
// typedef pub ImGuiModFlags: i32,        // -> enum ImGuiModFlags_        // flags: for io.key_mods (Ctrl/Shift/Alt/Super)
// typedef pub ImGuiPopupFlags: i32,      // -> enum ImGuiPopupFlags_      // flags: for OpenPopup*(), BeginPopupContext*(), IsPopupOpen()
// typedef pub ImGuiSelectableFlags: i32, // -> enum ImGuiSelectableFlags_ // flags: for Selectable()
// typedef pub ImGuiSliderFlags: i32,     // -> enum ImGuiSliderFlags_     // flags: for DragFloat(), DragInt(), SliderFloat(), SliderInt() etc.
// typedef pub ImGuiTabBarFlags: i32,     // -> enum ImGuiTabBarFlags_     // flags: for BeginTabBar()
// typedef pub ImGuiTabItemFlags: i32,    // -> enum ImGuiTabItemFlags_    // flags: for BeginTabItem()
// typedef pub ImGuiTableFlags: i32,      // -> enum ImGuiTableFlags_      // flags: For BeginTable()
// typedef pub ImGuiTableColumnFlags: i32,// -> enum ImGuiTableColumnFlags_// flags: For TableSetupColumn()
// typedef pub ImGuiTableRowFlags: i32,   // -> enum ImGuiTableRowFlags_   // flags: For TableNextRow()
// typedef pub ImGuiTreeNodeFlags: i32,   // -> enum ImGuiTreeNodeFlags_   // flags: for TreeNode(), TreeNodeEx(), CollapsingHeader()
// typedef pub ImGuiViewportFlags: i32,   // -> enum    // flags: for ImGuiViewport
// typedef pub ImGuiWindowFlags: i32,     // -> enum ImGuiWindowFlags_     // flags: for Begin(), BeginChild()

// #endif

// ImDrawIdx: vertex index. [Compile-time configurable type]
// - To use 16-bit indices + allow large meshes: backend need to set 'io.backend_flags |= ImGuiBackendFlags_RendererHasVtxOffset' and handle ImDrawCmd::vtx_offset (recommended).
// - To use 32-bit indices: override with '#define ImDrawIdx unsigned int' in your imconfig.h file.
// #ifndef ImDrawIdx
// typedef unsigned short ImDrawIdx;   // Default: 16-bit (for maximum compatibility with renderer backends)
// #endif
// typedef signed char         ImS8;   // 8-bit signed integer
// typedef unsigned char       ImU8;   // 8-bit unsigned integer
// typedef signed short        ImS16;  // 16-bit signed integer
// typedef unsigned short      ImU16;  // 16-bit unsigned integer
// typedef signed pub ImS32: i32,// 32-bit signed integer == int
// typedef unsigned pub ImU32: i32,// 32-bit unsigned integer (often used to store packed colors)
// typedef signed   long long  ImS64;  // 64-bit signed integer
// typedef unsigned long long  ImU64;  // 64-bit unsigned integer
// typedef void*   (*ImGuiMemAllocFunc)(size_t sz, void* user_data);               // Function signature for ImGui::SetAllocatorFunctions()
// pub type ImGuiMemAllocFunc = fn(usize, *mut c_void) -> *mut c_void;
// typedef void    (*ImGuiMemFreeFunc)(void* ptr, void* user_data);                // Function signature for ImGui::SetAllocatorFunctions()
// pub type ImGuiMemFreeFunc = fn(*mut c_void, *mut c_void);

// IM_MSVC_RUNTIME_CHECKS_RESTORE

//-----------------------------------------------------------------------------
// [SECTION] Dear ImGui end-user API functions
// (Note that ImGui:: being a namespace, you can add extra ImGui:: functions in your own separate file. Please don't modify imgui source files!)
//-----------------------------------------------------------------------------

// namespace ImGui
// {
//     // Context creation and access
//     // - Each context create its own ImFontAtlas by default. You may instance one yourself and pass it to CreateContext() to share a font atlas between contexts.
//     // - DLL users: heaps and globals are not shared across DLL boundaries! You will need to call SetCurrentContext() + SetAllocatorFunctions()
//     //   for each static/DLL boundary you are calling from. Read "Context and Memory Allocators" section of imgui.cpp for details.
//      ImGuiContext* CreateContext(ImFontAtlas* shared_font_atlas = NULL);
//      void          DestroyContext(ImGuiContext* ctx = NULL);   // NULL = destroy current context
//      ImGuiContext* GetCurrentContext();
//      void          SetCurrentContext(ImGuiContext* ctx);
//
//     // Main
//      ImGuiIO&      GetIO();                                    // access the io structure (mouse/keyboard/gamepad inputs, time, various configuration options/flags)
//      ImGuiStyle&   GetStyle();                                 // access the style structure (colors, sizes). Always use PushStyleCol(), PushStyleVar() to modify style mid-frame!
//      void          NewFrame();                                 // start a new Dear ImGui frame, you can submit any command from this point until Render()/EndFrame().
//      void          EndFrame();                                 // ends the Dear ImGui frame. automatically called by Render(). If you don't need to render data (skipping rendering) you may call EndFrame() without Render()... but you'll have wasted CPU already! If you don't need to render, better to not create any windows and not call NewFrame() at all!
//      void          Render();                                   // ends the Dear ImGui frame, finalize the draw data. You can then get call GetDrawData().
//      ImDrawData*   GetDrawData();                              // valid after Render() and until the next call to NewFrame(). this is what you have to render.
//
//     // Demo, Debug, Information
//      void          ShowDemoWindow(bool* p_open = NULL);        // create Demo window. demonstrate most ImGui features. call this to learn about the library! try to make it always available in your application!
//      void          ShowMetricsWindow(bool* p_open = NULL);     // create Metrics/Debugger window. display Dear ImGui internals: windows, draw commands, various internal state, etc.
//      void          ShowDebugLogWindow(bool* p_open = NULL);    // create Debug Log window. display a simplified log of important dear imgui events.
//      void          ShowStackToolWindow(bool* p_open = NULL);   // create Stack Tool window. hover items with mouse to query information about the source of their unique id.
//      void          ShowAboutWindow(bool* p_open = NULL);       // create About window. display Dear ImGui version, credits and build/system information.
//      void          ShowStyleEditor(ImGuiStyle* ref = NULL);    // add style editor block (not a window). you can pass in a reference ImGuiStyle structure to compare to, revert to and save to (else it uses the default style)
//      bool          ShowStyleSelector(const char* label);       // add style selector block (not a window), essentially a combo listing the default styles.
//      void          ShowFontSelector(const char* label);        // add font selector block (not a window), essentially a combo listing the loaded fonts.
//      void          ShowUserGuide();                            // add basic help/info block (not a window): how to manipulate ImGui as a end-user (mouse/keyboard controls).
//      const char*   GetVersion();                               // get the compiled version string e.g. "1.80 WIP" (essentially the value for IMGUI_VERSION from the compiled version of imgui.cpp)
//
//     // Styles
//      void          StyleColorsDark(ImGuiStyle* dst = NULL);    // new, recommended style (default)
//      void          StyleColorsLight(ImGuiStyle* dst = NULL);   // best used with borders and a custom, thicker font
//      void          StyleColorsClassic(ImGuiStyle* dst = NULL); // classic imgui style
//
//     // windows
//     // - Begin() = push window to the stack and start appending to it. End() = pop window from the stack.
//     // - Passing 'bool* p_open != NULL' shows a window-closing widget in the upper-right corner of the window,
//     //   which clicking will set the boolean to false when clicked.
//     // - You may append multiple times to the same window during the same frame by calling Begin()/End() pairs multiple times.
//     //   Some information such as 'flags' or 'p_open' will only be considered by the first call to Begin().
//     // - Begin() return false to indicate the window is collapsed or fully clipped, so you may early out and omit submitting
//     //   anything to the window. Always call a matching End() for each Begin() call, regardless of its return value!
//     //   [Important: due to legacy reason, this is inconsistent with most other functions such as BeginMenu/EndMenu,
//     //    BeginPopup/EndPopup, etc. where the EndXXX call should only be called if the corresponding BeginXXX function
//     //    returned true. Begin and BeginChild are the only odd ones out. Will be fixed in a future update.]
//     // - Note that the bottom of window stack always contains a window called "Debug".
//      bool          Begin(const char* name, bool* p_open = NULL, ImGuiWindowFlags flags = 0);
//      void          End();
//
//     // Child windows
//     // - Use child windows to begin into a self-contained independent scrolling/clipping regions within a host window. Child windows can embed their own child.
//     // - For each independent axis of 'size': ==0.0: use remaining host window size / >0.0: fixed size / <0.0: use remaining window size minus abs(size) / Each axis can use a different mode, e.g. ImVec2(0,400).
//     // - BeginChild() returns false to indicate the window is collapsed or fully clipped, so you may early out and omit submitting anything to the window.
//     //   Always call a matching EndChild() for each BeginChild() call, regardless of its return value.
//     //   [Important: due to legacy reason, this is inconsistent with most other functions such as BeginMenu/EndMenu,
//     //    BeginPopup/EndPopup, etc. where the EndXXX call should only be called if the corresponding BeginXXX function
//     //    returned true. Begin and BeginChild are the only odd ones out. Will be fixed in a future update.]
//      bool          BeginChild(const char* str_id, const ImVec2& size = ImVec2(0, 0), bool border = false, ImGuiWindowFlags flags = 0);
//      bool          BeginChild(ImGuiID id, const ImVec2& size = ImVec2(0, 0), bool border = false, ImGuiWindowFlags flags = 0);
//      void          EndChild();
//
//     // windows Utilities
//     // - 'current window' = the window we are appending into while inside a Begin()/End() block. 'next window' = next window we will Begin() into.
//      bool          IsWindowAppearing();
//      bool          IsWindowCollapsed();
//      bool          IsWindowFocused(ImGuiFocusedFlags flags=0); // is current window focused? or its root/child, depending on flags. see flags for options.
//      bool          IsWindowHovered(ImGuiHoveredFlags flags=0); // is current window hovered (and typically: not blocked by a popup/modal)? see flags for options. NB: If you are trying to check whether your mouse should be dispatched to imgui or to your app, you should use the 'io.want_capture_mouse' boolean for that! Please read the FAQ!
//      ImDrawList*   GetWindowDrawList();                        // get draw list associated to the current window, to append your own drawing primitives
//      float         GetWindowDpiScale();                        // get DPI scale currently associated to the current window's viewport.
//      ImVec2        GetWindowPos();                             // get current window position in screen space (useful if you want to do your own drawing via the draw_list API)
//      ImVec2        GetWindowSize();                            // get current window size
//      float         GetWindowWidth();                           // get current window width (shortcut for GetWindowSize().x)
//      float         GetWindowHeight();                          // get current window height (shortcut for GetWindowSize().y)
//      ImGuiViewport*GetWindowViewport();                        // get viewport currently associated to the current window.
//
//     // Window manipulation
//     // - Prefer using SetNextXXX functions (before Begin) rather that SetXXX functions (after Begin).
//      void          SetNextWindowPos(const ImVec2& pos, ImGuiCond cond = 0, const ImVec2& pivot = ImVec2(0, 0)); // set next window position. call before Begin(). use pivot=(0.5,0.5) to center on given point, etc.
//      void          SetNextWindowSize(const ImVec2& size, ImGuiCond cond = 0);                  // set next window size. set axis to 0.0 to force an auto-fit on this axis. call before Begin()
//      void          SetNextWindowSizeConstraints(const ImVec2& size_min, const ImVec2& size_max, ImGuiSizeCallback custom_callback = NULL, void* custom_callback_data = NULL); // set next window size limits. use -1,-1 on either x/Y axis to preserve the current size. Sizes will be rounded down. Use callback to apply non-trivial programmatic constraints.
//      void          SetNextWindowContentSize(const ImVec2& size);                               // set next window content size (~ scrollable client area, which enforce the range of scrollbars). Not including window decorations (title bar, menu bar, etc.) nor window_padding. set an axis to 0.0 to leave it automatic. call before Begin()
//      void          SetNextWindowCollapsed(bool collapsed, ImGuiCond cond = 0);                 // set next window collapsed state. call before Begin()
//      void          SetNextWindowFocus();                                                       // set next window to be focused / top-most. call before Begin()
//      void          SetNextWindowBgAlpha(float alpha);                                          // set next window background color alpha. helper to easily override the Alpha component of ImGuiCol_WindowBg/ChildBg/PopupBg. you may also use ImGuiWindowFlags_NoBackground.
//      void          SetNextWindowViewport(ImGuiID viewport_id);                                 // set next window viewport
//      void          SetWindowPos(const ImVec2& pos, ImGuiCond cond = 0);                        // (not recommended) set current window position - call within Begin()/End(). prefer using SetNextWindowPos(), as this may incur tearing and side-effects.
//      void          SetWindowSize(const ImVec2& size, ImGuiCond cond = 0);                      // (not recommended) set current window size - call within Begin()/End(). set to ImVec2(0, 0) to force an auto-fit. prefer using SetNextWindowSize(), as this may incur tearing and minor side-effects.
//      void          SetWindowCollapsed(bool collapsed, ImGuiCond cond = 0);                     // (not recommended) set current window collapsed state. prefer using SetNextWindowCollapsed().
//      void          SetWindowFocus();                                                           // (not recommended) set current window to be focused / top-most. prefer using SetNextWindowFocus().
//      void          SetWindowFontScale(float scale);                                            // [OBSOLETE] set font scale. Adjust io.font_global_scale if you want to scale all windows. This is an old API! For correct scaling, prefer to reload font + rebuild ImFontAtlas + call style.scale_all_sizes().
//      void          SetWindowPos(const char* name, const ImVec2& pos, ImGuiCond cond = 0);      // set named window position.
//      void          SetWindowSize(const char* name, const ImVec2& size, ImGuiCond cond = 0);    // set named window size. set axis to 0.0 to force an auto-fit on this axis.
//      void          SetWindowCollapsed(const char* name, bool collapsed, ImGuiCond cond = 0);   // set named window collapsed state
//      void          SetWindowFocus(const char* name);                                           // set named window to be focused / top-most. use NULL to remove focus.
//
//     // Content region
//     // - Retrieve available space from a given point. GetContentRegionAvail() is frequently useful.
//     // - Those functions are bound to be redesigned (they are confusing, incomplete and the Min/Max return values are in local window coordinates which increases confusion)
//      ImVec2        GetContentRegionAvail();                                        // == GetContentRegionMax() - GetCursorPos()
//      ImVec2        GetContentRegionMax();                                          // current content boundaries (typically window boundaries including scrolling, or current column boundaries), in windows coordinates
//      ImVec2        GetWindowContentRegionMin();                                    // content boundaries min for the full window (roughly (0,0)-scroll), in window coordinates
//      ImVec2        GetWindowContentRegionMax();                                    // content boundaries max for the full window (roughly (0,0)+size-scroll) where size can be override with SetNextWindowContentSize(), in window coordinates
//
//     // windows Scrolling
//      float         GetScrollX();                                                   // get scrolling amount [0 .. GetScrollMaxX()]
//      float         GetScrollY();                                                   // get scrolling amount [0 .. GetScrollMaxY()]
//      void          SetScrollX(float scroll_x);                                     // set scrolling amount [0 .. GetScrollMaxX()]
//      void          SetScrollY(float scroll_y);                                     // set scrolling amount [0 .. GetScrollMaxY()]
//      float         GetScrollMaxX();                                                // get maximum scrolling amount ~~ content_size.x - WindowSize.x - DecorationsSize.x
//      float         GetScrollMaxY();                                                // get maximum scrolling amount ~~ content_size.y - WindowSize.y - DecorationsSize.y
//      void          SetScrollHereX(float center_x_ratio = 0.5);                    // adjust scrolling amount to make current cursor position visible. center_x_ratio=0.0: left, 0.5: center, 1.0: right. When using to make a "default/current item" visible, consider using SetItemDefaultFocus() instead.
//      void          SetScrollHereY(float center_y_ratio = 0.5);                    // adjust scrolling amount to make current cursor position visible. center_y_ratio=0.0: top, 0.5: center, 1.0: bottom. When using to make a "default/current item" visible, consider using SetItemDefaultFocus() instead.
//      void          SetScrollFromPosX(float local_x, float center_x_ratio = 0.5);  // adjust scrolling amount to make given position visible. Generally GetCursorStartPos() + offset to compute a valid position.
//      void          SetScrollFromPosY(float local_y, float center_y_ratio = 0.5);  // adjust scrolling amount to make given position visible. Generally GetCursorStartPos() + offset to compute a valid position.
//
//     // Parameters stacks (shared)
//      void          PushFont(ImFont* font);                                         // use NULL as a shortcut to push default font
//      void          PopFont();
//      void          PushStyleColor(ImGuiCol idx, ImU32 col);                        // modify a style color. always use this if you modify the style after NewFrame().
//      void          PushStyleColor(ImGuiCol idx, const ImVec4& col);
//      void          PopStyleColor(int count = 1);
//      void          PushStyleVar(ImGuiStyleVar idx, float val);                     // modify a style float variable. always use this if you modify the style after NewFrame().
//      void          PushStyleVar(ImGuiStyleVar idx, const ImVec2& val);             // modify a style ImVec2 variable. always use this if you modify the style after NewFrame().
//      void          PopStyleVar(int count = 1);
//      void          PushAllowKeyboardFocus(bool allow_keyboard_focus);              // == tab stop enable. Allow focusing using TAB/Shift-TAB, enabled by default but you can disable it for certain widgets
//      void          PopAllowKeyboardFocus();
//      void          PushButtonRepeat(bool repeat);                                  // in 'repeat' mode, Button*() functions return repeated true in a typematic manner (using io.key_repeat_delay/io.key_repeat_rate setting). Note that you can call IsItemActive() after any Button() to tell if the button is held in the current frame.
//      void          PopButtonRepeat();
//
//     // Parameters stacks (current window)
//      void          PushItemWidth(float item_width);                                // push width of items for common large "item+label" widgets. >0.0: width in pixels, <0.0 align xx pixels to the right of window (so -FLT_MIN always align width to the right side).
//      void          PopItemWidth();
//      void          SetNextItemWidth(float item_width);                             // set width of the _next_ common large "item+label" widget. >0.0: width in pixels, <0.0 align xx pixels to the right of window (so -FLT_MIN always align width to the right side)
//      float         CalcItemWidth();                                                // width of item given pushed settings and current cursor position. NOT necessarily the width of last item unlike most 'Item' functions.
//      void          PushTextWrapPos(float wrap_local_pos_x = 0.0);                 // push word-wrapping position for Text*() commands. < 0.0: no wrapping; 0.0: wrap to end of window (or column); > 0.0: wrap at 'wrap_pos_x' position in window local space
//      void          PopTextWrapPos();
//
//     // style read access
//     // - Use the style editor (ShowStyleEditor() function) to interactively see what the colors are)
//      ImFont*       GetFont();                                                      // get current font
//      float         GetFontSize();                                                  // get current font size (= height in pixels) of current font with current scale applied
//      ImVec2        GetFontTexUvWhitePixel();                                       // get UV coordinate for a while pixel, useful to draw custom shapes via the ImDrawList API
//      ImU32         GetColorU32(ImGuiCol idx, float alpha_mul = 1.0);              // retrieve given style color with style alpha applied and optional extra alpha multiplier, packed as a 32-bit value suitable for ImDrawList
//      ImU32         GetColorU32(const ImVec4& col);                                 // retrieve given color with style alpha applied, packed as a 32-bit value suitable for ImDrawList
//      ImU32         GetColorU32(ImU32 col);                                         // retrieve given color with style alpha applied, packed as a 32-bit value suitable for ImDrawList
//      const ImVec4& GetStyleColorVec4(ImGuiCol idx);                                // retrieve style color as stored in ImGuiStyle structure. use to feed back into PushStyleColor(), otherwise use GetColorU32() to get style color with style alpha baked in.
//
//     // Cursor / Layout
//     // - By "cursor" we mean the current output position.
//     // - The typical widget behavior is to output themselves at the current cursor position, then move the cursor one line down.
//     // - You can call SameLine() between widgets to undo the last carriage return and output at the right of the preceding widget.
//     // - Attention! We currently have inconsistencies between window-local and absolute positions we will aim to fix with future API:
//     //    Window-local coordinates:   SameLine(), GetCursorPos(), SetCursorPos(), GetCursorStartPos(), GetContentRegionMax(), GetWindowContentRegion*(), PushTextWrapPos()
//     //    Absolute coordinate:        GetCursorScreenPos(), SetCursorScreenPos(), all ImDrawList:: functions.
//      void          Separator();                                                    // separator, generally horizontal. inside a menu bar or in horizontal layout mode, this becomes a vertical separator.
//      void          SameLine(float offset_from_start_x=0.0, float spacing=-1.0);  // call between widgets or groups to layout them horizontally. x position given in window coordinates.
//      void          NewLine();                                                      // undo a SameLine() or force a new line when in an horizontal-layout context.
//      void          Spacing();                                                      // add vertical spacing.
//      void          Dummy(const ImVec2& size);                                      // add a dummy item of given size. unlike InvisibleButton(), Dummy() won't take the mouse click or be navigable into.
//      void          Indent(float indent_w = 0.0);                                  // move content position toward the right, by indent_w, or style.IndentSpacing if indent_w <= 0
//      void          Unindent(float indent_w = 0.0);                                // move content position back to the left, by indent_w, or style.IndentSpacing if indent_w <= 0
//      void          BeginGroup();                                                   // lock horizontal starting position
//      void          EndGroup();                                                     // unlock horizontal starting position + capture the whole group bounding box into one "item" (so you can use IsItemHovered() or layout primitives such as SameLine() on whole group, etc.)
//      ImVec2        GetCursorPos();                                                 // cursor position in window coordinates (relative to window position)
//      float         GetCursorPosX();                                                //   (some functions are using window-relative coordinates, such as: GetCursorPos, GetCursorStartPos, GetContentRegionMax, GetWindowContentRegion* etc.
//      float         GetCursorPosY();                                                //    other functions such as GetCursorScreenPos or everything in ImDrawList::
//      void          SetCursorPos(const ImVec2& local_pos);                          //    are using the main, absolute coordinate system.
//      void          SetCursorPosX(float local_x);                                   //    GetWindowPos() + GetCursorPos() == GetCursorScreenPos() etc.)
//      void          SetCursorPosY(float local_y);                                   //
//      ImVec2        GetCursorStartPos();                                            // initial cursor position in window coordinates
//      ImVec2        GetCursorScreenPos();                                           // cursor position in absolute coordinates (useful to work with ImDrawList API). generally top-left == GetMainViewport()->pos == (0,0) in single viewport mode, and bottom-right == GetMainViewport()->pos+size == io.display_size in single-viewport mode.
//      void          SetCursorScreenPos(const ImVec2& pos);                          // cursor position in absolute coordinates
//      void          AlignTextToFramePadding();                                      // vertically align upcoming text baseline to FramePadding.y so that it will align properly to regularly framed items (call if you have text on a line before a framed item)
//      float         GetTextLineHeight();                                            // ~ font_size
//      float         GetTextLineHeightWithSpacing();                                 // ~ font_size + style.ItemSpacing.y (distance in pixels between 2 consecutive lines of text)
//      float         GetFrameHeight();                                               // ~ font_size + style.FramePadding.y * 2
//      float         GetFrameHeightWithSpacing();                                    // ~ font_size + style.FramePadding.y * 2 + style.ItemSpacing.y (distance in pixels between 2 consecutive lines of framed widgets)
//
//     // id stack/scopes
//     // Read the FAQ (docs/FAQ.md or http://dearimgui.org/faq) for more details about how id are handled in dear imgui.
//     // - Those questions are answered and impacted by understanding of the id stack system:
//     //   - "Q: Why is my widget not reacting when I click on it?"
//     //   - "Q: How can I have widgets with an empty label?"
//     //   - "Q: How can I have multiple widgets with the same label?"
//     // - Short version: id are hashes of the entire id stack. If you are creating widgets in a loop you most likely
//     //   want to push a unique identifier (e.g. object pointer, loop index) to uniquely differentiate them.
//     // - You can also use the "Label##foobar" syntax within widget label to distinguish them from each others.
//     // - In this header file we use the "label"/"name" terminology to denote a string that will be displayed + used as an id,
//     //   whereas "str_id" denote a string that is only used as an id and not normally displayed.
//      void          PushID(const char* str_id);                                     // push string into the id stack (will hash string).
//      void          PushID(const char* str_id_begin, const char* str_id_end);       // push string into the id stack (will hash string).
//      void          PushID(const void* ptr_id);                                     // push pointer into the id stack (will hash pointer).
//      void          PushID(int int_id);                                             // push integer into the id stack (will hash integer).
//      void          PopID();                                                        // pop from the id stack.
//      ImGuiID       GetID(const char* str_id);                                      // calculate unique id (hash of whole id stack + given parameter). e.g. if you want to query into ImGuiStorage yourself
//      ImGuiID       GetID(const char* str_id_begin, const char* str_id_end);
//      ImGuiID       GetID(const void* ptr_id);
//
//     // Widgets: Text
//      void          TextUnformatted(const char* text, const char* text_end = NULL); // raw text without formatting. Roughly equivalent to Text("%s", text) but: A) doesn't require null terminated string if 'text_end' is specified, B) it's faster, no memory copy is done, no buffer size limits, recommended for long chunks of text.
//      void          Text(const char* fmt, ...)                                      IM_FMTARGS(1); // formatted text
//      void          TextV(const char* fmt, va_list args)                            IM_FMTLIST(1);
//      void          TextColored(const ImVec4& col, const char* fmt, ...)            IM_FMTARGS(2); // shortcut for PushStyleColor(ImGuiCol_Text, col); Text(fmt, ...); PopStyleColor();
//      void          TextColoredV(const ImVec4& col, const char* fmt, va_list args)  IM_FMTLIST(2);
//      void          TextDisabled(const char* fmt, ...)                              IM_FMTARGS(1); // shortcut for PushStyleColor(ImGuiCol_Text, style.Colors[ImGuiCol_TextDisabled]); Text(fmt, ...); PopStyleColor();
//      void          TextDisabledV(const char* fmt, va_list args)                    IM_FMTLIST(1);
//      void          TextWrapped(const char* fmt, ...)                               IM_FMTARGS(1); // shortcut for PushTextWrapPos(0.0); Text(fmt, ...); PopTextWrapPos();. Note that this won't work on an auto-resizing window if there's no other widgets to extend the window width, yoy may need to set a size using SetNextWindowSize().
//      void          TextWrappedV(const char* fmt, va_list args)                     IM_FMTLIST(1);
//      void          LabelText(const char* label, const char* fmt, ...)              IM_FMTARGS(2); // display text+label aligned the same way as value+label widgets
//      void          LabelTextV(const char* label, const char* fmt, va_list args)    IM_FMTLIST(2);
//      void          BulletText(const char* fmt, ...)                                IM_FMTARGS(1); // shortcut for Bullet()+Text()
//      void          BulletTextV(const char* fmt, va_list args)                      IM_FMTLIST(1);
//
//     // Widgets: Main
//     // - Most widgets return true when the value has been changed or when pressed/selected
//     // - You may also use one of the many IsItemXXX functions (e.g. IsItemActive, IsItemHovered, etc.) to query widget state.
//      bool          Button(const char* label, const ImVec2& size = ImVec2(0, 0));   // button
//      bool          SmallButton(const char* label);                                 // button with FramePadding=(0,0) to easily embed within text
//      bool          InvisibleButton(const char* str_id, const ImVec2& size, ImGuiButtonFlags flags = 0); // flexible button behavior without the visuals, frequently useful to build custom behaviors using the public api (along with IsItemActive, IsItemHovered, etc.)
//      bool          ArrowButton(const char* str_id, ImGuiDir dir);                  // square button with an arrow shape
//      void          Image(ImTextureID user_texture_id, const ImVec2& size, const ImVec2& uv0 = ImVec2(0, 0), const ImVec2& uv1 = ImVec2(1,1), const ImVec4& tint_col = ImVec4(1,1,1,1), const ImVec4& border_col = ImVec4(0,0,0,0));
//      bool          ImageButton(ImTextureID user_texture_id, const ImVec2& size, const ImVec2& uv0 = ImVec2(0, 0),  const ImVec2& uv1 = ImVec2(1,1), int frame_padding = -1, const ImVec4& bg_col = ImVec4(0,0,0,0), const ImVec4& tint_col = ImVec4(1,1,1,1));    // <0 frame_padding uses default frame padding settings. 0 for no padding
//      bool          Checkbox(const char* label, bool* v);
//      bool          CheckboxFlags(const char* label, int* flags, int flags_value);
//      bool          CheckboxFlags(const char* label, unsigned int* flags, unsigned int flags_value);
//      bool          RadioButton(const char* label, bool active);                    // use with e.g. if (RadioButton("one", my_value==1)) { my_value = 1; }
//      bool          RadioButton(const char* label, int* v, int v_button);           // shortcut to handle the above pattern when value is an integer
//      void          ProgressBar(float fraction, const ImVec2& size_arg = ImVec2(-FLT_MIN, 0), const char* overlay = NULL);
//      void          Bullet();                                                       // draw a small circle + keep the cursor on the same line. advance cursor x position by GetTreeNodeToLabelSpacing(), same distance that TreeNode() uses
//
//     // Widgets: Combo Box
//     // - The BeginCombo()/EndCombo() api allows you to manage your contents and selection state however you want it, by creating e.g. Selectable() items.
//     // - The old Combo() api are helpers over BeginCombo()/EndCombo() which are kept available for convenience purpose. This is analogous to how ListBox are created.
//      bool          BeginCombo(const char* label, const char* preview_value, ImGuiComboFlags flags = 0);
//      void          EndCombo(); // only call EndCombo() if BeginCombo() returns true!
//      bool          Combo(const char* label, int* current_item, const char* const items[], int items_count, int popup_max_height_in_items = -1);
//      bool          Combo(const char* label, int* current_item, const char* items_separated_by_zeros, int popup_max_height_in_items = -1);      // Separate items with \0 within a string, end item-list with \0\0. e.g. "One\0Two\0Three\0"
//      bool          Combo(const char* label, int* current_item, bool(*items_getter)(void* data, int idx, const char** out_text), void* data, int items_count, int popup_max_height_in_items = -1);
//
//     // Widgets: Drag Sliders
//     // - CTRL+Click on any drag box to turn them into an input box. Manually input values aren't clamped by default and can go off-bounds. Use ImGuiSliderFlags_AlwaysClamp to always clamp.
//     // - For all the Float2/Float3/Float4/Int2/Int3/Int4 versions of every functions, note that a 'float v[x]' function argument is the same as 'float* v',
//     //   the array syntax is just a way to document the number of elements that are expected to be accessible. You can pass address of your first element out of a contiguous set, e.g. &myvector.x
//     // - Adjust format string to decorate the value with a prefix, a suffix, or adapt the editing and display precision e.g. "%.3" -> 1.234; "%5.2 secs" -> 01.23 secs; "Biscuit: %.0" -> Biscuit: 1; etc.
//     // - Format string may also be set to NULL or use the default format ("%f" or "%d").
//     // - Speed are per-pixel of mouse movement (v_speed=0.2: mouse needs to move by 5 pixels to increase value by 1). For gamepad/keyboard navigation, minimum speed is Max(v_speed, minimum_step_at_given_precision).
//     // - Use v_min < v_max to clamp edits to given limits. Note that CTRL+Click manual input can override those limits if ImGuiSliderFlags_AlwaysClamp is not used.
//     // - Use v_max = FLT_MAX / INT_MAX etc to avoid clamping to a maximum, same with v_min = -FLT_MAX / INT_MIN to avoid clamping to a minimum.
//     // - We use the same sets of flags for DragXXX() and SliderXXX() functions as the features are the same and it makes it easier to swap them.
//     // - Legacy: Pre-1.78 there are DragXXX() function signatures that takes a final `float power=1.0' argument instead of the `ImGuiSliderFlags flags=0' argument.
//     //   If you get a warning converting a float to ImGuiSliderFlags, read https://github.com/ocornut/imgui/issues/3361
//      bool          DragFloat(const char* label, float* v, float v_speed = 1.0, float v_min = 0.0, float v_max = 0.0, const char* format = "%.3", ImGuiSliderFlags flags = 0);     // If v_min >= v_max we have no bound
//      bool          DragFloat2(const char* label, float v[2], float v_speed = 1.0, float v_min = 0.0, float v_max = 0.0, const char* format = "%.3", ImGuiSliderFlags flags = 0);
//      bool          DragFloat3(const char* label, float v[3], float v_speed = 1.0, float v_min = 0.0, float v_max = 0.0, const char* format = "%.3", ImGuiSliderFlags flags = 0);
//      bool          DragFloat4(const char* label, float v[4], float v_speed = 1.0, float v_min = 0.0, float v_max = 0.0, const char* format = "%.3", ImGuiSliderFlags flags = 0);
//      bool          DragFloatRange2(const char* label, float* v_current_min, float* v_current_max, float v_speed = 1.0, float v_min = 0.0, float v_max = 0.0, const char* format = "%.3", const char* format_max = NULL, ImGuiSliderFlags flags = 0);
//      bool          DragInt(const char* label, int* v, float v_speed = 1.0, int v_min = 0, int v_max = 0, const char* format = "%d", ImGuiSliderFlags flags = 0);  // If v_min >= v_max we have no bound
//      bool          DragInt2(const char* label, int v[2], float v_speed = 1.0, int v_min = 0, int v_max = 0, const char* format = "%d", ImGuiSliderFlags flags = 0);
//      bool          DragInt3(const char* label, int v[3], float v_speed = 1.0, int v_min = 0, int v_max = 0, const char* format = "%d", ImGuiSliderFlags flags = 0);
//      bool          DragInt4(const char* label, int v[4], float v_speed = 1.0, int v_min = 0, int v_max = 0, const char* format = "%d", ImGuiSliderFlags flags = 0);
//      bool          DragIntRange2(const char* label, int* v_current_min, int* v_current_max, float v_speed = 1.0, int v_min = 0, int v_max = 0, const char* format = "%d", const char* format_max = NULL, ImGuiSliderFlags flags = 0);
//      bool          DragScalar(const char* label, ImGuiDataType data_type, void* p_data, float v_speed = 1.0, const void* p_min = NULL, const void* p_max = NULL, const char* format = NULL, ImGuiSliderFlags flags = 0);
//      bool          DragScalarN(const char* label, ImGuiDataType data_type, void* p_data, int components, float v_speed = 1.0, const void* p_min = NULL, const void* p_max = NULL, const char* format = NULL, ImGuiSliderFlags flags = 0);
//
//     // Widgets: Regular Sliders
//     // - CTRL+Click on any slider to turn them into an input box. Manually input values aren't clamped by default and can go off-bounds. Use ImGuiSliderFlags_AlwaysClamp to always clamp.
//     // - Adjust format string to decorate the value with a prefix, a suffix, or adapt the editing and display precision e.g. "%.3" -> 1.234; "%5.2 secs" -> 01.23 secs; "Biscuit: %.0" -> Biscuit: 1; etc.
//     // - Format string may also be set to NULL or use the default format ("%f" or "%d").
//     // - Legacy: Pre-1.78 there are SliderXXX() function signatures that takes a final `float power=1.0' argument instead of the `ImGuiSliderFlags flags=0' argument.
//     //   If you get a warning converting a float to ImGuiSliderFlags, read https://github.com/ocornut/imgui/issues/3361
//      bool          SliderFloat(const char* label, float* v, float v_min, float v_max, const char* format = "%.3", ImGuiSliderFlags flags = 0);     // adjust format to decorate the value with a prefix or a suffix for in-slider labels or unit display.
//      bool          SliderFloat2(const char* label, float v[2], float v_min, float v_max, const char* format = "%.3", ImGuiSliderFlags flags = 0);
//      bool          SliderFloat3(const char* label, float v[3], float v_min, float v_max, const char* format = "%.3", ImGuiSliderFlags flags = 0);
//      bool          SliderFloat4(const char* label, float v[4], float v_min, float v_max, const char* format = "%.3", ImGuiSliderFlags flags = 0);
//      bool          SliderAngle(const char* label, float* v_rad, float v_degrees_min = -360.0, float v_degrees_max = +360.0, const char* format = "%.0 deg", ImGuiSliderFlags flags = 0);
//      bool          SliderInt(const char* label, int* v, int v_min, int v_max, const char* format = "%d", ImGuiSliderFlags flags = 0);
//      bool          SliderInt2(const char* label, int v[2], int v_min, int v_max, const char* format = "%d", ImGuiSliderFlags flags = 0);
//      bool          SliderInt3(const char* label, int v[3], int v_min, int v_max, const char* format = "%d", ImGuiSliderFlags flags = 0);
//      bool          SliderInt4(const char* label, int v[4], int v_min, int v_max, const char* format = "%d", ImGuiSliderFlags flags = 0);
//      bool          SliderScalar(const char* label, ImGuiDataType data_type, void* p_data, const void* p_min, const void* p_max, const char* format = NULL, ImGuiSliderFlags flags = 0);
//      bool          SliderScalarN(const char* label, ImGuiDataType data_type, void* p_data, int components, const void* p_min, const void* p_max, const char* format = NULL, ImGuiSliderFlags flags = 0);
//      bool          VSliderFloat(const char* label, const ImVec2& size, float* v, float v_min, float v_max, const char* format = "%.3", ImGuiSliderFlags flags = 0);
//      bool          VSliderInt(const char* label, const ImVec2& size, int* v, int v_min, int v_max, const char* format = "%d", ImGuiSliderFlags flags = 0);
//      bool          VSliderScalar(const char* label, const ImVec2& size, ImGuiDataType data_type, void* p_data, const void* p_min, const void* p_max, const char* format = NULL, ImGuiSliderFlags flags = 0);
//
//     // Widgets: Input with Keyboard
//     // - If you want to use InputText() with std::string or any custom dynamic string type, see misc/cpp/imgui_stdlib.h and comments in imgui_demo.cpp.
//     // - Most of the ImGuiInputTextFlags flags are only useful for InputText() and not for InputFloatX, InputIntX, InputDouble etc.
//      bool          InputText(const char* label, char* buf, size_t buf_size, ImGuiInputTextFlags flags = 0, ImGuiInputTextCallback callback = NULL, void* user_data = NULL);
//      bool          InputTextMultiline(const char* label, char* buf, size_t buf_size, const ImVec2& size = ImVec2(0, 0), ImGuiInputTextFlags flags = 0, ImGuiInputTextCallback callback = NULL, void* user_data = NULL);
//      bool          InputTextWithHint(const char* label, const char* hint, char* buf, size_t buf_size, ImGuiInputTextFlags flags = 0, ImGuiInputTextCallback callback = NULL, void* user_data = NULL);
//      bool          InputFloat(const char* label, float* v, float step = 0.0, float step_fast = 0.0, const char* format = "%.3", ImGuiInputTextFlags flags = 0);
//      bool          InputFloat2(const char* label, float v[2], const char* format = "%.3", ImGuiInputTextFlags flags = 0);
//      bool          InputFloat3(const char* label, float v[3], const char* format = "%.3", ImGuiInputTextFlags flags = 0);
//      bool          InputFloat4(const char* label, float v[4], const char* format = "%.3", ImGuiInputTextFlags flags = 0);
//      bool          InputInt(const char* label, int* v, int step = 1, int step_fast = 100, ImGuiInputTextFlags flags = 0);
//      bool          InputInt2(const char* label, int v[2], ImGuiInputTextFlags flags = 0);
//      bool          InputInt3(const char* label, int v[3], ImGuiInputTextFlags flags = 0);
//      bool          InputInt4(const char* label, int v[4], ImGuiInputTextFlags flags = 0);
//      bool          InputDouble(const char* label, double* v, double step = 0.0, double step_fast = 0.0, const char* format = "%.6", ImGuiInputTextFlags flags = 0);
//      bool          InputScalar(const char* label, ImGuiDataType data_type, void* p_data, const void* p_step = NULL, const void* p_step_fast = NULL, const char* format = NULL, ImGuiInputTextFlags flags = 0);
//      bool          InputScalarN(const char* label, ImGuiDataType data_type, void* p_data, int components, const void* p_step = NULL, const void* p_step_fast = NULL, const char* format = NULL, ImGuiInputTextFlags flags = 0);
//
//     // Widgets: Color Editor/Picker (tip: the ColorEdit* functions have a little color square that can be left-clicked to open a picker, and right-clicked to open an option menu.)
//     // - Note that in C++ a 'float v[x]' function argument is the _same_ as 'float* v', the array syntax is just a way to document the number of elements that are expected to be accessible.
//     // - You can pass the address of a first float element out of a contiguous structure, e.g. &myvector.x
//      bool          ColorEdit3(const char* label, float col[3], ImGuiColorEditFlags flags = 0);
//      bool          ColorEdit4(const char* label, float col[4], ImGuiColorEditFlags flags = 0);
//      bool          ColorPicker3(const char* label, float col[3], ImGuiColorEditFlags flags = 0);
//      bool          ColorPicker4(const char* label, float col[4], ImGuiColorEditFlags flags = 0, const float* ref_col = NULL);
//      bool          ColorButton(const char* desc_id, const ImVec4& col, ImGuiColorEditFlags flags = 0, const ImVec2& size = ImVec2(0, 0)); // display a color square/button, hover for details, return true when pressed.
//      void          SetColorEditOptions(ImGuiColorEditFlags flags);                     // initialize current options (generally on application startup) if you want to select a default format, picker type, etc. User will be able to change many settings, unless you pass the _NoOptions flag to your calls.
//
//     // Widgets: Trees
//     // - TreeNode functions return true when the node is open, in which case you need to also call TreePop() when you are finished displaying the tree node contents.
//      bool          TreeNode(const char* label);
//      bool          TreeNode(const char* str_id, const char* fmt, ...) IM_FMTARGS(2);   // helper variation to easily decorelate the id from the displayed string. Read the FAQ about why and how to use id. to align arbitrary text at the same level as a TreeNode() you can use Bullet().
//      bool          TreeNode(const void* ptr_id, const char* fmt, ...) IM_FMTARGS(2);   // "
//      bool          TreeNodeV(const char* str_id, const char* fmt, va_list args) IM_FMTLIST(2);
//      bool          TreeNodeV(const void* ptr_id, const char* fmt, va_list args) IM_FMTLIST(2);
//      bool          TreeNodeEx(const char* label, ImGuiTreeNodeFlags flags = 0);
//      bool          TreeNodeEx(const char* str_id, ImGuiTreeNodeFlags flags, const char* fmt, ...) IM_FMTARGS(3);
//      bool          TreeNodeEx(const void* ptr_id, ImGuiTreeNodeFlags flags, const char* fmt, ...) IM_FMTARGS(3);
//      bool          TreeNodeExV(const char* str_id, ImGuiTreeNodeFlags flags, const char* fmt, va_list args) IM_FMTLIST(3);
//      bool          TreeNodeExV(const void* ptr_id, ImGuiTreeNodeFlags flags, const char* fmt, va_list args) IM_FMTLIST(3);
//      void          TreePush(const char* str_id);                                       // ~ Indent()+PushId(). Already called by TreeNode() when returning true, but you can call TreePush/TreePop yourself if desired.
//      void          TreePush(const void* ptr_id = NULL);                                // "
//      void          TreePop();                                                          // ~ Unindent()+PopId()
//      float         GetTreeNodeToLabelSpacing();                                        // horizontal distance preceding label when using TreeNode*() or Bullet() == (g.font_size + style.FramePadding.x*2) for a regular unframed TreeNode
//      bool          CollapsingHeader(const char* label, ImGuiTreeNodeFlags flags = 0);  // if returning 'true' the header is open. doesn't indent nor push on id stack. user doesn't have to call TreePop().
//      bool          CollapsingHeader(const char* label, bool* p_visible, ImGuiTreeNodeFlags flags = 0); // when 'p_visible != NULL': if '*p_visible==true' display an additional small close button on upper right of the header which will set the bool to false when clicked, if '*p_visible==false' don't display the header.
//      void          SetNextItemOpen(bool is_open, ImGuiCond cond = 0);                  // set next TreeNode/CollapsingHeader open state.
//
//     // Widgets: Selectables
//     // - A selectable highlights when hovered, and can display another color when selected.
//     // - Neighbors selectable extend their highlight bounds in order to leave no gap between them. This is so a series of selected Selectable appear contiguous.
//      bool          Selectable(const char* label, bool selected = false, ImGuiSelectableFlags flags = 0, const ImVec2& size = ImVec2(0, 0)); // "bool selected" carry the selection state (read-only). Selectable() is clicked is returns true so you can modify your selection state. size.x==0.0: use remaining width, size.x>0.0: specify width. size.y==0.0: use label height, size.y>0.0: specify height
//      bool          Selectable(const char* label, bool* p_selected, ImGuiSelectableFlags flags = 0, const ImVec2& size = ImVec2(0, 0));      // "bool* p_selected" point to the selection state (read-write), as a convenient helper.
//
//     // Widgets: List Boxes
//     // - This is essentially a thin wrapper to using BeginChild/EndChild with some stylistic changes.
//     // - The BeginListBox()/EndListBox() api allows you to manage your contents and selection state however you want it, by creating e.g. Selectable() or any items.
//     // - The simplified/old ListBox() api are helpers over BeginListBox()/EndListBox() which are kept available for convenience purpose. This is analoguous to how Combos are created.
//     // - Choose frame width:   size.x > 0.0: custom  /  size.x < 0.0 or -FLT_MIN: right-align   /  size.x = 0.0 (default): use current ItemWidth
//     // - Choose frame height:  size.y > 0.0: custom  /  size.y < 0.0 or -FLT_MIN: bottom-align  /  size.y = 0.0 (default): arbitrary default height which can fit ~7 items
//      bool          BeginListBox(const char* label, const ImVec2& size = ImVec2(0, 0)); // open a framed scrolling region
//      void          EndListBox();                                                       // only call EndListBox() if BeginListBox() returned true!
//      bool          ListBox(const char* label, int* current_item, const char* const items[], int items_count, int height_in_items = -1);
//      bool          ListBox(const char* label, int* current_item, bool (*items_getter)(void* data, int idx, const char** out_text), void* data, int items_count, int height_in_items = -1);
//
//     // Widgets: data Plotting
//     // - Consider using ImPlot (https://github.com/epezent/implot) which is much better!
//      void          PlotLines(const char* label, const float* values, int values_count, int values_offset = 0, const char* overlay_text = NULL, float scale_min = FLT_MAX, float scale_max = FLT_MAX, ImVec2 graph_size = ImVec2(0, 0), int stride = sizeof(float));
//      void          PlotLines(const char* label, float(*values_getter)(void* data, int idx), void* data, int values_count, int values_offset = 0, const char* overlay_text = NULL, float scale_min = FLT_MAX, float scale_max = FLT_MAX, ImVec2 graph_size = ImVec2(0, 0));
//      void          PlotHistogram(const char* label, const float* values, int values_count, int values_offset = 0, const char* overlay_text = NULL, float scale_min = FLT_MAX, float scale_max = FLT_MAX, ImVec2 graph_size = ImVec2(0, 0), int stride = sizeof(float));
//      void          PlotHistogram(const char* label, float(*values_getter)(void* data, int idx), void* data, int values_count, int values_offset = 0, const char* overlay_text = NULL, float scale_min = FLT_MAX, float scale_max = FLT_MAX, ImVec2 graph_size = ImVec2(0, 0));
//
//     // Widgets: Value() Helpers.
//     // - Those are merely shortcut to calling Text() with a format string. Output single value in "name: value" format (tip: freely declare more in your code to handle your types. you can add functions to the ImGui namespace)
//      void          Value(const char* prefix, bool b);
//      void          Value(const char* prefix, int v);
//      void          Value(const char* prefix, unsigned int v);
//      void          Value(const char* prefix, float v, const char* float_format = NULL);
//
//     // Widgets: Menus
//     // - Use BeginMenuBar() on a window ImGuiWindowFlags_MenuBar to append to its menu bar.
//     // - Use BeginMainMenuBar() to create a menu bar at the top of the screen and append to it.
//     // - Use BeginMenu() to create a menu. You can call BeginMenu() multiple time with the same identifier to append more items to it.
//     // - Not that MenuItem() keyboardshortcuts are displayed as a convenience but _not processed_ by Dear ImGui at the moment.
//      bool          BeginMenuBar();                                                     // append to menu-bar of current window (requires ImGuiWindowFlags_MenuBar flag set on parent window).
//      void          EndMenuBar();                                                       // only call EndMenuBar() if BeginMenuBar() returns true!
//      bool          BeginMainMenuBar();                                                 // create and append to a full screen menu-bar.
//      void          EndMainMenuBar();                                                   // only call EndMainMenuBar() if BeginMainMenuBar() returns true!
//      bool          BeginMenu(const char* label, bool enabled = true);                  // create a sub-menu entry. only call EndMenu() if this returns true!
//      void          EndMenu();                                                          // only call EndMenu() if BeginMenu() returns true!
//      bool          MenuItem(const char* label, const char* shortcut = NULL, bool selected = false, bool enabled = true);  // return true when activated.
//      bool          MenuItem(const char* label, const char* shortcut, bool* p_selected, bool enabled = true);              // return true when activated + toggle (*p_selected) if p_selected != NULL
//
//     // Tooltips
//     // - Tooltip are windows following the mouse. They do not take focus away.
//      void          BeginTooltip();                                                     // begin/append a tooltip window. to create full-featured tooltip (with any kind of items).
//      void          EndTooltip();
//      void          SetTooltip(const char* fmt, ...) IM_FMTARGS(1);                     // set a text-only tooltip, typically use with ImGui::IsItemHovered(). override any previous call to SetTooltip().
//      void          SetTooltipV(const char* fmt, va_list args) IM_FMTLIST(1);
//
//     // Popups, Modals
//     //  - They block normal mouse hovering detection (and therefore most mouse interactions) behind them.
//     //  - If not modal: they can be closed by clicking anywhere outside them, or by pressing ESCAPE.
//     //  - Their visibility state (~bool) is held internally instead of being held by the programmer as we are used to with regular Begin*() calls.
//     //  - The 3 properties above are related: we need to retain popup visibility state in the library because popups may be closed as any time.
//     //  - You can bypass the hovering restriction by using ImGuiHoveredFlags_AllowWhenBlockedByPopup when calling IsItemHovered() or IsWindowHovered().
//     //  - IMPORTANT: Popup identifiers are relative to the current id stack, so OpenPopup and BeginPopup generally needs to be at the same level of the stack.
//     //    This is sometimes leading to confusing mistakes. May rework this in the future.
//
//     // Popups: begin/end functions
//     //  - BeginPopup(): query popup state, if open start appending into the window. Call EndPopup() afterwards. ImGuiWindowFlags are forwarded to the window.
//     //  - BeginPopupModal(): block every interactions behind the window, cannot be closed by user, add a dimming background, has a title bar.
//      bool          BeginPopup(const char* str_id, ImGuiWindowFlags flags = 0);                         // return true if the popup is open, and you can start outputting to it.
//      bool          BeginPopupModal(const char* name, bool* p_open = NULL, ImGuiWindowFlags flags = 0); // return true if the modal is open, and you can start outputting to it.
//      void          EndPopup();                                                                         // only call EndPopup() if BeginPopupXXX() returns true!
//
//     // Popups: open/close functions
//     //  - OpenPopup(): set popup state to open. ImGuiPopupFlags are available for opening options.
//     //  - If not modal: they can be closed by clicking anywhere outside them, or by pressing ESCAPE.
//     //  - CloseCurrentPopup(): use inside the BeginPopup()/EndPopup() scope to close manually.
//     //  - CloseCurrentPopup() is called by default by Selectable()/MenuItem() when activated (FIXME: need some options).
//     //  - Use ImGuiPopupFlags_NoOpenOverExistingPopup to avoid opening a popup if there's already one at the same level. This is equivalent to e.g. testing for !IsAnyPopupOpen() prior to OpenPopup().
//     //  - Use IsWindowAppearing() after BeginPopup() to tell if a window just opened.
//     //  - IMPORTANT: Notice that for OpenPopupOnItemClick() we exceptionally default flags to 1 (== ImGuiPopupFlags_MouseButtonRight) for backward compatibility with older API taking 'int mouse_button = 1' parameter
//      void          OpenPopup(const char* str_id, ImGuiPopupFlags popup_flags = 0);                     // call to mark popup as open (don't call every frame!).
//      void          OpenPopup(ImGuiID id, ImGuiPopupFlags popup_flags = 0);                             // id overload to facilitate calling from nested stacks
//      void          OpenPopupOnItemClick(const char* str_id = NULL, ImGuiPopupFlags popup_flags = 1);   // helper to open popup when clicked on last item. Default to ImGuiPopupFlags_MouseButtonRight == 1. (note: actually triggers on the mouse _released_ event to be consistent with popup behaviors)
//      void          CloseCurrentPopup();                                                                // manually close the popup we have begin-ed into.
//
//     // Popups: open+begin combined functions helpers
//     //  - Helpers to do OpenPopup+BeginPopup where the Open action is triggered by e.g. hovering an item and right-clicking.
//     //  - They are convenient to easily create context menus, hence the name.
//     //  - IMPORTANT: Notice that BeginPopupContextXXX takes ImGuiPopupFlags just like OpenPopup() and unlike BeginPopup(). For full consistency, we may add ImGuiWindowFlags to the BeginPopupContextXXX functions in the future.
//     //  - IMPORTANT: Notice that we exceptionally default their flags to 1 (== ImGuiPopupFlags_MouseButtonRight) for backward compatibility with older API taking 'int mouse_button = 1' parameter, so if you add other flags remember to re-add the ImGuiPopupFlags_MouseButtonRight.
//      bool          BeginPopupContextItem(const char* str_id = NULL, ImGuiPopupFlags popup_flags = 1);  // open+begin popup when clicked on last item. Use str_id==NULL to associate the popup to previous item. If you want to use that on a non-interactive item such as Text() you need to pass in an explicit id here. read comments in .cpp!
//      bool          BeginPopupContextWindow(const char* str_id = NULL, ImGuiPopupFlags popup_flags = 1);// open+begin popup when clicked on current window.
//      bool          BeginPopupContextVoid(const char* str_id = NULL, ImGuiPopupFlags popup_flags = 1);  // open+begin popup when clicked in void (where there are no windows).
//
//     // Popups: query functions
//     //  - IsPopupOpen(): return true if the popup is open at the current BeginPopup() level of the popup stack.
//     //  - IsPopupOpen() with ImGuiPopupFlags_AnyPopupId: return true if any popup is open at the current BeginPopup() level of the popup stack.
//     //  - IsPopupOpen() with ImGuiPopupFlags_AnyPopupId + ImGuiPopupFlags_AnyPopupLevel: return true if any popup is open.
//      bool          IsPopupOpen(const char* str_id, ImGuiPopupFlags flags = 0);                         // return true if the popup is open.
//
//     // tables
//     // - Full-featured replacement for old Columns API.
//     // - See Demo->tables for demo code. See top of imgui_tables.cpp for general commentary.
//     // - See ImGuiTableFlags_ and ImGuiTableColumnFlags_ enums for a description of available flags.
//     // The typical call flow is:
//     // - 1. Call BeginTable(), early out if returning false.
//     // - 2. Optionally call TableSetupColumn() to submit column name/flags/defaults.
//     // - 3. Optionally call TableSetupScrollFreeze() to request scroll freezing of columns/rows.
//     // - 4. Optionally call TableHeadersRow() to submit a header row. Names are pulled from TableSetupColumn() data.
//     // - 5. Populate contents:
//     //    - In most situations you can use TableNextRow() + TableSetColumnIndex(N) to start appending into a column.
//     //    - If you are using tables as a sort of grid, where every columns is holding the same type of contents,
//     //      you may prefer using TableNextColumn() instead of TableNextRow() + TableSetColumnIndex().
//     //      TableNextColumn() will automatically wrap-around into the next row if needed.
//     //    - IMPORTANT: Comparatively to the old Columns() API, we need to call TableNextColumn() for the first column!
//     //    - Summary of possible call flow:
//     //        --------------------------------------------------------------------------------------------------------
//     //        TableNextRow() -> TableSetColumnIndex(0) -> Text("Hello 0") -> TableSetColumnIndex(1) -> Text("Hello 1")  // OK
//     //        TableNextRow() -> TableNextColumn()      -> Text("Hello 0") -> TableNextColumn()      -> Text("Hello 1")  // OK
//     //                          TableNextColumn()      -> Text("Hello 0") -> TableNextColumn()      -> Text("Hello 1")  // OK: TableNextColumn() automatically gets to next row!
//     //        TableNextRow()                           -> Text("Hello 0")                                               // Not OK! Missing TableSetColumnIndex() or TableNextColumn()! Text will not appear!
//     //        --------------------------------------------------------------------------------------------------------
//     // - 5. Call EndTable()
//      bool          BeginTable(const char* str_id, int column, ImGuiTableFlags flags = 0, const ImVec2& outer_size = ImVec2(0.0, 0.0), float inner_width = 0.0);
//      void          EndTable();                                         // only call EndTable() if BeginTable() returns true!
//      void          TableNextRow(ImGuiTableRowFlags row_flags = 0, float min_row_height = 0.0); // append into the first cell of a new row.
//      bool          TableNextColumn();                                  // append into the next column (or first column of next row if currently in last column). Return true when column is visible.
//      bool          TableSetColumnIndex(int column_n);                  // append into the specified column. Return true when column is visible.
//
//     // tables: Headers & Columns declaration
//     // - Use TableSetupColumn() to specify label, resizing policy, default width/weight, id, various other flags etc.
//     // - Use TableHeadersRow() to create a header row and automatically submit a TableHeader() for each column.
//     //   Headers are required to perform: reordering, sorting, and opening the context menu.
//     //   The context menu can also be made available in columns body using ImGuiTableFlags_ContextMenuInBody.
//     // - You may manually submit headers using TableNextRow() + TableHeader() calls, but this is only useful in
//     //   some advanced use cases (e.g. adding custom widgets in header row).
//     // - Use TableSetupScrollFreeze() to lock columns/rows so they stay visible when scrolled.
//      void          TableSetupColumn(const char* label, ImGuiTableColumnFlags flags = 0, float init_width_or_weight = 0.0, ImGuiID user_id = 0);
//      void          TableSetupScrollFreeze(int cols, int rows);         // lock columns/rows so they stay visible when scrolled.
//      void          TableHeadersRow();                                  // submit all headers cells based on data provided to TableSetupColumn() + submit context menu
//      void          TableHeader(const char* label);                     // submit one header cell manually (rarely used)
//
//     // tables: Sorting & Miscellaneous functions
//     // - Sorting: call TableGetSortSpecs() to retrieve latest sort specs for the table. NULL when not sorting.
//     //   When 'sort_specs->specs_dirty == true' you should sort your data. It will be true when sorting specs have
//     //   changed since last call, or the first time. Make sure to set 'specs_dirty = false' after sorting,
//     //   else you may wastefully sort your data every frame!
//     // - Functions args 'int column_n' treat the default value of -1 as the same as passing the current column index.
//      ImGuiTableSortSpecs*  TableGetSortSpecs();                        // get latest sort specs for the table (NULL if not sorting).  Lifetime: don't hold on this pointer over multiple frames or past any subsequent call to BeginTable().
//      int                   TableGetColumnCount();                      // return number of columns (value passed to BeginTable)
//      int                   TableGetColumnIndex();                      // return current column index.
//      int                   TableGetRowIndex();                         // return current row index.
//      const char*           TableGetColumnName(int column_n = -1);      // return "" if column didn't have a name declared by TableSetupColumn(). Pass -1 to use current column.
//      ImGuiTableColumnFlags TableGetColumnFlags(int column_n = -1);     // return column flags so you can query their Enabled/visible/Sorted/Hovered status flags. Pass -1 to use current column.
//      void                  TableSetColumnEnabled(int column_n, bool v);// change user accessible enabled/disabled state of a column. Set to false to hide the column. User can use the context menu to change this themselves (right-click in headers, or right-click in columns body with ImGuiTableFlags_ContextMenuInBody)
//      void                  TableSetBgColor(ImGuiTableBgTarget target, ImU32 color, int column_n = -1);  // change the color of a cell, row, or column. See ImGuiTableBgTarget_ flags for details.
//
//     // Legacy Columns API (prefer using tables!)
//     // - You can also use SameLine(pos_x) to mimic simplified columns.
//      void          Columns(int count = 1, const char* id = NULL, bool border = true);
//      void          NextColumn();                                                       // next column, defaults to current row or next row if the current row is finished
//      int           GetColumnIndex();                                                   // get current column index
//      float         GetColumnWidth(int column_index = -1);                              // get column width (in pixels). pass -1 to use current column
//      void          SetColumnWidth(int column_index, float width);                      // set column width (in pixels). pass -1 to use current column
//      float         GetColumnOffset(int column_index = -1);                             // get position of column line (in pixels, from the left side of the contents region). pass -1 to use current column, otherwise 0..GetColumnsCount() inclusive. column 0 is typically 0.0
//      void          SetColumnOffset(int column_index, float offset_x);                  // set position of column line (in pixels, from the left side of the contents region). pass -1 to use current column
//      int           GetColumnsCount();
//
//     // Tab Bars, Tabs
//     // Note: Tabs are automatically created by the docking system. Use this to create tab bars/tabs yourself without docking being involved.
//      bool          BeginTabBar(const char* str_id, ImGuiTabBarFlags flags = 0);        // create and append into a tab_bar
//      void          EndTabBar();                                                        // only call EndTabBar() if BeginTabBar() returns true!
//      bool          BeginTabItem(const char* label, bool* p_open = NULL, ImGuiTabItemFlags flags = 0); // create a Tab. Returns true if the Tab is selected.
//      void          EndTabItem();                                                       // only call EndTabItem() if BeginTabItem() returns true!
//      bool          TabItemButton(const char* label, ImGuiTabItemFlags flags = 0);      // create a Tab behaving like a button. return true when clicked. cannot be selected in the tab bar.
//      void          SetTabItemClosed(const char* tab_or_docked_window_label);           // notify tab_bar or Docking system of a closed tab/window ahead (useful to reduce visual flicker on reorderable tab bars). For tab-bar: call after BeginTabBar() and before Tab submissions. Otherwise call with a window name.
//
//     // Docking
//     // [BETA API] Enable with io.config_flags |= ImGuiConfigFlags_DockingEnable.
//     // Note: You can use most Docking facilities without calling any API. You DO NOT need to call DockSpace() to use Docking!
//     // - Drag from window title bar or their tab to dock/undock. Hold SHIFT to disable docking/undocking.
//     // - Drag from window menu button (upper-left button) to undock an entire node (all windows).
//     // - When io.config_docking_with_shift == true, you instead need to hold SHIFT to _enable_ docking/undocking.
//     // About dockspaces:
//     // - Use DockSpace() to create an explicit dock node _within_ an existing window. See Docking demo for details.
//     // - Use DockSpaceOverViewport() to create an explicit dock node covering the screen or a specific viewport.
//     //   This is often used with ImGuiDockNodeFlags_PassthruCentralNode.
//     // - Important: Dockspaces need to be submitted _before_ any window they can host. Submit it early in your frame!
//     // - Important: Dockspaces need to be kept alive if hidden, otherwise windows docked into it will be undocked.
//     //   e.g. if you have multiple tabs with a dockspace inside each tab: submit the non-visible dockspaces with ImGuiDockNodeFlags_KeepAliveOnly.
//      ImGuiID       DockSpace(ImGuiID id, const ImVec2& size = ImVec2(0, 0), ImGuiDockNodeFlags flags = 0, const ImGuiWindowClass* window_class = NULL);
//      ImGuiID       DockSpaceOverViewport(const ImGuiViewport* viewport = NULL, ImGuiDockNodeFlags flags = 0, const ImGuiWindowClass* window_class = NULL);
//      void          SetNextWindowDockID(ImGuiID dock_id, ImGuiCond cond = 0);           // set next window dock id
//      void          SetNextWindowClass(const ImGuiWindowClass* window_class);           // set next window class (control docking compatibility + provide hints to platform backend via custom viewport flags and platform parent/child relationship)
//      ImGuiID       GetWindowDockID();
//      bool          IsWindowDocked();                                                   // is current window docked into another window?
//
//     // Logging/Capture
//     // - All text output from the interface can be captured into tty/file/clipboard. By default, tree nodes are automatically opened during logging.
//      void          LogToTTY(int auto_open_depth = -1);                                 // start logging to tty (stdout)
//      void          LogToFile(int auto_open_depth = -1, const char* filename = NULL);   // start logging to file
//      void          LogToClipboard(int auto_open_depth = -1);                           // start logging to OS clipboard
//      void          LogFinish();                                                        // stop logging (close file, etc.)
//      void          LogButtons();                                                       // helper to display buttons for logging to tty/file/clipboard
//      void          LogText(const char* fmt, ...) IM_FMTARGS(1);                        // pass text data straight to log (without being displayed)
//      void          LogTextV(const char* fmt, va_list args) IM_FMTLIST(1);
//
//     // Drag and Drop
//     // - On source items, call BeginDragDropSource(), if it returns true also call SetDragDropPayload() + EndDragDropSource().
//     // - On target candidates, call BeginDragDropTarget(), if it returns true also call AcceptDragDropPayload() + EndDragDropTarget().
//     // - If you stop calling BeginDragDropSource() the payload is preserved however it won't have a preview tooltip (we currently display a fallback "..." tooltip, see #1725)
//     // - An item can be both drag source and drop target.
//      bool          BeginDragDropSource(ImGuiDragDropFlags flags = 0);                                      // call after submitting an item which may be dragged. when this return true, you can call SetDragDropPayload() + EndDragDropSource()
//      bool          SetDragDropPayload(const char* type, const void* data, size_t sz, ImGuiCond cond = 0);  // type is a user defined string of maximum 32 characters. Strings starting with '_' are reserved for dear imgui internal types. data is copied and held by imgui. Return true when payload has been accepted.
//      void          EndDragDropSource();                                                                    // only call EndDragDropSource() if BeginDragDropSource() returns true!
//      bool                  BeginDragDropTarget();                                                          // call after submitting an item that may receive a payload. If this returns true, you can call AcceptDragDropPayload() + EndDragDropTarget()
//      const ImGuiPayload*   AcceptDragDropPayload(const char* type, ImGuiDragDropFlags flags = 0);          // accept contents of a given type. If ImGuiDragDropFlags_AcceptBeforeDelivery is set you can peek into the payload before the mouse button is released.
//      void                  EndDragDropTarget();                                                            // only call EndDragDropTarget() if BeginDragDropTarget() returns true!
//      const ImGuiPayload*   GetDragDropPayload();                                                           // peek directly into the current payload from anywhere. may return NULL. use ImGuiPayload::is_data_type() to test for the payload type.
//
//     // Disabling [BETA API]
//     // - Disable all user interactions and dim items visuals (applying style.DisabledAlpha over current colors)
//     // - Those can be nested but it cannot be used to enable an already disabled section (a single BeginDisabled(true) in the stack is enough to keep everything disabled)
//     // - BeginDisabled(false) essentially does nothing useful but is provided to facilitate use of boolean expressions. If you can avoid calling BeginDisabled(False)/EndDisabled() best to avoid it.
//      void          BeginDisabled(bool disabled = true);
//      void          EndDisabled();
//
//     // Clipping
//     // - Mouse hovering is affected by ImGui::PushClipRect() calls, unlike direct calls to ImDrawList::PushClipRect() which are render only.
//      void          PushClipRect(const ImVec2& clip_rect_min, const ImVec2& clip_rect_max, bool intersect_with_current_clip_rect);
//      void          PopClipRect();
//
//     // Focus, Activation
//     // - Prefer using "SetItemDefaultFocus()" over "if (IsWindowAppearing()) SetScrollHereY()" when applicable to signify "this is the default item"
//      void          SetItemDefaultFocus();                                              // make last item the default focused item of a window.
//      void          SetKeyboardFocusHere(int offset = 0);                               // focus keyboard on the next widget. Use positive 'offset' to access sub components of a multiple component widget. Use -1 to access previous widget.
//
//     // Item/Widgets Utilities and Query Functions
//     // - Most of the functions are referring to the previous Item that has been submitted.
//     // - See Demo Window under "Widgets->Querying Status" for an interactive visualization of most of those functions.
//      bool          IsItemHovered(ImGuiHoveredFlags flags = 0);                         // is the last item hovered? (and usable, aka not blocked by a popup, etc.). See ImGuiHoveredFlags for more options.
//      bool          IsItemActive();                                                     // is the last item active? (e.g. button being held, text field being edited. This will continuously return true while holding mouse button on an item. Items that don't interact will always return false)
//      bool          IsItemFocused();                                                    // is the last item focused for keyboard/gamepad navigation?
//      bool          IsItemClicked(ImGuiMouseButton mouse_button = 0);                   // is the last item hovered and mouse clicked on? (**)  == IsMouseClicked(mouse_button) && IsItemHovered()Important. (**) this it NOT equivalent to the behavior of e.g. Button(). Read comments in function definition.
//      bool          IsItemVisible();                                                    // is the last item visible? (items may be out of sight because of clipping/scrolling)
//      bool          IsItemEdited();                                                     // did the last item modify its underlying value this frame? or was pressed? This is generally the same as the "bool" return value of many widgets.
//      bool          IsItemActivated();                                                  // was the last item just made active (item was previously inactive).
//      bool          IsItemDeactivated();                                                // was the last item just made inactive (item was previously active). Useful for Undo/Redo patterns with widgets that requires continuous editing.
//      bool          IsItemDeactivatedAfterEdit();                                       // was the last item just made inactive and made a value change when it was active? (e.g. Slider/Drag moved). Useful for Undo/Redo patterns with widgets that requires continuous editing. Note that you may get false positives (some widgets such as Combo()/ListBox()/Selectable() will return true even when clicking an already selected item).
//      bool          IsItemToggledOpen();                                                // was the last item open state toggled? set by TreeNode().
//      bool          IsAnyItemHovered();                                                 // is any item hovered?
//      bool          IsAnyItemActive();                                                  // is any item active?
//      bool          IsAnyItemFocused();                                                 // is any item focused?
//      ImVec2        GetItemRectMin();                                                   // get upper-left bounding rectangle of the last item (screen space)
//      ImVec2        GetItemRectMax();                                                   // get lower-right bounding rectangle of the last item (screen space)
//      ImVec2        GetItemRectSize();                                                  // get size of last item
//      void          SetItemAllowOverlap();                                              // allow last item to be overlapped by a subsequent item. sometimes useful with invisible buttons, selectables, etc. to catch unused area.
//
//     // viewports
//     // - Currently represents the Platform Window created by the application which is hosting our Dear ImGui windows.
//     // - In 'docking' branch with multi-viewport enabled, we extend this concept to have multiple active viewports.
//     // - In the future we will extend this concept further to also represent Platform Monitor and support a "no main platform window" operation mode.
//      ImGuiViewport* GetMainViewport();                                                 // return primary/default viewport. This can never be NULL.
//
//     // Background/Foreground Draw Lists
//      ImDrawList*   GetBackgroundDrawList();                                            // get background draw list for the viewport associated to the current window. this draw list will be the first rendering one. Useful to quickly draw shapes/text behind dear imgui contents.
//      ImDrawList*   GetForegroundDrawList();                                            // get foreground draw list for the viewport associated to the current window. this draw list will be the last rendered one. Useful to quickly draw shapes/text over dear imgui contents.
//      ImDrawList*   GetBackgroundDrawList(ImGuiViewport* viewport);                     // get background draw list for the given viewport. this draw list will be the first rendering one. Useful to quickly draw shapes/text behind dear imgui contents.
//      ImDrawList*   GetForegroundDrawList(ImGuiViewport* viewport);                     // get foreground draw list for the given viewport. this draw list will be the last rendered one. Useful to quickly draw shapes/text over dear imgui contents.
//
//     // Miscellaneous Utilities
//      bool          IsRectVisible(const ImVec2& size);                                  // test if rectangle (of given size, starting from cursor position) is visible / not clipped.
//      bool          IsRectVisible(const ImVec2& rect_min, const ImVec2& rect_max);      // test if rectangle (in screen space) is visible / not clipped. to perform coarse clipping on user's side.
//      double        GetTime();                                                          // get global imgui time. incremented by io.delta_time every frame.
//      int           GetFrameCount();                                                    // get global imgui frame count. incremented by 1 every frame.
//      ImDrawListSharedData* GetDrawListSharedData();                                    // you may use this when creating your own ImDrawList instances.
//      const char*   GetStyleColorName(ImGuiCol idx);                                    // get a string corresponding to the enum value (for display, saving, etc.).
//      void          SetStateStorage(ImGuiStorage* storage);                             // replace current window storage with our own (if you want to manipulate it yourself, typically clear subsection of it)
//      ImGuiStorage* GetStateStorage();
//      bool          BeginChildFrame(ImGuiID id, const ImVec2& size, ImGuiWindowFlags flags = 0); // helper to create a child window / scrolling region that looks like a normal widget frame
//      void          EndChildFrame();                                                    // always call EndChildFrame() regardless of BeginChildFrame() return values (which indicates a collapsed/clipped window)
//
//     // Text Utilities
//      ImVec2        CalcTextSize(const char* text, const char* text_end = NULL, bool hide_text_after_double_hash = false, float wrap_width = -1.0);
//
//     // Color Utilities
//      ImVec4        ColorConvertU32ToFloat4(ImU32 in);
//      ImU32         ColorConvertFloat4ToU32(const ImVec4& in);
//      void          ColorConvertRGBtoHSV(float r, float g, float b, float& out_h, float& out_s, float& out_v);
//      void          ColorConvertHSVtoRGB(float h, float s, float v, float& out_r, float& out_g, float& out_b);
//
//     // Inputs Utilities: Keyboard
//     // Without IMGUI_DISABLE_OBSOLETE_KEYIO: (legacy support)
//     //   - For 'ImGuiKey key' you can still use your legacy native/user indices according to how your backend/engine stored them in io.KeysDown[].
//     // With IMGUI_DISABLE_OBSOLETE_KEYIO: (this is the way forward)
//     //   - Any use of 'ImGuiKey' will assert when key < 512 will be passed, previously reserved as native/user keys indices
//     //   - GetKeyIndex() is pass-through and therefore deprecated (gone if IMGUI_DISABLE_OBSOLETE_KEYIO is defined)
//      bool          IsKeyDown(ImGuiKey key);                                            // is key being held.
//      bool          IsKeyPressed(ImGuiKey key, bool repeat = true);                     // was key pressed (went from !down to down)? if repeat=true, uses io.key_repeat_delay / key_repeat_rate
//      bool          IsKeyReleased(ImGuiKey key);                                        // was key released (went from down to !down)?
//      int           GetKeyPressedAmount(ImGuiKey key, float repeat_delay, float rate);  // uses provided repeat rate/delay. return a count, most often 0 or 1 but might be >1 if RepeatRate is small enough that delta_time > RepeatRate
//      const char*   GetKeyName(ImGuiKey key);                                           // [DEBUG] returns English name of the key. Those names a provided for debugging purpose and are not meant to be saved persistently not compared.
//      void          SetNextFrameWantCaptureKeyboard(bool want_capture_keyboard);        // Override io.want_capture_keyboard flag next frame (said flag is left for your application to handle, typically when true it instructs your app to ignore inputs). e.g. force capture keyboard when your widget is being hovered. This is equivalent to setting "io.want_capture_keyboard = want_capture_keyboard"; after the next NewFrame() call.
//
//     // Inputs Utilities: Mouse
//     // - To refer to a mouse button, you may use named enums in your code e.g. ImGuiMouseButton_Left, ImGuiMouseButton_Right.
//     // - You can also use regular integer: it is forever guaranteed that 0=Left, 1=Right, 2=Middle.
//     // - Dragging operations are only reported after mouse has moved a certain distance away from the initial clicking position (see 'lock_threshold' and 'io.MouseDraggingThreshold')
//      bool          IsMouseDown(ImGuiMouseButton button);                               // is mouse button held?
//      bool          IsMouseClicked(ImGuiMouseButton button, bool repeat = false);       // did mouse button clicked? (went from !down to down). Same as GetMouseClickedCount() == 1.
//      bool          IsMouseReleased(ImGuiMouseButton button);                           // did mouse button released? (went from down to !down)
//      bool          IsMouseDoubleClicked(ImGuiMouseButton button);                      // did mouse button double-clicked? Same as GetMouseClickedCount() == 2. (note that a double-click will also report IsMouseClicked() == true)
//      int           GetMouseClickedCount(ImGuiMouseButton button);                      // return the number of successive mouse-clicks at the time where a click happen (otherwise 0).
//      bool          IsMouseHoveringRect(const ImVec2& r_min, const ImVec2& r_max, bool clip = true);// is mouse hovering given bounding rect (in screen space). clipped by current clipping settings, but disregarding of other consideration of focus/window ordering/popup-block.
//      bool          IsMousePosValid(const ImVec2* mouse_pos = NULL);                    // by convention we use (-FLT_MAX,-FLT_MAX) to denote that there is no mouse available
//      bool          IsAnyMouseDown();                                                   // [WILL OBSOLETE] is any mouse button held? This was designed for backends, but prefer having backend maintain a mask of held mouse buttons, because upcoming input queue system will make this invalid.
//      ImVec2        GetMousePos();                                                      // shortcut to ImGui::GetIO().mouse_pos provided by user, to be consistent with other calls
//      ImVec2        GetMousePosOnOpeningCurrentPopup();                                 // retrieve mouse position at the time of opening popup we have BeginPopup() into (helper to avoid user backing that value themselves)
//      bool          IsMouseDragging(ImGuiMouseButton button, float lock_threshold = -1.0);         // is mouse dragging? (if lock_threshold < -1.0, uses io.MouseDraggingThreshold)
//      ImVec2        GetMouseDragDelta(ImGuiMouseButton button = 0, float lock_threshold = -1.0);   // return the delta from the initial clicking position while the mouse button is pressed or was just released. This is locked and return 0.0 until the mouse moves past a distance threshold at least once (if lock_threshold < -1.0, uses io.MouseDraggingThreshold)
//      void          ResetMouseDragDelta(ImGuiMouseButton button = 0);                   //
//      ImGuiMouseCursor GetMouseCursor();                                                // get desired cursor type, reset in ImGui::NewFrame(), this is updated during the frame. valid before Render(). If you use software rendering by setting io.mouse_draw_cursor ImGui will render those for you
//      void          SetMouseCursor(ImGuiMouseCursor cursor_type);                       // set desired cursor type
//      void          SetNextFrameWantCaptureMouse(bool want_capture_mouse);              // Override io.want_capture_mouse flag next frame (said flag is left for your application to handle, typical when true it instucts your app to ignore inputs). This is equivalent to setting "io.want_capture_mouse = want_capture_mouse;" after the next NewFrame() call.
//
//     // Clipboard Utilities
//     // - Also see the LogToClipboard() function to capture GUI into clipboard, or easily output text data to the clipboard.
//      const char*   GetClipboardText();
//      void          SetClipboardText(const char* text);
//
//     // Settings/.Ini Utilities
//     // - The disk functions are automatically called if io.ini_filename != NULL (default is "imgui.ini").
//     // - Set io.ini_filename to NULL to load/save manually. Read io.want_save_ini_settings description about handling .ini saving manually.
//     // - Important: default value "imgui.ini" is relative to current working dir! Most apps will want to lock this to an absolute path (e.g. same path as executables).
//      void          LoadIniSettingsFromDisk(const char* ini_filename);                  // call after CreateContext() and before the first call to NewFrame(). NewFrame() automatically calls LoadIniSettingsFromDisk(io.ini_filename).
//      void          LoadIniSettingsFromMemory(const char* ini_data, size_t ini_size=0); // call after CreateContext() and before the first call to NewFrame() to provide .ini data from your own data source.
//      void          SaveIniSettingsToDisk(const char* ini_filename);                    // this is automatically called (if io.ini_filename is not empty) a few seconds after any modification that should be reflected in the .ini file (and also by DestroyContext).
//      const char*   SaveIniSettingsToMemory(size_t* out_ini_size = NULL);               // return a zero-terminated string with the .ini data which you can save by your own mean. call when io.want_save_ini_settings is set, then save data by your own mean and clear io.want_save_ini_settings.
//
//     // Debug Utilities
//      void          DebugTextEncoding(const char* text);
//      bool          DebugCheckVersionAndDataLayout(const char* version_str, size_t sz_io, size_t sz_style, size_t sz_vec2, size_t sz_vec4, size_t sz_drawvert, size_t sz_drawidx); // This is called by IMGUI_CHECKVERSION() macro.
//
//     // Memory Allocators
//     // - Those functions are not reliant on the current context.
//     // - DLL users: heaps and globals are not shared across DLL boundaries! You will need to call SetCurrentContext() + SetAllocatorFunctions()
//     //   for each static/DLL boundary you are calling from. Read "Context and Memory Allocators" section of imgui.cpp for more details.
//      void          SetAllocatorFunctions(ImGuiMemAllocFunc alloc_func, ImGuiMemFreeFunc free_func, void* user_data = NULL);
//      void          GetAllocatorFunctions(ImGuiMemAllocFunc* p_alloc_func, ImGuiMemFreeFunc* p_free_func, void** p_user_data);
//      void*         MemAlloc(size_t size);
//      void          MemFree(void* ptr);
//
//     // (Optional) Platform/OS interface for multi-viewport support
//     // Read comments around the ImGuiPlatformIO structure for more details.
//     // Note: You may use GetWindowViewport() to get the current viewport of the current window.
//      ImGuiPlatformIO&  GetPlatformIO();                                                // platform/renderer functions, for backend to setup + viewports list.
//      void              UpdatePlatformWindows();                                        // call in main loop. will call CreateWindow/ResizeWindow/etc. platform functions for each secondary viewport, and DestroyWindow for each inactive viewport.
//      void              RenderPlatformWindowsDefault(void* platform_render_arg = NULL, void* renderer_render_arg = NULL); // call in main loop. will call RenderWindow/SwapBuffers platform functions for each secondary viewport which doesn't have the Minimized flag set. May be reimplemented by user for custom rendering needs.
//      void              DestroyPlatformWindows();                                       // call DestroyWindow platform functions for all viewports. call from backend Shutdown() if you need to close platform windows before imgui shutdown. otherwise will be called by DestroyContext().
//      ImGuiViewport*    FindViewportByID(ImGuiID id);                                   // this is a helper for backends.
//      ImGuiViewport*    FindViewportByPlatformHandle(void* platform_handle);            // this is a helper for backends. the type platform_handle is decided by the backend (e.g. HWND, MyWindow*, GLFWwindow* etc.)
//
// } // namespace ImGui

//-----------------------------------------------------------------------------
// [SECTION] flags & Enumerations
//-----------------------------------------------------------------------------

// pub const StatusMask_      : i32     = DimgTableColumnFlags::IsEnabled | DimgTableColumnFlags::is_visible | DimgTableColumnFlags::IsSorted | DimgTableColumnFlags::IsHovered;


// pub const RootAndChildWindows: i32           = DimgFocusedFlags::RootWindow | DimgFocusedFlags::ChildWindows;
// float[4]: Standard type for colors. User code may use this type.

// A primary data type
// pub enum ImGuiDataType_
// {
//     ImGuiDataType_S8,       // signed char / char (with sensible compilers)
//     ImGuiDataType_U8,       // unsigned char
//     ImGuiDataType_S16,      // short
//     ImGuiDataType_U16,      // unsigned short
//     ImGuiDataType_S32,      // int
//     ImGuiDataType_U32,      // unsigned int
//     ImGuiDataType_S64,      // long long / __int64
//     ImGuiDataType_U64,      // unsigned long long / unsigned __int64
//     ImGuiDataType_Float,    // float
//     ImGuiDataType_Double,   // double
//     ImGuiDataType_COUNT
// };

// pub const     NamedKey_COUNT:          = DimgKey::NamedKey_END - DimgKey::NamedKey_BEGIN;

// pub const DataTypeMask_: i32   = DimgColorEditFlags::Uint8 | DimgColorEditFlags::Float;

//-----------------------------------------------------------------------------
// [SECTION] Helpers: Memory allocations macros, ImVector<>
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// IM_MALLOC(), IM_FREE(), IM_NEW(), IM_PLACEMENT_NEW(), IM_DELETE()
// We call C++ constructor on own allocated memory via the placement "new(ptr) Type()" syntax.
// Defining a custom placement new() with a custom parameter allows us to bypass including <new> which on some platforms complains when user has disabled exceptions.
//-----------------------------------------------------------------------------

// pub struct ImNewWrapper {};
// inline void* operator new(size_t, ImNewWrapper, void* ptr) { return ptr; }
// inline void  operator delete(void*, ImNewWrapper, void*)   {} // This is only required so we can use the symmetrical new()
// #define IM_ALLOC(_SIZE)                     ImGui::MemAlloc(_SIZE)
// #define IM_FREE(_PTR)                       ImGui::MemFree(_PTR)
// #define IM_PLACEMENT_NEW(_PTR)              new(ImNewWrapper(), _PTR)
// #define IM_NEW(_TYPE)                       new(ImNewWrapper(), ImGui::MemAlloc(sizeof(_TYPE))) _TYPE
// template<typename T> void IM_DELETE(T* p)   { if (p) { p->~T(); ImGui::MemFree(p); } }

//-----------------------------------------------------------------------------
// ImVector<>
// Lightweight std::vector<>-like class to avoid dragging dependencies (also, some implementations of STL with debug enabled are absurdly slow, we bypass it so our code runs fast in debug).
//-----------------------------------------------------------------------------
// - You generally do NOT need to care or use this ever. But we need to make it available in imgui.h because some of our public structures are relying on it.
// - We use std-like naming convention here, which is a little unusual for this codebase.
// - Important: clear() frees memory, resize(0) keep the allocated buffer. We use resize(0) a lot to intentionally recycle allocated buffers across frames and amortize our costs.
// - Important: our implementation does NOT call C++ constructors/destructors, we treat everything as raw data! This is intentional but be extra mindful of that,
//   Do NOT use this class as a std::vector replacement in your own code! Many of the structures used by dear imgui can be safely initialized by a zero-memset.
//-----------------------------------------------------------------------------

// IM_MSVC_RUNTIME_CHECKS_OFF
// template<typename T>
// struct ImVector
// {
//     int                 size;
//     int                 Capacity;
//     T*                  data;
// 
//     // Provide standard typedefs but we don't use them ourselves.
//     typedef T                   value_type;
//     typedef value_type*         iterator;
//     typedef const value_type*   const_iterator;
// 
//     // Constructors, destructor
//     inline ImVector()                                       { size = Capacity = 0; data = NULL; }
//     inline ImVector(const ImVector<T>& src)                 { size = Capacity = 0; data = NULL; operator=(src); }
//     inline ImVector<T>& operator=(const ImVector<T>& src)   { clear(); resize(src.size); memcpy(data, src.data, (size_t)size * sizeof(T)); return *this; }
//     inline ~ImVector()                                      { if (data) IM_FREE(data); } // Important: does not destruct anything
// 
//     inline void         clear()                             { if (data) { size = Capacity = 0; IM_FREE(data); data = NULL; } }  // Important: does not destruct anything
//     inline void         clear_delete()                      { for (int n = 0; n < size; n++) IM_DELETE(data[n]); clear(); }     // Important: never called automatically! always explicit.
//     inline void         clear_destruct()                    { for (int n = 0; n < size; n++) data[n].~T(); clear(); }           // Important: never called automatically! always explicit.
// 
//     inline bool         empty() const                       { return size == 0; }
//     inline int          size() const                        { return size; }
//     inline int          size_in_bytes() const               { return size * sizeof(T); }
//     inline int          max_size() const                    { return 0x7FFFFFFF / sizeof(T); }
//     inline int          capacity() const                    { return Capacity; }
//     inline T&           operator[](int i)                   { IM_ASSERT(i >= 0 && i < size); return data[i]; }
//     inline const T&     operator[](int i) const             { IM_ASSERT(i >= 0 && i < size); return data[i]; }
// 
//     inline T*           begin()                             { return data; }
//     inline const T*     begin() const                       { return data; }
//     inline T*           end()                               { return data + size; }
//     inline const T*     end() const                         { return data + size; }
//     inline T&           front()                             { IM_ASSERT(size > 0); return data[0]; }
//     inline const T&     front() const                       { IM_ASSERT(size > 0); return data[0]; }
//     inline T&           back()                              { IM_ASSERT(size > 0); return data[size - 1]; }
//     inline const T&     back() const                        { IM_ASSERT(size > 0); return data[size - 1]; }
//     inline void         swap(ImVector<T>& rhs)              { int rhs_size = rhs.size; rhs.size = size; size = rhs_size; int rhs_cap = rhs.Capacity; rhs.Capacity = Capacity; Capacity = rhs_cap; T* rhs_data = rhs.data; rhs.data = data; data = rhs_data; }
// 
//     inline int          _grow_capacity(int sz) const        { int new_capacity = Capacity ? (Capacity + Capacity / 2) : 8; return new_capacity > sz ? new_capacity : sz; }
//     inline void         resize(int new_size)                { if (new_size > Capacity) reserve(_grow_capacity(new_size)); size = new_size; }
//     inline void         resize(int new_size, const T& v)    { if (new_size > Capacity) reserve(_grow_capacity(new_size)); if (new_size > size) for (int n = size; n < new_size; n++) memcpy(&data[n], &v, sizeof(v)); size = new_size; }
//     inline void         shrink(int new_size)                { IM_ASSERT(new_size <= size); size = new_size; } // Resize a vector to a smaller size, guaranteed not to cause a reallocation
//     inline void         reserve(int new_capacity)           { if (new_capacity <= Capacity) return; T* new_data = (T*)IM_ALLOC((size_t)new_capacity * sizeof(T)); if (data) { memcpy(new_data, data, (size_t)size * sizeof(T)); IM_FREE(data); } data = new_data; Capacity = new_capacity; }
//     inline void         reserve_discard(int new_capacity)   { if (new_capacity <= Capacity) return; if (data) IM_FREE(data); data = (T*)IM_ALLOC((size_t)new_capacity * sizeof(T)); Capacity = new_capacity; }
// 
//     // NB: It is illegal to call push_back/push_front/insert with a reference pointing inside the ImVector data itself! e.g. v.push_back(v[10]) is forbidden.
//     inline void         push_back(const T& v)               { if (size == Capacity) reserve(_grow_capacity(size + 1)); memcpy(&data[size], &v, sizeof(v)); size++; }
//     inline void         pop_back()                          { IM_ASSERT(size > 0); size--; }
//     inline void         push_front(const T& v)              { if (size == 0) push_back(v); else insert(data, v); }
//     inline T*           erase(const T* it)                  { IM_ASSERT(it >= data && it < data + size); const ptrdiff_t off = it - data; memmove(data + off, data + off + 1, ((size_t)size - (size_t)off - 1) * sizeof(T)); size--; return data + off; }
//     inline T*           erase(const T* it, const T* it_last){ IM_ASSERT(it >= data && it < data + size && it_last >= it && it_last <= data + size); const ptrdiff_t count = it_last - it; const ptrdiff_t off = it - data; memmove(data + off, data + off + count, ((size_t)size - (size_t)off - (size_t)count) * sizeof(T)); size -= count; return data + off; }
//     inline T*           erase_unsorted(const T* it)         { IM_ASSERT(it >= data && it < data + size);  const ptrdiff_t off = it - data; if (it < data + size - 1) memcpy(data + off, data + size - 1, sizeof(T)); size--; return data + off; }
//     inline T*           insert(const T* it, const T& v)     { IM_ASSERT(it >= data && it <= data + size); const ptrdiff_t off = it - data; if (size == Capacity) reserve(_grow_capacity(size + 1)); if (off < size) memmove(data + off + 1, data + off, ((size_t)size - (size_t)off) * sizeof(T)); memcpy(&data[off], &v, sizeof(v)); size++; return data + off; }
//     inline bool         contains(const T& v) const          { const T* data = data;  const T* data_end = data + size; while (data < data_end) if (*data++ == v) return true; return false; }
//     inline T*           find(const T& v)                    { T* data = data;  const T* data_end = data + size; while (data < data_end) if (*data == v) break; else ++data; return data; }
//     inline const T*     find(const T& v) const              { const T* data = data;  const T* data_end = data + size; while (data < data_end) if (*data == v) break; else ++data; return data; }
//     inline bool         find_erase(const T& v)              { const T* it = find(v); if (it < data + size) { erase(it); return true; } return false; }
//     inline bool         find_erase_unsorted(const T& v)     { const T* it = find(v); if (it < data + size) { erase_unsorted(it); return true; } return false; }
//     inline int          index_from_ptr(const T* it) const   { IM_ASSERT(it >= data && it < data + size); const ptrdiff_t off = it - data; return off; }
// };
// IM_MSVC_RUNTIME_CHECKS_RESTORE

//-----------------------------------------------------------------------------
// [SECTION] ImGuiStyle
//-----------------------------------------------------------------------------
// You may modify the ImGui::GetStyle() main instance during initialization and before NewFrame().
// During the frame, use ImGui::PushStyleVar(ImGuiStyleVar_XXXX)/PopStyleVar() to alter the main style values,
// and ImGui::PushStyleColor(ImGuiCol_XXX)/PopStyleColor() for colors.
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] ImGuiIO
//-----------------------------------------------------------------------------
// Communicate most settings and inputs/outputs to Dear ImGui using this structure.
// Access via ImGui::GetIO(). Read 'Programmer guide' section in .cpp file for general usage.
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] Misc data structures
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] Helpers (ImGuiOnceUponAFrame, ImGuiTextFilter, ImGuiTextBuffer, ImGuiStorage, ImGuiListClipper, ImColor)
//-----------------------------------------------------------------------------
// Maximum Unicode code point supported by this build.
// #else
// #define IM_UNICODE_CODEPOINT_MAX     0xFFFF     // Maximum Unicode code point supported by this build.
// #endif

//-----------------------------------------------------------------------------
// [SECTION] Drawing API (ImDrawCmd, ImDrawIdx, ImDrawVert, ImDrawChannel, ImDrawListSplitter, ImDrawListFlags, ImDrawList, ImDrawData)
// Hold a series of drawing commands. The user provides a renderer for ImDrawData which essentially contains an array of ImDrawList.
//-----------------------------------------------------------------------------


// ImDrawCallback: Draw callbacks for advanced uses [configurable type: override in imconfig.h]
// NB: You most likely do NOT need to use draw callbacks just to create your own widget or customized UI rendering,
// you can poke into the draw list for that! Draw callback may be useful for example to:
//  A) Change your GPU render state,
//  B) render a complex 3D scene inside a UI element without an intermediate texture/render target, etc.
// The expected behavior from your rendering function is 'if (cmd.user_callback != NULL) { cmd.user_callback(parent_list, cmd); } else { RenderTriangles() }'
// If you want to override the signature of ImDrawCallback, you can simply use e.g. '#define ImDrawCallback MyDrawCallback' (in imconfig.h) + update rendering backend accordingly.
// #ifndef ImDrawCallback
// typedef void (*ImDrawCallback)(const ImDrawList* parent_list, const ImDrawCmd* cmd);
// #endif

// TODO
// Special Draw callback value to request renderer backend to reset the graphics/render state.
// The renderer backend needs to handle this special value, otherwise it will crash trying to call a function at this address.
// This is useful for example if you submitted callbacks which you know have altered the render state and you want it to be restored.
// It is not done by default because they are many perfectly useful way of altering render state for imgui contents (e.g. changing shader/blending settings before an Image call).
// #define ImDrawCallback_ResetRenderState     (ImDrawCallback)(-1)
// #else
// You can override the vertex format layout by defining IMGUI_OVERRIDE_DRAWVERT_STRUCT_LAYOUT in imconfig.h
// The code expect ImVec2 pos (8 bytes), ImVec2 uv (8 bytes), ImU32 col (4 bytes), but you can re-order them or add other fields as needed to simplify integration in your engine.
// The type has to be described within the macro (you can either declare the struct or use a typedef). This is because ImVec2/ImU32 are likely not declared a the time you'd want to set your type up.
// NOTE: IMGUI DOESN'T CLEAR THE STRUCTURE AND DOESN'T CALL A CONSTRUCTOR SO ANY CUSTOM FIELD WILL BE UNINITIALIZED. IF YOU ADD EXTRA FIELDS (SUCH AS A 'Z' COORDINATES) YOU WILL NEED TO CLEAR THEM DURING RENDER OR TO IGNORE THEM.
// IMGUI_OVERRIDE_DRAWVERT_STRUCT_LAYOUT;
// #endif

// pub const RoundCornersTop: u32             = DimgDrawFlags::RoundCornersTopLeft | DimgDrawFlags::RoundCornersTopRight;
// Default to ALL corners if none of the _RoundCornersXX flags are specified.

//-----------------------------------------------------------------------------
// [SECTION] font API (ImFontConfig, ImFontGlyph, ImFontAtlasFlags, ImFontAtlas, ImFontGlyphRangesBuilder, ImFont)
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] viewports
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] Platform Dependent Interfaces (for e.g. multi-viewport support)
//-----------------------------------------------------------------------------
// [BETA] (Optional) This is completely optional, for advanced users!
// If you are new to Dear ImGui and trying to integrate it into your engine, you can probably ignore this for now.
//
// This feature allows you to seamlessly drag Dear ImGui windows outside of your application viewport.
// This is achieved by creating new Platform/OS windows on the fly, and rendering into them.
// Dear ImGui manages the viewport structures, and the backend create and maintain one Platform/OS window for each of those viewports.
//
// See Glossary https://github.com/ocornut/imgui/wiki/Glossary for details about some of the terminology.
// See Thread https://github.com/ocornut/imgui/issues/1542 for gifs, news and questions about this evolving feature.
//
// About the coordinates system:
// - When multi-viewports are enabled, all Dear ImGui coordinates become absolute coordinates (same as OS coordinates!)
// - So e.g. ImGui::SetNextWindowPos(ImVec2(0,0)) will position a window relative to your primary monitor!
// - If you want to position windows relative to your main application viewport, use ImGui::GetMainViewport()->pos as a base position.
//
// Steps to use multi-viewports in your application, when using a default backend from the examples/ folder:
// - Application:  Enable feature with 'io.config_flags |= ImGuiConfigFlags_ViewportsEnable'.
// - Backend:      The backend initialization will setup all necessary ImGuiPlatformIO's functions and update monitors info every frame.
// - Application:  In your main loop, call ImGui::UpdatePlatformWindows(), ImGui::RenderPlatformWindowsDefault() after EndFrame() or Render().
// - Application:  Fix absolute coordinates used in ImGui::SetWindowPos() or ImGui::SetNextWindowPos() calls.
//
// Steps to use multi-viewports in your application, when using a custom backend:
// - Important:    THIS IS NOT EASY TO DO and comes with many subtleties not described here!
//                 It's also an experimental feature, so some of the requirements may evolve.
//                 Consider using default backends if you can. Either way, carefully follow and refer to examples/ backends for details.
// - Application:  Enable feature with 'io.config_flags |= ImGuiConfigFlags_ViewportsEnable'.
// - Backend:      Hook ImGuiPlatformIO's Platform_* and Renderer_* callbacks (see below).
//                 Set 'io.backend_flags |= ImGuiBackendFlags_PlatformHasViewports' and 'io.backend_flags |= ImGuiBackendFlags_PlatformHasViewports'.
//                 Update ImGuiPlatformIO's Monitors list every frame.
//                 Update mouse_pos every frame, in absolute coordinates.
// - Application:  In your main loop, call ImGui::UpdatePlatformWindows(), ImGui::RenderPlatformWindowsDefault() after EndFrame() or Render().
//                 You may skip calling RenderPlatformWindowsDefault() if its API is not convenient for your needs. Read comments below.
// - Application:  Fix absolute coordinates used in ImGui::SetWindowPos() or ImGui::SetNextWindowPos() calls.
//
// About ImGui::RenderPlatformWindowsDefault():
// - This function is a mostly a _helper_ for the common-most cases, and to facilitate using default backends.
// - You can check its simple source code to understand what it does.
//   It basically iterates secondary viewports and call 4 functions that are setup in ImGuiPlatformIO, if available:
//     Platform_RenderWindow(), Renderer_RenderWindow(), Platform_SwapBuffers(), Renderer_SwapBuffers()
//   Those functions pointers exists only for the benefit of RenderPlatformWindowsDefault().
// - If you have very specific rendering needs (e.g. flipping multiple swap-chain simultaneously, unusual sync/threading issues, etc.),
//   you may be tempted to ignore RenderPlatformWindowsDefault() and write customized code to perform your renderingg.
//   You may decide to setup the platform_io's *RenderWindow and *SwapBuffers pointers and call your functions through those pointers,
//   or you may decide to never setup those pointers and call your code directly. They are a convenience, not an obligatory interface.
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] Obsolete functions and types
// (Will be removed! Read 'API BREAKING CHANGES' section in imgui.cpp for details)
// Please keep your copy of dear imgui up to date! Occasionally set '#define IMGUI_DISABLE_OBSOLETE_FUNCTIONS' in imconfig.h to stay ahead.
//-----------------------------------------------------------------------------

// namespace ImGui
// {
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//      int       GetKeyIndex(ImGuiKey key);  // map ImGuiKey_* values into legacy native key index. == io.KeyMap[key]
// #else
//     static inline int   GetKeyIndex(ImGuiKey key)   { IM_ASSERT(key >= ImGuiKey_NamedKey_BEGIN && key < ImGuiKey_NamedKey_END && "ImGuiKey and native_index was merged together and native_index is disabled by IMGUI_DISABLE_OBSOLETE_KEYIO. Please switch to ImGuiKey."); return key; }
// #endif
// }
//
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
// namespace ImGui
// {
//     // OBSOLETED in 1.88 (from May 2022)
//     static inline void  CaptureKeyboardFromApp(bool want_capture_keyboard = true)   { SetNextFrameWantCaptureKeyboard(want_capture_keyboard); } // Renamed as name was misleading + removed default value.
//     static inline void  CaptureMouseFromApp(bool want_capture_mouse = true)         { SetNextFrameWantCaptureMouse(want_capture_mouse); }       // Renamed as name was misleading + removed default value.
//     // OBSOLETED in 1.86 (from November 2021)
//      void      CalcListClipping(int items_count, float items_height, int* out_items_display_start, int* out_items_display_end); // Calculate coarse clipping for large list of evenly sized items. Prefer using ImGuiListClipper.
//     // OBSOLETED in 1.85 (from August 2021)
//     static inline float GetWindowContentRegionWidth()                               { return GetWindowContentRegionMax().x - GetWindowContentRegionMin().x; }
//     // OBSOLETED in 1.81 (from February 2021)
//      bool      ListBoxHeader(const char* label, int items_count, int height_in_items = -1); // Helper to calculate size from items_count and height_in_items
//     static inline bool  ListBoxHeader(const char* label, const ImVec2& size = ImVec2(0, 0))         { return BeginListBox(label, size); }
//     static inline void  ListBoxFooter() { EndListBox(); }
//     // OBSOLETED in 1.79 (from August 2020)
//     static inline void  OpenPopupContextItem(const char* str_id = NULL, ImGuiMouseButton mb = 1)    { OpenPopupOnItemClick(str_id, mb); } // Bool return value removed. Use IsWindowAppearing() in BeginPopup() instead. Renamed in 1.77, renamed back in 1.79. Sorry!
//     // OBSOLETED in 1.78 (from June 2020)
//     // Old drag/sliders functions that took a 'float power = 1.0' argument instead of flags.
//     // For shared code, you can version check at compile-time with `#if IMGUI_VERSION_NUM >= 17704`.
//      bool      DragScalar(const char* label, ImGuiDataType data_type, void* p_data, float v_speed, const void* p_min, const void* p_max, const char* format, float power);
//      bool      DragScalarN(const char* label, ImGuiDataType data_type, void* p_data, int components, float v_speed, const void* p_min, const void* p_max, const char* format, float power);
//     static inline bool  DragFloat(const char* label, float* v, float v_speed, float v_min, float v_max, const char* format, float power)    { return DragScalar(label, ImGuiDataType_Float, v, v_speed, &v_min, &v_max, format, power); }
//     static inline bool  DragFloat2(const char* label, float v[2], float v_speed, float v_min, float v_max, const char* format, float power) { return DragScalarN(label, ImGuiDataType_Float, v, 2, v_speed, &v_min, &v_max, format, power); }
//     static inline bool  DragFloat3(const char* label, float v[3], float v_speed, float v_min, float v_max, const char* format, float power) { return DragScalarN(label, ImGuiDataType_Float, v, 3, v_speed, &v_min, &v_max, format, power); }
//     static inline bool  DragFloat4(const char* label, float v[4], float v_speed, float v_min, float v_max, const char* format, float power) { return DragScalarN(label, ImGuiDataType_Float, v, 4, v_speed, &v_min, &v_max, format, power); }
//      bool      SliderScalar(const char* label, ImGuiDataType data_type, void* p_data, const void* p_min, const void* p_max, const char* format, float power);
//      bool      SliderScalarN(const char* label, ImGuiDataType data_type, void* p_data, int components, const void* p_min, const void* p_max, const char* format, float power);
//     static inline bool  SliderFloat(const char* label, float* v, float v_min, float v_max, const char* format, float power)                 { return SliderScalar(label, ImGuiDataType_Float, v, &v_min, &v_max, format, power); }
//     static inline bool  SliderFloat2(const char* label, float v[2], float v_min, float v_max, const char* format, float power)              { return SliderScalarN(label, ImGuiDataType_Float, v, 2, &v_min, &v_max, format, power); }
//     static inline bool  SliderFloat3(const char* label, float v[3], float v_min, float v_max, const char* format, float power)              { return SliderScalarN(label, ImGuiDataType_Float, v, 3, &v_min, &v_max, format, power); }
//     static inline bool  SliderFloat4(const char* label, float v[4], float v_min, float v_max, const char* format, float power)              { return SliderScalarN(label, ImGuiDataType_Float, v, 4, &v_min, &v_max, format, power); }
//     // OBSOLETED in 1.77 (from June 2020)
//     static inline bool  BeginPopupContextWindow(const char* str_id, ImGuiMouseButton mb, bool over_items) { return BeginPopupContextWindow(str_id, mb | (over_items ? 0 : ImGuiPopupFlags_NoOpenOverItems)); }
//
//     // Some of the older obsolete names along with their replacement (commented out so they are not reported in IDE)
//     //static inline void  TreeAdvanceToLabelPos()               { SetCursorPosX(GetCursorPosX() + GetTreeNodeToLabelSpacing()); }   // OBSOLETED in 1.72 (from July 2019)
//     //static inline void  SetNextTreeNodeOpen(bool open, ImGuiCond cond = 0) { SetNextItemOpen(open, cond); }                       // OBSOLETED in 1.71 (from June 2019)
//     //static inline float GetContentRegionAvailWidth()          { return GetContentRegionAvail().x; }                               // OBSOLETED in 1.70 (from May 2019)
//     //static inline ImDrawList* GetOverlayDrawList()            { return GetForegroundDrawList(); }                                 // OBSOLETED in 1.69 (from Mar 2019)
//     //static inline void  SetScrollHere(float ratio = 0.5)     { SetScrollHereY(ratio); }                                          // OBSOLETED in 1.66 (from Nov 2018)
//     //static inline bool  IsItemDeactivatedAfterChange()        { return IsItemDeactivatedAfterEdit(); }                            // OBSOLETED in 1.63 (from Aug 2018)
//     //static inline bool  IsAnyWindowFocused()                  { return IsWindowFocused(ImGuiFocusedFlags_AnyWindow); }            // OBSOLETED in 1.60 (from Apr 2018)
//     //static inline bool  IsAnyWindowHovered()                  { return IsWindowHovered(ImGuiHoveredFlags_AnyWindow); }            // OBSOLETED in 1.60 (between Dec 2017 and Apr 2018)
//     //static inline void  ShowTestWindow()                      { return ShowDemoWindow(); }                                        // OBSOLETED in 1.53 (between Oct 2017 and Dec 2017)
//     //static inline bool  IsRootWindowFocused()                 { return IsWindowFocused(ImGuiFocusedFlags_RootWindow); }           // OBSOLETED in 1.53 (between Oct 2017 and Dec 2017)
//     //static inline bool  IsRootWindowOrAnyChildFocused()       { return IsWindowFocused(ImGuiFocusedFlags_RootAndChildWindows); }  // OBSOLETED in 1.53 (between Oct 2017 and Dec 2017)
//     //static inline void  SetNextWindowContentWidth(float w)    { SetNextWindowContentSize(ImVec2(w, 0.0)); }                      // OBSOLETED in 1.53 (between Oct 2017 and Dec 2017)
//     //static inline float GetItemsLineHeightWithSpacing()       { return GetFrameHeightWithSpacing(); }                             // OBSOLETED in 1.53 (between Oct 2017 and Dec 2017)
// }
//
// // OBSOLETED in 1.82 (from Mars 2021): flags for AddRect(), AddRectFilled(), AddImageRounded(), PathRect()
// // typedef ImDrawFlags ImDrawCornerFlags;
// type
//
// enum ImDrawCornerFlags_
//
// {
//     ImDrawCornerFlags_None      = ImDrawFlags_RoundCornersNone,         // Was == 0 prior to 1.82, this is now == ImDrawFlags_RoundCornersNone which is != 0 and not implicit
//     ImDrawCornerFlags_TopLeft   = ImDrawFlags_RoundCornersTopLeft,      // Was == 0x01 (1 << 0) prior to 1.82. Order matches ImDrawFlags_NoRoundCorner* flag (we exploit this internally).
//     ImDrawCornerFlags_TopRight  = ImDrawFlags_RoundCornersTopRight,     // Was == 0x02 (1 << 1) prior to 1.82.
//     ImDrawCornerFlags_BotLeft   = ImDrawFlags_RoundCornersBottomLeft,   // Was == 0x04 (1 << 2) prior to 1.82.
//     ImDrawCornerFlags_BotRight  = ImDrawFlags_RoundCornersBottomRight,  // Was == 0x08 (1 << 3) prior to 1.82.
//     ImDrawCornerFlags_All       = ImDrawFlags_RoundCornersAll,          // Was == 0x0F prior to 1.82
//     ImDrawCornerFlags_Top       = ImDrawCornerFlags_TopLeft | ImDrawCornerFlags_TopRight,
//     ImDrawCornerFlags_Bot       = ImDrawCornerFlags_BotLeft | ImDrawCornerFlags_BotRight,
//     ImDrawCornerFlags_Left      = ImDrawCornerFlags_TopLeft | ImDrawCornerFlags_BotLeft,
//     ImDrawCornerFlags_Right     = ImDrawCornerFlags_TopRight | ImDrawCornerFlags_BotRight
// };
//
// // RENAMED ImGuiKeyModFlags -> ImGuiModFlags in 1.88 (from April 2022)
// // typedef int ImGuiKeyModFlags;
//
// pub type ImGuiKeyModFlags = i32;
//
// enum ImGuiKeyModFlags_ { ImGuiKeyModFlags_None = ImGuiModFlags_None, ImGuiKeyModFlags_Ctrl = ImGuiModFlags_Ctrl, ImGuiKeyModFlags_Shift = ImGuiModFlags_Shift, ImGuiKeyModFlags_Alt = ImGuiModFlags_Alt, ImGuiKeyModFlags_Super = ImGuiModFlags_Super };

// #endif // #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS

// RENAMED IMGUI_DISABLE_METRICS_WINDOW > IMGUI_DISABLE_DEBUG_TOOLS in 1.88 (from June 2022)
// #if defined(IMGUI_DISABLE_METRICS_WINDOW) && !defined(IMGUI_DISABLE_OBSOLETE_FUNCTIONS) && !defined(IMGUI_DISABLE_DEBUG_TOOLS)
// #define IMGUI_DISABLE_DEBUG_TOOLS
// #endif
// #if defined(IMGUI_DISABLE_METRICS_WINDOW) && defined(IMGUI_DISABLE_OBSOLETE_FUNCTIONS)
// #error IMGUI_DISABLE_METRICS_WINDOW was renamed to IMGUI_DISABLE_DEBUG_TOOLS, please use std::default::default;
// use std::ffi::c_void;
// use std::ops::Index;
// use std::ptr;
// use new
// use crate::imgui_h::ImGuiCol::ImGuiCol_COUNT;
// use crate::imgui_h::ImGuiWindowFlags::{ImGuiWindowFlags_NoCollapse, ImGuiWindowFlags_NoMouseInputs, ImGuiWindowFlags_NoNavFocus, ImGuiWindowFlags_NoNavInputs, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoScrollbar, ImGuiWindowFlags_NoTitleBar}; name.
// #endif

//-----------------------------------------------------------------------------

// #if defined(__clang__)
// #pragma clang diagnostic pop
// #elif defined(__GNUC__)
// #pragma GCC diagnostic pop
// #endif

// #ifdef _MSC_VER
// #pragma warning (pop)
// #endif
