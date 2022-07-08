// dear imgui, v1.88

use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use crate::draw_list::DimgDrawList;
use crate::vec_nd::{DimgVec2D, DimgVec4};
use crate::input::ImGuiInputCallbackData;

// Debug options
// #define IMGUI_DEBUG_NAV_SCORING     0   // Display navigation scoring preview when hovering items. Display last moving direction matches when holding CTRL
pub const IMGUI_DEBUG_NAV_SCORING: bool = false;
// #define IMGUI_DEBUG_NAV_RECTS       0   // Display the reference navigation rectangle for each window
pub const IMGUI_DEBUG_NAV_RECTS: bool = false;
// #define IMGUI_DEBUG_INI_SETTINGS    0   // Save additional comments in .ini file (particularly helps for Docking, but makes saving slower)
pub const IMGUI_DEBUG_INI_SETINGS: bool = false;

// When using CTRL+TAB (or Gamepad Square+L/R) we delay the visual a little in order to reduce visual noise doing a fast switch.
// static const float NAV_WINDOWING_HIGHLIGHT_DELAY            = 0.20;    // time before the highlight and screen dimming starts fading in
pub const NAV_WINDOWING_HIGHLIGHT_DELAY: f32 = 0.20;
// static const float NAV_WINDOWING_LIST_APPEAR_DELAY          = 0.15;    // time before the window list starts to appear
pub const NAV_WINDOWING_LIST_APPEAR_DELAY: f32 = 0.15;

// Window resizing from edges (when io.ConfigWindowsResizeFromEdges = true and ImGuiBackendFlags_HasMouseCursors is set in io.BackendFlags by backend)
// static const float WINDOWS_HOVER_PADDING                    = 4.0;     // Extend outside window for hovering/resizing (maxxed with TouchPadding) and inside windows for borders. Affect FindHoveredWindow().
pub const WINDOWS_HOVER_PADDING: f32 = 4.0;
// static const float WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER = 0.04;    // Reduce visual noise by only highlighting the border after a certain time.
pub const WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER: f32 = 0.04;
// static const float WINDOWS_MOUSE_WHEEL_SCROLL_LOCK_TIMER    = 2.00;    // Lock scrolled window (so it doesn't pick child windows that are scrolling through) for a certain time, unless mouse moved.
pub const WINDOWS_MOUSE_WHEEL_SCROLL_LOCK_TIMER: f32 = 2.00;

// Docking
// static const float DOCKING_TRANSPARENT_PAYLOAD_ALPHA        = 0.50;    // For use with io.ConfigDockingTransparentPayload. Apply to viewport _or_ WindowBg in host viewport.
pub const DOCKING_TRANSPARENT_PAYLOAD_ALPHA: f32 = 0.50;
// static const float DOCKING_SPLITTER_SIZE                    = 2.0;
pub const DOCKING_SPLITTER_SIZE: f32 = 2.0;

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
// typedef pub ImDrawListFlags: i32,      // -> enum ImDrawListFlags_      // flags: for ImDrawList instance
// typedef pub ImFontAtlasFlags: i32,     // -> enum ImFontAtlasFlags_     // flags: for ImFontAtlas build
// typedef pub ImGuiBackendFlags: i32,    // -> enum ImGuiBackendFlags_    // flags: for io.BackendFlags
// typedef pub ImGuiButtonFlags: i32,     // -> enum ImGuiButtonFlags_     // flags: for InvisibleButton()
// typedef pub ImGuiColorEditFlags: i32,  // -> enum ImGuiColorEditFlags_  // flags: for ColorEdit4(), ColorPicker4() etc.
// typedef pub ImGuiConfigFlags: i32,     // -> enum ImGuiConfigFlags_     // flags: for io.ConfigFlags
// typedef pub ImGuiComboFlags: i32,      // -> enum ImGuiComboFlags_      // flags: for BeginCombo()
// typedef pub ImGuiDockNodeFlags: i32,   // -> enum ImGuiDockNodeFlags_   // flags: for DockSpace()
// typedef pub ImGuiDragDropFlags: i32,   // -> enum ImGuiDragDropFlags_   // flags: for BeginDragDropSource(), AcceptDragDropPayload()
// typedef pub ImGuiFocusedFlags: i32,    // -> enum ImGuiFocusedFlags_    // flags: for IsWindowFocused()
// typedef pub ImGuiHoveredFlags: i32,    // -> enum ImGuiHoveredFlags_    // flags: for IsItemHovered(), IsWindowHovered() etc.
// typedef pub ImGuiInputTextFlags: i32,  // -> enum ImGuiInputTextFlags_  // flags: for InputText(), InputTextMultiline()
// typedef pub ImGuiModFlags: i32,        // -> enum ImGuiModFlags_        // flags: for io.KeyMods (Ctrl/Shift/Alt/Super)
// typedef pub ImGuiPopupFlags: i32,      // -> enum ImGuiPopupFlags_      // flags: for OpenPopup*(), BeginPopupContext*(), IsPopupOpen()
// typedef pub ImGuiSelectableFlags: i32, // -> enum ImGuiSelectableFlags_ // flags: for Selectable()
// typedef pub ImGuiSliderFlags: i32,     // -> enum ImGuiSliderFlags_     // flags: for DragFloat(), DragInt(), SliderFloat(), SliderInt() etc.
// typedef pub ImGuiTabBarFlags: i32,     // -> enum ImGuiTabBarFlags_     // flags: for BeginTabBar()
// typedef pub ImGuiTabItemFlags: i32,    // -> enum ImGuiTabItemFlags_    // flags: for BeginTabItem()
// typedef pub ImGuiTableFlags: i32,      // -> enum ImGuiTableFlags_      // flags: For BeginTable()
// typedef pub ImGuiTableColumnFlags: i32,// -> enum ImGuiTableColumnFlags_// flags: For TableSetupColumn()
// typedef pub ImGuiTableRowFlags: i32,   // -> enum ImGuiTableRowFlags_   // flags: For TableNextRow()
// typedef pub ImGuiTreeNodeFlags: i32,   // -> enum ImGuiTreeNodeFlags_   // flags: for TreeNode(), TreeNodeEx(), CollapsingHeader()
// typedef pub ImGuiViewportFlags: i32,   // -> enum ImGuiViewportFlags_   // flags: for ImGuiViewport
// typedef pub ImGuiWindowFlags: i32,     // -> enum ImGuiWindowFlags_     // flags: for Begin(), BeginChild()

// ImTexture: user data for renderer backend to identify a texture [Compile-time configurable type]
// - To use something else than an opaque void* pointer: override with e.g. '#define ImTextureID MyTextureType*' in your imconfig.h file.
// - This can be whatever to you want it to be! read the FAQ about ImTextureID for details.
// #ifndef ImTextureID
// typedef void* ImTextureID;          // Default: store a pointer or an integer fitting in a pointer (most renderer backends are ok with that)
pub type DimgTextureId = DimgId;

// #endif

// ImDrawIdx: vertex index. [Compile-time configurable type]
// - To use 16-bit indices + allow large meshes: backend need to set 'io.BackendFlags |= ImGuiBackendFlags_RendererHasVtxOffset' and handle ImDrawCmd::vtx_offset (recommended).
// - To use 32-bit indices: override with '#define ImDrawIdx unsigned int' in your imconfig.h file.
// #ifndef ImDrawIdx
// typedef unsigned short ImDrawIdx;   // Default: 16-bit (for maximum compatibility with renderer backends)
// #endif



// Scalar data types
// typedef unsigned int        ImGuiID;// A unique ID used by widgets (typically the result of hashing a stack of string)
pub type DimgId = u32;
// typedef signed char         ImS8;   // 8-bit signed integer
// typedef unsigned char       ImU8;   // 8-bit unsigned integer
// typedef signed short        ImS16;  // 16-bit signed integer
// typedef unsigned short      ImU16;  // 16-bit unsigned integer
// typedef signed pub ImS32: i32,// 32-bit signed integer == int
// typedef unsigned pub ImU32: i32,// 32-bit unsigned integer (often used to store packed colors)
// typedef signed   long long  ImS64;  // 64-bit signed integer
// typedef unsigned long long  ImU64;  // 64-bit unsigned integer

// Character types
// (we generally use UTF-8 encoded string in the API. This is storage specifically for a decoded character used for keyboard input and display)
// typedef unsigned short ImWchar16;   // A single decoded U16 character/code point. We encode them as multi bytes UTF-8 when used in strings.
pub type ImWchar16 = u16;
// typedef unsigned pub ImWchar32: i32,   // A single decoded U32 character/code point. We encode them as multi bytes UTF-8 when used in strings.
pub type ImWchar32 = u32;
// #ifdef IMGUI_USE_WCHAR32            // ImWchar [configurable type: override in imconfig.h with '#define IMGUI_USE_WCHAR32' to support Unicode planes 1-16]
// typedef ImWchar32 ImWchar;
// #else
// typedef ImWchar16 ImWchar;
// #endif
pub type ImWchar = ImWchar32;
// Callback and functions types
// typedef int     (*ImGuiInputTextCallback)(ImGuiInputTextCallbackData* data);    // Callback function for ImGui::InputText()
pub type ImGuiInputTextCallback = fn(*mut ImGuiInputCallbackData) -> i32;
// typedef void    (*ImGuiSizeCallback)(ImGuiSizeCallbackData* data);              // Callback function for ImGui::SetNextWindowSizeConstraints()
pub type ImGuiSizeCallback = fn(*mut DimgSizeCallbackData);
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
//      void          ShowStackToolWindow(bool* p_open = NULL);   // create Stack Tool window. hover items with mouse to query information about the source of their unique ID.
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
//      bool          IsWindowHovered(ImGuiHoveredFlags flags=0); // is current window hovered (and typically: not blocked by a popup/modal)? see flags for options. NB: If you are trying to check whether your mouse should be dispatched to imgui or to your app, you should use the 'io.WantCaptureMouse' boolean for that! Please read the FAQ!
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
//      void          SetNextWindowSizeConstraints(const ImVec2& size_min, const ImVec2& size_max, ImGuiSizeCallback custom_callback = NULL, void* custom_callback_data = NULL); // set next window size limits. use -1,-1 on either X/Y axis to preserve the current size. Sizes will be rounded down. Use callback to apply non-trivial programmatic constraints.
//      void          SetNextWindowContentSize(const ImVec2& size);                               // set next window content size (~ scrollable client area, which enforce the range of scrollbars). Not including window decorations (title bar, menu bar, etc.) nor window_padding. set an axis to 0.0 to leave it automatic. call before Begin()
//      void          SetNextWindowCollapsed(bool collapsed, ImGuiCond cond = 0);                 // set next window collapsed state. call before Begin()
//      void          SetNextWindowFocus();                                                       // set next window to be focused / top-most. call before Begin()
//      void          SetNextWindowBgAlpha(float alpha);                                          // set next window background color alpha. helper to easily override the Alpha component of ImGuiCol_WindowBg/ChildBg/PopupBg. you may also use ImGuiWindowFlags_NoBackground.
//      void          SetNextWindowViewport(ImGuiID viewport_id);                                 // set next window viewport
//      void          SetWindowPos(const ImVec2& pos, ImGuiCond cond = 0);                        // (not recommended) set current window position - call within Begin()/End(). prefer using SetNextWindowPos(), as this may incur tearing and side-effects.
//      void          SetWindowSize(const ImVec2& size, ImGuiCond cond = 0);                      // (not recommended) set current window size - call within Begin()/End(). set to ImVec2(0, 0) to force an auto-fit. prefer using SetNextWindowSize(), as this may incur tearing and minor side-effects.
//      void          SetWindowCollapsed(bool collapsed, ImGuiCond cond = 0);                     // (not recommended) set current window collapsed state. prefer using SetNextWindowCollapsed().
//      void          SetWindowFocus();                                                           // (not recommended) set current window to be focused / top-most. prefer using SetNextWindowFocus().
//      void          SetWindowFontScale(float scale);                                            // [OBSOLETE] set font scale. Adjust io.FontGlobalScale if you want to scale all windows. This is an old API! For correct scaling, prefer to reload font + rebuild ImFontAtlas + call style.scale_all_sizes().
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
//      void          PushButtonRepeat(bool repeat);                                  // in 'repeat' mode, Button*() functions return repeated true in a typematic manner (using io.KeyRepeatDelay/io.KeyRepeatRate setting). Note that you can call IsItemActive() after any Button() to tell if the button is held in the current frame.
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
//      void          SameLine(float offset_from_start_x=0.0, float spacing=-1.0);  // call between widgets or groups to layout them horizontally. X position given in window coordinates.
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
//      ImVec2        GetCursorScreenPos();                                           // cursor position in absolute coordinates (useful to work with ImDrawList API). generally top-left == GetMainViewport()->pos == (0,0) in single viewport mode, and bottom-right == GetMainViewport()->pos+size == io.DisplaySize in single-viewport mode.
//      void          SetCursorScreenPos(const ImVec2& pos);                          // cursor position in absolute coordinates
//      void          AlignTextToFramePadding();                                      // vertically align upcoming text baseline to FramePadding.y so that it will align properly to regularly framed items (call if you have text on a line before a framed item)
//      float         GetTextLineHeight();                                            // ~ font_size
//      float         GetTextLineHeightWithSpacing();                                 // ~ font_size + style.ItemSpacing.y (distance in pixels between 2 consecutive lines of text)
//      float         GetFrameHeight();                                               // ~ font_size + style.FramePadding.y * 2
//      float         GetFrameHeightWithSpacing();                                    // ~ font_size + style.FramePadding.y * 2 + style.ItemSpacing.y (distance in pixels between 2 consecutive lines of framed widgets)
//
//     // ID stack/scopes
//     // Read the FAQ (docs/FAQ.md or http://dearimgui.org/faq) for more details about how ID are handled in dear imgui.
//     // - Those questions are answered and impacted by understanding of the ID stack system:
//     //   - "Q: Why is my widget not reacting when I click on it?"
//     //   - "Q: How can I have widgets with an empty label?"
//     //   - "Q: How can I have multiple widgets with the same label?"
//     // - Short version: ID are hashes of the entire ID stack. If you are creating widgets in a loop you most likely
//     //   want to push a unique identifier (e.g. object pointer, loop index) to uniquely differentiate them.
//     // - You can also use the "Label##foobar" syntax within widget label to distinguish them from each others.
//     // - In this header file we use the "label"/"name" terminology to denote a string that will be displayed + used as an ID,
//     //   whereas "str_id" denote a string that is only used as an ID and not normally displayed.
//      void          PushID(const char* str_id);                                     // push string into the ID stack (will hash string).
//      void          PushID(const char* str_id_begin, const char* str_id_end);       // push string into the ID stack (will hash string).
//      void          PushID(const void* ptr_id);                                     // push pointer into the ID stack (will hash pointer).
//      void          PushID(int int_id);                                             // push integer into the ID stack (will hash integer).
//      void          PopID();                                                        // pop from the ID stack.
//      ImGuiID       GetID(const char* str_id);                                      // calculate unique ID (hash of whole ID stack + given parameter). e.g. if you want to query into ImGuiStorage yourself
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
//     // - For all the Float2/Float3/Float4/Int2/Int3/Int4 versions of every functions, note that a 'float v[X]' function argument is the same as 'float* v',
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
//     // - Note that in C++ a 'float v[X]' function argument is the _same_ as 'float* v', the array syntax is just a way to document the number of elements that are expected to be accessible.
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
//      bool          TreeNode(const char* str_id, const char* fmt, ...) IM_FMTARGS(2);   // helper variation to easily decorelate the id from the displayed string. Read the FAQ about why and how to use ID. to align arbitrary text at the same level as a TreeNode() you can use Bullet().
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
//      bool          CollapsingHeader(const char* label, ImGuiTreeNodeFlags flags = 0);  // if returning 'true' the header is open. doesn't indent nor push on ID stack. user doesn't have to call TreePop().
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
//     //  - IMPORTANT: Popup identifiers are relative to the current ID stack, so OpenPopup and BeginPopup generally needs to be at the same level of the stack.
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
//      bool          BeginPopupContextItem(const char* str_id = NULL, ImGuiPopupFlags popup_flags = 1);  // open+begin popup when clicked on last item. Use str_id==NULL to associate the popup to previous item. If you want to use that on a non-interactive item such as Text() you need to pass in an explicit ID here. read comments in .cpp!
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
//      ImGuiTableColumnFlags TableGetColumnFlags(int column_n = -1);     // return column flags so you can query their Enabled/Visible/Sorted/Hovered status flags. Pass -1 to use current column.
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
//      bool          BeginTabBar(const char* str_id, ImGuiTabBarFlags flags = 0);        // create and append into a TabBar
//      void          EndTabBar();                                                        // only call EndTabBar() if BeginTabBar() returns true!
//      bool          BeginTabItem(const char* label, bool* p_open = NULL, ImGuiTabItemFlags flags = 0); // create a Tab. Returns true if the Tab is selected.
//      void          EndTabItem();                                                       // only call EndTabItem() if BeginTabItem() returns true!
//      bool          TabItemButton(const char* label, ImGuiTabItemFlags flags = 0);      // create a Tab behaving like a button. return true when clicked. cannot be selected in the tab bar.
//      void          SetTabItemClosed(const char* tab_or_docked_window_label);           // notify TabBar or Docking system of a closed tab/window ahead (useful to reduce visual flicker on reorderable tab bars). For tab-bar: call after BeginTabBar() and before Tab submissions. Otherwise call with a window name.
//
//     // Docking
//     // [BETA API] Enable with io.ConfigFlags |= ImGuiConfigFlags_DockingEnable.
//     // Note: You can use most Docking facilities without calling any API. You DO NOT need to call DockSpace() to use Docking!
//     // - Drag from window title bar or their tab to dock/undock. Hold SHIFT to disable docking/undocking.
//     // - Drag from window menu button (upper-left button) to undock an entire node (all windows).
//     // - When io.ConfigDockingWithShift == true, you instead need to hold SHIFT to _enable_ docking/undocking.
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
//      double        GetTime();                                                          // get global imgui time. incremented by io.DeltaTime every frame.
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
//      bool          IsKeyPressed(ImGuiKey key, bool repeat = true);                     // was key pressed (went from !down to down)? if repeat=true, uses io.KeyRepeatDelay / KeyRepeatRate
//      bool          IsKeyReleased(ImGuiKey key);                                        // was key released (went from down to !down)?
//      int           GetKeyPressedAmount(ImGuiKey key, float repeat_delay, float rate);  // uses provided repeat rate/delay. return a count, most often 0 or 1 but might be >1 if RepeatRate is small enough that DeltaTime > RepeatRate
//      const char*   GetKeyName(ImGuiKey key);                                           // [DEBUG] returns English name of the key. Those names a provided for debugging purpose and are not meant to be saved persistently not compared.
//      void          SetNextFrameWantCaptureKeyboard(bool want_capture_keyboard);        // Override io.WantCaptureKeyboard flag next frame (said flag is left for your application to handle, typically when true it instructs your app to ignore inputs). e.g. force capture keyboard when your widget is being hovered. This is equivalent to setting "io.WantCaptureKeyboard = want_capture_keyboard"; after the next NewFrame() call.
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
//      ImVec2        GetMousePos();                                                      // shortcut to ImGui::GetIO().MousePos provided by user, to be consistent with other calls
//      ImVec2        GetMousePosOnOpeningCurrentPopup();                                 // retrieve mouse position at the time of opening popup we have BeginPopup() into (helper to avoid user backing that value themselves)
//      bool          IsMouseDragging(ImGuiMouseButton button, float lock_threshold = -1.0);         // is mouse dragging? (if lock_threshold < -1.0, uses io.MouseDraggingThreshold)
//      ImVec2        GetMouseDragDelta(ImGuiMouseButton button = 0, float lock_threshold = -1.0);   // return the delta from the initial clicking position while the mouse button is pressed or was just released. This is locked and return 0.0 until the mouse moves past a distance threshold at least once (if lock_threshold < -1.0, uses io.MouseDraggingThreshold)
//      void          ResetMouseDragDelta(ImGuiMouseButton button = 0);                   //
//      ImGuiMouseCursor GetMouseCursor();                                                // get desired cursor type, reset in ImGui::NewFrame(), this is updated during the frame. valid before Render(). If you use software rendering by setting io.MouseDrawCursor ImGui will render those for you
//      void          SetMouseCursor(ImGuiMouseCursor cursor_type);                       // set desired cursor type
//      void          SetNextFrameWantCaptureMouse(bool want_capture_mouse);              // Override io.WantCaptureMouse flag next frame (said flag is left for your application to handle, typical when true it instucts your app to ignore inputs). This is equivalent to setting "io.WantCaptureMouse = want_capture_mouse;" after the next NewFrame() call.
//
//     // Clipboard Utilities
//     // - Also see the LogToClipboard() function to capture GUI into clipboard, or easily output text data to the clipboard.
//      const char*   GetClipboardText();
//      void          SetClipboardText(const char* text);
//
//     // Settings/.Ini Utilities
//     // - The disk functions are automatically called if io.IniFilename != NULL (default is "imgui.ini").
//     // - Set io.IniFilename to NULL to load/save manually. Read io.WantSaveIniSettings description about handling .ini saving manually.
//     // - Important: default value "imgui.ini" is relative to current working dir! Most apps will want to lock this to an absolute path (e.g. same path as executables).
//      void          LoadIniSettingsFromDisk(const char* ini_filename);                  // call after CreateContext() and before the first call to NewFrame(). NewFrame() automatically calls LoadIniSettingsFromDisk(io.IniFilename).
//      void          LoadIniSettingsFromMemory(const char* ini_data, size_t ini_size=0); // call after CreateContext() and before the first call to NewFrame() to provide .ini data from your own data source.
//      void          SaveIniSettingsToDisk(const char* ini_filename);                    // this is automatically called (if io.IniFilename is not empty) a few seconds after any modification that should be reflected in the .ini file (and also by DestroyContext).
//      const char*   SaveIniSettingsToMemory(size_t* out_ini_size = NULL);               // return a zero-terminated string with the .ini data which you can save by your own mean. call when io.WantSaveIniSettings is set, then save data by your own mean and clear io.WantSaveIniSettings.
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
//      void              RenderPlatformWindowsDefault(void* platform_render_arg = NULL, void* renderer_render_arg = NULL); // call in main loop. will call RenderWindow/SwapBuffers platform functions for each secondary viewport which doesn't have the ImGuiViewportFlags_Minimized flag set. May be reimplemented by user for custom rendering needs.
//      void              DestroyPlatformWindows();                                       // call DestroyWindow platform functions for all viewports. call from backend Shutdown() if you need to close platform windows before imgui shutdown. otherwise will be called by DestroyContext().
//      ImGuiViewport*    FindViewportByID(ImGuiID id);                                   // this is a helper for backends.
//      ImGuiViewport*    FindViewportByPlatformHandle(void* platform_handle);            // this is a helper for backends. the type platform_handle is decided by the backend (e.g. HWND, MyWindow*, GLFWwindow* etc.)
//
// } // namespace ImGui

//-----------------------------------------------------------------------------
// [SECTION] flags & Enumerations
//-----------------------------------------------------------------------------


//#define ImDrawIdx unsigned int
pub type ImDrawIdx = u32;

// flags for ImGui::Begin()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgWindowFlags
{
    None                   = 0,
    NoTitleBar             = 1 << 0,   // Disable title-bar
    NoResize               = 1 << 1,   // Disable user resizing with the lower-right grip
    NoMove                 = 1 << 2,   // Disable user moving the window
    NoScrollbar            = 1 << 3,   // Disable scrollbars (window can still scroll with mouse or programmatically)
    NoScrollWithMouse      = 1 << 4,   // Disable user vertically scrolling with mouse wheel. On child window, mouse wheel will be forwarded to the parent unless NoScrollbar is also set.
    NoCollapse             = 1 << 5,   // Disable user collapsing window by double-clicking on it. Also referred to as Window Menu Button (e.g. within a docking node).
    AlwaysAutoResize       = 1 << 6,   // Resize every window to its content every frame
    NoBackground           = 1 << 7,   // Disable drawing background color (WindowBg, etc.) and outside border. Similar as using SetNextWindowBgAlpha(0.0).
    NoSavedSettings        = 1 << 8,   // Never load/save settings in .ini file
    NoMouseInputs          = 1 << 9,   // Disable catching mouse, hovering test with pass through.
    MenuBar                = 1 << 10,  // Has a menu-bar
    HorizontalScrollbar    = 1 << 11,  // Allow horizontal scrollbar to appear (off by default). You may use SetNextWindowContentSize(ImVec2(width,0.0)); prior to calling Begin() to specify width. Read code in imgui_demo in the "Horizontal Scrolling" section.
    NoFocusOnAppearing     = 1 << 12,  // Disable taking focus when transitioning from hidden to visible state
    NoBringToFrontOnFocus  = 1 << 13,  // Disable bringing window to front when taking focus (e.g. clicking on it or programmatically giving it focus)
    AlwaysVerticalScrollbar= 1 << 14,  // Always show vertical scrollbar (even if content_size.y < size.y)
    AlwaysHorizontalScrollbar=1<< 15,  // Always show horizontal scrollbar (even if content_size.x < size.x)
    AlwaysUseWindowPadding = 1 << 16,  // Ensure child windows without border uses style.window_padding (ignored by default for non-bordered child windows, because more convenient)
    NoNavInputs            = 1 << 18,  // No gamepad/keyboard navigation within the window
    NoNavFocus             = 1 << 19,  // No focusing toward this window with gamepad/keyboard navigation (e.g. skipped by CTRL+TAB)
    UnsavedDocument        = 1 << 20,  // Display a dot next to the title. When used in a tab/docking context, tab is selected when clicking the X + closure is not assumed (will wait for user to stop submitting the tab). Otherwise closure is assumed when pressing the X, so if you keep submitting the tab may reappear at end of tab bar.
    NoDocking              = 1 << 21,  // Disable docking of this window
    // [Internal]
    NavFlattened           = 1 << 23,  // [BETA] On child window: allow gamepad/keyboard navigation to cross over parent border to this child or between sibling child windows.
    ChildWindow            = 1 << 24,  // Don't use! For internal use by BeginChild()
    Tooltip                = 1 << 25,  // Don't use! For internal use by BeginTooltip()
    Popup                  = 1 << 26,  // Don't use! For internal use by BeginPopup()
    Modal                  = 1 << 27,  // Don't use! For internal use by BeginPopupModal()
    ChildMenu              = 1 << 28,  // Don't use! For internal use by BeginMenu()
    DockNodeHost           = 1 << 29   // Don't use! For internal use by Begin()/NewFrame()
    // [Obsolete]
    //ImGuiWindowFlags_ResizeFromAnySide    = 1 << 17,  // [Obsolete] --> Set io.ConfigWindowsResizeFromEdges=true and make sure mouse cursors are supported by backend (io.BackendFlags & ImGuiBackendFlags_HasMouseCursors)
}

// ImGuiWindowFlags_NoNav                  = ImGuiWindowFlags_NoNavInputs | ImGuiWindowFlags_NoNavFocus,
// pub const ImGuiWindowFlags_NoNav: i32 = DimgWindowFlags::NoNavInputs | DimgWindowFlags::NoNavFocus;
pub const DIMG_WIN_FLAGS_NO_NAV: HashSet<DimgWindowFlags> = HashSet::from([DimgWindowFlags::NoNavInputs, DimgWindowFlags::NoNavFocus]);

//     ImGuiWindowFlags_NoDecoration           = ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoCollapse,
// pub const NoDecoration: i32 = DimgWindowFlags::NoTitleBar | DimgWindowFlags::NoResize | DimgWindowFlags::NoScrollbar | DimgWindowFlags::NoCollapse;
pub const DIMG_WIN_FLAGS_NO_DECORATION: HashSet<DimgWindowFlags> = HashSet::from([
    DimgWindowFlags::NoTitleBar, DimgWindowFlags::NoResize, DimgWindowFlags::NoScrollbar, DimgWindowFlags::NoCollapse
]);

//     ImGuiWindowFlags_NoInputs               = ImGuiWindowFlags_NoMouseInputs | ImGuiWindowFlags_NoNavInputs | ImGuiWindowFlags_NoNavFocus,
// pub const NoInputs: i32 = DimgWindowFlags::NoMouseInputs | DimgWindowFlags::NoNavInputs | DimgWindowFlags::NoNavFocus;
pub const DIMG_WIN_FLAGS_NO_INPUTS: HashSet<DimgWindowFlags> = HashSet::from([
    DimgWindowFlags::NoMouseInputs, DimgWindowFlags::NoNavInputs, DimgWindowFlags::NoNavFocus
]);

// flags for ImGui::InputText()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum ImGuiInputTextFlags
{
    None= 0,
    CharsDecimal = 1 << 0,   // Allow 0123456789.+-*/
    CharsHexadecimal = 1 << 1,   // Allow 0123456789ABCDEFabcdef
    CharsUppercase = 1 << 2,   // Turn a..z into A..Z
    CharsNoBlank = 1 << 3,   // Filter out spaces, tabs
    AutoSelectAll = 1 << 4,   // Select entire text when first taking mouse focus
    EnterReturnsTrue = 1 << 5,   // Return 'true' when Enter is pressed (as opposed to every time the value was modified). Consider looking at the IsItemDeactivatedAfterEdit() function.
    CallbackCompletion = 1 << 6,   // Callback on pressing TAB (for completion handling)
    CallbackHistory = 1 << 7,   // Callback on pressing Up/down arrows (for history handling)
    CallbackAlways = 1 << 8,   // Callback on each iteration. User code may query cursor position, modify text buffer.
    CallbackCharFilter = 1 << 9,   // Callback on character inputs to replace or discard them. Modify 'EventChar' to replace or discard, or return 1 in callback to discard.
    AllowTabInput = 1 << 10,  // Pressing TAB input a '\t' character into the text field
    CtrlEnterForNewLine = 1 << 11,  // In multi-line mode, unfocus with Enter, add new line with Ctrl+Enter (default is opposite = unfocus with Ctrl+Enter, add line with Enter).
    NoHorizontalScroll = 1 << 12,  // Disable following the cursor horizontally
    AlwaysOverwrite = 1 << 13,  // Overwrite mode
    ReadOnly = 1 << 14,  // Read-only mode
    Password = 1 << 15,  // Password mode, display all characters as '*'
    NoUndoRedo = 1 << 16,  // Disable undo/redo. Note that input text owns the text data while active, if you want to provide your own undo/redo stack you need e.g. to call ClearActiveID().
    CharsScientific = 1 << 17,  // Allow 0123456789.+-*/eE (Scientific notation input)
    CallbackResize = 1 << 18,  // Callback on buffer capacity changes request (beyond 'buf_size' parameter value), allowing the string to grow. Notify when the string wants to be resized (for string types which hold a cache of their size). You will be provided a new BufSize in the callback and NEED to honor it. (see misc/cpp/imgui_stdlib.h for an example of using this)
    CallbackEdit = 1 << 19,   // Callback on any edit (note that InputText() already returns true on edit, the callback is useful mainly to manipulate the underlying buffer while focus is active)
    // Obsolete names (will be removed soon)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     , ImGuiInputTextFlags_AlwaysInsertMode    = ImGuiInputTextFlags_AlwaysOverwrite   // [renamed in 1.82] name was not matching behavior
// #endif
}

// flags for ImGui::TreeNodeEx(), ImGui::CollapsingHeader*()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgTreeNodeFlags
{
    None                 = 0,
    Selected             = 1 << 0,   // Draw as selected
    Framed               = 1 << 1,   // Draw frame with background (e.g. for CollapsingHeader)
    AllowItemOverlap     = 1 << 2,   // Hit testing to allow subsequent widgets to overlap this one
    NoTreePushOnOpen     = 1 << 3,   // Don't do a TreePush() when open (e.g. for CollapsingHeader) = no extra indent nor pushing on ID stack
    NoAutoOpenOnLog      = 1 << 4,   // Don't automatically and temporarily open node when Logging is active (by default logging will automatically open tree nodes)
    DefaultOpen          = 1 << 5,   // Default node to be open
    OpenOnDoubleClick    = 1 << 6,   // Need double-click to open node
    OpenOnArrow          = 1 << 7,   // Only open when clicking on the arrow part. If ImGuiTreeNodeFlags_OpenOnDoubleClick is also set, single-click arrow or double-click all box to open.
    Leaf                 = 1 << 8,   // No collapsing, no arrow (use as a convenience for leaf nodes).
    Bullet               = 1 << 9,   // Display a bullet instead of arrow
    FramePadding         = 1 << 10,  // Use FramePadding (even for an unframed text node) to vertically align text baseline to regular widget height. Equivalent to calling AlignTextToFramePadding().
    SpanAvailWidth       = 1 << 11,  // Extend hit box to the right-most edge, even if not framed. This is not the default in order to allow adding other items on the same line. In the future we may refactor the hit system to be front-to-back, allowing natural overlaps and then this can become the default.
    SpanFullWidth        = 1 << 12,  // Extend hit box to the left-most and right-most edges (bypass the indented area).
    NavLeftJumpsBackHere = 1 << 13,  // (WIP) Nav: left direction may move to this TreeNode() from any of its child (items submitted between TreeNode and TreePop)
    //ImGuiTreeNodeFlags_NoScrollOnOpen     = 1 << 14,  // FIXME: TODO: Disable automatic scroll on TreePop() if node got just open and contents is not visible
    // ImGuiTreeNodeFlags_CollapsingHeader     = ImGuiTreeNodeFlags_Framed | ImGuiTreeNodeFlags_NoTreePushOnOpen | ImGuiTreeNodeFlags_NoAutoOpenOnLog
}


// pub const ImGuiTreeNodeFlags_CollapsingHeader: i32     = ImGuiTreeNodeFlags::ImGuiTreeNodeFlags_Framed | ImGuiTreeNodeFlags::ImGuiTreeNodeFlags_NoTreePushOnOpen | ImGuiTreeNodeFlags::ImGuiTreeNodeFlags_NoAutoOpenOnLog;
pub const TREE_NODE_FLAGS_COLLAPSING_HDR: HashSet<DimgTreeNodeFlags> = HashSet::from([
    DimgTreeNodeFlags::Framed, DimgTreeNodeFlags::NoTreePushOnOpen, DimgTreeNodeFlags::NoAutoOpenOnLog
]);

// flags for OpenPopup*(), BeginPopupContext*(), IsPopupOpen() functions.
// - To be backward compatible with older API which took an 'int mouse_button = 1' argument, we need to treat
//   small flags values as a mouse button index, so we encode the mouse button in the first few bits of the flags.
//   It is therefore guaranteed to be legal to pass a mouse button index in ImGuiPopupFlags.
// - For the same reason, we exceptionally default the ImGuiPopupFlags argument of BeginPopupContextXXX functions to 1 instead of 0.
//   IMPORTANT: because the default parameter is 1 (==ImGuiPopupFlags_MouseButtonRight), if you rely on the default parameter
//   and want to another another flag, you need to pass in the ImGuiPopupFlags_MouseButtonRight flag.
// - Multiple buttons currently cannot be combined/or-ed in those functions (we could allow it later).
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgPopupFlags
{
    None                    = 0,
    // ImGuiPopupFlags_MouseButtonLeft         = 0,        // For BeginPopupContext*(): open on Left Mouse release. Guaranteed to always be == 0 (same as ImGuiMouseButton_Left)
    MouseButtonRight        = 1,        // For BeginPopupContext*(): open on Right Mouse release. Guaranteed to always be == 1 (same as ImGuiMouseButton_Right)
    MouseButtonMiddle       = 2,        // For BeginPopupContext*(): open on Middle Mouse release. Guaranteed to always be == 2 (same as ImGuiMouseButton_Middle)
    MouseButtonMask_        = 0x1F,
    // ImGuiPopupFlags_MouseButtonDefault_     = 1,
    NoOpenOverExistingPopup = 1 << 5,   // For OpenPopup*(), BeginPopupContext*(): don't open if there's already a popup at the same level of the popup stack
    NoOpenOverItems         = 1 << 6,   // For BeginPopupContextWindow(): don't return true when hovering items, only when hovering empty space
    AnyPopupId              = 1 << 7,   // For IsPopupOpen(): ignore the ImGuiID parameter and test for any popup.
    AnyPopupLevel           = 1 << 8,   // For IsPopupOpen(): search/test at any level of the popup stack (default test in the current level)

}

pub const MouseButtonLeft: i32         = 0;

pub const MouseButtonDefault: i32      = 1;

// pub const AnyPopup: i32                = DimgPopupFlags::AnyPopupId | DimgPopupFlags::AnyPopupLevel;
pub const DIMG_POPUP_FLAGS_ANY_POPUP: HashSet<DimgPopupFlags> = HashSet::from([
    DimgPopupFlags::AnyPopupId, DimgPopupFlags::AnyPopupLevel
]);

// flags for ImGui::Selectable()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgSelectableFlags
{
    None               = 0,
    DontClosePopups    = 1 << 0,   // Clicking this don't close parent popup window
    SpanAllColumns     = 1 << 1,   // Selectable frame can span all columns (text will still fit in current column)
    AllowDoubleClick   = 1 << 2,   // Generate press events on double clicks too
    Disabled           = 1 << 3,   // Cannot be selected, display grayed out text
    AllowItemOverlap   = 1 << 4    // (WIP) Hit testing to allow subsequent widgets to overlap this one
}


// flags for ImGui::BeginCombo()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgComboFlags
{
    None                    = 0,
    PopupAlignLeft          = 1 << 0,   // Align the popup toward the left by default
    HeightSmall             = 1 << 1,   // Max ~4 items visible. Tip: If you want your combo popup to be a specific size you can use SetNextWindowSizeConstraints() prior to calling BeginCombo()
    HeightRegular           = 1 << 2,   // Max ~8 items visible (default)
    HeightLarge             = 1 << 3,   // Max ~20 items visible
    HeightLargest           = 1 << 4,   // As many fitting items as possible
    NoArrowButton           = 1 << 5,   // Display on the preview box without the square arrow button
    NoPreview               = 1 << 6,   // Display only a square arrow button
    // ImGuiComboFlags_HeightMask_             = ImGuiComboFlags_HeightSmall | ImGuiComboFlags_HeightRegular | ImGuiComboFlags_HeightLarge | ImGuiComboFlags_HeightLargest
}


// pub const HeightMask: i32             = DimgComboFlags::HeightSmall | DimgComboFlags::HeightRegular | DimgComboFlags::HeightLarge | DimgComboFlags::HeightLargest;
pub const HEIGHT_MASK: HashSet<DimgComboFlags> = HashSet::from([
    DimgComboFlags::HeightSmall, DimgComboFlags::HeightRegular, DimgComboFlags::HeightLarge, DimgComboFlags::HeightLargest
]);

// flags for ImGui::BeginTabBar()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgTabBarFlags
{
    None                           = 0,
    Reorderable                    = 1 << 0,   // Allow manually dragging tabs to re-order them + New tabs are appended at the end of list
    AutoSelectNewTabs              = 1 << 1,   // Automatically select new tabs when they appear
    TabListPopupButton             = 1 << 2,   // Disable buttons to open the tab list popup
    NoCloseWithMiddleMouseButton   = 1 << 3,   // Disable behavior of closing tabs (that are submitted with p_open != NULL) with middle mouse button. You can still repro this behavior on user's side with if (IsItemHovered() && IsMouseClicked(2)) *p_open = false.
    NoTabListScrollingButtons      = 1 << 4,   // Disable scrolling buttons (apply when fitting policy is ImGuiTabBarFlags_FittingPolicyScroll)
    NoTooltip                      = 1 << 5,   // Disable tooltips when hovering a tab
    FittingPolicyResizeDown        = 1 << 6,   // Resize tabs when they don't fit
    FittingPolicyScroll            = 1 << 7,   // Add scroll buttons when tabs don't fit
    // ImGuiTabBarFlags_FittingPolicyMask_             = ImGuiTabBarFlags_FittingPolicyResizeDown | ImGuiTabBarFlags_FittingPolicyScroll,
    // ImGuiTabBarFlags_FittingPolicyDefault_          = ImGuiTabBarFlags_FittingPolicyResizeDown
}


// pub const FittingPolicyMask_ : i32            = DimgTabBarFlags::FittingPolicyResizeDown | DimgTabBarFlags::FittingPolicyScroll;
pub const FITTING_POLICY_MASK: HashSet<DimgTabBarFlags> = HashSet::from([
    DimgTabBarFlags::FittingPolicyResizeDown, DimgTabBarFlags::FittingPolicyScroll
]);

// pub const     FittingPolicyDefault_: i32          = DimgTabBarFlags::FittingPolicyResizeDown as i32;
pub const FITTING_POLICY_DFLT: DimgTabBarFlags = DimgTabBarFlags::FittingPolicyResizeDown;



// flags for ImGui::BeginTable()
// - Important! Sizing policies have complex and subtle side effects, much more so than you would expect.
//   Read comments/demos carefully + experiment with live demos to get acquainted with them.
// - The DEFAULT sizing policies are:
//    - Default to ImGuiTableFlags_SizingFixedFit    if ScrollX is on, or if host window has ImGuiWindowFlags_AlwaysAutoResize.
//    - Default to ImGuiTableFlags_SizingStretchSame if ScrollX is off.
// - When ScrollX is off:
//    - Table defaults to ImGuiTableFlags_SizingStretchSame -> all Columns defaults to ImGuiTableColumnFlags_WidthStretch with same weight.
//    - Columns sizing policy allowed: Stretch (default), Fixed/Auto.
//    - Fixed Columns (if any) will generally obtain their requested width (unless the table cannot fit them all).
//    - Stretch Columns will share the remaining width according to their respective weight.
//    - Mixed Fixed/Stretch columns is possible but has various side-effects on resizing behaviors.
//      The typical use of mixing sizing policies is: any number of LEADING Fixed columns, followed by one or two TRAILING Stretch columns.
//      (this is because the visible order of columns have subtle but necessary effects on how they react to manual resizing).
// - When ScrollX is on:
//    - Table defaults to ImGuiTableFlags_SizingFixedFit -> all Columns defaults to ImGuiTableColumnFlags_WidthFixed
//    - Columns sizing policy allowed: Fixed/Auto mostly.
//    - Fixed Columns can be enlarged as needed. Table will show an horizontal scrollbar if needed.
//    - When using auto-resizing (non-resizable) fixed columns, querying the content width to use item right-alignment e.g. SetNextItemWidth(-FLT_MIN) doesn't make sense, would create a feedback loop.
//    - Using Stretch columns OFTEN DOES NOT MAKE SENSE if ScrollX is on, UNLESS you have specified a value for 'inner_width' in BeginTable().
//      If you specify a value for 'inner_width' then effectively the scrolling space is known and Stretch or mixed Fixed/Stretch columns become meaningful again.
// - Read on documentation at the top of imgui_tables.cpp for details.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgTableFlags
{
    // Features
    None                       = 0,
    Resizable                  = 1 << 0,   // Enable resizing columns.
    Reorderable                = 1 << 1,   // Enable reordering columns in header row (need calling TableSetupColumn() + TableHeadersRow() to display headers)
    Hideable                   = 1 << 2,   // Enable hiding/disabling columns in context menu.
    Sortable                   = 1 << 3,   // Enable sorting. Call TableGetSortSpecs() to obtain sort specs. Also see ImGuiTableFlags_SortMulti and ImGuiTableFlags_SortTristate.
    NoSavedSettings            = 1 << 4,   // Disable persisting columns order, width and sort settings in the .ini file.
    ContextMenuInBody          = 1 << 5,   // Right-click on columns body/contents will display table context menu. By default it is available in TableHeadersRow().
    // Decorations
    RowBg                      = 1 << 6,   // Set each RowBg color with ImGuiCol_TableRowBg or ImGuiCol_TableRowBgAlt (equivalent of calling TableSetBgColor with ImGuiTableBgFlags_RowBg0 on each row manually)
    BordersInnerH              = 1 << 7,   // Draw horizontal borders between rows.
    BordersOuterH              = 1 << 8,   // Draw horizontal borders at the top and bottom.
    BordersInnerV              = 1 << 9,   // Draw vertical borders between columns.
    BordersOuterV              = 1 << 10,  // Draw vertical borders on the left and right sides.
    // ImGuiTableFlags_BordersH                   = ImGuiTableFlags_BordersInnerH | ImGuiTableFlags_BordersOuterH, // Draw horizontal borders.
    // ImGuiTableFlags_BordersV                   = ImGuiTableFlags_BordersInnerV | ImGuiTableFlags_BordersOuterV, // Draw vertical borders.
    // ImGuiTableFlags_BordersInner               = ImGuiTableFlags_BordersInnerV | ImGuiTableFlags_BordersInnerH, // Draw inner borders.
    // ImGuiTableFlags_BordersOuter               = ImGuiTableFlags_BordersOuterV | ImGuiTableFlags_BordersOuterH, // Draw outer borders.
    // ImGuiTableFlags_Borders                    = ImGuiTableFlags_BordersInner | ImGuiTableFlags_BordersOuter,   // Draw all borders.
    NoBordersInBody            = 1 << 11,  // [ALPHA] Disable vertical borders in columns Body (borders will always appears in Headers). -> May move to style
    NoBordersInBodyUntilResize = 1 << 12,  // [ALPHA] Disable vertical borders in columns Body until hovered for resize (borders will always appears in Headers). -> May move to style
    // Sizing Policy (read above for defaults)
    SizingFixedFit             = 1 << 13,  // Columns default to _WidthFixed or _WidthAuto (if resizable or not resizable), matching contents width.
    SizingFixedSame            = 2 << 13,  // Columns default to _WidthFixed or _WidthAuto (if resizable or not resizable), matching the maximum contents width of all columns. Implicitly enable ImGuiTableFlags_NoKeepColumnsVisible.
    SizingStretchProp          = 3 << 13,  // Columns default to _WidthStretch with default weights proportional to each columns contents widths.
    SizingStretchSame          = 4 << 13,  // Columns default to _WidthStretch with default weights all equal, unless overridden by TableSetupColumn().
    // Sizing Extra Options
    NoHostExtendX              = 1 << 16,  // Make outer width auto-fit to columns, overriding outer_size.x value. Only available when ScrollX/ScrollY are disabled and Stretch columns are not used.
    NoHostExtendY              = 1 << 17,  // Make outer height stop exactly at outer_size.y (prevent auto-extending table past the limit). Only available when ScrollX/ScrollY are disabled. data below the limit will be clipped and not visible.
    NoKeepColumnsVisible       = 1 << 18,  // Disable keeping column always minimally visible when ScrollX is off and table gets too small. Not recommended if columns are resizable.
    PreciseWidths              = 1 << 19,  // Disable distributing remainder width to stretched columns (width allocation on a 100-wide table with 3 columns: Without this flag: 33,33,34. With this flag: 33,33,33). With larger number of columns, resizing will appear to be less smooth.
    // Clipping
    NoClip                     = 1 << 20,  // Disable clipping rectangle for every individual columns (reduce draw command count, items will be able to overflow into other columns). Generally incompatible with TableSetupScrollFreeze().
    // Padding
    PadOuterX                  = 1 << 21,  // Default if BordersOuterV is on. Enable outer-most padding. Generally desirable if you have headers.
    NoPadOuterX                = 1 << 22,  // Default if BordersOuterV is off. Disable outer-most padding.
    NoPadInnerX                = 1 << 23,  // Disable inner padding between columns (double inner padding if BordersOuterV is on, single inner padding if BordersOuterV is off).
    // Scrolling
    ScrollX                    = 1 << 24,  // Enable horizontal scrolling. Require 'outer_size' parameter of BeginTable() to specify the container size. Changes default sizing policy. Because this create a child window, ScrollY is currently generally recommended when using ScrollX.
    ScrollY                    = 1 << 25,  // Enable vertical scrolling. Require 'outer_size' parameter of BeginTable() to specify the container size.
    // Sorting
    SortMulti                  = 1 << 26,  // Hold shift when clicking headers to sort on multiple column. TableGetSortSpecs() may return specs where (specs_count > 1).
    SortTristate               = 1 << 27,  // Allow no sorting, disable default sorting. TableGetSortSpecs() may return specs where (specs_count == 0).

    // [Internal] Combinations and masks
    // ImGuiTableFlags_SizingMask_                = ImGuiTableFlags_SizingFixedFit | ImGuiTableFlags_SizingFixedSame | ImGuiTableFlags_SizingStretchProp | ImGuiTableFlags_SizingStretchSame

    // Obsolete names (will be removed soon)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     //, ImGuiTableFlags_ColumnsWidthFixed = ImGuiTableFlags_SizingFixedFit, ImGuiTableFlags_ColumnsWidthStretch = ImGuiTableFlags_SizingStretchSame   // WIP tables 2020/12
//     //, ImGuiTableFlags_SizingPolicyFixed = ImGuiTableFlags_SizingFixedFit, ImGuiTableFlags_SizingPolicyStretch = ImGuiTableFlags_SizingStretchSame   // WIP tables 2021/01
// #endif
}


// pub const BordersH: i32                   = DimgTableFlags::BordersInnerH | DimgTableFlags::BordersOuterH;
pub const BORDERS_H: HashSet<DimgTableFlags> = HashSet::from([
    DimgTableFlags::BordersInnerH, DimgTableFlags::BordersOuterH
]) ;

// pub const     BordersV : i32                  = DimgTableFlags::BordersInnerV | DimgTableFlags::BordersOuterV;
pub const BORDERS_V: HashSet<DimgTableFlags> = HashSet::from([
    DimgTableFlags::BordersInnerV, DimgTableFlags::BordersOuterV
]);

// pub const     BordersInner     : i32          = DimgTableFlags::BordersInnerV | DimgTableFlags::BordersInnerH;
pub const BORDERS_INNER: HashSet<DimgTableFlags> = HashSet::from(
    [
        DimgTableFlags::BordersInnerV, DimgTableFlags::BordersInnerH
    ]
);

// pub const     BordersOuter  : i32             = DimgTableFlags::BordersOuterV | DimgTableFlags::BordersOuterH;
pub const BORDERS_OUTER: HashSet<DimgTableFlags> = HashSet::from([
    DimgTableFlags::BordersOuterV, DimgTableFlags::BordersOuterH
]);

// pub const     Borders    : i32                = BordersInner | BordersOuter;
pub const BORDERS: HashSet<DimgTableFlags> = BORDERS_INNER.union(&BORDERS_OUTER).cloned().collect();

// pub const SizingMask_: i32                 = DimgTableFlags::SizingFixedFit | DimgTableFlags::SizingFixedSame | DimgTableFlags::SizingStretchProp | DimgTableFlags::SizingStretchSame;
pub const SIZING_MASK: HashSet<DimgTableFlags> = HashSet::from([
    DimgTableFlags::SizingFixedFit, DimgTableFlags::SizingFixedSame, DimgTableFlags::SizingStretchProp, DimgTableFlags::SizingStretchSame
]);

// flags for ImGui::TableSetupColumn()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgTableColumnFlags
{
    // Input configuration flags
    None                  = 0,
    Disabled              = 1 << 0,   // Overriding/master disable flag: hide column, won't show in context menu (unlike calling TableSetColumnEnabled() which manipulates the user accessible state)
    DefaultHide           = 1 << 1,   // Default as a hidden/disabled column.
    DefaultSort           = 1 << 2,   // Default as a sorting column.
    WidthStretch          = 1 << 3,   // column will stretch. Preferable with horizontal scrolling disabled (default if table sizing policy is _SizingStretchSame or _SizingStretchProp).
    WidthFixed            = 1 << 4,   // column will not stretch. Preferable with horizontal scrolling enabled (default if table sizing policy is _SizingFixedFit and table is resizable).
    NoResize              = 1 << 5,   // Disable manual resizing.
    NoReorder             = 1 << 6,   // Disable manual reordering this column, this will also prevent other columns from crossing over this column.
    NoHide                = 1 << 7,   // Disable ability to hide/disable this column.
    NoClip                = 1 << 8,   // Disable clipping for this column (all NoClip columns will render in a same draw command).
    NoSort                = 1 << 9,   // Disable ability to sort on this field (even if ImGuiTableFlags_Sortable is set on the table).
    NoSortAscending       = 1 << 10,  // Disable ability to sort in the ascending direction.
    NoSortDescending      = 1 << 11,  // Disable ability to sort in the descending direction.
    NoHeaderLabel         = 1 << 12,  // TableHeadersRow() will not submit label for this column. Convenient for some small columns. Name will still appear in context menu.
    NoHeaderWidth         = 1 << 13,  // Disable header text width contribution to automatic column width.
    PreferSortAscending   = 1 << 14,  // Make the initial sort direction Ascending when first sorting on this column (default).
    PreferSortDescending  = 1 << 15,  // Make the initial sort direction Descending when first sorting on this column.
    IndentEnable          = 1 << 16,  // Use current Indent value when entering cell (default for column 0).
    IndentDisable         = 1 << 17,  // Ignore current Indent value when entering cell (default for columns > 0). Indentation changes _within_ the cell will still be honored.

    // Output status flags, read-only via TableGetColumnFlags()
    IsEnabled             = 1 << 24,  // Status: is enabled == not hidden by user/api (referred to as "Hide" in _DefaultHide and _NoHide) flags.
    IsVisible             = 1 << 25,  // Status: is visible == is enabled AND not clipped by scrolling.
    IsSorted              = 1 << 26,  // Status: is currently part of the sort specs
    IsHovered             = 1 << 27,  // Status: is hovered by mouse

    // [Internal] Combinations and masks
    // ImGuiTableColumnFlags_WidthMask_            = ImGuiTableColumnFlags_WidthStretch | ImGuiTableColumnFlags_WidthFixed,
    // ImGuiTableColumnFlags_IndentMask_           = ImGuiTableColumnFlags_IndentEnable | ImGuiTableColumnFlags_IndentDisable,
    // ImGuiTableColumnFlags_StatusMask_           = ImGuiTableColumnFlags_IsEnabled | ImGuiTableColumnFlags_IsVisible | ImGuiTableColumnFlags_IsSorted | ImGuiTableColumnFlags_IsHovered,
    NoDirectResize_       = 1 << 30   // [Internal] Disable user resizing this column directly (it may however we resized indirectly from its left edge)

    // Obsolete names (will be removed soon)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     //ImGuiTableColumnFlags_WidthAuto           = ImGuiTableColumnFlags_WidthFixed | ImGuiTableColumnFlags_NoResize, // column will not stretch and keep resizing based on submitted contents.
// #endif
}


// pub const WidthMask_ : i32           = DimgTableColumnFlags::WidthStretch | DimgTableColumnFlags::WidthFixed;
pub const WIDTH_MASK: HashSet<DimgTableColumnFlags> = HashSet::from([
    DimgTableColumnFlags::WidthStretch, DimgTableColumnFlags::WidthFixed
]);

// pub const     IndentMask_ : i32          = DimgTableColumnFlags::IndentEnable | DimgTableColumnFlags::IndentDisable;
pub const INDENT_MASK: HashSet<DimgTableColumnFlags> = HashSet::from([
    DimgTableColumnFlags::IndentEnable, DimgTableColumnFlags::IndentDisable
]);

    // pub const StatusMask_      : i32     = DimgTableColumnFlags::IsEnabled | DimgTableColumnFlags::IsVisible | DimgTableColumnFlags::IsSorted | DimgTableColumnFlags::IsHovered;

pub const STATUS_MASK: HashSet<DimgTableColumnFlags> = HashSet::from([
    DimgTableColumnFlags::IsEnabled, DimgTableColumnFlags::IsVisible, DimgTableColumnFlags::IsSorted, DimgTableColumnFlags::IsHovered
]);

// flags for ImGui::TableNextRow()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgTableRowFlags
{
    None                         = 0,
    Headers                      = 1 << 0    // Identify header row (set default background color + width of its contents accounted differently for auto column width)
}

// Enum for ImGui::TableSetBgColor()
// Background colors are rendering in 3 layers:
//  - Layer 0: draw with RowBg0 color if set, otherwise draw with ColumnBg0 if set.
//  - Layer 1: draw with RowBg1 color if set, otherwise draw with ColumnBg1 if set.
//  - Layer 2: draw with CellBg color if set.
// The purpose of the two row/columns layers is to let you decide if a background color changes should override or blend with the existing color.
// When using ImGuiTableFlags_RowBg on the table, each row has the RowBg0 color automatically set for odd/even rows.
// If you set the color of RowBg0 target, your color will override the existing RowBg0 color.
// If you set the color of RowBg1 or ColumnBg1 target, your color will blend over the RowBg0 color.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgTableBgTarget
{
    None                         = 0,
    RowBg0                       = 1,        // Set row background color 0 (generally used for background, automatically set when ImGuiTableFlags_RowBg is used)
    RowBg1                       = 2,        // Set row background color 1 (generally used for selection marking)
    CellBg                       = 3         // Set cell background color (top-most color)
}

// flags for ImGui::IsWindowFocused()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgFocusedFlags
{
    None                          = 0,
    ChildWindows                  = 1 << 0,   // Return true if any children of the window is focused
    RootWindow                    = 1 << 1,   // Test from root window (top most parent of the current hierarchy)
    AnyWindow                     = 1 << 2,   // Return true if any window is focused. Important: If you are trying to tell how to dispatch your low-level inputs, do NOT use this. Use 'io.WantCaptureMouse' instead! Please read the FAQ!
    NoPopupHierarchy              = 1 << 3,   // Do not consider popup hierarchy (do not treat popup emitter as parent of popup) (when used with _ChildWindows or _RootWindow)
    DockHierarchy                 = 1 << 4,   // Consider docking hierarchy (treat dockspace host as parent of docked window) (when used with _ChildWindows or _RootWindow)
    // ImGuiFocusedFlags_RootAndChildWindows           = ImGuiFocusedFlags_RootWindow | ImGuiFocusedFlags_ChildWindows
}


// pub const RootAndChildWindows: i32           = DimgFocusedFlags::RootWindow | DimgFocusedFlags::ChildWindows;



// flags for ImGui::IsItemHovered(), ImGui::IsWindowHovered()
// Note: if you are trying to check whether your mouse should be dispatched to Dear ImGui or to your app, you should use 'io.WantCaptureMouse' instead! Please read the FAQ!
// Note: windows with the ImGuiWindowFlags_NoInputs flag are ignored by IsWindowHovered() calls.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgHoveredFlags
{
    None                          = 0,        // Return true if directly over the item/window, not obstructed by another window, not obstructed by an active popup or modal blocking inputs under them.
    ChildWindows                  = 1 << 0,   // IsWindowHovered() only: Return true if any children of the window is hovered
    RootWindow                    = 1 << 1,   // IsWindowHovered() only: Test from root window (top most parent of the current hierarchy)
    AnyWindow                     = 1 << 2,   // IsWindowHovered() only: Return true if any window is hovered
    NoPopupHierarchy              = 1 << 3,   // IsWindowHovered() only: Do not consider popup hierarchy (do not treat popup emitter as parent of popup) (when used with _ChildWindows or _RootWindow)
    DockHierarchy                 = 1 << 4,   // IsWindowHovered() only: Consider docking hierarchy (treat dockspace host as parent of docked window) (when used with _ChildWindows or _RootWindow)
    AllowWhenBlockedByPopup       = 1 << 5,   // Return true even if a popup window is normally blocking access to this item/window
    //ImGuiHoveredFlags_AllowWhenBlockedByModal     = 1 << 6,   // Return true even if a modal popup window is normally blocking access to this item/window. FIXME-TODO: Unavailable yet.
    AllowWhenBlockedByActiveItem  = 1 << 7,   // Return true even if an active item is blocking access to this item/window. Useful for Drag and Drop patterns.
    AllowWhenOverlapped           = 1 << 8,   // IsItemHovered() only: Return true even if the position is obstructed or overlapped by another window
    AllowWhenDisabled             = 1 << 9,   // IsItemHovered() only: Return true even if the item is disabled
    NoNavOverride                 = 1 << 10,  // Disable using gamepad/keyboard navigation state when active, always query mouse.
    // ImGuiHoveredFlags_RectOnly                      = ImGuiHoveredFlags_AllowWhenBlockedByPopup | ImGuiHoveredFlags_AllowWhenBlockedByActiveItem | ImGuiHoveredFlags_AllowWhenOverlapped,
    // ImGuiHoveredFlags_RootAndChildWindows           = ImGuiHoveredFlags_RootWindow | ImGuiHoveredFlags_ChildWindows
}

 
 // pub const RectOnly : i32                     = DimgHoveredFlags::AllowWhenBlockedByPopup | DimgHoveredFlags::AllowWhenBlockedByActiveItem | DimgHoveredFlags::AllowWhenOverlapped;
 pub const RECT_ONLY: HashSet<DimgHoveredFlags> = HashSet::from([
     DimgHoveredFlags::AllowWhenBlockedByPopup, DimgHoveredFlags::AllowWhenBlockedByActiveItem, DimgHoveredFlags::AllowWhenOverlapped
 ]);

    // pub const RootAndChildWindows: i32           = DimgHoveredFlags::RootWindow | DimgHoveredFlags::ChildWindows;
pub const ROOT_AND_CHILD_WINDOWS: HashSet<DimgHoveredFlags> = HashSet::from([
        DimgHoveredFlags::RootWindow, DimgHoveredFlags::ChildWindows
    ]);

// flags for ImGui::BeginDragDropSource(), ImGui::AcceptDragDropPayload()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgDragDropFlags
{
    None                         = 0,
    // BeginDragDropSource() flags
    SourceNoPreviewTooltip       = 1 << 0,   // By default, a successful call to BeginDragDropSource opens a tooltip so you can display a preview or description of the source contents. This flag disable this behavior.
    SourceNoDisableHover         = 1 << 1,   // By default, when dragging we clear data so that IsItemHovered() will return false, to avoid subsequent user code submitting tooltips. This flag disable this behavior so you can still call IsItemHovered() on the source item.
    SourceNoHoldToOpenOthers     = 1 << 2,   // Disable the behavior that allows to open tree nodes and collapsing header by holding over them while dragging a source item.
    SourceAllowNullID            = 1 << 3,   // Allow items such as Text(), Image() that have no unique identifier to be used as drag source, by manufacturing a temporary identifier based on their window-relative position. This is extremely unusual within the dear imgui ecosystem and so we made it explicit.
    SourceExtern                 = 1 << 4,   // External source (from outside of dear imgui), won't attempt to read current item/window info. Will always return true. Only one Extern source can be active simultaneously.
    SourceAutoExpirePayload      = 1 << 5,   // Automatically expire the payload if the source cease to be submitted (otherwise payloads are persisting while being dragged)
    // AcceptDragDropPayload() flags
    AcceptBeforeDelivery         = 1 << 10,  // AcceptDragDropPayload() will returns true even before the mouse button is released. You can then call is_delivery() to test if the payload needs to be delivered.
    AcceptNoDrawDefaultRect      = 1 << 11,  // Do not draw the default highlight rectangle when hovering over target.
    AcceptNoPreviewTooltip       = 1 << 12,  // Request hiding the BeginDragDropSource tooltip from the BeginDragDropTarget site.
    // AcceptPeekOnly               = AcceptBeforeDelivery | AcceptNoDrawDefaultRect  // For peeking ahead and inspecting the payload before delivery.
}


// pub const AcceptPeekOnly: i32               = DimgDragDropFlags::AcceptBeforeDelivery | DimgDragDropFlags::AcceptNoDrawDefaultRect;
pub const ACCEPT_PEEK_ONLY: HashSet<DimgDragDropFlags> = HashSet::from([
    DimgDragDropFlags::AcceptBeforeDelivery, DimgDragDropFlags::AcceptNoDrawDefaultRect
]);

// Standard Drag and Drop payload types. You can define you own payload types using short strings. Types starting with '_' are defined by Dear ImGui.
pub const IMGUI_PAYLOAD_TYPE_COLOR_3F: String =     String::from("_COL3F");    // float[3]: Standard type for colors, without alpha. User code may use this type.
pub const IMGUI_PAYLOAD_TYPE_COLOR_4F: String =     String::from("_COL4F");    // float[4]: Standard type for colors. User code may use this type.

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

// A cardinal direction
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgDir
{
    None    ,
    Left    ,
    Right   ,
    Up,
    Down,
}

// A sorting direction
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgSortDirection
{
    None         = 0,
    Ascending    = 1,    // Ascending = 0->9, A->Z etc.
    Descending   = 2     // Descending = 9->0, Z->A etc.
}

impl Default for DimgSortDirection {
    fn default() -> Self {
        Self::None
    }
}

// Keys value 0 to 511 are left unused as legacy native/opaque key values (< 1.87)
// Keys value >= 512 are named keys (>= 1.87)
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgKey
{
    // Keyboard
    None = 0,
    Tab = 512,             // == ImGuiKey_NamedKey_BEGIN
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    PageUp,
    PageDown,
    Home,
    End,
    Insert,
    Delete,
    Backspace,
    Space,
    Enter,
    Escape,
    LeftCtrl, LeftShift, LeftAlt, LeftSuper,
    RightCtrl, RightShift, RightAlt, RightSuper,
    Menu,
    Key_0, Key_1, Key_2, Key_3, Key_4, Key_5, Key_6, Key_7, Key_8, Key_9,
    Key_A, Key_B, Key_C, Key_D, Key_E, Key_F, Key_G, Key_H, Key_I, Key_J,
    Key_K, Key_L, Key_M, Key_N, Key_O, Key_P, Key_Q, Key_R, Key_S, Key_T,
    Key_U, Key_V, Key_W, Key_X, Key_Y, Key_Z,
    F1, F2, F3, F4, F5, F6,
    F7, F8, F9, F10, F11, F12,
    Apostrophe,        // '
    Comma,             // ,
    Minus,             // -
    Period,            // .
    Slash,             // /
    Semicolon,         // ;
    Equal,             // =
    LeftBracket,       // [
    Backslash,         // \ (this text inhibit multiline comment caused by backslash)
    RightBracket,      // ]
    GraveAccent,       // `
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    Keypad0, Keypad1, Keypad2, Keypad3, Keypad4,
    Keypad5, Keypad6, Keypad7, Keypad8, Keypad9,
    KeypadDecimal,
    KeypadDivide,
    KeypadMultiply,
    KeypadSubtract,
    KeypadAdd,
    KeypadEnter,
    KeypadEqual,

    // Gamepad (some of those are analog values, 0.0 to 1.0)                              // NAVIGATION action
    GamepadStart,          // Menu (Xbox)          + (Switch)   Start/Options (PS) // --
    GamepadBack,           // View (Xbox)          - (Switch)   Share (PS)         // --
    GamepadFaceUp,         // Y (Xbox)             X (Switch)   Triangle (PS)      // -> ImGuiNavInput_Input
    GamepadFaceDown,       // A (Xbox)             B (Switch)   Cross (PS)         // -> ImGuiNavInput_Activate
    GamepadFaceLeft,       // X (Xbox)             Y (Switch)   Square (PS)        // -> ImGuiNavInput_Menu
    GamepadFaceRight,      // B (Xbox)             A (Switch)   Circle (PS)        // -> ImGuiNavInput_Cancel
    GamepadDpadUp,         // D-pad Up                                             // -> ImGuiNavInput_DpadUp
    GamepadDpadDown,       // D-pad down                                           // -> ImGuiNavInput_DpadDown
    GamepadDpadLeft,       // D-pad Left                                           // -> ImGuiNavInput_DpadLeft
    GamepadDpadRight,      // D-pad Right                                          // -> ImGuiNavInput_DpadRight
    GamepadL1,             // L Bumper (Xbox)      L (Switch)   L1 (PS)            // -> ImGuiNavInput_FocusPrev + ImGuiNavInput_TweakSlow
    GamepadR1,             // R Bumper (Xbox)      R (Switch)   R1 (PS)            // -> ImGuiNavInput_FocusNext + ImGuiNavInput_TweakFast
    GamepadL2,             // L Trigger (Xbox)     ZL (Switch)  L2 (PS) [Analog]
    GamepadR2,             // R Trigger (Xbox)     ZR (Switch)  R2 (PS) [Analog]
    GamepadL3,             // L Thumbstick (Xbox)  L3 (Switch)  L3 (PS)
    GamepadR3,             // R Thumbstick (Xbox)  R3 (Switch)  R3 (PS)
    GamepadLStickUp,       // [Analog]                                             // -> ImGuiNavInput_LStickUp
    GamepadLStickDown,     // [Analog]                                             // -> ImGuiNavInput_LStickDown
    GamepadLStickLeft,     // [Analog]                                             // -> ImGuiNavInput_LStickLeft
    GamepadLStickRight,    // [Analog]                                             // -> ImGuiNavInput_LStickRight
    GamepadRStickUp,       // [Analog]
    GamepadRStickDown,     // [Analog]
    GamepadRStickLeft,     // [Analog]
    GamepadRStickRight,    // [Analog]

    // Keyboard Modifiers (explicitly submitted by backend via AddKeyEvent() calls)
    // - This is mirroring the data also written to io.KeyCtrl, io.KeyShift, io.KeyAlt, io.KeySuper, in a format allowing
    //   them to be accessed via standard key API, allowing calls such as IsKeyPressed(), IsKeyReleased(), querying duration etc.
    // - Code polling every keys (e.g. an interface to detect a key press for input mapping) might want to ignore those
    //   and prefer using the real keys (e.g. ImGuiKey_LeftCtrl, ImGuiKey_RightCtrl instead of ImGuiKey_ModCtrl).
    // - In theory the value of keyboard modifiers should be roughly equivalent to a logical or of the equivalent left/right keys.
    //   In practice: it's complicated; mods are often provided from different sources. Keyboard layout, IME, sticky keys and
    //   backends tend to interfere and break that equivalence. The safer decision is to relay that ambiguity down to the end-user...
    ModCtrl, ModShift, ModAlt, ModSuper,

    // End of list
    LastItem,                 // No valid ImGuiKey is ever greater than this value

    // [Internal] Prior to 1.87 we required user to fill io.KeysDown[512] using their own native index + a io.KeyMap[] array.
    // We are ditching this method but keeping a legacy path for user code doing e.g. IsKeyPressed(MY_NATIVE_KEY_CODE)
    // ImGuiKey_NamedKey_BEGIN         = 512,
    // ImGuiKey_NamedKey_END           = ImGuiKey_COUNT,
    // ImGuiKey_NamedKey_COUNT         = ImGuiKey_NamedKey_END - ImGuiKey_NamedKey_BEGIN,
// #ifdef IMGUI_DISABLE_OBSOLETE_KEYIO
//     ImGuiKey_KeysData_SIZE          = ImGuiKey_NamedKey_COUNT,          // size of KeysData[]: only hold named keys
//     ImGuiKey_KeysData_OFFSET        = ImGuiKey_NamedKey_BEGIN           // First key stored in io.KeysData[0]. Accesses to io.KeysData[] must use (key - ImGuiKey_KeysData_OFFSET).
// #else
//     ImGuiKey_KeysData_SIZE          = ImGuiKey_COUNT,                   // size of KeysData[]: hold legacy 0..512 keycodes + named keys
//     ImGuiKey_KeysData_OFFSET        = 0                                 // First key stored in io.KeysData[0]. Accesses to io.KeysData[] must use (key - ImGuiKey_KeysData_OFFSET).
// #endif

// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     , ImGuiKey_KeyPadEnter = ImGuiKey_KeypadEnter   // Renamed in 1.87
// #endif
}


pub const DIMG_NAMED_KEY_BEGIN: DimgKey         = DimgKey::Tab;

pub const DIMG_NAMED_KEY_END: DimgKey = DimgKey::LastItem;

// pub const     NamedKey_COUNT:          = DimgKey::NamedKey_END - DimgKey::NamedKey_BEGIN;

pub const DIMG_KEYS_DATA_SZ: usize = DimgKey::LastItem as usize - DimgKey::Tab as usize;

pub const DIMG_KEYS_DATA_OFFSET: usize        = 0    ;

// Helper "flags" version of key-mods to store and compare multiple key-mods easily. Sometimes used for storage (e.g. io.KeyMods) but otherwise not much used in public API.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgModFlags
{
    None,
    Ctrl,
    Shift,
    Alt,   // Menu
    Super    // Cmd/Super/windows key
}

// Gamepad/Keyboard navigation
// Since >= 1.87 backends you generally don't need to care about this enum since io.NavInputs[] is setup automatically. This might become private/internal some day.
// Keyboard: Set io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard to enable. NewFrame() will automatically fill io.NavInputs[] based on your io.AddKeyEvent() calls.
// Gamepad:  Set io.ConfigFlags |= ImGuiConfigFlags_NavEnableGamepad to enable. Backend: set ImGuiBackendFlags_HasGamepad and fill the io.NavInputs[] fields before calling NewFrame(). Note that io.NavInputs[] is cleared by EndFrame().
// Read instructions in imgui.cpp for more details. Download PNG/PSD at http://dearimgui.org/controls_sheets.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum ImGuiNavInput
{
    // Gamepad Mapping
    ImGuiNavInput_Activate,      // Activate / Open / Toggle / Tweak value       // e.g. Cross  (PS4), A (Xbox), A (Switch), Space (Keyboard)
    ImGuiNavInput_Cancel,        // Cancel / Close / Exit                        // e.g. Circle (PS4), B (Xbox), B (Switch), Escape (Keyboard)
    ImGuiNavInput_Input,         // Text input / On-Screen keyboard              // e.g. Triang.(PS4), Y (Xbox), X (Switch), Return (Keyboard)
    ImGuiNavInput_Menu,          // Tap: Toggle menu / Hold: Focus, Move, Resize // e.g. Square (PS4), X (Xbox), Y (Switch), Alt (Keyboard)
    ImGuiNavInput_DpadLeft,      // Move / Tweak / Resize window (w/ PadMenu)    // e.g. D-pad Left/Right/Up/down (Gamepads), Arrow keys (Keyboard)
    ImGuiNavInput_DpadRight,     //
    ImGuiNavInput_DpadUp,        //
    ImGuiNavInput_DpadDown,      //
    ImGuiNavInput_LStickLeft,    // scroll / Move window (w/ PadMenu)            // e.g. Left Analog Stick Left/Right/Up/down
    ImGuiNavInput_LStickRight,   //
    ImGuiNavInput_LStickUp,      //
    ImGuiNavInput_LStickDown,    //
    ImGuiNavInput_FocusPrev,     // Focus Next window (w/ PadMenu)               // e.g. L1 or L2 (PS4), LB or LT (Xbox), L or ZL (Switch)
    ImGuiNavInput_FocusNext,     // Focus Prev window (w/ PadMenu)               // e.g. R1 or R2 (PS4), RB or RT (Xbox), R or ZL (Switch)
    ImGuiNavInput_TweakSlow,     // Slower tweaks                                // e.g. L1 or L2 (PS4), LB or LT (Xbox), L or ZL (Switch)
    ImGuiNavInput_TweakFast,     // Faster tweaks                                // e.g. R1 or R2 (PS4), RB or RT (Xbox), R or ZL (Switch)

    // [Internal] Don't use directly! This is used internally to differentiate keyboard from gamepad inputs for behaviors that require to differentiate them.
    // Keyboard behavior that have no corresponding gamepad mapping (e.g. CTRL+TAB) will be directly reading from keyboard keys instead of io.NavInputs[].
    ImGuiNavInput_KeyLeft_,      // Move left                                    // = Arrow keys
    ImGuiNavInput_KeyRight_,     // Move right
    ImGuiNavInput_KeyUp_,        // Move up
    ImGuiNavInput_KeyDown_,      // Move down
    ImGuiNavInput_COUNT
}

// Configuration flags stored in io.ConfigFlags. Set by user/application.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgConfigFlags
{
    None                   = 0,
    NavEnableKeyboard      = 1 << 0,   // Master keyboard navigation enable flag. NewFrame() will automatically fill io.NavInputs[] based on io.AddKeyEvent() calls
    NavEnableGamepad       = 1 << 1,   // Master gamepad navigation enable flag. This is mostly to instruct your imgui backend to fill io.NavInputs[]. Backend also needs to set ImGuiBackendFlags_HasGamepad.
    NavEnableSetMousePos   = 1 << 2,   // Instruct navigation to move the mouse cursor. May be useful on TV/console systems where moving a virtual mouse is awkward. Will update io.MousePos and set io.WantSetMousePos=true. If enabled you MUST honor io.WantSetMousePos requests in your backend, otherwise ImGui will react as if the mouse is jumping around back and forth.
    NavNoCaptureKeyboard   = 1 << 3,   // Instruct navigation to not set the io.WantCaptureKeyboard flag when io.NavActive is set.
    NoMouse                = 1 << 4,   // Instruct imgui to clear mouse position/buttons in NewFrame(). This allows ignoring the mouse information set by the backend.
    NoMouseCursorChange    = 1 << 5,   // Instruct backend to not alter mouse cursor shape and visibility. Use if the backend cursor changes are interfering with yours and you don't want to use SetMouseCursor() to change mouse cursor. You may want to honor requests from imgui by reading GetMouseCursor() yourself instead.

    // [BETA] Docking
    DockingEnable          = 1 << 6,   // Docking enable flags.

    // [BETA] viewports
    // When using viewports it is recommended that your default value for ImGuiCol_WindowBg is opaque (Alpha=1.0) so transition to a viewport won't be noticeable.
    ViewportsEnable        = 1 << 10,  // viewport enable flags (require both ImGuiBackendFlags_PlatformHasViewports + ImGuiBackendFlags_RendererHasViewports set by the respective backends)
    DpiEnableScaleViewports= 1 << 14,  // [BETA: Don't use] FIXME-DPI: Reposition and resize imgui windows when the DpiScale of a viewport changed (mostly useful for the main viewport hosting other window). Note that resizing the main window itself is up to your application.
    DpiEnableScaleFonts    = 1 << 15,  // [BETA: Don't use] FIXME-DPI: Request bitmap-scaled fonts to match DpiScale. This is a very low-quality workaround. The correct way to handle DPI is _currently_ to replace the atlas and/or fonts in the Platform_OnChangedViewport callback, but this is all early work in progress.

    // User storage (to allow your backend/engine to communicate to code that may be shared between multiple projects. Those flags are NOT used by core Dear ImGui)
    IsSRGB                 = 1 << 20,  // Application is SRGB-aware.
    IsTouchScreen          = 1 << 21   // Application is using a touch screen instead of a mouse.
}

// Backend capabilities flags stored in io.BackendFlags. Set by imgui_impl_xxx or custom backend.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgBackendFlags
{
    None                  = 0,
    HasGamepad            = 1 << 0,   // Backend Platform supports gamepad and currently has one connected.
    HasMouseCursors       = 1 << 1,   // Backend Platform supports honoring GetMouseCursor() value to change the OS cursor shape.
    HasSetMousePos        = 1 << 2,   // Backend Platform supports io.WantSetMousePos requests to reposition the OS mouse position (only used if ImGuiConfigFlags_NavEnableSetMousePos is set).
    RendererHasVtxOffset  = 1 << 3,   // Backend Renderer supports ImDrawCmd::vtx_offset. This enables output of large meshes (64K+ vertices) while still using 16-bit indices.

    // [BETA] viewports
    PlatformHasViewports  = 1 << 10,  // Backend Platform supports multiple viewports.
    HasMouseHoveredViewport=1 << 11,  // Backend Platform supports calling io.AddMouseViewportEvent() with the viewport under the mouse. IF POSSIBLE, ignore viewports with the ImGuiViewportFlags_NoInputs flag (Win32 backend, GLFW 3.30+ backend can do this, SDL backend cannot). If this cannot be done, Dear ImGui needs to use a flawed heuristic to find the viewport under.
    RendererHasViewports  = 1 << 12   // Backend Renderer supports multiple viewports.
}

/// Enumeration for PushStyleColor() / PopStyleColor()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgColor
{
    Text,
    TextDisabled,
    WindowBg,              // Background of normal windows
    ChildBg,               // Background of child windows
    PopupBg,               // Background of popups, menus, tooltips windows
    Border,
    BorderShadow,
    FrameBg,               // Background of checkbox, radio button, plot, slider, text input
    FrameBgHovered,
    FrameBgActive,
    TitleBg,
    TitleBgActive,
    TitleBgCollapsed,
    MenuBarBg,
    ScrollbarBg,
    ScrollbarGrab,
    ScrollbarGrabHovered,
    ScrollbarGrabActive,
    CheckMark,
    SliderGrab,
    SliderGrabActive,
    Button,
    ButtonHovered,
    ButtonActive,
    Header,                // Header* colors are used for CollapsingHeader, TreeNode, Selectable, MenuItem
    HeaderHovered,
    HeaderActive,
    Separator,
    SeparatorHovered,
    SeparatorActive,
    ResizeGrip,            // Resize grip in lower-right and lower-left corners of windows.
    ResizeGripHovered,
    ResizeGripActive,
    Tab,                   // TabItem in a TabBar
    TabHovered,
    TabActive,
    TabUnfocused,
    TabUnfocusedActive,
    DockingPreview,        // preview overlay color when about to docking something
    DockingEmptyBg,        // Background color for empty node (e.g. CentralNode with no window docked into it)
    PlotLines,
    PlotLinesHovered,
    PlotHistogram,
    PlotHistogramHovered,
    TableHeaderBg,         // Table header background
    TableBorderStrong,     // Table outer and header borders (prefer using Alpha=1.0 here)
    TableBorderLight,      // Table inner borders (prefer using Alpha=1.0 here)
    TableRowBg,            // Table row background (even rows)
    TableRowBgAlt,         // Table row background (odd rows)
    TextSelectedBg,
    DragDropTarget,        // Rectangle highlighting a drop target
    NavHighlight,          // Gamepad/keyboard: current highlighted item
    NavWindowingHighlight, // Highlight window when using CTRL+TAB
    NavWindowingDimBg,     // Darken/colorize entire screen behind the CTRL+TAB window list, when active
    ModalWindowDimBg,      // Darken/colorize entire screen behind a modal window, when one is active
}

// Enumeration for PushStyleVar() / PopStyleVar() to temporarily modify the ImGuiStyle structure.
// - The enum only refers to fields of ImGuiStyle which makes sense to be pushed/popped inside UI code.
//   During initialization or between frames, feel free to just poke into ImGuiStyle directly.
// - Tip: Use your programming IDE navigation facilities on the names in the _second column_ below to find the actual members and their description.
//   In Visual Studio IDE: CTRL+comma ("Edit.GoToAll") can follow symbols in comments, whereas CTRL+F12 ("Edit.GoToImplementation") cannot.
//   With Visual Assist installed: ALT+G ("VAssistX.GoToImplementation") can also follow symbols in comments.
// - When changing this enum, you need to update the associated internal table GStyleVarInfo[] accordingly. This is where we link enum values to members offset/type.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgStyleVar
{
    // Enum name --------------------- // Member in ImGuiStyle structure (see ImGuiStyle for descriptions)
    Alpha,               // float     Alpha
    DisabledAlpha,       // float     DisabledAlpha
    WindowPadding,       // ImVec2    window_padding
    WindowRounding,      // float     window_rounding
    WindowBorderSize,    // float     WindowBorderSize
    WindowMinSize,       // ImVec2    WindowMinSize
    WindowTitleAlign,    // ImVec2    WindowTitleAlign
    ChildRounding,       // float     ChildRounding
    ChildBorderSize,     // float     ChildBorderSize
    PopupRounding,       // float     PopupRounding
    PopupBorderSize,     // float     PopupBorderSize
    FramePadding,        // ImVec2    FramePadding
    FrameRounding,       // float     FrameRounding
    FrameBorderSize,     // float     FrameBorderSize
    ItemSpacing,         // ImVec2    ItemSpacing
    ItemInnerSpacing,    // ImVec2    ItemInnerSpacing
    IndentSpacing,       // float     IndentSpacing
    CellPadding,         // ImVec2    CellPadding
    ScrollbarSize,       // float     ScrollbarSize
    ScrollbarRounding,   // float     ScrollbarRounding
    GrabMinSize,         // float     GrabMinSize
    GrabRounding,        // float     GrabRounding
    TabRounding,         // float     TabRounding
    ButtonTextAlign,     // ImVec2    ButtonTextAlign
    SelectableTextAlign, // ImVec2    SelectableTextAlign
    COUNT
}

// flags for InvisibleButton() [extended in imgui_internal.h]
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgButtonFlags
{
    None                   = 0,
    MouseButtonLeft        = 1 << 0,   // React on left mouse button (default)
    MouseButtonRight       = 1 << 1,   // React on right mouse button
    MouseButtonMiddle      = 1 << 2,   // React on center mouse button

    // [Internal]
    // ImGuiButtonFlags_MouseButtonMask_       = ImGuiButtonFlags_MouseButtonLeft | ImGuiButtonFlags_MouseButtonRight | ImGuiButtonFlags_MouseButtonMiddle,
    // ImGuiButtonFlags_MouseButtonDefault_    = ImGuiButtonFlags_MouseButtonLeft
}

// pub const MouseButtonMask_: i32       = DimgButtonFlags::MouseButtonLeft | DimgButtonFlags::MouseButtonRight | DimgButtonFlags::MouseButtonMiddle;
pub const MOUSE_BTN_MASK: HashSet<DimgButtonFlags> = HashSet::from([
   DimgButtonFlags::MouseButtonLeft, DimgButtonFlags::MouseButtonRight, DimgButtonFlags::MouseButtonMiddle
]);

pub const MOUSE_BTN_DFLT: DimgButtonFlags = DimgButtonFlags::MouseButtonLeft;

pub const    MouseButtonDefault_: i32    = DimgButtonFlags::MouseButtonLeft as i32;

// flags for ColorEdit3() / ColorEdit4() / ColorPicker3() / ColorPicker4() / ColorButton()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgColorEditFlags
{
    None            = 0,
    NoAlpha         = 1 << 1,   //              // ColorEdit, ColorPicker, ColorButton: ignore Alpha component (will only read 3 components from the input pointer).
    NoPicker        = 1 << 2,   //              // ColorEdit: disable picker when clicking on color square.
    NoOptions       = 1 << 3,   //              // ColorEdit: disable toggling options menu when right-clicking on inputs/small preview.
    NoSmallPreview  = 1 << 4,   //              // ColorEdit, ColorPicker: disable color square preview next to the inputs. (e.g. to show only the inputs)
    NoInputs        = 1 << 5,   //              // ColorEdit, ColorPicker: disable inputs sliders/text widgets (e.g. to show only the small preview color square).
    NoTooltip       = 1 << 6,   //              // ColorEdit, ColorPicker, ColorButton: disable tooltip when hovering the preview.
    NoLabel         = 1 << 7,   //              // ColorEdit, ColorPicker: disable display of inline text label (the label is still forwarded to the tooltip and picker).
    NoSidePreview   = 1 << 8,   //              // ColorPicker: disable bigger color preview on right side of the picker, use small color square preview instead.
    NoDragDrop      = 1 << 9,   //              // ColorEdit: disable drag and drop target. ColorButton: disable drag and drop source.
    NoBorder        = 1 << 10,  //              // ColorButton: disable border (which is enforced by default)

    // User Options (right-click on widget to change some of them).
    AlphaBar        = 1 << 16,  //              // ColorEdit, ColorPicker: show vertical alpha bar/gradient in picker.
    AlphaPreview    = 1 << 17,  //              // ColorEdit, ColorPicker, ColorButton: display preview as a transparent color over a checkerboard, instead of opaque.
    AlphaPreviewHalf= 1 << 18,  //              // ColorEdit, ColorPicker, ColorButton: display half opaque / half checkerboard, instead of opaque.
    HDR             = 1 << 19,  //              // (WIP) ColorEdit: Currently only disable 0.0..1.0 limits in RGBA edition (note: you probably want to use ImGuiColorEditFlags_Float flag as well).
    DisplayRGB      = 1 << 20,  // [Display]    // ColorEdit: override _display_ type among RGB/HSV/Hex. ColorPicker: select any combination using one or more of RGB/HSV/Hex.
    DisplayHSV      = 1 << 21,  // [Display]    // "
    DisplayHex      = 1 << 22,  // [Display]    // "
    Uint8           = 1 << 23,  // [data_type]   // ColorEdit, ColorPicker, ColorButton: _display_ values formatted as 0..255.
    Float           = 1 << 24,  // [data_type]   // ColorEdit, ColorPicker, ColorButton: _display_ values formatted as 0.0..1.0 floats instead of 0..255 integers. No round-trip of value via integers.
    PickerHueBar    = 1 << 25,  // [Picker]     // ColorPicker: bar for Hue, rectangle for Sat/Value.
    PickerHueWheel  = 1 << 26,  // [Picker]     // ColorPicker: wheel for Hue, triangle for Sat/Value.
    InputRGB        = 1 << 27,  // [Input]      // ColorEdit, ColorPicker: input and output data in RGB format.
    InputHSV        = 1 << 28,  // [Input]      // ColorEdit, ColorPicker: input and output data in HSV format.

    // Defaults Options. You can set application defaults using SetColorEditOptions(). The intent is that you probably don't want to
    // override them in most of your calls. Let the user choose via the option menu and/or call SetColorEditOptions() once during startup.
    // ImGuiColorEditFlags_DefaultOptions_ = ImGuiColorEditFlags_Uint8 | ImGuiColorEditFlags_DisplayRGB | ImGuiColorEditFlags_InputRGB | ImGuiColorEditFlags_PickerHueBar,
    //
    // // [Internal] Masks
    // ImGuiColorEditFlags_DisplayMask_    = ImGuiColorEditFlags_DisplayRGB | ImGuiColorEditFlags_DisplayHSV | ImGuiColorEditFlags_DisplayHex,
    // ImGuiColorEditFlags_DataTypeMask_   = ImGuiColorEditFlags_Uint8 | ImGuiColorEditFlags_Float,
    // ImGuiColorEditFlags_PickerMask_     = ImGuiColorEditFlags_PickerHueWheel | ImGuiColorEditFlags_PickerHueBar,
    // ImGuiColorEditFlags_InputMask_      = ImGuiColorEditFlags_InputRGB | ImGuiColorEditFlags_InputHSV

    // Obsolete names (will be removed)
    // ImGuiColorEditFlags_RGB = ImGuiColorEditFlags_DisplayRGB, ImGuiColorEditFlags_HSV = ImGuiColorEditFlags_DisplayHSV, ImGuiColorEditFlags_HEX = ImGuiColorEditFlags_DisplayHex  // [renamed in 1.69]
}

 // pub const DefaultOptions_: i32 = DimgColorEditFlags::Uint8 | DimgColorEditFlags::DisplayRGB | DimgColorEditFlags::InputRGB | DimgColorEditFlags::PickerHueBar;
pub const DFLT_OPTS: HashSet<DimgColorEditFlags> = HashSet::from([
     DimgColorEditFlags::Uint8, DimgColorEditFlags::DisplayRGB, DimgColorEditFlags::InputRGB, DimgColorEditFlags::PickerHueBar
 ]);

    // [Internal] Masks
    // pub const DisplayMask_: i32    = DimgColorEditFlags::DisplayRGB | DimgColorEditFlags::DisplayHSV | DimgColorEditFlags::DisplayHex;
   pub const DISPLAY_MASK: HashSet<DimgColorEditFlags> = HashSet::from([
        DimgColorEditFlags::DisplayRGB, DimgColorEditFlags::DisplayHSV, DimgColorEditFlags::DisplayHex
    ]);

    // pub const DataTypeMask_: i32   = DimgColorEditFlags::Uint8 | DimgColorEditFlags::Float;

    // pub const PickerMask_: i32     = DimgColorEditFlags::PickerHueWheel | DimgColorEditFlags::PickerHueBar;
    pub const PICKER_MASK: HashSet<DimgColorEditFlags> = HashSet::from([
        DimgColorEditFlags::PickerHueWheel, DimgColorEditFlags::PickerHueBar
    ]);

    // pub const InputMask_: i32      = DimgColorEditFlags::InputRGB | DimgColorEditFlags::InputHSV;
    pub const INPUT_MASK: HashSet<DimgColorEditFlags> = HashSet::from([
        DimgColorEditFlags::InputRGB, DimgColorEditFlags::InputHSV
    ]);

// flags for DragFloat(), DragInt(), SliderFloat(), SliderInt() etc.
// We use the same sets of flags for DragXXX() and SliderXXX() functions as the features are the same and it makes it easier to swap them.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgSliderFlags
{
    None                   = 0,
    AlwaysClamp            = 1 << 4,       // Clamp value to min/max bounds when input manually with CTRL+Click. By default CTRL+Click allows going out of bounds.
    Logarithmic            = 1 << 5,       // Make the widget logarithmic (linear otherwise). Consider using ImGuiSliderFlags_NoRoundToFormat with this if using a format-string with small amount of digits.
    NoRoundToFormat        = 1 << 6,       // Disable rounding underlying value to match precision of the display format string (e.g. %.3 values are rounded to those 3 digits)
    NoInput                = 1 << 7,       // Disable CTRL+Click or Enter key allowing to input text directly into the widget
    InvalidMask           = 0x7000000F    // [Internal] We treat using those bits as being potentially a 'float power' argument from the previous API that has got miscast to this enum, and will trigger an assert if needed.

    // Obsolete names (will be removed)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     , ImGuiSliderFlags_ClampOnInput = ImGuiSliderFlags_AlwaysClamp // [renamed in 1.79]
// #endif
}

// Identify a mouse button.
// Those values are guaranteed to be stable and we frequently use 0/1 directly. Named enums provided for convenience.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgMouseButton
{
    Left = 0,
    Right = 1,
    Middle = 2,
    COUNT = 5
}

// Enumeration for GetMouseCursor()
// User code may request backend to display given cursor by calling SetMouseCursor(), which is why we have some cursors that are marked unused here
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgMouseCursor
{
    None,
    Arrow,
    TextInput,         // When hovering over InputText, etc.
    ResizeAll,         // (Unused by Dear ImGui functions)
    ResizeNS,          // When hovering over an horizontal border
    ResizeEW,          // When hovering over a vertical border or a column
    ResizeNESW,        // When hovering over the bottom-left corner of a window
    ResizeNWSE,        // When hovering over the bottom-right corner of a window
    Hand,              // (Unused by Dear ImGui functions. Use for e.g. hyperlinks)
    NotAllowed,        // When hovering something with disallowed interaction. Usually a crossed circle.
}

// Enumeration for ImGui::SetWindow***(), SetNextWindow***(), SetNextItem***() functions
// Represent a condition.
// Important: Treat as a regular enum! Do NOT combine multiple values using binary operators! All the functions above treat 0 as a shortcut to ImGuiCond_Always.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgCond
{
    None          = 0,        // No condition (always set the variable), same as _Always
    Always        = 1 << 0,   // No condition (always set the variable)
    Once          = 1 << 1,   // Set the variable once per runtime session (only the first call will succeed)
    FirstUseEver  = 1 << 2,   // Set the variable if the object/window has no persistently saved data (no entry in .ini file)
    Appearing     = 1 << 3    // Set the variable if the object/window is appearing after being hidden/inactive (or the first time)
}

impl Default for DimgCond {
    fn default() -> Self {
        Self::None
    }
}

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

// [Internal] Storage used by IsKeyDown(), IsKeyPressed() etc functions.
// If prior to 1.87 you used io.KeysDownDuration[] (which was marked as internal), you should use GetKeyData(key)->down_duration and not io.KeysData[key]->down_duration.
pub struct DimgKeyData
{
    pub down: bool,               // True for if key is down
    pub down_duration: f32,      // Duration the key has been down (<0.0: not pressed, 0.0: just pressed, >0.0: time held)
    pub down_duration_prev: f32,  // Last frame duration the key has been down
    pub analog_value: f32,       // 0.0..1.0 for gamepad values
}

//-----------------------------------------------------------------------------
// [SECTION] Misc data structures
//-----------------------------------------------------------------------------

// Resizing callback data to apply custom constraint. As enabled by SetNextWindowSizeConstraints(). Callback is called during the next Begin().
// NB: For basic min/max size constraint on each axis you don't need to use the callback! The SetNextWindowSizeConstraints() parameters are enough.
pub struct DimgSizeCallbackData
{
    // void*   user_data;       // Read-only.   What user passed to SetNextWindowSizeConstraints()
    pub user_data: Vec<u8>,
    // pub pos: ImVec2,            // Read-only.   Window position, for reference.
    pub pos: DimgVec2D,
    // pub current_size: ImVec2,    // Read-only.   Current window size.
    pub current_size: DimgVec2D,
    // pub desired_size: ImVec2,    // Read-write.  Desired size, based on user's mouse position. Write to this field to restrain resizing.
    pub desired_size: DimgVec2D,
}

//-----------------------------------------------------------------------------
// [SECTION] Helpers (ImGuiOnceUponAFrame, ImGuiTextFilter, ImGuiTextBuffer, ImGuiStorage, ImGuiListClipper, ImColor)
//-----------------------------------------------------------------------------

// Helper: Unicode defines
pub const IM_UNICODE_CODEPOINT_INVALID: u32 = 0xFFFD;     // Invalid Unicode code point (standard value).
// #ifdef IMGUI_USE_WCHAR32
pub const IM_UNICODE_CODEPOINT_MAX: u32     = 0x10FFFF;   // Maximum Unicode code point supported by this build.
// #else
// #define IM_UNICODE_CODEPOINT_MAX     0xFFFF     // Maximum Unicode code point supported by this build.
// #endif

// Helper: Execute a block of code at maximum once a frame. Convenient if you want to quickly create an UI within deep-nested code that runs multiple times every frame.
// Usage: static ImGuiOnceUponAFrame oaf; if (oaf) ImGui::Text("This will be called only once per frame");
#[derive(Default,Debug,Clone,PartialEq)]
pub struct ImGuiOnceUponAFrame
{
    pub ref_frame: i32,
    // ImGuiOnceUponAFrame() { ref_frame = -1; }
    // mutable int ref_frame;
    // operator bool() const { int current_frame = ImGui::GetFrameCount(); if (ref_frame == current_frame) return false; ref_frame = current_frame; return true; }
}

impl ImGuiOnceUponAFrame {
    pub fn new() -> Self {
        Self {
            ref_frame: -1,
        }
    }
}

//-----------------------------------------------------------------------------
// [SECTION] Drawing API (ImDrawCmd, ImDrawIdx, ImDrawVert, ImDrawChannel, ImDrawListSplitter, ImDrawListFlags, ImDrawList, ImDrawData)
// Hold a series of drawing commands. The user provides a renderer for ImDrawData which essentially contains an array of ImDrawList.
//-----------------------------------------------------------------------------

// The maximum line width to bake anti-aliased textures for. Build atlas with ImFontAtlasFlags_NoBakedLines to disable baking.
// #ifndef IM_DRAWLIST_TEX_LINES_WIDTH_MAX
// #define IM_DRAWLIST_TEX_LINES_WIDTH_MAX     (63)
// #endif
pub const IM_DRAWLIST_TEX_LINES_WIDTH_MAX: usize = 63;


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

pub type DimgDrawCallback = fn(&mut DimgDrawList, &DimgDrawCmd);

pub fn im_draw_callback_nop(_: &mut DimgDrawList, _: &DimgDrawCmd) {
    todo!()
}

// TODO
// Special Draw callback value to request renderer backend to reset the graphics/render state.
// The renderer backend needs to handle this special value, otherwise it will crash trying to call a function at this address.
// This is useful for example if you submitted callbacks which you know have altered the render state and you want it to be restored.
// It is not done by default because they are many perfectly useful way of altering render state for imgui contents (e.g. changing shader/blending settings before an Image call).
// #define ImDrawCallback_ResetRenderState     (ImDrawCallback)(-1)

// Typically, 1 command = 1 GPU draw call (unless command is a callback)
// - vtx_offset: When 'io.BackendFlags & ImGuiBackendFlags_RendererHasVtxOffset' is enabled,
//   this fields allow us to render meshes larger than 64K vertices while keeping 16-bit indices.
//   Backends made for <1.71. will typically ignore the vtx_offset fields.
// - The clip_rect/texture_id/vtx_offset fields must be contiguous as we memcmp() them together (this is asserted for).
#[derive(Default,Clone)]
pub struct DimgDrawCmd
{
    pub clip_rect: DimgVec4,           // 4*4  // Clipping rectangle (x1, y1, x2, y2). Subtract ImDrawData->DisplayPos to get clipping rectangle in "viewport" coordinates
    // ImTextureID     texture_id,          // 4-8  // User-provided texture ID. Set by user in ImfontAtlas::SetTexID() for fonts or passed to Image*() functions. Ignore if never using images or multiple fonts atlas.
    pub texture_id: DimgTextureId,
pub vtx_offset: i32,        // 4    // Start offset in vertex buffer. ImGuiBackendFlags_RendererHasVtxOffset: always 0, otherwise may be >0 to support meshes larger than 64K vertices with 16-bit indices.
    pub idx_offset: i32,        // 4    // Start offset in index buffer.
    pub elem_count: i32,        // 4    // Number of indices (multiple of 3) to be rendered as triangles. Vertices are stored in the callee ImDrawList's vtx_buffer[] array, indices in idx_buffer[].
    // ImDrawCallback  user_callback;       // 4-8  // If != NULL, call the function instead of rendering the vertices. clip_rect and texture_id will be set normally.
    pub user_callback: Option<DimgDrawCallback>,
    // void*           user_callback_data;   // 4-8  // The draw callback code can access this.
    pub user_callback_data: Vec<u8>,
}

impl Debug for DimgDrawCmd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl DimgDrawCmd {
    // ImDrawCmd() { memset(this, 0, sizeof(*this)); } // Also ensure our padding fields are zeroed
    //
    pub fn new() -> Self {
        Self {
            clip_rect: Default::default(),
            texture_id: DimgId::MAX,
            vtx_offset: 0,
            idx_offset: 0,
            elem_count: 0,
            user_callback: Some(im_draw_callback_nop),
            user_callback_data: vec![]
        }
    }
    //     // Since 1.83: returns ImTextureID associated with this draw call. Warning: DO NOT assume this is always same as 'texture_id' (we will change this function for an upcoming feature)
    //     inline ImTextureID get_tex_id() const { return texture_id; }
    pub fn get_tex_id(&self) -> DimgTextureId {
        self.texture_id
    }
}

// Vertex layout
// #ifndef IMGUI_OVERRIDE_DRAWVERT_STRUCT_LAYOUT
#[derive(Debug,Clone,Default)]
pub struct DimgDrawVert
{
    pub pos: DimgVec2D,
    pub uv: DimgVec2D,
    pub col: u32,
}
// #else
// You can override the vertex format layout by defining IMGUI_OVERRIDE_DRAWVERT_STRUCT_LAYOUT in imconfig.h
// The code expect ImVec2 pos (8 bytes), ImVec2 uv (8 bytes), ImU32 col (4 bytes), but you can re-order them or add other fields as needed to simplify integration in your engine.
// The type has to be described within the macro (you can either declare the struct or use a typedef). This is because ImVec2/ImU32 are likely not declared a the time you'd want to set your type up.
// NOTE: IMGUI DOESN'T CLEAR THE STRUCTURE AND DOESN'T CALL A CONSTRUCTOR SO ANY CUSTOM FIELD WILL BE UNINITIALIZED. IF YOU ADD EXTRA FIELDS (SUCH AS A 'Z' COORDINATES) YOU WILL NEED TO CLEAR THEM DURING RENDER OR TO IGNORE THEM.
// IMGUI_OVERRIDE_DRAWVERT_STRUCT_LAYOUT;
// #endif

// [Internal] For use by ImDrawList
#[derive(Debug,Clone,Default)]
pub struct ImDrawCmdHeader
{
    // ImVec4          clip_rect;
    pub clip_rect: DimgVec4,
    // ImTextureID     texture_id;
    pub texture_id: DimgTextureId,
    // unsigned int    vtx_offset;
    pub vtx_offset: u32,
}

// [Internal] For use by ImDrawListSplitter
#[derive(Debug,Clone,Default)]
pub struct DimgDrawChannel
{
    // ImVector<ImDrawCmd>         _cmd_buffer;
    pub cmd_buffer: Vec<DimgDrawCmd>,
    // ImVector<ImDrawIdx>         _idx_buffer;
    pub idx_buffer: Vec<u32>,
}


// split/merge functions are used to split the draw list into different layers which can be drawn into out of order.
// This is used by the Columns/tables API, so items of each column can be batched together in a same draw call.
#[derive(Debug,Clone,Default)]
pub struct ImDrawListSplitter
{
    pub current: i32,  // Current channel number (0)
    pub count: i32,    // Number of active channels (1+)
    // ImVector<ImDrawChannel>     _channels;   // Draw channels (not resized down so _count might be < Channels.size)
    pub channels: Vec<DimgDrawChannel>,
}

impl ImDrawListSplitter {
    // inline ImDrawListSplitter()  { memset(this, 0, sizeof(*this)); }
    //     inline ~ImDrawListSplitter() { clear_free_memory(); }
    //     inline void                 clear() { _current = 0; _count = 1; } // Do not clear Channels[] so our allocations are reused next frame
    pub fn clear(&mut self) {
        self.current = 0;
        self.count = 1;
    }
    //      void              clear_free_memory();
    pub fn clear_free_memory(&mut self) {
        todo!()
    }
    //      void              split(ImDrawList* draw_list, int count);
    pub fn split(&mut self, draw_list: &DimgDrawList, count: i32) {
        todo!()
    }
    //      void              merge(ImDrawList* draw_list);
    pub fn merge(&mut self, draw_list: &DimgDrawList) {
        todo!()
    }
    //      void              SetCurrentChannel(ImDrawList* draw_list, int channel_idx);
    pub fn set_current_channel(&mut self, draw_list: &DimgDrawList, channel_idx: i32) {
        todo!()
    }
}

// flags for ImDrawList functions
// (Legacy: bit 0 must always correspond to ImDrawFlags_Closed to be backward compatible with old API using a bool. Bits 1..3 must be unused)
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgDrawFlags
{
    None                        = 0,
    Closed                      = 1 << 0, // PathStroke(), AddPolyline(): specify that shape should be closed (Important: this is always == 1 for legacy reason)
    RoundCornersTopLeft         = 1 << 4, // AddRect(), AddRectFilled(), PathRect(): enable rounding top-left corner only (when rounding > 0.0, we default to all corners). Was 0x01.
    RoundCornersTopRight        = 1 << 5, // AddRect(), AddRectFilled(), PathRect(): enable rounding top-right corner only (when rounding > 0.0, we default to all corners). Was 0x02.
    RoundCornersBottomLeft      = 1 << 6, // AddRect(), AddRectFilled(), PathRect(): enable rounding bottom-left corner only (when rounding > 0.0, we default to all corners). Was 0x04.
    RoundCornersBottomRight     = 1 << 7, // AddRect(), AddRectFilled(), PathRect(): enable rounding bottom-right corner only (when rounding > 0.0, we default to all corners). Wax 0x08.
    RoundCornersNone            = 1 << 8, // AddRect(), AddRectFilled(), PathRect(): disable rounding on all corners (when rounding > 0.0). This is NOT zero, NOT an implicit flag!

}

// pub const RoundCornersTop: u32             = DimgDrawFlags::RoundCornersTopLeft | DimgDrawFlags::RoundCornersTopRight;


pub const RoundCornersBottom: u32          = DimgDrawFlags::RoundCornersBottomLeft | DimgDrawFlags::RoundCornersBottomRight;


    pub const RoundCornersLeft: u32            = DimgDrawFlags::RoundCornersBottomLeft | DimgDrawFlags::RoundCornersTopLeft;


pub const RoundCornersRight: u32           = DimgDrawFlags::RoundCornersBottomRight | DimgDrawFlags::RoundCornersTopRight;


    pub const RoundCornersAll: u32             = DimgDrawFlags::RoundCornersTopLeft | DimgDrawFlags::RoundCornersTopRight | DimgDrawFlags::RoundCornersBottomLeft | DimgDrawFlags::RoundCornersBottomRight;


    pub const RoundCornersDefault_: u32        = RoundCornersAll; // Default to ALL corners if none of the _RoundCornersXX flags are specified.


    pub const RoundCornersMask_: u32           = RoundCornersAll | DimgDrawFlags::RoundCornersNone;

// flags for ImDrawList instance. Those are set automatically by ImGui:: functions from ImGuiIO settings, and generally not manipulated directly.
// It is however possible to temporarily alter flags between calls to ImDrawList:: functions.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum ImDrawListFlags
{
    ImDrawListFlags_None                    = 0,
    ImDrawListFlags_AntiAliasedLines        = 1 << 0,  // Enable anti-aliased lines/borders (*2 the number of triangles for 1.0 wide line or lines thin enough to be drawn using textures, otherwise *3 the number of triangles)
    ImDrawListFlags_AntiAliasedLinesUseTex  = 1 << 1,  // Enable anti-aliased lines/borders using textures when possible. Require backend to render with bilinear filtering (NOT point/nearest filtering).
    ImDrawListFlags_AntiAliasedFill         = 1 << 2,  // Enable anti-aliased edge around filled shapes (rounded rectangles, circles).
    ImDrawListFlags_AllowVtxOffset          = 1 << 3   // Can emit 'vtx_offset > 0' to allow large meshes. Set when 'ImGuiBackendFlags_RendererHasVtxOffset' is enabled.
}

// All draw data to render a Dear ImGui frame
// (NB: the style and the naming convention here is a little inconsistent, we currently preserve them for backward compatibility purpose,
// as this is one of the oldest structure exposed by the library! Basically, ImDrawList == CmdList)
#[derive(Debug,Clone,Default)]
pub struct ImDrawData
{
    pub Valid: bool,                  // Only valid after Render() is called and before the next NewFrame() is called.
    pub CmdListsCount: i32,        // Number of ImDrawList* to render
    pub TotalIdxCount: i32,        // For convenience, sum of all ImDrawList's IdxBuffer.size
    pub TotalVtxCount: i32,        // For convenience, sum of all ImDrawList's VtxBuffer.size
    // ImDrawList**    CmdLists;               // Array of ImDrawList* to render. The ImDrawList are owned by ImGuiContext and only pointed to from here.
    pub CmdLists: Vec<ImDrawList>,
    pub DisplayPos: DimgVec2D,             // Top-left position of the viewport to render (== top-left of the orthogonal projection matrix to use) (== GetMainViewport()->pos for the main viewport, == (0.0) in most single-viewport applications)
    pub DisplaySize: DimgVec2D,            // size of the viewport to render (== GetMainViewport()->size for the main viewport, == io.DisplaySize in most single-viewport applications)
    pub FramebufferScale: DimgVec2D,       // Amount of pixels for each unit of DisplaySize. Based on io.DisplayFramebufferScale. Generally (1,1) on normal display, (2,2) on OSX with Retina display.
    // ImGuiViewport*  OwnerViewport;          // viewport carrying the ImDrawData instance, might be of use to the renderer (generally not).
    pub OnwerViewport: DimgViewport,

}

impl ImDrawData {
    // // Functions
    //     ImDrawData()    { clear(); }
    //     void clear()    { memset(this, 0, sizeof(*this)); }     // The ImDrawList are owned by ImGuiContext!
    pub fn Clear(&mut self) {
        self.Valid = false;
        self.CmdListsCount = 0;
        self.TotalIdxCount = 0;
        self.TotalVtxCount = 0;
        self.CmdLists.clear();
        self.DisplayPos.clear();
        self.DisplaySize.clear();
        self.FramebufferScale.clear();
        self.OnwerViewport.clear();
    }
    //      void  DeIndexAllBuffers();                    // Helper to convert all buffers from indexed to non-indexed, in case you cannot render indexed. Note: this is slow and most likely a waste of resources. Always prefer indexed rendering!
    pub fn DeIndexAllBuffers(&mut self) {
        todo!()
    }
    //      void  ScaleClipRects(const ImVec2& fb_scale); // Helper to scale the clip_rect field of each ImDrawCmd. Use if your final output buffer is at a different scale than Dear ImGui expects, or if there is a difference between your window resolution and framebuffer resolution.
    pub fn ScaleClipRects(&mut self, fb_scale: &DimgVec2D) {
        todo!()
    }
}

//-----------------------------------------------------------------------------
// [SECTION] font API (ImFontConfig, ImFontGlyph, ImFontAtlasFlags, ImFontAtlas, ImFontGlyphRangesBuilder, ImFont)
//-----------------------------------------------------------------------------

#[derive(Clone,Debug,Default)]
pub struct ImFontConfig
{
    pub FontData: *mut c_void, // void*           FontData;               //          // TTF/OTF data
    pub FontDataSize: i32,         //          // TTF/OTF data size
    pub FontDataOwnedByAtlas: bool,   // true     // TTF/OTF data ownership taken by the container ImFontAtlas (will delete memory itself).
    pub FontNo: i32,               // 0        // Index of font within TTF/OTF file
    pub SizePixels: f32,            //          // size in pixels for rasterizer (more or less maps to the resulting font height).
    pub OversampleH: i32,          // 3        // Rasterize at higher quality for sub-pixel positioning. Note the difference between 2 and 3 is minimal so you can reduce this to 2 to save memory. Read https://github.com/nothings/stb/blob/master/tests/oversample/README.md for details.
    pub OversampleV: i32,          // 1        // Rasterize at higher quality for sub-pixel positioning. This is not really useful as we don't use sub-pixel positions on the Y axis.
    pub PixelSnapH: bool,             // false    // Align every glyph to pixel boundary. Useful e.g. if you are merging a non-pixel aligned font with the default font. If enabled, you can set OversampleH/V to 1.
    pub GlyphExtraSpacing: DimgVec2D,      // 0, 0     // Extra spacing (in pixels) between glyphs. Only X axis is supported for now.
    pub GlyphOffset: DimgVec2D,            // 0, 0     // Offset all glyphs from this font input.
    pub GlyphRanges: Vec<ImWchar>, // const ImWchar*  GlyphRanges;            // NULL     // Pointer to a user-provided list of Unicode range (2 value per range, values are inclusive, zero-terminated list). THE ARRAY DATA NEEDS TO PERSIST AS LONG AS THE FONT IS ALIVE.
    pub GlyphMinAdvanceX: f32,      // 0        // Minimum AdvanceX for glyphs, set Min to align font icons, set both Min/Max to enforce mono-space font
    pub GlyphMaxAdvanceX: f32,      // FLT_MAX  // Maximum AdvanceX for glyphs
    pub MergeMode: bool,              // false    // merge into previous ImFont, so you can combine multiple inputs font into one ImFont (e.g. ASCII font + icons + Japanese glyphs). You may want to use GlyphOffset.y when merge font of different heights.
    pub FontBuilderFlags: u32,     // 0        // Settings for custom font builder. THIS IS BUILDER IMPLEMENTATION DEPENDENT. Leave as zero if unsure.
    pub RasterizerMultiply: f32,    // 1.0     // Brighten (>1.0) or darken (<1.0) font output. Brightening small fonts may be a good workaround to make them more readable.
    // ImWchar         EllipsisChar;           // -1       // Explicitly specify unicode codepoint of ellipsis character. When fonts are being merged first specified ellipsis will be used.
    pub EllipsisChar: ImWchar,

    // [Internal]
    // char            Name[40];               // Name (strictly to ease debugging)
    pub Name: String,
    // ImFont*         DstFont;
    pub DstFont: DimgFont,

    //  ImFontConfig();
}

// Hold rendering data for one glyph.
// (Note: some language parsers may fail to convert the 31+1 bitfield members, in this case maybe drop store a single u32 or we can rework this)
#[derive(Clone,Debug,Default)]
pub struct ImFontGlyph
{
    // unsigned int    Colored : 1;        // Flag to indicate glyph is colored and should generally ignore tinting (make it usable with no shift on little-endian as this is used in loops)
    pub Colored: bool,
    // unsigned int    Visible : 1;        // Flag to indicate glyph has no visible pixels (e.g. space). Allow early out when rendering.
    pub Visible: bool,
    // unsigned int    Codepoint : 30;     // 0x0000..0x10FFFF
    pub Codepoint: u32,
    // pub AdvanceX: f32,          // Distance to next character (= data from font + ImFontConfig::GlyphExtraSpacing.x baked in)
    pub AdvanceX: f32,
    // float           X0, Y0, X1, Y1;     // Glyph corners
    pub X0: f32,
    pub Y0: f32,
    pub X1: f32,
    pub Y1: f32,
    // float           U0, V0, U1, V1;     // Texture coordinates
    pub U0: f32,
    pub V0: f32,
    pub U1: f32,
    pub V1: f32,
}

// Helper to build glyph ranges from text/string data. Feed your application strings/characters to it then call BuildRanges().
// This is essentially a tightly packed of vector of 64k booleans = 8KB storage.
#[derive(Clone,Debug,Default)]
pub struct ImFontGlyphRangesBuilder
{
    pub UsedChars: Vec<u32>, //ImVector<ImU32> UsedChars;            // Store 1-bit per Unicode code point (0=unused, 1=used)


}

impl ImFontGlyphRangesBuilder {
    // ImFontGlyphRangesBuilder()              { clear(); }
    //     inline void     clear()                 { int size_in_bytes = (IM_UNICODE_CODEPOINT_MAX + 1) / 8; UsedChars.resize(size_in_bytes / sizeof); memset(UsedChars.data, 0, (size_t)size_in_bytes); }
    pub fn Clear(&mut self) {
        self.UsedChars.clear()
    }
    //     inline bool     GetBit(size_t n) const  { int off = (n >> 5); ImU32 mask = 1u << (n & 31); return (UsedChars[off] & mask) != 0; }  // Get bit n in the array
    pub fn GetBit(&mut self, n: usize) -> bool {
        let off = n >> 5;
        let mask: u32 = 1 << (n * 31);
        self.UsedChars[off] & mask != 0
    }
    //     inline void     SetBit(size_t n)        { int off = (n >> 5); ImU32 mask = 1u << (n & 31); UsedChars[off] |= mask; }               // Set bit n in the array
    pub fn SetBit(&mut self, n: usize) {
        let off = n >> 5;
        let mask: u32 = 1 << (n & 31);
        self.UsedChars[off] |= mask;
    }
    //     inline void     AddChar(ImWchar c)      { SetBit(c); }                      // Add character
    pub fn AddChar(&mut self, c: u8) {
        self.SetBit(c as usize)
    }
    //      void  AddText(const char* text, const char* text_end = NULL);     // Add string (each character of the UTF-8 string are added)
    pub fn AddText(&mut self, text: &String) {
        todo!()
    }
    //      void  AddRanges(const ImWchar* ranges);                           // Add ranges, e.g. builder.AddRanges(ImFontAtlas::GetGlyphRangesDefault()) to force add all of ASCII/Latin+Ext
    pub fn AddRanges(&mut self, ranges: &[ImWchar]) {
        todo!()
    }
    //      void  BuildRanges(ImVector<ImWchar>* out_ranges);                 // Output new ranges
    pub fn BuildRanges(&mut self, out_ranges: &mut Vec<ImWchar>) {
        todo!()
    }
}

// See ImFontAtlas::AddCustomRectXXX functions.
#[derive(Default,Debug,Clone)]
pub struct ImFontAtlasCustomRect
{
    // unsigned short  Width, Height;  // Input    // Desired rectangle dimension
    pub Width: u16,
    pub Height: u16,
    // unsigned short  X, Y;           // Output   // Packed position in Atlas
    pub X: u16,
    pub Y: u16,
    // unsigned pub GlyphID: i32,      // Input    // For custom font glyphs only (ID < 0x110000)
    pub GlyphID: u32,
    pub GlyphAdvanceX: f32, // Input    // For custom font glyphs only: glyph xadvance
    pub GlyphOffset: DimgVec2D,    // Input    // For custom font glyphs only: glyph display offset
    pub Font: DimgFont, // ImFont*         font;           // Input    // For custom font glyphs only: target font

}

impl ImFontAtlasCustomRect {
    // ImFontAtlasCustomRect()         { Width = Height = 0; X = Y = 0xFFFF; GlyphID = 0; GlyphAdvanceX = 0.0; GlyphOffset = ImVec2(0, 0); font = NULL;
    pub fn new() -> Self {
        Self {
            Width: 0,
            Height: 0,
            X: 0xFFFF,
            Y: 0xFFFF,
            GlyphID: 0,
            GlyphAdvanceX: 0.0,
            GlyphOffset: Default::default(),
            Font: Default::default(),
        }
    }
    //     bool IsPacked() const           { return X != 0xFFFF; }
    pub fn IsPacked(&self) -> bool {
        self.X != 0xFFFF
    }
}

// flags for ImFontAtlas build
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum ImFontAtlasFlags
{
    ImFontAtlasFlags_None               = 0,
    ImFontAtlasFlags_NoPowerOfTwoHeight = 1 << 0,   // Don't round the height to next power of two
    ImFontAtlasFlags_NoMouseCursors     = 1 << 1,   // Don't build software mouse cursors into the atlas (save a little texture memory)
    ImFontAtlasFlags_NoBakedLines       = 1 << 2    // Don't build thick line textures into the atlas (save a little texture memory, allow support for point/nearest filtering). The AntiAliasedLinesUseTex features uses them, otherwise they will be rendered using polygons (more expensive for CPU/GPU).
}

// Load and rasterize multiple TTF/OTF fonts into a same texture. The font atlas will build a single texture holding:
//  - One or more fonts.
//  - Custom graphics data needed to render the shapes needed by Dear ImGui.
//  - Mouse cursor shapes for software cursor rendering (unless setting 'flags |= ImFontAtlasFlags_NoMouseCursors' in the font atlas).
// It is the user-code responsibility to setup/build the atlas, then upload the pixel data into a texture accessible by your graphics api.
//  - Optionally, call any of the AddFont*** functions. If you don't call any, the default font embedded in the code will be loaded for you.
//  - Call GetTexDataAsAlpha8() or GetTexDataAsRGBA32() to build and retrieve pixels data.
//  - Upload the pixels data into a texture within your graphics system (see imgui_impl_xxxx.cpp examples)
//  - Call SetTexID(my_tex_id); and pass the pointer/identifier to your texture in a format natural to your graphics API.
//    This value will be passed back to you during rendering to identify the texture. Read FAQ entry about ImTextureID for more details.
// Common pitfalls:
// - If you pass a 'glyph_ranges' array to AddFont*** functions, you need to make sure that your array persist up until the
//   atlas is build (when calling GetTexData*** or Build()). We only copy the pointer, not the data.
// - Important: By default, AddFontFromMemoryTTF() takes ownership of the data. Even though we are not writing to it, we will free the pointer on destruction.
//   You can set font_cfg->FontDataOwnedByAtlas=false to keep ownership of your data and it won't be freed,
// - Even though many functions are suffixed with "TTF", OTF data is supported just as well.
// - This is an old API and it is currently awkward for those and and various other reasons! We will address them in the future!
#[derive(Clone,Debug,Default)]
pub struct ImFontAtlas
{
    //-------------------------------------------
    // Members
    //-------------------------------------------

    pub Flags: ImFontAtlasFlags, // ImFontAtlasFlags            flags;              // Build flags (see ImFontAtlasFlags_)
    pub TexID: DimgTextureId, // ImTextureID                 TexID;              // User data to refer to the texture once it has been uploaded to user's graphic systems. It is passed back to you during rendering via the ImDrawCmd structure.
    pub TexDesiredWidth: i32,  // Texture width desired by user before Build(). Must be a power-of-two. If have many glyphs your graphics API have texture size restrictions you may want to increase texture width to decrease height.
    pub TexGlyphPadding: i32,  // Padding between glyphs within texture in pixels. Defaults to 1. If your rendering method doesn't rely on bilinear filtering you may set this to 0 (will also need to set AntiAliasedLinesUseTex = false).
    pub Locked: bool,             // Marked as Locked by ImGui::NewFrame() so attempt to modify the atlas will assert.

    // [Internal]
    // NB: Access texture data via GetTexData*() calls! Which will setup a default font for you.
    pub TexReady: bool,           // Set when texture was built matching current font input
    pub TexPixelsUseColors: bool, // Tell whether our texture data is known to use colors (rather than just alpha channel), in order to help backend select a format.
    pub TexPixelsAlpha8: Vec<u8>, // unsigned char*              TexPixelsAlpha8;    // 1 component per pixel, each component is unsigned 8-bit. Total size = TexWidth * TexHeight
    pub TexPixelsRGBA32: Vec<u32>, // unsigned int*               TexPixelsRGBA32;    // 4 component per pixel, each component is unsigned 8-bit. Total size = TexWidth * TexHeight * 4
    pub TexWidth: i32,         // Texture width calculated during Build().
    pub TexHeight: i32,        // Texture height calculated during Build().
    pub TexUvScale: DimgVec2D,         // = (1.0/TexWidth, 1.0/TexHeight)
    pub TexUvWhitePixel: DimgVec2D,    // Texture coordinates to a white pixel
    pub Fonts: Vec<DimgFont>, // ImVector<ImFont*>           Fonts;              // Hold all the fonts returned by AddFont*. Fonts[0] is the default font upon calling ImGui::NewFrame(), use ImGui::PushFont()/PopFont() to change the current font.
    // ImVector<ImFontAtlasCustomRect> CustomRects;    // Rectangles for packing custom texture data into the atlas.
    pub CustomRects: Vec<ImFontAtlasCustomRect>,
// ImVector<ImFontConfig>      ConfigData;         // Configuration data
    pub ConfigData: Vec<ImFontConfig>,
    // ImVec4                      TexUvLines[IM_DRAWLIST_TEX_LINES_WIDTH_MAX + 1];  // UVs for baked anti-aliased lines
    pub TexUvLines: Vec<ImVec4>,

    // [Internal] font builder
    // const ImFontBuilderIO*      FontBuilderIO;      // Opaque interface to a font builder (default to stb_truetype, can be changed to use FreeType by defining IMGUI_ENABLE_FREETYPE).
    pub FontBuilderIO: ImFontBuilderIO,
    // unsigned pub FontBuilderFlags: i32, // Shared flags (for all fonts) for custom font builder. THIS IS BUILD IMPLEMENTATION DEPENDENT. Per-font override is also available in ImFontConfig.
    pub FontBuilderFlags: i32,

    // [Internal] Packing data
    // int                         PackIdMouseCursors; // Custom texture rectangle ID for white pixel and mouse cursors
    pub PackIdMouseCursors: i32,
    pub PackIdLines: i32,      // Custom texture rectangle ID for baked anti-aliased lines

    // [Obsolete]
    //typedef ImFontAtlasCustomRect    CustomRect;         // OBSOLETED in 1.72+
    //typedef ImFontGlyphRangesBuilder GlyphRangesBuilder; // OBSOLETED in 1.67+
}

impl ImFontAtlas {
    //  ImFontAtlas();
    //      ~ImFontAtlas();
    //      ImFont*           AddFont(const ImFontConfig* font_cfg);
    pub fn AddFont(&mut self, font_cfg: &ImFontConfig) -> DimgFont {
        todo!()
    }
    //      ImFont*           AddFontDefault(const ImFontConfig* font_cfg = NULL);
    pub fn AddFontDefault(&mut self, font_cfg: &ImFontConfig) -> DimgFont {
        todo!()
    }
    //      ImFont*           AddFontFromFileTTF(const char* filename, float size_pixels, const ImFontConfig* font_cfg = NULL, const ImWchar* glyph_ranges = NULL);
    pub fn AddFontFileTTF(&mut self, filename: &String, size_pixels: f32, font_cfg: &ImFontConfig, glyph_ranges: &[ImWchar]) -> DimgFont {
        todo!()
    }
    //      ImFont*           AddFontFromMemoryTTF(void* font_data, int font_size, float size_pixels, const ImFontConfig* font_cfg = NULL, const ImWchar* glyph_ranges = NULL); // Note: Transfer ownership of 'ttf_data' to ImFontAtlas! Will be deleted after destruction of the atlas. Set font_cfg->FontDataOwnedByAtlas=false to keep ownership of your data and it won't be freed.
    pub fn AddFontFromMemoryTTF(&mut self, font_data: &Vec<u8>, font_size: i32, size_pixels: f32, font_cfg: &ImFontConfig, glyph_ranges: &[ImWchar]) -> DimgFont {
        todo!()
    }
    //      ImFont*           AddFontFromMemoryCompressedTTF(const void* compressed_font_data, int compressed_font_size, float size_pixels, const ImFontConfig* font_cfg = NULL, const ImWchar* glyph_ranges = NULL); // 'compressed_font_data' still owned by caller. Compress with binary_to_compressed_c.cpp.
    pub fn AddFontFromMemoryCompressedTTF(&mut self, compressed_font_data: &Vec<u8>, compressed_font_size: usize, size_pixels: f32, font_config: &ImFontConfig, glyph_ranges: &Vec<ImWchar>) -> DimgFont {
        todo!()
    }
    //      ImFont*           AddFontFromMemoryCompressedBase85TTF(const char* compressed_font_data_base85, float size_pixels, const ImFontConfig* font_cfg = NULL, const ImWchar* glyph_ranges = NULL);              // 'compressed_font_data_base85' still owned by caller. Compress with binary_to_compressed_c.cpp with -base85 parameter.
    pub fn AddFontFromMemoryCompressedBase85TTF(&mut self, compressed_font_data_base85: &String, size_pixels: f32, font_cfg: &ImFontConfig, glyph_ranges: &Vec<ImWchar>) -> DimgFont {
        todo!()
    }
    //      void              ClearInputData();           // clear input data (all ImFontConfig structures including sizes, TTF data, glyph ranges, etc.) = all the data used to build the texture and fonts.
    pub fn ClearInputData(&mut self) {
        todo!()
    }
    //      void              ClearTexData();             // clear output texture data (CPU side). Saves RAM once the texture has been copied to graphics memory.
    pub fn ClearTexData(&mut self) {
        todo!()
    }
    //      void              ClearFonts();               // clear output font data (glyphs storage, UV coordinates).
    pub fn ClearFonts(&mut self) {
        todo!()
    }
    //      void              clear();                    // clear all input and output.
    pub fn Clear(&mut self) {
        todo!()
    }
    //
    //     // Build atlas, retrieve pixel data.
    //     // User is in charge of copying the pixels into graphics memory (e.g. create a texture with your engine). Then store your texture handle with SetTexID().
    //     // The pitch is always = Width * BytesPerPixels (1 or 4)
    //     // Building in RGBA32 format is provided for convenience and compatibility, but note that unless you manually manipulate or copy color data into
    //     // the texture (e.g. when using the AddCustomRect*** api), then the RGB pixels emitted will always be white (~75% of memory/bandwidth waste.
    //      bool              Build();                    // Build pixels data. This is called automatically for you by the GetTexData*** functions.
    pub fn Build(&mut self) {
        todo!()
    }
    //      void              GetTexDataAsAlpha8(unsigned char** out_pixels, int* out_width, int* out_height, int* out_bytes_per_pixel = NULL);  // 1 byte per-pixel
    pub fn GetTextDataAsAlpha8(&mut self, out_pixels: &Vec<Vec<u8>>, out_width: &mut i32, out_height: &mut i32, out_bytes_per_pixel: &mut i32) {
        todo!()
    }
    //      void              GetTexDataAsRGBA32(unsigned char** out_pixels, int* out_width, int* out_height, int* out_bytes_per_pixel = NULL);  // 4 bytes-per-pixel
    pub fn GetTextDataAsRGBA32(&mut self, out_pixels: &Vec<Vec<u8>>, out_width: &mut i32, out_height: &mut i32, out_bytes_per_pixel: &mut i32) {
        todo!()
    }
    //     bool                        IsBuilt() const             { return Fonts.size > 0 && TexReady; } // Bit ambiguous: used to detect when user didn't built texture but effectively we should check TexID != 0 except that would be backend dependent...
    pub fn IsBuilt(&self) -> bool {
        self.Fonts.len() > 0 && self.TexReady
    }
    //     void                        SetTexID(ImTextureID id)    { TexID = id; }
    pub fn SetTexID(&mut self, id: DimgTextureId) {
        self.TexID = id
    }
    //
    //     //-------------------------------------------
    //     // Glyph Ranges
    //     //-------------------------------------------
    //
    //     // Helpers to retrieve list of common Unicode ranges (2 value per range, values are inclusive, zero-terminated list)
    //     // NB: Make sure that your string are UTF-8 and NOT in your local code page. In C++11, you can create UTF-8 string literal using the u8"Hello world" syntax. See FAQ for details.
    //     // NB: Consider using ImFontGlyphRangesBuilder to build glyph ranges from textual data.
    //      const ImWchar*    GetGlyphRangesDefault();                // Basic Latin, Extended Latin
    pub fn GetGlyphRangesDefault(&self) -> Vec<ImWchar> {
        todo!()
    }
    //      const ImWchar*    GetGlyphRangesKorean();                 // Default + Korean characters
    pub fn GetGlyphRangesKorean(&self) -> Vec<ImWchar> {
        todo!()
    }
    //      const ImWchar*    GetGlyphRangesJapanese();               // Default + Hiragana, Katakana, Half-Width, Selection of 2999 Ideographs
    pub fn GetGlyphRangesJapanese(&self) -> Vec<ImWchar> {
        todo!()
    }
    //      const ImWchar*    GetGlyphRangesChineseFull();            // Default + Half-Width + Japanese Hiragana/Katakana + full set of about 21000 CJK Unified Ideographs
    pub fn GetGlyphRangesChineseFull(&self) -> Vec<ImWchar> {
        todo!()
    }
    //      const ImWchar*    GetGlyphRangesChineseSimplifiedCommon();// Default + Half-Width + Japanese Hiragana/Katakana + set of 2500 CJK Unified Ideographs for common simplified Chinese
    pub fn GetGlyphRangesChineseSimplifiedCommon(&self) -> Vec<ImWchar> {
        todo!()
    }
    //      const ImWchar*    GetGlyphRangesCyrillic();               // Default + about 400 Cyrillic characters
    pub fn GetGlyphRangesCyrillic(&self) -> Vec<ImWchar> {
        todo!()
    }
    //      const ImWchar*    GetGlyphRangesThai();                   // Default + Thai characters
    pub fn GetGlyphRangesThai(&self) -> Vec<ImWchar> {
        todo!()
    }
    //      const ImWchar*    GetGlyphRangesVietnamese();             // Default + Vietnamese characters
    pub fn GetGlyphRangesVietnamese(&self) -> Vec<ImWchar> {
        todo!()
    }
    //
    //     //-------------------------------------------
    //     // [BETA] Custom Rectangles/Glyphs API
    //     //-------------------------------------------
    //
    //     // You can request arbitrary rectangles to be packed into the atlas, for your own purposes.
    //     // - After calling Build(), you can query the rectangle position and render your pixels.
    //     // - If you render colored output, set 'atlas->TexPixelsUseColors = true' as this may help some backends decide of prefered texture format.
    //     // - You can also request your rectangles to be mapped as font glyph (given a font + Unicode point),
    //     //   so you can render e.g. custom colorful icons and use them as regular glyphs.
    //     // - Read docs/FONTS.md for more details about using colorful icons.
    //     // - Note: this API may be redesigned later in order to support multi-monitor varying DPI settings.
    //      int               AddCustomRectRegular(int width, int height);
    pub fn AddCustomRegular(&mut self, width: i32, height: i32) -> i32 {
        todo!()
    }
    //      int               AddCustomRectFontGlyph(ImFont* font, ImWchar id, int width, int height, float advance_x, const ImVec2& offset = ImVec2(0, 0));
    pub fn AddCustomRectFontGlyph(&mut self, font: &DimgFont, id: ImWchar, width: i32, height: i32, advance_x: f32, offset: &DimgVec2D) -> i32 {
        todo!()
    }
    //     ImFontAtlasCustomRect*      GetCustomRectByIndex(int index) { IM_ASSERT(index >= 0); return &CustomRects[index]; }
    pub fn GetCustomRectByIndex(&mut self, index: i32) -> Result<ImFontAtlasCustomRect, String> {
        if index >= 0 {
            Ok(self.CustomRects[index])
        }
        Err(format!("Invalid index arg: {}", index))
    }
    //
    //     // [Internal]
    //      void              CalcCustomRectUV(const ImFontAtlasCustomRect* rect, ImVec2* out_uv_min, ImVec2* out_uv_max) const;
    pub fn CalcCustomRectUV(&mut self, rect: &ImFontAtlastCustomRect, out_uv_min: &DimgVec2D, out_uv_max: &DimgVec2D) {
        todo!()
    }
    //      bool              GetMouseCursorTexData(ImGuiMouseCursor cursor, ImVec2* out_offset, ImVec2* out_size, ImVec2 out_uv_border[2], ImVec2 out_uv_fill[2]);
    pub fn GetMouseCursorTexData(&mut self, cursor: DimgMouseCursor, out_offset: &mut DimgVec2D, out_size: &mut DimgVec2D, out_uv_border: &mut [DimgVec2D;2], out_uv_fill: &mut [DimgVec2D;2] ) -> bool {
        todo!()
    }
}


// font runtime data and rendering
// ImFontAtlas automatically loads a default embedded font for you when you call GetTexDataAsAlpha8() or GetTexDataAsRGBA32().
#[derive(Debug,Clone,Default)]
pub struct DimgFont
{
    // Members: Hot ~20/24 bytes (for CalcTextSize)
    pub IndexAdvanceX: Vec<f32>, // ImVector<float>             IndexAdvanceX;      // 12-16 // out //            // Sparse. Glyphs->AdvanceX in a directly indexable way (cache-friendly for CalcTextSize functions which only this this info, and are often bottleneck in large UI).
    pub FallbackAdvanceX: f32,  // 4     // out // = FallbackGlyph->AdvanceX
    pub FontSize: f32,          // 4     // in  //            // Height of characters/line, set during loading (don't change after loading)

    // Members: Hot ~28/40 bytes (for CalcTextSize + render loop)
    pub IndexLookup: Vec<ImWchar>, //ImVector<ImWchar>           IndexLookup;        // 12-16 // out //            // Sparse. Index glyphs by Unicode code-point.
    pub Glyphs: Vec<ImFontGlyph>, // ImVector<ImFontGlyph>       Glyphs;             // 12-16 // out //            // All glyphs.
    pub FallbackGlyph: ImFontGlyph, // const ImFontGlyph*          FallbackGlyph;      // 4-8   // out // = FindGlyph(FontFallbackChar)

    // Members: Cold ~32/40 bytes
    pub ContainerAtlas: Option<ImFontAtlas>, // ImFontAtlas*                ContainerAtlas;     // 4-8   // out //            // What we has been loaded into
    // const ImFontConfig*         ConfigData;         // 4-8   // in  //            // Pointer within ContainerAtlas->ConfigData
    pub ConfigData: Option<ImFontConfig>,
// short                       ConfigDataCount;    // 2     // in  // ~ 1        // Number of ImFontConfig involved in creating this font. Bigger than 1 when merging multiple font sources into one ImFont.
    pub ConfigDataCount: isize,
    // ImWchar                     FallbackChar;       // 2     // out // = FFFD/'?' // Character used if a glyph isn't found.
    pub FallbackChar: ImWchar,
    // ImWchar                     EllipsisChar;       // 2     // out // = '...'    // Character used for ellipsis rendering.
    pub EllipsisChar: ImWchar,
    // ImWchar                     DotChar;            // 2     // out // = '.'      // Character used for ellipsis rendering (if a single '...' character isn't found)
    pub DotChar: ImWchar,
    pub DirtyLookupTables: bool,  // 1     // out //
    pub Scale: f32,             // 4     // in  // = 1.f      // Base font scale, multiplied by the per-window font scale which you can adjust with SetWindowFontScale()
    // float                       Ascent, Descent;    // 4+4   // out //            // Ascent: distance from top to bottom of e.g. 'A' [0..font_size]
    pub Ascent: f32,
    pub Descent: f32,
// int                         MetricsTotalSurface;// 4     // out //            // Total surface in pixels to get an idea of the font rasterization/texture cost (not exact, we approximate the cost of padding between glyphs)
    pub MetricsTotalSurface: i32,
    // ImU8                        Used4kPagesMap[(IM_UNICODE_CODEPOINT_MAX+1)/4096/8]; // 2 bytes if ImWchar=ImWchar16, 34 bytes if ImWchar==ImWchar32. Store 1-bit for each block of 4K
    // codepoints that has one active glyph. This is mainly used to facilitate iterations across all used codepoints.
    pub Used4kPagesMap: Vec<u8>,
    // Methods

}

impl DimgFont {
    //  ImFont();
    //      ~ImFont();
    //      const ImFontGlyph*FindGlyph(ImWchar c) const;
    pub fn FindGlyph(&self, c: ImWchar) -> ImFontGlyph {
        todo!()
    }
    //      const ImFontGlyph*FindGlyphNoFallback(ImWchar c) const;
    pub fn FindGlyphNoFallback(&self, c: ImWchar) -> ImFontGlyph {
        todo!()
    }
    //     float                       GetCharAdvance(ImWchar c) const     { return (c < IndexAdvanceX.size) ? IndexAdvanceX[c] : FallbackAdvanceX; }
    pub fn GetCharAdvance(&self, c: ImWchar) -> f32 {
        if c < self.IndexAdvanceX.len() as ImWchar {
            self.IndexAdvanceX[c]
        }
        self.FallbackAdvanceX
    }
    //     bool                        IsLoaded() const                    { return ContainerAtlas != NULL; }
    pub fn IsLoaded(&self) -> bool {
        self.ContainerAtlas.is_some()
    }
    //     const char*                 GetDebugName() const                { return ConfigData ? ConfigData->Name : "<unknown>"; }
    pub fn GetDebugName(&self) -> String {
        if self.ConfigData.is_some() {
            self.ConfigData.unwrap().Name
        }
        "<unknown>".to_string()
    }
    //
    //     // 'max_width' stops rendering after a certain width (could be turned into a 2d size). FLT_MAX to disable.
    //     // 'wrap_width' enable automatic word-wrapping across multiple lines to fit into given width. 0.0 to disable.
    //      ImVec2            CalcTextSizeA(float size, float max_width, float wrap_width, const char* text_begin, const char* text_end = NULL, const char** remaining = NULL) const; // utf8
    pub fn CalcTextSizeA(&self, size: f32, max_width: f32, wrap_width: f32, text: &String) -> DimgVec2D {
        todo!()
    }
    //      const char*       CalcWordWrapPositionA(float scale, const char* text, const char* text_end, float wrap_width) const;
    pub fn CalcWordWrapPositionA(&self, scale: f32, text: &String, wrap_width: f32) -> String{
        todo!()
    }
    //      void              RenderChar(ImDrawList* draw_list, float size, const ImVec2& pos, ImU32 col, ImWchar c) const;
    pub fn RenderChar(&self, draw_list: &ImDrawList, size: f32, pos: &DimgVec2D, col: u32, c: ImWchar) {
        todo!()
    }
    //      void              RenderText(ImDrawList* draw_list, float size, const ImVec2& pos, ImU32 col, const ImVec4& clip_rect, const char* text_begin, const char* text_end, float wrap_width = 0.0, bool cpu_fine_clip = false) const;
    pub fn RenderText(&self, draw_list: &mut DrawList, size: f32, pos: &DimgVec2D, col: u32, clip_rect: &ImVec4, text: &String, wrap_width: f32, cpu_fine_clip: bool) {
        todo!()
    }
    //
    //     // [Internal] Don't use!
    //      void              BuildLookupTable();
    pub fn BuildLookupTable(&mut self) {
        todo!()
    }
    //      void              ClearOutputData();
    pub fn ClearOutputData(&mut self) {
        todo!()
    }
    //      void              GrowIndex(int new_size);
    pub fn GrowIndex(&mut self) {
        todo!()
    }
    //      void              AddGlyph(const ImFontConfig* src_cfg, ImWchar c, float x0, float y0, float x1, float y1, float u0, float v0, float u1, float v1, float advance_x);
    pub fn AddGlyph(&mut self, src_cfg: &ImFontConfig, c: ImWchar, x0: f32, y0: f32, x1: f32, y1: f32, u0: f32, v0: f32, u1: f32, v1: f32, advance_x: f32) {
        todo!()
    }
    //      void              AddRemapChar(ImWchar dst, ImWchar src, bool overwrite_dst = true); // Makes 'dst' character/glyph points to 'src' character/glyph. Currently needs to be called AFTER fonts have been built.
    pub fn AddRemapChar(&mut self, dst: ImWchar, src: ImWchar, overwrite_dst: bool){
        todo!()
    }
    //      void              SetGlyphVisible(ImWchar c, bool visible);
    pub fn SetGlyphVisible(&mut self, c: ImWchar, visible: bool) {
        todo!()
    }
    //      bool              IsGlyphRangeUnused(unsigned int c_begin, unsigned int c_last);
    pub fn IsGlyphRangeUnused(&mut self, c_begin: u32, c_lst: u32) -> bool {
        todo!()
    }
}

//-----------------------------------------------------------------------------
// [SECTION] viewports
//-----------------------------------------------------------------------------

// flags stored in ImGuiViewport::flags, giving indications to the platform backends.
pub enum DimgViewportFlags
{
    ImGuiViewportFlags_None                     = 0,
    ImGuiViewportFlags_IsPlatformWindow         = 1 << 0,   // Represent a Platform Window
    ImGuiViewportFlags_IsPlatformMonitor        = 1 << 1,   // Represent a Platform Monitor (unused yet)
    ImGuiViewportFlags_OwnedByApp               = 1 << 2,   // Platform Window: is created/managed by the application (rather than a dear imgui backend)
    ImGuiViewportFlags_NoDecoration             = 1 << 3,   // Platform Window: Disable platform decorations: title bar, borders, etc. (generally set all windows, but if ImGuiConfigFlags_ViewportsDecoration is set we only set this on popups/tooltips)
    ImGuiViewportFlags_NoTaskBarIcon            = 1 << 4,   // Platform Window: Disable platform task bar icon (generally set on popups/tooltips, or all windows if ImGuiConfigFlags_ViewportsNoTaskBarIcon is set)
    ImGuiViewportFlags_NoFocusOnAppearing       = 1 << 5,   // Platform Window: Don't take focus when created.
    ImGuiViewportFlags_NoFocusOnClick           = 1 << 6,   // Platform Window: Don't take focus when clicked on.
    ImGuiViewportFlags_NoInputs                 = 1 << 7,   // Platform Window: Make mouse pass through so we can drag this window while peaking behind it.
    ImGuiViewportFlags_NoRendererClear          = 1 << 8,   // Platform Window: Renderer doesn't need to clear the framebuffer ahead (because we will fill it entirely).
    ImGuiViewportFlags_TopMost                  = 1 << 9,   // Platform Window: Display on top (for tooltips only).
    ImGuiViewportFlags_Minimized                = 1 << 10,  // Platform Window: Window is minimized, can skip render. When minimized we tend to avoid using the viewport pos/size for clipping window or testing if they are contained in the viewport.
    ImGuiViewportFlags_NoAutoMerge              = 1 << 11,  // Platform Window: Avoid merging this window into another host window. This can only be set via ImGuiWindowClass viewport flags override (because we need to now ahead if we are going to create a viewport in the first place!).
    ImGuiViewportFlags_CanHostOtherWindows      = 1 << 12   // Main viewport: can host multiple imgui windows (secondary viewports are associated to a single window).
}

// - Currently represents the Platform Window created by the application which is hosting our Dear ImGui windows.
// - With multi-viewport enabled, we extend this concept to have multiple active viewports.
// - In the future we will extend this concept further to also represent Platform Monitor and support a "no main platform window" operation mode.
// - About Main Area vs Work Area:
//   - Main Area = entire viewport.
//   - Work Area = entire viewport minus sections used by main menu bars (for platform windows), or by task bar (for platform monitor).
//   - windows are generally trying to stay within the Work Area of their host viewport.
#[derive(Debug,Clone,Default)]
pub struct DimgViewport
{
    pub ID: DimgId,                   // Unique identifier for the viewport
    pub Flags: DimgViewportFlags, //ImGuiViewportFlags  flags;                  // See ImGuiViewportFlags_
    pub Pos: DimgVec2D,                    // Main Area: Position of the viewport (Dear ImGui coordinates are the same as OS desktop/native coordinates)
    pub Size: DimgVec2D,                   // Main Area: size of the viewport.
    pub WorkPos: DimgVec2D,                // Work Area: Position of the viewport minus task bars, menus bars, status bars (>= pos)
    pub WorkSize: DimgVec2D,               // Work Area: size of the viewport minus task bars, menu bars, status bars (<= size)
    pub DpiScale: f32,              // 1.0 = 96 DPI = No extra scale.
    pub ParentViewportId: DimgId,     // (Advanced) 0: no parent. Instruct the platform backend to setup a parent/child relationship between platform windows.
    // ImDrawData*         DrawData;               // The ImDrawData corresponding to this viewport. Valid after Render() and until the next call to NewFrame().
    pub DrawData: ImDrawData,

    // Platform/Backend Dependent data
    // Our design separate the Renderer and Platform backends to facilitate combining default backends with each others.
    // When our create your own backend for a custom engine, it is possible that both Renderer and Platform will be handled
    // by the same system and you may not need to use all the user_data/Handle fields.
    // The library never uses those fields, they are merely storage to facilitate backend implementation.
    // void*               RendererUserData;       // void* to hold custom data structure for the renderer (e.g. swap chain, framebuffers etc.). generally set by your Renderer_CreateWindow function.
    pub RendererUserData: Vec<u8>,
    // void*               PlatformUserData;       // void* to hold custom data structure for the OS / platform (e.g. windowing info, render context). generally set by your Platform_CreateWindow function.
    pub PlatformuserData: Vec<u8>,
    // void*               PlatformHandle;         // void* for FindViewportByPlatformHandle(). (e.g. suggested to use natural platform handle such as HWND, GLFWWindow*, SDL_Window*)
    pub PlatformHandle: Vec<u8>,
    // void*               PlatformHandleRaw;      // void* to hold lower-level, platform-native window handle (under Win32 this is expected to be a HWND, unused for other platforms), when using an abstraction layer like GLFW or SDL (where PlatformHandle would be a SDL_Window*)
    pub PlatformHandleRaw: Vec<u8>,
    pub PlatformRequestMove: bool,    // Platform window requested move (e.g. window was moved by the OS / host window manager, authoritative position will be OS window position)
    pub PlatformRequestResize: bool,  // Platform window requested resize (e.g. window was resized by the OS / host window manager, authoritative size will be OS window size)
    pub PlatformRequestClose: bool,   // Platform window requested closure (e.g. window was moved by the OS / host window manager, e.g. pressing ALT-F4)

    // ImGuiViewport()     { memset(this, 0, sizeof(*this)); }
    // ~ImGuiViewport()    { IM_ASSERT(PlatformUserData == NULL && RendererUserData == NULL); }

    // Helpers

}

impl DimgViewport {
    // ImVec2              GetCenter() const       { return ImVec2(pos.x + size.x * 0.5, pos.y + size.y * 0.5); }
    // ImVec2              GetWorkCenter() const   { return ImVec2(WorkPos.x + WorkSize.x * 0.5, WorkPos.y + WorkSize.y * 0.5); }
    pub fn GetCenter(&self) -> DimgVec2D {
        DimgVec2D {
            x: self.Pos.x + self.Size.x * 0.5,
            y: self.Pos.y + self.Size.y * 0.5
        }
    }

    pub fn GetWorkCenter(&self) -> DimgVec2D {
        DimgVec2D {
            x: self.WorkPos.x + self.WorkSize.x * 0.5,
            y: self.WorkPos.y + self.WorkSize.y * 0.5
        }
    }
}

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
// - Application:  Enable feature with 'io.ConfigFlags |= ImGuiConfigFlags_ViewportsEnable'.
// - Backend:      The backend initialization will setup all necessary ImGuiPlatformIO's functions and update monitors info every frame.
// - Application:  In your main loop, call ImGui::UpdatePlatformWindows(), ImGui::RenderPlatformWindowsDefault() after EndFrame() or Render().
// - Application:  Fix absolute coordinates used in ImGui::SetWindowPos() or ImGui::SetNextWindowPos() calls.
//
// Steps to use multi-viewports in your application, when using a custom backend:
// - Important:    THIS IS NOT EASY TO DO and comes with many subtleties not described here!
//                 It's also an experimental feature, so some of the requirements may evolve.
//                 Consider using default backends if you can. Either way, carefully follow and refer to examples/ backends for details.
// - Application:  Enable feature with 'io.ConfigFlags |= ImGuiConfigFlags_ViewportsEnable'.
// - Backend:      Hook ImGuiPlatformIO's Platform_* and Renderer_* callbacks (see below).
//                 Set 'io.BackendFlags |= ImGuiBackendFlags_PlatformHasViewports' and 'io.BackendFlags |= ImGuiBackendFlags_PlatformHasViewports'.
//                 Update ImGuiPlatformIO's Monitors list every frame.
//                 Update MousePos every frame, in absolute coordinates.
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

// (Optional) This is required when enabling multi-viewport. Represent the bounds of each connected monitor/display and their DPI.
// We use this information for multiple DPI support + clamping the position of popups and tooltips so they don't straddle multiple monitors.
#[derive(Debug,Clone,Default)]
pub struct DimgPlatformMonitor
{
    // ImVec2  MainPos, MainSize;      // Coordinates of the area displayed on this monitor (Min = upper left, Max = bottom right)
    pub MainPos: DimgVec2D,
    pub MainSize: DimgVec2D,
    // ImVec2  WorkPos, WorkSize;      // Coordinates without task bars / side bars / menu bars. Used to avoid positioning popups/tooltips inside this region. If you don't have this info, please copy the value for MainPos/MainSize.
    pub WorkPos: DimgVec2D,
    pub WorkSize: DimgVec2D,
    pub DpiScale: f32,              // 1.0 = 96 DPI
    // ImGuiPlatformMonitor()          { MainPos = MainSize = WorkPos = WorkSize = ImVec2(0, 0); DpiScale = 1.0; }
}

impl DimgPlatformMonitor {
    pub fn new() -> Self {
        Self {
            MainPos: Default::default(),
            MainSize: Default::default(),
            WorkPos: Default::default(),
            WorkSize: Default::default(),
            DpiScale: 1.0
        }
    }
}

// (Optional) Support for IME (Input Method Editor) via the io.SetPlatformImeDataFn() function.
#[derive(Debug,Default,Clone)]
pub struct DimgPlatformImeData
{
    pub WantVisible: bool,        // A widget wants the IME to be visible
    pub InputPos: DimgVec2D,           // Position of the input cursor
    pub InputLineHeight: f32,   // Line height

    // ImGuiPlatformImeData() { memset(this, 0, sizeof(*this)); }
}

impl DimgPlatformImeData {
    pub fn new(initial_input_pos: DimgVec2D) -> Self {
        Self {
            WantVisible: false,
            InputPos: initial_input_pos,
            InputLineHeight: 0.0
        }
    }
}

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

// Include imgui_user.h at the end of imgui.h (convenient for user to only explicitly include vanilla imgui.h)
// #ifdef IMGUI_INCLUDE_IMGUI_USER_H
// #include "imgui_user.h"
// #endif
//
// #endif // #ifndef IMGUI_DISABLE
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgNavLayer
{
    Main,    // Main scrolling layer
    Menu,    // Menu layer (access with Alt/ImGuiNavInput_Menu)

}


// Simple column measurement, currently used for MenuItem() only.. This is very short-sighted/throw-away code and NOT a generic helper.
#[derive(Debug,Clone,Default)]
pub struct  ImGuiMenuColumns
{
    // ImU32       TotalWidth;
    pub TotalWidth: u32,
    // ImU32       NextTotalWidth;
    pub NextTotalWidth: u32,
    // ImU16       Spacing;
    pub Spacing: u16,
    // ImU16       OffsetIcon;         // Always zero for now
    pub OffsetIcon: u16,
    // ImU16       OffsetLabel;        // Offsets are locked in Update()
    pub OffsetLabel: u16,
    // ImU16       OffsetShortcut;
    pub OffsetShortcut: u16,
    // pImU16       OffsetMark;
    pub OffsetMark: *mut u16,
    // ImU16       Widths[4];          // Width of:   Icon, Label, Shortcut, Mark  (accumulators for current frame)
    pub Widths: [u16;4],
}

impl ImGuiMenuColumns {
    // ImGuiMenuColumns() { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    // void        Update(float spacing, bool window_reappearing);
    pub fn Update(&mut self, spacing: f32, window_reappearing: bool) {
        todo!()
    }
    // float       DeclColumns(float w_icon, float w_label, float w_shortcut, float w_mark);
    pub fn DeclColumns(&mut self, w_icon: f32, w_label: f32, w_shortcut: f32, w_mark: f32) -> f32 {
        todo!()
    }
    // void        CalcNextTotalWidth(bool update_offsets);
    pub fn CalcNextTotalWidth(&mut self, update_offsets: bool) {
        todo!()
    }
}

// FIXME: this is in development, not exposed/functional as a generic feature yet.
// Horizontal/Vertical enums are fixed to 0/1 so they may be used to index ImVec2
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum ImGuiLayoutType
{
    Horizontal,
    Vertical
}

// X/Y enums are fixed to 0/1 so they may be used to index ImVec2
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum ImGuiAxis
{
    None = -1,
    X = 0,
    Y = 1
}

// Store the source authority (dock node vs window) of a field
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum ImGuiDataAuthority
{
    Auto,
    DockNode,
    Window
}

#[derive(Debug,Default,Clone)]
pub struct  ImGuiStackSizes
{
    // short   SizeOfIDStack;
    pub SizeofIDStack: i16,
    // short   SizeOfColorStack;
    pub SizeOfColorStack: i16,
    // short   SizeOfStyleVarStack;
    pub SizeOfStyleVarStack: i16,
    // short   SizeOfFontStack;
    pub SizeOfFontStack: i16,
    // short   SizeOfFocusScopeStack;
    pub SizeOfFocusScopeStack: i16,
    // short   SizeOfGroupStack;
    pub SizeOfGroupStack: i16,
    // short   SizeOfItemFlagsStack;
    pub SizeOfItemFlagsStack: i16,
    // short   SizeOfBeginPopupStack;
    pub SizeOfBeginPopupStack: i16,
    // short   SizeOfDisabledStack;
    pub SizeOfDisabledStack: i16,
}

impl ImGuiStackSizes {
    // ImGuiStackSizes() { memset(this, 0, sizeof(*this)); }
    pub fn new()-> Self {
        Self {
            ..Default::default()
        }
    }
    //     void SetToCurrentState();
    pub fn SetToCurrentState(&mut self) {
        todo!()
    }
    //     void CompareWithCurrentState();
    pub fn CompareWithCurrentState(&self) {

    }
}


#[derive(Debug,Default,Clone)]
pub struct DimgPtrOrIndex
{
    // void*       Ptr;            // Either field can be set, not both. e.g. Dock node tab bars are loose while BeginTabBar() ones are in a pool.
    Ptr: *mut c_void,
    // int         Index;          // Usually index in a main pool.
    Index: i32,

}

impl DimgPtrOrIndex {
    // ImGuiPtrOrIndex(void* ptr)  { Ptr = ptr; Index = -1; }
    pub fn new(ptr:*mut c_void) -> Self {
        Self {
            Ptr: ptr,
            Index: -1,
        }
    }
    //     ImGuiPtrOrIndex(int index)  { Ptr = NULL; Index = index; }
    pub fn new2(index: i32) -> Self {
        Self {
            Ptr: null_mut(),
            Index: index
        }
    }
}

#[derive(Debug,Clone,Default)]
pub struct DimgShrinkWidthItem
{
    // int         Index;
    pub Index: i32,
    // float       Width;
    pub Width: f32,
    // float       InitialWidth;
    pub InitialWidth: f32,
}


// Extend ImGuiDataType_
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum ImGuiDataType
{
    String,
    Pointer,
    ID
}
