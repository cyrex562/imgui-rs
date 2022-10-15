// dear imgui, v1.89 WIP
// (headers)

// Help:
// - Read FAQ at http://dearimgui.org/faq
// - Newcomers, read 'Programmer guide' in imgui.cpp for notes on how to setup Dear ImGui in your codebase.
// - Call and read ShowDemoWindow() in imgui_demo.cpp. All applications in examples/ are doing that.
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
// [SECTION] Flags & Enumerations
// [SECTION] Helpers: Memory allocations macros, ImVector<>
// [SECTION] ImGuiStyle
// [SECTION] ImGuiIO
// [SECTION] Misc data structures (ImGuiInputTextCallbackData, ImGuiSizeCallbackData, ImGuiWindowClass, ImGuiPayload, ImGuiTableSortSpecs, ImGuiTableColumnSortSpecs)
// [SECTION] Helpers (ImGuiOnceUponAFrame, ImGuiTextFilter, ImGuiTextBuffer, ImGuiStorage, ImGuiListClipper, ImColor)
// [SECTION] Drawing API (ImDrawCallback, ImDrawCmd, ImDrawIdx, ImDrawVert, ImDrawChannel, ImDrawListSplitter, ImDrawFlags, ImDrawListFlags, ImDrawList, ImDrawData)
// [SECTION] Font API (ImFontConfig, ImFontGlyph, ImFontGlyphRangesBuilder, ImFontAtlasFlags, ImFontAtlas, ImFont)
// [SECTION] Viewports (ImGuiViewportFlags, ImGuiViewport)
// [SECTION] Platform Dependent Interfaces (ImGuiPlatformIO, ImGuiPlatformMonitor, ImGuiPlatformImeData)
// [SECTION] Obsolete functions and types

*/

// #pragma once

// Configuration file with compile-time options (edit imconfig.h or '#define IMGUI_USER_CONFIG "myfilename.h" from your build system')
// #ifdef IMGUI_USER_CONFIG
// #include IMGUI_USER_CONFIG
// #endif
// #if !defined(IMGUI_DISABLE_INCLUDE_IMCONFIG_H) || defined(IMGUI_INCLUDE_IMCONFIG_H)
// #include "imconfig.h"
// #endif

// #ifndef IMGUI_DISABLE

//-----------------------------------------------------------------------------
// [SECTION] Header mess
//-----------------------------------------------------------------------------

// Includes
// #include <float.h>                  // FLT_MIN, f32::MAX
// #include <stdarg.h>                 // va_list, va_start, va_end
// #include <stddef.h>                 // ptrdiff_t, NULL
// #include <string.h>                 // memset, memmove, memcpy, strlen, strchr, strcpy, strcmp

// Version
// (Integer encoded as XYYZZ for use in #if preprocessor conditionals. Work in progress versions typically starts at XYY99 then bounce up to XYY00, XYY01 etc. when release tagging happens)
// #define IMGUI_VERSION               "1.89 WIP"
// #define IMGUI_VERSION_NUM           18818
// #define IMGUI_CHECKVERSION()        DebugCheckVersionAndDataLayout(IMGUI_VERSION, sizeof(ImGuiIO), sizeof(ImGuiStyle), sizeof(ImVec2), sizeof(ImVec4), sizeof(ImDrawVert), sizeof(ImDrawIdx))
// #define IMGUI_HAS_TABLE
// #define IMGUI_HAS_VIEWPORT          // Viewport WIP branch
// #define IMGUI_HAS_DOCK              // Docking WIP branch

// Define attributes of all API symbols declarations (e.g. for DLL under Windows)
// IMGUI_API is used for core imgui functions, IMGUI_IMPL_API is used for the default backends files (imgui_impl_xxx.h)
// Using dear imgui via a shared library is not recommended, because we don't guarantee backward nor forward ABI compatibility (also function call overhead, as dear imgui is a call-heavy API)
// #ifndef IMGUI_API
// #define IMGUI_API
// #endif
// #ifndef IMGUI_IMPL_API
// #define IMGUI_IMPL_API              IMGUI_API
// #endif

// Helper Macros
// #ifndef IM_ASSERT
// #include <assert.h>
// #define IM_ASSERT(_EXPR)            assert(_EXPR)                               // You can override the default assert handler by editing imconfig.h
// #endif
// #define IM_ARRAYSIZE(_ARR)          ((sizeof(_ARR) / sizeof(*(_ARR))))     // Size of a static C-style array. Don't use on pointers!
// #define IM_UNUSED(_VAR)             ((void)(_VAR))                              // Used to silence "unused variable warnings". Often useful as asserts may be stripped out from final builds.
// #define IM_OFFSETOF(_TYPE,_MEMBER)  offsetof(_TYPE, _MEMBER)                    // Offset of _MEMBER within _TYPE. Standardized as offsetof() in C++11

// Helper Macros - IM_FMTARGS, IM_FMTLIST: Apply printf-style warnings to our formatting functions.
// #if !defined(IMGUI_USE_STB_SPRINT0f32) && defined(__MINGW32__) && !defined(__clang__)
// #define IM_FMTARGS(FMT)             __attribute__((format(gnu_printf, FMT, FMT+1)))
// #define IM_FMTLIST(FMT)             __attribute__((format(gnu_printf, FMT, 0)))
// #elif !defined(IMGUI_USE_STB_SPRINT0f32) && (defined(__clang__) || defined(__GNUC__))
// #define IM_FMTARGS(FMT)             __attribute__((format(printf, FMT, FMT+1)))
// #define IM_FMTLIST(FMT)             __attribute__((format(printf, FMT, 0)))
// #else
// #define IM_FMTARGS(FMT)
// #define IM_FMTLIST(FMT)
// #endif

// Disable some of MSVC most aggressive Debug runtime checks in function header/footer (used in some simple/low-level functions)
// #if defined(_MSC_VER) && !defined(__clang__)  && !defined(__INTEL_COMPILER) && !defined(IMGUI_DEBUG_PARANOID)
// #define IM_MSVC_RUNTIME_CHECKS_OFF      __pragma(runtime_checks("",of0f32))     __pragma(check_stack(of0f32)) __pragma(strict_gs_check(push,of0f32))
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
// struct ImDrawListSharedData;        // Data shared among multiple draw lists (typically owned by parent ImGui context, but you may create one yoursel0f32)
// struct ImDrawListSplitter;          // Helper to split a draw list into different layers which can be drawn into out of order, then flattened back.
// struct ImDrawVert;                  // A single vertex (pos + uv + col = 20 bytes by default. Override layout with IMGUI_OVERRIDE_DRAWVERT_STRUCT_LAYOUT)
// struct ImFont;                      // Runtime data for a single font within a parent ImFontAtlas
// struct ImFontAtlas;                 // Runtime data for multiple fonts, bake multiple fonts into a single texture, TTF/OTF font loader
// struct ImFontBuilderIO;             // Opaque interface to a font builder (stb_truetype or FreeType).
// struct ImFontConfig;                // Configuration data when adding a font or merging fonts
// struct ImFontGlyph;                 // A single font glyph (code point + coordinates within in ImFontAtlas + offset)
// struct ImFontGlyphRangesBuilder;    // Helper to build glyph ranges from text/string data
// struct ImColor;                     // Helper functions to create a color that can be converted to either or: u32 float4 (*OBSOLETE* please avoid using)
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

// Enums/Flags (declared as int for compatibility with old C++, to allow using as flags without overhead, and to not pollute the top of this file)
// - Tip: Use your programming IDE navigation facilities on the names in the _central column_ below to find the actual flags/enum lists!
//   In Visual Studio IDE: CTRL+comma ("Edit.GoToAll") can follow symbols in comments, whereas CTRL+F12 ("Edit.GoToImplementation") cannot.

// ImVec4: 4D vector used to store clipping rectangles, colors etc. [Compile-time configurable type]

IM_MSVC_RUNTIME_CHECKS_RESTORE

//-----------------------------------------------------------------------------
// [SECTION] Dear ImGui end-user API functionsf
// (Note that  being a namespace, you can add extra  functions in your own separate file. Please don't modify imgui source files!)
//-----------------------------------------------------------------------------

namespace ImGui
{
    // Context creation and access
    // - Each context create its own ImFontAtlas by default. You may instance one yourself and pass it to CreateContext() to share a font atlas between contexts.
    // - DLL users: heaps and globals are not shared across DLL boundaries! You will need to call SetCurrentContext() + SetAllocatorFunctions()
    //   for each static/DLL boundary you are calling from. Read "Context and Memory Allocators" section of imgui.cpp for details.
     *mut ImGuiContext CreateContext(*mut ImFontAtlas shared_font_atlas = null_mut());
     c_void          DestroyContext(*mut ImGuiContext ctx = null_mut());   // NULL = destroy current context
     *mut ImGuiContext GetCurrentContext();
     c_void          SetCurrentContext(*mut ImGuiContext ctx);

    // Main
     ImGuiIO&      GetIO();                                    // access the IO structure (mouse/keyboard/gamepad inputs, time, various configuration options/flags)
     ImGuiStyle&   GetStyle();                                 // access the Style structure (colors, sizes). Always use PushStyleCol(), PushStyleVar() to modify style mid-frame!
     c_void          NewFrame();                                 // start a new Dear ImGui frame, you can submit any command from this point until Render()/EndFrame().
     c_void          EndFrame();                                 // ends the Dear ImGui frame. automatically called by Render(). If you don't need to render data (skipping rendering) you may call EndFrame() without Render()... but you'll have wasted CPU already! If you don't need to render, better to not create any windows and not call NewFrame() at all!
     c_void          Render();                                   // ends the Dear ImGui frame, finalize the draw data. You can then get call GetDrawData().
     ImDrawData*   GetDrawData();                              // valid after Render() and until the next call to NewFrame(). this is what you have to render.

    // Demo, Debug, Information
     c_void          ShowDemoWindow(bool* p_open = null_mut());        // create Demo window. demonstrate most ImGui features. call this to learn about the library! try to make it always available in your application!
     c_void          ShowMetricsWindow(bool* p_open = null_mut());     // create Metrics/Debugger window. display Dear ImGui internals: windows, draw commands, various internal state, etc.
     c_void          ShowDebugLogWindow(bool* p_open = null_mut());    // create Debug Log window. display a simplified log of important dear imgui events.
     c_void          ShowStackToolWindow(bool* p_open = null_mut());   // create Stack Tool window. hover items with mouse to query information about the source of their unique ID.
     c_void          ShowAboutWindow(bool* p_open = null_mut());       // create About window. display Dear ImGui version, credits and build/system information.
     c_void          ShowStyleEditor(ImGuiStyle* ref = null_mut());    // add style editor block (not a window). you can pass in a reference ImGuiStyle structure to compare to, revert to and save to (else it uses the default style)
     bool          ShowStyleSelector(label: *const c_char);       // add style selector block (not a window), essentially a combo listing the default styles.
     c_void          ShowFontSelector(label: *const c_char);        // add font selector block (not a window), essentially a combo listing the loaded fonts.
     c_void          ShowUserGuide();                            // add basic help/info block (not a window): how to manipulate ImGui as a end-user (mouse/keyboard controls).
     *const char   GetVersion();                               // get the compiled version string e.g. "1.80 WIP" (essentially the value for IMGUI_VERSION from the compiled version of imgui.cpp)

    // Styles
     c_void          StyleColorsDark(ImGuiStyle* dst = null_mut());    // new, recommended style (default)
     c_void          StyleColorsLight(ImGuiStyle* dst = null_mut());   // best used with borders and a custom, thicker font
     c_void          StyleColorsClassic(ImGuiStyle* dst = null_mut()); // classic imgui style

    // Windows
    // - Begin() = push window to the stack and start appending to it. End() = pop window from the stack.
    // - Passing 'bool* p_open != NULL' shows a window-closing widget in the upper-right corner of the window,
    //   which clicking will set the boolean to false when clicked.
    // - You may append multiple times to the same window during the same frame by calling Begin()/End() pairs multiple times.
    //   Some information such as 'flags' or 'p_open' will only be considered by the first call to Begin().
    // - Begin() return false to indicate the window is collapsed or fully clipped, so you may early out and omit submitting
    //   anything to the window. Always call a matching End() for each Begin() call, regardless of its return value!
    //   [Important: due to legacy reason, this is inconsistent with most other functions such as BeginMenu/EndMenu,
    //    BeginPopup/EndPopup, etc. where the EndXXX call should only be called if the corresponding BeginXXX function
    //    returned true. Begin and BeginChild are the only odd ones out. Will be fixed in a future update.]
    // - Note that the bottom of window stack always contains a window called "Debug".
     bool          Begin(name: *const c_char,p_open: *mut bool = null_mut(), flags: ImGuiWindowFlags = 0);
     c_void          End();

    // Child Windows
    // - Use child windows to begin into a self-contained independent scrolling/clipping regions within a host window. Child windows can embed their own child.
    // - For each independent axis of 'size': ==0.0: use remaining host window size / >0.0: fixed size / <0.0: use remaining window size minus abs(size) / Each axis can use a different mode, e.g. ImVec2::new(0,400).
    // - BeginChild() returns false to indicate the window is collapsed or fully clipped, so you may early out and omit submitting anything to the window.
    //   Always call a matching EndChild() for each BeginChild() call, regardless of its return value.
    //   [Important: due to legacy reason, this is inconsistent with most other functions such as BeginMenu/EndMenu,
    //    BeginPopup/EndPopup, etc. where the EndXXX call should only be called if the corresponding BeginXXX function
    //    returned true. Begin and BeginChild are the only odd ones out. Will be fixed in a future update.]
     bool          BeginChild(str_id: *const c_char, size: &ImVec2 = ImVec2::new(0, 0), let mut border: bool =  false, flags: ImGuiWindowFlags = 0);
     bool          BeginChild(id: ImGuiID, size: &ImVec2 = ImVec2::new(0, 0), let mut border: bool =  false, flags: ImGuiWindowFlags = 0);
     c_void          EndChild();

    // Windows Utilities
    // - 'current window' = the window we are appending into while inside a Begin()/End() block. 'next window' = next window we will Begin() into.
     bool          IsWindowAppearing();
     bool          IsWindowCollapsed();
     bool          IsWindowFocused(flags: ImGuiFocusedFlags=0); // is current window focused? or its root/child, depending on flags. see flags for options.
     bool          IsWindowHovered(flags: ImGuiHoveredFlags=0); // is current window hovered (and typically: not blocked by a popup/modal)? see flags for options. NB: If you are trying to check whether your mouse should be dispatched to imgui or to your app, you should use the 'io.WantCaptureMouse' boolean for that! Please read the FAQ!
     ImDrawList*   GetWindowDrawList();                        // get draw list associated to the current window, to append your own drawing primitivesGetWindowDpiScale: c_float();                        // get DPI scale currently associated to the current window's viewport.
     ImVec2        GetWindowPos();                             // get current window position in screen space (useful if you want to do your own drawing via the DrawList API)
     ImVec2        GetWindowSize();                            // get current window sizeGetWindowWidth: c_float();                           // get current window width (shortcut for GetWindowSize().x)GetWindowHeight: c_float();                          // get current window height (shortcut for GetWindowSize().y)
     ImGuiViewport*GetWindowViewport();                        // get viewport currently associated to the current window.

    // Window manipulation
    // - Prefer using SetNextXXX functions (before Begin) rather that SetXXX functions (after Begin).
     c_void          SetNextWindowPos(pos: &ImVec2, cond: ImGuiCond = 0, pivot: &ImVec2 = ImVec2::new(0, 0)); // set next window position. call before Begin(). use pivot=(0.5f32,0.5f32) to center on given point, etc.
     c_void          SetNextWindowSize(size: &ImVec2, cond: ImGuiCond = 0);                  // set next window size. set axis to 0.0 to force an auto-fit on this axis. call before Begin()
     c_void          SetNextWindowSizeConstraints(size_min: &ImVec2, size_max: &ImVec2, ImGuiSizeCallback custom_callback = null_mut(), custom_callback_data: *mut c_void = null_mut()); // set next window size limits. use -1,-1 on either X/Y axis to preserve the current size. Sizes will be rounded down. Use callback to apply non-trivial programmatic constraints.
     c_void          SetNextWindowContentSize(size: &ImVec2);                               // set next window content size (~ scrollable client area, which enforce the range of scrollbars). Not including window decorations (title bar, menu bar, etc.) nor WindowPadding. set an axis to 0.0 to leave it automatic. call before Begin()
     c_void          SetNextWindowCollapsed(collapsed: bool, cond: ImGuiCond = 0);                 // set next window collapsed state. call before Begin()
     c_void          SetNextWindowFocus();                                                       // set next window to be focused / top-most. call before Begin()
     c_void          SetNextWindowBgAlpha(alpha: c_float);                                          // set next window background color alpha. helper to easily override the Alpha component of ImGuiCol_WindowBg/ChildBg/PopupBg. you may also use ImGuiWindowFlags_NoBackground.
     c_void          SetNextWindowViewport(viewport_id: ImGuiID);                                 // set next window viewport
     c_void          SetWindowPos(pos: &ImVec2, cond: ImGuiCond = 0);                        // (not recommended) set current window position - call within Begin()/End(). prefer using SetNextWindowPos(), as this may incur tearing and side-effects.
     c_void          SetWindowSize(size: &ImVec2, cond: ImGuiCond = 0);                      // (not recommended) set current window size - call within Begin()/End(). set to ImVec2::new(0, 0) to force an auto-fit. prefer using SetNextWindowSize(), as this may incur tearing and minor side-effects.
     c_void          SetWindowCollapsed(collapsed: bool, cond: ImGuiCond = 0);                     // (not recommended) set current window collapsed state. prefer using SetNextWindowCollapsed().
     c_void          SetWindowFocus();                                                           // (not recommended) set current window to be focused / top-most. prefer using SetNextWindowFocus().
     c_void          SetWindowFontScale(scale: c_float);                                            // [OBSOLETE] set font scale. Adjust IO.FontGlobalScale if you want to scale all windows. This is an old API! For correct scaling, prefer to reload font + rebuild ImFontAtlas + call style.ScaleAllSizes().
     c_void          SetWindowPos(name: *const c_char, pos: &ImVec2, cond: ImGuiCond = 0);      // set named window position.
     c_void          SetWindowSize(name: *const c_char, size: &ImVec2, cond: ImGuiCond = 0);    // set named window size. set axis to 0.0 to force an auto-fit on this axis.
     c_void          SetWindowCollapsed(name: *const c_char, collapsed: bool, cond: ImGuiCond = 0);   // set named window collapsed state
     c_void          SetWindowFocus(name: *const c_char);                                           // set named window to be focused / top-most. use NULL to remove focus.

    // Content region
    // - Retrieve available space from a given point. GetContentRegionAvail() is frequently useful.
    // - Those functions are bound to be redesigned (they are confusing, incomplete and the Min/Max return values are in local window coordinates which increases confusion)
     ImVec2        GetContentRegionAvail();                                        // == GetContentRegionMax() - GetCursorPos()
     ImVec2        GetContentRegionMax();                                          // current content boundaries (typically window boundaries including scrolling, or current column boundaries), in windows coordinates
     ImVec2        GetWindowContentRegionMin();                                    // content boundaries min for the full window (roughly (0,0)-Scroll), in window coordinates
     ImVec2        GetWindowContentRegionMax();                                    // content boundaries max for the full window (roughly (0,0)+Size-Scroll) where Size can be override with SetNextWindowContentSize(), in window coordinates

    // Windows ScrollingGetScrollX: c_float();                                                   // get scrolling amount [0 .. GetScrollMaxX()]GetScrollY: c_float();                                                   // get scrolling amount [0 .. GetScrollMaxY()]
     c_void          SetScrollX(scroll_x: c_float);                                     // set scrolling amount [0 .. GetScrollMaxX()]
     c_void          SetScrollY(scroll_y: c_float);                                     // set scrolling amount [0 .. GetScrollMaxY()]GetScrollMaxX: c_float();                                                // get maximum scrolling amount ~~ ContentSize.x - WindowSize.x - DecorationsSize.xGetScrollMaxY: c_float();                                                // get maximum scrolling amount ~~ ContentSize.y - WindowSize.y - DecorationsSize.y
     c_void          SetScrollHereX(let center_x_ratio: c_float =  0.5);                    // adjust scrolling amount to make current cursor position visible. center_x_ratio=0.0: left, 0.5: center, 1.0: right. When using to make a "default/current item" visible, consider using SetItemDefaultFocus() instead.
     c_void          SetScrollHereY(let center_y_ratio: c_float =  0.5);                    // adjust scrolling amount to make current cursor position visible. center_y_ratio=0.0: top, 0.5: center, 1.0: bottom. When using to make a "default/current item" visible, consider using SetItemDefaultFocus() instead.
     c_void          SetScrollFromPosX(local_x: c_float, let center_x_ratio: c_float =  0.5);  // adjust scrolling amount to make given position visible. Generally GetCursorStartPos() + offset to compute a valid position.
     c_void          SetScrollFromPosY(local_y: c_float, let center_y_ratio: c_float =  0.5);  // adjust scrolling amount to make given position visible. Generally GetCursorStartPos() + offset to compute a valid position.

    // Parameters stacks (shared)
     c_void          PushFont(font: *mut ImFont);                                         // use NULL as a shortcut to push default font
     c_void          PopFont();
     c_void          PushStyleColor(ImGuiCol idx, col: u32);                        // modify a style color. always use this if you modify the style after NewFrame().
     c_void          PushStyleColor(ImGuiCol idx, const ImVec4& col);
     c_void          PopStyleColor(let count: c_int = 1);
     c_void          PushStyleVar(ImGuiStyleVar idx,val: c_float);                     // modify a style float variable. always use this if you modify the style after NewFrame().
     c_void          PushStyleVar(ImGuiStyleVar idx, val: &ImVec2);             // modify a style variable: ImVec2. always use this if you modify the style after NewFrame().
     c_void          PopStyleVar(let count: c_int = 1);
     c_void          PushAllowKeyboardFocus(allow_keyboard_focus: bool);              // == tab stop enable. Allow focusing using TAB/Shift-TAB, enabled by default but you can disable it for certain widgets
     c_void          PopAllowKeyboardFocus();
     c_void          PushButtonRepeat(repeat: bool);                                  // in 'repeat' mode, Button*() functions return repeated true in a typematic manner (using io.KeyRepeatDelay/io.KeyRepeatRate setting). Note that you can call IsItemActive() after any Button() to tell if the button is held in the current frame.
     c_void          PopButtonRepeat();

    // Parameters stacks (current window)
     c_void          PushItemWidth(item_width: c_float);                                // push width of items for common large "item+label" widgets. >0.0: width in pixels, <0.0 align xx pixels to the right of window (so -FLT_MIN always align width to the right side).
     c_void          PopItemWidth();
     c_void          SetNextItemWidth(item_width: c_float);                             // set width of the _next_ common large "item+label" widget. >0.0: width in pixels, <0.0 align xx pixels to the right of window (so -FLT_MIN always align width to the right side)CalcItemWidth: c_float();                                                // width of item given pushed settings and current cursor position. NOT necessarily the width of last item unlike most 'Item' functions.
     c_void          PushTextWrapPos(let wrap_local_pos_x: c_float =  0.0);                 // push word-wrapping position for Text*() commands. < 0.0: no wrapping; 0.0: wrap to end of window (or column); > 0.0: wrap at 'wrap_pos_x' position in window local space
     c_void          PopTextWrapPos();

    // Style read access
    // - Use the style editor (ShowStyleEditor() function) to interactively see what the colors are)
     ImFont*       GetFont();                                                      // get current fontGetFontSize: c_float();                                                  // get current font size (= height in pixels) of current font with current scale applied
     ImVec2        GetFontTexUvWhitePixel();                                       // get UV coordinate for a while pixel, useful to draw custom shapes via the ImDrawList API
     u32         GetColorU32(ImGuiCol idx, let alpha_mul: c_float =  1.0);              // retrieve given style color with style alpha applied and optional extra alpha multiplier, packed as a 32-bit value suitable for ImDrawList
     u32         GetColorU32(const ImVec4& col);                                 // retrieve given color with style alpha applied, packed as a 32-bit value suitable for ImDrawList
     u32         GetColorU32(col: u32);                                         // retrieve given color with style alpha applied, packed as a 32-bit value suitable for ImDrawList
     const ImVec4& GetStyleColorVec4(ImGuiCol idx);                                // retrieve style color as stored in ImGuiStyle structure. use to feed back into PushStyleColor(), otherwise use GetColorU32() to get style color with style alpha baked in.

    // Cursor / Layout
    // - By "cursor" we mean the current output position.
    // - The typical widget behavior is to output themselves at the current cursor position, then move the cursor one line down.
    // - You can call SameLine() between widgets to undo the last carriage return and output at the right of the preceding widget.
    // - Attention! We currently have inconsistencies between window-local and absolute positions we will aim to fix with future API:
    //    Window-local coordinates:   SameLine(), GetCursorPos(), SetCursorPos(), GetCursorStartPos(), GetContentRegionMax(), GetWindowContentRegion*(), PushTextWrapPos()
    //    Absolute coordinate:        GetCursorScreenPos(), SetCursorScreenPos(), all ImDrawList:: functions.
     c_void          Separator();                                                    // separator, generally horizontal. inside a menu bar or in horizontal layout mode, this becomes a vertical separator.
     c_void          SameLine(offset_from_start_x: c_float=0.0,spacing: c_float=-1.0);  // call between widgets or groups to layout them horizontally. X position given in window coordinates.
     c_void          NewLine();                                                      // undo a SameLine() or force a new line when in an horizontal-layout context.
     c_void          Spacing();                                                      // add vertical spacing.
     c_void          Dummy(size: &ImVec2);                                      // add a dummy item of given size. unlike InvisibleButton(), Dummy() won't take the mouse click or be navigable into.
     c_void          Indent(let indent_w: c_float =  0.0);                                  // move content position toward the right, by indent_w, or style.IndentSpacing if indent_w <= 0
     c_void          Unindent(let indent_w: c_float =  0.0);                                // move content position back to the left, by indent_w, or style.IndentSpacing if indent_w <= 0
     c_void          BeginGroup();                                                   // lock horizontal starting position
     c_void          EndGroup();                                                     // unlock horizontal starting position + capture the whole group bounding box into one "item" (so you can use IsItemHovered() or layout primitives such as SameLine() on whole group, etc.)
     ImVec2        GetCursorPos();                                                 // cursor position in window coordinates (relative to window position)GetCursorPosX: c_float();                                                //   (some functions are using window-relative coordinates, such as: GetCursorPos, GetCursorStartPos, GetContentRegionMax, GetWindowContentRegion* etc.GetCursorPosY: c_float();                                                //    other functions such as GetCursorScreenPos or everything in ImDrawList::
     c_void          SetCursorPos(local_pos: &ImVec2);                          //    are using the main, absolute coordinate system.
     c_void          SetCursorPosX(local_x: c_float);                                   //    GetWindowPos() + GetCursorPos() == GetCursorScreenPos() etc.)
     c_void          SetCursorPosY(local_y: c_float);                                   //
     ImVec2        GetCursorStartPos();                                            // initial cursor position in window coordinates
     ImVec2        GetCursorScreenPos();                                           // cursor position in absolute coordinates (useful to work with ImDrawList API). generally top-left == GetMainViewport()->Pos == (0,0) in single viewport mode, and bottom-right == GetMainViewport()->Pos+Size == io.DisplaySize in single-viewport mode.
     c_void          SetCursorScreenPos(pos: &ImVec2);                          // cursor position in absolute coordinates
     c_void          AlignTextToFramePadding();                                      // vertically align upcoming text baseline to FramePadding.y so that it will align properly to regularly framed items (call if you have text on a line before a framed item)GetTextLineHeight: c_float();                                            // ~ FontSizeGetTextLineHeightWithSpacing: c_float();                                 // ~ FontSize + style.ItemSpacing.y (distance in pixels between 2 consecutive lines of text)GetFrameHeight: c_float();                                               // ~ FontSize + style.FramePadding.y * 2GetFrameHeightWithSpacing: c_float();                                    // ~ FontSize + style.FramePadding.y * 2 + style.ItemSpacing.y (distance in pixels between 2 consecutive lines of framed widgets)

    // ID stack/scopes
    // Read the FAQ (docs/FAQ.md or http://dearimgui.org/faq) for more details about how ID are handled in dear imgui.
    // - Those questions are answered and impacted by understanding of the ID stack system:
    //   - "Q: Why is my widget not reacting when I click on it?"
    //   - "Q: How can I have widgets with an empty label?"
    //   - "Q: How can I have multiple widgets with the same label?"
    // - Short version: ID are hashes of the entire ID stack. If you are creating widgets in a loop you most likely
    //   want to push a unique identifier (e.g. object pointer, loop index) to uniquely differentiate them.
    // - You can also use the "Label##foobar" syntax within widget label to distinguish them from each others.
    // - In this header file we use the "label"/"name" terminology to denote a string that will be displayed + used as an ID,
    //   whereas "str_id" denote a string that is only used as an ID and not normally displayed.
     c_void          PushID(str_id: *const c_char);                                     // push string into the ID stack (will hash string).
     c_void          PushID(str_id_begin: *const c_char, str_id_end: *const c_char);       // push string into the ID stack (will hash string).
     c_void          PushID(ptr_id: *const c_void);                                     // push pointer into the ID stack (will hash pointer).
     c_void          PushID(int_id: c_int);                                             // push integer into the ID stack (will hash integer).
     c_void          PopID();                                                        // pop from the ID stack.
     ImGuiID       GetID(str_id: *const c_char);                                      // calculate unique ID (hash of whole ID stack + given parameter). e.g. if you want to query into ImGuiStorage yourself
     ImGuiID       GetID(str_id_begin: *const c_char, str_id_end: *const c_char);
     ImGuiID       GetID(ptr_id: *const c_void);

    // Widgets: Text
     c_void          TextUnformatted(text: *const c_char, text_end: *const c_char = null_mut()); // raw text without formatting. Roughly equivalent to Text("%s", text) but: A) doesn't require null terminated string if 'text_end' is specified, B) it's faster, no memory copy is done, no buffer size limits, recommended for long chunks of text.
     c_void          Text(fmt: *const c_char, ...)                                      IM_FMTARGS(1); // formatted text
     c_void          TextV(fmt: *const c_char, va_list args)                            IM_FMTLIST(1);
     c_void          TextColored(const ImVec4& col, fmt: *const c_char, ...)            IM_FMTARGS(2); // shortcut for PushStyleColor(ImGuiCol_Text, col); Text(fmt, ...); PopStyleColor();
     c_void          TextColoredV(const ImVec4& col, fmt: *const c_char, va_list args)  IM_FMTLIST(2);
     c_void          TextDisabled(fmt: *const c_char, ...)                              IM_FMTARGS(1); // shortcut for PushStyleColor(ImGuiCol_Text, style.Colors[ImGuiCol_TextDisabled]); Text(fmt, ...); PopStyleColor();
     c_void          TextDisabledV(fmt: *const c_char, va_list args)                    IM_FMTLIST(1);
     c_void          TextWrapped(fmt: *const c_char, ...)                               IM_FMTARGS(1); // shortcut for PushTextWrapPos(0.0); Text(fmt, ...); PopTextWrapPos();. Note that this won't work on an auto-resizing window if there's no other widgets to extend the window width, yoy may need to set a size using SetNextWindowSize().
     c_void          TextWrappedV(fmt: *const c_char, va_list args)                     IM_FMTLIST(1);
     c_void          LabelText(label: *const c_char, fmt: *const c_char, ...)              IM_FMTARGS(2); // display text+label aligned the same way as value+label widgets
     c_void          LabelTextV(label: *const c_char, fmt: *const c_char, va_list args)    IM_FMTLIST(2);
     c_void          BulletText(fmt: *const c_char, ...)                                IM_FMTARGS(1); // shortcut for Bullet()+Text()
     c_void          BulletTextV(fmt: *const c_char, va_list args)                      IM_FMTLIST(1);

    // Widgets: Main
    // - Most widgets return true when the value has been changed or when pressed/selected
    // - You may also use one of the many IsItemXXX functions (e.g. IsItemActive, IsItemHovered, etc.) to query widget state.
     bool          Button(label: *const c_char, size: &ImVec2 = ImVec2::new(0, 0));   // button
     bool          SmallButton(label: *const c_char);                                 // button with FramePadding=(0,0) to easily embed within text
     bool          InvisibleButton(str_id: *const c_char, size: &ImVec2, ImGuiButtonFlags flags = 0); // flexible button behavior without the visuals, frequently useful to build custom behaviors using the public api (along with IsItemActive, IsItemHovered, etc.)
     bool          ArrowButton(str_id: *const c_char, dir: ImGuiDir);                  // square button with an arrow shape
     bool          Checkbox(label: *const c_char,v: *mut bool);
     bool          CheckboxFlags(label: *const c_char, flags:  *mut c_int, flags_value: c_int);
     bool          CheckboxFlags(label: *const c_char, flags: *mut c_uint, flags_value: c_uint);
     bool          RadioButton(label: *const c_char, active: bool);                    // use with e.g. if (RadioButton("one", my_value==1)) { my_value = 1; }
     bool          RadioButton(label: *const c_char, v:  *mut c_int, v_button: c_int);           // shortcut to handle the above pattern when value is an integer
     c_void          ProgressBar(fraction: c_float, size_arg: &ImVec2 = ImVec2::new(-FLT_MIN, 0), overlay: *const c_char = null_mut());
     c_void          Bullet();                                                       // draw a small circle + keep the cursor on the same line. advance cursor x position by GetTreeNodeToLabelSpacing(), same distance that TreeNode() uses

    // Widgets: Images
    // - Read about ImTextureID here: https://github.com/ocornut/imgui/wiki/Image-Loading-and-Displaying-Examples
     c_void          Image(ImTextureID user_texture_id, size: &ImVec2, uv0: &ImVec2 = ImVec2::new(0, 0), uv1: &ImVec2 = ImVec2::new(1, 1), const ImVec4& tint_col = ImVec4(1, 1, 1, 1), const ImVec4& border_col = ImVec4(0, 0, 0, 0));
     bool          ImageButton(str_id: *const c_char, ImTextureID user_texture_id, size: &ImVec2, uv0: &ImVec2 = ImVec2::new(0, 0), uv1: &ImVec2 = ImVec2::new(1, 1), const ImVec4& bg_col = ImVec4(0, 0, 0, 0), const ImVec4& tint_col = ImVec4(1, 1, 1, 1));

    // Widgets: Combo Box
    // - The BeginCombo()/EndCombo() api allows you to manage your contents and selection state however you want it, by creating e.g. Selectable() items.
    // - The old Combo() api are helpers over BeginCombo()/EndCombo() which are kept available for convenience purpose. This is analogous to how ListBox are created.
     bool          BeginCombo(label: *const c_char, preview_value: *const c_char, ImGuiComboFlags flags = 0);
     c_void          EndCombo(); // only call EndCombo() if BeginCombo() returns true!
     bool          Combo(label: *const c_char, current_item:  *mut c_int, const: *const c_char items[], items_count: c_int, let popup_max_height_in_items: c_int = -1);
     bool          Combo(label: *const c_char, current_item:  *mut c_int, items_separated_by_zeros: *const c_char, let popup_max_height_in_items: c_int = -1);      // Separate items with \0 within a string, end item-list with \0\0. e.g. "One\0Two\0Three\0"
     bool          Combo(label: *const c_char, current_item:  *mut c_int, bool(*items_getter)(data: *mut c_void, idx: c_int, *const char* out_text), data: *mut c_void, items_count: c_int, let popup_max_height_in_items: c_int = -1);

    // Widgets: Drag Sliders
    // - CTRL+Click on any drag box to turn them into an input box. Manually input values aren't clamped by default and can go off-bounds. Use ImGuiSliderFlags_AlwaysClamp to always clamp.
    // - For all the Float2/Float3/Float4/Int2/Int3/Int4 versions of every functions, note that a 'float v[X]' function argument is the same as 'float* v',
    //   the array syntax is just a way to document the number of elements that are expected to be accessible. You can pass address of your first element out of a contiguous set, e.g. &myvector.x
    // - Adjust format string to decorate the value with a prefix, a suffix, or adapt the editing and display precision e.g. "%.3f" -> 1.234; "%5.2f secs" -> 01.23 secs; "Biscuit: %.0f" -> Biscuit: 1; etc.
    // - Format string may also be set to NULL or use the default format ("%f" or "%d").
    // - Speed are per-pixel of mouse movement (v_speed=0.2f: mouse needs to move by 5 pixels to increase value by 1). For gamepad/keyboard navigation, minimum speed is Max(v_speed, minimum_step_at_given_precision).
    // - Use v_min < v_max to clamp edits to given limits. Note that CTRL+Click manual input can override those limits if ImGuiSliderFlags_AlwaysClamp is not used.
    // - Use v_max = f32::MAX / INT_MAX etc to avoid clamping to a maximum, same with v_min = -f32::MAX / INT_MIN to avoid clamping to a minimum.
    // - We use the same sets of flags for DragXXX() and SliderXXX() functions as the features are the same and it makes it easier to swap them.
    // - Legacy: Pre-1.78 there are DragXXX() function signatures that takes a final `float power=1.0f' argument instead of the `ImGuiSliderFlags flags=0' argument.
    //   If you get a warning converting a float to ImGuiSliderFlags, read https://github.com/ocornut/imgui/issues/3361
     bool          DragFloat(label: *const c_char, c_float* v, let v_speed: c_float =  1.0, let v_min: c_float =  0.0, let v_max: c_float =  0.0, format: *const c_char = "%.3f", ImGuiSliderFlags flags = 0);     // If v_min >= v_max we have no bound
     bool          DragFloat2(label: *const c_char,v: c_float[2], let v_speed: c_float =  1.0, let v_min: c_float =  0.0, let v_max: c_float =  0.0, format: *const c_char = "%.3f", ImGuiSliderFlags flags = 0);
     bool          DragFloat3(label: *const c_char,v: c_float[3], let v_speed: c_float =  1.0, let v_min: c_float =  0.0, let v_max: c_float =  0.0, format: *const c_char = "%.3f", ImGuiSliderFlags flags = 0);
     bool          DragFloat4(label: *const c_char,v: c_float[4], let v_speed: c_float =  1.0, let v_min: c_float =  0.0, let v_max: c_float =  0.0, format: *const c_char = "%.3f", ImGuiSliderFlags flags = 0);
     bool          DragFloatRange2(label: *const c_char, c_float* v_current_min, c_float* v_current_max, let v_speed: c_float =  1.0, let v_min: c_float =  0.0, let v_max: c_float =  0.0, format: *const c_char = "%.3f", format_max: *const c_char = null_mut(), ImGuiSliderFlags flags = 0);
     bool          DragInt(label: *const c_char, v:  *mut c_int, let v_speed: c_float =  1.0, let v_min: c_int = 0, let v_max: c_int = 0, format: *const c_char = "%d", ImGuiSliderFlags flags = 0);  // If v_min >= v_max we have no bound
     bool          DragInt2(label: *const c_char, v: c_int[2], let v_speed: c_float =  1.0, let v_min: c_int = 0, let v_max: c_int = 0, format: *const c_char = "%d", ImGuiSliderFlags flags = 0);
     bool          DragInt3(label: *const c_char, v: c_int[3], let v_speed: c_float =  1.0, let v_min: c_int = 0, let v_max: c_int = 0, format: *const c_char = "%d", ImGuiSliderFlags flags = 0);
     bool          DragInt4(label: *const c_char, v: c_int[4], let v_speed: c_float =  1.0, let v_min: c_int = 0, let v_max: c_int = 0, format: *const c_char = "%d", ImGuiSliderFlags flags = 0);
     bool          DragIntRange2(label: *const c_char, v_current_min:  *mut c_int, v_current_max:  *mut c_int, let v_speed: c_float =  1.0, let v_min: c_int = 0, let v_max: c_int = 0, format: *const c_char = "%d", format_max: *const c_char = null_mut(), ImGuiSliderFlags flags = 0);
     bool          DragScalar(label: *const c_char, ImGuiDataType data_type, p_data: *mut c_void, let v_speed: c_float =  1.0, p_min: *const c_void = null_mut(), p_max: *const c_void = null_mut(), format: *const c_char = null_mut(), ImGuiSliderFlags flags = 0);
     bool          DragScalarN(label: *const c_char, ImGuiDataType data_type, p_data: *mut c_void, components: c_int, let v_speed: c_float =  1.0, p_min: *const c_void = null_mut(), p_max: *const c_void = null_mut(), format: *const c_char = null_mut(), ImGuiSliderFlags flags = 0);

    // Widgets: Regular Sliders
    // - CTRL+Click on any slider to turn them into an input box. Manually input values aren't clamped by default and can go off-bounds. Use ImGuiSliderFlags_AlwaysClamp to always clamp.
    // - Adjust format string to decorate the value with a prefix, a suffix, or adapt the editing and display precision e.g. "%.3f" -> 1.234; "%5.2f secs" -> 01.23 secs; "Biscuit: %.0f" -> Biscuit: 1; etc.
    // - Format string may also be set to NULL or use the default format ("%f" or "%d").
    // - Legacy: Pre-1.78 there are SliderXXX() function signatures that takes a final `float power=1.0f' argument instead of the `ImGuiSliderFlags flags=0' argument.
    //   If you get a warning converting a float to ImGuiSliderFlags, read https://github.com/ocornut/imgui/issues/3361
     bool          SliderFloat(label: *const c_char, c_float* v,v_min: c_float,v_max: c_float, format: *const c_char = "%.3f", ImGuiSliderFlags flags = 0);     // adjust format to decorate the value with a prefix or a suffix for in-slider labels or unit display.
     bool          SliderFloat2(label: *const c_char,v: c_float[2],v_min: c_float,v_max: c_float, format: *const c_char = "%.3f", ImGuiSliderFlags flags = 0);
     bool          SliderFloat3(label: *const c_char,v: c_float[3],v_min: c_float,v_max: c_float, format: *const c_char = "%.3f", ImGuiSliderFlags flags = 0);
     bool          SliderFloat4(label: *const c_char,v: c_float[4],v_min: c_float,v_max: c_float, format: *const c_char = "%.3f", ImGuiSliderFlags flags = 0);
     bool          SliderAngle(label: *const c_char, c_float* v_rad, let v_degrees_min: c_float =  -360f32, let v_degrees_max: c_float =  360f32, format: *const c_char = "%.0.0 deg", ImGuiSliderFlags flags = 0);
     bool          SliderInt(label: *const c_char, v:  *mut c_int, v_min: c_int, v_max: c_int, format: *const c_char = "%d", ImGuiSliderFlags flags = 0);
     bool          SliderInt2(label: *const c_char, v: c_int[2], v_min: c_int, v_max: c_int, format: *const c_char = "%d", ImGuiSliderFlags flags = 0);
     bool          SliderInt3(label: *const c_char, v: c_int[3], v_min: c_int, v_max: c_int, format: *const c_char = "%d", ImGuiSliderFlags flags = 0);
     bool          SliderInt4(label: *const c_char, v: c_int[4], v_min: c_int, v_max: c_int, format: *const c_char = "%d", ImGuiSliderFlags flags = 0);
     bool          SliderScalar(label: *const c_char, ImGuiDataType data_type, p_data: *mut c_void, p_min: *const c_void, p_max: *const c_void, format: *const c_char = null_mut(), ImGuiSliderFlags flags = 0);
     bool          SliderScalarN(label: *const c_char, ImGuiDataType data_type, p_data: *mut c_void, components: c_int, p_min: *const c_void, p_max: *const c_void, format: *const c_char = null_mut(), ImGuiSliderFlags flags = 0);
     bool          VSliderFloat(label: *const c_char, size: &ImVec2, c_float* v,v_min: c_float,v_max: c_float, format: *const c_char = "%.3f", ImGuiSliderFlags flags = 0);
     bool          VSliderInt(label: *const c_char, size: &ImVec2, v:  *mut c_int, v_min: c_int, v_max: c_int, format: *const c_char = "%d", ImGuiSliderFlags flags = 0);
     bool          VSliderScalar(label: *const c_char, size: &ImVec2, ImGuiDataType data_type, p_data: *mut c_void, p_min: *const c_void, p_max: *const c_void, format: *const c_char = null_mut(), ImGuiSliderFlags flags = 0);

    // Widgets: Input with Keyboard
    // - If you want to use InputText() with std::string or any custom dynamic string type, see misc/cpp/imgui_stdlib.h and comments in imgui_demo.cpp.
    // - Most of the ImGuiInputTextFlags flags are only useful for InputText() and not for InputFloatX, InputIntX, InputDouble etc.
     bool          InputText(label: *const c_char, char* buf, buf_size: size_t, ImGuiInputTextFlags flags = 0, ImGuiInputTextCallback callback = null_mut(), user_data: *mut c_void = null_mut());
     bool          InputTextMultiline(label: *const c_char, char* buf, buf_size: size_t, size: &ImVec2 = ImVec2::new(0, 0), ImGuiInputTextFlags flags = 0, ImGuiInputTextCallback callback = null_mut(), user_data: *mut c_void = null_mut());
     bool          InputTextWithHint(label: *const c_char, hint: *const c_char, char* buf, buf_size: size_t, ImGuiInputTextFlags flags = 0, ImGuiInputTextCallback callback = null_mut(), user_data: *mut c_void = null_mut());
     bool          InputFloat(label: *const c_char, c_float* v, let step: c_float =  0.0, let step_fast: c_float =  0.0, format: *const c_char = "%.3f", ImGuiInputTextFlags flags = 0);
     bool          InputFloat2(label: *const c_char,v: c_float[2], format: *const c_char = "%.3f", ImGuiInputTextFlags flags = 0);
     bool          InputFloat3(label: *const c_char,v: c_float[3], format: *const c_char = "%.3f", ImGuiInputTextFlags flags = 0);
     bool          InputFloat4(label: *const c_char,v: c_float[4], format: *const c_char = "%.3f", ImGuiInputTextFlags flags = 0);
     bool          InputInt(label: *const c_char, v:  *mut c_int, let step: c_int = 1, let step_fast: c_int = 100, ImGuiInputTextFlags flags = 0);
     bool          InputInt2(label: *const c_char, v: c_int[2], ImGuiInputTextFlags flags = 0);
     bool          InputInt3(label: *const c_char, v: c_int[3], ImGuiInputTextFlags flags = 0);
     bool          InputInt4(label: *const c_char, v: c_int[4], ImGuiInputTextFlags flags = 0);
     bool          InputDouble(label: *const c_char, double* v, double step = 0.0, double step_fast = 0.0, format: *const c_char = "%.6f", ImGuiInputTextFlags flags = 0);
     bool          InputScalar(label: *const c_char, ImGuiDataType data_type, p_data: *mut c_void, p_step: *const c_void = null_mut(), p_step_fast: *const c_void = null_mut(), format: *const c_char = null_mut(), ImGuiInputTextFlags flags = 0);
     bool          InputScalarN(label: *const c_char, ImGuiDataType data_type, p_data: *mut c_void, components: c_int, p_step: *const c_void = null_mut(), p_step_fast: *const c_void = null_mut(), format: *const c_char = null_mut(), ImGuiInputTextFlags flags = 0);

    // Widgets: Color Editor/Picker (tip: the ColorEdit* functions have a little color square that can be left-clicked to open a picker, and right-clicked to open an option menu.)
    // - Note that in C++ a 'float v[X]' function argument is the _same_ as 'float* v', the array syntax is just a way to document the number of elements that are expected to be accessible.
    // - You can pass the address of a first float element out of a contiguous structure, e.g. &myvector.x
     bool          ColorEdit3(label: *const c_char,col: c_float[3], ImGuiColorEditFlags flags = 0);
     bool          ColorEdit4(label: *const c_char,col: c_float[4], ImGuiColorEditFlags flags = 0);
     bool          ColorPicker3(label: *const c_char,col: c_float[3], ImGuiColorEditFlags flags = 0);
     bool          ColorPicker4(label: *const c_char,col: c_float[4], ImGuiColorEditFlags flags = 0, *let ref_col: c_float =  null_mut());
     bool          ColorButton(desc_id: *const c_char, const ImVec4& col, ImGuiColorEditFlags flags = 0, size: &ImVec2 = ImVec2::new(0, 0)); // display a color square/button, hover for details, return true when pressed.
     c_void          SetColorEditOptions(ImGuiColorEditFlags flags);                     // initialize current options (generally on application startup) if you want to select a default format, picker type, etc. User will be able to change many settings, unless you pass the _NoOptions flag to your calls.

    // Widgets: Trees
    // - TreeNode functions return true when the node is open, in which case you need to also call TreePop() when you are finished displaying the tree node contents.
     bool          TreeNode(label: *const c_char);
     bool          TreeNode(str_id: *const c_char, fmt: *const c_char, ...) IM_FMTARGS(2);   // helper variation to easily decorelate the id from the displayed string. Read the FAQ about why and how to use ID. to align arbitrary text at the same level as a TreeNode() you can use Bullet().
     bool          TreeNode(ptr_id: *const c_void, fmt: *const c_char, ...) IM_FMTARGS(2);   // "
     bool          TreeNodeV(str_id: *const c_char, fmt: *const c_char, va_list args) IM_FMTLIST(2);
     bool          TreeNodeV(ptr_id: *const c_void, fmt: *const c_char, va_list args) IM_FMTLIST(2);
     bool          TreeNodeEx(label: *const c_char, ImGuiTreeNodeFlags flags = 0);
     bool          TreeNodeEx(str_id: *const c_char, ImGuiTreeNodeFlags flags, fmt: *const c_char, ...) IM_FMTARGS(3);
     bool          TreeNodeEx(ptr_id: *const c_void, ImGuiTreeNodeFlags flags, fmt: *const c_char, ...) IM_FMTARGS(3);
     bool          TreeNodeExV(str_id: *const c_char, ImGuiTreeNodeFlags flags, fmt: *const c_char, va_list args) IM_FMTLIST(3);
     bool          TreeNodeExV(ptr_id: *const c_void, ImGuiTreeNodeFlags flags, fmt: *const c_char, va_list args) IM_FMTLIST(3);
     c_void          TreePush(str_id: *const c_char);                                       // ~ Indent()+PushId(). Already called by TreeNode() when returning true, but you can call TreePush/TreePop yourself if desired.
     c_void          TreePush(ptr_id: *const c_void = null_mut());                                // "
     c_void          TreePop();                                                          // ~ Unindent()+PopId()GetTreeNodeToLabelSpacing: c_float();                                        // horizontal distance preceding label when using TreeNode*() or Bullet() == (g.FontSize + style.FramePadding.x*2) for a regular unframed TreeNode
     bool          CollapsingHeader(label: *const c_char, ImGuiTreeNodeFlags flags = 0);  // if returning 'true' the header is open. doesn't indent nor push on ID stack. user doesn't have to call TreePop().
     bool          CollapsingHeader(label: *const c_char,p_visible: *mut bool, ImGuiTreeNodeFlags flags = 0); // when 'p_visible != NULL': if '*p_visible==true' display an additional small close button on upper right of the header which will set the to: bool false when clicked, if '*p_visible==false' don't display the header.
     c_void          SetNextItemOpen(is_open: bool, cond: ImGuiCond = 0);                  // set next TreeNode/CollapsingHeader open state.

    // Widgets: Selectables
    // - A selectable highlights when hovered, and can display another color when selected.
    // - Neighbors selectable extend their highlight bounds in order to leave no gap between them. This is so a series of selected Selectable appear contiguous.
     bool          Selectable(label: *const c_char, let mut selected: bool =  false, ImGuiSelectableFlags flags = 0, size: &ImVec2 = ImVec2::new(0, 0)); // "selected: bool" carry the selection state (read-only). Selectable() is clicked is returns true so you can modify your selection state. size.x==0.0: use remaining width, size.x>0.0: specify width. size.y==0.0: use label height, size.y>0.0: specify height
     bool          Selectable(label: *const c_char,p_selected: *mut bool, ImGuiSelectableFlags flags = 0, size: &ImVec2 = ImVec2::new(0, 0));      // "bool* p_selected" point to the selection state (read-write), as a convenient helper.

    // Widgets: List Boxes
    // - This is essentially a thin wrapper to using BeginChild/EndChild with some stylistic changes.
    // - The BeginListBox()/EndListBox() api allows you to manage your contents and selection state however you want it, by creating e.g. Selectable() or any items.
    // - The simplified/old ListBox() api are helpers over BeginListBox()/EndListBox() which are kept available for convenience purpose. This is analoguous to how Combos are created.
    // - Choose frame width:   size.x > 0.0: custom  /  size.x < 0.0 or -FLT_MIN: right-align   /  size.x = 0.0 (default): use current ItemWidth
    // - Choose frame height:  size.y > 0.0: custom  /  size.y < 0.0 or -FLT_MIN: bottom-align  /  size.y = 0.0 (default): arbitrary default height which can fit ~7 items
     bool          BeginListBox(label: *const c_char, size: &ImVec2 = ImVec2::new(0, 0)); // open a framed scrolling region
     c_void          EndListBox();                                                       // only call EndListBox() if BeginListBox() returned true!
     bool          ListBox(label: *const c_char, current_item:  *mut c_int, const: *const c_char items[], items_count: c_int, let height_in_items: c_int = -1);
     bool          ListBox(label: *const c_char, current_item:  *mut c_int, bool (*items_getter)(data: *mut c_void, idx: c_int, *const char* out_text), data: *mut c_void, items_count: c_int, let height_in_items: c_int = -1);

    // Widgets: Data Plotting
    // - Consider using ImPlot (https://github.com/epezent/implot) which is much better!
     c_void          PlotLines(label: *const c_char, *values: c_float, values_count: c_int, let values_offset: c_int = 0, overlay_text: *const c_char = null_mut(), let scale_min: c_float =  f32::MAX, let scale_max: c_float =  f32::MAX, let mut graph_size: ImVec2 =  ImVec2::new(0, 0), let stride: c_int = sizeof);
     c_void          PlotLines(label: *const c_char, c_float(*values_getter)(data: *mut c_void, idx: c_int), data: *mut c_void, values_count: c_int, let values_offset: c_int = 0, overlay_text: *const c_char = null_mut(), let scale_min: c_float =  f32::MAX, let scale_max: c_float =  f32::MAX, let mut graph_size: ImVec2 =  ImVec2::new(0, 0));
     c_void          PlotHistogram(label: *const c_char, *values: c_float, values_count: c_int, let values_offset: c_int = 0, overlay_text: *const c_char = null_mut(), let scale_min: c_float =  f32::MAX, let scale_max: c_float =  f32::MAX, let mut graph_size: ImVec2 =  ImVec2::new(0, 0), let stride: c_int = sizeof);
     c_void          PlotHistogram(label: *const c_char, c_float(*values_getter)(data: *mut c_void, idx: c_int), data: *mut c_void, values_count: c_int, let values_offset: c_int = 0, overlay_text: *const c_char = null_mut(), let scale_min: c_float =  f32::MAX, let scale_max: c_float =  f32::MAX, let mut graph_size: ImVec2 =  ImVec2::new(0, 0));

    // Widgets: Value() Helpers.
    // - Those are merely shortcut to calling Text() with a format string. Output single value in "name: value" format (tip: freely declare more in your code to handle your types. you can add functions to the ImGui namespace)
     c_void          Value(prefix: *const c_char, b: bool);
     c_void          Value(prefix: *const c_char, v: c_int);
     c_void          Value(prefix: *const c_char, v: c_uint);
     c_void          Value(prefix: *const c_char,v: c_float, float_format: *const c_char = null_mut());

    // Widgets: Menus
    // - Use BeginMenuBar() on a window ImGuiWindowFlags_MenuBar to append to its menu bar.
    // - Use BeginMainMenuBar() to create a menu bar at the top of the screen and append to it.
    // - Use BeginMenu() to create a menu. You can call BeginMenu() multiple time with the same identifier to append more items to it.
    // - Not that MenuItem() keyboardshortcuts are displayed as a convenience but _not processed_ by Dear ImGui at the moment.
     bool          BeginMenuBar();                                                     // append to menu-bar of current window (requires ImGuiWindowFlags_MenuBar flag set on parent window).
     c_void          EndMenuBar();                                                       // only call EndMenuBar() if BeginMenuBar() returns true!
     bool          BeginMainMenuBar();                                                 // create and append to a full screen menu-bar.
     c_void          EndMainMenuBar();                                                   // only call EndMainMenuBar() if BeginMainMenuBar() returns true!
     bool          BeginMenu(label: *const c_char, let mut enabled: bool =  true);                  // create a sub-menu entry. only call EndMenu() if this returns true!
     c_void          EndMenu();                                                          // only call EndMenu() if BeginMenu() returns true!
     bool          MenuItem(label: *const c_char, shortcut: *const c_char = null_mut(), let mut selected: bool =  false, let mut enabled: bool =  true);  // return true when activated.
     bool          MenuItem(label: *const c_char, shortcut: *const c_char,p_selected: *mut bool, let mut enabled: bool =  true);              // return true when activated + toggle (*p_selected) if p_selected != NULL

    // Tooltips
    // - Tooltip are windows following the mouse. They do not take focus away.
     c_void          BeginTooltip();                                                     // begin/append a tooltip window. to create full-featured tooltip (with any kind of items).
     c_void          EndTooltip();
     c_void          SetTooltip(fmt: *const c_char, ...) IM_FMTARGS(1);                     // set a text-only tooltip, typically use with IsItemHovered(). override any previous call to SetTooltip().
     c_void          SetTooltipV(fmt: *const c_char, va_list args) IM_FMTLIST(1);

    // Popups, Modals
    //  - They block normal mouse hovering detection (and therefore most mouse interactions) behind them.
    //  - If not modal: they can be closed by clicking anywhere outside them, or by pressing ESCAPE.
    //  - Their visibility state (~bool) is held internally instead of being held by the programmer as we are used to with regular Begin*() calls.
    //  - The 3 properties above are related: we need to retain popup visibility state in the library because popups may be closed as any time.
    //  - You can bypass the hovering restriction by using ImGuiHoveredFlags_AllowWhenBlockedByPopup when calling IsItemHovered() or IsWindowHovered().
    //  - IMPORTANT: Popup identifiers are relative to the current ID stack, so OpenPopup and BeginPopup generally needs to be at the same level of the stack.
    //    This is sometimes leading to confusing mistakes. May rework this in the future.

    // Popups: begin/end functions
    //  - BeginPopup(): query popup state, if open start appending into the window. Call EndPopup() afterwards. ImGuiWindowFlags are forwarded to the window.
    //  - BeginPopupModal(): block every interactions behind the window, cannot be closed by user, add a dimming background, has a title bar.
     bool          BeginPopup(str_id: *const c_char, flags: ImGuiWindowFlags = 0);                         // return true if the popup is open, and you can start outputting to it.
     bool          BeginPopupModal(name: *const c_char,p_open: *mut bool = null_mut(), flags: ImGuiWindowFlags = 0); // return true if the modal is open, and you can start outputting to it.
     c_void          EndPopup();                                                                         // only call EndPopup() if BeginPopupXXX() returns true!

    // Popups: open/close functions
    //  - OpenPopup(): set popup state to open. ImGuiPopupFlags are available for opening options.
    //  - If not modal: they can be closed by clicking anywhere outside them, or by pressing ESCAPE.
    //  - CloseCurrentPopup(): use inside the BeginPopup()/EndPopup() scope to close manually.
    //  - CloseCurrentPopup() is called by default by Selectable()/MenuItem() when activated (FIXME: need some options).
    //  - Use ImGuiPopupFlags_NoOpenOverExistingPopup to avoid opening a popup if there's already one at the same level. This is equivalent to e.g. testing for !IsAnyPopupOpen() prior to OpenPopup().
    //  - Use IsWindowAppearing() after BeginPopup() to tell if a window just opened.
    //  - IMPORTANT: Notice that for OpenPopupOnItemClick() we exceptionally default flags to 1 (== ImGuiPopupFlags_MouseButtonRight) for backward compatibility with older API taking 'int mouse_button = 1' parameter
     c_void          OpenPopup(str_id: *const c_char, popup_flags: ImGuiPopupFlags = 0);                     // call to mark popup as open (don't call every frame!).
     c_void          OpenPopup(id: ImGuiID, popup_flags: ImGuiPopupFlags = 0);                             // id overload to facilitate calling from nested stacks
     c_void          OpenPopupOnItemClick(str_id: *const c_char = null_mut(), popup_flags: ImGuiPopupFlags = 1);   // helper to open popup when clicked on last item. Default to ImGuiPopupFlags_MouseButtonRight == 1. (note: actually triggers on the mouse _released_ event to be consistent with popup behaviors)
     c_void          CloseCurrentPopup();                                                                // manually close the popup we have begin-ed into.

    // Popups: open+begin combined functions helpers
    //  - Helpers to do OpenPopup+BeginPopup where the Open action is triggered by e.g. hovering an item and right-clicking.
    //  - They are convenient to easily create context menus, hence the name.
    //  - IMPORTANT: Notice that BeginPopupContextXXX takes ImGuiPopupFlags just like OpenPopup() and unlike BeginPopup(). For full consistency, we may add ImGuiWindowFlags to the BeginPopupContextXXX functions in the future.
    //  - IMPORTANT: Notice that we exceptionally default their flags to 1 (== ImGuiPopupFlags_MouseButtonRight) for backward compatibility with older API taking 'int mouse_button = 1' parameter, so if you add other flags remember to re-add the ImGuiPopupFlags_MouseButtonRight.
     bool          BeginPopupContextItem(str_id: *const c_char = null_mut(), popup_flags: ImGuiPopupFlags = 1);  // open+begin popup when clicked on last item. Use str_id==NULL to associate the popup to previous item. If you want to use that on a non-interactive item such as Text() you need to pass in an explicit ID here. read comments in .cpp!
     bool          BeginPopupContextWindow(str_id: *const c_char = null_mut(), popup_flags: ImGuiPopupFlags = 1);// open+begin popup when clicked on current window.
     bool          BeginPopupContextVoid(str_id: *const c_char = null_mut(), popup_flags: ImGuiPopupFlags = 1);  // open+begin popup when clicked in void (where there are no windows).

    // Popups: query functions
    //  - IsPopupOpen(): return true if the popup is open at the current BeginPopup() level of the popup stack.
    //  - IsPopupOpen() with ImGuiPopupFlags_AnyPopupId: return true if any popup is open at the current BeginPopup() level of the popup stack.
    //  - IsPopupOpen() with ImGuiPopupFlags_AnyPopupId + ImGuiPopupFlags_AnyPopupLevel: return true if any popup is open.
     bool          IsPopupOpen(str_id: *const c_char, ImGuiPopupFlags flags = 0);                         // return true if the popup is open.

    // Tables
    // - Full-featured replacement for old Columns API.
    // - See Demo->Tables for demo code. See top of imgui_tables.cpp for general commentary.
    // - See ImGuiTableFlags_ and ImGuiTableColumnFlags_ enums for a description of available flags.
    // The typical call flow is:
    // - 1. Call BeginTable(), early out if returning false.
    // - 2. Optionally call TableSetupColumn() to submit column name/flags/defaults.
    // - 3. Optionally call TableSetupScrollFreeze() to request scroll freezing of columns/rows.
    // - 4. Optionally call TableHeadersRow() to submit a header row. Names are pulled from TableSetupColumn() data.
    // - 5. Populate contents:
    //    - In most situations you can use TableNextRow() + TableSetColumnIndex(N) to start appending into a column.
    //    - If you are using tables as a sort of grid, where every columns is holding the same type of contents,
    //      you may prefer using TableNextColumn() instead of TableNextRow() + TableSetColumnIndex().
    //      TableNextColumn() will automatically wrap-around into the next row if needed.
    //    - IMPORTANT: Comparatively to the old Columns() API, we need to call TableNextColumn() for the first column!
    //    - Summary of possible call flow:
    //        --------------------------------------------------------------------------------------------------------
    //        TableNextRow() -> TableSetColumnIndex(0) -> Text("Hello 0") -> TableSetColumnIndex(1) -> Text("Hello 1")  // OK
    //        TableNextRow() -> TableNextColumn()      -> Text("Hello 0") -> TableNextColumn()      -> Text("Hello 1")  // OK
    //                          TableNextColumn()      -> Text("Hello 0") -> TableNextColumn()      -> Text("Hello 1")  // OK: TableNextColumn() automatically gets to next row!
    //        TableNextRow()                           -> Text("Hello 0")                                               // Not OK! Missing TableSetColumnIndex() or TableNextColumn()! Text will not appear!
    //        --------------------------------------------------------------------------------------------------------
    // - 5. Call EndTable()
     bool          BeginTable(str_id: *const c_char, column: c_int, ImGuiTableFlags flags = 0, outer_size: &ImVec2 = ImVec2::new(0.0, 0.0), let inner_width: c_float =  0.0);
     c_void          EndTable();                                         // only call EndTable() if BeginTable() returns true!
     c_void          TableNextRow(ImGuiTableRowFlags row_flags = 0, let min_row_height: c_float =  0.0); // append into the first cell of a new row.
     bool          TableNextColumn();                                  // append into the next column (or first column of next row if currently in last column). Return true when column is visible.
     bool          TableSetColumnIndex(column_n: c_int);                  // append into the specified column. Return true when column is visible.

    // Tables: Headers & Columns declaration
    // - Use TableSetupColumn() to specify label, resizing policy, default width/weight, id, various other flags etc.
    // - Use TableHeadersRow() to create a header row and automatically submit a TableHeader() for each column.
    //   Headers are required to perform: reordering, sorting, and opening the context menu.
    //   The context menu can also be made available in columns body using ImGuiTableFlags_ContextMenuInBody.
    // - You may manually submit headers using TableNextRow() + TableHeader() calls, but this is only useful in
    //   some advanced use cases (e.g. adding custom widgets in header row).
    // - Use TableSetupScrollFreeze() to lock columns/rows so they stay visible when scrolled.
     c_void          TableSetupColumn(label: *const c_char, ImGuiTableColumnFlags flags = 0, let init_width_or_weight: c_float =  0.0, let mut user_id: ImGuiID =  0);
     c_void          TableSetupScrollFreeze(cols: c_int, rows: c_int);         // lock columns/rows so they stay visible when scrolled.
     c_void          TableHeadersRow();                                  // submit all headers cells based on data provided to TableSetupColumn() + submit context menu
     c_void          TableHeader(label: *const c_char);                     // submit one header cell manually (rarely used)

    // Tables: Sorting & Miscellaneous functions
    // - Sorting: call TableGetSortSpecs() to retrieve latest sort specs for the table. NULL when not sorting.
    //   When 'sort_specs->SpecsDirty == true' you should sort your data. It will be true when sorting specs have
    //   changed since last call, or the first time. Make sure to set 'SpecsDirty = false' after sorting,
    //   else you may wastefully sort your data every frame!
    // - Functions args 'int column_n' treat the default value of -1 as the same as passing the current column index.
     ImGuiTableSortSpecs*  TableGetSortSpecs();                        // get latest sort specs for the table (NULL if not sorting).  Lifetime: don't hold on this pointer over multiple frames or past any subsequent call to BeginTable().
     c_int                   TableGetColumnCount();                      // return number of columns (value passed to BeginTable)
     c_int                   TableGetColumnIndex();                      // return current column index.
     c_int                   TableGetRowIndex();                         // return current row index.
     *const char           TableGetColumnName(let column_n: c_int = -1);      // return "" if column didn't have a name declared by TableSetupColumn(). Pass -1 to use current column.
     ImGuiTableColumnFlags TableGetColumnFlags(let column_n: c_int = -1);     // return column flags so you can query their Enabled/Visible/Sorted/Hovered status flags. Pass -1 to use current column.
     c_void                  TableSetColumnEnabled(column_n: c_int, v: bool);// change user accessible enabled/disabled state of a column. Set to false to hide the column. User can use the context menu to change this themselves (right-click in headers, or right-click in columns body with ImGuiTableFlags_ContextMenuInBody)
     c_void                  TableSetBgColor(ImGuiTableBgTarget target, color: u32, let column_n: c_int = -1);  // change the color of a cell, row, or column. See ImGuiTableBgTarget_ flags for details.

    // Legacy Columns API (prefer using Tables!)
    // - You can also use SameLine(pos_x) to mimic simplified columns.
     c_void          Columns(let count: c_int = 1, id: *const c_char = null_mut(), let mut border: bool =  true);
     c_void          NextColumn();                                                       // next column, defaults to current row or next row if the current row is finished
     c_int           GetColumnIndex();                                                   // get current column indexGetColumnWidth: c_float(let column_index: c_int = -1);                              // get column width (in pixels). pass -1 to use current column
     c_void          SetColumnWidth(column_index: c_int,width: c_float);                      // set column width (in pixels). pass -1 to use current columnGetColumnOffset: c_float(let column_index: c_int = -1);                             // get position of column line (in pixels, from the left side of the contents region). pass -1 to use current column, otherwise 0..GetColumnsCount() inclusive. column 0 is typically 0.0
     c_void          SetColumnOffset(column_index: c_int,offset_x: c_float);                  // set position of column line (in pixels, from the left side of the contents region). pass -1 to use current column
     c_int           GetColumnsCount();

    // Tab Bars, Tabs
    // Note: Tabs are automatically created by the docking system. Use this to create tab bars/tabs yourself without docking being involved.
     bool          BeginTabBar(str_id: *const c_char, ImGuiTabBarFlags flags = 0);        // create and append into a TabBar
     c_void          EndTabBar();                                                        // only call EndTabBar() if BeginTabBar() returns true!
     bool          BeginTabItem(label: *const c_char,p_open: *mut bool = null_mut(), ImGuiTabItemFlags flags = 0); // create a Tab. Returns true if the Tab is selected.
     c_void          EndTabItem();                                                       // only call EndTabItem() if BeginTabItem() returns true!
     bool          TabItemButton(label: *const c_char, ImGuiTabItemFlags flags = 0);      // create a Tab behaving like a button. return true when clicked. cannot be selected in the tab bar.
     c_void          SetTabItemClosed(tab_or_docked_window_label: *const c_char);           // notify TabBar or Docking system of a closed tab/window ahead (useful to reduce visual flicker on reorderable tab bars). For tab-bar: call after BeginTabBar() and before Tab submissions. Otherwise call with a window name.

    // Docking
    // [BETA API] Enable with io.ConfigFlags |= ImGuiConfigFlags_DockingEnable.
    // Note: You can use most Docking facilities without calling any API. You DO NOT need to call DockSpace() to use Docking!
    // - Drag from window title bar or their tab to dock/undock. Hold SHIFT to disable docking/undocking.
    // - Drag from window menu button (upper-left button) to undock an entire node (all windows).
    // - When io.ConfigDockingWithShift == true, you instead need to hold SHIFT to _enable_ docking/undocking.
    // About dockspaces:
    // - Use DockSpace() to create an explicit dock node _within_ an existing window. See Docking demo for details.
    // - Use DockSpaceOverViewport() to create an explicit dock node covering the screen or a specific viewport.
    //   This is often used with ImGuiDockNodeFlags_PassthruCentralNode.
    // - Important: Dockspaces need to be submitted _before_ any window they can host. Submit it early in your frame!
    // - Important: Dockspaces need to be kept alive if hidden, otherwise windows docked into it will be undocked.
    //   e.g. if you have multiple tabs with a dockspace inside each tab: submit the non-visible dockspaces with ImGuiDockNodeFlags_KeepAliveOnly.
     ImGuiID       DockSpace(id: ImGuiID, size: &ImVec2 = ImVec2::new(0, 0), ImGuiDockNodeFlags flags = 0, *const ImGuiWindowClass window_class = null_mut());
     ImGuiID       DockSpaceOverViewport(*const ImGuiViewport viewport = null_mut(), ImGuiDockNodeFlags flags = 0, *const ImGuiWindowClass window_class = null_mut());
     c_void          SetNextWindowDockID(dock_id: ImGuiID, cond: ImGuiCond = 0);           // set next window dock id
     c_void          SetNextWindowClass(*const ImGuiWindowClass window_class);           // set next window class (control docking compatibility + provide hints to platform backend via custom viewport flags and platform parent/child relationship)
     ImGuiID       GetWindowDockID();
     bool          IsWindowDocked();                                                   // is current window docked into another window?

    // Logging/Capture
    // - All text output from the interface can be captured into tty/file/clipboard. By default, tree nodes are automatically opened during logging.
     c_void          LogToTTY(let auto_open_depth: c_int = -1);                                 // start logging to tty (stdout)
     c_void          LogToFile(let auto_open_depth: c_int = -1, filename: *const c_char = null_mut());   // start logging to file
     c_void          LogToClipboard(let auto_open_depth: c_int = -1);                           // start logging to OS clipboard
     c_void          LogFinish();                                                        // stop logging (close file, etc.)
     c_void          LogButtons();                                                       // helper to display buttons for logging to tty/file/clipboard
     c_void          LogText(fmt: *const c_char, ...) IM_FMTARGS(1);                        // pass text data straight to log (without being displayed)
     c_void          LogTextV(fmt: *const c_char, va_list args) IM_FMTLIST(1);

    // Drag and Drop
    // - On source items, call BeginDragDropSource(), if it returns true also call SetDragDropPayload() + EndDragDropSource().
    // - On target candidates, call BeginDragDropTarget(), if it returns true also call AcceptDragDropPayload() + EndDragDropTarget().
    // - If you stop calling BeginDragDropSource() the payload is preserved however it won't have a preview tooltip (we currently display a fallback "..." tooltip, see #1725)
    // - An item can be both drag source and drop target.
     bool          BeginDragDropSource(flags: ImGuiDragDropFlags = 0);                                      // call after submitting an item which may be dragged. when this return true, you can call SetDragDropPayload() + EndDragDropSource()
     bool          SetDragDropPayload(type: *const c_char, data: *const c_void, sz: size_t, cond: ImGuiCond = 0);  // type is a user defined string of maximum 32 characters. Strings starting with '_' are reserved for dear imgui internal types. Data is copied and held by imgui. Return true when payload has been accepted.
     c_void          EndDragDropSource();                                                                    // only call EndDragDropSource() if BeginDragDropSource() returns true!
     bool                  BeginDragDropTarget();                                                          // call after submitting an item that may receive a payload. If this returns true, you can call AcceptDragDropPayload() + EndDragDropTarget()
     *const ImGuiPayload   AcceptDragDropPayload(type: *const c_char, flags: ImGuiDragDropFlags = 0);          // accept contents of a given type. If ImGuiDragDropFlags_AcceptBeforeDelivery is set you can peek into the payload before the mouse button is released.
     c_void                  EndDragDropTarget();                                                            // only call EndDragDropTarget() if BeginDragDropTarget() returns true!
     *const ImGuiPayload   GetDragDropPayload();                                                           // peek directly into the current payload from anywhere. may return NULL. use ImGuiPayload::IsDataType() to test for the payload type.

    // Disabling [BETA API]
    // - Disable all user interactions and dim items visuals (applying style.DisabledAlpha over current colors)
    // - Those can be nested but it cannot be used to enable an already disabled section (a single BeginDisabled(true) in the stack is enough to keep everything disabled)
    // - BeginDisabled(false) essentially does nothing useful but is provided to facilitate use of boolean expressions. If you can avoid calling BeginDisabled(False)/EndDisabled() best to avoid it.
     c_void          BeginDisabled(let mut disabled: bool =  true);
     c_void          EndDisabled();

    // Clipping
    // - Mouse hovering is affected by PushClipRect() calls, unlike direct calls to ImDrawList::PushClipRect() which are render only.
     c_void          PushClipRect(clip_rect_min: &ImVec2, clip_rect_max: &ImVec2, intersect_with_current_clip_rect: bool);
     c_void          PopClipRect();

    // Focus, Activation
    // - Prefer using "SetItemDefaultFocus()" over "if (IsWindowAppearing()) SetScrollHereY()" when applicable to signify "this is the default item"
     c_void          SetItemDefaultFocus();                                              // make last item the default focused item of a window.
     c_void          SetKeyboardFocusHere(let offset: c_int = 0);                               // focus keyboard on the next widget. Use positive 'offset' to access sub components of a multiple component widget. Use -1 to access previous widget.

    // Item/Widgets Utilities and Query Functions
    // - Most of the functions are referring to the previous Item that has been submitted.
    // - See Demo Window under "Widgets->Querying Status" for an interactive visualization of most of those functions.
     bool          IsItemHovered(flags: ImGuiHoveredFlags = 0);                         // is the last item hovered? (and usable, aka not blocked by a popup, etc.). See for: ImGuiHoveredFlags more options.
     bool          IsItemActive();                                                     // is the last item active? (e.g. button being held, text field being edited. This will continuously return true while holding mouse button on an item. Items that don't interact will always return false)
     bool          IsItemFocused();                                                    // is the last item focused for keyboard/gamepad navigation?
     bool          IsItemClicked(let mut mouse_button: ImGuiMouseButton =  0);                   // is the last item hovered and mouse clicked on? (**)  == IsMouseClicked(mouse_button) && IsItemHovered()Important. (**) this it NOT equivalent to the behavior of e.g. Button(). Read comments in function definition.
     bool          IsItemVisible();                                                    // is the last item visible? (items may be out of sight because of clipping/scrolling)
     bool          IsItemEdited();                                                     // did the last item modify its underlying value this frame? or was pressed? This is generally the same as the "bool" return value of many widgets.
     bool          IsItemActivated();                                                  // was the last item just made active (item was previously inactive).
     bool          IsItemDeactivated();                                                // was the last item just made inactive (item was previously active). Useful for Undo/Redo patterns with widgets that requires continuous editing.
     bool          IsItemDeactivatedAfterEdit();                                       // was the last item just made inactive and made a value change when it was active? (e.g. Slider/Drag moved). Useful for Undo/Redo patterns with widgets that requires continuous editing. Note that you may get false positives (some widgets such as Combo()/ListBox()/Selectable() will return true even when clicking an already selected item).
     bool          IsItemToggledOpen();                                                // was the last item open state toggled? set by TreeNode().
     bool          IsAnyItemHovered();                                                 // is any item hovered?
     bool          IsAnyItemActive();                                                  // is any item active?
     bool          IsAnyItemFocused();                                                 // is any item focused?
     ImVec2        GetItemRectMin();                                                   // get upper-left bounding rectangle of the last item (screen space)
     ImVec2        GetItemRectMax();                                                   // get lower-right bounding rectangle of the last item (screen space)
     ImVec2        GetItemRectSize();                                                  // get size of last item
     c_void          SetItemAllowOverlap();                                              // allow last item to be overlapped by a subsequent item. sometimes useful with invisible buttons, selectables, etc. to catch unused area.

    // Viewports
    // - Currently represents the Platform Window created by the application which is hosting our Dear ImGui windows.
    // - In 'docking' branch with multi-viewport enabled, we extend this concept to have multiple active viewports.
    // - In the future we will extend this concept further to also represent Platform Monitor and support a "no main platform window" operation mode.
     ImGuiViewport* GetMainViewport();                                                 // return primary/default viewport. This can never be NULL.

    // Background/Foreground Draw Lists
     ImDrawList*   GetBackgroundDrawList();                                            // get background draw list for the viewport associated to the current window. this draw list will be the first rendering one. Useful to quickly draw shapes/text behind dear imgui contents.
     ImDrawList*   GetForegroundDrawList();                                            // get foreground draw list for the viewport associated to the current window. this draw list will be the last rendered one. Useful to quickly draw shapes/text over dear imgui contents.
     ImDrawList*   GetBackgroundDrawList(ImGuiViewport* viewport);                     // get background draw list for the given viewport. this draw list will be the first rendering one. Useful to quickly draw shapes/text behind dear imgui contents.
     ImDrawList*   GetForegroundDrawList(ImGuiViewport* viewport);                     // get foreground draw list for the given viewport. this draw list will be the last rendered one. Useful to quickly draw shapes/text over dear imgui contents.

    // Miscellaneous Utilities
     bool          IsRectVisible(size: &ImVec2);                                  // test if rectangle (of given size, starting from cursor position) is visible / not clipped.
     bool          IsRectVisible(rect_min: &ImVec2, rect_max: &ImVec2);      // test if rectangle (in screen space) is visible / not clipped. to perform coarse clipping on user's side.
     double        GetTime();                                                          // get global imgui time. incremented by io.DeltaTime every frame.
     c_int           GetFrameCount();                                                    // get global imgui frame count. incremented by 1 every frame.
     ImDrawListSharedData* GetDrawListSharedData();                                    // you may use this when creating your own ImDrawList instances.
     *const char   GetStyleColorName(ImGuiCol idx);                                    // get a string corresponding to the enum value (for display, saving, etc.).
     c_void          SetStateStorage(ImGuiStorage* storage);                             // replace current window storage with our own (if you want to manipulate it yourself, typically clear subsection of it)
     ImGuiStorage* GetStateStorage();
     bool          BeginChildFrame(id: ImGuiID, size: &ImVec2, flags: ImGuiWindowFlags = 0); // helper to create a child window / scrolling region that looks like a normal widget frame
     c_void          EndChildFrame();                                                    // always call EndChildFrame() regardless of BeginChildFrame() return values (which indicates a collapsed/clipped window)

    // Text Utilities
     ImVec2        CalcTextSize(text: *const c_char, text_end: *const c_char = null_mut(), let mut hide_text_after_double_hash: bool =  false, let wrap_width: c_float =  -1.0);

    // Color Utilities
     ImVec4        ColorConvertU32ToFloat4(in: u32);
     u32         ColorConvertFloat4ToU32(const ImVec4& in);
     c_void          ColorConvertRGBtoHSV(r: c_float,g: c_float,b: c_float, c_float& out_h, c_float& out_s, c_float& out_v);
     c_void          ColorConvertHSVtoRGB(h: c_float,s: c_float,v: c_float, c_float& out_r, c_float& out_g, c_float& out_b);

    // Inputs Utilities: Keyboard
    // Without IMGUI_DISABLE_OBSOLETE_KEYIO: (legacy support)
    //   - For 'ImGuiKey key' you can still use your legacy native/user indices according to how your backend/engine stored them in io.KeysDown[].
    // With IMGUI_DISABLE_OBSOLETE_KEYIO: (this is the way forward)
    //   - Any use of 'ImGuiKey' will assert when key < 512 will be passed, previously reserved as native/user keys indices
    //   - GetKeyIndex() is pass-through and therefore deprecated (gone if IMGUI_DISABLE_OBSOLETE_KEYIO is defined)
     bool          IsKeyDown(ImGuiKey key);                                            // is key being held.
     bool          IsKeyPressed(ImGuiKey key, let mut repeat: bool =  true);                     // was key pressed (went from !Down to Down)? if repeat=true, uses io.KeyRepeatDelay / KeyRepeatRate
     bool          IsKeyReleased(ImGuiKey key);                                        // was key released (went from Down to !Down)?
     c_int           GetKeyPressedAmount(ImGuiKey key,repeat_delay: c_float,rate: c_float);  // uses provided repeat rate/delay. return a count, most often 0 or 1 but might be >1 if RepeatRate is small enough that DeltaTime > RepeatRate
     *const char   GetKeyName(ImGuiKey key);                                           // [DEBUG] returns English name of the key. Those names a provided for debugging purpose and are not meant to be saved persistently not compared.
     c_void          SetNextFrameWantCaptureKeyboard(want_capture_keyboard: bool);        // Override io.WantCaptureKeyboard flag next frame (said flag is left for your application to handle, typically when true it instructs your app to ignore inputs). e.g. force capture keyboard when your widget is being hovered. This is equivalent to setting "io.WantCaptureKeyboard = want_capture_keyboard"; after the next NewFrame() call.

    // Inputs Utilities: Mouse
    // - To refer to a mouse button, you may use named enums in your code e.g. ImGuiMouseButton_Left, ImGuiMouseButton_Right.
    // - You can also use regular integer: it is forever guaranteed that 0=Left, 1=Right, 2=Middle.
    // - Dragging operations are only reported after mouse has moved a certain distance away from the initial clicking position (see 'lock_threshold' and 'io.MouseDraggingThreshold')
     bool          IsMouseDown(ImGuiMouseButton button);                               // is mouse button held?
     bool          IsMouseClicked(ImGuiMouseButton button, let mut repeat: bool =  false);       // did mouse button clicked? (went from !Down to Down). Same as GetMouseClickedCount() == 1.
     bool          IsMouseReleased(ImGuiMouseButton button);                           // did mouse button released? (went from Down to !Down)
     bool          IsMouseDoubleClicked(ImGuiMouseButton button);                      // did mouse button double-clicked? Same as GetMouseClickedCount() == 2. (note that a double-click will also report IsMouseClicked() == true)
     c_int           GetMouseClickedCount(ImGuiMouseButton button);                      // return the number of successive mouse-clicks at the time where a click happen (otherwise 0).
     bool          IsMouseHoveringRect(r_min: &ImVec2, r_max: &ImVec2, let mut clip: bool =  true);// is mouse hovering given bounding rect (in screen space). clipped by current clipping settings, but disregarding of other consideration of focus/window ordering/popup-block.
     bool          IsMousePosValid(*let mouse_pos: ImVec2 = null_mut());                    // by convention we use (-f32::MAX,-f32::MAX) to denote that there is no mouse available
     bool          IsAnyMouseDown();                                                   // [WILL OBSOLETE] is any mouse button held? This was designed for backends, but prefer having backend maintain a mask of held mouse buttons, because upcoming input queue system will make this invalid.
     ImVec2        GetMousePos();                                                      // shortcut to GetIO().MousePos provided by user, to be consistent with other calls
     ImVec2        GetMousePosOnOpeningCurrentPopup();                                 // retrieve mouse position at the time of opening popup we have BeginPopup() into (helper to avoid user backing that value themselves)
     bool          IsMouseDragging(ImGuiMouseButton button, let lock_threshold: c_float =  -1.0);         // is mouse dragging? (if lock_threshold < -1.0, uses io.MouseDraggingThreshold)
     ImVec2        GetMouseDragDelta(let mut button: ImGuiMouseButton =  0, let lock_threshold: c_float =  -1.0);   // return the delta from the initial clicking position while the mouse button is pressed or was just released. This is locked and return 0.0 until the mouse moves past a distance threshold at least once (if lock_threshold < -1.0, uses io.MouseDraggingThreshold)
     c_void          ResetMouseDragDelta(let mut button: ImGuiMouseButton =  0);                   //
     ImGuiMouseCursor GetMouseCursor();                                                // get desired cursor type, reset in NewFrame(), this is updated during the frame. valid before Render(). If you use software rendering by setting io.MouseDrawCursor ImGui will render those for you
     c_void          SetMouseCursor(ImGuiMouseCursor cursor_type);                       // set desired cursor type
     c_void          SetNextFrameWantCaptureMouse(want_capture_mouse: bool);              // Override io.WantCaptureMouse flag next frame (said flag is left for your application to handle, typical when true it instucts your app to ignore inputs). This is equivalent to setting "io.WantCaptureMouse = want_capture_mouse;" after the next NewFrame() call.

    // Clipboard Utilities
    // - Also see the LogToClipboard() function to capture GUI into clipboard, or easily output text data to the clipboard.
     *const char   GetClipboardText();
     c_void          SetClipboardText(text: *const c_char);

    // Settings/.Ini Utilities
    // - The disk functions are automatically called if io.IniFilename != NULL (default is "imgui.ini").
    // - Set io.IniFilename to NULL to load/save manually. Read io.WantSaveIniSettings description about handling .ini saving manually.
    // - Important: default value "imgui.ini" is relative to current working dir! Most apps will want to lock this to an absolute path (e.g. same path as executables).
     c_void          LoadIniSettingsFromDisk(ini_filename: *const c_char);                  // call after CreateContext() and before the first call to NewFrame(). NewFrame() automatically calls LoadIniSettingsFromDisk(io.IniFilename).
     c_void          LoadIniSettingsFromMemory(ini_data: *const c_char, ini_size: size_t=0); // call after CreateContext() and before the first call to NewFrame() to provide .ini data from your own data source.
     c_void          SaveIniSettingsToDisk(ini_filename: *const c_char);                    // this is automatically called (if io.IniFilename is not empty) a few seconds after any modification that should be reflected in the .ini file (and also by DestroyContext).
     *const char   SaveIniSettingsToMemory(size_t* out_ini_size = null_mut());               // return a zero-terminated string with the .ini data which you can save by your own mean. call when io.WantSaveIniSettings is set, then save data by your own mean and clear io.WantSaveIniSettings.

    // Debug Utilities
     c_void          DebugTextEncoding(text: *const c_char);
     bool          DebugCheckVersionAndDataLayout(version_str: *const c_char, sz_io: size_t, sz_style: size_t, sz_vec2: size_t, sz_vec4: size_t, sz_drawvert: size_t, sz_drawidx: size_t); // This is called by IMGUI_CHECKVERSION() macro.

    // Memory Allocators
    // - Those functions are not reliant on the current context.
    // - DLL users: heaps and globals are not shared across DLL boundaries! You will need to call SetCurrentContext() + SetAllocatorFunctions()
    //   for each static/DLL boundary you are calling from. Read "Context and Memory Allocators" section of imgui.cpp for more details.
     c_void          SetAllocatorFunctions(ImGuiMemAllocFunc alloc_func, ImGuiMemFreeFunc free_func, user_data: *mut c_void = null_mut());
     c_void          GetAllocatorFunctions(ImGuiMemAllocFunc* p_alloc_func, ImGuiMemFreeFunc* p_free_func, c_void** p_user_data);
     *mut c_void         MemAlloc(size: size_t);
     c_void          MemFree(ptr: *mut c_void);

    // (Optional) Platform/OS interface for multi-viewport support
    // Read comments around the ImGuiPlatformIO structure for more details.
    // Note: You may use GetWindowViewport() to get the current viewport of the current window.
     ImGuiPlatformIO&  GetPlatformIO();                                                // platform/renderer functions, for backend to setup + viewports list.
     c_void              UpdatePlatformWindows();                                        // call in main loop. will call CreateWindow/ResizeWindow/etc. platform functions for each secondary viewport, and DestroyWindow for each inactive viewport.
     c_void              RenderPlatformWindowsDefault(platform_render_arg: *mut c_void = null_mut(), renderer_render_arg: *mut c_void = null_mut()); // call in main loop. will call RenderWindow/SwapBuffers platform functions for each secondary viewport which doesn't have the ImGuiViewportFlags_Minimized flag set. May be reimplemented by user for custom rendering needs.
     c_void              DestroyPlatformWindows();                                       // call DestroyWindow platform functions for all viewports. call from backend Shutdown() if you need to close platform windows before imgui shutdown. otherwise will be called by DestroyContext().
     ImGuiViewport*    FindViewportByID(id: ImGuiID);                                   // this is a helper for backends.
     ImGuiViewport*    FindViewportByPlatformHandle(platform_handle: *mut c_void);            // this is a helper for backends. the type platform_handle is decided by the backend (e.g. HWND, MyWindow*, GLFWwindow* etc.)

} // namespace ImGui

//-----------------------------------------------------------------------------
// [SECTION] Flags & Enumerations
//-----------------------------------------------------------------------------


// Flags for InputText()
enum ImGuiInputTextFlags_
{
    ImGuiInputTextFlags_None                = 0,
    ImGuiInputTextFlags_CharsDecimal        = 1 << 0,   // Allow 0123456789.+-*/
    ImGuiInputTextFlags_CharsHexadecimal    = 1 << 1,   // Allow 0123456789ABCDEFabcdef
    ImGuiInputTextFlags_CharsUppercase      = 1 << 2,   // Turn a..z into A..Z
    ImGuiInputTextFlags_CharsNoBlank        = 1 << 3,   // Filter out spaces, tabs
    ImGuiInputTextFlags_AutoSelectAll       = 1 << 4,   // Select entire text when first taking mouse focus
    ImGuiInputTextFlags_EnterReturnsTrue    = 1 << 5,   // Return 'true' when Enter is pressed (as opposed to every time the value was modified). Consider looking at the IsItemDeactivatedAfterEdit() function.
    ImGuiInputTextFlags_CallbackCompletion  = 1 << 6,   // Callback on pressing TAB (for completion handling)
    ImGuiInputTextFlags_CallbackHistory     = 1 << 7,   // Callback on pressing Up/Down arrows (for history handling)
    ImGuiInputTextFlags_CallbackAlways      = 1 << 8,   // Callback on each iteration. User code may query cursor position, modify text buffer.
    ImGuiInputTextFlags_CallbackCharFilter  = 1 << 9,   // Callback on character inputs to replace or discard them. Modify 'EventChar' to replace or discard, or return 1 in callback to discard.
    ImGuiInputTextFlags_AllowTabInput       = 1 << 10,  // Pressing TAB input a '\t' character into the text field
    ImGuiInputTextFlags_CtrlEnterForNewLine = 1 << 11,  // In multi-line mode, unfocus with Enter, add new line with Ctrl+Enter (default is opposite: unfocus with Ctrl+Enter, add line with Enter).
    ImGuiInputTextFlags_NoHorizontalScroll  = 1 << 12,  // Disable following the cursor horizontally
    ImGuiInputTextFlags_AlwaysOverwrite     = 1 << 13,  // Overwrite mode
    ImGuiInputTextFlags_ReadOnly            = 1 << 14,  // Read-only mode
    ImGuiInputTextFlags_Password            = 1 << 15,  // Password mode, display all characters as '*'
    ImGuiInputTextFlags_NoUndoRedo          = 1 << 16,  // Disable undo/redo. Note that input text owns the text data while active, if you want to provide your own undo/redo stack you need e.g. to call ClearActiveID().
    ImGuiInputTextFlags_CharsScientific     = 1 << 17,  // Allow 0123456789.+-*/eE (Scientific notation input)
    ImGuiInputTextFlags_CallbackResize      = 1 << 18,  // Callback on buffer capacity changes request (beyond 'buf_size' parameter value), allowing the string to grow. Notify when the string wants to be resized (for string types which hold a cache of their Size). You will be provided a new BufSize in the callback and NEED to honor it. (see misc/cpp/imgui_stdlib.h for an example of using this)
    ImGuiInputTextFlags_CallbackEdit        = 1 << 19,  // Callback on any edit (note that InputText() already returns true on edit, the callback is useful mainly to manipulate the underlying buffer while focus is active)

    // Obsolete names (will be removed soon)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    ImGuiInputTextFlags_AlwaysInsertMode    = ImGuiInputTextFlags_AlwaysOverwrite   // [renamed in 1.82] name was not matching behavior
// #endif
};

// Flags for TreeNodeEx(), CollapsingHeader*()
enum ImGuiTreeNodeFlags_
{
    ImGuiTreeNodeFlags_None                 = 0,
    ImGuiTreeNodeFlags_Selected             = 1 << 0,   // Draw as selected
    ImGuiTreeNodeFlags_Framed               = 1 << 1,   // Draw frame with background (e.g. for CollapsingHeader)
    ImGuiTreeNodeFlags_AllowItemOverlap     = 1 << 2,   // Hit testing to allow subsequent widgets to overlap this one
    ImGuiTreeNodeFlags_NoTreePushOnOpen     = 1 << 3,   // Don't do a TreePush() when open (e.g. for CollapsingHeader) = no extra indent nor pushing on ID stack
    ImGuiTreeNodeFlags_NoAutoOpenOnLog      = 1 << 4,   // Don't automatically and temporarily open node when Logging is active (by default logging will automatically open tree nodes)
    ImGuiTreeNodeFlags_DefaultOpen          = 1 << 5,   // Default node to be open
    ImGuiTreeNodeFlags_OpenOnDoubleClick    = 1 << 6,   // Need double-click to open node
    ImGuiTreeNodeFlags_OpenOnArrow          = 1 << 7,   // Only open when clicking on the arrow part. If ImGuiTreeNodeFlags_OpenOnDoubleClick is also set, single-click arrow or double-click all box to open.
    ImGuiTreeNodeFlags_Leaf                 = 1 << 8,   // No collapsing, no arrow (use as a convenience for leaf nodes).
    ImGuiTreeNodeFlags_Bullet               = 1 << 9,   // Display a bullet instead of arrow
    ImGuiTreeNodeFlags_FramePadding         = 1 << 10,  // Use FramePadding (even for an unframed text node) to vertically align text baseline to regular widget height. Equivalent to calling AlignTextToFramePadding().
    ImGuiTreeNodeFlags_SpanAvailWidth       = 1 << 11,  // Extend hit box to the right-most edge, even if not framed. This is not the default in order to allow adding other items on the same line. In the future we may refactor the hit system to be front-to-back, allowing natural overlaps and then this can become the default.
    ImGuiTreeNodeFlags_SpanFullWidth        = 1 << 12,  // Extend hit box to the left-most and right-most edges (bypass the indented area).
    ImGuiTreeNodeFlags_NavLeftJumpsBackHere = 1 << 13,  // (WIP) Nav: left direction may move to this TreeNode() from any of its child (items submitted between TreeNode and TreePop)
    //ImGuiTreeNodeFlags_NoScrollOnOpen     = 1 << 14,  // FIXME: TODO: Disable automatic scroll on TreePop() if node got just open and contents is not visible
    ImGuiTreeNodeFlags_CollapsingHeader     = ImGuiTreeNodeFlags_Framed | ImGuiTreeNodeFlags_NoTreePushOnOpen | ImGuiTreeNodeFlags_NoAutoOpenOnLog,
};



// Flags for Selectable()
enum ImGuiSelectableFlags_
{
    ImGuiSelectableFlags_None               = 0,
    ImGuiSelectableFlags_DontClosePopups    = 1 << 0,   // Clicking this don't close parent popup window
    ImGuiSelectableFlags_SpanAllColumns     = 1 << 1,   // Selectable frame can span all columns (text will still fit in current column)
    ImGuiSelectableFlags_AllowDoubleClick   = 1 << 2,   // Generate press events on double clicks too
    ImGuiSelectableFlags_Disabled           = 1 << 3,   // Cannot be selected, display grayed out text
    ImGuiSelectableFlags_AllowItemOverlap   = 1 << 4,   // (WIP) Hit testing to allow subsequent widgets to overlap this one
};

// Flags for BeginCombo()
enum ImGuiComboFlags_
{
    ImGuiComboFlags_None                    = 0,
    ImGuiComboFlags_PopupAlignLeft          = 1 << 0,   // Align the popup toward the left by default
    ImGuiComboFlags_HeightSmall             = 1 << 1,   // Max ~4 items visible. Tip: If you want your combo popup to be a specific size you can use SetNextWindowSizeConstraints() prior to calling BeginCombo()
    ImGuiComboFlags_HeightRegular           = 1 << 2,   // Max ~8 items visible (default)
    ImGuiComboFlags_HeightLarge             = 1 << 3,   // Max ~20 items visible
    ImGuiComboFlags_HeightLargest           = 1 << 4,   // As many fitting items as possible
    ImGuiComboFlags_NoArrowButton           = 1 << 5,   // Display on the preview box without the square arrow button
    ImGuiComboFlags_NoPreview               = 1 << 6,   // Display only a square arrow button
    ImGuiComboFlags_HeightMask_             = ImGuiComboFlags_HeightSmall | ImGuiComboFlags_HeightRegular | ImGuiComboFlags_HeightLarge | ImGuiComboFlags_HeightLargest,
};

// Flags for BeginTabBar()


// Flags for BeginTabItem()
enum ImGuiTabItemFlags_
{
    ImGuiTabItemFlags_None                          = 0,
    ImGuiTabItemFlags_UnsavedDocument               = 1 << 0,   // Display a dot next to the title + tab is selected when clicking the X + closure is not assumed (will wait for user to stop submitting the tab). Otherwise closure is assumed when pressing the X, so if you keep submitting the tab may reappear at end of tab bar.
    ImGuiTabItemFlags_SetSelected                   = 1 << 1,   // Trigger flag to programmatically make the tab selected when calling BeginTabItem()
    ImGuiTabItemFlags_NoCloseWithMiddleMouseButton  = 1 << 2,   // Disable behavior of closing tabs (that are submitted with p_open != NULL) with middle mouse button. You can still repro this behavior on user's side with if (IsItemHovered() && IsMouseClicked(2)) *p_open = false.
    ImGuiTabItemFlags_NoPushId                      = 1 << 3,   // Don't call PushID(tab->ID)/PopID() on BeginTabItem()/EndTabItem()
    ImGuiTabItemFlags_NoTooltip                     = 1 << 4,   // Disable tooltip for the given tab
    ImGuiTabItemFlags_NoReorder                     = 1 << 5,   // Disable reordering this tab or having another tab cross over this tab
    ImGuiTabItemFlags_Leading                       = 1 << 6,   // Enforce the tab position to the left of the tab bar (after the tab list popup button)
    ImGuiTabItemFlags_Trailing                      = 1 << 7,   // Enforce the tab position to the right of the tab bar (before the scrolling buttons)
};


// Flags for TableSetupColumn()
enum ImGuiTableColumnFlags_
{
    // Input configuration flags
    ImGuiTableColumnFlags_None                  = 0,
    ImGuiTableColumnFlags_Disabled              = 1 << 0,   // Overriding/master disable flag: hide column, won't show in context menu (unlike calling TableSetColumnEnabled() which manipulates the user accessible state)
    ImGuiTableColumnFlags_DefaultHide           = 1 << 1,   // Default as a hidden/disabled column.
    ImGuiTableColumnFlags_DefaultSort           = 1 << 2,   // Default as a sorting column.
    ImGuiTableColumnFlags_WidthStretch          = 1 << 3,   // Column will stretch. Preferable with horizontal scrolling disabled (default if table sizing policy is _SizingStretchSame or _SizingStretchProp).
    ImGuiTableColumnFlags_WidthFixed            = 1 << 4,   // Column will not stretch. Preferable with horizontal scrolling enabled (default if table sizing policy is _SizingFixedFit and table is resizable).
    ImGuiTableColumnFlags_NoResize              = 1 << 5,   // Disable manual resizing.
    ImGuiTableColumnFlags_NoReorder             = 1 << 6,   // Disable manual reordering this column, this will also prevent other columns from crossing over this column.
    ImGuiTableColumnFlags_NoHide                = 1 << 7,   // Disable ability to hide/disable this column.
    ImGuiTableColumnFlags_NoClip                = 1 << 8,   // Disable clipping for this column (all NoClip columns will render in a same draw command).
    ImGuiTableColumnFlags_NoSort                = 1 << 9,   // Disable ability to sort on this field (even if ImGuiTableFlags_Sortable is set on the table).
    ImGuiTableColumnFlags_NoSortAscending       = 1 << 10,  // Disable ability to sort in the ascending direction.
    ImGuiTableColumnFlags_NoSortDescending      = 1 << 11,  // Disable ability to sort in the descending direction.
    ImGuiTableColumnFlags_NoHeaderLabel         = 1 << 12,  // TableHeadersRow() will not submit label for this column. Convenient for some small columns. Name will still appear in context menu.
    ImGuiTableColumnFlags_NoHeaderWidth         = 1 << 13,  // Disable header text width contribution to automatic column width.
    ImGuiTableColumnFlags_PreferSortAscending   = 1 << 14,  // Make the initial sort direction Ascending when first sorting on this column (default).
    ImGuiTableColumnFlags_PreferSortDescending  = 1 << 15,  // Make the initial sort direction Descending when first sorting on this column.
    ImGuiTableColumnFlags_IndentEnable          = 1 << 16,  // Use current Indent value when entering cell (default for column 0).
    ImGuiTableColumnFlags_IndentDisable         = 1 << 17,  // Ignore current Indent value when entering cell (default for columns > 0). Indentation changes _within_ the cell will still be honored.

    // Output status flags, read-only via TableGetColumnFlags()
    ImGuiTableColumnFlags_IsEnabled             = 1 << 24,  // Status: is enabled == not hidden by user/api (referred to as "Hide" in _DefaultHide and _NoHide) flags.
    ImGuiTableColumnFlags_IsVisible             = 1 << 25,  // Status: is visible == is enabled AND not clipped by scrolling.
    ImGuiTableColumnFlags_IsSorted              = 1 << 26,  // Status: is currently part of the sort specs
    ImGuiTableColumnFlags_IsHovered             = 1 << 27,  // Status: is hovered by mouse

    // [Internal] Combinations and masks
    ImGuiTableColumnFlags_WidthMask_            = ImGuiTableColumnFlags_WidthStretch | ImGuiTableColumnFlags_WidthFixed,
    ImGuiTableColumnFlags_IndentMask_           = ImGuiTableColumnFlags_IndentEnable | ImGuiTableColumnFlags_IndentDisable,
    ImGuiTableColumnFlags_StatusMask_           = ImGuiTableColumnFlags_IsEnabled | ImGuiTableColumnFlags_IsVisible | ImGuiTableColumnFlags_IsSorted | ImGuiTableColumnFlags_IsHovered,
    ImGuiTableColumnFlags_NoDirectResize_       = 1 << 30,  // [Internal] Disable user resizing this column directly (it may however we resized indirectly from its left edge)

    // Obsolete names (will be removed soon)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    //ImGuiTableColumnFlags_WidthAuto           = ImGuiTableColumnFlags_WidthFixed | ImGuiTableColumnFlags_NoResize, // Column will not stretch and keep resizing based on submitted contents.
// #endif
};



// Enum for TableSetBgColor()
// Background colors are rendering in 3 layers:
//  - Layer 0: draw with RowBg0 color if set, otherwise draw with ColumnBg0 if set.
//  - Layer 1: draw with RowBg1 color if set, otherwise draw with ColumnBg1 if set.
//  - Layer 2: draw with CellBg color if set.
// The purpose of the two row/columns layers is to let you decide if a background color changes should override or blend with the existing color.
// When using ImGuiTableFlags_RowBg on the table, each row has the RowBg0 color automatically set for odd/even rows.
// If you set the color of RowBg0 target, your color will override the existing RowBg0 color.
// If you set the color of RowBg1 or ColumnBg1 target, your color will blend over the RowBg0 color.
enum ImGuiTableBgTarget_
{
    ImGuiTableBgTarget_None                     = 0,
    ImGuiTableBgTarget_RowBg0                   = 1,        // Set row background color 0 (generally used for background, automatically set when ImGuiTableFlags_RowBg is used)
    ImGuiTableBgTarget_RowBg1                   = 2,        // Set row background color 1 (generally used for selection marking)
    ImGuiTableBgTarget_CellBg                   = 3,        // Set cell background color (top-most color)
};

// Flags for IsWindowFocused()
enum ImGuiFocusedFlags_
{
    ImGuiFocusedFlags_None                          = 0,
    ImGuiFocusedFlags_ChildWindows                  = 1 << 0,   // Return true if any children of the window is focused
    ImGuiFocusedFlags_RootWindow                    = 1 << 1,   // Test from root window (top most parent of the current hierarchy)
    ImGuiFocusedFlags_AnyWindow                     = 1 << 2,   // Return true if any window is focused. Important: If you are trying to tell how to dispatch your low-level inputs, do NOT use this. Use 'io.WantCaptureMouse' instead! Please read the FAQ!
    ImGuiFocusedFlags_NoPopupHierarchy              = 1 << 3,   // Do not consider popup hierarchy (do not treat popup emitter as parent of popup) (when used with _ChildWindows or _RootWindow)
    ImGuiFocusedFlags_DockHierarchy                 = 1 << 4,   // Consider docking hierarchy (treat dockspace host as parent of docked window) (when used with _ChildWindows or _RootWindow)
    ImGuiFocusedFlags_RootAndChildWindows           = ImGuiFocusedFlags_RootWindow | ImGuiFocusedFlags_ChildWindows,
};

// Flags for IsItemHovered(), IsWindowHovered()
// Note: if you are trying to check whether your mouse should be dispatched to Dear ImGui or to your app, you should use 'io.WantCaptureMouse' instead! Please read the FAQ!
// Note: windows with the ImGuiWindowFlags_NoInputs flag are ignored by IsWindowHovered() calls.


// Flags for DockSpace(), shared/inherited by child nodes.
// (Some flags can be applied to individual nodes directly)
// FIXME-DOCK: Also see ImGuiDockNodeFlagsPrivate_ which may involve using the WIP and internal DockBuilder api.
enum ImGuiDockNodeFlags_
{
    ImGuiDockNodeFlags_None                         = 0,
    ImGuiDockNodeFlags_KeepAliveOnly                = 1 << 0,   // Shared       // Don't display the dockspace node but keep it alive. Windows docked into this dockspace node won't be undocked.
    //ImGuiDockNodeFlags_NoCentralNode              = 1 << 1,   // Shared       // Disable Central Node (the node which can stay empty)
    ImGuiDockNodeFlags_NoDockingInCentralNode       = 1 << 2,   // Shared       // Disable docking inside the Central Node, which will be always kept empty.
    ImGuiDockNodeFlags_PassthruCentralNode          = 1 << 3,   // Shared       // Enable passthru dockspace: 1) DockSpace() will render a ImGuiCol_WindowBg background covering everything excepted the Central Node when empty. Meaning the host window should probably use SetNextWindowBgAlpha(0.0) prior to Begin() when using this. 2) When Central Node is empty: let inputs pass-through + won't display a DockingEmptyBg background. See demo for details.
    ImGuiDockNodeFlags_NoSplit                      = 1 << 4,   // Shared/Local // Disable splitting the node into smaller nodes. Useful e.g. when embedding dockspaces into a main root one (the root one may have splitting disabled to reduce confusion). Note: when turned off, existing splits will be preserved.
    ImGuiDockNodeFlags_NoResize                     = 1 << 5,   // Shared/Local // Disable resizing node using the splitter/separators. Useful with programmatically setup dockspaces.
    ImGuiDockNodeFlags_AutoHideTabBar               = 1 << 6,   // Shared/Local // Tab bar will automatically hide when there is a single window in the dock node.
};












// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
// OBSOLETED in 1.88 (from July 2022): ImGuiNavInput and io.NavInputs[].
// Official backends between 1.60 and 1.86: will keep working and feed gamepad inputs as long as IMGUI_DISABLE_OBSOLETE_KEYIO is not set.
// Custom backends: feed gamepad inputs via io.AddKeyEvent() and ImGuiKey_GamepadXXX enums.
enum ImGuiNavInput
{
    ImGuiNavInput_Activate, ImGuiNavInput_Cancel, ImGuiNavInput_Input, ImGuiNavInput_Menu, ImGuiNavInput_DpadLeft, ImGuiNavInput_DpadRight, ImGuiNavInput_DpadUp, ImGuiNavInput_DpadDown,
    ImGuiNavInput_LStickLeft, ImGuiNavInput_LStickRight, ImGuiNavInput_LStickUp, ImGuiNavInput_LStickDown, ImGuiNavInput_FocusPrev, ImGuiNavInput_FocusNext, ImGuiNavInput_TweakSlow, ImGuiNavInput_TweakFast,
    ImGuiNavInput_COUNT,
};
// #endif





// Enumeration for PushStyleVar() / PopStyleVar() to temporarily modify the ImGuiStyle structure.
// - The enum only refers to fields of ImGuiStyle which makes sense to be pushed/popped inside UI code.
//   During initialization or between frames, feel free to just poke into ImGuiStyle directly.
// - Tip: Use your programming IDE navigation facilities on the names in the _second column_ below to find the actual members and their description.
//   In Visual Studio IDE: CTRL+comma ("Edit.GoToAll") can follow symbols in comments, whereas CTRL+F12 ("Edit.GoToImplementation") cannot.
//   With Visual Assist installed: ALT+G ("VAssistX.GoToImplementation") can also follow symbols in comments.
// - When changing this enum, you need to update the associated internal table GStyleVarInfo[] accordingly. This is where we link enum values to members offset/type.
enum ImGuiStyleVar_
{
    // Enum name --------------------- // Member in ImGuiStyle structure (see ImGuiStyle for descriptions)
    ImGuiStyleVar_Alpha,               // float     Alpha
    ImGuiStyleVar_DisabledAlpha,       // float     DisabledAlpha
    ImGuiStyleVar_WindowPadding,       // ImVec2    WindowPadding
    ImGuiStyleVar_WindowRounding,      // float     WindowRounding
    ImGuiStyleVar_WindowBorderSize,    // float     WindowBorderSize
    ImGuiStyleVar_WindowMinSize,       // ImVec2    WindowMinSize
    ImGuiStyleVar_WindowTitleAlign,    // ImVec2    WindowTitleAlign
    ImGuiStyleVar_ChildRounding,       // float     ChildRounding
    ImGuiStyleVar_ChildBorderSize,     // float     ChildBorderSize
    ImGuiStyleVar_PopupRounding,       // float     PopupRounding
    ImGuiStyleVar_PopupBorderSize,     // float     PopupBorderSize
    ImGuiStyleVar_FramePadding,        // ImVec2    FramePadding
    ImGuiStyleVar_FrameRounding,       // float     FrameRounding
    ImGuiStyleVar_FrameBorderSize,     // float     FrameBorderSize
    ImGuiStyleVar_ItemSpacing,         // ImVec2    ItemSpacing
    ImGuiStyleVar_ItemInnerSpacing,    // ImVec2    ItemInnerSpacing
    ImGuiStyleVar_IndentSpacing,       // float     IndentSpacing
    ImGuiStyleVar_CellPadding,         // ImVec2    CellPadding
    ImGuiStyleVar_ScrollbarSize,       // float     ScrollbarSize
    ImGuiStyleVar_ScrollbarRounding,   // float     ScrollbarRounding
    ImGuiStyleVar_GrabMinSize,         // float     GrabMinSize
    ImGuiStyleVar_GrabRounding,        // float     GrabRounding
    ImGuiStyleVar_TabRounding,         // float     TabRounding
    ImGuiStyleVar_ButtonTextAlign,     // ImVec2    ButtonTextAlign
    ImGuiStyleVar_SelectableTextAlign, // ImVec2    SelectableTextAlign
    ImGuiStyleVar_COUNT
};

// Flags for InvisibleButton() [extended in imgui_internal.h]
enum ImGuiButtonFlags_
{
    ImGuiButtonFlags_None                   = 0,
    ImGuiButtonFlags_MouseButtonLeft        = 1 << 0,   // React on left mouse button (default)
    ImGuiButtonFlags_MouseButtonRight       = 1 << 1,   // React on right mouse button
    ImGuiButtonFlags_MouseButtonMiddle      = 1 << 2,   // React on center mouse button

    // [Internal]
    ImGuiButtonFlags_MouseButtonMask_       = ImGuiButtonFlags_MouseButtonLeft | ImGuiButtonFlags_MouseButtonRight | ImGuiButtonFlags_MouseButtonMiddle,
    ImGuiButtonFlags_MouseButtonDefault_    = ImGuiButtonFlags_MouseButtonLeft,
};


// Flags for DragFloat(), DragInt(), SliderFloat(), SliderInt() etc.
// We use the same sets of flags for DragXXX() and SliderXXX() functions as the features are the same and it makes it easier to swap them.
enum ImGuiSliderFlags_
{
    ImGuiSliderFlags_None                   = 0,
    ImGuiSliderFlags_AlwaysClamp            = 1 << 4,       // Clamp value to min/max bounds when input manually with CTRL+Click. By default CTRL+Click allows going out of bounds.
    ImGuiSliderFlags_Logarithmic            = 1 << 5,       // Make the widget logarithmic (linear otherwise). Consider using ImGuiSliderFlags_NoRoundToFormat with this if using a format-string with small amount of digits.
    ImGuiSliderFlags_NoRoundToFormat        = 1 << 6,       // Disable rounding underlying value to match precision of the display format string (e.g. %.3f values are rounded to those 3 digits)
    ImGuiSliderFlags_NoInput                = 1 << 7,       // Disable CTRL+Click or Enter key allowing to input text directly into the widget
    ImGuiSliderFlags_InvalidMask_           = 0x7000000f32,   // [Internal] We treat using those bits as being potentially a 'float power' argument from the previous API that has got miscast to this enum, and will trigger an assert if needed.

    // Obsolete names (will be removed)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    ImGuiSliderFlags_ClampOnInput = ImGuiSliderFlags_AlwaysClamp, // [renamed in 1.79]
// #endif
};






//-----------------------------------------------------------------------------
// [SECTION] Helpers: Memory allocations macros, ImVector<>
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// IM_MALLOC(), IM_FREE(), IM_NEW(), IM_PLACEMENT_NEW(), IM_DELETE()
// We call C++ constructor on own allocated memory via the placement "new(ptr) Type()" syntax.
// Defining a custom placement new() with a custom parameter allows us to bypass including <new> which on some platforms complains when user has disabled exceptions.
//-----------------------------------------------------------------------------

struct ImNewWrapper {};
inline operator: *mut c_void new(size_t, ImNewWrapper, ptr: *mut c_void) { return ptr; }
inline c_void  operator delete(*mut c_void, ImNewWrapper, *mut c_void)   {} // This is only required so we can use the symmetrical new()
// #define IM_ALLOC(_SIZE)                     MemAlloc(_SIZE)
// #define IM_FREE(_PTR)                       MemFree(_PTR)
// #define IM_PLACEMENT_NEW(_PTR)              new(ImNewWrapper(), _PTR)
// #define IM_NEW(_TYPE)                       new(ImNewWrapper(), MemAlloc(sizeof(_TYPE))) _TYPE
template<typename T> c_void IM_DELETE(T* p)   { if (p) { p->!T(); MemFree(p); } }

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

IM_MSVC_RUNTIME_CHECKS_OFF
template<typename T>
struct Vec
{
    c_int                 Size;
    c_int                 Capacity;
    T*                  Data;

    // Provide standard typedefs but we don't use them ourselves.
    typedef T                   value_type;
    typedef value_type*         iterator;
    typedef *const value_type   const_iterator;

    // Constructors, destructor
    inline Vec()                                       { Size = Capacity = 0; Data= null_mut(); }
    inline Vec(const Vec<T>& src)                 { Size = Capacity = 0; Data= null_mut(); operator=(src); }
    inline Vec<T>& operator=(const Vec<T>& src)   { clear(); resize(src.Size); if (src.Data) memcpy(Data, src.Data, Size * sizeof(T)); return *this; }
    inline !Vec()                                      { if (Data) IM_FREE(Data); } // Important: does not destruct anything

    inline c_void         clear()                             { if (Data) { Size = Capacity = 0; IM_FREE(Data); Data= null_mut(); } }  // Important: does not destruct anything
    inline c_void         clear_delete()                      { for (let n: c_int = 0; n < Size; n++) IM_DELETE(Data[n]); clear(); }     // Important: never called automatically! always explicit.
    inline c_void         clear_destruct()                    { for (let n: c_int = 0; n < Size; n++) Data[n].!T(); clear(); }           // Important: never called automatically! always explicit.

    inline bool         empty() const                       { return Size == 0; }
    inline c_int          size() const                        { return Size; }
    inline c_int          size_in_bytes() const               { return Size * sizeof(T); }
    inline c_int          max_size() const                    { return 0x7FFFFFFF / sizeof(T); }
    inline c_int          capacity() const                    { return Capacity; }
    inline T&           operator[](i: c_int)                   { IM_ASSERT(i >= 0 && i < Size); return Data[i]; }
    inline const T&     operator[](i: c_int) const             { IM_ASSERT(i >= 0 && i < Size); return Data[i]; }

    inline T*           begin()                             { return Data; }
    inline *const T     begin() const                       { return Data; }
    inline T*           end()                               { return Data + Size; }
    inline *const T     end() const                         { return Data + Size; }
    inline T&           front()                             { IM_ASSERT(Size > 0); return Data[0]; }
    inline const T&     front() const                       { IM_ASSERT(Size > 0); return Data[0]; }
    inline T&           back()                              { IM_ASSERT(Size > 0); return Data[Size - 1]; }
    inline const T&     back() const                        { IM_ASSERT(Size > 0); return Data[Size - 1]; }
    inline c_void         swap(Vec<T>& rhs)              { let rhs_size: c_int = rhs.Size; rhs.Size = Size; Size = rhs_size; let rhs_cap: c_int = rhs.Capacity; rhs.Capacity = Capacity; Capacity = rhs_cap; T* rhs_data = rhs.Data; rhs.Data = Data; Data = rhs_data; }

    inline c_int          _grow_capacity(sz: c_int) const        { let new_capacity: c_int = Capacity ? (Capacity + Capacity / 2) : 8; return new_capacity > sz ? new_capacity : sz; }
    inline c_void         resize(new_size: c_int)                { if (new_size > Capacity) reserve(_grow_capacity(new_size)); Size = new_size; }
    inline c_void         resize(new_size: c_int, const T& v)    { if (new_size > Capacity) reserve(_grow_capacity(new_size)); if (new_size > Size) for (let n: c_int = Size; n < new_size; n++) memcpy(&Data[n], &v, sizeof(v)); Size = new_size; }
    inline c_void         shrink(new_size: c_int)                { IM_ASSERT(new_size <= Size); Size = new_size; } // Resize a vector to a smaller size, guaranteed not to cause a reallocation
    inline c_void         reserve(new_capacity: c_int)           { if (new_capacity <= Capacity) return; T* new_data = (T*)IM_ALLOC(new_capacity * sizeof(T)); if (Data) { memcpy(new_data, Data, Size * sizeof(T)); IM_FREE(Data); } Data = new_data; Capacity = new_capacity; }
    inline c_void         reserve_discard(new_capacity: c_int)   { if (new_capacity <= Capacity) return; if (Data) IM_FREE(Data); Data = (T*)IM_ALLOC(new_capacity * sizeof(T)); Capacity = new_capacity; }

    // NB: It is illegal to call push_back/push_front/insert with a reference pointing inside the ImVector data itself! e.g. v.push(v[10]) is forbidden.
    inline c_void         push_back(const T& v)               { if (Size == Capacity) reserve(_grow_capacity(Size + 1)); memcpy(&Data[Size], &v, sizeof(v)); Size+= 1; }
    inline c_void         pop_back()                          { IM_ASSERT(Size > 0); Size-= 1; }
    inline c_void         push_front(const T& v)              { if (Size == 0) push_back(v); else insert(Data, v); }
    inline T*           erase(*const T it)                  { IM_ASSERT(it >= Data && it < Data + Size); const ptrdiff_t off = it - Data; memmove(Data + off, Data + off + 1, (Size - off - 1) * sizeof(T)); Size-= 1; return Data + off; }
    inline T*           erase(*const T it, *const T it_last){ IM_ASSERT(it >= Data && it < Data + Size && it_last >= it && it_last <= Data + Size); const ptrdiff_t count = it_last - it; const ptrdiff_t off = it - Data; memmove(Data + off, Data + off + count, (Size - off - count) * sizeof(T)); Size -= count; return Data + off; }
    inline T*           erase_unsorted(*const T it)         { IM_ASSERT(it >= Data && it < Data + Size);  const ptrdiff_t off = it - Data; if (it < Data + Size - 1) memcpy(Data + off, Data + Size - 1, sizeof(T)); Size-= 1; return Data + off; }
    inline T*           insert(*const T it, const T& v)     { IM_ASSERT(it >= Data && it <= Data + Size); const ptrdiff_t off = it - Data; if (Size == Capacity) reserve(_grow_capacity(Size + 1)); if (off < Size) memmove(Data + off + 1, Data + off, (Size - of0f32) * sizeof(T)); memcpy(&Data[off], &v, sizeof(v)); Size+= 1; return Data + off; }
    inline bool         contains(const T& v) const          { *const T data = Data;  *const T data_end = Data + Size; while (data < data_end) if (*data++ == v) return true; return false; }
    inline T*           find(const T& v)                    { T* data = Data;  *const T data_end = Data + Size; while (data < data_end) if (*data == v) break; else ++data; return data; }
    inline *const T     find(const T& v) const              { *const T data = Data;  *const T data_end = Data + Size; while (data < data_end) if (*data == v) break; else ++data; return data; }
    inline bool         find_erase(const T& v)              { *const T it = find(v); if (it < Data + Size) { erase(it); return true; } return false; }
    inline bool         find_erase_unsorted(const T& v)     { *const T it = find(v); if (it < Data + Size) { erase_unsorted(it); return true; } return false; }
    inline c_int          index_from_ptr(*const T it) const   { IM_ASSERT(it >= Data && it < Data + Size); const ptrdiff_t off = it - Data; return off; }
};
IM_MSVC_RUNTIME_CHECKS_RESTORE

//-----------------------------------------------------------------------------
// [SECTION] ImGuiStyle
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] ImGuiIO
//-----------------------------------------------------------------------------
// Communicate most settings and inputs/outputs to Dear ImGui using this structure.
// Access via GetIO(). Read 'Programmer guide' section in .cpp file for general usage.
//-----------------------------------------------------------------------------




//-----------------------------------------------------------------------------
// [SECTION] Misc data structures
//-----------------------------------------------------------------------------

// Shared state of InputText(), passed as an argument to your callback when a ImGuiInputTextFlags_Callback* flag is used.
// The callback function should return 0 by default.
// Callbacks (follow a flag name and see comments in ImGuiInputTextFlags_ declarations for more details)
// - ImGuiInputTextFlags_CallbackEdit:        Callback on buffer edit (note that InputText() already returns true on edit, the callback is useful mainly to manipulate the underlying buffer while focus is active)
// - ImGuiInputTextFlags_CallbackAlways:      Callback on each iteration
// - ImGuiInputTextFlags_CallbackCompletion:  Callback on pressing TAB
// - ImGuiInputTextFlags_CallbackHistory:     Callback on pressing Up/Down arrows
// - ImGuiInputTextFlags_CallbackCharFilter:  Callback on character inputs to replace or discard them. Modify 'EventChar' to replace or discard, or return 1 in callback to discard.
// - ImGuiInputTextFlags_CallbackResize:      Callback on buffer capacity changes request (beyond 'buf_size' parameter value), allowing the string to grow.
struct ImGuiInputTextCallbackData
{
    ImGuiInputTextFlags EventFlag;      // One ImGuiInputTextFlags_Callback*    // Read-only
    ImGuiInputTextFlags Flags;          // What user passed to InputText()      // Read-only
    *mut c_void               UserData;       // What user passed to InputText()      // Read-only

    // Arguments for the different callback events
    // - To modify the text buffer in a callback, prefer using the InsertChars() / DeleteChars() function. InsertChars() will take care of calling the resize callback if necessary.
    // - If you know your edits are not going to resize the underlying buffer allocation, you may modify the contents of 'Buf[]' directly. You need to update 'BufTextLen' accordingly (0 <= BufTextLen < BufSize) and set 'BufDirty'' to true so InputText can update its internal state.
    ImWchar             EventChar;      // Character input                      // Read-write   // [CharFilter] Replace character with another one, or set to zero to drop. return 1 is equivalent to setting EventChar=0;
    ImGuiKey            EventKey;       // Key pressed (Up/Down/TAB)            // Read-only    // [Completion,History]
    char*               Buf;            // Text buffer                          // Read-write   // [Resize] Can replace pointer / [Completion,History,Always] Only write to pointed data, don't replace the actual pointer!
    c_int                 BufTextLen;     // Text length (in bytes)               // Read-write   // [Resize,Completion,History,Always] Exclude zero-terminator storage. In C land: == strlen(some_text), in C++ land: string.length()
    c_int                 BufSize;        // Buffer size (in bytes) = capacity+1  // Read-only    // [Resize,Completion,History,Always] Include zero-terminator storage. In C land == ARRAYSIZE(my_char_array), in C++ land: string.capacity()+1
    bool                BufDirty;       // Set if you modify Buf/BufTextLen!    // Write        // [Completion,History,Always]
    c_int                 CursorPos;      //                                      // Read-write   // [Completion,History,Always]
    c_int                 SelectionStart; //                                      // Read-write   // [Completion,History,Always] == to SelectionEnd when no selection)
    c_int                 SelectionEnd;   //                                      // Read-write   // [Completion,History,Always]

    // Helper functions for text manipulation.
    // Use those function to benefit from the CallbackResize behaviors. Calling those function reset the selection.
     ImGuiInputTextCallbackData();
     c_void      DeleteChars(pos: c_int, bytes_count: c_int);
     c_void      InsertChars(pos: c_int, text: *const c_char, text_end: *const c_char = null_mut());
    c_void                SelectAll()             { SelectionStart = 0; SelectionEnd = BufTextLen; }
    c_void                ClearSelection()        { SelectionStart = SelectionEnd = BufTextLen; }
    bool                HasSelection() const    { return SelectionStart != SelectionEnd; }
};





// Sorting specifications for a table (often handling sort specs for a single column, occasionally more)
// Obtained by calling TableGetSortSpecs().
// When 'SpecsDirty == true' you can sort your data. It will be true with sorting specs have changed since last call, or the first time.
// Make sure to set 'SpecsDirty = false' after sorting, else you may wastefully sort your data every frame!
struct ImGuiTableSortSpecs
{
    *const ImGuiTableColumnSortSpecs Specs;     // Pointer to sort spec array.
    c_int                         SpecsCount;     // Sort spec count. Most often 1. May be > 1 when ImGuiTableFlags_SortMulti is enabled. May be == 0 when ImGuiTableFlags_SortTristate is enabled.
    bool                        SpecsDirty;     // Set to true when specs have changed since last time! Use this to sort again, then clear the flag.

    ImGuiTableSortSpecs()       { memset(this, 0, sizeof(*this)); }
};

//-----------------------------------------------------------------------------
// [SECTION] Helpers (ImGuiOnceUponAFrame, ImGuiTextFilter, ImGuiTextBuffer, ImGuiStorage, ImGuiListClipper, ImColor)
//-----------------------------------------------------------------------------

// Helper: Unicode defines
// #define IM_UNICODE_CODEPOINT_INVALID 0xFFFD     // Invalid Unicode code point (standard value).
// #ifdef IMGUI_USE_WCHAR32
// #define IM_UNICODE_CODEPOINT_MAX     0x10FFFF   // Maximum Unicode code point supported by this build.
// #else
// #define IM_UNICODE_CODEPOINT_MAX     0xFFFF     // Maximum Unicode code point supported by this build.
// #endif

// Helper: Execute a block of code at maximum once a frame. Convenient if you want to quickly create an UI within deep-nested code that runs multiple times every frame.
// Usage: static ImGuiOnceUponAFrame oaf; if (oa0f32) Text("This will be called only once per frame");
struct ImGuiOnceUponAFrame
{
    ImGuiOnceUponAFrame() { RefFrame = -1; }
    mutable let mut RefFrame: c_int = 0;
    operator bool() const { let current_frame: c_int = GetFrameCount(); if (RefFrame == current_frame) return false; RefFrame = current_frame; return true; }
};









// Helper: ImColor() implicitly converts colors to either ImU32 (packed 4x1 byte) or ImVec4 (4x1 float)
// Prefer using IM_COL32() macros if you want a guaranteed compile-time ImU32 for usage with ImDrawList API.
// **Avoid storing ImColor! Store either of: u32 ImVec4. This is not a full-featured color class. MAY OBSOLETE.
// **None of the ImGui API are using ImColor directly but you can use it as a convenience to pass colors in either ImU32 or ImVec4 formats. Explicitly cast to ImU32 or ImVec4 if needed.
struct ImColor
{
    ImVec4          Value;

    constexpr ImColor()                                             { }
    constexpr ImColor(r: c_float,g: c_float,b: c_float, let a: c_float =  1.0)    : Value(r, g, b, a) { }
    constexpr ImColor(const ImVec4& col)                            : Value(col) {}
    ImColor(r: c_int, g: c_int, b: c_int, let a: c_int = 255)                       { let sc: c_float =  1.0 / 255f32; Value.x = r * sc; Value.y = g * sc; Value.z = b * sc; Value.w = a * sc; }
    ImColor(rgba: u32)                                             { let sc: c_float =  1.0 / 255f32; Value.x = ((rgba >> IM_COL32_R_SHIFT) & 0xF0f32) * sc; Value.y = ((rgba >> IM_COL32_G_SHIFT) & 0xF0f32) * sc; Value.z = ((rgba >> IM_COL32_B_SHIFT) & 0xF0f32) * sc; Value.w = ((rgba >> IM_COL32_A_SHIFT) & 0xF0f32) * sc; }
    inline operator u32() const                                   { return ColorConvertFloat4ToU32(Value); }
    inline operator ImVec4() const                                  { return Value; }

    // FIXME-OBSOLETE: May need to obsolete/cleanup those helpers.
    inline c_void    SetHSV(h: c_float,s: c_float,v: c_float, let a: c_float =  1.0){ ColorConvertHSVtoRGB(h, s, v, Value.x, Value.y, Value.z); Value.w = a; }
    static ImColor HSV(h: c_float,s: c_float,v: c_float, let a: c_float =  1.0)   {r: c_float, g, b; ColorConvertHSVtoRGB(h, s, v, r, g, b); return ImColor(r, g, b, a); }
};

//-----------------------------------------------------------------------------
// [SECTION] Drawing API (ImDrawCmd, ImDrawIdx, ImDrawVert, ImDrawChannel, ImDrawListSplitter, ImDrawListFlags, ImDrawList, ImDrawData)
// Hold a series of drawing commands. The user provides a renderer for ImDrawData which essentially contains an array of ImDrawList.
//-----------------------------------------------------------------------------

// The maximum line width to bake anti-aliased textures for. Build atlas with ImFontAtlasFlags_NoBakedLines to disable baking.
// #ifndef IM_DRAWLIST_TEX_LINES_WIDTH_MAX
// #define IM_DRAWLIST_TEX_LINES_WIDTH_MAX     (63)
// #endif



// Special Draw callback value to request renderer backend to reset the graphics/render state.
// The renderer backend needs to handle this special value, otherwise it will crash trying to call a function at this address.
// This is useful for example if you submitted callbacks which you know have altered the render state and you want it to be restored.
// It is not done by default because they are many perfectly useful way of altering render state for imgui contents (e.g. changing shader/blending settings before an Image call).
// #define ImDrawCallback_ResetRenderState     (ImDrawCallback)(-1)




// #else
// You can override the vertex format layout by defining IMGUI_OVERRIDE_DRAWVERT_STRUCT_LAYOUT in imconfig.h
// The code expect pos: ImVec2 (8 bytes), uv: ImVec2 (8 bytes), ImU32 col (4 bytes), but you can re-order them or add other fields as needed to simplify integration in your engine.
// The type has to be described within the macro (you can either declare the struct or use a typede0f32). This is because ImVec2/ImU32 are likely not declared a the time you'd want to set your type up.
// NOTE: IMGUI DOESN'T CLEAR THE STRUCTURE AND DOESN'T CALL A CONSTRUCTOR SO ANY CUSTOM FIELD WILL BE UNINITIALIZED. IF YOU ADD EXTRA FIELDS (SUCH AS A 'Z' COORDINATES) YOU WILL NEED TO CLEAR THEM DURING RENDER OR TO IGNORE THEM.
IMGUI_OVERRIDE_DRAWVERT_STRUCT_LAYOUT;
// #endif











//-----------------------------------------------------------------------------
// [SECTION] Font API (ImFontConfig, ImFontGlyph, ImFontAtlasFlags, ImFontAtlas, ImFontGlyphRangesBuilder, ImFont)
//-








//-----------------------------------------------------------------------------
// [SECTION] Viewports
//-----------------------------------------------------------------------------

// Flags stored in ImGuiViewport::Flags, giving indications to the platform backends.
enum ImGuiViewportFlags_
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
    ImGuiViewportFlags_CanHostOtherWindows      = 1 << 12,  // Main viewport: can host multiple imgui windows (secondary viewports are associated to a single window).
};

// };

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
// - So e.g. SetNextWindowPos(ImVec2::new(0,0)) will position a window relative to your primary monitor!
// - If you want to position windows relative to your main application viewport, use GetMainViewport()->Pos as a base position.
//
// Steps to use multi-viewports in your application, when using a default backend from the examples/ folder:
// - Application:  Enable feature with 'io.ConfigFlags |= ImGuiConfigFlags_ViewportsEnable'.
// - Backend:      The backend initialization will setup all necessary ImGuiPlatformIO's functions and update monitors info every frame.
// - Application:  In your main loop, call UpdatePlatformWindows(), RenderPlatformWindowsDefault() after EndFrame() or Render().
// - Application:  Fix absolute coordinates used in SetWindowPos() or SetNextWindowPos() calls.
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
// - Application:  In your main loop, call UpdatePlatformWindows(), RenderPlatformWindowsDefault() after EndFrame() or Render().
//                 You may skip calling RenderPlatformWindowsDefault() if its API is not convenient for your needs. Read comments below.
// - Application:  Fix absolute coordinates used in SetWindowPos() or SetNextWindowPos() calls.
//
// About RenderPlatformWindowsDefault():
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

namespace ImGui
{
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
     c_int       GetKeyIndex(ImGuiKey key);  // map ImGuiKey_* values into legacy native key index. == io.KeyMap[key]
// #else
    static inline c_int   GetKeyIndex(ImGuiKey key)   { IM_ASSERT(key >= ImGuiKey_NamedKey_BEGIN && key < ImGuiKey_NamedKey_END && "ImGuiKey and native_index was merged together and native_index is disabled by IMGUI_DISABLE_OBSOLETE_KEYIO. Please switch to ImGuiKey."); return key; }
// #endif
}

// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
namespace ImGui
{
    // OBSOLETED in 1.89 (from August 2022)
     bool      ImageButton(ImTextureID user_texture_id, size: &ImVec2, uv0: &ImVec2 = ImVec2::new(0, 0), uv1: &ImVec2 = ImVec2::new(1, 1), let frame_padding: c_int = -1, const ImVec4& bg_col = ImVec4(0, 0, 0, 0), const ImVec4& tint_col = ImVec4(1, 1, 1, 1)); // Use new ImageButton() signature (explicit item id, regular FramePadding)
    // OBSOLETED in 1.88 (from May 2022)
    static inline c_void  CaptureKeyboardFromApp(let mut want_capture_keyboard: bool =  true)   { SetNextFrameWantCaptureKeyboard(want_capture_keyboard); } // Renamed as name was misleading + removed default value.
    static inline c_void  CaptureMouseFromApp(let mut want_capture_mouse: bool =  true)         { SetNextFrameWantCaptureMouse(want_capture_mouse); }       // Renamed as name was misleading + removed default value.
    // OBSOLETED in 1.86 (from November 2021)
     c_void      CalcListClipping(items_count: c_int,items_height: c_float, out_items_display_start:  *mut c_int, out_items_display_end:  *mut c_int); // Calculate coarse clipping for large list of evenly sized items. Prefer using ImGuiListClipper.
    // OBSOLETED in 1.85 (from August 2021)
    static inlineGetWindowContentRegionWidth: c_float()                               { return GetWindowContentRegionMax().x - GetWindowContentRegionMin().x; }
    // OBSOLETED in 1.81 (from February 2021)
     bool      ListBoxHeader(label: *const c_char, items_count: c_int, let height_in_items: c_int = -1); // Helper to calculate size from items_count and height_in_items
    static inline bool  ListBoxHeader(label: *const c_char, size: &ImVec2 = ImVec2::new(0, 0))         { return BeginListBox(label, size); }
    static inline c_void  ListBoxFooter() { EndListBox(); }
    // OBSOLETED in 1.79 (from August 2020)
    static inline c_void  OpenPopupContextItem(str_id: *const c_char = null_mut(), let mut mb: ImGuiMouseButton =  1)    { OpenPopupOnItemClick(str_id, mb); } // Bool return value removed. Use IsWindowAppearing() in BeginPopup() instead. Renamed in 1.77, renamed back in 1.79. Sorry!

    // Some of the older obsolete names along with their replacement (commented out so they are not reported in IDE)
    // [OBSOLETED in 1.78 (from June 2020] Old drag/sliders functions that took a 'float power > 1.0f' argument instead of ImGuiSliderFlags_Logarithmic. See github.com/ocornut/imgui/issues/3361 for details.
    //IMGUI_API bool      DragScalar(const char* label, ImGuiDataType data_type, void* p_data, float v_speed, const void* p_min, const void* p_max, const char* format, float power = 1.0)                                                            // OBSOLETED in 1.78 (from June 2020)
    //IMGUI_API bool      DragScalarN(const char* label, ImGuiDataType data_type, void* p_data, int components, float v_speed, const void* p_min, const void* p_max, const char* format, float power = 1.0);                                          // OBSOLETED in 1.78 (from June 2020)
    //IMGUI_API bool      SliderScalar(const char* label, ImGuiDataType data_type, void* p_data, const void* p_min, const void* p_max, const char* format, float power = 1.0);                                                                        // OBSOLETED in 1.78 (from June 2020)
    //IMGUI_API bool      SliderScalarN(const char* label, ImGuiDataType data_type, void* p_data, int components, const void* p_min, const void* p_max, const char* format, float power = 1.0);                                                       // OBSOLETED in 1.78 (from June 2020)
    //static inline bool  DragFloat(const char* label, float* v, float v_speed, float v_min, float v_max, const char* format, float power = 1.0)    { return DragScalar(label, ImGuiDataType_Float, v, v_speed, &v_min, &v_max, format, power); }     // OBSOLETED in 1.78 (from June 2020)
    //static inline bool  DragFloat2(const char* label, float v[2], float v_speed, float v_min, float v_max, const char* format, float power = 1.0) { return DragScalarN(label, ImGuiDataType_Float, v, 2, v_speed, &v_min, &v_max, format, power); } // OBSOLETED in 1.78 (from June 2020)
    //static inline bool  DragFloat3(const char* label, float v[3], float v_speed, float v_min, float v_max, const char* format, float power = 1.0) { return DragScalarN(label, ImGuiDataType_Float, v, 3, v_speed, &v_min, &v_max, format, power); } // OBSOLETED in 1.78 (from June 2020)
    //static inline bool  DragFloat4(const char* label, float v[4], float v_speed, float v_min, float v_max, const char* format, float power = 1.0) { return DragScalarN(label, ImGuiDataType_Float, v, 4, v_speed, &v_min, &v_max, format, power); } // OBSOLETED in 1.78 (from June 2020)
    //static inline bool  SliderFloat(const char* label, float* v, float v_min, float v_max, const char* format, float power = 1.0)                 { return SliderScalar(label, ImGuiDataType_Float, v, &v_min, &v_max, format, power); }            // OBSOLETED in 1.78 (from June 2020)
    //static inline bool  SliderFloat2(const char* label, float v[2], float v_min, float v_max, const char* format, float power = 1.0)              { return SliderScalarN(label, ImGuiDataType_Float, v, 2, &v_min, &v_max, format, power); }        // OBSOLETED in 1.78 (from June 2020)
    //static inline bool  SliderFloat3(const char* label, float v[3], float v_min, float v_max, const char* format, float power = 1.0)              { return SliderScalarN(label, ImGuiDataType_Float, v, 3, &v_min, &v_max, format, power); }        // OBSOLETED in 1.78 (from June 2020)
    //static inline bool  SliderFloat4(const char* label, float v[4], float v_min, float v_max, const char* format, float power = 1.0)              { return SliderScalarN(label, ImGuiDataType_Float, v, 4, &v_min, &v_max, format, power); }        // OBSOLETED in 1.78 (from June 2020)
    // [OBSOLETED in 1.77 and before]
    //static inline bool  BeginPopupContextWindow(const char* str_id, ImGuiMouseButton mb, over_items: bool) { return BeginPopupContextWindow(str_id, mb | (over_items ? 0 : ImGuiPopupFlags_NoOpenOverItems)); } // OBSOLETED in 1.77 (from June 2020)
    //static inline void  TreeAdvanceToLabelPos()               { SetCursorPosX(GetCursorPosX() + GetTreeNodeToLabelSpacing()); }   // OBSOLETED in 1.72 (from July 2019)
    //static inline void  SetNextTreeNodeOpen(open: bool, ImGuiCond cond = 0) { SetNextItemOpen(open, cond); }                       // OBSOLETED in 1.71 (from June 2019)
    //static inline float GetContentRegionAvailWidth()          { return GetContentRegionAvail().x; }                               // OBSOLETED in 1.70 (from May 2019)
    //static inline ImDrawList* GetOverlayDrawList()            { return GetForegroundDrawList(); }                                 // OBSOLETED in 1.69 (from Mar 2019)
    //static inline void  SetScrollHere(float ratio = 0.5f32)     { SetScrollHereY(ratio); }                                          // OBSOLETED in 1.66 (from Nov 2018)
    //static inline bool  IsItemDeactivatedAfterChange()        { return IsItemDeactivatedAfterEdit(); }                            // OBSOLETED in 1.63 (from Aug 2018)
    //static inline bool  IsAnyWindowFocused()                  { return IsWindowFocused(ImGuiFocusedFlags_AnyWindow); }            // OBSOLETED in 1.60 (from Apr 2018)
    //static inline bool  IsAnyWindowHovered()                  { return IsWindowHovered(ImGuiHoveredFlags_AnyWindow); }            // OBSOLETED in 1.60 (between Dec 2017 and Apr 2018)
    //static inline void  ShowTestWindow()                      { return ShowDemoWindow(); }                                        // OBSOLETED in 1.53 (between Oct 2017 and Dec 2017)
    //static inline bool  IsRootWindowFocused()                 { return IsWindowFocused(ImGuiFocusedFlags_RootWindow); }           // OBSOLETED in 1.53 (between Oct 2017 and Dec 2017)
    //static inline bool  IsRootWindowOrAnyChildFocused()       { return IsWindowFocused(ImGuiFocusedFlags_RootAndChildWindows); }  // OBSOLETED in 1.53 (between Oct 2017 and Dec 2017)
    //static inline void  SetNextWindowContentWidth(float w)    { SetNextWindowContentSize(ImVec2::new(w, 0.0)); }                      // OBSOLETED in 1.53 (between Oct 2017 and Dec 2017)
    //static inline float GetItemsLineHeightWithSpacing()       { return GetFrameHeightWithSpacing(); }                             // OBSOLETED in 1.53 (between Oct 2017 and Dec 2017)
    //IMGUI_API bool      Begin(char* name,p_open: *mut bool, size_first_use: ImVec2, float bg_alpha = -1.0, ImGuiWindowFlags flags=0); // OBSOLETED in 1.52 (between Aug 2017 and Oct 2017): Equivalent of using SetNextWindowSize(size, ImGuiCond_FirstUseEver) and SetNextWindowBgAlpha().
    //static inline bool  IsRootWindowOrAnyChildHovered()       { return IsWindowHovered(ImGuiHoveredFlags_RootAndChildWindows); }  // OBSOLETED in 1.52 (between Aug 2017 and Oct 2017)
    //static inline void  AlignFirstTextHeightToWidgets()       { AlignTextToFramePadding(); }                                      // OBSOLETED in 1.52 (between Aug 2017 and Oct 2017)
    //static inline void  SetNextWindowPosCenter(ImGuiCond c=0) { SetNextWindowPos(GetMainViewport()->GetCenter(), c, ImVec2::new(0.5f32,0.5f32)); } // OBSOLETED in 1.52 (between Aug 2017 and Oct 2017)
    //static inline bool  IsItemHoveredRect()                   { return IsItemHovered(ImGuiHoveredFlags_RectOnly); }               // OBSOLETED in 1.51 (between Jun 2017 and Aug 2017)
    //static inline bool  IsPosHoveringAnyWindow(const ImVec2&) { IM_ASSERT(0); return false; }                                     // OBSOLETED in 1.51 (between Jun 2017 and Aug 2017): This was misleading and partly broken. You probably want to use the io.WantCaptureMouse flag instead.
    //static inline bool  IsMouseHoveringAnyWindow()            { return IsWindowHovered(ImGuiHoveredFlags_AnyWindow); }            // OBSOLETED in 1.51 (between Jun 2017 and Aug 2017)
    //static inline bool  IsMouseHoveringWindow()               { return IsWindowHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup | ImGuiHoveredFlags_AllowWhenBlockedByActiveItem); }       // OBSOLETED in 1.51 (between Jun 2017 and Aug 2017)
    //static inline bool  CollapsingHeader(char* label, const char* str_id, framed: bool = true, default_open: bool = false) { return CollapsingHeader(label, (default_open ? (1 << 5) : 0)); } // OBSOLETED in 1.49
    //static inline ImFont*GetWindowFont()                      { return GetFont(); }                                               // OBSOLETED in 1.48
    //static inline float GetWindowFontSize()                   { return GetFontSize(); }                                           // OBSOLETED in 1.48
    //static inline void  SetScrollPosHere()                    { SetScrollHere(); }                                                // OBSOLETED in 1.42
}

// OBSOLETED in 1.82 (from Mars 2021): flags for AddRect(), AddRectFilled(), AddImageRounded(), PathRect()
typedef ImDrawCornerFlags: ImDrawFlags;
enum ImDrawCornerFlags_
{
    ImDrawCornerFlags_None      = ImDrawFlags_RoundCornersNone,         // Was == 0 prior to 1.82, this is now == ImDrawFlags_RoundCornersNone which is != 0 and not implicit
    ImDrawCornerFlags_TopLeft   = ImDrawFlags_RoundCornersTopLeft,      // Was == 0x01 (1 << 0) prior to 1.82. Order matches ImDrawFlags_NoRoundCorner* flag (we exploit this internally).
    ImDrawCornerFlags_TopRight  = ImDrawFlags_RoundCornersTopRight,     // Was == 0x02 (1 << 1) prior to 1.82.
    ImDrawCornerFlags_BotLeft   = ImDrawFlags_RoundCornersBottomLeft,   // Was == 0x04 (1 << 2) prior to 1.82.
    ImDrawCornerFlags_BotRight  = ImDrawFlags_RoundCornersBottomRight,  // Was == 0x08 (1 << 3) prior to 1.82.
    ImDrawCornerFlags_All       = ImDrawFlags_RoundCornersAll,          // Was == 0x0f32 prior to 1.82
    ImDrawCornerFlags_Top       = ImDrawCornerFlags_TopLeft | ImDrawCornerFlags_TopRight,
    ImDrawCornerFlags_Bot       = ImDrawCornerFlags_BotLeft | ImDrawCornerFlags_BotRight,
    ImDrawCornerFlags_Left      = ImDrawCornerFlags_TopLeft | ImDrawCornerFlags_BotLeft,
    ImDrawCornerFlags_Right     = ImDrawCornerFlags_TopRight | ImDrawCornerFlags_BotRight,
};

// RENAMED ImGuiKeyModFlags -> ImGuiModFlags in 1.88 (from April 2022)
typedef let mut ImGuiKeyModFlags: c_int = 0;
enum ImGuiKeyModFlags_ { let mut ModFlags_None: ImGuiKey =  ImGuiModFlags_None, let mut ModFlags_Ctrl: ImGuiKey =  ImGuiModFlags_Ctrl, let mut ModFlags_Shift: ImGuiKey =  ImGuiModFlags_Shift, let mut ModFlags_Alt: ImGuiKey =  ImGuiModFlags_Alt, let mut ModFlags_Super: ImGuiKey =  ImGuiModFlags_Super };

// #endif // #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS

// RENAMED IMGUI_DISABLE_METRICS_WINDOW > IMGUI_DISABLE_DEBUG_TOOLS in 1.88 (from June 2022)
// #if defined(IMGUI_DISABLE_METRICS_WINDOW) && !defined(IMGUI_DISABLE_OBSOLETE_FUNCTIONS) && !defined(IMGUI_DISABLE_DEBUG_TOOLS)
// #define IMGUI_DISABLE_DEBUG_TOOLS
// #endif
// #if defined(IMGUI_DISABLE_METRICS_WINDOW) && defined(IMGUI_DISABLE_OBSOLETE_FUNCTIONS)
#error IMGUI_DISABLE_METRICS_WINDOW was renamed to IMGUI_DISABLE_DEBUG_TOOLS, please use new name.
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

// #endif // #ifndef IMGUI_DISABLE
