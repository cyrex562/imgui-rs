// dear imgui, v1.89 WIP
// (demo code)

// Help:
// - Read FAQ at http://dearimgui.org/faq
// - Newcomers, read 'Programmer guide' in imgui.cpp for notes on how to setup Dear ImGui in your codebase.
// - Call and read ShowDemoWindow() in imgui_demo.cpp. All applications in examples/ are doing that.
// Read imgui.cpp for more details, documentation and comments.
// Get the latest version at https://github.com/ocornut/imgui

// Message to the person tempted to delete this file when integrating Dear ImGui into their codebase:
// Do NOT remove this file from your project! Think again! It is the most useful reference code that you and other
// coders will want to refer to and call. Have the ShowDemoWindow() function wired in an always-available
// debug menu of your game/app! Removing this file from your project is hindering access to documentation for everyone
// in your team, likely leading you to poorer usage of the library.
// Everything in this file will be stripped out by the linker if you don't call ShowDemoWindow().
// If you want to link core Dear ImGui in your shipped builds but want a thorough guarantee that the demo will not be
// linked, you can setup your imconfig.h with #define IMGUI_DISABLE_DEMO_WINDOWS and those functions will be empty.
// In another situation, whenever you have Dear ImGui available you probably want this to be available for reference.
// Thank you,
// -Your beloved friend, imgui_demo.cpp (which you won't delete)

// Message to beginner C/C++ programmers about the meaning of the 'static' keyword:
// In this demo code, we frequently use 'static' variables inside functions. A static variable persists across calls,
// so it is essentially like a global variable but declared inside the scope of the function. We do this as a way to
// gather code and data in the same place, to make the demo source code faster to read, faster to write, and smaller
// in size. It also happens to be a convenient way of storing simple UI related information as long as your function
// doesn't need to be reentrant or used in multiple threads. This might be a pattern you will want to use in your code,
// but most of the real data you would be editing is likely going to be stored outside your functions.

// The Demo code in this file is designed to be easy to copy-and-paste into your application!
// Because of this:
// - We never omit the  prefix when calling functions, even though most code here is in the same namespace.
// - We try to declare static variables in the local scope, as close as possible to the code using them.
// - We never use any of the helpers/facilities used internally by Dear ImGui, unless available in the public API.
// - We never use maths operators on ImVec2/ImVec4. For our other sources files we use them, and they are provided
//   by imgui_internal.h using the IMGUI_DEFINE_MATH_OPERATORS define. For your own sources file they are optional
//   and require you either enable those, either provide your own via IM_VEC2_CLASS_EXTRA in imconfig.h.
//   Because we can't assume anything about your support of maths operators, we cannot use them in imgui_demo.cpp.

// Navigating this file:
// - In Visual Studio IDE: CTRL+comma ("Edit.GoToAll") can follow symbols in comments, whereas CTRL+F12 ("Edit.GoToImplementation") cannot.
// - With Visual Assist installed: ALT+G ("VAssistX.GoToImplementation") can also follow symbols in comments.

/*

Index of this file:

// [SECTION] Forward Declarations, Helpers
// [SECTION] Demo Window / ShowDemoWindow()
// - sub section: ShowDemoWindowWidgets()
// - sub section: ShowDemoWindowLayout()
// - sub section: ShowDemoWindowPopups()
// - sub section: ShowDemoWindowTables()
// - sub section: ShowDemoWindowMisc()
// [SECTION] About Window / ShowAboutWindow()
// [SECTION] Style Editor / ShowStyleEditor()
// [SECTION] Example App: Main Menu Bar / ShowExampleAppMainMenuBar()
// [SECTION] Example App: Debug Console / ShowExampleAppConsole()
// [SECTION] Example App: Debug Log / ShowExampleAppLog()
// [SECTION] Example App: Simple Layout / ShowExampleAppLayout()
// [SECTION] Example App: Property Editor / ShowExampleAppPropertyEditor()
// [SECTION] Example App: Long Text / ShowExampleAppLongText()
// [SECTION] Example App: Auto Resize / ShowExampleAppAutoResize()
// [SECTION] Example App: Constrained Resize / ShowExampleAppConstrainedResize()
// [SECTION] Example App: Simple overlay / ShowExampleAppSimpleOverlay()
// [SECTION] Example App: Fullscreen window / ShowExampleAppFullscreen()
// [SECTION] Example App: Manipulating window titles / ShowExampleAppWindowTitles()
// [SECTION] Example App: Custom Rendering using ImDrawList API / ShowExampleAppCustomRendering()
// [SECTION] Example App: Docking, DockSpace / ShowExampleAppDockSpace()
// [SECTION] Example App: Documents Handling / ShowExampleAppDocuments()

*/

// #if defined(_MSC_VER) && !defined(_CRT_SECURE_NO_WARNINGS)
// #define _CRT_SECURE_NO_WARNINGS
// #endif

// #include "imgui.h"
// #ifndef IMGUI_DISABLE

// System includes
// #include <ctype.h>          // toupper
// #include <limits.h>         // INT_MIN, INT_MAX
// #include <math.h>           // sqrtf, powf, cosf, sinf, floorf, ceilf
// #include <stdio.h>          // vsnprintf, sscanf, printf
// #include <stdlib.h>         // NULL, malloc, free, atoi
// #if defined(_MSC_VER) && _MSC_VER <= 1500 // MSVC 2008 or earlier
// #include <stddef.h>         // intptr_t
// #else
// #include <stdint.h>         // intptr_t
// #endif

// Visual Studio warnings
// #ifdef _MSC_VER
// #pragma warning (disable: 4127)     // condition expression is constant
// #pragma warning (disable: 4996)     // 'This function or variable may be unsafe': strcpy, strdup, sprintf, vsnprintf, sscanf, fopen
// #pragma warning (disable: 26451)    // [Static Analyzer] Arithmetic overflow : Using operator 'xxx' on a 4 byte value and then casting the result to a 8 byte value. Cast the value to the wider type before calling operator 'xxx' to avoid overflow(io.2).
// #endif

// Clang/GCC warnings with -Weverything
// #if defined(__clang__)
// #if __has_warning("-Wunknown-warning-option")
// #pragma clang diagnostic ignored "-Wunknown-warning-option"         // warning: unknown warning group 'xxx'                     // not all warnings are known by all Clang versions and they tend to be rename-happy.. so ignoring warnings triggers new warnings on some configuration. Great!
// #endif
// #pragma clang diagnostic ignored "-Wunknown-pragmas"                // warning: unknown warning group 'xxx'
// #pragma clang diagnostic ignored "-Wold-style-cast"                 // warning: use of old-style cast                           // yes, they are more terse.
// #pragma clang diagnostic ignored "-Wdeprecated-declarations"        // warning: 'xx' is deprecated: The POSIX name for this..   // for strdup used in demo code (so user can copy & paste the code)
// #pragma clang diagnostic ignored "-Wint-to-void-pointer-cast"       // warning: cast to 'void *' from smaller integer type
// #pragma clang diagnostic ignored "-Wformat-security"                // warning: format string is not a string literal
// #pragma clang diagnostic ignored "-Wexit-time-destructors"          // warning: declaration requires an exit-time destructor    // exit-time destruction order is undefined. if MemFree() leads to users code that has been disabled before exit it might cause problems. ImGui coding style welcomes static/globals.
// #pragma clang diagnostic ignored "-Wunused-macros"                  // warning: macro is not used                               // we define snprintf/vsnprintf on Windows so they are available, but not always used.
// #pragma clang diagnostic ignored "-Wzero-as-null-pointer-constant"  // warning: zero as null pointer constant                   // some standard header variations use #define NULL 0
// #pragma clang diagnostic ignored "-Wdouble-promotion"               // warning: implicit conversion from 'float' to 'double' when passing argument to function  // using printf() is a misery with this as C++ va_arg ellipsis changes float to double.
// #pragma clang diagnostic ignored "-Wreserved-id-macro"              // warning: macro name is a reserved identifier
// #pragma clang diagnostic ignored "-Wimplicit-int-float-conversion"  // warning: implicit conversion from 'xxx' to 'float' may lose precision
// #elif defined(__GNUC__)
// #pragma GCC diagnostic ignored "-Wpragmas"                  // warning: unknown option after '#pragma GCC diagnostic' kind
// #pragma GCC diagnostic ignored "-Wint-to-pointer-cast"      // warning: cast to pointer from integer of different size
// #pragma GCC diagnostic ignored "-Wformat-security"          // warning: format string is not a string literal (potentially insecure)
// #pragma GCC diagnostic ignored "-Wdouble-promotion"         // warning: implicit conversion from 'float' to 'double' when passing argument to function
// #pragma GCC diagnostic ignored "-Wconversion"               // warning: conversion to 'xxxx' from 'xxxx' may alter its value
// #pragma GCC diagnostic ignored "-Wmisleading-indentation"   // [__GNUC__ >= 6] warning: this 'if' clause does not guard this statement      // GCC 6.0+ only. See #883 on GitHub.
// #endif

// Play it nice with Windows users (Update: May 2018, Notepad now supports Unix-style carriage returns!)
// #ifdef _WIN32
// #define IM_NEWLINE  "\r\n"
// #else
// #define IM_NEWLINE  "\n"
// #endif

// Helpers
// #if defined(_MSC_VER) && !defined(snprint0f32)
// #define snprintf    _snprintf
// #endif
// #if defined(_MSC_VER) && !defined(vsnprint0f32)
// #define vsnprintf   _vsnprintf
// #endif

// Format specifiers, printing 64-bit hasn't been decently standardized...
// In a real application you should be using PRId64 and PRIu64 from <inttypes.h> (non-windows) and on Windows define them yourself.
// #ifdef _MSC_VER
// #define IM_PRId64   "I64d"
// #define IM_PRIu64   "I64u"
// #else
// #define IM_PRId64   "lld"
// #define IM_PRIu64   "llu"
// #endif

// Helpers macros
// We normally try to not use many helpers in imgui_demo.cpp in order to make code easier to copy and paste,
// but making an exception here as those are largely simplifying code...
// In other imgui sources we can use nicer internal functions from imgui_internal.h (ImMin/ImMax) but not in the demo.
// #define IM_MIN(A, B)            (((A) < (B)) ? (A) : (B))
// #define IM_MAX(A, B)            (((A) >= (B)) ? (A) : (B))
// #define IM_CLAMP(V, MN, MX)     ((V) < (MN) ? (MN) : (V) > (MX) ? (MX) : (V))

// Enforce cdecl calling convention for functions called by the standard library, in case compilation settings changed the default to e.g. __vectorcall
// #ifndef IMGUI_CDECL
// #ifdef _MSC_VER
// #define IMGUI_CDECL __cdecl
// #else
// #define IMGUI_CDECL
// #endif
// #endif

//-----------------------------------------------------------------------------
// [SECTION] Forward Declarations, Helpers
//-----------------------------------------------------------------------------

// #if !defined(IMGUI_DISABLE_DEMO_WINDOWS)

// Forward Declarations
// pub unsafe fn ShowExampleAppDockSpace(p_open: *mut bool);
// pub unsafe fn ShowExampleAppDocuments(p_open: *mut bool);
// pub unsafe fn ShowExampleAppMainMenuBar();
// pub unsafe fn ShowExampleAppConsole(p_open: *mut bool);
// pub unsafe fn ShowExampleAppLog(p_open: *mut bool);
// pub unsafe fn ShowExampleAppLayout(p_open: *mut bool);
// pub unsafe fn ShowExampleAppPropertyEditor(p_open: *mut bool);
// pub unsafe fn ShowExampleAppLongText(p_open: *mut bool);
// pub unsafe fn ShowExampleAppAutoResize(p_open: *mut bool);
// pub unsafe fn ShowExampleAppConstrainedResize(p_open: *mut bool);
// pub unsafe fn ShowExampleAppSimpleOverlay(p_open: *mut bool);
// pub unsafe fn ShowExampleAppFullscreen(p_open: *mut bool);
// pub unsafe fn ShowExampleAppWindowTitles(p_open: *mut bool);
// pub unsafe fn ShowExampleAppCustomRendering(p_open: *mut bool);
// pub unsafe fn ShowExampleMenuFile();

use libc::{c_float, c_int};

// Helper to display a little (?) mark which shows a tooltip when hovered.
// In your own code you may want to display an actual icon if you are using a merged icon fonts (see docs/FONTS.md)
pub unsafe fn HelpMarker(desc: *const c_char)
{
    TextDisabled("(?)");
    if (IsItemHovered(ImGuiHoveredFlags_DelayShort))
    {
        BeginTooltip();
        PushTextWrapPos(GetFontSize() * 35.0);
        TextUnformatted(desc);
        PopTextWrapPos();
        EndTooltip();
    }
}

pub unsafe fn ShowDockingDisabledMessage()
{
    ImGuiIO& io = GetIO();
    Text("ERROR: Docking is not enabled! See Demo > Configuration.");
    Text("Set io.ConfigFlags |= ImGuiConfigFlags_DockingEnable in your code, or ");
    SameLine(0.0, 0.0);
    if (SmallButton("click here"))
        io.ConfigFlags |= ImGuiConfigFlags_DockingEnable;
}

// Helper to wire demo markers located in code to a interactive browser
typedef c_void (*ImGuiDemoMarkerCallback)(file: *const c_char, line: c_int, section: *const c_char, user_data: *mut c_void);
extern ImGuiDemoMarkerCallback  GImGuiDemoMarkerCallback;
extern *mut c_void                    GImGuiDemoMarkerCallbackUserData;
ImGuiDemoMarkerCallback         GImGuiDemoMarkerCallback= None;
*mut c_void                           GImGuiDemoMarkerCallbackUserData= None;
// #define IMGUI_DEMO_MARKER(section)  do { if (GImGuiDemoMarkerCallback != NULL) GImGuiDemoMarkerCallback(__FILE__, __LINE__, section, GImGuiDemoMarkerCallbackUserData); } while (0)

// Helper to display basic user controls.
pub unsafe fn ShowUserGuide()
{
    ImGuiIO& io = GetIO();
    BulletText("Double-click on title bar to collapse window.");
    BulletText(
        "Click and drag on lower corner to resize window\n"
        "(double-click to auto fit window to its contents).");
    BulletText("CTRL+Click on a slider or drag box to input value as text.");
    BulletText("TAB/SHIFT+TAB to cycle through keyboard editable fields.");
	BulletText("CTRL+Tab to select a window.");
	if (io.FontAllowUserScaling)
        BulletText("CTRL+Mouse Wheel to zoom window contents.");
    BulletText("While inputing text:\n");
    Indent();
    BulletText("CTRL+Left/Right to word jump.");
    BulletText("CTRL+A or double-click to select all.");
    BulletText("CTRL+X/C/V to use clipboard cut/copy/paste.");
    BulletText("CTRL+Z,CTRL+Y to undo/redo.");
    BulletText("ESCAPE to revert.");
    Unindent();
    BulletText("With keyboard navigation enabled:");
    Indent();
    BulletText("Arrow keys to navigate.");
    BulletText("Space to activate a widget.");
    BulletText("Return to input text into a widget.");
    BulletText("Escape to deactivate a widget, close popup, exit child window.");
    BulletText("Alt to jump to the menu layer of a window.");
    Unindent();
}

//-----------------------------------------------------------------------------
// [SECTION] Demo Window / ShowDemoWindow()
//-----------------------------------------------------------------------------
// - ShowDemoWindowWidgets()
// - ShowDemoWindowLayout()
// - ShowDemoWindowPopups()
// - ShowDemoWindowTables()
// - ShowDemoWindowColumns()
// - ShowDemoWindowMisc()
//-----------------------------------------------------------------------------

// We split the contents of the big ShowDemoWindow() function into smaller functions
// (because the link time of very large functions grow non-linearly)
pub unsafe fn ShowDemoWindowWidgets();
pub unsafe fn ShowDemoWindowLayout();
pub unsafe fn ShowDemoWindowPopups();
pub unsafe fn ShowDemoWindowTables();
pub unsafe fn ShowDemoWindowColumns();
pub unsafe fn ShowDemoWindowMisc();

// Demonstrate most Dear ImGui features (this is big function!)
// You may execute this function to experiment with the UI and understand what it does.
// You may then search for keywords in the code when you are interested by a specific feature.
pub unsafe fn ShowDemoWindow(p_open: *mut bool)
{
    // Exceptionally add an extra assert here for people confused about initial Dear ImGui setup
    // Most ImGui functions would normally just crash if the context is missing.
    // IM_ASSERT(GetCurrentContext() != NULL && "Missing dear imgui context. Refer to examples app!");

    // Examples Apps (accessible from the "Examples" menu)
    static let mut show_app_main_menu_bar: bool =  false;
    static let mut show_app_dockspace: bool =  false;
    static let mut show_app_documents: bool =  false;

    static let mut show_app_console: bool =  false;
    static let mut show_app_log: bool =  false;
    static let mut show_app_layout: bool =  false;
    static let mut show_app_property_editor: bool =  false;
    static let mut show_app_long_text: bool =  false;
    static let mut show_app_auto_resize: bool =  false;
    static let mut show_app_constrained_resize: bool =  false;
    static let mut show_app_simple_overlay: bool =  false;
    static let mut show_app_fullscreen: bool =  false;
    static let mut show_app_window_titles: bool =  false;
    static let mut show_app_custom_rendering: bool =  false;

    if show_app_main_menu_bar {        ShowExampleAppMainMenuBar(); }
    if (show_app_dockspace)           ShowExampleAppDockSpace(&show_app_dockspace);     // Process the Docking app first, as explicit DockSpace() nodes needs to be submitted early (read comments near the DockSpace function)
    if (show_app_documents)           ShowExampleAppDocuments(&show_app_documents);     // Process the Document app next, as it may also use a DockSpace()

    if (show_app_console)             ShowExampleAppConsole(&show_app_console);
    if (show_app_log)                 ShowExampleAppLog(&show_app_log);
    if (show_app_layout)              ShowExampleAppLayout(&show_app_layout);
    if (show_app_property_editor)     ShowExampleAppPropertyEditor(&show_app_property_editor);
    if (show_app_long_text)           ShowExampleAppLongText(&show_app_long_text);
    if (show_app_auto_resize)         ShowExampleAppAutoResize(&show_app_auto_resize);
    if (show_app_constrained_resize)  ShowExampleAppConstrainedResize(&show_app_constrained_resize);
    if (show_app_simple_overlay)      ShowExampleAppSimpleOverlay(&show_app_simple_overlay);
    if (show_app_fullscreen)          ShowExampleAppFullscreen(&show_app_fullscreen);
    if (show_app_window_titles)       ShowExampleAppWindowTitles(&show_app_window_titles);
    if (show_app_custom_rendering)    ShowExampleAppCustomRendering(&show_app_custom_rendering);

    // Dear ImGui Apps (accessible from the "Tools" menu)
    static let mut show_app_metrics: bool =  false;
    static let mut show_app_debug_log: bool =  false;
    static let mut show_app_stack_tool: bool =  false;
    static let mut show_app_about: bool =  false;
    static let mut show_app_style_editor: bool =  false;

    if (show_app_metrics)
        ShowMetricsWindow(&show_app_metrics);
    if (show_app_debug_log)
        ShowDebugLogWindow(&show_app_debug_log);
    if (show_app_stack_tool)
        ShowStackToolWindow(&show_app_stack_tool);
    if (show_app_about)
        ShowAboutWindow(&show_app_about);
    if (show_app_style_editor)
    {
        Begin("Dear ImGui Style Editor", &show_app_style_editor);
        ShowStyleEditor();
        End();
    }

    // Demonstrate the various window flags. Typically you would just use the default!
    static let mut no_titlebar: bool =  false;
    static let mut no_scrollbar: bool =  false;
    static let mut no_menu: bool =  false;
    static let mut no_move: bool =  false;
    static let mut no_resize: bool =  false;
    static let mut no_collapse: bool =  false;
    static let mut no_close: bool =  false;
    static let mut no_nav: bool =  false;
    static let mut no_background: bool =  false;
    static let mut no_bring_to_front: bool =  false;
    static let mut no_docking: bool =  false;
    static let mut unsaved_document: bool =  false;

    window_flags: ImGuiWindowFlags = 0;
    if (no_titlebar)        window_flags |= ImGuiWindowFlags_NoTitleBar;
    if (no_scrollbar)       window_flags |= ImGuiWindowFlags_NoScrollbar;
    if (!no_menu)           window_flags |= ImGuiWindowFlags_MenuBar;
    if (no_move)            window_flags |= ImGuiWindowFlags_NoMove;
    if (no_resize)          window_flags |= ImGuiWindowFlags_NoResize;
    if (no_collapse)        window_flags |= ImGuiWindowFlags_NoCollapse;
    if (no_nav)             window_flags |= ImGuiWindowFlags_NoNav;
    if (no_background)      window_flags |= ImGuiWindowFlags_NoBackground;
    if (no_bring_to_front)  window_flags |= ImGuiWindowFlags_NoBringToFrontOnFocus;
    if (no_docking)         window_flags |= ImGuiWindowFlags_NoDocking;
    if (unsaved_document)   window_flags |= ImGuiWindowFlags_UnsavedDocument;
    if no_close {            p_open= None(); }  // Don't pass ourto: *mut bool Begin

    // We specify a default position/size in case there's no data in the .ini file.
    // We only do it to make the demo applications a little more welcoming, but typically this isn't required.
    main_viewport: *const ImGuiViewport = GetMainViewport();
    SetNextWindowPos(ImVec2::new(main_viewport.WorkPos.x + 650, main_viewport.WorkPos.y + 20), ImGuiCond_FirstUseEver);
    SetNextWindowSize(ImVec2::new(550, 680), ImGuiCond_FirstUseEver);

    // Main body of the Demo window starts here.
    if (!Begin("Dear ImGui Demo", p_open, window_flags))
    {
        // Early out if the window is collapsed, as an optimization.
        End();
        return;
    }

    // Most "big" widgets share a common width settings by default. See 'Demo->Layout->Widgets Width' for details.

    // e.g. Use 2/3 of the space for widgets and 1/3 for labels (right align)
    //PushItemWidth(-GetWindowWidth() * 0.350f32);

    // e.g. Leave a fixed amount of width for labels (by passing a negative value), the rest goes to widgets.
    PushItemWidth(GetFontSize() * -12);

    // Menu Bar
    if (BeginMenuBar())
    {
        if (BeginMenu("Menu"))
        {
            IMGUI_DEMO_MARKER("Menu/File");
            ShowExampleMenuFile();
            EndMenu();
        }
        if (BeginMenu("Examples"))
        {
            IMGUI_DEMO_MARKER("Menu/Examples");
            MenuItem("Main menu bar", None, &show_app_main_menu_bar);
            MenuItem("Console", None, &show_app_console);
            MenuItem("Log", None, &show_app_log);
            MenuItem("Simple layout", None, &show_app_layout);
            MenuItem("Property editor", None, &show_app_property_editor);
            MenuItem("Long text display", None, &show_app_long_text);
            MenuItem("Auto-resizing window", None, &show_app_auto_resize);
            MenuItem("Constrained-resizing window", None, &show_app_constrained_resize);
            MenuItem("Simple overlay", None, &show_app_simple_overlay);
            MenuItem("Fullscreen window", None, &show_app_fullscreen);
            MenuItem("Manipulating window titles", None, &show_app_window_titles);
            MenuItem("Custom rendering", None, &show_app_custom_rendering);
            MenuItem("Dockspace", None, &show_app_dockspace);
            MenuItem("Documents", None, &show_app_documents);
            EndMenu();
        }
        //if (MenuItem("MenuItem")) {} // You can also use MenuItem() inside a menu bar!
        if (BeginMenu("Tools"))
        {
            IMGUI_DEMO_MARKER("Menu/Tools");
// #ifndef IMGUI_DISABLE_DEBUG_TOOLS
            let has_debug_tools: bool = true;
// #else
            let has_debug_tools: bool = false;
// #endif
            MenuItem("Metrics/Debugger", None, &show_app_metrics, has_debug_tools);
            MenuItem("Debug Log", None, &show_app_debug_log, has_debug_tools);
            MenuItem("Stack Tool", None, &show_app_stack_tool, has_debug_tools);
            MenuItem("Style Editor", None, &show_app_style_editor);
            MenuItem("About Dear ImGui", None, &show_app_about);
            EndMenu();
        }
        EndMenuBar();
    }

    Text("dear imgui says hello! ({}) ({})", IMGUI_VERSION, IMGUI_VERSION_NUM);
    Spacing();

    IMGUI_DEMO_MARKER("Help");
    if (CollapsingHeader("Help"))
    {
        Text("ABOUT THIS DEMO:");
        BulletText("Sections below are demonstrating many aspects of the library.");
        BulletText("The \"Examples\" menu above leads to more demo contents.");
        BulletText("The \"Tools\" menu above gives access to: About Box, Style Editor,\n"
                          "and Metrics/Debugger (general purpose Dear ImGui debugging tool).");
        Separator();

        Text("PROGRAMMER GUIDE:");
        BulletText("See the ShowDemoWindow() code in imgui_demo.cpp. <- you are here!");
        BulletText("See comments in imgui.cpp.");
        BulletText("See example applications in the examples/ folder.");
        BulletText("Read the FAQ at http://www.dearimgui.org/faq/");
        BulletText("Set 'io.ConfigFlags |= NavEnableKeyboard' for keyboard controls.");
        BulletText("Set 'io.ConfigFlags |= NavEnableGamepad' for gamepad controls.");
        Separator();

        Text("USER GUIDE:");
        ShowUserGuide();
    }

    IMGUI_DEMO_MARKER("Configuration");
    if (CollapsingHeader("Configuration"))
    {
        ImGuiIO& io = GetIO();

        if (TreeNode("Configuration##2"))
        {
            CheckboxFlags("io.ConfigFlags: NavEnableKeyboard",    &io.ConfigFlags, ImGuiConfigFlags_NavEnableKeyboard);
            SameLine(); HelpMarker("Enable keyboard controls.");
            CheckboxFlags("io.ConfigFlags: NavEnableGamepad",     &io.ConfigFlags, ImGuiConfigFlags_NavEnableGamepad);
            SameLine(); HelpMarker("Enable gamepad controls. Require backend to set io.BackendFlags |= IM_GUI_BACKEND_FLAGS_HAS_GAMEPAD.\n\nRead instructions in imgui.cpp for details.");
            CheckboxFlags("io.ConfigFlags: NavEnableSetMousePos", &io.ConfigFlags, ImGuiConfigFlags_NavEnableSetMousePos);
            SameLine(); HelpMarker("Instruct navigation to move the mouse cursor. See comment for ImGuiConfigFlags_NavEnableSetMousePos.");
            CheckboxFlags("io.ConfigFlags: NoMouse",              &io.ConfigFlags, ImGuiConfigFlags_NoMouse);
            if (io.ConfigFlags & ImGuiConfigFlags_NoMouse)
            {
                // The "NoMouse" option can get us stuck with a disabled mouse! Let's provide an alternative way to fix it:
                if (fmodf(GetTime(), 0.4) < 0.200)
                {
                    SameLine();
                    Text("<<PRESS SPACE TO DISABLE>>");
                }
                if (IsKeyPressed(ImGuiKey_Space))
                    io.ConfigFlags &= !ImGuiConfigFlags_NoMouse;
            }
            CheckboxFlags("io.ConfigFlags: NoMouseCursorChange", &io.ConfigFlags, ImGuiConfigFlags_NoMouseCursorChange);
            SameLine(); HelpMarker("Instruct backend to not alter mouse cursor shape and visibility.");

            CheckboxFlags("io.ConfigFlags: DockingEnable", &io.ConfigFlags, ImGuiConfigFlags_DockingEnable);
            SameLine();
            if (io.ConfigDockingWithShift)
                HelpMarker("Drag from window title bar or their tab to dock/undock. Hold SHIFT to enable docking.\n\nDrag from window menu button (upper-left button) to undock an entire node (all windows).");
            else
                HelpMarker("Drag from window title bar or their tab to dock/undock. Hold SHIFT to disable docking.\n\nDrag from window menu button (upper-left button) to undock an entire node (all windows).");
            if (io.ConfigFlags & ImGuiConfigFlags_DockingEnable)
            {
                Indent();
                Checkbox("io.ConfigDockingNoSplit", &io.ConfigDockingNoSplit);
                SameLine(); HelpMarker("Simplified docking mode: disable window splitting, so docking is limited to merging multiple windows together into tab-bars.");
                Checkbox("io.ConfigDockingWithShift", &io.ConfigDockingWithShift);
                SameLine(); HelpMarker("Enable docking when holding Shift only (allow to drop in wider space, reduce visual noise)");
                Checkbox("io.ConfigDockingAlwaysTabBar", &io.ConfigDockingAlwaysTabBar);
                SameLine(); HelpMarker("Create a docking node and tab-bar on single floating windows.");
                Checkbox("io.ConfigDockingTransparentPayload", &io.ConfigDockingTransparentPayload);
                SameLine(); HelpMarker("Make window or viewport transparent when docking and only display docking boxes on the target viewport. Useful if rendering of multiple viewport cannot be synced. Best used with ConfigViewportsNoAutoMerge.");
                Unindent();
            }

            CheckboxFlags("io.ConfigFlags: ViewportsEnable", &io.ConfigFlags, ImGuiConfigFlags_ViewportsEnable);
            SameLine(); HelpMarker("[beta] Enable beta multi-viewports support. See ImGuiPlatformIO for details.");
            if (io.ConfigFlags & ImGuiConfigFlags_ViewportsEnable)
            {
                Indent();
                Checkbox("io.ConfigViewportsNoAutoMerge", &io.ConfigViewportsNoAutoMerge);
                SameLine(); HelpMarker("Set to make all floating imgui windows always create their own viewport. Otherwise, they are merged into the main host viewports when overlapping it.");
                Checkbox("io.ConfigViewportsNoTaskBarIcon", &io.ConfigViewportsNoTaskBarIcon);
                SameLine(); HelpMarker("Toggling this at runtime is normally unsupported (most platform backends won't refresh the task bar icon state right away).");
                Checkbox("io.ConfigViewportsNoDecoration", &io.ConfigViewportsNoDecoration);
                SameLine(); HelpMarker("Toggling this at runtime is normally unsupported (most platform backends won't refresh the decoration right away).");
                Checkbox("io.ConfigViewportsNoDefaultParent", &io.ConfigViewportsNoDefaultParent);
                SameLine(); HelpMarker("Toggling this at runtime is normally unsupported (most platform backends won't refresh the parenting right away).");
                Unindent();
            }

            Checkbox("io.ConfigInputTrickleEventQueue", &io.ConfigInputTrickleEventQueue);
            SameLine(); HelpMarker("Enable input queue trickling: some types of events submitted during the same frame (e.g. button down + up) will be spread over multiple frames, improving interactions with low framerates.");
            Checkbox("io.ConfigInputTextCursorBlink", &io.ConfigInputTextCursorBlink);
            SameLine(); HelpMarker("Enable blinking cursor (optional as some users consider it to be distracting).");
            Checkbox("io.ConfigInputTextEnterKeepActive", &io.ConfigInputTextEnterKeepActive);
            SameLine(); HelpMarker("Pressing Enter will keep item active and select contents (single-line only).");
            Checkbox("io.ConfigDragClickToInputText", &io.ConfigDragClickToInputText);
            SameLine(); HelpMarker("Enable turning DragXXX widgets into text input with a simple mouse click-release (without moving).");
            Checkbox("io.ConfigWindowsResizeFromEdges", &io.ConfigWindowsResizeFromEdges);
            SameLine(); HelpMarker("Enable resizing of windows from their edges and from the lower-left corner.\nThis requires (io.BackendFlags & IM_GUI_BACKEND_FLAGS_HAS_MOUSE_CURSORS) because it needs mouse cursor feedback.");
            Checkbox("io.ConfigWindowsMoveFromTitleBarOnly", &io.ConfigWindowsMoveFromTitleBarOnly);
            Checkbox("io.MouseDrawCursor", &io.MouseDrawCursor);
            SameLine(); HelpMarker("Instruct Dear ImGui to render a mouse cursor itself. Note that a mouse cursor rendered via your application GPU rendering path will feel more laggy than hardware cursor, but will be more in sync with your other visuals.\n\nSome desktop applications may use both kinds of cursors (e.g. enable software cursor only when resizing/dragging something).");
            Text("Also see Style->Rendering for rendering options.");
            TreePop();
            Separator();
        }

        IMGUI_DEMO_MARKER("Configuration/Backend Flags");
        if (TreeNode("Backend Flags"))
        {
            HelpMarker(
                "Those flags are set by the backends (imgui_impl_xxx files) to specify their capabilities.\n"
                "Here we expose them as read-only fields to avoid breaking interactions with your backend.");

            // Make a local copy to avoid modifying actual backend flags.
            // FIXME: We don't use BeginDisabled() to keep label bright, maybe we need a BeginReadonly() equivalent..
            ImGuiBackendFlags backend_flags = io.BackendFlags;
            CheckboxFlags("io.BackendFlags: HasGamepad",             &backend_flags, ImGuiBackendFlags_HasGamepad);
            CheckboxFlags("io.BackendFlags: HasMouseCursors",        &backend_flags, ImGuiBackendFlags_HasMouseCursors);
            CheckboxFlags("io.BackendFlags: HasSetMousePos",         &backend_flags, ImGuiBackendFlags_HasSetMousePos);
            CheckboxFlags("io.BackendFlags: PlatformHasViewports",   &backend_flags, ImGuiBackendFlags_PlatformHasViewports);
            CheckboxFlags("io.BackendFlags: HasMouseHoveredViewport",&backend_flags, ImGuiBackendFlags_HasMouseHoveredViewport);
            CheckboxFlags("io.BackendFlags: RendererHasVtxOffset",   &backend_flags, ImGuiBackendFlags_RendererHasVtxOffset);
            CheckboxFlags("io.BackendFlags: RendererHasViewports",   &backend_flags, ImGuiBackendFlags_RendererHasViewports);
            TreePop();
            Separator();
        }

        IMGUI_DEMO_MARKER("Configuration/Style");
        if (TreeNode("Style"))
        {
            HelpMarker("The same contents can be accessed in 'Tools->Style Editor' or by calling the ShowStyleEditor() function.");
            ShowStyleEditor();
            TreePop();
            Separator();
        }

        IMGUI_DEMO_MARKER("Configuration/Capture, Logging");
        if (TreeNode("Capture/Logging"))
        {
            HelpMarker(
                "The logging API redirects all text output so you can easily capture the content of "
                "a window or a block. Tree nodes can be automatically expanded.\n"
                "Try opening any of the contents below in this window and then click one of the \"Log To\" button.");
            LogButtons();

            HelpMarker("You can also call LogText() to output directly to the log without a visual output.");
            if (Button("Copy \"Hello, world!\" to clipboard"))
            {
                LogToClipboard();
                LogText("Hello, world!");
                LogFinish();
            }
            TreePop();
        }
    }

    IMGUI_DEMO_MARKER("Window options");
    if (CollapsingHeader("Window options"))
    {
        if (BeginTable("split", 3))
        {
            TableNextColumn(); Checkbox("No titlebar", &no_titlebar);
            TableNextColumn(); Checkbox("No scrollbar", &no_scrollbar);
            TableNextColumn(); Checkbox("No menu", &no_menu);
            TableNextColumn(); Checkbox("No move", &no_move);
            TableNextColumn(); Checkbox("No resize", &no_resize);
            TableNextColumn(); Checkbox("No collapse", &no_collapse);
            TableNextColumn(); Checkbox("No close", &no_close);
            TableNextColumn(); Checkbox("No nav", &no_nav);
            TableNextColumn(); Checkbox("No background", &no_background);
            TableNextColumn(); Checkbox("No bring to front", &no_bring_to_front);
            TableNextColumn(); Checkbox("No docking", &no_docking);
            TableNextColumn(); Checkbox("Unsaved document", &unsaved_document);
            EndTable();
        }
    }

    // All demo contents
    ShowDemoWindowWidgets();
    ShowDemoWindowLayout();
    ShowDemoWindowPopups();
    ShowDemoWindowTables();
    ShowDemoWindowMisc();

    // End of ShowDemoWindow()
    PopItemWidth();
    End();
}

pub unsafe fn ShowDemoWindowWidgets()
{
    IMGUI_DEMO_MARKER("Widgets");
    if !CollapsingHeader("Widgets") { return ; }

    static let mut disable_all: bool =  false; // The Checkbox for that is inside the "Disabled" section at the bottom
    if disable_all {
        BeginDisabled(); }

    IMGUI_DEMO_MARKER("Widgets/Basic");
    if (TreeNode("Basic"))
    {
        IMGUI_DEMO_MARKER("Widgets/Basic/Button");
        static let clicked: c_int = 0;
        if (Button("Button"))
            clicked+= 1;
        if (clicked & 1)
        {
            SameLine();
            Text("Thanks for clicking me!");
        }

        IMGUI_DEMO_MARKER("Widgets/Basic/Checkbox");
        static let mut check: bool =  true;
        Checkbox("checkbox", &check);

        IMGUI_DEMO_MARKER("Widgets/Basic/RadioButton");
        static let e: c_int = 0;
        RadioButton("radio a", &e, 0); SameLine();
        RadioButton("radio b", &e, 1); SameLine();
        RadioButton("radio c", &e, 2);

        // Color buttons, demonstrate using PushID() to add unique identifier in the ID stack, and changing style.
        IMGUI_DEMO_MARKER("Widgets/Basic/Buttons (Colored)");
        for (let i: c_int = 0; i < 7; i++)
        {
            if i > 0 {
                SameLine(); }
            PushID(i);
            PushStyleColor(ImGuiCol_Button, (ImVec4)ImColor::HSV(i / 7.0, 0.6f, 0.60));
            PushStyleColor(ImGuiCol_ButtonHovered, (ImVec4)ImColor::HSV(i / 7.0, 0.7f, 0.70f32));
            PushStyleColor(ImGuiCol_ButtonActive, (ImVec4)ImColor::HSV(i / 7.0, 0.8f, 0.80));
            Button("Click");
            PopStyleColor(3);
            PopID();
        }

        // Use AlignTextToFramePadding() to align text baseline to the baseline of framed widgets elements
        // (otherwise a Text+SameLine+Button sequence will have the text a little too high by default!)
        // See 'Demo->Layout->Text Baseline Alignment' for details.
        AlignTextToFramePadding();
        Text("Hold to repeat:");
        SameLine();

        // Arrow buttons with Repeater
        IMGUI_DEMO_MARKER("Widgets/Basic/Buttons (Repeating)");
        static let counter: c_int = 0;
        let spacing: c_float =  GetStyle().ItemInnerSpacing.x;
        PushButtonRepeat(true);
        if (ArrowButton("##left", ImGuiDir_Left)) { counter-= 1; }
        SameLine(0.0, spacing);
        if (ArrowButton("##right", ImGuiDir_Right)) { counter+= 1; }
        PopButtonRepeat();
        SameLine();
        Text("{}", counter);

        Separator();
        LabelText("label", "Value");

        {
            // Using the _simplified_ one-liner Combo() api here
            // See "Combo" section for examples of how to use the more flexible BeginCombo()/EndCombo() api.
            IMGUI_DEMO_MARKER("Widgets/Basic/Combo");
            items: *const c_char[] = { "AAAA", "BBBB", "CCCC", "DDDD", "EEEE", "FFFF", "GGGG", "HHHH", "IIIIIII", "JJJJ", "KKKKKKK" };
            static let item_current: c_int = 0;
            Combo("combo", &item_current, items, items.len());
            SameLine(); HelpMarker(
                "Using the simplified one-liner Combo API here.\nRefer to the \"Combo\" section below for an explanation of how to use the more flexible and general BeginCombo/EndCombo API.");
        }

        {
            // To wire InputText() with std::string or any other custom string type,
            // see the "Text Input > Resize Callback" section of this demo, and the misc/cpp/imgui_stdlib.h file.
            IMGUI_DEMO_MARKER("Widgets/Basic/InputText");
            static str0: [c_char;128] = "Hello, world!";
            InputText("input text", str0, str0.len());
            SameLine(); HelpMarker(
                "USER:\n"
                "Hold SHIFT or use mouse to select text.\n"
                "CTRL+Left/Right to word jump.\n"
                "CTRL+A or double-click to select all.\n"
                "CTRL+X,CTRL+C,CTRL+V clipboard.\n"
                "CTRL+Z,CTRL+Y undo/redo.\n"
                "ESCAPE to revert.\n\n"
                "PROGRAMMER:\n"
                "You can use the ImGuiInputTextFlags_CallbackResize facility if you need to wire InputText() "
                "to a dynamic string type. See misc/cpp/imgui_stdlib.h for an example (this is not demonstrated "
                "in imgui_demo.cpp).");

            static str1: [c_char;128] = "";
            InputTextWithHint("input text (w/ hint)", "enter text here", str1, str1.len());

            IMGUI_DEMO_MARKER("Widgets/Basic/InputInt, InputFloat");
            static let i0: c_int = 123;
            InputInt("input int", &i0);

            static let f0: c_float =  0.001f;
            InputFloat("input float", &f0, 0.01f, 1.0, "{}");

            static double d0 = 999999.000001;
            InputDouble("input double", &d0, 0.01f, 1.0, "%.8f");

            static let f1: c_float =  1.e10f32;
            InputFloat("input scientific", &f1, 0.0, 0.0, "%e");
            SameLine(); HelpMarker(
                "You can input value using the scientific notation,\n"
                "  e.g. \"1e+8\" becomes \"100000000\".");

            staticvec4a: c_float[4] = { 0.1.0, 0.20, 0.3f32, 0.44f };
            InputFloat3("input float3", vec4a);
        }

        {
            IMGUI_DEMO_MARKER("Widgets/Basic/DragInt, DragFloat");
            static let i1: c_int = 50, i2 = 42;
            DragInt("drag int", &i1, 1);
            SameLine(); HelpMarker(
                "Click and drag to edit value.\n"
                "Hold SHIFT/ALT for faster/slower edit.\n"
                "Double-click or CTRL+click to input value.");

            DragInt("drag int 0..100", &i2, 1, 0, 100, "{}%%", ImGuiSliderFlags_AlwaysClamp);

            static let f1: c_float =  1.0, f2 = 0.0067f;
            DragFloat("drag float", &f1, 0.0050f32);
            DragFloat("drag small float", &f2, 0.01f, 0.0, 0.0, "{}6f ns");
        }

        {
            IMGUI_DEMO_MARKER("Widgets/Basic/SliderInt, SliderFloat");
            static let i1: c_int = 0;
            SliderInt("slider int", &i1, -1, 3);
            SameLine(); HelpMarker("CTRL+click to input value.");

            static let f1: c_float =  0.123f, f2 = 0.0;
            SliderFloat("slider float", &f1, 0.0, 1.0, "ratio = {}");
            SliderFloat("slider float (log)", &f2, -10.0, 10.0, "%.4f", ImGuiSliderFlags_Logarithmic);

            IMGUI_DEMO_MARKER("Widgets/Basic/SliderAngle");
            static let angle: c_float =  0.0;
            SliderAngle("slider angle", &angle);

            // Using the format string to display a name instead of an integer.
            // Here we completely omit '{}' from the format string, so it'll only display a name.
            // This technique can also be used with DragInt().
            IMGUI_DEMO_MARKER("Widgets/Basic/Slider (enum)");
            enum Element { Element_Fire, Element_Earth, Element_Air, Element_Water, Element_COUNT };
            static let elem: c_int = Element_Fire;
            elems_names: *const c_char[Element_COUNT] = { "Fire", "Earth", "Air", "Water" };
            let mut  elem_name: *const c_char = if elem >= 0 && elem < Element_COUNT { elems_names[elem]} else { "Unknown"};
            SliderInt("slider enum", &elem, 0, Element_COUNT - 1, elem_name);
            SameLine(); HelpMarker("Using the format string parameter to display a name instead of the underlying integer.");
        }

        {
            IMGUI_DEMO_MARKER("Widgets/Basic/ColorEdit3, ColorEdit4");
            staticcol1: c_float[3] = { 1.0, 0.0, 0.2f };
            staticcol2: c_float[4] = { 0.4f, 0.7f, 0.0, 0.5 };
            ColorEdit3("color 1", col1);
            SameLine(); HelpMarker(
                "Click on the color square to open a color picker.\n"
                "Click and hold to use drag and drop.\n"
                "Right-click on the color square to show options.\n"
                "CTRL+click on individual component to input value.\n");

            ColorEdit4("color 2", col2);
        }

        {
            // Using the _simplified_ one-liner ListBox() api here
            // See "List boxes" section for examples of how to use the more flexible BeginListBox()/EndListBox() api.
            IMGUI_DEMO_MARKER("Widgets/Basic/ListBox");
            items: *const c_char[] = { "Apple", "Banana", "Cherry", "Kiwi", "Mango", "Orange", "Pineapple", "Strawberry", "Watermelon" };
            static let item_current: c_int = 1;
            ListBox("listbox", &item_current, items, items.len(), 4);
            SameLine(); HelpMarker(
                "Using the simplified one-liner ListBox API here.\nRefer to the \"List boxes\" section below for an explanation of how to use the more flexible and general BeginListBox/EndListBox API.");
        }

        {
            // Tooltips
            IMGUI_DEMO_MARKER("Widgets/Basic/Tooltips");
            AlignTextToFramePadding();
            Text("Tooltips:");

            SameLine();
            Button("Button");
            if IsItemHovered() {
                SetTooltip("I am a tooltip")(); }

            SameLine();
            Button("Fancy");
            if (IsItemHovered())
            {
                BeginTooltip();
                Text("I am a fancy tooltip");
                staticarr: c_float[] = { 0.6f, 0.1f, 1.0, 0.5, 0.92f, 0.1f, 0.2f };
                PlotLines("Curve", arr, arr.len());
                Text("Sin(time) = {}", sinf(GetTime()));
                EndTooltip();
            }

            SameLine();
            Button("Delayed");
            if (IsItemHovered(ImGuiHoveredFlags_DelayNormal)) // Delay best used on items that highlight on hover, so this not a great example!
                SetTooltip("I am a tooltip with a delay.");

            SameLine();
            HelpMarker(
                "Tooltip are created by using the IsItemHovered() function over any kind of item.");

        }

        TreePop();
    }

    // Testing ImGuiOnceUponAFrame helper.
    //static ImGuiOnceUponAFrame once;
    //for (int i = 0; i < 5; i++)
    //    if (once)
    //        Text("This will be displayed only once.");

    IMGUI_DEMO_MARKER("Widgets/Trees");
    if (TreeNode("Trees"))
    {
        IMGUI_DEMO_MARKER("Widgets/Trees/Basic trees");
        if (TreeNode("Basic trees"))
        {
            for (let i: c_int = 0; i < 5; i++)
            {
                // Use SetNextItemOpen() so set the default state of a node to be open. We could
                // also use TreeNodeEx() with the ImGuiTreeNodeFlags_DefaultOpen flag to achieve the same thing!
                if (i == 0)
                    SetNextItemOpen(true, ImGuiCond_Once);

                if (TreeNode(i, "Child {}", i))
                {
                    Text("blah blah");
                    SameLine();
                    if (SmallButton("button")) {}
                    TreePop();
                }
            }
            TreePop();
        }

        IMGUI_DEMO_MARKER("Widgets/Trees/Advanced, with Selectable nodes");
        if (TreeNode("Advanced, with Selectable nodes"))
        {
            HelpMarker(
                "This is a more typical looking tree with selectable nodes.\n"
                "Click to select, CTRL+Click to toggle, click on arrows or double-click to open.");
            static base_flags: ImGuiTreeNodeFlags = ImGuiTreeNodeFlags_OpenOnArrow | ImGuiTreeNodeFlags_OpenOnDoubleClick | ImGuiTreeNodeFlags_SpanAvailWidth;
            static let mut align_label_with_current_x_position: bool =  false;
            static let mut test_drag_and_drop: bool =  false;
            CheckboxFlags("ImGuiTreeNodeFlags_OpenOnArrow",       &base_flags, ImGuiTreeNodeFlags_OpenOnArrow);
            CheckboxFlags("ImGuiTreeNodeFlags_OpenOnDoubleClick", &base_flags, ImGuiTreeNodeFlags_OpenOnDoubleClick);
            CheckboxFlags("ImGuiTreeNodeFlags_SpanAvailWidth",    &base_flags, ImGuiTreeNodeFlags_SpanAvailWidth); SameLine(); HelpMarker("Extend hit area to all available width instead of allowing more items to be laid out after the node.");
            CheckboxFlags("ImGuiTreeNodeFlags_SpanFullWidth",     &base_flags, ImGuiTreeNodeFlags_SpanFullWidth);
            Checkbox("Align label with current X position", &align_label_with_current_x_position);
            Checkbox("Test tree node as drag source", &test_drag_and_drop);
            Text("Hello!");
            if align_label_with_current_x_position {
                Unindent(GetTreeNodeToLabelSpacing())(); }

            // 'selection_mask' is dumb representation of what may be user-side selection state.
            //  You may retain selection state inside or outside your objects in whatever format you see fit.
            // 'node_clicked' is temporary storage of what node we have clicked to process selection at the end
            /// of the loop. May be a pointer to your own node type, etc.
            static let selection_mask: c_int = (1 << 2);
            let node_clicked: c_int = -1;
            for (let i: c_int = 0; i < 6; i++)
            {
                // Disable the default "open on single-click behavior" + set Selected flag according to our selection.
                // To alter selection we use IsItemClicked() && !IsItemToggledOpen(), so clicking on an arrow doesn't alter selection.
                node_flags: ImGuiTreeNodeFlags = base_flags;
                let is_selected: bool = (selection_mask & (1 << i)) != 0;
                if (is_selected)
                    node_flags |= ImGuiTreeNodeFlags_Selected;
                if (i < 3)
                {
                    // Items 0..2 are Tree Node
                    let mut node_open: bool =  TreeNodeEx(i, node_flags, "Selectable Node {}", i);
                    if IsItemClicked() && !IsItemToggledOpen() {
                        node_clicked = i;}
                    if (test_drag_and_drop && BeginDragDropSource())
                    {
                        SetDragDropPayload("_TREENODE", None, 0);
                        Text("This is a drag and drop source");
                        EndDragDropSource();
                    }
                    if (node_open)
                    {
                        BulletText("Blah blah\nBlah Blah");
                        TreePop();
                    }
                }
                else
                {
                    // Items 3..5 are Tree Leaves
                    // The only reason we use TreeNode at all is to allow selection of the leaf. Otherwise we can
                    // use BulletText() or advance the cursor by GetTreeNodeToLabelSpacing() and call Text().
                    node_flags |= ImGuiTreeNodeFlags_Leaf | ImGuiTreeNodeFlags_NoTreePushOnOpen; // ImGuiTreeNodeFlags_Bullet
                    TreeNodeEx(i, node_flags, "Selectable Leaf {}", i);
                    if IsItemClicked() && !IsItemToggledOpen() {
                        node_clicked = i;}
                    if (test_drag_and_drop && BeginDragDropSource())
                    {
                        SetDragDropPayload("_TREENODE", None, 0);
                        Text("This is a drag and drop source");
                        EndDragDropSource();
                    }
                }
            }
            if (node_clicked != -1)
            {
                // Update selection state
                // (process outside of tree loop to avoid visual inconsistencies during the clicking frame)
                if (GetIO().KeyCtrl)
                    selection_mask ^= (1 << node_clicked);          // CTRL+click to toggle
                else //if (!(selection_mask & (1 << node_clicked))) // Depending on selection behavior you want, may want to preserve selection when clicking on item that is part of the selection
                    selection_mask = (1 << node_clicked);           // Click to single-select
            }
            if align_label_with_current_x_position {
                Indent(GetTreeNodeToLabelSpacing())(); }
            TreePop();
        }
        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/Collapsing Headers");
    if (TreeNode("Collapsing Headers"))
    {
        static let mut closable_group: bool =  true;
        Checkbox("Show 2nd header", &closable_group);
        if (CollapsingHeader("Header", ImGuiTreeNodeFlags_None))
        {
            Text("IsItemHovered: {}", IsItemHovered());
            for (let i: c_int = 0; i < 5; i++)
                Text("Some content {}", i);
        }
        if (CollapsingHeader("Header with a close button", &closable_group))
        {
            Text("IsItemHovered: {}", IsItemHovered());
            for (let i: c_int = 0; i < 5; i++)
                Text("More content {}", i);
        }
        /*
        if (CollapsingHeader("Header with a bullet", ImGuiTreeNodeFlags_Bullet))
            Text("IsItemHovered: {}", IsItemHovered());
        */
        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/Bullets");
    if (TreeNode("Bullets"))
    {
        BulletText("Bullet point 1");
        BulletText("Bullet point 2\nOn multiple lines");
        if (TreeNode("Tree node"))
        {
            BulletText("Another bullet point");
            TreePop();
        }
        Bullet(); Text("Bullet point 3 (two calls)");
        Bullet(); SmallButton("Button");
        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/Text");
    if (TreeNode("Text"))
    {
        IMGUI_DEMO_MARKER("Widgets/Text/Colored Text");
        if (TreeNode("Colorful Text"))
        {
            // Using shortcut. You can use PushStyleColor()/PopStyleColor() for more flexibility.
            TextColored(ImVec4(1.0, 0.0, 1.0, 1.0), "Pink");
            TextColored(ImVec4(1.0, 1.0, 0.0, 1.0), "Yellow");
            TextDisabled("Disabled");
            SameLine(); HelpMarker("The TextDisabled color is stored in ImGuiStyle.");
            TreePop();
        }

        IMGUI_DEMO_MARKER("Widgets/Text/Word Wrapping");
        if (TreeNode("Word Wrapping"))
        {
            // Using shortcut. You can use PushTextWrapPos()/PopTextWrapPos() for more flexibility.
            TextWrapped(
                "This text should automatically wrap on the edge of the window. The current implementation "
                "for text wrapping follows simple rules suitable for English and possibly other languages.");
            Spacing();

            static let wrap_width: c_float =  200;
            SliderFloat("Wrap width", &wrap_width, -20, 600, "{}f");

            draw_list: *mut ImDrawList = GetWindowDrawList();
            for (let n: c_int = 0; n < 2; n++)
            {
                Text("Test paragraph {}:", n);
                let pos: ImVec2 = GetCursorScreenPos();
                let marker_min: ImVec2 = ImVec2::new(pos.x + wrap_width, pos.y);
                let marker_max: ImVec2 = ImVec2::new(pos.x + wrap_width + 10, pos.y + GetTextLineHeight());
                PushTextWrapPos(GetCursorPos().x + wrap_width);
                if (n == 0)
                    Text("The lazy dog is a good dog. This paragraph should fit within {} pixels. Testing a 1 character word. The quick brown fox jumps over the lazy dog.", wrap_width);
                else
                    Text("aaaaaaaa bbbbbbbb, c cccccccc,dddddddd. d eeeeeeee   ffffffff. gggggggg!hhhhhhhh");

                // Draw actual text bounding box, following by marker of our expected limit (should not overlap!)
                draw_list.AddRect(GetItemRectMin(), GetItemRectMax(), IM_COL32(255, 255, 0, 255));
                draw_list.AddRectFilled(marker_min, marker_max, IM_COL32(255, 0, 255, 255));
                PopTextWrapPos();
            }

            TreePop();
        }

        IMGUI_DEMO_MARKER("Widgets/Text/UTF-8 Text");
        if (TreeNode("UTF-8 Text"))
        {
            // UTF-8 test with Japanese characters
            // (Needs a suitable font? Try "Google Noto" or "Arial Unicode". See docs/FONTS.md for details.)
            // - From C++11 you can use the u8"my text" syntax to encode literal strings as UTF-8
            // - For earlier compiler, you may be able to encode your sources as UTF-8 (e.g. in Visual Studio, you
            //   can save your source files as 'UTF-8 without signature').
            // - FOR THIS DEMO FILE ONLY, BECAUSE WE WANT TO SUPPORT OLD COMPILERS, WE ARE *NOT* INCLUDING RAW UTF-8
            //   CHARACTERS IN THIS SOURCE FILE. Instead we are encoding a few strings with hexadecimal constants.
            //   Don't do this in your application! Please use u8"text in any language" in your application!
            // Note that characters values are preserved even by InputText() if the font cannot be displayed,
            // so you can safely copy & paste garbled characters into another application.
            TextWrapped(
                "CJK text will only appears if the font was loaded with the appropriate CJK character ranges. "
                "Call io.Fonts.AddFontFromFileTTF() manually to load extra character ranges. "
                "Read docs/FONTS.md for details.");
            Text("Hiragana: \xe3\x81\x8b\xe3\x81\x8d\xe3\x81\x8f\xe3\x81\x91\xe3\x81\x93 (kakikukeko)"); // Normally we would use u8"blah blah" with the proper characters directly in the string.
            Text("Kanjis: \xe6\x97\xa5\xe6\x9c\xac\xe8\xaa\x9e (nihongo)");
            static buf: [c_char;32] = "\xe6\x97\xa5\xe6\x9c\xac\xe8\xaa\x9e";
            //static char buf[32] = u8"NIHONGO"; // <- this is how you would write it with C++11, using real kanjis
            InputText("UTF-8 input", buf, buf.len());
            TreePop();
        }
        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/Images");
    if (TreeNode("Images"))
    {
        ImGuiIO& io = GetIO();
        TextWrapped(
            "Below we are displaying the font texture (which is the only texture we have access to in this demo). "
            "Use the 'ImTextureID' type as storage to pass pointers or identifier to your own texture data. "
            "Hover the texture for a zoomed view!");

        // Below we are displaying the font texture because it is the only texture we have access to inside the demo!
        // Remember that is: ImTextureID just storage for whatever you want it to be. It is essentially a value that
        // will be passed to the rendering backend via the ImDrawCmd structure.
        // If you use one of the default imgui_impl_XXXX.cpp rendering backend, they all have comments at the top
        // of their respective source file to specify what they expect to be stored in ImTextureID, for example:
        // - The imgui_impl_dx11.cpp renderer expect a 'ID3D11ShaderResourceView*' pointer
        // - The imgui_impl_opengl3.cpp renderer expect a GLuint OpenGL texture identifier, etc.
        // More:
        // - If you decided that ImTextureID = MyEngineTexture*, then you can pass your MyEngineTexture* pointers
        //   to Image(), and gather width/height through your own functions, etc.
        // - You can use ShowMetricsWindow() to inspect the draw data that are being passed to your renderer,
        //   it will help you debug issues if you are confused about it.
        // - Consider using the lower-level ImDrawList::AddImage() API, via GetWindowDrawList()->AddImage().
        // - Read https://github.com/ocornut/imgui/blob/master/docs/FAQ.md
        // - Read https://github.com/ocornut/imgui/wiki/Image-Loading-and-Displaying-Examples
        let mut  my_tex_id: ImTextureID =  io.Fonts.TexID;
        let my_tex_w: c_float =  io.Fonts.TexWidth;
        let my_tex_h: c_float =  io.Fonts.TexHeight;
        {
            Text("{}fx{}f", my_tex_w, my_tex_h);
            let pos: ImVec2 = GetCursorScreenPos();
            let uv_min: ImVec2 = ImVec2::new(0.0, 0.0);                 // Top-left
            let uv_max: ImVec2 = ImVec2::new(1.0, 1.0);                 // Lower-right
            tint_col: ImVec4 = ImVec4(1.0, 1.0, 1.0, 1.0);   // No tint
            border_col: ImVec4 = ImVec4(1.0, 1.0, 1.0, 0.5); // 50% opaque white
            Image(my_tex_id, ImVec2::new(my_tex_w, my_tex_h), uv_min, uv_max, tint_col, border_col);
            if (IsItemHovered())
            {
                BeginTooltip();
                let region_sz: c_float =  32.0;
                let region_x: c_float =  io.MousePos.x - pos.x - region_sz * 0.5;
                let region_y: c_float =  io.MousePos.y - pos.y - region_sz * 0.5;
                let zoom: c_float =  4.0;
                if (region_x < 0.0) { region_x = 0.0; }
                else if (region_x > my_tex_w - region_sz) { region_x = my_tex_w - region_sz; }
                if (region_y < 0.0) { region_y = 0.0; }
                else if (region_y > my_tex_h - region_sz) { region_y = my_tex_h - region_sz; }
                Text("Min: ({}, {})", region_x, region_y);
                Text("Max: ({}, {})", region_x + region_sz, region_y + region_sz);
                let uv0: ImVec2 = ImVec2::new((region_x) / my_tex_w, (region_y) / my_tex_h);
                let uv1: ImVec2 = ImVec2::new((region_x + region_sz) / my_tex_w, (region_y + region_sz) / my_tex_h);
                Image(my_tex_id, ImVec2::new(region_sz * zoom, region_sz * zoom), uv0, uv1, tint_col, border_col);
                EndTooltip();
            }
        }

        IMGUI_DEMO_MARKER("Widgets/Images/Textured buttons");
        TextWrapped("And now some textured buttons..");
        static let pressed_count: c_int = 0;
        for (let i: c_int = 0; i < 8; i++)
        {
            // UV coordinates are often (0.0, 0.0) and (1.0, 1.0) to display an entire textures.
            // Here are trying to display only a 32x32 pixels area of the texture, hence the UV computation.
            // Read about UV coordinates here: https://github.com/ocornut/imgui/wiki/Image-Loading-and-Displaying-Examples
            PushID(i);
            if (i > 0)
                PushStyleVar(ImGuiStyleVar_FramePadding, ImVec2::new(i - 1.0, i - 1.0));
            let size: ImVec2 = ImVec2::new(32.0, 32.0);                         // Size of the image we want to make visible
            let uv0: ImVec2 = ImVec2::new(0.0, 0.0);                            // UV coordinates for lower-left
            let uv1: ImVec2 = ImVec2::new(32.0 / my_tex_w, 32.0 / my_tex_h);    // UV coordinates for (32,32) in our texture
            bg_col: ImVec4 = ImVec4(0.0, 0.0, 0.0, 1.0);             // Black background
            tint_col: ImVec4 = ImVec4(1.0, 1.0, 1.0, 1.0);           // No tint
            if (ImageButton("", my_tex_id, size, uv0, uv1, bg_col, tint_col))
                pressed_count += 1;
            if i > 0 {
                PopStyleVar(); }
            PopID();
            SameLine();
        }
        NewLine();
        Text("Pressed {} times.", pressed_count);
        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/Combo");
    if (TreeNode("Combo"))
    {
        // Expose flags as checkbox for the demo
        static flags: ImGuiComboFlags = 0;
        CheckboxFlags("ImGuiComboFlags_PopupAlignLeft", &flags, ImGuiComboFlags_PopupAlignLeft);
        SameLine(); HelpMarker("Only makes a difference if the popup is larger than the combo");
        if (CheckboxFlags("ImGuiComboFlags_NoArrowButton", &flags, ImGuiComboFlags_NoArrowButton))
            flags &= !ImGuiComboFlags_NoPreview;     // Clear the other flag, as we cannot combine both
        if (CheckboxFlags("ImGuiComboFlags_NoPreview", &flags, ImGuiComboFlags_NoPreview))
            flags &= !ImGuiComboFlags_NoArrowButton; // Clear the other flag, as we cannot combine both

        // Using the generic BeginCombo() API, you have full control over how to display the combo contents.
        // (your selection data could be an index, a pointer to the object, an id for the object, a flag intrusively
        // stored in the object itself, etc.)
        items: *const c_char[] = { "AAAA", "BBBB", "CCCC", "DDDD", "EEEE", "FFFF", "GGGG", "HHHH", "IIII", "JJJJ", "KKKK", "LLLLLLL", "MMMM", "OOOOOOO" };
        static let item_current_idx: c_int = 0; // Here we store our selection data as an index.
        let mut  combo_preview_value: *const c_char = items[item_current_idx];  // Pass in the preview value visible before opening the combo (it could be anything)
        if (BeginCombo("combo 1", combo_preview_value, flags))
        {
            for (let n: c_int = 0; n < items.len(); n++)
            {
                let is_selected: bool = (item_current_idx == n);
                if Selectable(items[n], is_selected) {
                    item_current_idx = n;}

                // Set the initial focus when opening the combo (scrolling + keyboard navigation focus)
                if is_selected {
                    SetItemDefaultFocus(); }
            }
            EndCombo();
        }

        // Simplified one-liner Combo() API, using values packed in a single constant string
        // This is a convenience for when the selection set is small and known at compile-time.
        static let item_current_2: c_int = 0;
        Combo("combo 2 (one-liner)", &item_current_2, "aaaa\0bbbb\0cccc\0dddd\0eeee\0\0");

        // Simplified one-liner Combo() using an array of const char*
        // This is not very useful (may obsolete): prefer using BeginCombo()/EndCombo() for full control.
        static let item_current_3: c_int = -1; // If the selection isn't within 0..count, Combo won't display a preview
        Combo("combo 3 (array)", &item_current_3, items, items.len());

        // Simplified one-liner Combo() using an accessor function
        struct Funcs { static ItemGetter: bool(data: *mut c_void, n: c_int, *out_str: *const c_char) { *out_str = ((**const char)data)[n]; return true; } };
        static let item_current_4: c_int = 0;
        Combo("combo 4 (function)", &item_current_4, &Funcs::ItemGetter, items, items.len());

        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/List Boxes");
    if (TreeNode("List boxes"))
    {
        // Using the generic BeginListBox() API, you have full control over how to display the combo contents.
        // (your selection data could be an index, a pointer to the object, an id for the object, a flag intrusively
        // stored in the object itself, etc.)
        items: *const c_char[] = { "AAAA", "BBBB", "CCCC", "DDDD", "EEEE", "FFFF", "GGGG", "HHHH", "IIII", "JJJJ", "KKKK", "LLLLLLL", "MMMM", "OOOOOOO" };
        static let item_current_idx: c_int = 0; // Here we store our selection data as an index.
        if (BeginListBox("listbox 1"))
        {
            for (let n: c_int = 0; n < items.len(); n++)
            {
                let is_selected: bool = (item_current_idx == n);
                if Selectable(items[n], is_selected) {
                    item_current_idx = n;}

                // Set the initial focus when opening the combo (scrolling + keyboard navigation focus)
                if is_selected {
                    SetItemDefaultFocus(); }
            }
            EndListBox();
        }

        // Custom size: use all width, 5 items tall
        Text("Full-width:");
        if (BeginListBox("##listbox 2", ImVec2::new(-FLT_MIN, 5 * GetTextLineHeightWithSpacing())))
        {
            for (let n: c_int = 0; n < items.len(); n++)
            {
                let is_selected: bool = (item_current_idx == n);
                if Selectable(items[n], is_selected) {
                    item_current_idx = n;}

                // Set the initial focus when opening the combo (scrolling + keyboard navigation focus)
                if is_selected {
                    SetItemDefaultFocus(); }
            }
            EndListBox();
        }

        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/Selectables");
    if (TreeNode("Selectables"))
    {
        // Selectable() has 2 overloads:
        // - The one taking "selected: bool" as a read-only selection information.
        //   When Selectable() has been clicked it returns true and you can alter selection state accordingly.
        // - The one taking "bool* p_selected" as a read-write selection information (convenient in some cases)
        // The earlier is more flexible, as in real application your selection may be stored in many different ways
        // and not necessarily inside a value: bool (e.g. in flags within objects, as an external list, etc).
        IMGUI_DEMO_MARKER("Widgets/Selectables/Basic");
        if (TreeNode("Basic"))
        {
            static selection: bool[5] = { false, true, false, false, false };
            Selectable("1. I am selectable", &selection[0]);
            Selectable("2. I am selectable", &selection[1]);
            Text("(I am not selectable)");
            Selectable("4. I am selectable", &selection[3]);
            if (Selectable("5. I am double clickable", selection[4], ImGuiSelectableFlags_AllowDoubleClick))
                if (IsMouseDoubleClicked(0))
                    selection[4] = !selection[4];
            TreePop();
        }
        IMGUI_DEMO_MARKER("Widgets/Selectables/Single Selection");
        if (TreeNode("Selection State: Single Selection"))
        {
            static let selected: c_int = -1;
            for (let n: c_int = 0; n < 5; n++)
            {
                buf: [c_char;32];
                sprintf(buf, "Object {}", n);
                if Selectable(buf, selected == n) {
                    selected = n;}
            }
            TreePop();
        }
        IMGUI_DEMO_MARKER("Widgets/Selectables/Multiple Selection");
        if (TreeNode("Selection State: Multiple Selection"))
        {
            HelpMarker("Hold CTRL and click to select multiple items.");
            static selection: bool[5] = { false, false, false, false, false };
            for (let n: c_int = 0; n < 5; n++)
            {
                buf: [c_char;32];
                sprintf(buf, "Object {}", n);
                if (Selectable(buf, selection[n]))
                {
                    if (!GetIO().KeyCtrl)    // Clear selection when CTRL is not held
                        memset(selection, 0, sizeof(selection));
                    selection[n] ^= 1;
                }
            }
            TreePop();
        }
        IMGUI_DEMO_MARKER("Widgets/Selectables/Rendering more text into the same line");
        if (TreeNode("Rendering more text into the same line"))
        {
            // Using the Selectable() override that takes "bool* p_selected" parameter,
            // this function toggle your value: bool automatically.
            static selected: bool[3] = { false, false, false };
            Selectable("main.c",    &selected[0]); SameLine(300); Text(" 2,345 bytes");
            Selectable("Hello.cpp", &selected[1]); SameLine(300); Text("12,345 bytes");
            Selectable("Hello.h",   &selected[2]); SameLine(300); Text(" 2,345 bytes");
            TreePop();
        }
        IMGUI_DEMO_MARKER("Widgets/Selectables/In columns");
        if (TreeNode("In columns"))
        {
            static selected: bool[10] = {};

            if (BeginTable("split1", 3, ImGuiTableFlags_Resizable | ImGuiTableFlags_NoSavedSettings | ImGuiTableFlags_Borders))
            {
                for (let i: c_int = 0; i < 10; i++)
                {
                    label: [c_char;32];
                    sprintf(label, "Item {}", i);
                    TableNextColumn();
                    Selectable(label, &selected[i]); // FIXME-TABLE: Selection overlap
                }
                EndTable();
            }
            Spacing();
            if (BeginTable("split2", 3, ImGuiTableFlags_Resizable | ImGuiTableFlags_NoSavedSettings | ImGuiTableFlags_Borders))
            {
                for (let i: c_int = 0; i < 10; i++)
                {
                    label: [c_char;32];
                    sprintf(label, "Item {}", i);
                    TableNextRow();
                    TableNextColumn();
                    Selectable(label, &selected[i], ImGuiSelectableFlags_SpanAllColumns);
                    TableNextColumn();
                    Text("Some other contents");
                    TableNextColumn();
                    Text("123456");
                }
                EndTable();
            }
            TreePop();
        }
        IMGUI_DEMO_MARKER("Widgets/Selectables/Grid");
        if (TreeNode("Grid"))
        {
            static selected: [c_char;4][4] = { { 1, 0, 0, 0 }, { 0, 1, 0, 0 }, { 0, 0, 1, 0 }, { 0, 0, 0, 1 } };

            // Add in a bit of silly fun...
            let time: c_float =  GetTime();
            let winning_state: bool = memchr(selected, 0, sizeof(selected)) == None; // If all cells are selected...
            if (winning_state)
                PushStyleVar(ImGuiStyleVar_SelectableTextAlign, ImVec2::new(0.5 + 0.5 * cosf(time * 2.0), 0.5 + 0.5 * sinf(time * 3.0)));

            for (let y: c_int = 0; y < 4; y++)
                for (let x: c_int = 0; x < 4; x++)
                {
                    if x > 0 {
                        SameLine(); }
                    PushID(y * 4 + x);
                    if (Selectable("Sailor", selected[y][x] != 0, 0, ImVec2::new(50, 50)))
                    {
                        // Toggle clicked cell + toggle neighbors
                        selected[y][x] ^= 1;
                        if (x > 0) { selected[y][x - 1] ^= 1; }
                        if (x < 3) { selected[y][x + 1] ^= 1; }
                        if (y > 0) { selected[y - 1][x] ^= 1; }
                        if (y < 3) { selected[y + 1][x] ^= 1; }
                    }
                    PopID();
                }

            if winning_state {
                PopStyleVar(); }
            TreePop();
        }
        IMGUI_DEMO_MARKER("Widgets/Selectables/Alignment");
        if (TreeNode("Alignment"))
        {
            HelpMarker(
                "By default, Selectables uses style.SelectableTextAlign but it can be overridden on a per-item "
                "basis using PushStyleVar(). You'll probably want to always keep your default situation to "
                "left-align otherwise it becomes difficult to layout multiple items on a same line");
            static selected: bool[3 * 3] = { true, false, true, false, true, false, true, false, true };
            for (let y: c_int = 0; y < 3; y++)
            {
                for (let x: c_int = 0; x < 3; x++)
                {
                    let alignment: ImVec2 = ImVec2::new(x / 2.0, y / 2.0);
                    name: [c_char;32];
                    sprintf(name, "({},{})", alignment.x, alignment.y);
                    if x > 0 {  SameLine(); }
                    PushStyleVar(ImGuiStyleVar_SelectableTextAlign, alignment);
                    Selectable(name, &selected[3 * y + x], ImGuiSelectableFlags_None, ImVec2::new(80, 80));
                    PopStyleVar();
                }
            }
            TreePop();
        }
        TreePop();
    }

    // To wire InputText() with std::string or any other custom string type,
    // see the "Text Input > Resize Callback" section of this demo, and the misc/cpp/imgui_stdlib.h file.
    IMGUI_DEMO_MARKER("Widgets/Text Input");
    if (TreeNode("Text Input"))
    {
        IMGUI_DEMO_MARKER("Widgets/Text Input/Multi-line Text Input");
        if (TreeNode("Multi-line Text Input"))
        {
            // Note: we are using a fixed-sized buffer for simplicity here. See ImGuiInputTextFlags_CallbackResize
            // and the code in misc/cpp/imgui_stdlib.h for how to setup InputText() for dynamically resizing strings.
            static  text: c_char[1024 * 16] =
                "/*\n"
                " The Pentium F00 bug, shorthand for F0 0.0 C7 C8,\n"
                " the hexadecimal encoding of one offending instruction,\n"
                " more formally, the invalid operand with locked CMPXCHG8B\n"
                " instruction bug, is a design flaw in the majority of\n"
                " Intel Pentium, Pentium MMX, and Pentium OverDrive\n"
                " processors (all in the P5 microarchitecture).\n"
                "*/\n\n"
                "label:\n"
                "\tlock cmpxchg8b eax\n";

            static flags: ImGuiInputTextFlags = ImGuiInputTextFlags_AllowTabInput;
            HelpMarker("You can use the ImGuiInputTextFlags_CallbackResize facility if you need to wire InputTextMultiline() to a dynamic string type. See misc/cpp/imgui_stdlib.h for an example. (This is not demonstrated in imgui_demo.cpp because we don't want to include <string> in here)");
            CheckboxFlags("ImGuiInputTextFlags_ReadOnly", &flags, ImGuiInputTextFlags_ReadOnly);
            CheckboxFlags("ImGuiInputTextFlags_AllowTabInput", &flags, ImGuiInputTextFlags_AllowTabInput);
            CheckboxFlags("ImGuiInputTextFlags_CtrlEnterForNewLine", &flags, ImGuiInputTextFlags_CtrlEnterForNewLine);
            InputTextMultiline("##source", text, text.len(), ImVec2::new(-FLT_MIN, GetTextLineHeight() * 16), flags);
            TreePop();
        }

        IMGUI_DEMO_MARKER("Widgets/Text Input/Filtered Text Input");
        if (TreeNode("Filtered Text Input"))
        {
            struct TextFilters
            {
                // Return 0 (pass) if the character is 'i' or 'm' or 'g' or 'u' or 'i'
                pub fn FilterImGuiLetters(*mut data: ImGuiInputTextCallbackData) -> c_int
                {
                    if data.EventChar < 256 && strchr("imgui", data.EventChar) { return  0; }
                    return 1;
                }
            };

            static buf1: [c_char;64] = ""; InputText("default",     buf1, 64);
            static buf2: [c_char;64] = ""; InputText("decimal",     buf2, 64, ImGuiInputTextFlags_CharsDecimal);
            static buf3: [c_char;64] = ""; InputText("hexadecimal", buf3, 64, ImGuiInputTextFlags_CharsHexadecimal | ImGuiInputTextFlags_CharsUppercase);
            static buf4: [c_char;64] = ""; InputText("uppercase",   buf4, 64, ImGuiInputTextFlags_CharsUppercase);
            static buf5: [c_char;64] = ""; InputText("no blank",    buf5, 64, ImGuiInputTextFlags_CharsNoBlank);
            static buf6: [c_char;64] = ""; InputText("\"imgui\" letters", buf6, 64, ImGuiInputTextFlags_CallbackCharFilter, TextFilters::FilterImGuiLetters);
            TreePop();
        }

        IMGUI_DEMO_MARKER("Widgets/Text Input/Password input");
        if (TreeNode("Password Input"))
        {
            static password: [c_char;64] = "password123";
            InputText("password", password, password.len(), ImGuiInputTextFlags_Password);
            SameLine(); HelpMarker("Display all characters as '*'.\nDisable clipboard cut and copy.\nDisable logging.\n");
            InputTextWithHint("password (w/ hint)", "<password>", password, password.len(), ImGuiInputTextFlags_Password);
            InputText("password (clear)", password, password.len());
            TreePop();
        }

        if (TreeNode("Completion, History, Edit Callbacks"))
        {
            struct Funcs
            {
                pub fn MyCallback(*mut data: ImGuiInputTextCallbackData) -> c_int
                {
                    if (data.EventFlag == ImGuiInputTextFlags_CallbackCompletion)
                    {
                        data.InsertChars(data.CursorPos, "..");
                    }
                    else if (data.EventFlag == ImGuiInputTextFlags_CallbackHistory)
                    {
                        if (data.EventKey == ImGuiKey_UpArrow)
                        {
                            data.DeleteChars(0, data.BufTextLen);
                            data.InsertChars(0, "Pressed Up!");
                            data.SelectAll();
                        }
                        else if (data.EventKey == ImGuiKey_DownArrow)
                        {
                            data.DeleteChars(0, data.BufTextLen);
                            data.InsertChars(0, "Pressed Down!");
                            data.SelectAll();
                        }
                    }
                    else if (data.EventFlag == ImGuiInputTextFlags_CallbackEdit)
                    {
                        // Toggle casing of first character
                         c: c_char = data.Buf[0];
                        if ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) data.Buf[0] ^= 32;
                        data.BufDirty = true;

                        // Increment a counter
                        *mut let p_int: c_int = (*mut c_int)data.UserData;
                        *p_int = *p_int + 1;
                    }
                    return 0;
                }
            };
            static buf1: [c_char;64];
            InputText("Completion", buf1, 64, ImGuiInputTextFlags_CallbackCompletion, Funcs::MyCallback);
            SameLine(); HelpMarker("Here we append \"..\" each time Tab is pressed. See 'Examples>Console' for a more meaningful demonstration of using this callback.");

            static buf2: [c_char;64];
            InputText("History", buf2, 64, ImGuiInputTextFlags_CallbackHistory, Funcs::MyCallback);
            SameLine(); HelpMarker("Here we replace and select text each time Up/Down are pressed. See 'Examples>Console' for a more meaningful demonstration of using this callback.");

            static buf3: [c_char;64];
            static let edit_count: c_int = 0;
            InputText("Edit", buf3, 64, ImGuiInputTextFlags_CallbackEdit, Funcs::MyCallback, &edit_count);
            SameLine(); HelpMarker("Here we toggle the casing of the first character on every edits + count edits.");
            SameLine(); Text("({})", edit_count);

            TreePop();
        }

        IMGUI_DEMO_MARKER("Widgets/Text Input/Resize Callback");
        if (TreeNode("Resize Callback"))
        {
            // To wire InputText() with std::string or any other custom string type,
            // you can use the ImGuiInputTextFlags_CallbackResize flag + create a custom InputText() wrapper
            // using your preferred type. See misc/cpp/imgui_stdlib.h for an implementation of this using std::string.
            HelpMarker(
                "Using ImGuiInputTextFlags_CallbackResize to wire your custom string type to InputText().\n\n"
                "See misc/cpp/imgui_stdlib.h for an implementation of this for std::string.");
            struct Funcs
            {
                pub fn MyResizeCallback(*mut data: ImGuiInputTextCallbackData) -> c_int
                {
                    if (data.EventFlag == ImGuiInputTextFlags_CallbackResize)
                    {
                        Vec<char>* my_str = (Vec<char>*)data.UserData;
                        // IM_ASSERT(my_str.begin() == data->Bu0f32);
                        my_str.resize(data.BufSize); // NB: On resizing calls, generally data->BufSize == data->BufTextLen + 1
                        data.Buf = my_str.begin();
                    }
                    return 0;
                }

                // Note: Because  is a namespace you would typically add your own function into the namespace.
                // For example, you code may declare a function 'InputText(const char* label, MyString* my_str)'
                static MyInputTextMultiline: bool(label: *const c_char, Vec<char>* my_str, size: &ImVec2 = ImVec2::new(0, 0), flags: ImGuiInputTextFlags = 0)
                {
                    // IM_ASSERT(flag_set(flags, ImGuiInputTextFlags_CallbackResize) == 0);
                    return InputTextMultiline(label, my_str.begin(), my_str.size(), size, flags | ImGuiInputTextFlags_CallbackResize, Funcs::MyResizeCallback, my_str);
                }
            };

            // For this demo we are using ImVector as a string container.
            // Note that because we need to store a terminating zero character, our size/capacity are 1 more
            // than usually reported by a typical string class.
            static Vec<char> my_str;
            if my_str.empty(){
                my_str.push(0);}
            Funcs::MyInputTextMultiline("##MyStr", &my_str, ImVec2::new(-FLT_MIN, GetTextLineHeight() * 16));
            Text("Data: {}\nSize: {}\nCapacity: {}", my_str.begin(), my_str.size(), my_str.capacity());
            TreePop();
        }

        TreePop();
    }

    // Tabs
    IMGUI_DEMO_MARKER("Widgets/Tabs");
    if (TreeNode("Tabs"))
    {
        IMGUI_DEMO_MARKER("Widgets/Tabs/Basic");
        if (TreeNode("Basic"))
        {
            tab_bar_flags: ImGuiTabBarFlags = ImGuiTabBarFlags_None;
            if (BeginTabBar("MyTabBar", tab_bar_flags))
            {
                if (BeginTabItem("Avocado"))
                {
                    Text("This is the Avocado tab!\nblah blah blah blah blah");
                    EndTabItem();
                }
                if (BeginTabItem("Broccoli"))
                {
                    Text("This is the Broccoli tab!\nblah blah blah blah blah");
                    EndTabItem();
                }
                if (BeginTabItem("Cucumber"))
                {
                    Text("This is the Cucumber tab!\nblah blah blah blah blah");
                    EndTabItem();
                }
                EndTabBar();
            }
            Separator();
            TreePop();
        }

        IMGUI_DEMO_MARKER("Widgets/Tabs/Advanced & Close Button");
        if (TreeNode("Advanced & Close Button"))
        {
            // Expose a couple of the available flags. In most cases you may just call BeginTabBar() with no flags (0).
            static tab_bar_flags: ImGuiTabBarFlags = ImGuiTabBarFlags_Reorderable;
            CheckboxFlags("ImGuiTabBarFlags_Reorderable", &tab_bar_flags, ImGuiTabBarFlags_Reorderable);
            CheckboxFlags("ImGuiTabBarFlags_AutoSelectNewTabs", &tab_bar_flags, ImGuiTabBarFlags_AutoSelectNewTabs);
            CheckboxFlags("ImGuiTabBarFlags_TabListPopupButton", &tab_bar_flags, ImGuiTabBarFlags_TabListPopupButton);
            CheckboxFlags("ImGuiTabBarFlags_NoCloseWithMiddleMouseButton", &tab_bar_flags, ImGuiTabBarFlags_NoCloseWithMiddleMouseButton);
            if ((tab_bar_flags & ImGuiTabBarFlags_FittingPolicyMask_) == 0)
                tab_bar_flags |= ImGuiTabBarFlags_FittingPolicyDefault_;
            if (CheckboxFlags("ImGuiTabBarFlags_FittingPolicyResizeDown", &tab_bar_flags, ImGuiTabBarFlags_FittingPolicyResizeDown))
                tab_bar_flags &= ~(ImGuiTabBarFlags_FittingPolicyMask_ ^ ImGuiTabBarFlags_FittingPolicyResizeDown);
            if (CheckboxFlags("ImGuiTabBarFlags_FittingPolicyScroll", &tab_bar_flags, ImGuiTabBarFlags_FittingPolicyScroll))
                tab_bar_flags &= ~(ImGuiTabBarFlags_FittingPolicyMask_ ^ ImGuiTabBarFlags_FittingPolicyScroll);

            // Tab Bar
            *const names: [c_char;4] = { "Artichoke", "Beetroot", "Celery", "Daikon" };
            static opened: bool[4] = { true, true, true, true }; // Persistent user state
            for (let n: c_int = 0; n < opened.len(); n++)
            {
                if (n > 0) { SameLine(); }
                Checkbox(names[n], &opened[n]);
            }

            // Passing ato: *mut bool BeginTabItem() is similar to passing one to Begin():
            // the underlying will: bool be set to false when the tab is closed.
            if (BeginTabBar("MyTabBar", tab_bar_flags))
            {
                for (let n: c_int = 0; n < opened.len(); n++)
                    if (opened[n] && BeginTabItem(names[n], &opened[n], ImGuiTabItemFlags_None))
                    {
                        Text("This is the {} tab!", names[n]);
                        if n & 1{
                            Text("I am an odd tab.");}
                        EndTabItem();
                    }
                EndTabBar();
            }
            Separator();
            TreePop();
        }

        IMGUI_DEMO_MARKER("Widgets/Tabs/TabItemButton & Leading-Trailing flags");
        if (TreeNode("TabItemButton & Leading/Trailing flags"))
        {
            static Vec<c_int> active_tabs;
            static let next_tab_id: c_int = 0;
            if (next_tab_id == 0) // initialize with some default tabs
                for (let i: c_int = 0; i < 3; i++)
                    active_tabs.push(next_tab_id++);

            // TabItemButton() and Leading/Trailing flags are distinct features which we will demo together.
            // (It is possible to submit regular tabs with Leading/Trailing flags, or TabItemButton tabs without Leading/Trailing flags...
            // but they tend to make more sense together)
            static let mut show_leading_button: bool =  true;
            static let mut show_trailing_button: bool =  true;
            Checkbox("Show Leading TabItemButton()", &show_leading_button);
            Checkbox("Show Trailing TabItemButton()", &show_trailing_button);

            // Expose some other flags which are useful to showcase how they interact with Leading/Trailing tabs
            static tab_bar_flags: ImGuiTabBarFlags = ImGuiTabBarFlags_AutoSelectNewTabs | ImGuiTabBarFlags_Reorderable | ImGuiTabBarFlags_FittingPolicyResizeDown;
            CheckboxFlags("ImGuiTabBarFlags_TabListPopupButton", &tab_bar_flags, ImGuiTabBarFlags_TabListPopupButton);
            if (CheckboxFlags("ImGuiTabBarFlags_FittingPolicyResizeDown", &tab_bar_flags, ImGuiTabBarFlags_FittingPolicyResizeDown))
                tab_bar_flags &= ~(ImGuiTabBarFlags_FittingPolicyMask_ ^ ImGuiTabBarFlags_FittingPolicyResizeDown);
            if (CheckboxFlags("ImGuiTabBarFlags_FittingPolicyScroll", &tab_bar_flags, ImGuiTabBarFlags_FittingPolicyScroll))
                tab_bar_flags &= ~(ImGuiTabBarFlags_FittingPolicyMask_ ^ ImGuiTabBarFlags_FittingPolicyScroll);

            if (BeginTabBar("MyTabBar", tab_bar_flags))
            {
                // Demo a Leading TabItemButton(): click the "?" button to open a menu
                if (show_leading_button)
                    if (TabItemButton("?", ImGuiTabItemFlags_Leading | ImGuiTabItemFlags_NoTooltip))
                        OpenPopup("MyHelpMenu");
                if (BeginPopup("MyHelpMenu"))
                {
                    Selectable("Hello!");
                    EndPopup();
                }

                // Demo Trailing Tabs: click the "+" button to add a new tab (in your app you may want to use a font icon instead of the "+")
                // Note that we submit it before the regular tabs, but because of the ImGuiTabItemFlags_Trailing flag it will always appear at the end.
                if (show_trailing_button)
                    if (TabItemButton("+", ImGuiTabItemFlags_Trailing | ImGuiTabItemFlags_NoTooltip))
                        active_tabs.push(next_tab_id++); // Add new tab

                // Submit our regular tabs
                for (let n: c_int = 0; n < active_tabs.Size; )
                {
                    let mut open: bool =  true;
                    name: [c_char;16];
                    snprintf(name, name.len(), "%04d", active_tabs[n]);
                    if (BeginTabItem(name, &open, ImGuiTabItemFlags_None))
                    {
                        Text("This is the {} tab!", name);
                        EndTabItem();
                    }

                    if (!open)
                        active_tabs.erase(active_tabs.Data + n);
                    else
                        n+= 1;
                }

                EndTabBar();
            }
            Separator();
            TreePop();
        }
        TreePop();
    }

    // Plot/Graph widgets are not very good.
    // Consider using a third-party library such as ImPlot: https://github.com/epezent/implot
    // (see others https://github.com/ocornut/imgui/wiki/Useful-Extensions)
    IMGUI_DEMO_MARKER("Widgets/Plotting");
    if (TreeNode("Plotting"))
    {
        static let mut animate: bool =  true;
        Checkbox("Animate", &animate);

        // Plot as lines and plot as histogram
        IMGUI_DEMO_MARKER("Widgets/Plotting/PlotLines, PlotHistogram");
        staticarr: c_float[] = { 0.6f, 0.1f, 1.0, 0.5, 0.92f, 0.1f, 0.2f };
        PlotLines("Frame Times", arr, arr.len());
        PlotHistogram("Histogram", arr, arr.len(), 0, None, 0.0, 1.0, ImVec2::new(0, 80f32));

        // Fill an array of contiguous float values to plot
        // Tip: If your float aren't contiguous but part of a structure, you can pass a pointer to your first float
        // and the sizeof() of your structure in the "stride" parameter.
        staticvalues: c_float[90] = {};
        static let values_offset: c_int = 0;
        static double refresh_time = 0.0;
        if (!animate || refresh_time == 0.0)
            refresh_time = GetTime();
        while (refresh_time < GetTime()) // Create data at fixed 60 Hz rate for the demo
        {
            static let phase: c_float =  0.0;
            values[values_offset] = cosf(phase);
            values_offset = (values_offset + 1) % values.len();
            phase += 0.1.0 * values_offset;
            refresh_time += 1.0 / 60f32;
        }

        // Plots can display overlay texts
        // (in this example, we will display an average value)
        {
            let average: c_float =  0.0;
            for (let n: c_int = 0; n < values.len(); n++)
                average += values[n];
            average /= values.len();
            overlay: [c_char;32];
            sprintf(overlay, "avg {}", average);
            PlotLines("Lines", values, values.len(), values_offset, overlay, -1.0, 1.0, ImVec2::new(0, 80f32));
        }

        // Use functions to generate output
        // FIXME: This is rather awkward because current plot API only pass in indices.
        // We probably want an API passing floats and user provide sample rate/count.
        struct Funcs
        {
            staticSin: c_float(*mut c_void, i: c_int) { return sinf(i * 0.1.0); }
            staticSaw: c_float(*mut c_void, i: c_int) { return if (i & 1) { 1.0} else {- 1.0}; }
        };
        static let func_type: c_int = 0, display_count = 70;
        Separator();
        SetNextItemWidth(GetFontSize() * 8);
        Combo("func", &func_type, "Sin\0Saw\0");
        SameLine();
        SliderInt("Sample count", &display_count, 1, 400);
        c_float (*func)(*mut c_void, c_int) = if (func_type == 0) { Funcs::Sin} else {Funcs::Saw};
        PlotLines("Lines", func, None, display_count, 0, None, -1.0, 1.0, ImVec2::new(0, 80));
        PlotHistogram("Histogram", func, None, display_count, 0, None, -1.0, 1.0, ImVec2::new(0, 80));
        Separator();

        // Animate a simple progress bar
        IMGUI_DEMO_MARKER("Widgets/Plotting/ProgressBar");
        static let progress: c_float =  0.0, progress_dir = 1.0;
        if (animate)
        {
            progress += progress_dir * 0.4f * GetIO().DeltaTime;
            if (progress >= 1.10.0) { progress = 1.1f; progress_dir *= -1.0; }
            if (progress <= -0.1.0) { progress = -0.1f; progress_dir *= -1.0; }
        }

        // Typically we would use ImVec2::new(-1.0,0.0) or ImVec2::new(-FLT_MIN,0.0) to use all available width,
        // or ImVec2::new(width,0.0) for a specified width. ImVec2::new(0.0,0.0) uses ItemWidth.
        ProgressBar(progress, ImVec2::new(0.0, 0.0));
        SameLine(0.0, GetStyle().ItemInnerSpacing.x);
        Text("Progress Bar");

        let progress_saturated: c_float =  IM_CLAMP(progress, 0.0, 1.0);
        buf: [c_char;32];
        sprintf(buf, "{}/{}", (progress_saturated * 1753), 1753);
        ProgressBar(progress, ImVec2::new(0.f, 0.0), buf);
        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/Color");
    if (TreeNode("Color/Picker Widgets"))
    {
        static color: ImVec4 = ImVec4(114.0 / 255f32, 144.0 / 255f32, 154.0 / 255f32, 200 / 255.0);

        static let mut alpha_preview: bool =  true;
        static let mut alpha_half_preview: bool =  false;
        static let mut drag_and_drop: bool =  true;
        static let mut options_menu: bool =  true;
        static let mut hdr: bool =  false;
        Checkbox("With Alpha Preview", &alpha_preview);
        Checkbox("With Half Alpha Preview", &alpha_half_preview);
        Checkbox("With Drag and Drop", &drag_and_drop);
        Checkbox("With Options Menu", &options_menu); SameLine(); HelpMarker("Right-click on the individual color widget to show options.");
        Checkbox("With HDR", &hdr); SameLine(); HelpMarker("Currently all this does is to lift the 0..1 limits on dragging widgets.");
        misc_flags: ImGuiColorEditFlags = (if hdr {ImGuiColorEditFlags_HDR} else { 0 }) | (if drag_and_drop {0} else { ImGuiColorEditFlags_NoDragDrop }) | (if alpha_half_preview { ImGuiColorEditFlags_AlphaPreviewHalf} else {if alpha_preview {ImGuiColorEditFlags_AlphaPreview} else { 0 }}) | (if options_menu {0} else { ImGuiColorEditFlags_NoOptions });

        IMGUI_DEMO_MARKER("Widgets/Color/ColorEdit");
        Text("Color widget:");
        SameLine(); HelpMarker(
            "Click on the color square to open a color picker.\n"
            "CTRL+click on individual component to input value.\n");
        ColorEdit3("MyColor##1", (&mut c_float)&color, misc_flags);

        IMGUI_DEMO_MARKER("Widgets/Color/ColorEdit (HSV, with Alpha)");
        Text("Color widget HSV with Alpha:");
        ColorEdit4("MyColor##2", (&mut c_float)&color, ImGuiColorEditFlags_DisplayHSV | misc_flags);

        IMGUI_DEMO_MARKER("Widgets/Color/ColorEdit (float display)");
        Text("Color widget with Float Display:");
        ColorEdit4("MyColor##2f", (&mut c_float)&color, ImGuiColorEditFlags_Float | misc_flags);

        IMGUI_DEMO_MARKER("Widgets/Color/ColorButton (with Picker)");
        Text("Color button with Picker:");
        SameLine(); HelpMarker(
            "With the ImGuiColorEditFlags_NoInputs flag you can hide all the slider/text inputs.\n"
            "With the ImGuiColorEditFlags_NoLabel flag you can pass a non-empty label which will only "
            "be used for the tooltip and picker popup.");
        ColorEdit4("MyColor##3", (&mut c_float)&color, ImGuiColorEditFlags_NoInputs | ImGuiColorEditFlags_NoLabel | misc_flags);

        IMGUI_DEMO_MARKER("Widgets/Color/ColorButton (with custom Picker popup)");
        Text("Color button with Custom Picker Popup:");

        // Generate a default palette. The palette will persist and can be edited.
        static let mut saved_palette_init: bool =  true;
        static saved_palette: ImVec4[32] = {};
        if (saved_palette_init)
        {
            for (let n: c_int = 0; n < saved_palette.len(); n++)
            {
                ColorConvertHSVtoRGB(n / 31f32, 0.8f, 0.8f,
                    saved_palette[n].x, saved_palette[n].y, saved_palette[n].z);
                saved_palette[n].w = 1.0; // Alpha
            }
            saved_palette_init = false;
        }

        static backup_color: ImVec4;
        let mut open_popup: bool =  ColorButton("MyColor##3b", color, misc_flags);
        SameLine(0, GetStyle().ItemInnerSpacing.x);
        open_popup |= Button("Palette");
        if (open_popup)
        {
            OpenPopup("mypicker");
            backup_color = color;
        }
        if (BeginPopup("mypicker"))
        {
            Text("MY CUSTOM COLOR PICKER WITH AN AMAZING PALETTE!");
            Separator();
            ColorPicker4("##picker", (&mut c_float)&color, misc_flags | ImGuiColorEditFlags_NoSidePreview | ImGuiColorEditFlags_NoSmallPreview);
            SameLine();

            BeginGroup(); // Lock X position
            Text("Current");
            ColorButton("##current", color, ImGuiColorEditFlags_NoPicker | ImGuiColorEditFlags_AlphaPreviewHalf, ImVec2::new(60, 40));
            Text("Previous");
            if ColorButton("##previous", backup_color, ImGuiColorEditFlags_NoPicker | ImGuiColorEditFlags_AlphaPreviewHalf, ImVec2::new(60, 40)) {
                color = backup_color;}
            Separator();
            Text("Palette");
            for (let n: c_int = 0; n < saved_palette.len(); n++)
            {
                PushID(n);
                if ((n % 8) != 0)
                    SameLine(0.0, GetStyle().ItemSpacing.y);

                palette_button_flags: ImGuiColorEditFlags = ImGuiColorEditFlags_NoAlpha | ImGuiColorEditFlags_NoPicker | ImGuiColorEditFlags_NoTooltip;
                if (ColorButton("##palette", saved_palette[n], palette_button_flags, ImVec2::new(20, 20)))
                    color = ImVec4(saved_palette[n].x, saved_palette[n].y, saved_palette[n].z, color.w); // Preserve alpha!

                // Allow user to drop colors into each palette entry. Note that ColorButton() is already a
                // drag source by default, unless specifying the ImGuiColorEditFlags_NoDragDrop flag.
                if (BeginDragDropTarget())
                {
                    if (*const ImGuiPayload payload = AcceptDragDropPayload(IMGUI_PAYLOAD_TYPE_COLOR_30f32))
                        memcpy((&mut c_float)&saved_palette[n], payload.Data, sizeof * 3);
                    if (*const ImGuiPayload payload = AcceptDragDropPayload(IMGUI_PAYLOAD_TYPE_COLOR_40f32))
                        memcpy((&mut c_float)&saved_palette[n], payload.Data, sizeof * 4);
                    EndDragDropTarget();
                }

                PopID();
            }
            EndGroup();
            EndPopup();
        }

        IMGUI_DEMO_MARKER("Widgets/Color/ColorButton (simple)");
        Text("Color button only:");
        static let mut no_border: bool =  false;
        Checkbox("ImGuiColorEditFlags_NoBorder", &no_border);
        ColorButton("MyColor##3c", *(*mut ImVec4)&color, misc_flags | (if no_border { ImGuiColorEditFlags_NoBorder }else {0}), ImVec2::new(80, 80));

        IMGUI_DEMO_MARKER("Widgets/Color/ColorPicker");
        Text("Color picker:");
        static let mut alpha: bool =  true;
        static let mut alpha_bar: bool =  true;
        static let mut side_preview: bool =  true;
        static let mut ref_color: bool =  false;
        static ref_color_v: ImVec4(1.0, 0.0, 1.0, 0.5);
        static let display_mode: c_int = 0;
        static let picker_mode: c_int = 0;
        Checkbox("With Alpha", &alpha);
        Checkbox("With Alpha Bar", &alpha_bar);
        Checkbox("With Side Preview", &side_preview);
        if (side_preview)
        {
            SameLine();
            Checkbox("With Ref Color", &ref_color);
            if (ref_color)
            {
                SameLine();
                ColorEdit4("##RefColor", &ref_color_v.x, ImGuiColorEditFlags_NoInputs | misc_flags);
            }
        }
        Combo("Display Mode", &display_mode, "Auto/Current\0None\0RGB Only\0HSV Only\0Hex Only\0");
        SameLine(); HelpMarker(
            "ColorEdit defaults to displaying RGB inputs if you don't specify a display mode, "
            "but the user can change it with a right-click on those inputs.\n\nColorPicker defaults to displaying RGB+HSV+Hex "
            "if you don't specify a display mode.\n\nYou can change the defaults using SetColorEditOptions().");
        SameLine(); HelpMarker("When not specified explicitly (Auto/Current mode), user can right-click the picker to change mode.");
        flags: ImGuiColorEditFlags = misc_flags;
        if (!alpha)            flags |= ImGuiColorEditFlags_NoAlpha;        // This is by default if you call ColorPicker3() instead of ColorPicker4()
        if (alpha_bar)         flags |= ImGuiColorEditFlags_AlphaBar;
        if (!side_preview)     flags |= ImGuiColorEditFlags_NoSidePreview;
        if (picker_mode == 1)  flags |= ImGuiColorEditFlags_PickerHueBar;
        if (picker_mode == 2)  flags |= ImGuiColorEditFlags_PickerHueWheel;
        if (display_mode == 1) flags |= ImGuiColorEditFlags_NoInputs;       // Disable all RGB/HSV/Hex displays
        if (display_mode == 2) flags |= ImGuiColorEditFlags_DisplayRGB;     // Override display mode
        if (display_mode == 3) flags |= ImGuiColorEditFlags_DisplayHSV;
        if (display_mode == 4) flags |= ImGuiColorEditFlags_DisplayHex;
        ColorPicker4("MyColor##4", (&mut c_float)&color, flags, if ref_color { & ref_color_v.x} else {None});

        Text("Set defaults in code:");
        SameLine(); HelpMarker(
            "SetColorEditOptions() is designed to allow you to set boot-time default.\n"
            "We don't have Push/Pop functions because you can force options on a per-widget basis if needed,"
            "and the user can change non-forced ones with the options menu.\nWe don't have a getter to avoid"
            "encouraging you to persistently save values that aren't forward-compatible.");
        if (Button("Default: Uint8 + HSV + Hue Bar"))
            SetColorEditOptions(ImGuiColorEditFlags_Uint8 | ImGuiColorEditFlags_DisplayHSV | ImGuiColorEditFlags_PickerHueBar);
        if (Button("Default: Float + HDR + Hue Wheel"))
            SetColorEditOptions(ImGuiColorEditFlags_Float | ImGuiColorEditFlags_HDR | ImGuiColorEditFlags_PickerHueWheel);

        // Always both a small version of both types of pickers (to make it more visible in the demo to people who are skimming quickly through it)
        Text("Both types:");
        let w: c_float =  (GetContentRegionAvail().x - GetStyle().ItemSpacing.y) * 0.40f32;
        SetNextItemWidth(w);
        ColorPicker3("##MyColor##5", (&mut c_float)&color, ImGuiColorEditFlags_PickerHueBar | ImGuiColorEditFlags_NoSidePreview | ImGuiColorEditFlags_NoInputs | ImGuiColorEditFlags_NoAlpha);
        SameLine();
        SetNextItemWidth(w);
        ColorPicker3("##MyColor##6", (&mut c_float)&color, ImGuiColorEditFlags_PickerHueWheel | ImGuiColorEditFlags_NoSidePreview | ImGuiColorEditFlags_NoInputs | ImGuiColorEditFlags_NoAlpha);

        // HSV encoded support (to avoid RGB<>HSV round trips and singularities when S==0 or V==0)
        static color_hsv: ImVec4(0.23f, 1.0, 1.0, 1.0); // Stored as HSV!
        Spacing();
        Text("HSV encoded colors");
        SameLine(); HelpMarker(
            "By default, colors are given to ColorEdit and ColorPicker in RGB, but ImGuiColorEditFlags_InputHSV"
            "allows you to store colors as HSV and pass them to ColorEdit and ColorPicker as HSV. This comes with the"
            "added benefit that you can manipulate hue values with the picker even when saturation or value are zero.");
        Text("Color widget with InputHSV:");
        ColorEdit4("HSV shown as RGB##1", (&mut c_float)&color_hsv, ImGuiColorEditFlags_DisplayRGB | ImGuiColorEditFlags_InputHSV | ImGuiColorEditFlags_Float);
        ColorEdit4("HSV shown as HSV##1", (&mut c_float)&color_hsv, ImGuiColorEditFlags_DisplayHSV | ImGuiColorEditFlags_InputHSV | ImGuiColorEditFlags_Float);
        DragFloat4("Raw HSV values", (&mut c_float)&color_hsv, 0.01f, 0.0, 1.0);

        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/Drag and Slider Flags");
    if (TreeNode("Drag/Slider Flags"))
    {
        // Demonstrate using advanced flags for DragXXX and SliderXXX functions. Note that the flags are the same!
        static flags: ImGuiSliderFlags = ImGuiSliderFlags_None;
        CheckboxFlags("ImGuiSliderFlags_AlwaysClamp", &flags, ImGuiSliderFlags_AlwaysClamp);
        SameLine(); HelpMarker("Always clamp value to min/max bounds (if any) when input manually with CTRL+Click.");
        CheckboxFlags("ImGuiSliderFlags_Logarithmic", &flags, ImGuiSliderFlags_Logarithmic);
        SameLine(); HelpMarker("Enable logarithmic editing (more precision for small values).");
        CheckboxFlags("ImGuiSliderFlags_NoRoundToFormat", &flags, ImGuiSliderFlags_NoRoundToFormat);
        SameLine(); HelpMarker("Disable rounding underlying value to match precision of the format string (e.g. {} values are rounded to those 3 digits).");
        CheckboxFlags("ImGuiSliderFlags_NoInput", &flags, ImGuiSliderFlags_NoInput);
        SameLine(); HelpMarker("Disable CTRL+Click or Enter key allowing to input text directly into the widget.");

        // Drags
        static let drag_f: c_float =  0.5;
        static let drag_i: c_int = 50;
        Text("Underlying float value: {}", drag_0f32);
        DragFloat("DragFloat (0 -> 1)", &drag_f, 0.005f, 0.0, 1.0, "{}", flags);
        DragFloat("DragFloat (0 -> +in0f32)", &drag_f, 0.005f, 0.0, f32::MAX, "{}", flags);
        DragFloat("DragFloat (-inf -> 1)", &drag_f, 0.005f, -f32::MAX, 1.0, "{}", flags);
        DragFloat("DragFloat (-inf -> +in0f32)", &drag_f, 0.005f, -f32::MAX, f32::MAX, "{}", flags);
        DragInt("DragInt (0 -> 100)", &drag_i, 0.5, 0, 100, "{}", flags);

        // Sliders
        static let slider_f: c_float =  0.5;
        static let slider_i: c_int = 50;
        Text("Underlying float value: {}", slider_0f32);
        SliderFloat("SliderFloat (0 -> 1)", &slider_f, 0.0, 1.0, "{}", flags);
        SliderInt("SliderInt (0 -> 100)", &slider_i, 0, 100, "{}", flags);

        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/Range Widgets");
    if (TreeNode("Range Widgets"))
    {
        static let begin: c_float =  10, end = 90;
        static let begin_i: c_int = 100, end_i = 1000;
        DragFloatRange2("range float", &begin, &end, 0.25f, 0.0, 100, "Min: {} %%", "Max: {} %%", ImGuiSliderFlags_AlwaysClamp);
        DragIntRange2("range int", &begin_i, &end_i, 5, 0, 1000, "Min: {} units", "Max: {} units");
        DragIntRange2("range int (no bounds)", &begin_i, &end_i, 5, 0, 0, "Min: {} units", "Max: {} units");
        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/Data Types");
    if (TreeNode("Data Types"))
    {
        // DragScalar/InputScalar/SliderScalar functions allow various data types
        // - signed/unsigned
        // - 8/16/32/64-bits
        // - integer/float/double
        // To avoid polluting the public API with all possible combinations, we use the enum: ImGuiDataType
        // to pass the type, and passing all arguments by pointer.
        // This is the reason the test code below creates local variables to hold "zero" "one" etc. for each types.
        // In practice, if you frequently use a given type that is not covered by the normal API entry points,
        // you can wrap it yourself inside a 1 line function which can take typed argument as value instead of void*,
        // and then pass their address to the generic function. For example:
        //   MySliderU64: bool(const label: *mut c_char, u64* value, u64 min = 0, u64 max = 0, const char* format = "%lld")
        //   {
        //      return SliderScalar(label, IM_GUI_DATA_TYPE_U64, value, &min, &max, format);
        //   }

        // Setup limits (as helper variables so we can take their address, as explained above)
        // Note: SliderScalar() functions have a maximum usable range of half the natural type maximum, hence the /2.
        #ifndef LLONG_MIN
        i64 LLONG_MIN = -9223372036854775807LL - 1;
        i64 LLONG_MAX = 9223372036854775807LL;
        u64 ULLONG_MAX = (2ULL * 9223372036854775807LL + 1);
        #endif
        const char    s8_zero  = 0,   s8_one  = 1,   s8_fifty  = 50, s8_min  = -128,        s8_max = 127;
        const u8    u8_zero  = 0,   u8_one  = 1,   u8_fifty  = 50, u8_min  = 0,           u8_max = 255;
        const c_short   s16_zero = 0,   s16_one = 1,   s16_fifty = 50, s16_min = -32768,      s16_max = 32767;
        const ImU16   u16_zero = 0,   u16_one = 1,   u16_fifty = 50, u16_min = 0,           u16_max = 65535;
        const i32   s32_zero = 0,   s32_one = 1,   s32_fifty = 50, s32_min = INT_MIN/2,   s32_max = INT_MAX/2,    s32_hi_a = INT_MAX/2 - 100,    s32_hi_b = INT_MAX/2;
        const u32   u32_zero = 0,   u32_one = 1,   u32_fifty = 50, u32_min = 0,           u32_max = UINT_MAX/2,   u32_hi_a = UINT_MAX/2 - 100,   u32_hi_b = UINT_MAX/2;
        const i64   s64_zero = 0,   s64_one = 1,   s64_fifty = 50, s64_min = LLONG_MIN/2, s64_max = LLONG_MAX/2,  s64_hi_a = LLONG_MAX/2 - 100,  s64_hi_b = LLONG_MAX/2;
        const u64   u64_zero = 0,   u64_one = 1,   u64_fifty = 50, u64_min = 0,           u64_max = ULLONG_MAX/2, u64_hi_a = ULLONG_MAX/2 - 100, u64_hi_b = ULLONG_MAX/2;
        f32_zero: c_float = 0.f, f32_one = 1.f, f32_lo_a = -10000000000, f32_hi_a = 10000000000;
        const double  f64_zero = 0.,  f64_one = 1.,  f64_lo_a = -1000000000000000.0, f64_hi_a = 1000000000000000.0;

        // State
        static char   s8_v  = 127;
        static u8   u8_v  = 255;
        static c_short  s16_v = 32767;
        static ImU16  u16_v = 65535;
        static i32  s32_v = -1;
        static u32  u32_v = -1;
        static i64  s64_v = -1;
        static u64  u64_v = -1;
        staticf32_v: c_float = 0.123f;
        static double f64_v = 90000.01234567890123456789;

        let drag_speed: c_float =  0.2f;
        static let mut drag_clamp: bool =  false;
        IMGUI_DEMO_MARKER("Widgets/Data Types/Drags");
        Text("Drags:");
        Checkbox("Clamp integers to 0..50", &drag_clamp);
        SameLine(); HelpMarker(
            "As with every widgets in dear imgui, we never modify values unless there is a user interaction.\n"
            "You can override the clamping limits by using CTRL+Click to input a value.");
        DragScalar("drag s8",        ImGuiDataType_S8,     &s8_v,  drag_speed, if drag_clamp { & s8_zero} else { None}, if drag_clamp { & s8_fifty } else {None});
    DragScalar("drag u8",        ImGuiDataType_U8,     &u8_v,  drag_speed, if drag_clamp { & u8_zero}  else {None}, if drag_clamp { & u8_fifty  }else {None}, "%u ms");
        DragScalar("drag s16",       ImGuiDataType_S16,    &s16_v, drag_speed, if drag_clamp { & s16_zero} else {None}, if drag_clamp { & s16_fifty} else {None});
        DragScalar("drag u16",       ImGuiDataType_U16,    &u16_v, drag_speed, if drag_clamp { & u16_zero} else {None}, if drag_clamp{  & u16_fifty } else {None}, "%u ms");
        DragScalar("drag s32",       ImGuiDataType_S32,    &s32_v, drag_speed, if drag_clamp { & s32_zero} else {None}, if drag_clamp {& s32_fifty} else {None});
        DragScalar("drag s32 hex",   ImGuiDataType_S32,    &s32_v, drag_speed, if drag_clamp { & s32_zero} else {None}, if drag_clamp { & s32_fifty} else {None}, "0x{}");
        DragScalar("drag u32",       ImGuiDataType_U32,    &u32_v, drag_speed, if drag_clamp { & u32_zero }else {None}, if drag_clamp { & u32_fifty} else {None}, "%u ms");
        DragScalar("drag s64",       ImGuiDataType_S64,    &s64_v, drag_speed, if drag_clamp { & s64_zero} else {None}, if drag_clamp { & s64_fifty} else  {None});
        DragScalar("drag u64",       ImGuiDataType_U64,    &u64_v, drag_speed, if drag_clamp { & u64_zero} else {None}, if drag_clamp { & u64_fifty} else {None});
        DragScalar("drag float",     ImGuiDataType_Float,  &f32_v, 0.005f,  &f32_zero, &f32_one, "{}");
        DragScalar("drag float log", ImGuiDataType_Float,  &f32_v, 0.005f,  &f32_zero, &f32_one, "{}", ImGuiSliderFlags_Logarithmic);
        DragScalar("drag double",    ImGuiDataType_Double, &f64_v, 0.05f, &f64_zero, None,     "{} grams");
        DragScalar("drag double log",ImGuiDataType_Double, &f64_v, 0.05f, &f64_zero, &f64_one, "0 < {} < 1", ImGuiSliderFlags_Logarithmic);

        IMGUI_DEMO_MARKER("Widgets/Data Types/Sliders");
        Text("Sliders");
        SliderScalar("slider s8 full",       ImGuiDataType_S8,     &s8_v,  &s8_min,   &s8_max,   "{}");
        SliderScalar("slider u8 full",       ImGuiDataType_U8,     &u8_v,  &u8_min,   &u8_max,   "%u");
        SliderScalar("slider s16 full",      ImGuiDataType_S16,    &s16_v, &s16_min,  &s16_max,  "{}");
        SliderScalar("slider u16 full",      ImGuiDataType_U16,    &u16_v, &u16_min,  &u16_max,  "%u");
        SliderScalar("slider s32 low",       ImGuiDataType_S32,    &s32_v, &s32_zero, &s32_fifty,"{}");
        SliderScalar("slider s32 high",      ImGuiDataType_S32,    &s32_v, &s32_hi_a, &s32_hi_b, "{}");
        SliderScalar("slider s32 full",      ImGuiDataType_S32,    &s32_v, &s32_min,  &s32_max,  "{}");
        SliderScalar("slider s32 hex",       ImGuiDataType_S32,    &s32_v, &s32_zero, &s32_fifty, "{}");
        SliderScalar("slider low: u32",       ImGuiDataType_U32,    &u32_v, &u32_zero, &u32_fifty,"%u");
        SliderScalar("slider high: u32",      ImGuiDataType_U32,    &u32_v, &u32_hi_a, &u32_hi_b, "%u");
        SliderScalar("slider full: u32",      ImGuiDataType_U32,    &u32_v, &u32_min,  &u32_max,  "%u");
        SliderScalar("slider s64 low",       ImGuiDataType_S64,    &s64_v, &s64_zero, &s64_fifty,"%" IM_PRId64);
        SliderScalar("slider s64 high",      ImGuiDataType_S64,    &s64_v, &s64_hi_a, &s64_hi_b, "%" IM_PRId64);
        SliderScalar("slider s64 full",      ImGuiDataType_S64,    &s64_v, &s64_min,  &s64_max,  "%" IM_PRId64);
        SliderScalar("slider u64 low",       ImGuiDataType_U64,    &u64_v, &u64_zero, &u64_fifty,"%" IM_PRIu64 " ms");
        SliderScalar("slider u64 high",      ImGuiDataType_U64,    &u64_v, &u64_hi_a, &u64_hi_b, "%" IM_PRIu64 " ms");
        SliderScalar("slider u64 full",      ImGuiDataType_U64,    &u64_v, &u64_min,  &u64_max,  "%" IM_PRIu64 " ms");
        SliderScalar("slider float low",     ImGuiDataType_Float,  &f32_v, &f32_zero, &f32_one);
        SliderScalar("slider float low log", ImGuiDataType_Float,  &f32_v, &f32_zero, &f32_one,  "%.10f", ImGuiSliderFlags_Logarithmic);
        SliderScalar("slider float high",    ImGuiDataType_Float,  &f32_v, &f32_lo_a, &f32_hi_a, "%e");
        SliderScalar("slider double low",    ImGuiDataType_Double, &f64_v, &f64_zero, &f64_one,  "{} grams");
        SliderScalar("slider double low log",ImGuiDataType_Double, &f64_v, &f64_zero, &f64_one,  "%.10f", ImGuiSliderFlags_Logarithmic);
        SliderScalar("slider double high",   ImGuiDataType_Double, &f64_v, &f64_lo_a, &f64_hi_a, "%e grams");

        Text("Sliders (reverse)");
        SliderScalar("slider s8 reverse",    ImGuiDataType_S8,   &s8_v,  &s8_max,    &s8_min,   "{}");
        SliderScalar("slider u8 reverse",    ImGuiDataType_U8,   &u8_v,  &u8_max,    &u8_min,   "%u");
        SliderScalar("slider s32 reverse",   ImGuiDataType_S32,  &s32_v, &s32_fifty, &s32_zero, "{}");
        SliderScalar("slider reverse: u32",   ImGuiDataType_U32,  &u32_v, &u32_fifty, &u32_zero, "%u");
        SliderScalar("slider s64 reverse",   ImGuiDataType_S64,  &s64_v, &s64_fifty, &s64_zero, "%" IM_PRId64);
        SliderScalar("slider u64 reverse",   ImGuiDataType_U64,  &u64_v, &u64_fifty, &u64_zero, "%" IM_PRIu64 " ms");

        IMGUI_DEMO_MARKER("Widgets/Data Types/Inputs");
        static let mut inputs_step: bool =  true;
        Text("Inputs");
        Checkbox("Show step buttons", &inputs_step);
        InputScalar("input s8",      ImGuiDataType_S8,     &s8_v,  if inputs_step { & s8_one}  else {None}, None, "{}");
        InputScalar("input u8",      ImGuiDataType_U8,     &u8_v,  if inputs_step { & u8_one } else {None}, None, "%u");
        InputScalar("input s16",     ImGuiDataType_S16,    &s16_v, if inputs_step { & s16_one} else {None}, None, "{}");
        InputScalar("input u16",     ImGuiDataType_U16,    &u16_v, if inputs_step { &u16_one} else { None }, None, "%u");
        InputScalar("input s32",     ImGuiDataType_S32,    &s32_v, if inputs_step { &s32_one} else { None }, None, "{}");
        InputScalar("input s32 hex", ImGuiDataType_S32,    &s32_v, if inputs_step { &s32_one} else { None }, None, "{}");
        InputScalar("input u32",     ImGuiDataType_U32,    &u32_v, if inputs_step { &u32_one} else { None }, None, "%u");
        InputScalar("input hex: u32", ImGuiDataType_U32,    &u32_v, if inputs_step { &u32_one} else { None }, None, "{}");
        InputScalar("input s64",     ImGuiDataType_S64,    &s64_v, if inputs_step { &s64_one} else { None });
        InputScalar("input u64",     ImGuiDataType_U64,    &u64_v, if inputs_step { &u64_one} else { None });
        InputScalar("input float",   ImGuiDataType_Float,  &f32_v, if inputs_step { &f32_one} else { None });
        InputScalar("input double",  ImGuiDataType_Double, &f64_v, if inputs_step { &f64_one} else { None });

        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/Multi-component Widgets");
    if (TreeNode("Multi-component Widgets"))
    {
        staticvec4f: c_float[4] = { 0.1.0, 0.20, 0.3f32, 0.44f };
        static vec4i: c_int[4] = { 1, 5, 100, 255 };

        InputFloat2("input float2", vec40f32);
        DragFloat2("drag float2", vec4f, 0.01f, 0.0, 1.0);
        SliderFloat2("slider float2", vec4f, 0.0, 1.0);
        InputInt2("input int2", vec4i);
        DragInt2("drag int2", vec4i, 1, 0, 255);
        SliderInt2("slider int2", vec4i, 0, 255);
        Spacing();

        InputFloat3("input float3", vec40f32);
        DragFloat3("drag float3", vec4f, 0.01f, 0.0, 1.0);
        SliderFloat3("slider float3", vec4f, 0.0, 1.0);
        InputInt3("input int3", vec4i);
        DragInt3("drag int3", vec4i, 1, 0, 255);
        SliderInt3("slider int3", vec4i, 0, 255);
        Spacing();

        InputFloat4("input float4", vec40f32);
        DragFloat4("drag float4", vec4f, 0.01f, 0.0, 1.0);
        SliderFloat4("slider float4", vec4f, 0.0, 1.0);
        InputInt4("input int4", vec4i);
        DragInt4("drag int4", vec4i, 1, 0, 255);
        SliderInt4("slider int4", vec4i, 0, 255);

        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/Vertical Sliders");
    if (TreeNode("Vertical Sliders"))
    {
        let spacing: c_float =  4;
        PushStyleVar(ImGuiStyleVar_ItemSpacing, ImVec2::new(spacing, spacing));

        static let int_value: c_int = 0;
        VSliderInt("##int", ImVec2::new(18, 160), &int_value, 0, 5);
        SameLine();

        staticvalues: c_float[7] = { 0.0, 0.60, 0.35f, 0.9f, 0.70f32, 0.20, 0.0 };
        PushID("set1");
        for (let i: c_int = 0; i < 7; i++)
        {
            if i > 0 {  SameLine(); }
            PushID(i);
            PushStyleColor(ImGuiCol_FrameBg, (ImVec4)ImColor::HSV(i / 7.0, 0.5, 0.5));
            PushStyleColor(ImGuiCol_FrameBgHovered, (ImVec4)ImColor::HSV(i / 7.0, 0.6f, 0.5));
            PushStyleColor(ImGuiCol_FrameBgActive, (ImVec4)ImColor::HSV(i / 7.0, 0.7f, 0.5));
            PushStyleColor(ImGuiCol_SliderGrab, (ImVec4)ImColor::HSV(i / 7.0, 0.9f, 0.90));
            VSliderFloat("##v", ImVec2::new(18, 160), &values[i], 0.0, 1.0, "");
            if (IsItemActive() || IsItemHovered())
                SetTooltip("{}", values[i]);
            PopStyleColor(4);
            PopID();
        }
        PopID();

        SameLine();
        PushID("set2");
        staticvalues2: c_float[4] = { 0.20, 0.80, 0.40f32, 0.25f };
        let rows: c_int = 3;
        const small_slider_size: ImVec2(18, ((160f32 - (rows - 1) * spacing) / rows));
        for (let nx: c_int = 0; nx < 4; nx++)
        {
            if nx > 0 {  SameLine(); }
            BeginGroup();
            for (let ny: c_int = 0; ny < rows; ny++)
            {
                PushID(nx * rows + ny);
                VSliderFloat("##v", small_slider_size, &values2[nx], 0.0, 1.0, "");
                if (IsItemActive() || IsItemHovered())
                    SetTooltip("{}", values2[nx]);
                PopID();
            }
            EndGroup();
        }
        PopID();

        SameLine();
        PushID("set3");
        for (let i: c_int = 0; i < 4; i++)
        {
            if i > 0 {  SameLine(); }
            PushID(i);
            PushStyleVar(ImGuiStyleVar_GrabMinSize, 40);
            VSliderFloat("##v", ImVec2::new(40, 160), &values[i], 0.0, 1.0, "{}\nsec");
            PopStyleVar();
            PopID();
        }
        PopID();
        PopStyleVar();
        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/Drag and drop");
    if (TreeNode("Drag and Drop"))
    {
        IMGUI_DEMO_MARKER("Widgets/Drag and drop/Standard widgets");
        if (TreeNode("Drag and drop in standard widgets"))
        {
            // ColorEdit widgets automatically act as drag source and drag target.
            // They are using standardized payload strings IMGUI_PAYLOAD_TYPE_COLOR_3F and IMGUI_PAYLOAD_TYPE_COLOR_4F
            // to allow your own widgets to use colors in their drag and drop interaction.
            // Also see 'Demo->Widgets->Color/Picker Widgets->Palette' demo.
            HelpMarker("You can drag from the color squares.");
            staticcol1: c_float[3] = { 1.0, 0.0, 0.2f };
            staticcol2: c_float[4] = { 0.4f, 0.7f, 0.0, 0.5 };
            ColorEdit3("color 1", col1);
            ColorEdit4("color 2", col2);
            TreePop();
        }

        IMGUI_DEMO_MARKER("Widgets/Drag and drop/Copy-swap items");
        if (TreeNode("Drag and drop to copy/swap items"))
        {
            enum Mode
            {
                Mode_Copy,
                Mode_Move,
                Mode_Swap
            };
            static let mode: c_int = 0;
            if (RadioButton("Copy", mode == Mode_Copy)) { mode = Mode_Copy; } SameLine();
            if (RadioButton("Move", mode == Mode_Move)) { mode = Mode_Move; } SameLine();
            if (RadioButton("Swap", mode == Mode_Swap)) { mode = Mode_Swap; }
            static *const names: [c_char;9] =
            {
                "Bobby", "Beatrice", "Betty",
                "Brianna", "Barry", "Bernard",
                "Bibi", "Blaine", "Bryn"
            };
            for (let n: c_int = 0; n < names.len(); n++)
            {
                PushID(n);
                if ((n % 3) != 0)
                    SameLine();
                Button(names[n], ImVec2::new(60, 60));

                // Our buttons are both drag sources and drag targets here!
                if (BeginDragDropSource(ImGuiDragDropFlags_None))
                {
                    // Set payload to carry the index of our item (could be anything)
                    SetDragDropPayload("DND_DEMO_CELL", &n, sizeof);

                    // Display preview (could be anything, e.g. when dragging an image we could decide to display
                    // the filename and a small preview of the image, etc.)
                    if (mode == Mode_Copy) { Text("Copy {}", names[n]); }
                    if (mode == Mode_Move) { Text("Move {}", names[n]); }
                    if (mode == Mode_Swap) { Text("Swap {}", names[n]); }
                    EndDragDropSource();
                }
                if (BeginDragDropTarget())
                {
                    if (*const ImGuiPayload payload = AcceptDragDropPayload("DND_DEMO_CELL"))
                    {
                        // IM_ASSERT(payload->DataSize == sizeof);
                        let payload_n: c_int = *(*const c_int)payload.Data;
                        if (mode == Mode_Copy)
                        {
                            names[n] = names[payload_n];
                        }
                        if (mode == Mode_Move)
                        {
                            names[n] = names[payload_n];
                            names[payload_n] = "";
                        }
                        if (mode == Mode_Swap)
                        {
                            let mut  tmp: *const c_char = names[n];
                            names[n] = names[payload_n];
                            names[payload_n] = tmp;
                        }
                    }
                    EndDragDropTarget();
                }
                PopID();
            }
            TreePop();
        }

        IMGUI_DEMO_MARKER("Widgets/Drag and Drop/Drag to reorder items (simple)");
        if (TreeNode("Drag to reorder items (simple)"))
        {
            // Simple reordering
            HelpMarker(
                "We don't use the drag and drop api at all here! "
                "Instead we query when the item is held but not hovered, and order items accordingly.");
            static item_names: *const c_char[] = { "Item One", "Item Two", "Item Three", "Item Four", "Item Five" };
            for (let n: c_int = 0; n < item_names.len(); n++)
            {
                let mut  item: *const c_char = item_names[n];
                Selectable(item);

                if (IsItemActive() && !IsItemHovered())
                {
                    let n_next: c_int = if n + (GetMouseDragDelta(0).y < 0.f { - 1} else {1});
                    if (n_next >= 0 && n_next < item_names.len())
                    {
                        item_names[n] = item_names[n_next];
                        item_names[n_next] = item;
                        ResetMouseDragDelta();
                    }
                }
            }
            TreePop();
        }

        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/Querying Item Status (Edited,Active,Hovered etc.)");
    if (TreeNode("Querying Item Status (Edited/Active/Hovered etc.)"))
    {
        // Select an item type
        item_names: *const c_char[] =
        {
            "Text", "Button", "Button (w/ repeat)", "Checkbox", "SliderFloat", "InputText", "InputTextMultiline", "InputFloat",
            "InputFloat3", "ColorEdit4", "Selectable", "MenuItem", "TreeNode", "TreeNode (w/ double-click)", "Combo", "ListBox"
        };
        static let item_type: c_int = 4;
        static let mut item_disabled: bool =  false;
        Combo("Item Type", &item_type, item_names, item_names.len(), item_names.len());
        SameLine();
        HelpMarker("Testing how various types of items are interacting with the IsItemXXX functions. Note that the return: bool value of most ImGui function is generally equivalent to calling IsItemHovered().");
        Checkbox("Item Disabled",  &item_disabled);

        // Submit selected item item so we can query their status in the code following it.
        let mut ret: bool =  false;
        static let mut b: bool =  false;
        staticcol4f: c_float[4] = { 1.0, 0.5, 0.0, 1.0 };
        static str: [c_char;16] = {};
        if item_disabled {
            BeginDisabled(true)(); }
        if (item_type == 0) { Text("ITEM: Text"); }                                              // Testing text items with no identifier/interaction
        if (item_type == 1) { ret = Button("ITEM: Button"); }                                    // Testing button
        if (item_type == 2) { PushButtonRepeat(true); ret = Button("ITEM: Button"); PopButtonRepeat(); } // Testing button (with repeater)
        if (item_type == 3) { ret = Checkbox("ITEM: Checkbox", &b); }                            // Testing checkbox
        if (item_type == 4) { ret = SliderFloat("ITEM: SliderFloat", &col4f[0], 0.0, 1.0); }   // Testing basic item
        if (item_type == 5) { ret = InputText("ITEM: InputText", &str[0], str.len()); }  // Testing input text (which handles tabbing)
        if (item_type == 6) { ret = InputTextMultiline("ITEM: InputTextMultiline", &str[0], str.len()); } // Testing input text (which uses a child window)
        if (item_type == 7) { ret = InputFloat("ITEM: InputFloat", col4f, 1.0); }               // Testing +/- buttons on scalar input
        if (item_type == 8) { ret = InputFloat3("ITEM: InputFloat3", col40f32); }                   // Testing multi-component items (IsItemXXX flags are reported merged)
        if (item_type == 9) { ret = ColorEdit4("ITEM: ColorEdit4", col40f32); }                     // Testing multi-component items (IsItemXXX flags are reported merged)
        if (item_type == 10){ ret = Selectable("ITEM: Selectable"); }                            // Testing selectable item
        if (item_type == 11){ ret = MenuItem("ITEM: MenuItem"); }                                // Testing menu item (they use ImGuiButtonFlags_PressedOnRelease button policy)
        if (item_type == 12){ ret = TreeNode("ITEM: TreeNode"); if ret {  TreePop(); }  }     // Testing tree node
        if (item_type == 13){ ret = TreeNodeEx("ITEM: TreeNode w/ ImGuiTreeNodeFlags_OpenOnDoubleClick", ImGuiTreeNodeFlags_OpenOnDoubleClick | ImGuiTreeNodeFlags_NoTreePushOnOpen); } // Testing tree node with ImGuiButtonFlags_PressedOnDoubleClick button policy.
        if (item_type == 14){ items: *const c_char[] = { "Apple", "Banana", "Cherry", "Kiwi" }; static let current: c_int = 1; ret = Combo("ITEM: Combo", &current, items, items.len()); }
        if (item_type == 15){ items: *const c_char[] = { "Apple", "Banana", "Cherry", "Kiwi" }; static let current: c_int = 1; ret = ListBox("ITEM: ListBox", &current, items, items.len(), items.len()); }

        let mut hovered_delay_none: bool =  IsItemHovered();
        let mut hovered_delay_short: bool =  IsItemHovered(ImGuiHoveredFlags_DelayShort);
        let mut hovered_delay_normal: bool =  IsItemHovered(ImGuiHoveredFlags_DelayNormal);

        // Display the values of IsItemHovered() and other common item state functions.
        // Note that the ImGuiHoveredFlags_XXX flags can be combined.
        // Because BulletText is an item itself and that would affect the output of IsItemXXX functions,
        // we query every state in a single call to avoid storing them and to simplify the code.
        BulletText(
            "Return value = {}\n"
            "IsItemFocused() = {}\n"
            "IsItemHovered() = {}\n"
            "IsItemHovered(_AllowWhenBlockedByPopup) = {}\n"
            "IsItemHovered(_AllowWhenBlockedByActiveItem) = {}\n"
            "IsItemHovered(_AllowWhenOverlapped) = {}\n"
            "IsItemHovered(_AllowWhenDisabled) = {}\n"
            "IsItemHovered(_RectOnly) = {}\n"
            "IsItemActive() = {}\n"
            "IsItemEdited() = {}\n"
            "IsItemActivated() = {}\n"
            "IsItemDeactivated() = {}\n"
            "IsItemDeactivatedAfterEdit() = {}\n"
            "IsItemVisible() = {}\n"
            "IsItemClicked() = {}\n"
            "IsItemToggledOpen() = {}\n"
            "GetItemRectMin() = ({}, {})\n"
            "GetItemRectMax() = ({}, {})\n"
            "GetItemRectSize() = ({}, {})",
            ret,
            IsItemFocused(),
            IsItemHovered(),
            IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup),
            IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByActiveItem),
            IsItemHovered(ImGuiHoveredFlags_AllowWhenOverlapped),
            IsItemHovered(ImGuiHoveredFlags_AllowWhenDisabled),
            IsItemHovered(ImGuiHoveredFlags_RectOnly),
            IsItemActive(),
            IsItemEdited(),
            IsItemActivated(),
            IsItemDeactivated(),
            IsItemDeactivatedAfterEdit(),
            IsItemVisible(),
            IsItemClicked(),
            IsItemToggledOpen(),
            GetItemRectMin().x, GetItemRectMin().y,
            GetItemRectMax().x, GetItemRectMax().y,
            GetItemRectSize().x, GetItemRectSize().y
        );
        BulletText(
            "w/ Hovering Delay: None = {}, Fast {}, Normal = {}", hovered_delay_none, hovered_delay_short, hovered_delay_normal);

        if item_disabled {
            EndDisabled(); }

        buf: [c_char;1] = "";
        InputText("unused", buf, buf.len(), ImGuiInputTextFlags_ReadOnly);
        SameLine();
        HelpMarker("This widget is only here to be able to tab-out of the widgets above and see e.g. Deactivated() status.");

        TreePop();
    }

    IMGUI_DEMO_MARKER("Widgets/Querying Window Status (Focused,Hovered etc.)");
    if (TreeNode("Querying Window Status (Focused/Hovered etc.)"))
    {
        static let mut embed_all_inside_a_child_window: bool =  false;
        Checkbox("Embed everything inside a child window for testing _RootWindow flag.", &embed_all_inside_a_child_window);
        if (embed_all_inside_a_child_window)
            BeginChild("outer_child", ImVec2::new(0, GetFontSize() * 20f32), true);

        // Testing IsWindowFocused() function with its various flags.
        BulletText(
            "IsWindowFocused() = {}\n"
            "IsWindowFocused(_ChildWindows) = {}\n"
            "IsWindowFocused(_ChildWindows|_NoPopupHierarchy) = {}\n"
            "IsWindowFocused(_ChildWindows|_DockHierarchy) = {}\n"
            "IsWindowFocused(_ChildWindows|_RootWindow) = {}\n"
            "IsWindowFocused(_ChildWindows|_RootWindow|_NoPopupHierarchy) = {}\n"
            "IsWindowFocused(_ChildWindows|_RootWindow|_DockHierarchy) = {}\n"
            "IsWindowFocused(_RootWindow) = {}\n"
            "IsWindowFocused(_RootWindow|_NoPopupHierarchy) = {}\n"
            "IsWindowFocused(_RootWindow|_DockHierarchy) = {}\n"
            "IsWindowFocused(_AnyWindow) = {}\n",
            IsWindowFocused(),
            IsWindowFocused(ImGuiFocusedFlags_ChildWindows),
            IsWindowFocused(ImGuiFocusedFlags_ChildWindows | ImGuiFocusedFlags_NoPopupHierarchy),
            IsWindowFocused(ImGuiFocusedFlags_ChildWindows | ImGuiFocusedFlags_DockHierarchy),
            IsWindowFocused(ImGuiFocusedFlags_ChildWindows | ImGuiFocusedFlags_RootWindow),
            IsWindowFocused(ImGuiFocusedFlags_ChildWindows | ImGuiFocusedFlags_RootWindow | ImGuiFocusedFlags_NoPopupHierarchy),
            IsWindowFocused(ImGuiFocusedFlags_ChildWindows | ImGuiFocusedFlags_RootWindow | ImGuiFocusedFlags_DockHierarchy),
            IsWindowFocused(ImGuiFocusedFlags_RootWindow),
            IsWindowFocused(ImGuiFocusedFlags_RootWindow | ImGuiFocusedFlags_NoPopupHierarchy),
            IsWindowFocused(ImGuiFocusedFlags_RootWindow | ImGuiFocusedFlags_DockHierarchy),
            IsWindowFocused(ImGuiFocusedFlags_AnyWindow));

        // Testing IsWindowHovered() function with its various flags.
        BulletText(
            "IsWindowHovered() = {}\n"
            "IsWindowHovered(_AllowWhenBlockedByPopup) = {}\n"
            "IsWindowHovered(_AllowWhenBlockedByActiveItem) = {}\n"
            "IsWindowHovered(_ChildWindows) = {}\n"
            "IsWindowHovered(_ChildWindows|_NoPopupHierarchy) = {}\n"
            "IsWindowHovered(_ChildWindows|_DockHierarchy) = {}\n"
            "IsWindowHovered(_ChildWindows|_RootWindow) = {}\n"
            "IsWindowHovered(_ChildWindows|_RootWindow|_NoPopupHierarchy) = {}\n"
            "IsWindowHovered(_ChildWindows|_RootWindow|_DockHierarchy) = {}\n"
            "IsWindowHovered(_RootWindow) = {}\n"
            "IsWindowHovered(_RootWindow|_NoPopupHierarchy) = {}\n"
            "IsWindowHovered(_RootWindow|_DockHierarchy) = {}\n"
            "IsWindowHovered(_ChildWindows|_AllowWhenBlockedByPopup) = {}\n"
            "IsWindowHovered(_AnyWindow) = {}\n",
            IsWindowHovered(),
            IsWindowHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup),
            IsWindowHovered(ImGuiHoveredFlags_AllowWhenBlockedByActiveItem),
            IsWindowHovered(ImGuiHoveredFlags_ChildWindows),
            IsWindowHovered(ImGuiHoveredFlags_ChildWindows | ImGuiHoveredFlags_NoPopupHierarchy),
            IsWindowHovered(ImGuiHoveredFlags_ChildWindows | ImGuiHoveredFlags_DockHierarchy),
            IsWindowHovered(ImGuiHoveredFlags_ChildWindows | ImGuiHoveredFlags_RootWindow),
            IsWindowHovered(ImGuiHoveredFlags_ChildWindows | ImGuiHoveredFlags_RootWindow | ImGuiHoveredFlags_NoPopupHierarchy),
            IsWindowHovered(ImGuiHoveredFlags_ChildWindows | ImGuiHoveredFlags_RootWindow | ImGuiHoveredFlags_DockHierarchy),
            IsWindowHovered(ImGuiHoveredFlags_RootWindow),
            IsWindowHovered(ImGuiHoveredFlags_RootWindow | ImGuiHoveredFlags_NoPopupHierarchy),
            IsWindowHovered(ImGuiHoveredFlags_RootWindow | ImGuiHoveredFlags_DockHierarchy),
            IsWindowHovered(ImGuiHoveredFlags_ChildWindows | ImGuiHoveredFlags_AllowWhenBlockedByPopup),
            IsWindowHovered(ImGuiHoveredFlags_AnyWindow));

        BeginChild("child", ImVec2::new(0, 50), true);
        Text("This is another child window for testing the _ChildWindows flag.");
        EndChild();
        if embed_all_inside_a_child_window {
            EndChild(); }

        // Calling IsItemHovered() after begin returns the hovered status of the title bar.
        // This is useful in particular if you want to create a context menu associated to the title bar of a window.
        // This will also work when docked into a Tab (the Tab replace the Title Bar and guarantee the same properties).
        static let mut test_window: bool =  false;
        Checkbox("Hovered/Active tests after Begin() for title bar testing", &test_window);
        if (test_window)
        {
            // FIXME-DOCK: This window cannot be docked within the ImGui Demo window, this will cause a feedback loop and get them stuck.
            // Could we fix this through an ImGuiWindowClass feature? Or an API call to tag our parent as "don't skip items"?
            Begin("Title bar Hovered/Active tests", &test_window);
            if (BeginPopupContextItem()) // <-- This is using IsItemHovered()
            {
                if (MenuItem("Close")) { test_window = false; }
                EndPopup();
            }
            Text(
                "IsItemHovered() after begin = {} (== is title bar hovered)\n"
                "IsItemActive() after begin = {} (== is window being clicked/moved)\n",
                IsItemHovered(), IsItemActive());
            End();
        }

        TreePop();
    }

    // Demonstrate BeginDisabled/EndDisabled using a checkbox located at the bottom of the section (which is a bit odd:
    // logically we'd have this checkbox at the top of the section, but we don't want this feature to steal that space)
    if disable_all {
        EndDisabled(); }

    IMGUI_DEMO_MARKER("Widgets/Disable Block");
    if (TreeNode("Disable block"))
    {
        Checkbox("Disable entire section above", &disable_all);
        SameLine(); HelpMarker("Demonstrate using BeginDisabled()/EndDisabled() across this section.");
        TreePop();
    }
}

pub unsafe fn ShowDemoWindowLayout()
{
    IMGUI_DEMO_MARKER("Layout");
    if !CollapsingHeader("Layout & Scrolling") { return ; }

    IMGUI_DEMO_MARKER("Layout/Child windows");
    if (TreeNode("Child windows"))
    {
        HelpMarker("Use child windows to begin into a self-contained independent scrolling/clipping regions within a host window.");
        static let mut disable_mouse_wheel: bool =  false;
        static let mut disable_menu: bool =  false;
        Checkbox("Disable Mouse Wheel", &disable_mouse_wheel);
        Checkbox("Disable Menu", &disable_menu);

        // Child 1: no border, enable horizontal scrollbar
        {
            window_flags: ImGuiWindowFlags = ImGuiWindowFlags_HorizontalScrollbar;
            if (disable_mouse_wheel)
                window_flags |= ImGuiWindowFlags_NoScrollWithMouse;
            BeginChild("ChildL", ImVec2::new(GetContentRegionAvail().x * 0.5, 260), false, window_flags);
            for (let i: c_int = 0; i < 100; i++)
                Text("%04d: scrollable region", i);
            EndChild();
        }

        SameLine();

        // Child 2: rounded border
        {
            window_flags: ImGuiWindowFlags = ImGuiWindowFlags_None;
            if (disable_mouse_wheel)
                window_flags |= ImGuiWindowFlags_NoScrollWithMouse;
            if (!disable_menu)
                window_flags |= ImGuiWindowFlags_MenuBar;
            PushStyleVar(ImGuiStyleVar_ChildRounding, 5.0);
            BeginChild("ChildR", ImVec2::new(0, 260), true, window_flags);
            if (!disable_menu && BeginMenuBar())
            {
                if (BeginMenu("Menu"))
                {
                    ShowExampleMenuFile();
                    EndMenu();
                }
                EndMenuBar();
            }
            if (BeginTable("split", 2, ImGuiTableFlags_Resizable | ImGuiTableFlags_NoSavedSettings))
            {
                for (let i: c_int = 0; i < 100; i++)
                {
                    buf: [c_char;32];
                    sprintf(buf, "%03d", i);
                    TableNextColumn();
                    Button(buf, ImVec2::new(-FLT_MIN, 0.0));
                }
                EndTable();
            }
            EndChild();
            PopStyleVar();
        }

        Separator();

        // Demonstrate a few extra things
        // - Changing ImGuiCol_ChildBg (which is transparent black in default styles)
        // - Using SetCursorPos() to position child window (the child window is an item from the POV of parent window)
        //   You can also call SetNextWindowPos() to position the child window. The parent window will effectively
        //   layout from this position.
        // - Using GetItemRectMin/Max() to query the "item" state (because the child window is an item from
        //   the POV of the parent window). See 'Demo->Querying Status (Edited/Active/Hovered etc.)' for details.
        {
            static let offset_x: c_int = 0;
            SetNextItemWidth(GetFontSize() * 8);
            DragInt("Offset X", &offset_x, 1.0, -1000, 1000);

            SetCursorPosX(GetCursorPosX() + offset_x);
            PushStyleColor(ImGuiCol_ChildBg, IM_COL32(255, 0, 0, 100));
            BeginChild("Red", ImVec2::new(200, 100), true, ImGuiWindowFlags_None);
            for (let n: c_int = 0; n < 50; n++)
                Text("Some test {}", n);
            EndChild();
            let mut child_is_hovered: bool =  IsItemHovered();
            let child_rect_min: ImVec2 = GetItemRectMin();
            let child_rect_max: ImVec2 = GetItemRectMax();
            PopStyleColor();
            Text("Hovered: {}", child_is_hovered);
            Text("Rect of child window is: ({},{}) ({},{})", child_rect_min.x, child_rect_min.y, child_rect_max.x, child_rect_max.y);
        }

        TreePop();
    }

    IMGUI_DEMO_MARKER("Layout/Widgets Width");
    if (TreeNode("Widgets Width"))
    {
        static let f: c_float =  0.0;
        static let mut show_indented_items: bool =  true;
        Checkbox("Show indented items", &show_indented_items);

        // Use SetNextItemWidth() to set the width of a single upcoming item.
        // Use PushItemWidth()/PopItemWidth() to set the width of a group of items.
        // In real code use you'll probably want to choose width values that are proportional to your font size
        // e.g. Using '20f32 * GetFontSize()' as width instead of '200', etc.

        Text("SetNextItemWidth/PushItemWidth(100)");
        SameLine(); HelpMarker("Fixed width.");
        PushItemWidth(100);
        DragFloat("float##1b", &0.0);
        if (show_indented_items)
        {
            Indent();
            DragFloat("float (indented)##1b", &0.0);
            Unindent();
        }
        PopItemWidth();

        Text("SetNextItemWidth/PushItemWidth(-100)");
        SameLine(); HelpMarker("Align to right edge minus 100");
        PushItemWidth(-100);
        DragFloat("float##2a", &0.0);
        if (show_indented_items)
        {
            Indent();
            DragFloat("float (indented)##2b", &0.0);
            Unindent();
        }
        PopItemWidth();

        Text("SetNextItemWidth/PushItemWidth(GetContentRegionAvail().x * 0.5)");
        SameLine(); HelpMarker("Half of available width.\n(~ right-cursor_pos)\n(works within a column set)");
        PushItemWidth(GetContentRegionAvail().x * 0.5);
        DragFloat("float##3a", &0.0);
        if (show_indented_items)
        {
            Indent();
            DragFloat("float (indented)##3b", &0.0);
            Unindent();
        }
        PopItemWidth();

        Text("SetNextItemWidth/PushItemWidth(-GetContentRegionAvail().x * 0.5)");
        SameLine(); HelpMarker("Align to right edge minus half");
        PushItemWidth(-GetContentRegionAvail().x * 0.5);
        DragFloat("float##4a", &0.0);
        if (show_indented_items)
        {
            Indent();
            DragFloat("float (indented)##4b", &0.0);
            Unindent();
        }
        PopItemWidth();

        // Demonstrate using PushItemWidth to surround three items.
        // Calling SetNextItemWidth() before each of them would have the same effect.
        Text("SetNextItemWidth/PushItemWidth(-FLT_MIN)");
        SameLine(); HelpMarker("Align to right edge");
        PushItemWidth(-FLT_MIN);
        DragFloat("##float5a", &0.0);
        if (show_indented_items)
        {
            Indent();
            DragFloat("float (indented)##5b", &0.0);
            Unindent();
        }
        PopItemWidth();

        TreePop();
    }

    IMGUI_DEMO_MARKER("Layout/Basic Horizontal Layout");
    if (TreeNode("Basic Horizontal Layout"))
    {
        TextWrapped("(Use SameLine() to keep adding items to the right of the preceding item)");

        // Text
        IMGUI_DEMO_MARKER("Layout/Basic Horizontal Layout/SameLine");
        Text("Two items: Hello"); SameLine();
        TextColored(ImVec4(1,1,0,1), "Sailor");

        // Adjust spacing
        Text("More spacing: Hello"); SameLine(0, 20);
        TextColored(ImVec4(1,1,0,1), "Sailor");

        // Button
        AlignTextToFramePadding();
        Text("Normal buttons"); SameLine();
        Button("Banana"); SameLine();
        Button("Apple"); SameLine();
        Button("Corniflower");

        // Button
        Text("Small buttons"); SameLine();
        SmallButton("Like this one"); SameLine();
        Text("can fit within a text block.");

        // Aligned to arbitrary position. Easy/cheap column.
        IMGUI_DEMO_MARKER("Layout/Basic Horizontal Layout/SameLine (with offset)");
        Text("Aligned");
        SameLine(150); Text("x=150");
        SameLine(300); Text("x=300");
        Text("Aligned");
        SameLine(150); SmallButton("x=150");
        SameLine(300); SmallButton("x=300");

        // Checkbox
        IMGUI_DEMO_MARKER("Layout/Basic Horizontal Layout/SameLine (more)");
        static let mut c1: bool =  false, c2 = false, c3 = false, c4 = false;
        Checkbox("My", &c1); SameLine();
        Checkbox("Tailor", &c2); SameLine();
        Checkbox("Is", &c3); SameLine();
        Checkbox("Rich", &c4);

        // Various
        static let f0: c_float =  1.0, f1 = 2.0, f2 = 3.0;
        PushItemWidth(80);
        items: *const c_char[] = { "AAAA", "BBBB", "CCCC", "DDDD" };
        static let item: c_int = -1;
        Combo("Combo", &item, items, items.len()); SameLine();
        SliderFloat("X", &f0, 0.0, 5.0); SameLine();
        SliderFloat("Y", &f1, 0.0, 5.0); SameLine();
        SliderFloat("Z", &f2, 0.0, 5.0);
        PopItemWidth();

        PushItemWidth(80);
        Text("Lists:");
        static selection: c_int[4] = { 0, 1, 2, 3 };
        for (let i: c_int = 0; i < 4; i++)
        {
            if i > 0 {  SameLine(); }
            PushID(i);
            ListBox("", &selection[i], items, items.len());
            PopID();
            //if (IsItemHovered()) SetTooltip("ListBox {} hovered", i);
        }
        PopItemWidth();

        // Dummy
        IMGUI_DEMO_MARKER("Layout/Basic Horizontal Layout/Dummy");
        button_sz: ImVec2::new(40, 40);
        Button("A", button_sz); SameLine();
        Dummy(button_sz); SameLine();
        Button("B", button_sz);

        // Manually wrapping
        // (we should eventually provide this as an automatic layout feature, but for now you can do it manually)
        IMGUI_DEMO_MARKER("Layout/Basic Horizontal Layout/Manual wrapping");
        Text("Manual wrapping:");
        ImGuiStyle& style = GetStyle();
        let buttons_count: c_int = 20;
        let window_visible_x2: c_float =  GetWindowPos().x + GetWindowContentRegionMax().x;
        for (let n: c_int = 0; n < buttons_count; n++)
        {
            PushID(n);
            Button("Box", button_sz);
            let last_button_x2: c_float =  GetItemRectMax().x;
            let next_button_x2: c_float =  last_button_x2 + style.ItemSpacing.x + button_sz.x; // Expected position if next button was on same line
            if n + 1 < buttons_count && next_button_x2 < window_visible_x2{
                SameLine();}
            PopID();
        }

        TreePop();
    }

    IMGUI_DEMO_MARKER("Layout/Groups");
    if (TreeNode("Groups"))
    {
        HelpMarker(
            "BeginGroup() basically locks the horizontal position for new line. "
            "EndGroup() bundles the whole group so that you can use \"item\" functions such as "
            "IsItemHovered()/IsItemActive() or SameLine() etc. on the whole group.");
        BeginGroup();
        {
            BeginGroup();
            Button("AAA");
            SameLine();
            Button("BBB");
            SameLine();
            BeginGroup();
            Button("CCC");
            Button("DDD");
            EndGroup();
            SameLine();
            Button("EEE");
            EndGroup();
            if IsItemHovered() {
                SetTooltip("First group hovered")(); }
        }
        // Capture the group size and create widgets using the same size
        let size: ImVec2 = GetItemRectSize();
        values: c_float[5] = { 0.5, 0.20, 0.80, 0.60, 0.25f };
        PlotHistogram("##values", values, values.len(), 0, None, 0.0, 1.0, size);

        Button("ACTION", ImVec2::new((size.x - GetStyle().ItemSpacing.x) * 0.5, size.y));
        SameLine();
        Button("REACTION", ImVec2::new((size.x - GetStyle().ItemSpacing.x) * 0.5, size.y));
        EndGroup();
        SameLine();

        Button("LEVERAGE\nBUZZWORD", size);
        SameLine();

        if (BeginListBox("List", size))
        {
            Selectable("Selected", true);
            Selectable("Not Selected", false);
            EndListBox();
        }

        TreePop();
    }

    IMGUI_DEMO_MARKER("Layout/Text Baseline Alignment");
    if (TreeNode("Text Baseline Alignment"))
    {
        {
            BulletText("Text baseline:");
            SameLine(); HelpMarker(
                "This is testing the vertical alignment that gets applied on text to keep it aligned with widgets. "
                "Lines only composed of text or \"small\" widgets use less vertical space than lines with framed widgets.");
            Indent();

            Text("KO Blahblah"); SameLine();
            Button("Some framed item"); SameLine();
            HelpMarker("Baseline of button will look misaligned with text..");

            // If your line starts with text, call AlignTextToFramePadding() to align text to upcoming widgets.
            // (because we don't know what's coming after the Text() statement, we need to move the text baseline
            // down by FramePadding.y ahead of time)
            AlignTextToFramePadding();
            Text("OK Blahblah"); SameLine();
            Button("Some framed item"); SameLine();
            HelpMarker("We call AlignTextToFramePadding() to vertically align the text baseline by +FramePadding.y");

            // SmallButton() uses the same vertical padding as Text
            Button("TEST##1"); SameLine();
            Text("TEST"); SameLine();
            SmallButton("TEST##2");

            // If your line starts with text, call AlignTextToFramePadding() to align text to upcoming widgets.
            AlignTextToFramePadding();
            Text("Text aligned to framed item"); SameLine();
            Button("Item##1"); SameLine();
            Text("Item"); SameLine();
            SmallButton("Item##2"); SameLine();
            Button("Item##3");

            Unindent();
        }

        Spacing();

        {
            BulletText("Multi-line text:");
            Indent();
            Text("One\nTwo\nThree"); SameLine();
            Text("Hello\nWorld"); SameLine();
            Text("Banana");

            Text("Banana"); SameLine();
            Text("Hello\nWorld"); SameLine();
            Text("One\nTwo\nThree");

            Button("HOP##1"); SameLine();
            Text("Banana"); SameLine();
            Text("Hello\nWorld"); SameLine();
            Text("Banana");

            Button("HOP##2"); SameLine();
            Text("Hello\nWorld"); SameLine();
            Text("Banana");
            Unindent();
        }

        Spacing();

        {
            BulletText("Misc items:");
            Indent();

            // SmallButton() sets FramePadding to zero. Text baseline is aligned to match baseline of previous Button.
            Button("80x80", ImVec2::new(80, 80));
            SameLine();
            Button("50x50", ImVec2::new(50, 50));
            SameLine();
            Button("Button()");
            SameLine();
            SmallButton("SmallButton()");

            // Tree
            let spacing: c_float =  GetStyle().ItemInnerSpacing.x;
            Button("Button##1");
            SameLine(0.0, spacing);
            if (TreeNode("Node##1"))
            {
                // Placeholder tree data
                for (let i: c_int = 0; i < 6; i++)
                    BulletText("Item {}..", i);
                TreePop();
            }

            // Vertically align text node a bit lower so it'll be vertically centered with upcoming widget.
            // Otherwise you can use SmallButton() (smaller fit).
            AlignTextToFramePadding();

            // Common mistake to avoid: if we want to SameLine after TreeNode we need to do it before we add
            // other contents below the node.
            let mut node_open: bool =  TreeNode("Node##2");
            SameLine(0.0, spacing); Button("Button##2");
            if (node_open)
            {
                // Placeholder tree data
                for (let i: c_int = 0; i < 6; i++)
                    BulletText("Item {}..", i);
                TreePop();
            }

            // Bullet
            Button("Button##3");
            SameLine(0.0, spacing);
            BulletText("Bullet text");

            AlignTextToFramePadding();
            BulletText("Node");
            SameLine(0.0, spacing); Button("Button##4");
            Unindent();
        }

        TreePop();
    }

    IMGUI_DEMO_MARKER("Layout/Scrolling");
    if (TreeNode("Scrolling"))
    {
        // Vertical scroll functions
        IMGUI_DEMO_MARKER("Layout/Scrolling/Vertical");
        HelpMarker("Use SetScrollHereY() or SetScrollFromPosY() to scroll to a given vertical position.");

        static let track_item: c_int = 50;
        static let mut enable_track: bool =  true;
        static let mut enable_extra_decorations: bool =  false;
        static let scroll_to_off_px: c_float =  0.0;
        static let scroll_to_pos_px: c_float =  200;

        Checkbox("Decoration", &enable_extra_decorations);

        Checkbox("Track", &enable_track);
        PushItemWidth(100);
        SameLine(140); enable_track |= DragInt("##item", &track_item, 0.25f, 0, 99, "Item = {}");

        let mut scroll_to_off: bool =  Button("Scroll Offset");
        SameLine(140); scroll_to_off |= DragFloat("##off", &scroll_to_off_px, 1.0, 0, f32::MAX, "+{} px");

        let mut scroll_to_pos: bool =  Button("Scroll To Pos");
        SameLine(140); scroll_to_pos |= DragFloat("##pos", &scroll_to_pos_px, 1.0, -10, f32::MAX, "X/Y = {} px");
        PopItemWidth();

        if scroll_to_off || scroll_to_pos {
            enable_track = false;}

        ImGuiStyle& style = GetStyle();
        let child_w: c_float =  (GetContentRegionAvail().x - 4 * style.ItemSpacing.x) / 5;
        if child_w < 1.0{
            child_w = 1.0;}
        PushID("##VerticalScrolling");
        for (let i: c_int = 0; i < 5; i++)
        {
            if i > 0 {  SameLine(); }
            BeginGroup();
            names: *const c_char[] = { "Top", "25%", "Center", "75%", "Bottom" };
            TextUnformatted(names[i]);

            const child_flags: ImGuiWindowFlags = if enable_extra_decorations { ImGuiWindowFlags_MenuBar } else { 0 };
            let mut child_id: ImguiHandle =  GetID(i);
            let child_is_visible: bool = BeginChild(child_id, ImVec2::new(child_w, 200), true, child_flags);
            if (BeginMenuBar())
            {
                TextUnformatted("abc");
                EndMenuBar();
            }
            if scroll_to_off {
                SetScrollY(scroll_to_off_px)(); }
            if (scroll_to_pos)
                SetScrollFromPosY(GetCursorStartPos().y + scroll_to_pos_px, i * 0.250f32);
            if (child_is_visible) // Avoid calling SetScrollHereY when running with culled items
            {
                for (let item: c_int = 0; item < 100; item++)
                {
                    if (enable_track && item == track_item)
                    {
                        TextColored(ImVec4(1, 1, 0, 1), "Item {}", item);
                        SetScrollHereY(i * 0.250f32); // 0.0:top, 0.5:center, 1.0f:bottom
                    }
                    else
                    {
                        Text("Item {}", item);
                    }
                }
            }
            let scroll_y: c_float =  GetScrollY();
            let scroll_max_y: c_float =  GetScrollMaxY();
            EndChild();
            Text("{}f/{}f", scroll_y, scroll_max_y);
            EndGroup();
        }
        PopID();

        // Horizontal scroll functions
        IMGUI_DEMO_MARKER("Layout/Scrolling/Horizontal");
        Spacing();
        HelpMarker(
            "Use SetScrollHereX() or SetScrollFromPosX() to scroll to a given horizontal position.\n\n"
            "Because the clipping rectangle of most window hides half worth of WindowPadding on the "
            "left/right, using SetScrollFromPosX(+1) will usually result in clipped text whereas the "
            "equivalent SetScrollFromPosY(+1) wouldn't.");
        PushID("##HorizontalScrolling");
        for (let i: c_int = 0; i < 5; i++)
        {
            let child_height: c_float =  GetTextLineHeight() + style.ScrollbarSize + style.WindowPadding.y * 2.0;
            child_flags: ImGuiWindowFlags = ImGuiWindowFlags_HorizontalScrollbar | (if enable_extra_decorations { ImGuiWindowFlags_AlwaysVerticalScrollbar }else { 0 });
            let mut child_id: ImguiHandle =  GetID(i);
            let mut child_is_visible: bool =  BeginChild(child_id, ImVec2::new(-100, child_height), true, child_flags);
            if scroll_to_off {
                SetScrollX(scroll_to_off_px)(); }
            if (scroll_to_pos)
                SetScrollFromPosX(GetCursorStartPos().x + scroll_to_pos_px, i * 0.250f32);
            if (child_is_visible) // Avoid calling SetScrollHereY when running with culled items
            {
                for (let item: c_int = 0; item < 100; item++)
                {
                    if item > 0 {
                        SameLine(); }
                    if (enable_track && item == track_item)
                    {
                        TextColored(ImVec4(1, 1, 0, 1), "Item {}", item);
                        SetScrollHereX(i * 0.250f32); // 0.0:left, 0.5:center, 1.0f:right
                    }
                    else
                    {
                        Text("Item {}", item);
                    }
                }
            }
            let scroll_x: c_float =  GetScrollX();
            let scroll_max_x: c_float =  GetScrollMaxX();
            EndChild();
            SameLine();
            names: *const c_char[] = { "Left", "25%", "Center", "75%", "Right" };
            Text("{}\n{}f/{}f", names[i], scroll_x, scroll_max_x);
            Spacing();
        }
        PopID();

        // Miscellaneous Horizontal Scrolling Demo
        IMGUI_DEMO_MARKER("Layout/Scrolling/Horizontal (more)");
        HelpMarker(
            "Horizontal scrolling for a window is enabled via the ImGuiWindowFlags_HorizontalScrollbar flag.\n\n"
            "You may want to also explicitly specify content width by using SetNextWindowContentWidth() before Begin().");
        static let lines: c_int = 7;
        SliderInt("Lines", &lines, 1, 15);
        PushStyleVar(ImGuiStyleVar_FrameRounding, 3.0);
        PushStyleVar(ImGuiStyleVar_FramePadding, ImVec2::new(2.0, 1.0));
        let scrolling_child_size: ImVec2 = ImVec2::new(0, GetFrameHeightWithSpacing() * 7 + 30);
        BeginChild("scrolling", scrolling_child_size, true, ImGuiWindowFlags_HorizontalScrollbar);
        for (let line: c_int = 0; line < lines; line++)
        {
            // Display random stuff. For the sake of this trivial demo we are using basic Button() + SameLine()
            // If you want to create your own time line for a real application you may be better off manipulating
            // the cursor position yourself, aka using SetCursorPos/SetCursorScreenPos to position the widgets
            // yourself. You may also want to use the lower-level ImDrawList API.
            let num_buttons: c_int = 10 + (if (line & 1) { line * 9 }else { line * 3 });
            for (let n: c_int = 0; n < num_buttons; n++)
            {
                if n > 0 {  SameLine(); }
                PushID(n + line * 1000);
                num_buf: [c_char;16];
                sprintf(num_buf, "{}", n);
                let mut  label: *const c_char = if !(n % 15) { "FizzBuzz" } else {if (!(n % 3)) { "Fizz"} else {if (!(n % 5) { "Buzz"} else { num_buf}}};
                let hue: c_float =  n * 0.05f32;
                PushStyleColor(ImGuiCol_Button, (ImVec4)ImColor::HSV(hue, 0.6f, 0.60));
                PushStyleColor(ImGuiCol_ButtonHovered, (ImVec4)ImColor::HSV(hue, 0.7f, 0.70f32));
                PushStyleColor(ImGuiCol_ButtonActive, (ImVec4)ImColor::HSV(hue, 0.8f, 0.80));
                Button(label, ImVec2::new(40f32 + sinf((line + n)) * 20f32, 0.0));
                PopStyleColor(3);
                PopID();
            }
        }
        let scroll_x: c_float =  GetScrollX();
        let scroll_max_x: c_float =  GetScrollMaxX();
        EndChild();
        PopStyleVar(2);
        let scroll_x_delta: c_float =  0.0;
        SmallButton("<<");
        if (IsItemActive())
            scroll_x_delta = -GetIO().DeltaTime * 1000;
        SameLine();
        Text("Scroll from code"); SameLine();
        SmallButton(">>");
        if (IsItemActive())
            scroll_x_delta = +GetIO().DeltaTime * 1000;
        SameLine();
        Text("{}f/{}f", scroll_x, scroll_max_x);
        if (scroll_x_delta != 0.0)
        {
            // Demonstrate a trick: you can use Begin to set yourself in the context of another window
            // (here we are already out of your child window)
            BeginChild("scrolling");
            SetScrollX(GetScrollX() + scroll_x_delta);
            EndChild();
        }
        Spacing();

        static let mut show_horizontal_contents_size_demo_window: bool =  false;
        Checkbox("Show Horizontal contents size demo window", &show_horizontal_contents_size_demo_window);

        if (show_horizontal_contents_size_demo_window)
        {
            static let mut show_h_scrollbar: bool =  true;
            static let mut show_button: bool =  true;
            static let mut show_tree_nodes: bool =  true;
            static let mut show_text_wrapped: bool =  false;
            static let mut show_columns: bool =  true;
            static let mut show_tab_bar: bool =  true;
            static let mut show_child: bool =  false;
            static let mut explicit_content_size: bool =  false;
            static let contents_size_x: c_float =  300.0;
            if (explicit_content_size) {
                SetNextWindowContentSize(ImVec2::new(contents_size_x, 0.0));
            }
            Begin("Horizontal contents size demo window", &show_horizontal_contents_size_demo_window, if show_h_scrollbar { ImGuiWindowFlags_HorizontalScrollbar } else { 0 });
            IMGUI_DEMO_MARKER("Layout/Scrolling/Horizontal contents size demo window");
            PushStyleVar(ImGuiStyleVar_ItemSpacing, ImVec2::new(2, 0));
            PushStyleVar(ImGuiStyleVar_FramePadding, ImVec2::new(2, 0));
            HelpMarker("Test of different widgets react and impact the work rectangle growing when horizontal scrolling is enabled.\n\nUse 'Metrics->Tools->Show windows rectangles' to visualize rectangles.");
            Checkbox("H-scrollbar", &show_h_scrollbar);
            Checkbox("Button", &show_button);            // Will grow contents size (unless explicitly overwritten)
            Checkbox("Tree nodes", &show_tree_nodes);    // Will grow contents size and display highlight over full width
            Checkbox("Text wrapped", &show_text_wrapped);// Will grow and use contents size
            Checkbox("Columns", &show_columns);          // Will use contents size
            Checkbox("Tab bar", &show_tab_bar);          // Will use contents size
            Checkbox("Child", &show_child);              // Will grow and use contents size
            Checkbox("Explicit content size", &explicit_content_size);
            Text("Scroll {}/{} {}/{}", GetScrollX(), GetScrollMaxX(), GetScrollY(), GetScrollMaxY());
            if explicit_content_size
            {
                SameLine();
                SetNextItemWidth(100);
                DragFloat("##csx", &contents_size_x);
                let p: ImVec2 = GetCursorScreenPos();
                GetWindowDrawList().AddRectFilled(p, ImVec2::new(p.x + 10, p.y + 10), IM_COL32_WHITE);
                GetWindowDrawList().AddRectFilled(ImVec2::new(p.x + contents_size_x - 10, p.y), ImVec2::new(p.x + contents_size_x, p.y + 10), IM_COL32_WHITE);
                Dummy(ImVec2::new(0, 10));
            }
            PopStyleVar(2);
            Separator();
            if show_button
            {
                Button("this is a 300-wide button", ImVec2::new(300, 0));
            }
            if show_tree_nodes
            {
                let mut open: bool =  true;
                if TreeNode("this is a tree node")
                {
                    if TreeNode("another one of those tree node...")
                    {
                        Text("Some tree contents");
                        TreePop();
                    }
                    TreePop();
                }
                CollapsingHeader("CollapsingHeader", &open);
            }
            if show_text_wrapped
            {
                TextWrapped("This text should automatically wrap on the edge of the work rectangle.");
            }
            if show_columns
            {
                Text("Tables:");
                if (BeginTable("table", 4, ImGuiTableFlags_Borders))
                {
                    for (let n: c_int = 0; n < 4; n++)
                    {
                        TableNextColumn();
                        Text("Width {}", GetContentRegionAvail().x);
                    }
                    EndTable();
                }
                Text("Columns:");
                Columns(4);
                for (let n: c_int = 0; n < 4; n++)
                {
                    Text("Width {}", GetColumnWidth());
                    NextColumn();
                }
                Columns(1);
            }
            if (show_tab_bar && BeginTabBar("Hello"))
            {
                if (BeginTabItem("OneOneOne")) { EndTabItem(); }
                if (BeginTabItem("TwoTwoTwo")) { EndTabItem(); }
                if (BeginTabItem("ThreeThreeThree")) { EndTabItem(); }
                if (BeginTabItem("FourFourFour")) { EndTabItem(); }
                EndTabBar();
            }
            if (show_child)
            {
                BeginChild("child", ImVec2::new(0, 0), true);
                EndChild();
            }
            End();
        }

        TreePop();
    }

    IMGUI_DEMO_MARKER("Layout/Clipping");
    if (TreeNode("Clipping"))
    {
        static size: ImVec2::new(100, 100);
        static offset: ImVec2::new(30f32, 30f32);
        DragFloat2("size", (&mut c_float)&size, 0.5, 1.0, 200, "{}f");
        TextWrapped("(Click and drag to scroll)");

        HelpMarker(
            "(Left) Using PushClipRect():\n"
            "Will alter ImGui hit-testing logic + ImDrawList rendering.\n"
            "(use this if you want your clipping rectangle to affect interactions)\n\n"
            "(Center) Using ImDrawList::PushClipRect():\n"
            "Will alter ImDrawList rendering only.\n"
            "(use this as a shortcut if you are only using ImDrawList calls)\n\n"
            "(Right) Using ImDrawList::AddText() with a fine ClipRect:\n"
            "Will alter only this specific ImDrawList::AddText() rendering.\n"
            "This is often used internally to avoid altering the clipping rectangle and minimize draw calls.");

        for (let n: c_int = 0; n < 3; n++)
        {
            if n > 0 {
                SameLine(); }

            PushID(n);
            InvisibleButton("##canvas", size);
            if (IsItemActive() && IsMouseDragging(ImGuiMouseButton_Left))
            {
                offset.x += GetIO().MouseDelta.x;
                offset.y += GetIO().MouseDelta.y;
            }
            PopID();
            if (!IsItemVisible()) // Skip rendering as ImDrawList elements are not clipped.
                continue;

            let p0: ImVec2 = GetItemRectMin();
            let p1: ImVec2 = GetItemRectMax();
            let mut  text_str: *const c_char = "Line 1 hello\nLine 2 clip me!";
            let text_pos: ImVec2 = ImVec2::new(p0.x + offset.x, p0.y + offset.y);
            draw_list: *mut ImDrawList = GetWindowDrawList();
            switch (n)
            {
            0 =>
                PushClipRect(p0, p1, true);
                draw_list.AddRectFilled(p0, p1, IM_COL32(90, 90, 120, 255));
                draw_list.AddText(text_pos, IM_COL32_WHITE, text_str);
                PopClipRect();
                break;
            1 =>
                draw_list.PushClipRect(p0, p1, true);
                draw_list.AddRectFilled(p0, p1, IM_COL32(90, 90, 120, 255));
                draw_list.AddText(text_pos, IM_COL32_WHITE, text_str);
                draw_list.PopClipRect();
                break;
            2 =>
                let mut clip_rect = ImVec4::new(p0.x, p0.y, p1.x, p1.y); // AddText() takes a ImVec4* here so let's convert.
                draw_list.AddRectFilled(p0, p1, IM_COL32(90, 90, 120, 255));
                draw_list.AddText(GetFont(), GetFontSize(), text_pos, IM_COL32_WHITE, text_str, None, 0.0, &clip_rect);
                break;
            }
        }

        TreePop();
    }
}

pub unsafe fn ShowDemoWindowPopups()
{
    IMGUI_DEMO_MARKER("Popups");
    if !CollapsingHeader("Popups & Modal windows") { return ; }

    // The properties of popups windows are:
    // - They block normal mouse hovering detection outside them. (*)
    // - Unless modal, they can be closed by clicking anywhere outside them, or by pressing ESCAPE.
    // - Their visibility state (~bool) is held internally by Dear ImGui instead of being held by the programmer as
    //   we are used to with regular Begin() calls. User can manipulate the visibility state by calling OpenPopup().
    // (*) One can use IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup) to bypass it and detect hovering even
    //     when normally blocked by a popup.
    // Those three properties are connected. The library needs to hold their visibility state BECAUSE it can close
    // popups at any time.

    // Typical use for regular windows:
    //   my_tool_is_active: bool = false; if (Button("Open")) my_tool_is_active = true; [...] if (my_tool_is_active) Begin("My Tool", &my_tool_is_active) { [...] } End();
    // Typical use for popups:
    //   if (Button("Open")) OpenPopup("MyPopup"); if (BeginPopup("MyPopup") { [...] EndPopup(); }

    // With popups we have to go through a library call (here OpenPopup) to manipulate the visibility state.
    // This may be a bit confusing at first but it should quickly make sense. Follow on the examples below.

    IMGUI_DEMO_MARKER("Popups/Popups");
    if (TreeNode("Popups"))
    {
        TextWrapped(
            "When a popup is active, it inhibits interacting with windows that are behind the popup. "
            "Clicking outside the popup closes it.");

        static let selected_fish: c_int = -1;
        names: *const c_char[] = { "Bream", "Haddock", "Mackerel", "Pollock", "Tilefish" };
        static toggles: bool[] = { true, false, false, false, false };

        // Simple selection popup (if you want to show the current selection inside the Button itself,
        // you may want to build a string using the "###" operator to preserve a constant ID with a variable label)
        if (Button("Select.."))
            OpenPopup("my_select_popup");
        SameLine();
        TextUnformatted(selected_fish == -1 ? "<None>" : names[selected_fish]);
        if (BeginPopup("my_select_popup"))
        {
            Text("Aquarium");
            Separator();
            for (let i: c_int = 0; i < names.len(); i++)
                if Selectable(names[i]) {
                    selected_fish = i;}
            EndPopup();
        }

        // Showing a menu with toggles
        if (Button("Toggle.."))
            OpenPopup("my_toggle_popup");
        if (BeginPopup("my_toggle_popup"))
        {
            for (let i: c_int = 0; i < names.len(); i++)
                MenuItem(names[i], "", &toggles[i]);
            if (BeginMenu("Sub-menu"))
            {
                MenuItem("Click me");
                EndMenu();
            }

            Separator();
            Text("Tooltip here");
            if IsItemHovered() {
                SetTooltip("I am a tooltip over a popup")(); }

            if (Button("Stacked Popup"))
                OpenPopup("another popup");
            if (BeginPopup("another popup"))
            {
                for (let i: c_int = 0; i < names.len(); i++)
                    MenuItem(names[i], "", &toggles[i]);
                if (BeginMenu("Sub-menu"))
                {
                    MenuItem("Click me");
                    if (Button("Stacked Popup"))
                        OpenPopup("another popup");
                    if (BeginPopup("another popup"))
                    {
                        Text("I am the last one here.");
                        EndPopup();
                    }
                    EndMenu();
                }
                EndPopup();
            }
            EndPopup();
        }

        // Call the more complete ShowExampleMenuFile which we use in various places of this demo
        if (Button("With a menu.."))
            OpenPopup("my_file_popup");
        if (BeginPopup("my_file_popup", ImGuiWindowFlags_MenuBar))
        {
            if (BeginMenuBar())
            {
                if (BeginMenu("File"))
                {
                    ShowExampleMenuFile();
                    EndMenu();
                }
                if (BeginMenu("Edit"))
                {
                    MenuItem("Dummy");
                    EndMenu();
                }
                EndMenuBar();
            }
            Text("Hello from popup!");
            Button("This is a dummy button..");
            EndPopup();
        }

        TreePop();
    }

    IMGUI_DEMO_MARKER("Popups/Context menus");
    if (TreeNode("Context menus"))
    {
        HelpMarker("\"Context\" functions are simple helpers to associate a Popup to a given Item or Window identifier.");

        // BeginPopupContextItem() is a helper to provide common/simple popup behavior of essentially doing:
        //     if (id == 0)
        //         id = GetItemID(); // Use last item id
        //     if (IsItemHovered() && IsMouseReleased(ImGuiMouseButton_Right))
        //         OpenPopup(id);
        //     return BeginPopup(id);
        // For advanced advanced uses you may want to replicate and customize this code.
        // See more details in BeginPopupContextItem().

        // Example 1
        // When used after an item that has an ID (e.g. Button), we can skip providing an ID to BeginPopupContextItem(),
        // and BeginPopupContextItem() will use the last item ID as the popup ID.
        {
            *const names: [c_char;5] = { "Label1", "Label2", "Label3", "Label4", "Label5" };
            for (let n: c_int = 0; n < 5; n++)
            {
                Selectable(names[n]);
                if (BeginPopupContextItem()) // <-- use last item id as popup id
                {
                    Text("This a popup for \"{}\"!", names[n]);
                    if (Button("Close"))
                        CloseCurrentPopup();
                    EndPopup();
                }
                if (IsItemHovered())
                    SetTooltip("Right-click to open popup");
            }
        }

        // Example 2
        // Popup on a Text() element which doesn't have an identifier: we need to provide an identifier to BeginPopupContextItem().
        // Using an explicit identifier is also convenient if you want to activate the popups from different locations.
        {
            HelpMarker("Text() elements don't have stable identifiers so we need to provide one.");
            static let value: c_float =  0.5;
            Text("Value = {} <-- (1) right-click this text", value);
            if (BeginPopupContextItem("my popup"))
            {
                if (Selectable("Set to zero")) value = 0.0;
                if (Selectable("Set to PI")) value = 3.1415f32;
                SetNextItemWidth(-FLT_MIN);
                DragFloat("##Value", &value, 0.1f, 0.0, 0.0);
                EndPopup();
            }

            // We can also use OpenPopupOnItemClick() to toggle the visibility of a given popup.
            // Here we make it that right-clicking this other text element opens the same popup as above.
            // The popup itself will be submitted by the code above.
            Text("(2) Or right-click this text");
            OpenPopupOnItemClick("my popup", ImGuiPopupFlags_MouseButtonRight);

            // Back to square one: manually open the same popup.
            if (Button("(3) Or click this button"))
                OpenPopup("my popup");
        }

        // Example 3
        // When using BeginPopupContextItem() with an implicit identifier (NULL == use last item ID),
        // we need to make sure your item identifier is stable.
        // In this example we showcase altering the item label while preserving its identifier, using the ### operator (see FAQ).
        {
            HelpMarker("Showcase using a popup ID linked to item ID, with the item having a changing label + stable ID using the ### operator.");
            static name: [c_char;32] = "Label1";
            buf: [c_char;64];
            sprintf(buf, "Button: {}###Button", name); // ### operator override ID ignoring the preceding label
            Button(buf);
            if (BeginPopupContextItem())
            {
                Text("Edit name:");
                InputText("##edit", name, name.len());
                if (Button("Close"))
                    CloseCurrentPopup();
                EndPopup();
            }
            SameLine(); Text("(<-- right-click here)");
        }

        TreePop();
    }

    IMGUI_DEMO_MARKER("Popups/Modals");
    if (TreeNode("Modals"))
    {
        TextWrapped("Modal windows are like popups but the user cannot close them by clicking outside.");

        if (Button("Delete.."))
            OpenPopup("Delete?");

        // Always center this window when appearing
        let center: ImVec2 = GetMainViewport()->GetCenter();
        SetNextWindowPos(center, ImGuiCond_Appearing, ImVec2::new(0.5, 0.5));

        if (BeginPopupModal("Delete?", None, ImGuiWindowFlags_AlwaysAutoResize))
        {
            Text("All those beautiful files will be deleted.\nThis operation cannot be undone!\n\n");
            Separator();

            //static int unused_i = 0;
            //Combo("Combo", &unused_i, "Delete\0Delete harder\0");

            static let mut dont_ask_me_next_time: bool =  false;
            PushStyleVar(ImGuiStyleVar_FramePadding, ImVec2::new(0, 0));
            Checkbox("Don't ask me next time", &dont_ask_me_next_time);
            PopStyleVar();

            if (Button("OK", ImVec2::new(120, 0))) { CloseCurrentPopup(); }
            SetItemDefaultFocus();
            SameLine();
            if (Button("Cancel", ImVec2::new(120, 0))) { CloseCurrentPopup(); }
            EndPopup();
        }

        if (Button("Stacked modals.."))
            OpenPopup("Stacked 1");
        if (BeginPopupModal("Stacked 1", None, ImGuiWindowFlags_MenuBar))
        {
            if (BeginMenuBar())
            {
                if (BeginMenu("File"))
                {
                    if (MenuItem("Some menu item")) {}
                    EndMenu();
                }
                EndMenuBar();
            }
            Text("Hello from Stacked The First\nUsing style.Colors[ImGuiCol_ModalWindowDimBg] behind it.");

            // Testing behavior of widgets stacking their own regular popups over the modal.
            static let item: c_int = 1;
            staticcolor: c_float[4] = { 0.4f, 0.7f, 0.0, 0.5 };
            Combo("Combo", &item, "aaaa\0bbbb\0cccc\0dddd\0eeee\0\0");
            ColorEdit4("color", color);

            if (Button("Add another modal.."))
                OpenPopup("Stacked 2");

            // Also demonstrate passing ato: *mut bool BeginPopupModal(), this will create a regular close button which
            // will close the popup. Note that the visibility state of popups is owned by imgui, so the input value
            // of the actually: bool doesn't matter here.
            let mut unused_open: bool =  true;
            if (BeginPopupModal("Stacked 2", &unused_open))
            {
                Text("Hello from Stacked The Second!");
                if (Button("Close"))
                    CloseCurrentPopup();
                EndPopup();
            }

            if (Button("Close"))
                CloseCurrentPopup();
            EndPopup();
        }

        TreePop();
    }

    IMGUI_DEMO_MARKER("Popups/Menus inside a regular window");
    if (TreeNode("Menus inside a regular window"))
    {
        TextWrapped("Below we are testing adding menu items to a regular window. It's rather unusual but should work!");
        Separator();

        MenuItem("Menu item", "CTRL+M");
        if (BeginMenu("Menu inside a regular window"))
        {
            ShowExampleMenuFile();
            EndMenu();
        }
        Separator();
        TreePop();
    }
}

// Dummy data structure that we use for the Table demo.
// (pre-C++11 doesn't allow us to instantiate ImVector<MyItem> template if this structure if defined inside the demo function)
namespace
{
// We are passing our own identifier to TableSetupColumn() to facilitate identifying columns in the sorting code.
// This identifier will be passed down into ImGuiTableSortSpec::ColumnUserID.
// But it is possible to omit the user id parameter of TableSetupColumn() and just use the column index instead! (ImGuiTableSortSpec::ColumnIndex)
// If you don't use sorting, you will generally never care about giving column an ID!
enum MyItemColumnID
{
    MyItemColumnID_ID,
    MyItemColumnID_Name,
    MyItemColumnID_Action,
    MyItemColumnID_Quantity,
    MyItemColumnID_Description
};

struct MyItem
{
    c_int         ID;
let Name: *const c_char;
    c_int         Quantity;

    // We have a problem which is affecting _only this demo_ and should not affect your code:
    // As we don't rely on std:: or other third-party library to compile dear imgui, we only have reliable access to qsort(),
    // however qsort doesn't allow passing user data to comparing function.
    // As a workaround, we are storing the sort specs in a static/global for the comparing function to access.
    // In your own use case you would probably pass the sort specs to your sorting/comparing functions directly and not use a global.
    // We could technically call TableGetSortSpecs() in CompareWithSortSpecs(), but considering that this function is called
    // very often by the sorting algorithm it would be a little wasteful.
    static *const ImGuiTableSortSpecs s_current_sort_specs;

    // Compare function to be used by qsort()
    : c_int CompareWithSortSpecs(lhs: *const c_void, rhs: *const c_void)
    {
        a: *const MyItem = (*const MyItem)lhs;
        b: *const MyItem = (*const MyItem)rhs;
        for (let n: c_int = 0; n < s_current_sort_specs.SpecsCount; n++)
        {
            // Here we identify columns using the ColumnUserID value that we ourselves passed to TableSetupColumn()
            // We could also choose to identify columns based on their index (sort_spec->ColumnIndex), which is simpler!
            sort_spec: *const ImGuiTableColumnSortSpecs = &s_current_sort_specs.Specs[n];
            let delta: c_int = 0;
            switch (sort_spec.ColumnUserID)
            {
            MyItemColumnID_ID =>             delta = (a.ID - b.ID);                break;
            MyItemColumnID_Name =>           delta = (strcmp(a.Name, b.Name));     break;
            MyItemColumnID_Quantity =>       delta = (a.Quantity - b.Quantity);    break;
            MyItemColumnID_Description =>    delta = (strcmp(a.Name, b.Name));     break;
            _ => IM_ASSERT(0); break;
            }
            if (delta > 0)
                return if (sort_spec->SortDirection == ImGuiSortDirection_Ascending) { 1} else {- 1};
            if (delta < 0)
                return if (sort_spec->SortDirection == ImGuiSortDirection_Ascending) { - 1} else {1};
        }

        // qsort() is instable so always return a way to differenciate items.
        // Your own compare function may want to avoid fallback on implicit sort specs e.g. a Name compare if it wasn't already part of the sort specs.
        return (a.ID - b.ID);
    }
};
*const ImGuiTableSortSpecs MyItem::s_current_sort_specs= None;
}

// Make the UI compact because there are so many fields
pub unsafe fn PushStyleCompact()
{
    ImGuiStyle& style = GetStyle();
    PushStyleVar(ImGuiStyleVar_FramePadding, ImVec2::new(style.FramePadding.x, (style.FramePadding.y * 0.60)));
    PushStyleVar(ImGuiStyleVar_ItemSpacing, ImVec2::new(style.ItemSpacing.x, (style.ItemSpacing.y * 0.60)));
}

pub unsafe fn PopStyleCompact()
{
    PopStyleVar(2);
}

// Show a combo box with a choice of sizing policies
pub unsafe fn EditTableSizingFlags(*mut p_flags: ImGuiTableFlags)
{
    struct EnumDesc { Value: ImGuiTableFlags; Name: *const c_char; Tooltip: *const c_char; };
    static const EnumDesc policies[] =
    {
        { ImGuiTableFlags_None,               "Default",                            "Use default sizing policy:\n- ImGuiTableFlags_SizingFixedFit if ScrollX is on or if host window has ImGuiWindowFlags_AlwaysAutoResize.\n- ImGuiTableFlags_SizingStretchSame otherwise." },
        { ImGuiTableFlags_SizingFixedFit,     "ImGuiTableFlags_SizingFixedFit",     "Columns default to _WidthFixed (if resizable) or _WidthAuto (if not resizable), matching contents width." },
        { ImGuiTableFlags_SizingFixedSame,    "ImGuiTableFlags_SizingFixedSame",    "Columns are all the same width, matching the maximum contents width.\nImplicitly disable ImGuiTableFlags_Resizable and enable ImGuiTableFlags_NoKeepColumnsVisible." },
        { ImGuiTableFlags_SizingStretchProp,  "ImGuiTableFlags_SizingStretchProp",  "Columns default to _WidthStretch with weights proportional to their widths." },
        { ImGuiTableFlags_SizingStretchSame,  "ImGuiTableFlags_SizingStretchSame",  "Columns default to _WidthStretch with same weights." }
    };
    let mut idx: c_int = 0;
    for (idx = 0; idx < policies.len(); idx++)
        if (policies[idx].Value == (*p_flags & ImGuiTableFlags_SizingMask_))
            break;
    let mut  preview_text: &String = if idx < policies.len() { policies[idx].Name + (idx > 0 ? strlen("ImGuiTableFlags") : 0)} else { ""};
    if (BeginCombo("Sizing Policy", preview_text))
    {
        for (let n: c_int = 0; n < policies.len(); n++)
            if (Selectable(policies[n].Name, idx == n))
                *p_flags = (*p_flags & !ImGuiTableFlags_SizingMask_) | policies[n].Value;
        EndCombo();
    }
    SameLine();
    TextDisabled("(?)");
    if (IsItemHovered())
    {
        BeginTooltip();
        PushTextWrapPos(GetFontSize() * 50f32);
        for (let m: c_int = 0; m < policies.len(); m++)
        {
            Separator();
            Text("{}:", policies[m].Name);
            Separator();
            SetCursorPosX(GetCursorPosX() + GetStyle().IndentSpacing * 0.5);
            TextUnformatted(policies[m].Tooltip);
        }
        PopTextWrapPos();
        EndTooltip();
    }
}

pub unsafe fn EditTableColumnsFlags(*mut p_flags: ImGuiTableColumnFlags)
{
    CheckboxFlags("_Disabled", p_flags, ImGuiTableColumnFlags_Disabled); SameLine(); HelpMarker("Master disable flag (also hide from context menu)");
    CheckboxFlags("_DefaultHide", p_flags, ImGuiTableColumnFlags_DefaultHide);
    CheckboxFlags("_DefaultSort", p_flags, ImGuiTableColumnFlags_DefaultSort);
    if (CheckboxFlags("_WidthStretch", p_flags, ImGuiTableColumnFlags_WidthStretch))
        *p_flags &= ~(ImGuiTableColumnFlags_WidthMask_ ^ ImGuiTableColumnFlags_WidthStretch);
    if (CheckboxFlags("_WidthFixed", p_flags, ImGuiTableColumnFlags_WidthFixed))
        *p_flags &= ~(ImGuiTableColumnFlags_WidthMask_ ^ ImGuiTableColumnFlags_WidthFixed);
    CheckboxFlags("_NoResize", p_flags, ImGuiTableColumnFlags_NoResize);
    CheckboxFlags("_NoReorder", p_flags, ImGuiTableColumnFlags_NoReorder);
    CheckboxFlags("_NoHide", p_flags, ImGuiTableColumnFlags_NoHide);
    CheckboxFlags("_NoClip", p_flags, ImGuiTableColumnFlags_NoClip);
    CheckboxFlags("_NoSort", p_flags, ImGuiTableColumnFlags_NoSort);
    CheckboxFlags("_NoSortAscending", p_flags, ImGuiTableColumnFlags_NoSortAscending);
    CheckboxFlags("_NoSortDescending", p_flags, ImGuiTableColumnFlags_NoSortDescending);
    CheckboxFlags("_NoHeaderLabel", p_flags, ImGuiTableColumnFlags_NoHeaderLabel);
    CheckboxFlags("_NoHeaderWidth", p_flags, ImGuiTableColumnFlags_NoHeaderWidth);
    CheckboxFlags("_PreferSortAscending", p_flags, ImGuiTableColumnFlags_PreferSortAscending);
    CheckboxFlags("_PreferSortDescending", p_flags, ImGuiTableColumnFlags_PreferSortDescending);
    CheckboxFlags("_IndentEnable", p_flags, ImGuiTableColumnFlags_IndentEnable); SameLine(); HelpMarker("Default for column 0");
    CheckboxFlags("_IndentDisable", p_flags, ImGuiTableColumnFlags_IndentDisable); SameLine(); HelpMarker("Default for column >0");
}

pub unsafe fn ShowTableColumnsStatusFlags(flags: ImGuiTableColumnFlags)
{
    CheckboxFlags("_IsEnabled", &flags, ImGuiTableColumnFlags_IsEnabled);
    CheckboxFlags("_IsVisible", &flags, ImGuiTableColumnFlags_IsVisible);
    CheckboxFlags("_IsSorted", &flags, ImGuiTableColumnFlags_IsSorted);
    CheckboxFlags("_IsHovered", &flags, ImGuiTableColumnFlags_IsHovered);
}

pub unsafe fn ShowDemoWindowTables()
{
    //SetNextItemOpen(true, ImGuiCond_Once);
    IMGUI_DEMO_MARKER("Tables");
    if !CollapsingHeader("Tables & Columns") { return ; }

    // Using those as a base value to create width/height that are factor of the size of our font
    let TEXT_BASE_WIDTH: c_float =  CalcTextSize("A").x;
    let TEXT_BASE_HEIGHT: c_float =  GetTextLineHeightWithSpacing();

    PushID("Tables");

    let open_action: c_int = -1;
    if Button("Open all") {
        open_action = 1;}
    SameLine();
    if Button("Close all") {
        open_action = 0;}
    SameLine();

    // Options
    static let mut disable_indent: bool =  false;
    Checkbox("Disable tree indentation", &disable_indent);
    SameLine();
    HelpMarker("Disable the indenting of tree nodes so demo tables can use the full window width.");
    Separator();
    if (disable_indent)
        PushStyleVar(ImGuiStyleVar_IndentSpacing, 0.0);

    // About Styling of tables
    // Most settings are configured on a per-table basis via the flags passed to BeginTable() and TableSetupColumns APIs.
    // There are however a few settings that a shared and part of the ImGuiStyle structure:
    //   style.CellPadding                          // Padding within each cell
    //   style.Colors[ImGuiCol_TableHeaderBg]       // Table header background
    //   style.Colors[ImGuiCol_TableBorderStrong]   // Table outer and header borders
    //   style.Colors[ImGuiCol_TableBorderLight]    // Table inner borders
    //   style.Colors[ImGuiCol_TableRowBg]          // Table row background when ImGuiTableFlags_RowBg is enabled (even rows)
    //   style.Colors[ImGuiCol_TableRowBgAlt]       // Table row background when ImGuiTableFlags_RowBg is enabled (odds rows)

    // Demos
    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Basic");
    if (TreeNode("Basic"))
    {
        // Here we will showcase three different ways to output a table.
        // They are very simple variations of a same thing!

        // [Method 1] Using TableNextRow() to create a new row, and TableSetColumnIndex() to select the column.
        // In many situations, this is the most flexible and easy to use pattern.
        HelpMarker("Using TableNextRow() + calling TableSetColumnIndex() _before_ each cell, in a loop.");
        if (BeginTable("table1", 3))
        {
            for (let row: c_int = 0; row < 4; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < 3; column++)
                {
                    TableSetColumnIndex(column);
                    Text("Row {} Column {}", row, column);
                }
            }
            EndTable();
        }

        // [Method 2] Using TableNextColumn() called multiple times, instead of using a for loop + TableSetColumnIndex().
        // This is generally more convenient when you have code manually submitting the contents of each columns.
        HelpMarker("Using TableNextRow() + calling TableNextColumn() _before_ each cell, manually.");
        if (BeginTable("table2", 3))
        {
            for (let row: c_int = 0; row < 4; row++)
            {
                TableNextRow();
                TableNextColumn();
                Text("Row {}", row);
                TableNextColumn();
                Text("Some contents");
                TableNextColumn();
                Text("123.456");
            }
            EndTable();
        }

        // [Method 3] We call TableNextColumn() _before_ each cell. We never call TableNextRow(),
        // as TableNextColumn() will automatically wrap around and create new rows as needed.
        // This is generally more convenient when your cells all contains the same type of data.
        HelpMarker(
            "Only using TableNextColumn(), which tends to be convenient for tables where every cells contains the same type of contents.\n"
            "This is also more similar to the old NextColumn() function of the Columns API, and provided to facilitate the Columns->Tables API transition.");
        if (BeginTable("table3", 3))
        {
            for (let item: c_int = 0; item < 14; item++)
            {
                TableNextColumn();
                Text("Item {}", item);
            }
            EndTable();
        }

        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Borders, background");
    if (TreeNode("Borders, background"))
    {
        // Expose a few Borders related flags interactively
        enum ContentsType { CT_Text, CT_FillButton };
        static flags: ImGuiTableFlags = ImGuiTableFlags_Borders | ImGuiTableFlags_RowBg;
        static let mut display_headers: bool =  false;
        static let contents_type: c_int = CT_Text;

        PushStyleCompact();
        CheckboxFlags("ImGuiTableFlags_RowBg", &flags, ImGuiTableFlags_RowBg);
        CheckboxFlags("ImGuiTableFlags_Borders", &flags, ImGuiTableFlags_Borders);
        SameLine(); HelpMarker("ImGuiTableFlags_Borders\n = ImGuiTableFlags_BordersInnerV\n | ImGuiTableFlags_BordersOuterV\n | ImGuiTableFlags_BordersInnerV\n | ImGuiTableFlags_BordersOuterH");
        Indent();

        CheckboxFlags("ImGuiTableFlags_BordersH", &flags, ImGuiTableFlags_BordersH);
        Indent();
        CheckboxFlags("ImGuiTableFlags_BordersOuterH", &flags, ImGuiTableFlags_BordersOuterH);
        CheckboxFlags("ImGuiTableFlags_BordersInnerH", &flags, ImGuiTableFlags_BordersInnerH);
        Unindent();

        CheckboxFlags("ImGuiTableFlags_BordersV", &flags, ImGuiTableFlags_BordersV);
        Indent();
        CheckboxFlags("ImGuiTableFlags_BordersOuterV", &flags, ImGuiTableFlags_BordersOuterV);
        CheckboxFlags("ImGuiTableFlags_BordersInnerV", &flags, ImGuiTableFlags_BordersInnerV);
        Unindent();

        CheckboxFlags("ImGuiTableFlags_BordersOuter", &flags, ImGuiTableFlags_BordersOuter);
        CheckboxFlags("ImGuiTableFlags_BordersInner", &flags, ImGuiTableFlags_BordersInner);
        Unindent();

        AlignTextToFramePadding(); Text("Cell contents:");
        SameLine(); RadioButton("Text", &contents_type, CT_Text);
        SameLine(); RadioButton("FillButton", &contents_type, CT_FillButton);
        Checkbox("Display headers", &display_headers);
        CheckboxFlags("ImGuiTableFlags_NoBordersInBody", &flags, ImGuiTableFlags_NoBordersInBody); SameLine(); HelpMarker("Disable vertical borders in columns Body (borders will always appears in Headers");
        PopStyleCompact();

        if (BeginTable("table1", 3, flags))
        {
            // Display headers so we can inspect their interaction with borders.
            // (Headers are not the main purpose of this section of the demo, so we are not elaborating on them too much. See other sections for details)
            if (display_headers)
            {
                TableSetupColumn("One");
                TableSetupColumn("Two");
                TableSetupColumn("Three");
                TableHeadersRow();
            }

            for (let row: c_int = 0; row < 5; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < 3; column++)
                {
                    TableSetColumnIndex(column);
                    buf: [c_char;32];
                    sprintf(buf, "Hello {},{}", column, row);
                    if contents_type == CT_Text {
                        TextUnformatted(buf)(); }
                    else if (contents_type == CT_FillButton)
                        Button(buf, ImVec2::new(-FLT_MIN, 0.0));
                }
            }
            EndTable();
        }
        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Resizable, stretch");
    if (TreeNode("Resizable, stretch"))
    {
        // By default, if we don't enable ScrollX the sizing policy for each columns is "Stretch"
        // Each columns maintain a sizing weight, and they will occupy all available width.
        static flags: ImGuiTableFlags = ImGuiTableFlags_SizingStretchSame | ImGuiTableFlags_Resizable | ImGuiTableFlags_BordersOuter | ImGuiTableFlags_BordersV | ImGuiTableFlags_ContextMenuInBody;
        PushStyleCompact();
        CheckboxFlags("ImGuiTableFlags_Resizable", &flags, ImGuiTableFlags_Resizable);
        CheckboxFlags("ImGuiTableFlags_BordersV", &flags, ImGuiTableFlags_BordersV);
        SameLine(); HelpMarker("Using the _Resizable flag automatically enables the _BordersInnerV flag as well, this is why the resize borders are still showing when unchecking this.");
        PopStyleCompact();

        if (BeginTable("table1", 3, flags))
        {
            for (let row: c_int = 0; row < 5; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < 3; column++)
                {
                    TableSetColumnIndex(column);
                    Text("Hello {},{}", column, row);
                }
            }
            EndTable();
        }
        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Resizable, fixed");
    if (TreeNode("Resizable, fixed"))
    {
        // Here we use ImGuiTableFlags_SizingFixedFit (even though _ScrollX is not set)
        // So columns will adopt the "Fixed" policy and will maintain a fixed width regardless of the whole available width (unless table is small)
        // If there is not enough available width to fit all columns, they will however be resized down.
        // FIXME-TABLE: Providing a stretch-on-init would make sense especially for tables which don't have saved settings
        HelpMarker(
            "Using _Resizable + _SizingFixedFit flags.\n"
            "Fixed-width columns generally makes more sense if you want to use horizontal scrolling.\n\n"
            "Double-click a column border to auto-fit the column to its contents.");
        PushStyleCompact();
        static flags: ImGuiTableFlags = ImGuiTableFlags_SizingFixedFit | ImGuiTableFlags_Resizable | ImGuiTableFlags_BordersOuter | ImGuiTableFlags_BordersV | ImGuiTableFlags_ContextMenuInBody;
        CheckboxFlags("ImGuiTableFlags_NoHostExtendX", &flags, ImGuiTableFlags_NoHostExtendX);
        PopStyleCompact();

        if (BeginTable("table1", 3, flags))
        {
            for (let row: c_int = 0; row < 5; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < 3; column++)
                {
                    TableSetColumnIndex(column);
                    Text("Hello {},{}", column, row);
                }
            }
            EndTable();
        }
        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Resizable, mixed");
    if (TreeNode("Resizable, mixed"))
    {
        HelpMarker(
            "Using TableSetupColumn() to alter resizing policy on a per-column basis.\n\n"
            "When combining Fixed and Stretch columns, generally you only want one, maybe two trailing columns to use _WidthStretch.");
        static flags: ImGuiTableFlags = ImGuiTableFlags_SizingFixedFit | ImGuiTableFlags_RowBg | ImGuiTableFlags_Borders | ImGuiTableFlags_Resizable | ImGuiTableFlags_Reorderable | ImGuiTableFlags_Hideable;

        if (BeginTable("table1", 3, flags))
        {
            TableSetupColumn("AAA", ImGuiTableColumnFlags_WidthFixed);
            TableSetupColumn("BBB", ImGuiTableColumnFlags_WidthFixed);
            TableSetupColumn("CCC", ImGuiTableColumnFlags_WidthStretch);
            TableHeadersRow();
            for (let row: c_int = 0; row < 5; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < 3; column++)
                {
                    TableSetColumnIndex(column);
                    Text("{} {},{}", (column == 2) ? "Stretch" : "Fixed", column, row);
                }
            }
            EndTable();
        }
        if (BeginTable("table2", 6, flags))
        {
            TableSetupColumn("AAA", ImGuiTableColumnFlags_WidthFixed);
            TableSetupColumn("BBB", ImGuiTableColumnFlags_WidthFixed);
            TableSetupColumn("CCC", ImGuiTableColumnFlags_WidthFixed | ImGuiTableColumnFlags_DefaultHide);
            TableSetupColumn("DDD", ImGuiTableColumnFlags_WidthStretch);
            TableSetupColumn("EEE", ImGuiTableColumnFlags_WidthStretch);
            TableSetupColumn("FFF", ImGuiTableColumnFlags_WidthStretch | ImGuiTableColumnFlags_DefaultHide);
            TableHeadersRow();
            for (let row: c_int = 0; row < 5; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < 6; column++)
                {
                    TableSetColumnIndex(column);
                    Text("{} {},{}", (column >= 3) ? "Stretch" : "Fixed", column, row);
                }
            }
            EndTable();
        }
        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Reorderable, hideable, with headers");
    if (TreeNode("Reorderable, hideable, with headers"))
    {
        HelpMarker(
            "Click and drag column headers to reorder columns.\n\n"
            "Right-click on a header to open a context menu.");
        static flags: ImGuiTableFlags = ImGuiTableFlags_Resizable | ImGuiTableFlags_Reorderable | ImGuiTableFlags_Hideable | ImGuiTableFlags_BordersOuter | ImGuiTableFlags_BordersV;
        PushStyleCompact();
        CheckboxFlags("ImGuiTableFlags_Resizable", &flags, ImGuiTableFlags_Resizable);
        CheckboxFlags("ImGuiTableFlags_Reorderable", &flags, ImGuiTableFlags_Reorderable);
        CheckboxFlags("ImGuiTableFlags_Hideable", &flags, ImGuiTableFlags_Hideable);
        CheckboxFlags("ImGuiTableFlags_NoBordersInBody", &flags, ImGuiTableFlags_NoBordersInBody);
        CheckboxFlags("ImGuiTableFlags_NoBordersInBodyUntilResize", &flags, ImGuiTableFlags_NoBordersInBodyUntilResize); SameLine(); HelpMarker("Disable vertical borders in columns Body until hovered for resize (borders will always appears in Headers)");
        PopStyleCompact();

        if (BeginTable("table1", 3, flags))
        {
            // Submit columns name with TableSetupColumn() and call TableHeadersRow() to create a row with a header in each column.
            // (Later we will show how TableSetupColumn() has other uses, optional flags, sizing weight etc.)
            TableSetupColumn("One");
            TableSetupColumn("Two");
            TableSetupColumn("Three");
            TableHeadersRow();
            for (let row: c_int = 0; row < 6; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < 3; column++)
                {
                    TableSetColumnIndex(column);
                    Text("Hello {},{}", column, row);
                }
            }
            EndTable();
        }

        // Use outer_size.x == 0.0 instead of default to make the table as tight as possible (only valid when no scrolling and no stretch column)
        if (BeginTable("table2", 3, flags | ImGuiTableFlags_SizingFixedFit, ImVec2::new(0.0, 0.0)))
        {
            TableSetupColumn("One");
            TableSetupColumn("Two");
            TableSetupColumn("Three");
            TableHeadersRow();
            for (let row: c_int = 0; row < 6; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < 3; column++)
                {
                    TableSetColumnIndex(column);
                    Text("Fixed {},{}", column, row);
                }
            }
            EndTable();
        }
        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Padding");
    if (TreeNode("Padding"))
    {
        // First example: showcase use of padding flags and effect of BorderOuterV/BorderInnerV on X padding.
        // We don't expose BorderOuterH/BorderInnerH here because they have no effect on X padding.
        HelpMarker(
            "We often want outer padding activated when any using features which makes the edges of a column visible:\n"
            "e.g.:\n"
            "- BorderOuterV\n"
            "- any form of row selection\n"
            "Because of this, activating BorderOuterV sets the default to PadOuterX. Using PadOuterX or NoPadOuterX you can override the default.\n\n"
            "Actual padding values are using style.CellPadding.\n\n"
            "In this demo we don't show horizontal borders to emphasis how they don't affect default horizontal padding.");

        static flags1: ImGuiTableFlags = ImGuiTableFlags_BordersV;
        PushStyleCompact();
        CheckboxFlags("ImGuiTableFlags_PadOuterX", &flags1, ImGuiTableFlags_PadOuterX);
        SameLine(); HelpMarker("Enable outer-most padding (default if ImGuiTableFlags_BordersOuterV is set)");
        CheckboxFlags("ImGuiTableFlags_NoPadOuterX", &flags1, ImGuiTableFlags_NoPadOuterX);
        SameLine(); HelpMarker("Disable outer-most padding (default if ImGuiTableFlags_BordersOuterV is not set)");
        CheckboxFlags("ImGuiTableFlags_NoPadInnerX", &flags1, ImGuiTableFlags_NoPadInnerX);
        SameLine(); HelpMarker("Disable inner padding between columns (double inner padding if BordersOuterV is on, single inner padding if BordersOuterV is off)");
        CheckboxFlags("ImGuiTableFlags_BordersOuterV", &flags1, ImGuiTableFlags_BordersOuterV);
        CheckboxFlags("ImGuiTableFlags_BordersInnerV", &flags1, ImGuiTableFlags_BordersInnerV);
        static let mut show_headers: bool =  false;
        Checkbox("show_headers", &show_headers);
        PopStyleCompact();

        if (BeginTable("table_padding", 3, flags1))
        {
            if (show_headers)
            {
                TableSetupColumn("One");
                TableSetupColumn("Two");
                TableSetupColumn("Three");
                TableHeadersRow();
            }

            for (let row: c_int = 0; row < 5; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < 3; column++)
                {
                    TableSetColumnIndex(column);
                    if (row == 0)
                    {
                        Text("Avail {}", GetContentRegionAvail().x);
                    }
                    else
                    {
                        buf: [c_char;32];
                        sprintf(buf, "Hello {},{}", column, row);
                        Button(buf, ImVec2::new(-FLT_MIN, 0.0));
                    }
                    //if (TableGetColumnFlags() & ImGuiTableColumnFlags_IsHovered)
                    //    TableSetBgColor(ImGuiTableBgTarget_CellBg, IM_COL32(0, 100, 0, 255));
                }
            }
            EndTable();
        }

        // Second example: set style.CellPadding to (0.0) or a custom value.
        // FIXME-TABLE: Vertical border effectively not displayed the same way as horizontal one...
        HelpMarker("Setting style.CellPadding to (0,0) or a custom value.");
        static flags2: ImGuiTableFlags = ImGuiTableFlags_Borders | ImGuiTableFlags_RowBg;
        static cell_padding: ImVec2::new(0.0, 0.0);
        static let mut show_widget_frame_bg: bool =  true;

        PushStyleCompact();
        CheckboxFlags("ImGuiTableFlags_Borders", &flags2, ImGuiTableFlags_Borders);
        CheckboxFlags("ImGuiTableFlags_BordersH", &flags2, ImGuiTableFlags_BordersH);
        CheckboxFlags("ImGuiTableFlags_BordersV", &flags2, ImGuiTableFlags_BordersV);
        CheckboxFlags("ImGuiTableFlags_BordersInner", &flags2, ImGuiTableFlags_BordersInner);
        CheckboxFlags("ImGuiTableFlags_BordersOuter", &flags2, ImGuiTableFlags_BordersOuter);
        CheckboxFlags("ImGuiTableFlags_RowBg", &flags2, ImGuiTableFlags_RowBg);
        CheckboxFlags("ImGuiTableFlags_Resizable", &flags2, ImGuiTableFlags_Resizable);
        Checkbox("show_widget_frame_bg", &show_widget_frame_bg);
        SliderFloat2("CellPadding", &cell_padding.x, 0.0, 10.0, "{}f");
        PopStyleCompact();

        PushStyleVar(ImGuiStyleVar_CellPadding, cell_padding);
        if (BeginTable("table_padding_2", 3, flags2))
        {
            static  text_bufs: c_char[3 * 5][16]; // Mini text storage for 3x5 cells
            static let mut init: bool =  true;
            if (!show_widget_frame_bg)
                PushStyleColor(ImGuiCol_FrameBg, 0);
            for (let cell: c_int = 0; cell < 3 * 5; cell++)
            {
                TableNextColumn();
                if (init)
                    strcpy(text_bufs[cell], "edit me");
                SetNextItemWidth(-FLT_MIN);
                PushID(cell);
                InputText("##cell", text_bufs[cell], IM_ARRAYSIZE(text_bufs[cell]));
                PopID();
            }
            if (!show_widget_frame_bg)
                PopStyleColor();
            init = false;
            EndTable();
        }
        PopStyleVar();

        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Explicit widths");
    if (TreeNode("Sizing policies"))
    {
        static flags1: ImGuiTableFlags = ImGuiTableFlags_BordersV | ImGuiTableFlags_BordersOuterH | ImGuiTableFlags_RowBg | ImGuiTableFlags_ContextMenuInBody;
        PushStyleCompact();
        CheckboxFlags("ImGuiTableFlags_Resizable", &flags1, ImGuiTableFlags_Resizable);
        CheckboxFlags("ImGuiTableFlags_NoHostExtendX", &flags1, ImGuiTableFlags_NoHostExtendX);
        PopStyleCompact();

        static sizing_policy_flags: ImGuiTableFlags[4] = { ImGuiTableFlags_SizingFixedFit, ImGuiTableFlags_SizingFixedSame, ImGuiTableFlags_SizingStretchProp, ImGuiTableFlags_SizingStretchSame };
        for (let table_n: c_int = 0; table_n < 4; table_n++)
        {
            PushID(table_n);
            SetNextItemWidth(TEXT_BASE_WIDTH * 30);
            EditTableSizingFlags(&sizing_policy_flags[table_n]);

            // To make it easier to understand the different sizing policy,
            // For each policy: we display one table where the columns have equal contents width, and one where the columns have different contents width.
            if (BeginTable("table1", 3, sizing_policy_flags[table_n] | flags1))
            {
                for (let row: c_int = 0; row < 3; row++)
                {
                    TableNextRow();
                    TableNextColumn(); Text("Oh dear");
                    TableNextColumn(); Text("Oh dear");
                    TableNextColumn(); Text("Oh dear");
                }
                EndTable();
            }
            if (BeginTable("table2", 3, sizing_policy_flags[table_n] | flags1))
            {
                for (let row: c_int = 0; row < 3; row++)
                {
                    TableNextRow();
                    TableNextColumn(); Text("AAAA");
                    TableNextColumn(); Text("BBBBBBBB");
                    TableNextColumn(); Text("CCCCCCCCCCCC");
                }
                EndTable();
            }
            PopID();
        }

        Spacing();
        TextUnformatted("Advanced");
        SameLine();
        HelpMarker("This section allows you to interact and see the effect of various sizing policies depending on whether Scroll is enabled and the contents of your columns.");

        enum ContentsType { CT_ShowWidth, CT_ShortText, CT_LongText, CT_Button, CT_FillButton, CT_InputText };
        static flags: ImGuiTableFlags = ImGuiTableFlags_ScrollY | ImGuiTableFlags_Borders | ImGuiTableFlags_RowBg | ImGuiTableFlags_Resizable;
        static let contents_type: c_int = CT_ShowWidth;
        static let column_count: c_int = 3;

        PushStyleCompact();
        PushID("Advanced");
        PushItemWidth(TEXT_BASE_WIDTH * 30);
        EditTableSizingFlags(&flags);
        Combo("Contents", &contents_type, "Show width\0Short Text\0Long Text\0Button\0Fill Button\0InputText\0");
        if (contents_type == CT_FillButton)
        {
            SameLine();
            HelpMarker("Be mindful that using right-alignment (e.g. size.x = -FLT_MIN) creates a feedback loop where contents width can feed into auto-column width can feed into contents width.");
        }
        DragInt("Columns", &column_count, 0.1f, 1, 64, "{}", ImGuiSliderFlags_AlwaysClamp);
        CheckboxFlags("ImGuiTableFlags_Resizable", &flags, ImGuiTableFlags_Resizable);
        CheckboxFlags("ImGuiTableFlags_PreciseWidths", &flags, ImGuiTableFlags_PreciseWidths);
        SameLine(); HelpMarker("Disable distributing remainder width to stretched columns (width allocation on a 100-wide table with 3 columns: Without this flag: 33,33,34. With this flag: 33,33,33). With larger number of columns, resizing will appear to be less smooth.");
        CheckboxFlags("ImGuiTableFlags_ScrollX", &flags, ImGuiTableFlags_ScrollX);
        CheckboxFlags("ImGuiTableFlags_ScrollY", &flags, ImGuiTableFlags_ScrollY);
        CheckboxFlags("ImGuiTableFlags_NoClip", &flags, ImGuiTableFlags_NoClip);
        PopItemWidth();
        PopID();
        PopStyleCompact();

        if (BeginTable("table2", column_count, flags, ImVec2::new(0.0, TEXT_BASE_HEIGHT * 7)))
        {
            for (let cell: c_int = 0; cell < 10 * column_count; cell++)
            {
                TableNextColumn();
                let column: c_int = TableGetColumnIndex();
                let row: c_int = TableGetRowIndex();

                PushID(cell);
                label: [c_char;32];
                static text_buf: [c_char;32] = "";
                sprintf(label, "Hello {},{}", column, row);
                switch (contents_type)
                {
                CT_ShortText =>  TextUnformatted(label); break;
                CT_LongText =>   Text("Some {} text {},{}\nOver two lines..", column == 0 ? "long" : "longeeer", column, row); break;
                CT_ShowWidth =>  Text("W: {}", GetContentRegionAvail().x); break;
                CT_Button =>     Button(label); break;
                CT_FillButton => Button(label, ImVec2::new(-FLT_MIN, 0.0)); break;
                CT_InputText =>  SetNextItemWidth(-FLT_MIN); InputText("##", text_buf, text_buf.len()); break;
                }
                PopID();
            }
            EndTable();
        }
        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Vertical scrolling, with clipping");
    if (TreeNode("Vertical scrolling, with clipping"))
    {
        HelpMarker("Here we activate ScrollY, which will create a child window container to allow hosting scrollable contents.\n\nWe also demonstrate using ImGuiListClipper to virtualize the submission of many items.");
        static flags: ImGuiTableFlags = ImGuiTableFlags_ScrollY | ImGuiTableFlags_RowBg | ImGuiTableFlags_BordersOuter | ImGuiTableFlags_BordersV | ImGuiTableFlags_Resizable | ImGuiTableFlags_Reorderable | ImGuiTableFlags_Hideable;

        PushStyleCompact();
        CheckboxFlags("ImGuiTableFlags_ScrollY", &flags, ImGuiTableFlags_ScrollY);
        PopStyleCompact();

        // When using ScrollX or ScrollY we need to specify a size for our table container!
        // Otherwise by default the table will fit all available space, like a BeginChild() call.
        let outer_size: ImVec2 = ImVec2::new(0.0, TEXT_BASE_HEIGHT * 8);
        if (BeginTable("table_scrolly", 3, flags, outer_size))
        {
            TableSetupScrollFreeze(0, 1); // Make top row always visible
            TableSetupColumn("One", ImGuiTableColumnFlags_None);
            TableSetupColumn("Two", ImGuiTableColumnFlags_None);
            TableSetupColumn("Three", ImGuiTableColumnFlags_None);
            TableHeadersRow();

            // Demonstrate using clipper for large vertical lists
            ImGuiListClipper clipper;
            clipper.Begin(1000);
            while (clipper.Step())
            {
                for (let row: c_int = clipper.DisplayStart; row < clipper.DisplayEnd; row++)
                {
                    TableNextRow();
                    for (let column: c_int = 0; column < 3; column++)
                    {
                        TableSetColumnIndex(column);
                        Text("Hello {},{}", column, row);
                    }
                }
            }
            EndTable();
        }
        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Horizontal scrolling");
    if (TreeNode("Horizontal scrolling"))
    {
        HelpMarker(
            "When ScrollX is enabled, the default sizing policy becomes ImGuiTableFlags_SizingFixedFit, "
            "as automatically stretching columns doesn't make much sense with horizontal scrolling.\n\n"
            "Also note that as of the current version, you will almost always want to enable ScrollY along with ScrollX,"
            "because the container window won't automatically extend vertically to fix contents (this may be improved in future versions).");
        static flags: ImGuiTableFlags = ImGuiTableFlags_ScrollX | ImGuiTableFlags_ScrollY | ImGuiTableFlags_RowBg | ImGuiTableFlags_BordersOuter | ImGuiTableFlags_BordersV | ImGuiTableFlags_Resizable | ImGuiTableFlags_Reorderable | ImGuiTableFlags_Hideable;
        static let freeze_cols: c_int = 1;
        static let freeze_rows: c_int = 1;

        PushStyleCompact();
        CheckboxFlags("ImGuiTableFlags_Resizable", &flags, ImGuiTableFlags_Resizable);
        CheckboxFlags("ImGuiTableFlags_ScrollX", &flags, ImGuiTableFlags_ScrollX);
        CheckboxFlags("ImGuiTableFlags_ScrollY", &flags, ImGuiTableFlags_ScrollY);
        SetNextItemWidth(GetFrameHeight());
        DragInt("freeze_cols", &freeze_cols, 0.2f, 0, 9, None, ImGuiSliderFlags_NoInput);
        SetNextItemWidth(GetFrameHeight());
        DragInt("freeze_rows", &freeze_rows, 0.2f, 0, 9, None, ImGuiSliderFlags_NoInput);
        PopStyleCompact();

        // When using ScrollX or ScrollY we need to specify a size for our table container!
        // Otherwise by default the table will fit all available space, like a BeginChild() call.
        let outer_size: ImVec2 = ImVec2::new(0.0, TEXT_BASE_HEIGHT * 8);
        if (BeginTable("table_scrollx", 7, flags, outer_size))
        {
            TableSetupScrollFreeze(freeze_cols, freeze_rows);
            TableSetupColumn("Line #", ImGuiTableColumnFlags_NoHide); // Make the first column not hideable to match our use of TableSetupScrollFreeze()
            TableSetupColumn("One");
            TableSetupColumn("Two");
            TableSetupColumn("Three");
            TableSetupColumn("Four");
            TableSetupColumn("Five");
            TableSetupColumn("Six");
            TableHeadersRow();
            for (let row: c_int = 0; row < 20; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < 7; column++)
                {
                    // Both TableNextColumn() and TableSetColumnIndex() return true when a column is visible or performing width measurement.
                    // Because here we know that:
                    // - A) all our columns are contributing the same to row height
                    // - B) column 0 is always visible,
                    // We only always submit this one column and can skip others.
                    // More advanced per-column clipping behaviors may benefit from polling the status flags via TableGetColumnFlags().
                    if (!TableSetColumnIndex(column) && column > 0)
                        continue;
                    if (column == 0)
                        Text("Line {}", row);
                    else
                        Text("Hello world {},{}", column, row);
                }
            }
            EndTable();
        }

        Spacing();
        TextUnformatted("Stretch + ScrollX");
        SameLine();
        HelpMarker(
            "Showcase using Stretch columns + ScrollX together: "
            "this is rather unusual and only makes sense when specifying an 'inner_width' for the table!\n"
            "Without an explicit value, inner_width is == outer_size.x and therefore using Stretch columns + ScrollX together doesn't make sense.");
        static flags2: ImGuiTableFlags = ImGuiTableFlags_SizingStretchSame | ImGuiTableFlags_ScrollX | ImGuiTableFlags_ScrollY | ImGuiTableFlags_BordersOuter | ImGuiTableFlags_RowBg | ImGuiTableFlags_ContextMenuInBody;
        static let inner_width: c_float =  1000;
        PushStyleCompact();
        PushID("flags3");
        PushItemWidth(TEXT_BASE_WIDTH * 30);
        CheckboxFlags("ImGuiTableFlags_ScrollX", &flags2, ImGuiTableFlags_ScrollX);
        DragFloat("inner_width", &inner_width, 1.0, 0.0, f32::MAX, "{}");
        PopItemWidth();
        PopID();
        PopStyleCompact();
        if (BeginTable("table2", 7, flags2, outer_size, inner_width))
        {
            for (let cell: c_int = 0; cell < 20 * 7; cell++)
            {
                TableNextColumn();
                Text("Hello world {},{}", TableGetColumnIndex(), TableGetRowIndex());
            }
            EndTable();
        }
        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Columns flags");
    if (TreeNode("Columns flags"))
    {
        // Create a first table just to show all the options/flags we want to make visible in our example!
        let column_count: c_int = 3;
        column_names: *const c_char[column_count] = { "One", "Two", "Three" };
        static column_flags: ImGuiTableColumnFlags[column_count] = { ImGuiTableColumnFlags_DefaultSort, ImGuiTableColumnFlags_None, ImGuiTableColumnFlags_DefaultHide };
        static column_flags_out: ImGuiTableColumnFlags[column_count] = { 0, 0, 0 }; // Output from TableGetColumnFlags()

        if (BeginTable("table_columns_flags_checkboxes", column_count, ImGuiTableFlags_None))
        {
            PushStyleCompact();
            for (let column: c_int = 0; column < column_count; column++)
            {
                TableNextColumn();
                PushID(column);
                AlignTextToFramePadding(); // FIXME-TABLE: Workaround for wrong text baseline propagation across columns
                Text("'{}'", column_names[column]);
                Spacing();
                Text("Input flags:");
                EditTableColumnsFlags(&column_flags[column]);
                Spacing();
                Text("Output flags:");
                BeginDisabled();
                ShowTableColumnsStatusFlags(column_flags_out[column]);
                EndDisabled();
                PopID();
            }
            PopStyleCompact();
            EndTable();
        }

        // Create the real table we care about for the example!
        // We use a scrolling table to be able to showcase the difference between the _IsEnabled and _IsVisible flags above, otherwise in
        // a non-scrolling table columns are always visible (unless using ImGuiTableFlags_NoKeepColumnsVisible + resizing the parent window down)
        const flags: ImGuiTableFlags
            = ImGuiTableFlags_SizingFixedFit | ImGuiTableFlags_ScrollX | ImGuiTableFlags_ScrollY
            | ImGuiTableFlags_RowBg | ImGuiTableFlags_BordersOuter | ImGuiTableFlags_BordersV
            | ImGuiTableFlags_Resizable | ImGuiTableFlags_Reorderable | ImGuiTableFlags_Hideable | ImGuiTableFlags_Sortable;
        let outer_size: ImVec2 = ImVec2::new(0.0, TEXT_BASE_HEIGHT * 9);
        if (BeginTable("table_columns_flags", column_count, flags, outer_size))
        {
            for (let column: c_int = 0; column < column_count; column++)
                TableSetupColumn(column_names[column], column_flags[column]);
            TableHeadersRow();
            for (let column: c_int = 0; column < column_count; column++)
                column_flags_out[column] = TableGetColumnFlags(column);
            let indent_step: c_float =  (TEXT_BASE_WIDTH / 2);
            for (let row: c_int = 0; row < 8; row++)
            {
                Indent(indent_step); // Add some indentation to demonstrate usage of per-column IndentEnable/IndentDisable flags.
                TableNextRow();
                for (let column: c_int = 0; column < column_count; column++)
                {
                    TableSetColumnIndex(column);
                    Text("{} {}", (column == 0) ? "Indented" : "Hello", TableGetColumnName(column));
                }
            }
            Unindent(indent_step * 8.0);

            EndTable();
        }
        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Columns widths");
    if (TreeNode("Columns widths"))
    {
        HelpMarker("Using TableSetupColumn() to setup default width.");

        static flags1: ImGuiTableFlags = ImGuiTableFlags_Borders | ImGuiTableFlags_NoBordersInBodyUntilResize;
        PushStyleCompact();
        CheckboxFlags("ImGuiTableFlags_Resizable", &flags1, ImGuiTableFlags_Resizable);
        CheckboxFlags("ImGuiTableFlags_NoBordersInBodyUntilResize", &flags1, ImGuiTableFlags_NoBordersInBodyUntilResize);
        PopStyleCompact();
        if (BeginTable("table1", 3, flags1))
        {
            // We could also set ImGuiTableFlags_SizingFixedFit on the table and all columns will default to ImGuiTableColumnFlags_WidthFixed.
            TableSetupColumn("one", ImGuiTableColumnFlags_WidthFixed, 100); // Default to 100
            TableSetupColumn("two", ImGuiTableColumnFlags_WidthFixed, 200); // Default to 200
            TableSetupColumn("three", ImGuiTableColumnFlags_WidthFixed);       // Default to auto
            TableHeadersRow();
            for (let row: c_int = 0; row < 4; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < 3; column++)
                {
                    TableSetColumnIndex(column);
                    if (row == 0)
                        Text("(w: %5.10.0)", GetContentRegionAvail().x);
                    else
                        Text("Hello {},{}", column, row);
                }
            }
            EndTable();
        }

        HelpMarker("Using TableSetupColumn() to setup explicit width.\n\nUnless _NoKeepColumnsVisible is set, fixed columns with set width may still be shrunk down if there's not enough space in the host.");

        static flags2: ImGuiTableFlags = ImGuiTableFlags_None;
        PushStyleCompact();
        CheckboxFlags("ImGuiTableFlags_NoKeepColumnsVisible", &flags2, ImGuiTableFlags_NoKeepColumnsVisible);
        CheckboxFlags("ImGuiTableFlags_BordersInnerV", &flags2, ImGuiTableFlags_BordersInnerV);
        CheckboxFlags("ImGuiTableFlags_BordersOuterV", &flags2, ImGuiTableFlags_BordersOuterV);
        PopStyleCompact();
        if (BeginTable("table2", 4, flags2))
        {
            // We could also set ImGuiTableFlags_SizingFixedFit on the table and all columns will default to ImGuiTableColumnFlags_WidthFixed.
            TableSetupColumn("", ImGuiTableColumnFlags_WidthFixed, 100);
            TableSetupColumn("", ImGuiTableColumnFlags_WidthFixed, TEXT_BASE_WIDTH * 15.0);
            TableSetupColumn("", ImGuiTableColumnFlags_WidthFixed, TEXT_BASE_WIDTH * 30f32);
            TableSetupColumn("", ImGuiTableColumnFlags_WidthFixed, TEXT_BASE_WIDTH * 15.0);
            for (let row: c_int = 0; row < 5; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < 4; column++)
                {
                    TableSetColumnIndex(column);
                    if (row == 0)
                        Text("(w: %5.10.0)", GetContentRegionAvail().x);
                    else
                        Text("Hello {},{}", column, row);
                }
            }
            EndTable();
        }
        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Nested tables");
    if (TreeNode("Nested tables"))
    {
        HelpMarker("This demonstrate embedding a table into another table cell.");

        if (BeginTable("table_nested1", 2, ImGuiTableFlags_Borders | ImGuiTableFlags_Resizable | ImGuiTableFlags_Reorderable | ImGuiTableFlags_Hideable))
        {
            TableSetupColumn("A0");
            TableSetupColumn("A1");
            TableHeadersRow();

            TableNextColumn();
            Text("A0 Row 0");
            {
                let rows_height: c_float =  TEXT_BASE_HEIGHT * 2;
                if (BeginTable("table_nested2", 2, ImGuiTableFlags_Borders | ImGuiTableFlags_Resizable | ImGuiTableFlags_Reorderable | ImGuiTableFlags_Hideable))
                {
                    TableSetupColumn("B0");
                    TableSetupColumn("B1");
                    TableHeadersRow();

                    TableNextRow(ImGuiTableRowFlags_None, rows_height);
                    TableNextColumn();
                    Text("B0 Row 0");
                    TableNextColumn();
                    Text("B1 Row 0");
                    TableNextRow(ImGuiTableRowFlags_None, rows_height);
                    TableNextColumn();
                    Text("B0 Row 1");
                    TableNextColumn();
                    Text("B1 Row 1");

                    EndTable();
                }
            }
            TableNextColumn(); Text("A1 Row 0");
            TableNextColumn(); Text("A0 Row 1");
            TableNextColumn(); Text("A1 Row 1");
            EndTable();
        }
        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Row height");
    if (TreeNode("Row height"))
    {
        HelpMarker("You can pass a 'min_row_height' to TableNextRow().\n\nRows are padded with 'style.CellPadding.y' on top and bottom, so effectively the minimum row height will always be >= 'style.CellPadding.y * 2.0f'.\n\nWe cannot honor a _maximum_ row height as that would requires a unique clipping rectangle per row.");
        if (BeginTable("table_row_height", 1, ImGuiTableFlags_BordersOuter | ImGuiTableFlags_BordersInnerV))
        {
            for (let row: c_int = 0; row < 10; row++)
            {
                let min_row_height: c_float =  (TEXT_BASE_HEIGHT * 0.3f32 * row);
                TableNextRow(ImGuiTableRowFlags_None, min_row_height);
                TableNextColumn();
                Text("min_row_height = {}", min_row_height);
            }
            EndTable();
        }
        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Outer size");
    if (TreeNode("Outer size"))
    {
        // Showcasing use of ImGuiTableFlags_NoHostExtendX and ImGuiTableFlags_NoHostExtendY
        // Important to that note how the two flags have slightly different behaviors!
        Text("Using NoHostExtendX and NoHostExtendY:");
        PushStyleCompact();
        static flags: ImGuiTableFlags = ImGuiTableFlags_Borders | ImGuiTableFlags_Resizable | ImGuiTableFlags_ContextMenuInBody | ImGuiTableFlags_RowBg | ImGuiTableFlags_SizingFixedFit | ImGuiTableFlags_NoHostExtendX;
        CheckboxFlags("ImGuiTableFlags_NoHostExtendX", &flags, ImGuiTableFlags_NoHostExtendX);
        SameLine(); HelpMarker("Make outer width auto-fit to columns, overriding outer_size.x value.\n\nOnly available when ScrollX/ScrollY are disabled and Stretch columns are not used.");
        CheckboxFlags("ImGuiTableFlags_NoHostExtendY", &flags, ImGuiTableFlags_NoHostExtendY);
        SameLine(); HelpMarker("Make outer height stop exactly at outer_size.y (prevent auto-extending table past the limit).\n\nOnly available when ScrollX/ScrollY are disabled. Data below the limit will be clipped and not visible.");
        PopStyleCompact();

        let outer_size: ImVec2 = ImVec2::new(0.0, TEXT_BASE_HEIGHT * 5.5);
        if (BeginTable("table1", 3, flags, outer_size))
        {
            for (let row: c_int = 0; row < 10; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < 3; column++)
                {
                    TableNextColumn();
                    Text("Cell {},{}", column, row);
                }
            }
            EndTable();
        }
        SameLine();
        Text("Hello!");

        Spacing();

        Text("Using explicit size:");
        if (BeginTable("table2", 3, ImGuiTableFlags_Borders | ImGuiTableFlags_RowBg, ImVec2::new(TEXT_BASE_WIDTH * 30, 0.0)))
        {
            for (let row: c_int = 0; row < 5; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < 3; column++)
                {
                    TableNextColumn();
                    Text("Cell {},{}", column, row);
                }
            }
            EndTable();
        }
        SameLine();
        if (BeginTable("table3", 3, ImGuiTableFlags_Borders | ImGuiTableFlags_RowBg, ImVec2::new(TEXT_BASE_WIDTH * 30, 0.0)))
        {
            for (let row: c_int = 0; row < 3; row++)
            {
                TableNextRow(0, TEXT_BASE_HEIGHT * 1.5);
                for (let column: c_int = 0; column < 3; column++)
                {
                    TableNextColumn();
                    Text("Cell {},{}", column, row);
                }
            }
            EndTable();
        }

        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Background color");
    if (TreeNode("Background color"))
    {
        static flags: ImGuiTableFlags = ImGuiTableFlags_RowBg;
        static let row_bg_type: c_int = 1;
        static let row_bg_target: c_int = 1;
        static let cell_bg_type: c_int = 1;

        PushStyleCompact();
        CheckboxFlags("ImGuiTableFlags_Borders", &flags, ImGuiTableFlags_Borders);
        CheckboxFlags("ImGuiTableFlags_RowBg", &flags, ImGuiTableFlags_RowBg);
        SameLine(); HelpMarker("ImGuiTableFlags_RowBg automatically sets RowBg0 to alternative colors pulled from the Style.");
        Combo("row bg type", (*mut c_int)&row_bg_type, "None\0Red\0Gradient\0");
        Combo("row bg target", (*mut c_int)&row_bg_target, "RowBg0\0RowBg1\0"); SameLine(); HelpMarker("Target RowBg0 to override the alternating odd/even colors,\nTarget RowBg1 to blend with them.");
        Combo("cell bg type", (*mut c_int)&cell_bg_type, "None\0Blue\0"); SameLine(); HelpMarker("We are colorizing cells to B1->C2 here.");
        // IM_ASSERT(row_bg_type >= 0 && row_bg_type <= 2);
        // IM_ASSERT(row_bg_target >= 0 && row_bg_target <= 1);
        // IM_ASSERT(cell_bg_type >= 0 && cell_bg_type <= 1);
        PopStyleCompact();

        if (BeginTable("table1", 5, flags))
        {
            for (let row: c_int = 0; row < 6; row++)
            {
                TableNextRow();

                // Demonstrate setting a row background color with 'TableSetBgColor(ImGuiTableBgTarget_RowBgX, ...)'
                // We use a transparent color so we can see the one behind in case our target is RowBg1 and RowBg0 was already targeted by the ImGuiTableFlags_RowBg flag.
                if (row_bg_type != 0)
                {
                    row_bg_color: u32 = GetColorU32(if row_bg_type == 1 { ImVec4(0.7f, 0.3f, 0.3f, 0.650f32) } else { ImVec4(0.2f + row * 0.1f, 0.2f, 0.2f, 0.650f32) }); // Flat or Gradient?
                    TableSetBgColor(ImGuiTableBgTarget_RowBg0 + row_bg_target, row_bg_color);
                }

                // Fill cells
                for (let column: c_int = 0; column < 5; column++)
                {
                    TableSetColumnIndex(column);
                    Text("{}{}", 'A' + row, '0' + column);

                    // Change background of Cells B1->C2
                    // Demonstrate setting a cell background color with 'TableSetBgColor(ImGuiTableBgTarget_CellBg, ...)'
                    // (the CellBg color will be blended over the RowBg and ColumnBg colors)
                    // We can also pass a column number as a third parameter to TableSetBgColor() and do this outside the column loop.
                    if (row >= 1 && row <= 2 && column >= 1 && column <= 2 && cell_bg_type == 1)
                    {
                        cell_bg_color: u32 = GetColorU32(ImVec4(0.3f, 0.3f, 0.7f, 0.650f32));
                        TableSetBgColor(ImGuiTableBgTarget_CellBg, cell_bg_color);
                    }
                }
            }
            EndTable();
        }
        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Tree view");
    if (TreeNode("Tree view"))
    {
        static flags: ImGuiTableFlags = ImGuiTableFlags_BordersV | ImGuiTableFlags_BordersOuterH | ImGuiTableFlags_Resizable | ImGuiTableFlags_RowBg | ImGuiTableFlags_NoBordersInBody;

        if (BeginTable("3ways", 3, flags))
        {
            // The first column will use the default _WidthStretch when ScrollX is Off and _WidthFixed when ScrollX is On
            TableSetupColumn("Name", ImGuiTableColumnFlags_NoHide);
            TableSetupColumn("Size", ImGuiTableColumnFlags_WidthFixed, TEXT_BASE_WIDTH * 12.0);
            TableSetupColumn("Type", ImGuiTableColumnFlags_WidthFixed, TEXT_BASE_WIDTH * 18.0);
            TableHeadersRow();

            // Simple storage to output a dummy file-system.
            struct MyTreeNode
            {
                *const char     Name;
                *const char     Type;
                c_int             Size;
                c_int             ChildIdx;
                c_int             ChildCount;
                static c_void DisplayNode(*const MyTreeNode node, *const MyTreeNode all_nodes)
                {
                    TableNextRow();
                    TableNextColumn();
                    let is_folder: bool = (node.ChildCount > 0);
                    if (is_folder)
                    {
                        let mut open: bool =  TreeNodeEx(node.Name, ImGuiTreeNodeFlags_SpanFullWidth);
                        TableNextColumn();
                        TextDisabled("--");
                        TableNextColumn();
                        TextUnformatted(node.Type);
                        if (open)
                        {
                            for (let child_n: c_int = 0; child_n < node.ChildCount; child_n++)
                                DisplayNode(&all_nodes[node.ChildIdx + child_n], all_nodes);
                            TreePop();
                        }
                    }
                    else
                    {
                        TreeNodeEx(node.Name, ImGuiTreeNodeFlags_Leaf | ImGuiTreeNodeFlags_Bullet | ImGuiTreeNodeFlags_NoTreePushOnOpen | ImGuiTreeNodeFlags_SpanFullWidth);
                        TableNextColumn();
                        Text("{}", node.Size);
                        TableNextColumn();
                        TextUnformatted(node.Type);
                    }
                }
            };
            static const MyTreeNode nodes[] =
            {
                { "Root",                         "Folder",       -1,       1, 3    }, // 0
                { "Music",                        "Folder",       -1,       4, 2    }, // 1
                { "Textures",                     "Folder",       -1,       6, 3    }, // 2
                { "desktop.ini",                  "System file",  1024,    -1,-1    }, // 3
                { "File1_a.wav",                  "Audio file",   123000,  -1,-1    }, // 4
                { "File1_b.wav",                  "Audio file",   456000,  -1,-1    }, // 5
                { "Image001.png",                 "Image file",   203128,  -1,-1    }, // 6
                { "Copy of Image001.png",         "Image file",   203256,  -1,-1    }, // 7
                { "Copy of Image001 (Final2).png","Image file",   203512,  -1,-1    }, // 8
            };

            MyTreeNode::DisplayNode(&nodes[0], nodes);

            EndTable();
        }
        TreePop();
    }

    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Item width");
    if (TreeNode("Item width"))
    {
        HelpMarker(
            "Showcase using PushItemWidth() and how it is preserved on a per-column basis.\n\n"
            "Note that on auto-resizing non-resizable fixed columns, querying the content width for e.g. right-alignment doesn't make sense.");
        if (BeginTable("table_item_width", 3, ImGuiTableFlags_Borders))
        {
            TableSetupColumn("small");
            TableSetupColumn("half");
            TableSetupColumn("right-align");
            TableHeadersRow();

            for (let row: c_int = 0; row < 3; row++)
            {
                TableNextRow();
                if (row == 0)
                {
                    // Setup ItemWidth once (instead of setting up every time, which is also possible but less efficient)
                    TableSetColumnIndex(0);
                    PushItemWidth(TEXT_BASE_WIDTH * 3.0); // Small
                    TableSetColumnIndex(1);
                    PushItemWidth(-GetContentRegionAvail().x * 0.5);
                    TableSetColumnIndex(2);
                    PushItemWidth(-FLT_MIN); // Right-aligned
                }

                // Draw our contents
                static let dummy_f: c_float =  0.0;
                PushID(row);
                TableSetColumnIndex(0);
                SliderFloat("float0", &dummy_f, 0.0, 1.0);
                TableSetColumnIndex(1);
                SliderFloat("float1", &dummy_f, 0.0, 1.0);
                TableSetColumnIndex(2);
                SliderFloat("##float2", &dummy_f, 0.0, 1.0); // No visible label since right-aligned
                PopID();
            }
            EndTable();
        }
        TreePop();
    }

    // Demonstrate using TableHeader() calls instead of TableHeadersRow()
    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Custom headers");
    if (TreeNode("Custom headers"))
    {
        let COLUMNS_COUNT: c_int = 3;
        if (BeginTable("table_custom_headers", COLUMNS_COUNT, ImGuiTableFlags_Borders | ImGuiTableFlags_Reorderable | ImGuiTableFlags_Hideable))
        {
            TableSetupColumn("Apricot");
            TableSetupColumn("Banana");
            TableSetupColumn("Cherry");

            // Dummy entire-column selection storage
            // FIXME: It would be nice to actually demonstrate full-featured selection using those checkbox.
            static column_selected: bool[3] = {};

            // Instead of calling TableHeadersRow() we'll submit custom headers ourselves
            TableNextRow(ImGuiTableRowFlags_Headers);
            for (let column: c_int = 0; column < COLUMNS_COUNT; column++)
            {
                TableSetColumnIndex(column);
                let mut  column_name: *const c_char = TableGetColumnName(column); // Retrieve name passed to TableSetupColumn()
                PushID(column);
                PushStyleVar(ImGuiStyleVar_FramePadding, ImVec2::new(0, 0));
                Checkbox("##checkall", &column_selected[column]);
                PopStyleVar();
                SameLine(0.0, GetStyle().ItemInnerSpacing.x);
                TableHeader(column_name);
                PopID();
            }

            for (let row: c_int = 0; row < 5; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < 3; column++)
                {
                    buf: [c_char;32];
                    sprintf(buf, "Cell {},{}", column, row);
                    TableSetColumnIndex(column);
                    Selectable(buf, column_selected[column]);
                }
            }
            EndTable();
        }
        TreePop();
    }

    // Demonstrate creating custom context menus inside columns, while playing it nice with context menus provided by TableHeadersRow()/TableHeader()
    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Context menus");
    if (TreeNode("Context menus"))
    {
        HelpMarker("By default, right-clicking over a TableHeadersRow()/TableHeader() line will open the default context-menu.\nUsing ImGuiTableFlags_ContextMenuInBody we also allow right-clicking over columns body.");
        static flags1: ImGuiTableFlags = ImGuiTableFlags_Resizable | ImGuiTableFlags_Reorderable | ImGuiTableFlags_Hideable | ImGuiTableFlags_Borders | ImGuiTableFlags_ContextMenuInBody;

        PushStyleCompact();
        CheckboxFlags("ImGuiTableFlags_ContextMenuInBody", &flags1, ImGuiTableFlags_ContextMenuInBody);
        PopStyleCompact();

        // Context Menus: first example
        // [1.1] Right-click on the TableHeadersRow() line to open the default table context menu.
        // [1.2] Right-click in columns also open the default table context menu (if ImGuiTableFlags_ContextMenuInBody is set)
        let COLUMNS_COUNT: c_int = 3;
        if (BeginTable("table_context_menu", COLUMNS_COUNT, flags1))
        {
            TableSetupColumn("One");
            TableSetupColumn("Two");
            TableSetupColumn("Three");

            // [1.1]] Right-click on the TableHeadersRow() line to open the default table context menu.
            TableHeadersRow();

            // Submit dummy contents
            for (let row: c_int = 0; row < 4; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < COLUMNS_COUNT; column++)
                {
                    TableSetColumnIndex(column);
                    Text("Cell {},{}", column, row);
                }
            }
            EndTable();
        }

        // Context Menus: second example
        // [2.1] Right-click on the TableHeadersRow() line to open the default table context menu.
        // [2.2] Right-click on the ".." to open a custom popup
        // [2.3] Right-click in columns to open another custom popup
        HelpMarker("Demonstrate mixing table context menu (over header), item context button (over button) and custom per-colum context menu (over column body).");
        flags2: ImGuiTableFlags = ImGuiTableFlags_Resizable | ImGuiTableFlags_SizingFixedFit | ImGuiTableFlags_Reorderable | ImGuiTableFlags_Hideable | ImGuiTableFlags_Borders;
        if (BeginTable("table_context_menu_2", COLUMNS_COUNT, flags2))
        {
            TableSetupColumn("One");
            TableSetupColumn("Two");
            TableSetupColumn("Three");

            // [2.1] Right-click on the TableHeadersRow() line to open the default table context menu.
            TableHeadersRow();
            for (let row: c_int = 0; row < 4; row++)
            {
                TableNextRow();
                for (let column: c_int = 0; column < COLUMNS_COUNT; column++)
                {
                    // Submit dummy contents
                    TableSetColumnIndex(column);
                    Text("Cell {},{}", column, row);
                    SameLine();

                    // [2.2] Right-click on the ".." to open a custom popup
                    PushID(row * COLUMNS_COUNT + column);
                    SmallButton("..");
                    if (BeginPopupContextItem())
                    {
                        Text("This is the popup for Button(\"..\") in Cell {},{}", column, row);
                        if (Button("Close"))
                            CloseCurrentPopup();
                        EndPopup();
                    }
                    PopID();
                }
            }

            // [2.3] Right-click anywhere in columns to open another custom popup
            // (instead of testing for !IsAnyItemHovered() we could also call OpenPopup() with ImGuiPopupFlags_NoOpenOverExistingPopup
            // to manage popup priority as the popups triggers, here "are we hovering a column" are overlapping)
            let hovered_column: c_int = -1;
            for (let column: c_int = 0; column < COLUMNS_COUNT + 1; column++)
            {
                PushID(column);
                if TableGetColumnFlags(column) & ImGuiTableColumnFlags_IsHovered {
                    hovered_column = column;}
                if (hovered_column == column && !IsAnyItemHovered() && IsMouseReleased(1))
                    OpenPopup("MyPopup");
                if (BeginPopup("MyPopup"))
                {
                    if column == COLUMNS_COUNT{
                        Text("This is a custom popup for unused space after the last column.");}
                    else
                        Text("This is a custom popup for Column {}", column);
                    if (Button("Close"))
                        CloseCurrentPopup();
                    EndPopup();
                }
                PopID();
            }

            EndTable();
            Text("Hovered column: {}", hovered_column);
        }
        TreePop();
    }

    // Demonstrate creating multiple tables with the same ID
    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Synced instances");
    if (TreeNode("Synced instances"))
    {
        HelpMarker("Multiple tables with the same identifier will share their settings, width, visibility, order etc.");
        for (let n: c_int = 0; n < 3; n++)
        {
            buf: [c_char;32];
            sprintf(buf, "Synced Table {}", n);
            let mut open: bool =  CollapsingHeader(buf, ImGuiTreeNodeFlags_DefaultOpen);
            if (open && BeginTable("Table", 3, ImGuiTableFlags_Resizable | ImGuiTableFlags_Reorderable | ImGuiTableFlags_Hideable | ImGuiTableFlags_Borders | ImGuiTableFlags_SizingFixedFit | ImGuiTableFlags_NoSavedSettings))
            {
                TableSetupColumn("One");
                TableSetupColumn("Two");
                TableSetupColumn("Three");
                TableHeadersRow();
                for (let cell: c_int = 0; cell < 9; cell++)
                {
                    TableNextColumn();
                    Text("this cell {}", cell);
                }
                EndTable();
            }
        }
        TreePop();
    }

    // Demonstrate using Sorting facilities
    // This is a simplified version of the "Advanced" example, where we mostly focus on the code necessary to handle sorting.
    // Note that the "Advanced" example also showcase manually triggering a sort (e.g. if item quantities have been modified)
    static template_items_names: *const c_char[] =
    {
        "Banana", "Apple", "Cherry", "Watermelon", "Grapefruit", "Strawberry", "Mango",
        "Kiwi", "Orange", "Pineapple", "Blueberry", "Plum", "Coconut", "Pear", "Apricot"
    };
    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Sorting");
    if (TreeNode("Sorting"))
    {
        // Create item list
        static Vec<MyItem> items;
        if (items.Size == 0)
        {
            items.resize(50, MyItem());
            for (let n: c_int = 0; n < items.Size; n++)
            {
                let template_n: c_int = n % template_items_names.len();
                MyItem& item = items[n];
                item.ID = n;
                item.Name = template_items_names[template_n];
                item.Quantity = (n * n - n) % 20; // Assign default quantities
            }
        }

        // Options
        static flags: ImGuiTableFlags =
            ImGuiTableFlags_Resizable | ImGuiTableFlags_Reorderable | ImGuiTableFlags_Hideable | ImGuiTableFlags_Sortable | ImGuiTableFlags_SortMulti
            | ImGuiTableFlags_RowBg | ImGuiTableFlags_BordersOuter | ImGuiTableFlags_BordersV | ImGuiTableFlags_NoBordersInBody
            | ImGuiTableFlags_ScrollY;
        PushStyleCompact();
        CheckboxFlags("ImGuiTableFlags_SortMulti", &flags, ImGuiTableFlags_SortMulti);
        SameLine(); HelpMarker("When sorting is enabled: hold shift when clicking headers to sort on multiple column. TableGetSortSpecs() may return specs where (SpecsCount > 1).");
        CheckboxFlags("ImGuiTableFlags_SortTristate", &flags, ImGuiTableFlags_SortTristate);
        SameLine(); HelpMarker("When sorting is enabled: allow no sorting, disable default sorting. TableGetSortSpecs() may return specs where (SpecsCount == 0).");
        PopStyleCompact();

        if (BeginTable("table_sorting", 4, flags, ImVec2::new(0.0, TEXT_BASE_HEIGHT * 15), 0.0))
        {
            // Declare columns
            // We use the "user_id" parameter of TableSetupColumn() to specify a user id that will be stored in the sort specifications.
            // This is so our sort function can identify a column given our own identifier. We could also identify them based on their index!
            // Demonstrate using a mixture of flags among available sort-related flags:
            // - ImGuiTableColumnFlags_DefaultSort
            // - ImGuiTableColumnFlags_NoSort / ImGuiTableColumnFlags_NoSortAscending / ImGuiTableColumnFlags_NoSortDescending
            // - ImGuiTableColumnFlags_PreferSortAscending / ImGuiTableColumnFlags_PreferSortDescending
            TableSetupColumn("ID",       ImGuiTableColumnFlags_DefaultSort          | ImGuiTableColumnFlags_WidthFixed,   0.0, MyItemColumnID_ID);
            TableSetupColumn("Name",                                                  ImGuiTableColumnFlags_WidthFixed,   0.0, MyItemColumnID_Name);
            TableSetupColumn("Action",   ImGuiTableColumnFlags_NoSort               | ImGuiTableColumnFlags_WidthFixed,   0.0, MyItemColumnID_Action);
            TableSetupColumn("Quantity", ImGuiTableColumnFlags_PreferSortDescending | ImGuiTableColumnFlags_WidthStretch, 0.0, MyItemColumnID_Quantity);
            TableSetupScrollFreeze(0, 1); // Make row always visible
            TableHeadersRow();

            // Sort our data if sort specs have been changed!
            if (*mut ImGuiTableSortSpecs sorts_specs = TableGetSortSpecs())
                if (sorts_specs->SpecsDirty)
                {
                    MyItem::s_current_sort_specs = sorts_specs; // Store in variable accessible by the sort function.
                    if (items.Size > 1)
                        qsort(&items[0], items.Size, sizeof(items[0]), MyItem::CompareWithSortSpecs);
                    MyItem::s_current_sort_specs= None;
                    sorts_specs->SpecsDirty = false;
                }

            // Demonstrate using clipper for large vertical lists
            ImGuiListClipper clipper;
            clipper.Begin(items.Size);
            while (clipper.Step())
                for (let row_n: c_int = clipper.DisplayStart; row_n < clipper.DisplayEnd; row_n++)
                {
                    // Display a data item
                    *mut MyItem item = &items[row_n];
                    PushID(item.ID);
                    TableNextRow();
                    TableNextColumn();
                    Text("%04d", item.ID);
                    TableNextColumn();
                    TextUnformatted(item.Name);
                    TableNextColumn();
                    SmallButton("None");
                    TableNextColumn();
                    Text("{}", item->Quantity);
                    PopID();
                }
            EndTable();
        }
        TreePop();
    }

    // In this example we'll expose most table flags and settings.
    // For specific flags and settings refer to the corresponding section for more detailed explanation.
    // This section is mostly useful to experiment with combining certain flags or settings with each others.
    //SetNextItemOpen(true, ImGuiCond_Once); // [DEBUG]
    if (open_action != -1)
        SetNextItemOpen(open_action != 0);
    IMGUI_DEMO_MARKER("Tables/Advanced");
    if (TreeNode("Advanced"))
    {
        static flags: ImGuiTableFlags =
            ImGuiTableFlags_Resizable | ImGuiTableFlags_Reorderable | ImGuiTableFlags_Hideable
            | ImGuiTableFlags_Sortable | ImGuiTableFlags_SortMulti
            | ImGuiTableFlags_RowBg | ImGuiTableFlags_Borders | ImGuiTableFlags_NoBordersInBody
            | ImGuiTableFlags_ScrollX | ImGuiTableFlags_ScrollY
            | ImGuiTableFlags_SizingFixedFit;

        enum ContentsType { CT_Text, CT_Button, CT_SmallButton, CT_FillButton, CT_Selectable, CT_SelectableSpanRow };
        static let contents_type: c_int = CT_SelectableSpanRow;
        contents_type_names: *const c_char[] = { "Text", "Button", "SmallButton", "FillButton", "Selectable", "Selectable (span row)" };
        static let freeze_cols: c_int = 1;
        static let freeze_rows: c_int = 1;
        static let items_count: c_int = template_items_names.len() * 2;
        static let mut outer_size_value: ImVec2 =  ImVec2::new(0.0, TEXT_BASE_HEIGHT * 12);
        static let row_min_height: c_float =  0.0; // Auto
        static let inner_width_with_scroll: c_float =  0.0; // Auto-extend
        static let mut outer_size_enabled: bool =  true;
        static let mut show_headers: bool =  true;
        static let mut show_wrapped_text: bool =  false;
        //static ImGuiTextFilter filter;
        //SetNextItemOpen(true, ImGuiCond_Once); // FIXME-TABLE: Enabling this results in initial clipped first pass on table which tend to affects column sizing
        if (TreeNode("Options"))
        {
            // Make the UI compact because there are so many fields
            PushStyleCompact();
            PushItemWidth(TEXT_BASE_WIDTH * 28.0);

            if (TreeNodeEx("Features:", ImGuiTreeNodeFlags_DefaultOpen))
            {
                CheckboxFlags("ImGuiTableFlags_Resizable", &flags, ImGuiTableFlags_Resizable);
                CheckboxFlags("ImGuiTableFlags_Reorderable", &flags, ImGuiTableFlags_Reorderable);
                CheckboxFlags("ImGuiTableFlags_Hideable", &flags, ImGuiTableFlags_Hideable);
                CheckboxFlags("ImGuiTableFlags_Sortable", &flags, ImGuiTableFlags_Sortable);
                CheckboxFlags("ImGuiTableFlags_NoSavedSettings", &flags, ImGuiTableFlags_NoSavedSettings);
                CheckboxFlags("ImGuiTableFlags_ContextMenuInBody", &flags, ImGuiTableFlags_ContextMenuInBody);
                TreePop();
            }

            if (TreeNodeEx("Decorations:", ImGuiTreeNodeFlags_DefaultOpen))
            {
                CheckboxFlags("ImGuiTableFlags_RowBg", &flags, ImGuiTableFlags_RowBg);
                CheckboxFlags("ImGuiTableFlags_BordersV", &flags, ImGuiTableFlags_BordersV);
                CheckboxFlags("ImGuiTableFlags_BordersOuterV", &flags, ImGuiTableFlags_BordersOuterV);
                CheckboxFlags("ImGuiTableFlags_BordersInnerV", &flags, ImGuiTableFlags_BordersInnerV);
                CheckboxFlags("ImGuiTableFlags_BordersH", &flags, ImGuiTableFlags_BordersH);
                CheckboxFlags("ImGuiTableFlags_BordersOuterH", &flags, ImGuiTableFlags_BordersOuterH);
                CheckboxFlags("ImGuiTableFlags_BordersInnerH", &flags, ImGuiTableFlags_BordersInnerH);
                CheckboxFlags("ImGuiTableFlags_NoBordersInBody", &flags, ImGuiTableFlags_NoBordersInBody); SameLine(); HelpMarker("Disable vertical borders in columns Body (borders will always appears in Headers");
                CheckboxFlags("ImGuiTableFlags_NoBordersInBodyUntilResize", &flags, ImGuiTableFlags_NoBordersInBodyUntilResize); SameLine(); HelpMarker("Disable vertical borders in columns Body until hovered for resize (borders will always appears in Headers)");
                TreePop();
            }

            if (TreeNodeEx("Sizing:", ImGuiTreeNodeFlags_DefaultOpen))
            {
                EditTableSizingFlags(&flags);
                SameLine(); HelpMarker("In the Advanced demo we override the policy of each column so those table-wide settings have less effect that typical.");
                CheckboxFlags("ImGuiTableFlags_NoHostExtendX", &flags, ImGuiTableFlags_NoHostExtendX);
                SameLine(); HelpMarker("Make outer width auto-fit to columns, overriding outer_size.x value.\n\nOnly available when ScrollX/ScrollY are disabled and Stretch columns are not used.");
                CheckboxFlags("ImGuiTableFlags_NoHostExtendY", &flags, ImGuiTableFlags_NoHostExtendY);
                SameLine(); HelpMarker("Make outer height stop exactly at outer_size.y (prevent auto-extending table past the limit).\n\nOnly available when ScrollX/ScrollY are disabled. Data below the limit will be clipped and not visible.");
                CheckboxFlags("ImGuiTableFlags_NoKeepColumnsVisible", &flags, ImGuiTableFlags_NoKeepColumnsVisible);
                SameLine(); HelpMarker("Only available if ScrollX is disabled.");
                CheckboxFlags("ImGuiTableFlags_PreciseWidths", &flags, ImGuiTableFlags_PreciseWidths);
                SameLine(); HelpMarker("Disable distributing remainder width to stretched columns (width allocation on a 100-wide table with 3 columns: Without this flag: 33,33,34. With this flag: 33,33,33). With larger number of columns, resizing will appear to be less smooth.");
                CheckboxFlags("ImGuiTableFlags_NoClip", &flags, ImGuiTableFlags_NoClip);
                SameLine(); HelpMarker("Disable clipping rectangle for every individual columns (reduce draw command count, items will be able to overflow into other columns). Generally incompatible with ScrollFreeze options.");
                TreePop();
            }

            if (TreeNodeEx("Padding:", ImGuiTreeNodeFlags_DefaultOpen))
            {
                CheckboxFlags("ImGuiTableFlags_PadOuterX", &flags, ImGuiTableFlags_PadOuterX);
                CheckboxFlags("ImGuiTableFlags_NoPadOuterX", &flags, ImGuiTableFlags_NoPadOuterX);
                CheckboxFlags("ImGuiTableFlags_NoPadInnerX", &flags, ImGuiTableFlags_NoPadInnerX);
                TreePop();
            }

            if (TreeNodeEx("Scrolling:", ImGuiTreeNodeFlags_DefaultOpen))
            {
                CheckboxFlags("ImGuiTableFlags_ScrollX", &flags, ImGuiTableFlags_ScrollX);
                SameLine();
                SetNextItemWidth(GetFrameHeight());
                DragInt("freeze_cols", &freeze_cols, 0.2f, 0, 9, None, ImGuiSliderFlags_NoInput);
                CheckboxFlags("ImGuiTableFlags_ScrollY", &flags, ImGuiTableFlags_ScrollY);
                SameLine();
                SetNextItemWidth(GetFrameHeight());
                DragInt("freeze_rows", &freeze_rows, 0.2f, 0, 9, None, ImGuiSliderFlags_NoInput);
                TreePop();
            }

            if (TreeNodeEx("Sorting:", ImGuiTreeNodeFlags_DefaultOpen))
            {
                CheckboxFlags("ImGuiTableFlags_SortMulti", &flags, ImGuiTableFlags_SortMulti);
                SameLine(); HelpMarker("When sorting is enabled: hold shift when clicking headers to sort on multiple column. TableGetSortSpecs() may return specs where (SpecsCount > 1).");
                CheckboxFlags("ImGuiTableFlags_SortTristate", &flags, ImGuiTableFlags_SortTristate);
                SameLine(); HelpMarker("When sorting is enabled: allow no sorting, disable default sorting. TableGetSortSpecs() may return specs where (SpecsCount == 0).");
                TreePop();
            }

            if (TreeNodeEx("Other:", ImGuiTreeNodeFlags_DefaultOpen))
            {
                Checkbox("show_headers", &show_headers);
                Checkbox("show_wrapped_text", &show_wrapped_text);

                DragFloat2("##OuterSize", &outer_size_value.x);
                SameLine(0.0, GetStyle().ItemInnerSpacing.x);
                Checkbox("outer_size", &outer_size_enabled);
                SameLine();
                HelpMarker("If scrolling is disabled (ScrollX and ScrollY not set):\n"
                    "- The table is output directly in the parent window.\n"
                    "- OuterSize.x < 0.0 will right-align the table.\n"
                    "- OuterSize.x = 0.0 will narrow fit the table unless there are any Stretch column.\n"
                    "- OuterSize.y then becomes the minimum size for the table, which will extend vertically if there are more rows (unless NoHostExtendY is set).");

                // From a user point of view we will tend to use 'inner_width' differently depending on whether our table is embedding scrolling.
                // To facilitate toying with this demo we will actually pass 0.0 to the BeginTable() when ScrollX is disabled.
                DragFloat("inner_width (when ScrollX active)", &inner_width_with_scroll, 1.0, 0.0, f32::MAX);

                DragFloat("row_min_height", &row_min_height, 1.0, 0.0, f32::MAX);
                SameLine(); HelpMarker("Specify height of the Selectable item.");

                DragInt("items_count", &items_count, 0.1f, 0, 9999);
                Combo("items_type (first column)", &contents_type, contents_type_names, contents_type_names.len());
                //filter.Draw("filter");
                TreePop();
            }

            PopItemWidth();
            PopStyleCompact();
            Spacing();
            TreePop();
        }

        // Update item list if we changed the number of items
        static Vec<MyItem> items;
        static Vec<c_int> selection;
        static let mut items_need_sort: bool =  false;
        if (items.Size != items_count)
        {
            items.resize(items_count, MyItem());
            for (let n: c_int = 0; n < items_count; n++)
            {
                let template_n: c_int = n % template_items_names.len();
                MyItem& item = items[n];
                item.ID = n;
                item.Name = template_items_names[template_n];
                item.Quantity = if template_n == 3 { 10} else { if template_n == 4 { 20 } else { 0 }}; // Assign default quantities
            }
        }

        let parent_draw_list: *const ImDrawList = GetWindowDrawList();
        let parent_draw_list_draw_cmd_count: c_int = parent_draw_list.CmdBuffer.len();
        table_scroll_cur: ImVec2, table_scroll_max; // For debug display
        let table_draw_list: *const ImDrawList= None;  // "

        // Submit table
        let inner_width_to_use: c_float =  if flag_set(flags, ImGuiTableFlags_ScrollX) { inner_width_with_scroll } else { 0.0 };
        if (BeginTable("table_advanced", 6, flags, if outer_size_enabled { outer_size_value } else { ImVec2::new(0, 0), inner_width_to_use) })
        {
            // Declare columns
            // We use the "user_id" parameter of TableSetupColumn() to specify a user id that will be stored in the sort specifications.
            // This is so our sort function can identify a column given our own identifier. We could also identify them based on their index!
            TableSetupColumn("ID",           ImGuiTableColumnFlags_DefaultSort | ImGuiTableColumnFlags_WidthFixed | ImGuiTableColumnFlags_NoHide, 0.0, MyItemColumnID_ID);
            TableSetupColumn("Name",         ImGuiTableColumnFlags_WidthFixed, 0.0, MyItemColumnID_Name);
            TableSetupColumn("Action",       ImGuiTableColumnFlags_NoSort | ImGuiTableColumnFlags_WidthFixed, 0.0, MyItemColumnID_Action);
            TableSetupColumn("Quantity",     ImGuiTableColumnFlags_PreferSortDescending, 0.0, MyItemColumnID_Quantity);
            TableSetupColumn("Description",  if flag_set(flags, ImGuiTableFlags_NoHostExtendX) { 0 } else { ImGuiTableColumnFlags_WidthStretch }, 0.0, MyItemColumnID_Description);
            TableSetupColumn("Hidden",       ImGuiTableColumnFlags_DefaultHide | ImGuiTableColumnFlags_NoSort);
            TableSetupScrollFreeze(freeze_cols, freeze_rows);

            // Sort our data if sort specs have been changed!
            *mut ImGuiTableSortSpecs sorts_specs = TableGetSortSpecs();
            if sorts_specs && sorts_specs->SpecsDirty {
                items_need_sort = true;}
            if (sorts_specs && items_need_sort && items.Size > 1)
            {
                MyItem::s_current_sort_specs = sorts_specs; // Store in variable accessible by the sort function.
                qsort(&items[0], items.Size, sizeof(items[0]), MyItem::CompareWithSortSpecs);
                MyItem::s_current_sort_specs= None;
                sorts_specs->SpecsDirty = false;
            }
            items_need_sort = false;

            // Take note of whether we are currently sorting based on the Quantity field,
            // we will use this to trigger sorting when we know the data of this column has been modified.
            let sorts_specs_using_quantity: bool = (TableGetColumnFlags(3) & ImGuiTableColumnFlags_IsSorted) != 0;

            // Show headers
            if show_headers {
                TableHeadersRow(); }

            // Show data
            // FIXME-TABLE FIXME-NAV: How we can get decent up/down even though we have the buttons here?
            PushButtonRepeat(true);
// #if 1
            // Demonstrate using clipper for large vertical lists
            ImGuiListClipper clipper;
            clipper.Begin(items.Size);
            while (clipper.Step())
            {
                for (let row_n: c_int = clipper.DisplayStart; row_n < clipper.DisplayEnd; row_n++)
// #else
            // Without clipper
            {
                for (let row_n: c_int = 0; row_n < items.Size; row_n++)
// #endif
                {
                    *mut MyItem item = &items[row_n];
                    //if (!filter.PassFilter(item.Name))
                    //    continue;

                    let item_is_selected: bool = selection.contains(item.ID);
                    PushID(item.ID);
                    TableNextRow(ImGuiTableRowFlags_None, row_min_height);

                    // For the demo purpose we can select among different type of items submitted in the first column
                    TableSetColumnIndex(0);
                    label: [c_char;32];
                    sprintf(label, "%04d", item.ID);
                    if contents_type == CT_Text {
                        TextUnformatted(label)(); }
                    else if contents_type == CT_Button {
                        Button(label)(); }
                    else if contents_type == CT_SmallButton {
                        SmallButton(label)(); }
                    else if (contents_type == CT_FillButton)
                        Button(label, ImVec2::new(-FLT_MIN, 0.0));
                    else if (contents_type == CT_Selectable || contents_type == CT_SelectableSpanRow)
                    {
                        selectable_flags ImGuiSelectableFlags = if (contents_type == CT_SelectableSpanRow) { ImGuiSelectableFlags_SpanAllColumns | ImGuiSelectableFlags_AllowItemOverlap } else { ImGuiSelectableFlags_None };
                        if (Selectable(label, item_is_selected, selectable_flags, ImVec2::new(0, row_min_height)))
                        {
                            if (GetIO().KeyCtrl)
                            {
                                if item_is_selected{
                                    selection.find_erase_unsorted(item.ID);}
                                else
                                    selection.push(item.ID);
                            }
                            else
                            {
                                selection.clear();
                                selection.push(item.ID);
                            }
                        }
                    }

                    if TableSetColumnIndex(1){
                        TextUnformatted(item.Name);}

                    // Here we demonstrate marking our data set as needing to be sorted again if we modified a quantity,
                    // and we are currently sorting on the column showing the Quantity.
                    // To avoid triggering a sort while holding the button, we only trigger it when the button has been released.
                    // You will probably need a more advanced system in your code if you want to automatically sort when a specific entry changes.
                    if (TableSetColumnIndex(2))
                    {
                        if (SmallButton("Chop")) { item->Quantity += 1; }
                        if (sorts_specs_using_quantity && IsItemDeactivated()) { items_need_sort = true; }
                        SameLine();
                        if (SmallButton("Eat")) { item->Quantity -= 1; }
                        if (sorts_specs_using_quantity && IsItemDeactivated()) { items_need_sort = true; }
                    }

                    if (TableSetColumnIndex(3))
                        Text("{}", item->Quantity);

                    TableSetColumnIndex(4);
                    if show_wrapped_text {
                        TextWrapped("Lorem ipsum dolor sit amet")(); }
                    else
                        Text("Lorem ipsum dolor sit amet");

                    if TableSetColumnIndex(5) {
                        Text("1234")(); }

                    PopID();
                }
            }
            PopButtonRepeat();

            // Store some info to display debug details below
            table_scroll_cur = ImVec2::new(GetScrollX(), GetScrollY());
            table_scroll_max = ImVec2::new(GetScrollMaxX(), GetScrollMaxY());
            table_draw_list = GetWindowDrawList();
            EndTable();
        }
        static let mut show_debug_details: bool =  false;
        Checkbox("Debug details", &show_debug_details);
        if (show_debug_details && table_draw_list)
        {
            SameLine(0.0, 0.0);
            let table_draw_list_draw_cmd_count: c_int = table_draw_list.CmdBuffer.len();
            if (table_draw_list == parent_draw_list)
                Text(": DrawCmd: +{} (in same window)",
                    table_draw_list_draw_cmd_count - parent_draw_list_draw_cmd_count);
            else
                Text(": DrawCmd: +{} (in child window), Scroll: (%.f/{}) (%.f/{})",
                    table_draw_list_draw_cmd_count - 1, table_scroll_cur.x, table_scroll_max.x, table_scroll_cur.y, table_scroll_max.y);
        }
        TreePop();
    }

    PopID();

    ShowDemoWindowColumns();

    if disable_indent {
        PopStyleVar(); }
}

// Demonstrate old/legacy Columns API!
// [2020: Columns are under-featured and not maintained. Prefer using the more flexible and powerful BeginTable() API!]
pub unsafe fn ShowDemoWindowColumns()
{
    IMGUI_DEMO_MARKER("Columns (legacy API)");
    let mut open: bool =  TreeNode("Legacy Columns API");
    SameLine();
    HelpMarker("Columns() is an old API! Prefer using the more flexible and powerful BeginTable() API!");
    if !open { return ; }

    // Basic columns
    IMGUI_DEMO_MARKER("Columns (legacy API)/Basic");
    if (TreeNode("Basic"))
    {
        Text("Without border:");
        Columns(3, "mycolumns3", false);  // 3-ways, no border
        Separator();
        for (let n: c_int = 0; n < 14; n++)
        {
            label: [c_char;32];
            sprintf(label, "Item {}", n);
            if (Selectable(label)) {}
            //if (Button(label, ImVec2::new(-FLT_MIN,0.0))) {}
            NextColumn();
        }
        Columns(1);
        Separator();

        Text("With border:");
        Columns(4, "mycolumns"); // 4-ways, with border
        Separator();
        Text("ID"); NextColumn();
        Text("Name"); NextColumn();
        Text("Path"); NextColumn();
        Text("Hovered"); NextColumn();
        Separator();
        *const names: [c_char;3] = { "One", "Two", "Three" };
        *const paths: [c_char;3] = { "/path/one", "/path/two", "/path/three" };
        static let selected: c_int = -1;
        for (let i: c_int = 0; i < 3; i++)
        {
            label: [c_char;32];
            sprintf(label, "%04d", i);
            if Selectable(label, selected == i, ImGuiSelectableFlags_SpanAllColumns) {
                selected = i;}
            let mut hovered: bool =  IsItemHovered();
            NextColumn();
            Text(names[i]); NextColumn();
            Text(paths[i]); NextColumn();
            Text("{}", hovered); NextColumn();
        }
        Columns(1);
        Separator();
        TreePop();
    }

    IMGUI_DEMO_MARKER("Columns (legacy API)/Borders");
    if (TreeNode("Borders"))
    {
        // NB: Future columns API should allow automatic horizontal borders.
        static let mut h_borders: bool =  true;
        static let mut v_borders: bool =  true;
        static let columns_count: c_int = 4;
        let lines_count: c_int = 3;
        SetNextItemWidth(GetFontSize() * 8);
        DragInt("##columns_count", &columns_count, 0.1f, 2, 10, "{} columns");
        if columns_count < 2 {
            columns_count = 2;}
        SameLine();
        Checkbox("horizontal", &h_borders);
        SameLine();
        Checkbox("vertical", &v_borders);
        Columns(columns_count, None, v_borders);
        for (let i: c_int = 0; i < columns_count * lines_count; i++)
        {
            if h_borders && GetColumnIndex() == 0{
                Separator();}
            Text("{}{}{}", 'a' + i, 'a' + i, 'a' + i);
            Text("Width {}", GetColumnWidth());
            Text("Avail {}", GetContentRegionAvail().x);
            Text("Offset {}", GetColumnOffset());
            Text("Long text that is likely to clip");
            Button("Button", ImVec2::new(-FLT_MIN, 0.0));
            NextColumn();
        }
        Columns(1);
        if h_borders {
            Separator(); }
        TreePop();
    }

    // Create multiple items in a same cell before switching to next column
    IMGUI_DEMO_MARKER("Columns (legacy API)/Mixed items");
    if (TreeNode("Mixed items"))
    {
        Columns(3, "mixed");
        Separator();

        Text("Hello");
        Button("Banana");
        NextColumn();

        Text("ImGui");
        Button("Apple");
        static let foo: c_float =  1.0;
        InputFloat("red", &foo, 0.05f, 0, "{}");
        Text("An extra line here.");
        NextColumn();

        Text("Sailor");
        Button("Corniflower");
        static let bar: c_float =  1.0;
        InputFloat("blue", &bar, 0.05f, 0, "{}");
        NextColumn();

        if (CollapsingHeader("Category A")) { Text("Blah blah blah"); } NextColumn();
        if (CollapsingHeader("Category B")) { Text("Blah blah blah"); } NextColumn();
        if (CollapsingHeader("Category C")) { Text("Blah blah blah"); } NextColumn();
        Columns(1);
        Separator();
        TreePop();
    }

    // Word wrapping
    IMGUI_DEMO_MARKER("Columns (legacy API)/Word-wrapping");
    if (TreeNode("Word-wrapping"))
    {
        Columns(2, "word-wrapping");
        Separator();
        TextWrapped("The quick brown fox jumps over the lazy dog.");
        TextWrapped("Hello Left");
        NextColumn();
        TextWrapped("The quick brown fox jumps over the lazy dog.");
        TextWrapped("Hello Right");
        Columns(1);
        Separator();
        TreePop();
    }

    IMGUI_DEMO_MARKER("Columns (legacy API)/Horizontal Scrolling");
    if (TreeNode("Horizontal Scrolling"))
    {
        SetNextWindowContentSize(ImVec2::new(1500, 0.0));
        let child_size: ImVec2 = ImVec2::new(0, GetFontSize() * 20f32);
        BeginChild("##ScrollingRegion", child_size, false, ImGuiWindowFlags_HorizontalScrollbar);
        Columns(10);

        // Also demonstrate using clipper for large vertical lists
        let ITEMS_COUNT: c_int = 2000;
        ImGuiListClipper clipper;
        clipper.Begin(ITEMS_COUNT);
        while (clipper.Step())
        {
            for (let i: c_int = clipper.DisplayStart; i < clipper.DisplayEnd; i++)
                for (let j: c_int = 0; j < 10; j++)
                {
                    Text("Line {} Column {}...", i, j);
                    NextColumn();
                }
        }
        Columns(1);
        EndChild();
        TreePop();
    }

    IMGUI_DEMO_MARKER("Columns (legacy API)/Tree");
    if (TreeNode("Tree"))
    {
        Columns(2, "tree", true);
        for (let x: c_int = 0; x < 3; x++)
        {
            let mut open1: bool =  TreeNode(x, "Node{}", x);
            NextColumn();
            Text("Node contents");
            NextColumn();
            if (open1)
            {
                for (let y: c_int = 0; y < 3; y++)
                {
                    let mut open2: bool =  TreeNode(y, "Node{}.{}", x, y);
                    NextColumn();
                    Text("Node contents");
                    if (open2)
                    {
                        Text("Even more contents");
                        if (TreeNode("Tree in column"))
                        {
                            Text("The quick brown fox jumps over the lazy dog");
                            TreePop();
                        }
                    }
                    NextColumn();
                    if open2 {
                        TreePop(); }
                }
                TreePop();
            }
        }
        Columns(1);
        TreePop();
    }

    TreePop();
}

namespace ImGui { extern *mut ImGuiKeyData GetKeyData(ImGuiKey key); }

pub unsafe fn ShowDemoWindowMisc()
{
    IMGUI_DEMO_MARKER("Filtering");
    if (CollapsingHeader("Filtering"))
    {
        // Helper class to easy setup a text filter.
        // You may want to implement a more feature-full filtering scheme in your own application.
        static ImGuiTextFilter filter;
        Text("Filter usage:\n"
                    "  \"\"         display all lines\n"
                    "  \"xxx\"      display lines containing \"xxx\"\n"
                    "  \"xxx,yyy\"  display lines containing \"xxx\" or \"yyy\"\n"
                    "  \"-xxx\"     hide lines containing \"xxx\"");
        filter.Draw();
        lines: *const c_char[] = { "aaa1.c", "bbb1.c", "ccc1.c", "aaa2.cpp", "bbb2.cpp", "ccc2.cpp", "abc.h", "hello, world" };
        for (let i: c_int = 0; i < lines.len(); i++)
            if (filter.PassFilter(lines[i]))
                BulletText("{}", lines[i]);
    }

    IMGUI_DEMO_MARKER("Inputs, Navigation & Focus");
    if (CollapsingHeader("Inputs, Navigation & Focus"))
    {
        ImGuiIO& io = GetIO();

        // Display ImGuiIO output flags
        IMGUI_DEMO_MARKER("Inputs, Navigation & Focus/Output");
        SetNextItemOpen(true, ImGuiCond_Once);
        if (TreeNode("Output"))
        {
            Text("io.WantCaptureMouse: {}", io.WantCaptureMouse);
            Text("io.WantCaptureMouseUnlessPopupClose: {}", io.WantCaptureMouseUnlessPopupClose);
            Text("io.WantCaptureKeyboard: {}", io.WantCaptureKeyboard);
            Text("io.WantTextInput: {}", io.WantTextInput);
            Text("io.WantSetMousePos: {}", io.WantSetMousePos);
            Text("io.NavActive: {}, io.NavVisible: {}", io.NavActive, io.NavVisible);
            TreePop();
        }

        // Display Mouse state
        IMGUI_DEMO_MARKER("Inputs, Navigation & Focus/Mouse State");
        if (TreeNode("Mouse State"))
        {
            if (IsMousePosValid())
                Text("Mouse pos: (%g, %g)", io.MousePos.x, io.MousePos.y);
            else
                Text("Mouse pos: <INVALID>");
            Text("Mouse delta: (%g, %g)", io.MouseDelta.x, io.MouseDelta.y);

            let count: c_int = IM_ARRAYSIZE(io.MouseDown);
            Text("Mouse down:");         for (let i: c_int = 0; i < count; i++) if (IsMouseDown(i))      { SameLine(); Text("b{} ({}2f secs)", i, io.MouseDownDuration[i]); }
            Text("Mouse clicked:");      for (let i: c_int = 0; i < count; i++) if (IsMouseClicked(i))   { SameLine(); Text("b{} ({})", i, GetMouseClickedCount(i)); }
            Text("Mouse released:");     for (let i: c_int = 0; i < count; i++) if (IsMouseReleased(i))  { SameLine(); Text("b{}", i); }
            Text("Mouse wheel: {}", io.MouseWheel);
            Text("Pen Pressure: {}", io.PenPressure); // Note: currently unused
            TreePop();
        }

        // Display Keyboard/Mouse state
        IMGUI_DEMO_MARKER("Inputs, Navigation & Focus/Keyboard, Gamepad & Navigation State");
        if (TreeNode("Keyboard, Gamepad & Navigation State"))
        {
            // We iterate both legacy native range and named ImGuiKey ranges, which is a little odd but this allow displaying the data for old/new backends.
            // User code should never have to go through such hoops: old code may use native keycodes, new code may use ImGuiKey codes.
// #ifdef IMGUI_DISABLE_OBSOLETE_KEYIO
            struct funcs { static IsLegacyNativeDupe: bool(ImGuiKey) { return false; } };
            let mut key_first: ImGuiKey =  ImGuiKey_NamedKey_BEGIN;
// #else
            struct funcs { static IsLegacyNativeDupe: bool(ImGuiKey key) { return key < 512 && GetIO().KeyMap[key] != -1; } }; // Hide Native<>ImGuiKey duplicates when both exists in the array
            let mut key_first: ImGuiKey =  0;
            //Text("Legacy raw:");       for (ImGuiKey key = key_first; key < ImGuiKey_COUNT; key++) { if (io.KeysDown[key]) { SameLine(); Text("\"{}\" {}", GetKeyName(key), key); } }
// #endif
            Text("Keys down:");          for (let mut key: ImGuiKey =  key_first; key < ImGuiKey_COUNT; key++) { if (funcs::IsLegacyNativeDupe(key)) continue; if (IsKeyDown(key)) { SameLine(); Text("\"{}\" {} ({}2f secs)", GetKeyName(key), key, GetKeyData(key)->DownDuration); } }
            Text("Keys pressed:");       for (let mut key: ImGuiKey =  key_first; key < ImGuiKey_COUNT; key++) { if (funcs::IsLegacyNativeDupe(key)) continue; if (IsKeyPressed(key)) { SameLine(); Text("\"{}\" {}", GetKeyName(key), key); } }
            Text("Keys released:");      for (let mut key: ImGuiKey =  key_first; key < ImGuiKey_COUNT; key++) { if (funcs::IsLegacyNativeDupe(key)) continue; if (IsKeyReleased(key)) { SameLine(); Text("\"{}\" {}", GetKeyName(key), key); } }
            Text("Keys mods: {}{}{}{}", io.KeyCtrl ? "CTRL " : "", io.KeyShift ? "SHIFT " : "", io.KeyAlt ? "ALT " : "", io.KeySuper ? "SUPER " : "");
            Text("Chars queue:");        for (let i: c_int = 0; i < io.InputQueueCharacters.Size; i++) { let c: ImWchar = io.InputQueueCharacters[i]; SameLine();  Text("\'{}\' ({})", (c > ' ' && c <= 255) ? c : '?', c); } // FIXME: We should convert 'c' to UTF-8 here but the functions are not public.

            // Draw an arbitrary US keyboard layout to visualize translated keys
            {
                let key_size: ImVec2 = ImVec2::new(35f32, 35.0);
                key_rounding: c_float = 3.0;
                let key_face_size: ImVec2 = ImVec2::new(25f32, 25.0);
                let key_face_pos: ImVec2 = ImVec2::new(5f32, 3.0);
                key_face_rounding: c_float = 2.0;
                let key_label_pos: ImVec2 = ImVec2::new(7.0, 4.0);
                let key_step: ImVec2 = ImVec2::new(key_size.x - 1.0, key_size.y - 1.0);
                key_row_offset: c_float = 9.0;

                let board_min: ImVec2 = GetCursorScreenPos();
                let board_max: ImVec2 = ImVec2::new(board_min.x + 3 * key_step.x + 2 * key_row_offset + 10.0, board_min.y + 3 * key_step.y + 10.0);
                let start_pos: ImVec2 = ImVec2::new(board_min.x + 5f32 - key_step.x, board_min.y);

                struct KeyLayoutData { Row: c_int, Col; Label: *const c_char; ImGuiKey Key; };
                const KeyLayoutData keys_to_display[] =
                {
                    { 0, 0, "", ImGuiKey_Tab },      { 0, 1, "Q", ImGuiKey_Q }, { 0, 2, "W", ImGuiKey_W }, { 0, 3, "E", ImGuiKey_E }, { 0, 4, "R", ImGuiKey_R },
                    { 1, 0, "", ImGuiKey_CapsLock }, { 1, 1, "A", ImGuiKey_A }, { 1, 2, "S", ImGuiKey_S }, { 1, 3, "D", ImGuiKey_D }, { 1, 4, "F", ImGuiKey_F },
                    { 2, 0, "", ImGuiKey_LeftShift },{ 2, 1, "Z", ImGuiKey_Z }, { 2, 2, "X", ImGuiKey_X }, { 2, 3, "C", ImGuiKey_C }, { 2, 4, "V", ImGuiKey_V }
                };

                // Elements rendered manually via ImDrawList API are not clipped automatically.
                // While not strictly necessary, here IsItemVisible() is used to avoid rendering these shapes when they are out of view.
                Dummy(ImVec2::new(board_max.x - board_min.x, board_max.y - board_min.y));
                if (IsItemVisible())
                {
                    draw_list: *mut ImDrawList = GetWindowDrawList();
                    draw_list.PushClipRect(board_min, board_max, true);
                    for (let n: c_int = 0; n < keys_to_display.len(); n++)
                    {
                        let key_data: *const KeyLayoutData = &keys_to_display[n];
                        let key_min: ImVec2 = ImVec2::new(start_pos.x + key_Data.Col * key_step.x + key_Data.Row * key_row_offset, start_pos.y + key_Data.Row * key_step.y);
                        let key_max: ImVec2 = ImVec2::new(key_min.x + key_size.x, key_min.y + key_size.y);
                        draw_list.AddRectFilled(key_min, key_max, IM_COL32(204, 204, 204, 255), key_rounding);
                        draw_list.AddRect(key_min, key_max, IM_COL32(24, 24, 24, 255), key_rounding);
                        let face_min: ImVec2 = ImVec2::new(key_min.x + key_face_pos.x, key_min.y + key_face_pos.y);
                        let face_max: ImVec2 = ImVec2::new(face_min.x + key_face_size.x, face_min.y + key_face_size.y);
                        draw_list.AddRect(face_min, face_max, IM_COL32(193, 193, 193, 255), key_face_rounding, ImDrawFlags_None, 2.0);
                        draw_list.AddRectFilled(face_min, face_max, IM_COL32(252, 252, 252, 255), key_face_rounding);
                        let label_min: ImVec2 = ImVec2::new(key_min.x + key_label_pos.x, key_min.y + key_label_pos.y);
                        draw_list.AddText(label_min, IM_COL32(64, 64, 64, 255), key_Data.Label);
                        if (IsKeyDown(key_Data.Key))
                            draw_list.AddRectFilled(key_min, key_max, IM_COL32(255, 0, 0, 128), key_rounding);
                    }
                    draw_list.PopClipRect();
                }
            }
            TreePop();
        }

        if (TreeNode("Capture override"))
        {
            HelpMarker(
                "The value of io.WantCaptureMouse and io.WantCaptureKeyboard are normally set by Dear ImGui "
                "to instruct your application of how to route inputs. Typically, when a value is true, it means "
                "Dear ImGui wants the corresponding inputs and we expect the underlying application to ignore them.\n\n"
                "The most typical is => when hovering a window, Dear ImGui set io.WantCaptureMouse to true, "
                "and underlying application should ignore mouse inputs (in practice there are many and more subtle "
                "rules leading to how those flags are set).");

            Text("io.WantCaptureMouse: {}", io.WantCaptureMouse);
            Text("io.WantCaptureMouseUnlessPopupClose: {}", io.WantCaptureMouseUnlessPopupClose);
            Text("io.WantCaptureKeyboard: {}", io.WantCaptureKeyboard);

            HelpMarker(
                "Hovering the colored canvas will override io.WantCaptureXXX fields.\n"
                "Notice how normally (when set to none), the value of io.WantCaptureKeyboard would be false when hovering and true when clicking.");
            static let capture_override_mouse: c_int = -1;
            static let capture_override_keyboard: c_int = -1;
            capture_override_desc: *const c_char[] = { "None", "Set to false", "Set to true" };
            SetNextItemWidth(GetFontSize() * 15);
            SliderInt("SetNextFrameWantCaptureMouse()", &capture_override_mouse, -1, 1, capture_override_desc[capture_override_mouse + 1], ImGuiSliderFlags_AlwaysClamp);
            SetNextItemWidth(GetFontSize() * 15);
            SliderInt("SetNextFrameWantCaptureKeyboard()", &capture_override_keyboard, -1, 1, capture_override_desc[capture_override_keyboard + 1], ImGuiSliderFlags_AlwaysClamp);

            ColorButton("##panel", ImVec4(0.7f, 0.1f, 0.7f, 1.0), ImGuiColorEditFlags_NoTooltip | ImGuiColorEditFlags_NoDragDrop, ImVec2::new(256f32, 192.0)); // Dummy item
            if (IsItemHovered() && capture_override_mouse != -1)
                SetNextFrameWantCaptureMouse(capture_override_mouse == 1);
            if (IsItemHovered() && capture_override_keyboard != -1)
                SetNextFrameWantCaptureKeyboard(capture_override_keyboard == 1);

            TreePop();
        }

        IMGUI_DEMO_MARKER("Inputs, Navigation & Focus/Tabbing");
        if (TreeNode("Tabbing"))
        {
            Text("Use TAB/SHIFT+TAB to cycle through keyboard editable fields.");
            static buf: [c_char;32] = "hello";
            InputText("1", buf, buf.len());
            InputText("2", buf, buf.len());
            InputText("3", buf, buf.len());
            PushAllowKeyboardFocus(false);
            InputText("4 (tab skip)", buf, buf.len());
            SameLine(); HelpMarker("Item won't be cycled through when using TAB or Shift+Tab.");
            PopAllowKeyboardFocus();
            InputText("5", buf, buf.len());
            TreePop();
        }

        IMGUI_DEMO_MARKER("Inputs, Navigation & Focus/Focus from code");
        if (TreeNode("Focus from code"))
        {
            let mut focus_1: bool =  Button("Focus on 1"); SameLine();
            let mut focus_2: bool =  Button("Focus on 2"); SameLine();
            let mut focus_3: bool =  Button("Focus on 3");
            let has_focus: c_int = 0;
            static buf: [c_char;128] = "click on a button to set focus";

            if focus_1 {  SetKeyboardFocusHere(); }
            InputText("1", buf, buf.len());
            if IsItemActive() {  has_focus = 1;}

            if focus_2 {  SetKeyboardFocusHere(); }
            InputText("2", buf, buf.len());
            if IsItemActive() {  has_focus = 2;}

            PushAllowKeyboardFocus(false);
            if focus_3 {  SetKeyboardFocusHere(); }
            InputText("3 (tab skip)", buf, buf.len());
            if IsItemActive() {  has_focus = 3;}
            SameLine(); HelpMarker("Item won't be cycled through when using TAB or Shift+Tab.");
            PopAllowKeyboardFocus();

            if (has_focus)
                Text("Item with focus: {}", has_focus);
            else
                Text("Item with focus: <none>");

            // Use >= 0 parameter to SetKeyboardFocusHere() to focus an upcoming item
            staticf3: c_float[3] = { 0.0, 0.0, 0.0 };
            let focus_ahead: c_int = -1;
            if (Button("Focus on X")) { focus_ahead = 0; } SameLine();
            if (Button("Focus on Y")) { focus_ahead = 1; } SameLine();
            if (Button("Focus on Z")) { focus_ahead = 2; }
            if (focus_ahead != -1) SetKeyboardFocusHere(focus_ahead);
            SliderFloat3("Float3", &f3[0], 0.0, 1.0);

            TextWrapped("NB: Cursor & selection are preserved when refocusing last used item in code.");
            TreePop();
        }

        IMGUI_DEMO_MARKER("Inputs, Navigation & Focus/Dragging");
        if (TreeNode("Dragging"))
        {
            TextWrapped("You can use GetMouseDragDelta(0) to query for the dragged amount on any widget.");
            for (let button: c_int = 0; button < 3; button++)
            {
                Text("IsMouseDragging({}):", button);
                Text("  w/ default threshold: {},", IsMouseDragging(button));
                Text("  w/ zero threshold: {},", IsMouseDragging(button, 0.0));
                Text("  w/ large threshold: {},", IsMouseDragging(button, 20f32));
            }

            Button("Drag Me");
            if (IsItemActive())
                GetForegroundDrawList().AddLine(io.MouseClickedPos[0], io.MousePos, GetColorU32(ImGuiCol_Button, 0.0), 4.0); // Draw a line between the button and the mouse cursor

            // Drag operations gets "unlocked" when the mouse has moved past a certain threshold
            // (the default threshold is stored in io.MouseDragThreshold). You can request a lower or higher
            // threshold using the second parameter of IsMouseDragging() and GetMouseDragDelta().
            let value_raw: ImVec2 = GetMouseDragDelta(0, 0.0);
            let value_with_lock_threshold: ImVec2 = GetMouseDragDelta(0);
            let mouse_delta: ImVec2 = io.MouseDelta;
            Text("GetMouseDragDelta(0):");
            Text("  w/ default threshold: ({}, {})", value_with_lock_threshold.x, value_with_lock_threshold.y);
            Text("  w/ zero threshold: ({}, {})", value_raw.x, value_raw.y);
            Text("io.MouseDelta: ({}, {})", mouse_delta.x, mouse_delta.y);
            TreePop();
        }

        IMGUI_DEMO_MARKER("Inputs, Navigation & Focus/Mouse cursors");
        if (TreeNode("Mouse cursors"))
        {
            mouse_cursors_names: *const c_char[] = { "Arrow", "TextInput", "ResizeAll", "ResizeNS", "ResizeEW", "ResizeNESW", "ResizeNWSE", "Hand", "NotAllowed" };
            // IM_ASSERT(IM_ARRAYSIZE(mouse_cursors_names) == ImGuiMouseCursor_COUNT);

            ImGuiMouseCursor current = GetMouseCursor();
            Text("Current mouse cursor = {}: {}", current, mouse_cursors_names[current]);
            Text("Hover to see mouse cursors:");
            SameLine(); HelpMarker(
                "Your application can render a different mouse cursor based on what GetMouseCursor() returns. "
                "If software cursor rendering (io.MouseDrawCursor) is set ImGui will draw the right cursor for you, "
                "otherwise your backend needs to handle it.");
            for (let i: c_int = 0; i < ImGuiMouseCursor_COUNT; i++)
            {
                label: [c_char;32];
                sprintf(label, "Mouse cursor {}: {}", i, mouse_cursors_names[i]);
                Bullet(); Selectable(label, false);
                if IsItemHovered() {
                    SetMouseCursor(i)(); }
            }
            TreePop();
        }
    }
}

//-----------------------------------------------------------------------------
// [SECTION] About Window / ShowAboutWindow()
// Access from Dear ImGui Demo -> Tools -> About
//-----------------------------------------------------------------------------

pub unsafe fn ShowAboutWindow(p_open: *mut bool)
{
    if (!Begin("About Dear ImGui", p_open, ImGuiWindowFlags_AlwaysAutoResize))
    {
        End();
        return;
    }
    IMGUI_DEMO_MARKER("Tools/About Dear ImGui");
    Text("Dear ImGui {}", GetVersion());
    Separator();
    Text("By Omar Cornut and all Dear ImGui contributors.");
    Text("Dear ImGui is licensed under the MIT License, see LICENSE for more information.");

    static let mut show_config_info: bool =  false;
    Checkbox("Config/Build Information", &show_config_info);
    if (show_config_info)
    {
        ImGuiIO& io = GetIO();
        ImGuiStyle& style = GetStyle();

        let mut copy_to_clipboard: bool =  Button("Copy to clipboard");
        let child_size: ImVec2 = ImVec2::new(0, GetTextLineHeightWithSpacing() * 18);
        BeginChildFrame(GetID("cfg_infos"), child_size, ImGuiWindowFlags_NoMove);
        if (copy_to_clipboard)
        {
            LogToClipboard();
            LogText("```\n"); // Back quotes will make text appears without formatting when pasting on GitHub
        }

        Text("Dear ImGui {} ({})", IMGUI_VERSION, IMGUI_VERSION_NUM);
        Separator();
        Text("sizeof: {}, sizeof(ImDrawIdx): {}, sizeof(ImDrawVert): {}", sizeof, sizeof, sizeof(ImDrawVert));
        Text("define: __cplusplus={}", __cplusplus);
// #ifdef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
        Text("define: IMGUI_DISABLE_OBSOLETE_FUNCTIONS");
// #endif
// #ifdef IMGUI_DISABLE_OBSOLETE_KEYIO
        Text("define: IMGUI_DISABLE_OBSOLETE_KEYIO");
// #endif
// #ifdef IMGUI_DISABLE_WIN32_DEFAULT_CLIPBOARD_FUNCTIONS
        Text("define: IMGUI_DISABLE_WIN32_DEFAULT_CLIPBOARD_FUNCTIONS");
// #endif
// #ifdef IMGUI_DISABLE_WIN32_DEFAULT_IME_FUNCTIONS
        Text("define: IMGUI_DISABLE_WIN32_DEFAULT_IME_FUNCTIONS");
// #endif
// #ifdef IMGUI_DISABLE_WIN32_FUNCTIONS
        Text("define: IMGUI_DISABLE_WIN32_FUNCTIONS");
// #endif
// #ifdef IMGUI_DISABLE_DEFAULT_FORMAT_FUNCTIONS
        Text("define: IMGUI_DISABLE_DEFAULT_FORMAT_FUNCTIONS");
// #endif
// #ifdef IMGUI_DISABLE_DEFAULT_MATH_FUNCTIONS
        Text("define: IMGUI_DISABLE_DEFAULT_MATH_FUNCTIONS");
// #endif
// #ifdef IMGUI_DISABLE_DEFAULT_FILE_FUNCTIONS
        Text("define: IMGUI_DISABLE_DEFAULT_FILE_FUNCTIONS");
// #endif
// #ifdef IMGUI_DISABLE_FILE_FUNCTIONS
        Text("define: IMGUI_DISABLE_FILE_FUNCTIONS");
// #endif
// #ifdef IMGUI_DISABLE_DEFAULT_ALLOCATORS
        Text("define: IMGUI_DISABLE_DEFAULT_ALLOCATORS");
// #endif
// #ifdef IMGUI_USE_BGRA_PACKED_COLOR
        Text("define: IMGUI_USE_BGRA_PACKED_COLOR");
// #endif
// #ifdef _WIN32
        Text("define: _WIN32");
// #endif
// #ifdef _WIN64
        Text("define: _WIN64");
// #endif
// #ifdef __linux__
        Text("define: __linux__");
// #endif
// #ifdef __APPLE__
        Text("define: __APPLE__");
// #endif
// #ifdef _MSC_VER
        Text("define: _MSC_VER={}", _MSC_VER);
// #endif
// #ifdef _MSVC_LANG
        Text("define: _MSVC_LANG={}", _MSVC_LANG);
// #endif
// #ifdef __MINGW32__
        Text("define: __MINGW32__");
// #endif
// #ifdef __MINGW64__
        Text("define: __MINGW64__");
// #endif
// #ifdef __GNUC__
        Text("define: __GNUC__={}", __GNUC__);
// #endif
// #ifdef __clang_version__
        Text("define: __clang_version__={}", __clang_version__);
// #endif
// #ifdef IMGUI_HAS_VIEWPORT
        Text("define: IMGUI_HAS_VIEWPORT");
// #endif
// #ifdef IMGUI_HAS_DOCK
        Text("define: IMGUI_HAS_DOCK");
// #endif
        Separator();
        Text("io.BackendPlatformName: {}", if io.BackendPlatformName { io.BackendPlatformName } else {"NULL"});
        Text("io.BackendRendererName: {}", if io.BackendRendererName { io.BackendRendererName } else { "NULL" });
        Text("io.ConfigFlags: 0x{}", io.ConfigFlags);
        if io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard{        Text(" NavEnableKeyboard");}
        if io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad{         Text(" NavEnableGamepad");}
        if io.ConfigFlags & ImGuiConfigFlags_NavEnableSetMousePos{     Text(" NavEnableSetMousePos");}
        if (io.ConfigFlags & ImGuiConfigFlags_NavNoCaptureKeyboard)     Text(" NavNoCaptureKeyboard");
        if (io.ConfigFlags & ImGuiConfigFlags_NoMouse)                  Text(" NoMouse");
        if (io.ConfigFlags & ImGuiConfigFlags_NoMouseCursorChange)      Text(" NoMouseCursorChange");
        if (io.ConfigFlags & ImGuiConfigFlags_DockingEnable)            Text(" DockingEnable");
        if (io.ConfigFlags & ImGuiConfigFlags_ViewportsEnable)          Text(" ViewportsEnable");
        if (io.ConfigFlags & ImGuiConfigFlags_DpiEnableScaleViewports)  Text(" DpiEnableScaleViewports");
        if (io.ConfigFlags & ImGuiConfigFlags_DpiEnableScaleFonts)      Text(" DpiEnableScaleFonts");
        if io.MouseDrawCursor{                                         Text("io.MouseDrawCursor");}
        if io.ConfigViewportsNoAutoMerge{                              Text("io.ConfigViewportsNoAutoMerge");}
        if io.ConfigViewportsNoTaskBarIcon{                            Text("io.ConfigViewportsNoTaskBarIcon");}
        if io.ConfigViewportsNoDecoration{                             Text("io.ConfigViewportsNoDecoration");}
        if io.ConfigViewportsNoDefaultParent{                          Text("io.ConfigViewportsNoDefaultParent");}
        if io.ConfigDockingNoSplit{                                    Text("io.ConfigDockingNoSplit");}
        if io.ConfigDockingWithShift{                                  Text("io.ConfigDockingWithShift");}
        if io.ConfigDockingAlwaysTabBar{                               Text("io.ConfigDockingAlwaysTabBar");}
        if io.ConfigDockingTransparentPayload{                         Text("io.ConfigDockingTransparentPayload");}
        if io.ConfigMacOSXBehaviors{                                   Text("io.ConfigMacOSXBehaviors");}
        if io.ConfigInputTextCursorBlink{                              Text("io.ConfigInputTextCursorBlink");}
        if io.ConfigWindowsResizeFromEdges{                            Text("io.ConfigWindowsResizeFromEdges");}
        if io.ConfigWindowsMoveFromTitleBarOnly{                       Text("io.ConfigWindowsMoveFromTitleBarOnly");}
        if (io.ConfigMemoryCompactTimer >= 0.0)                        Text("io.ConfigMemoryCompactTimer = {}", io.ConfigMemoryCompactTimer);
        Text("io.BackendFlags: 0x{}", io.BackendFlags);
        if (io.BackendFlags & ImGuiBackendFlags_HasGamepad)             Text(" HasGamepad");
        if (io.BackendFlags & ImGuiBackendFlags_HasMouseCursors)        Text(" HasMouseCursors");
        if (io.BackendFlags & ImGuiBackendFlags_HasSetMousePos)         Text(" HasSetMousePos");
        if (io.BackendFlags & ImGuiBackendFlags_PlatformHasViewports)   Text(" PlatformHasViewports");
        if (io.BackendFlags & ImGuiBackendFlags_HasMouseHoveredViewport)Text(" HasMouseHoveredViewport");
        if (io.BackendFlags & ImGuiBackendFlags_RendererHasVtxOffset)   Text(" RendererHasVtxOffset");
        if (io.BackendFlags & ImGuiBackendFlags_RendererHasViewports)   Text(" RendererHasViewports");
        Separator();
        Text("io.Fonts: {} fonts, Flags: 0x{}, TexSize: {},{}", io.Fonts.Fonts.Size, io.Fonts.Flags, io.Fonts.TexWidth, io.Fonts.TexHeight);
        Text("io.DisplaySize: {},{}", io.DisplaySize.x, io.DisplaySize.y);
        Text("io.DisplayFramebufferScale: {},{}", io.DisplayFramebufferScale.x, io.DisplayFramebufferScale.y);
        Separator();
        Text("style.WindowPadding: {},{}", style.WindowPadding.x, style.WindowPadding.y);
        Text("style.WindowBorderSize: {}", style.WindowBorderSize);
        Text("style.FramePadding: {},{}", style.FramePadding.x, style.FramePadding.y);
        Text("style.FrameRounding: {}", style.FrameRounding);
        Text("style.FrameBorderSize: {}", style.FrameBorderSize);
        Text("style.ItemSpacing: {},{}", style.ItemSpacing.x, style.ItemSpacing.y);
        Text("style.ItemInnerSpacing: {},{}", style.ItemInnerSpacing.x, style.ItemInnerSpacing.y);

        if (copy_to_clipboard)
        {
            LogText("\n```\n");
            LogFinish();
        }
        EndChildFrame();
    }
    End();
}

//-----------------------------------------------------------------------------
// [SECTION] Style Editor / ShowStyleEditor()
//-----------------------------------------------------------------------------
// - ShowFontSelector()
// - ShowStyleSelector()
// - ShowStyleEditor()
//-----------------------------------------------------------------------------

// Forward declare ShowFontAtlas() which isn't worth putting in public API yet
namespace ImGui {  c_void ShowFontAtlas(*mut ImFontAtlas atlas); }

// Demo helper function to select among loaded fonts.
// Here we use the regular BeginCombo()/EndCombo() api which is the more flexible one.
pub unsafe fn ShowFontSelector(label: *const c_char)
{
    ImGuiIO& io = GetIO();
    *mut ImFont font_current = GetFont();
    if (BeginCombo(label, font_current->GetDebugName()))
    {
        for (let n: c_int = 0; n < io.Fonts.Fonts.Size; n++)
        {
            *mut ImFont font = io.Fonts.Fonts[n];
            PushID(font);
            if (Selectable(font->GetDebugName(), font == font_current))
                io.FontDefault = font;
            PopID();
        }
        EndCombo();
    }
    SameLine();
    HelpMarker(
        "- Load additional fonts with io.Fonts.AddFontFromFileTTF().\n"
        "- The font atlas is built when calling io.Fonts.GetTexDataAsXXXX() or io.Fonts.Build().\n"
        "- Read FAQ and docs/FONTS.md for more details.\n"
        "- If you need to add/remove fonts at runtime (e.g. for DPI change), do it before calling NewFrame().");
}

// Demo helper function to select among default colors. See ShowStyleEditor() for more advanced options.
// Here we use the simplified Combo() api that packs items into a single literal string.
// Useful for quick combo boxes where the choices are known locally.
pub unsafe fn ShowStyleSelector(label: *const c_char) -> bool
{
    static let style_idx: c_int = -1;
    if (Combo(label, &style_idx, "Dark\0Light\0Classic\0"))
    {
        switch (style_idx)
        {
        0 => StyleColorsDark(); break;
        1 => StyleColorsLight(); break;
        2 => StyleColorsClassic(); break;
        }
        return true;
    }
    return false;
}

pub unsafe fn ShowStyleEditor(re0f32: *mut ImGuiStyle)
{
    IMGUI_DEMO_MARKER("Tools/Style Editor");
    // You can pass in a reference ImGuiStyle structure to compare to, revert to and save to
    // (without a reference style pointer, we will use one compared locally as a reference)
    ImGuiStyle& style = GetStyle();
    static ImGuiStyle ref_saved_style;

    // Default to using internal storage as reference
    static let mut init: bool =  true;
    if init && ref == None {
        ref_saved_style = style;}
    init = false;
    if (ref == null_mut())
        ref = &ref_saved_style;

    PushItemWidth(GetWindowWidth() * 0.5);

    if ShowStyleSelector("Colors##Selector") {
        ref_saved_style = style;}
    ShowFontSelector("Fonts##Selector");

    // Simplified Settings (expose floating-pointer border sizes as boolean representing 0.0 or 1.0)
    if (SliderFloat("FrameRounding", &style.FrameRounding, 0.0, 12.0, "{}f"))
        style.GrabRounding = style.FrameRounding; // Make GrabRounding always the same value as FrameRounding
    { let mut border: bool =  (style.WindowBorderSize > 0.0); if (Checkbox("WindowBorder", &border)) { if style.WindowBorderSize = border { 1.0 } else { 0.0 }; } }
    SameLine();
    { let mut border: bool =  (style.FrameBorderSize > 0.0);  if (Checkbox("FrameBorder",  &border)) { if style.FrameBorderSize  = border { 1.0 } else { 0.0 }; } }
    SameLine();
    { let mut border: bool =  (style.PopupBorderSize > 0.0);  if (Checkbox("PopupBorder",  &border)) { if style.PopupBorderSize  = border { 1.0 } else { 0.0 }; } }

    // Save/Revert button
    if (Button("Save Ref"))
        *ref = ref_saved_style = style;
    SameLine();
    if (Button("Revert Ref"))
        style = *ref;
    SameLine();
    HelpMarker(
        "Save/Revert in local non-persistent storage. Default Colors definition are not affected. "
        "Use \"Export\" below to save them somewhere.");

    Separator();

    if (BeginTabBar("##tabs", ImGuiTabBarFlags_None))
    {
        if (BeginTabItem("Sizes"))
        {
            Text("Main");
            SliderFloat2("WindowPadding", (&mut c_float)&style.WindowPadding, 0.0, 20f32, "{}f");
            SliderFloat2("FramePadding", &style.FramePadding, 0.0, 20f32, "{}f");
            SliderFloat2("CellPadding", &style.CellPadding, 0.0, 20f32, "{}f");
            SliderFloat2("ItemSpacing", &style.ItemSpacing, 0.0, 20f32, "{}f");
            SliderFloat2("ItemInnerSpacing", &style.ItemInnerSpacing, 0.0, 20f32, "{}f");
            SliderFloat2("TouchExtraPadding", &style.TouchExtraPadding, 0.0, 10.0, "{}f");
            SliderFloat("IndentSpacing", &style.IndentSpacing, 0.0, 30f32, "{}f");
            SliderFloat("ScrollbarSize", &style.ScrollbarSize, 1.0, 20f32, "{}f");
            SliderFloat("GrabMinSize", &style.GrabMinSize, 1.0, 20f32, "{}f");
            Text("Borders");
            SliderFloat("WindowBorderSize", &style.WindowBorderSize, 0.0, 1.0, "{}f");
            SliderFloat("ChildBorderSize", &style.ChildBorderSize, 0.0, 1.0, "{}f");
            SliderFloat("PopupBorderSize", &style.PopupBorderSize, 0.0, 1.0, "{}f");
            SliderFloat("FrameBorderSize", &style.FrameBorderSize, 0.0, 1.0, "{}f");
            SliderFloat("TabBorderSize", &style.TabBorderSize, 0.0, 1.0, "{}f");
            Text("Rounding");
            SliderFloat("WindowRounding", &style.WindowRounding, 0.0, 12.0, "{}f");
            SliderFloat("ChildRounding", &style.ChildRounding, 0.0, 12.0, "{}f");
            SliderFloat("FrameRounding", &style.FrameRounding, 0.0, 12.0, "{}f");
            SliderFloat("PopupRounding", &style.PopupRounding, 0.0, 12.0, "{}f");
            SliderFloat("ScrollbarRounding", &style.ScrollbarRounding, 0.0, 12.0, "{}f");
            SliderFloat("GrabRounding", &style.GrabRounding, 0.0, 12.0, "{}f");
            SliderFloat("LogSliderDeadzone", &style.LogSliderDeadzone, 0.0, 12.0, "{}f");
            SliderFloat("TabRounding", &style.TabRounding, 0.0, 12.0, "{}f");
            Text("Alignment");
            SliderFloat2("WindowTitleAlign", &style.WindowTitleAlign, 0.0, 1.0, "{}");
            let window_menu_button_position: c_int = style.WindowMenuButtonPosition + 1;
            if (Combo("WindowMenuButtonPosition", (c_int*)&window_menu_button_position, "None\0Left\0Right\0"))
                style.WindowMenuButtonPosition = window_menu_button_position - 1;
            Combo("ColorButtonPosition", (c_int*)&style.ColorButtonPosition, "Left\0Right\0");
            SliderFloat2("ButtonTextAlign", &style.ButtonTextAlign, 0.0, 1.0, "{}");
            SameLine(); HelpMarker("Alignment applies when a button is larger than its text content.");
            SliderFloat2("SelectableTextAlign", &style.SelectableTextAlign, 0.0, 1.0, "{}");
            SameLine(); HelpMarker("Alignment applies when a selectable is larger than its text content.");
            Text("Safe Area Padding");
            SameLine(); HelpMarker("Adjust if you cannot see the edges of your screen (e.g. on a TV where scaling has not been configured).");
            SliderFloat2("DisplaySafeAreaPadding", &style.DisplaySafeAreaPadding, 0.0, 30f32, "{}f");
            EndTabItem();
        }

        if (BeginTabItem("Colors"))
        {
            static let output_dest: c_int = 0;
            static let mut output_only_modified: bool =  true;
            if (Button("Export"))
            {
                if output_dest == 0 {
                    LogToClipboard()(); }
                else
                    LogToTTY();
                LogText("ImVec4* colors = GetStyle().Colors;" IM_NEWLINE);
                for (let i: c_int = 0; i < ImGuiCol_COUNT; i++)
                {
                    col: &ImVec4 = style.Colors[i];
                    let mut  name: *const c_char = GetStyleColorName(i);
                    if (!output_only_modified || memcmp(&col, &ref->Colors[i], sizeof(ImVec4)) != 0)
                        LogText("colors[ImGuiCol_{}]%*s= ImVec4(%.2ff, %.2ff, %.2ff, %.2f0f32);" IM_NEWLINE,
                            name, 23 - strlen(name), "", col.x, col.y, col.z, col.w);
                }
                LogFinish();
            }
            SameLine(); SetNextItemWidth(120); Combo("##output_type", &output_dest, "To Clipboard\0To TTY\0");
            SameLine(); Checkbox("Only Modified Colors", &output_only_modified);

            static ImGuiTextFilter filter;
            filter.Draw("Filter colors", GetFontSize() * 16);

            static alpha_flags: ImGuiColorEditFlags = 0;
            if (RadioButton("Opaque", alpha_flags == ImGuiColorEditFlags_None))             { alpha_flags = ImGuiColorEditFlags_None; } SameLine();
            if (RadioButton("Alpha",  alpha_flags == ImGuiColorEditFlags_AlphaPreview))     { alpha_flags = ImGuiColorEditFlags_AlphaPreview; } SameLine();
            if (RadioButton("Both",   alpha_flags == ImGuiColorEditFlags_AlphaPreviewHal0f32)) { alpha_flags = ImGuiColorEditFlags_AlphaPreviewHalf; } SameLine();
            HelpMarker(
                "In the color list:\n"
                "Left-click on color square to open color picker,\n"
                "Right-click to open edit options menu.");

            BeginChild("##colors", ImVec2::new(0, 0), true, ImGuiWindowFlags_AlwaysVerticalScrollbar | ImGuiWindowFlags_AlwaysHorizontalScrollbar | ImGuiWindowFlags_NavFlattened);
            PushItemWidth(-160);
            for (let i: c_int = 0; i < ImGuiCol_COUNT; i++)
            {
                let mut  name: *const c_char = GetStyleColorName(i);
                if (!filter.PassFilter(name))
                    continue;
                PushID(i);
                ColorEdit4("##color", &style.Colors[i], ImGuiColorEditFlags_AlphaBar | alpha_flags);
                if (memcmp(&style.Colors[i], &ref->Colors[i], sizeof(ImVec4)) != 0)
                {
                    // Tips: in a real user application, you may want to merge and use an icon font into the main font,
                    // so instead of "Save"/"Revert" you'd use icons!
                    // Read the FAQ and docs/FONTS.md about using icon fonts. It's really easy and super convenient!
                    SameLine(0.0, style.ItemInnerSpacing.x); if (Button("Save")) { ref->Colors[i] = style.Colors[i]; }
                    SameLine(0.0, style.ItemInnerSpacing.x); if (Button("Revert")) { style.Colors[i] = ref->Colors[i]; }
                }
                SameLine(0.0, style.ItemInnerSpacing.x);
                TextUnformatted(name);
                PopID();
            }
            PopItemWidth();
            EndChild();

            EndTabItem();
        }

        if (BeginTabItem("Fonts"))
        {
            ImGuiIO& io = GetIO();
            atlas: *mut ImFontAtlas = io.Fonts;
            HelpMarker("Read FAQ and docs/FONTS.md for details on font loading.");
            ShowFontAtlas(atlas);

            // Post-baking font scaling. Note that this is NOT the nice way of scaling fonts, read below.
            // (we enforce hard clamping manually as by default DragFloat/SliderFloat allows CTRL+Click text to get out of bounds).
            let MIN_SCALE: c_float =  0.3f;
            let MAX_SCALE: c_float =  2.0;
            HelpMarker(
                "Those are old settings provided for convenience.\n"
                "However, the _correct_ way of scaling your UI is currently to reload your font at the designed size, "
                "rebuild the font atlas, and call style.ScaleAllSizes() on a reference ImGuiStyle structure.\n"
                "Using those settings here will give you poor quality results.");
            static let window_scale: c_float =  1.0;
            PushItemWidth(GetFontSize() * 8);
            if (DragFloat("window scale", &window_scale, 0.005f, MIN_SCALE, MAX_SCALE, "{}", ImGuiSliderFlags_AlwaysClamp)) // Scale only this window
                SetWindowFontScale(window_scale);
            DragFloat("global scale", &io.FontGlobalScale, 0.005f, MIN_SCALE, MAX_SCALE, "{}", ImGuiSliderFlags_AlwaysClamp); // Scale everything
            PopItemWidth();

            EndTabItem();
        }

        if (BeginTabItem("Rendering"))
        {
            Checkbox("Anti-aliased lines", &style.AntiAliasedLines);
            SameLine();
            HelpMarker("When disabling anti-aliasing lines, you'll probably want to disable borders in your style as well.");

            Checkbox("Anti-aliased lines use texture", &style.AntiAliasedLinesUseTex);
            SameLine();
            HelpMarker("Faster lines using texture data. Require backend to render with bilinear filtering (not point/nearest filtering).");

            Checkbox("Anti-aliased fill", &style.AntiAliasedFill);
            PushItemWidth(GetFontSize() * 8);
            DragFloat("Curve Tessellation Tolerance", &style.CurveTessellationTol, 0.02f, 0.1.0, 10.0, "{}");
            if style.CurveTessellationTol < 0.100{ style.CurveTessellationTol = 0.1.0;}

            // When editing the "Circle Segment Max Error" value, draw a preview of its effect on auto-tessellated circles.
            DragFloat("Circle Tessellation Max Error", &style.CircleTessellationMaxError , 0.005f, 0.1.0, 5f32, "{}", ImGuiSliderFlags_AlwaysClamp);
            if (IsItemActive())
            {
                SetNextWindowPos(GetCursorScreenPos());
                BeginTooltip();
                TextUnformatted("(R = radius, N = number of segments)");
                Spacing();
                let mut  draw_list: *mut ImDrawList =  GetWindowDrawList();
                let min_widget_width: c_float =  CalcTextSize("N: MMM\nR: MMM").x;
                for (let n: c_int = 0; n < 8; n++)
                {
                    let RAD_MIN: c_float =  5f32;
                    let RAD_MAX: c_float =  70f32;
                    let rad: c_float =  RAD_MIN + (RAD_MAX - RAD_MIN) * n / (8.0 - 1.0);

                    BeginGroup();

                    Text("R: %.f\nN: {}", rad, draw_list._CalcCircleAutoSegmentCount(rad));

                    let canvas_width: c_float =  IM_MAX(min_widget_width, rad * 2.0);
                    offset_x: c_float     = floorf(canvas_width * 0.5);
                    offset_y: c_float     = floorf(RAD_MAX);

                    let p1: ImVec2 = GetCursorScreenPos();
                    draw_list.AddCircle(ImVec2::new(p1.x + offset_x, p1.y + offset_y), rad, GetColorU32(ImGuiCol_Text, 0.0));
                    Dummy(ImVec2::new(canvas_width, RAD_MAX * 2));

                    /*
                    let p2: ImVec2 = GetCursorScreenPos();
                    draw_list.AddCircleFilled(ImVec2::new(p2.x + offset_x, p2.y + offset_y), rad, GetColorU32(ImGuiCol_Text, 0.0));
                    Dummy(ImVec2::new(canvas_width, RAD_MAX * 2));
                    */

                    EndGroup();
                    SameLine();
                }
                EndTooltip();
            }
            SameLine();
            HelpMarker("When drawing circle primitives with \"num_segments == 0\" tesselation will be calculated automatically.");

            DragFloat("Global Alpha", &style.Alpha, 0.005f, 0.20, 1.0, "{}"); // Not exposing zero here so user doesn't "lose" the UI (zero alpha clips all widgets). But application code could have a toggle to switch between zero and non-zero.
            DragFloat("Disabled Alpha", &style.DisabledAlpha, 0.005f, 0.0, 1.0, "{}"); SameLine(); HelpMarker("Additional alpha multiplier for disabled items (multiply over current value of Alpha).");
            PopItemWidth();

            EndTabItem();
        }

        EndTabBar();
    }

    PopItemWidth();
}

//-----------------------------------------------------------------------------
// [SECTION] Example App: Main Menu Bar / ShowExampleAppMainMenuBar()
//-----------------------------------------------------------------------------
// - ShowExampleAppMainMenuBar()
// - ShowExampleMenuFile()
//-----------------------------------------------------------------------------

// Demonstrate creating a "main" fullscreen menu bar and populating it.
// Note the difference between BeginMainMenuBar() and BeginMenuBar():
// - BeginMenuBar() = menu-bar inside current window (which needs the ImGuiWindowFlags_MenuBar flag!)
// - BeginMainMenuBar() = helper to create menu-bar-sized window at the top of the main viewport + call BeginMenuBar() into it.
pub unsafe fn ShowExampleAppMainMenuBar()
{
    if (BeginMainMenuBar())
    {
        if (BeginMenu("File"))
        {
            ShowExampleMenuFile();
            EndMenu();
        }
        if (BeginMenu("Edit"))
        {
            if (MenuItem("Undo", "CTRL+Z")) {}
            if (MenuItem("Redo", "CTRL+Y", false, false)) {}  // Disabled item
            Separator();
            if (MenuItem("Cut", "CTRL+X")) {}
            if (MenuItem("Copy", "CTRL+C")) {}
            if (MenuItem("Paste", "CTRL+V")) {}
            EndMenu();
        }
        EndMainMenuBar();
    }
}

// Note that shortcuts are currently provided for display only
// (future version will add explicit flags to BeginMenu() to request processing shortcuts)
pub unsafe fn ShowExampleMenuFile()
{
    IMGUI_DEMO_MARKER("Examples/Menu");
    MenuItem("(demo menu)", None, false, false);
    if (MenuItem("New")) {}
    if (MenuItem("Open", "Ctrl+O")) {}
    if (BeginMenu("Open Recent"))
    {
        MenuItem("fish_hat.c");
        MenuItem("fish_hat.inl");
        MenuItem("fish_hat.h");
        if (BeginMenu("More.."))
        {
            MenuItem("Hello");
            MenuItem("Sailor");
            if (BeginMenu("Recurse.."))
            {
                ShowExampleMenuFile();
                EndMenu();
            }
            EndMenu();
        }
        EndMenu();
    }
    if (MenuItem("Save", "Ctrl+S")) {}
    if (MenuItem("Save As..")) {}

    Separator();
    IMGUI_DEMO_MARKER("Examples/Menu/Options");
    if (BeginMenu("Options"))
    {
        static let mut enabled: bool =  true;
        MenuItem("Enabled", "", &enabled);
        BeginChild("child", ImVec2::new(0, 60), true);
        for (let i: c_int = 0; i < 10; i++)
            Text("Scrolling Text {}", i);
        EndChild();
        static let f: c_float =  0.5;
        static let n: c_int = 0;
        SliderFloat("Value", &f, 0.0, 1.0);
        InputFloat("Input", &f, 0.1.0);
        Combo("Combo", &n, "Yes\0No\0Maybe\0\0");
        EndMenu();
    }

    IMGUI_DEMO_MARKER("Examples/Menu/Colors");
    if (BeginMenu("Colors"))
    {
        let sz: c_float =  GetTextLineHeight();
        for (let i: c_int = 0; i < ImGuiCol_COUNT; i++)
        {
            let mut  name: *const c_char = GetStyleColorName((ImGuiCol)i);
            let p: ImVec2 = GetCursorScreenPos();
            GetWindowDrawList().AddRectFilled(p, ImVec2::new(p.x + sz, p.y + sz), GetColorU32((ImGuiCol)i));
            Dummy(ImVec2::new(sz, sz));
            SameLine();
            MenuItem(name);
        }
        EndMenu();
    }

    // Here we demonstrate appending again to the "Options" menu (which we already created above)
    // Of course in this demo it is a little bit silly that this function calls BeginMenu("Options") twice.
    // In a real code-base using it would make senses to use this feature from very different code locations.
    if (BeginMenu("Options")) // <-- Append!
    {
        IMGUI_DEMO_MARKER("Examples/Menu/Append to an existing menu");
        static let mut b: bool =  true;
        Checkbox("SomeOption", &b);
        EndMenu();
    }

    if (BeginMenu("Disabled", false)) // Disabled
    {
        // IM_ASSERT(0);
    }
    if (MenuItem("Checked", None, true)) {}
    if (MenuItem("Quit", "Alt+F4")) {}
}

//-----------------------------------------------------------------------------
// [SECTION] Example App: Debug Console / ShowExampleAppConsole()
//-----------------------------------------------------------------------------

// Demonstrate creating a simple console window, with scrolling, filtering, completion and history.
// For the console example, we are using a more C++ like approach of declaring a class to hold both data and functions.
struct ExampleAppConsole
{
    InputBuf: [c_char;256];
    Vec<char*>       Items;
    Vec<*const char> Commands;
    Vec<char*>       History;
    c_int                   HistoryPos;    // -1: new line, 0..History.Size-1 browsing history.
    ImGuiTextFilter       Filter;
    bool                  AutoScroll;
    bool                  ScrollToBottom;

    ExampleAppConsole()
    {
        IMGUI_DEMO_MARKER("Examples/Console");
        ClearLog();
        memset(InputBuf, 0, sizeof(InputBu0f32));
        HistoryPos = -1;

        // "CLASSIFY" is here to provide the test case where "C"+[tab] completes to "CL" and display multiple matches.
        Commands.push("HELP");
        Commands.push("HISTORY");
        Commands.push("CLEAR");
        Commands.push("CLASSIFY");
        AutoScroll = true;
        ScrollToBottom = false;
        AddLog("Welcome to Dear ImGui!");
    }
    !ExampleAppConsole()
    {
        ClearLog();
        for (let i: c_int = 0; i < History.Size; i++)
            free(History[i]);
    }

    // Portable helpers
    static c_int   Stricmp(s1: *const c_char, s2: *const c_char)         { let mut d: c_int = 0; while ((d = toupper(*s2) - toupper(*s1)) == 0 && *s1) { s1+= 1; s2+= 1; } return d; }
    static c_int   Strnicmp(s1: *const c_char, s2: *const c_char, n: c_int) { let d: c_int = 0; while (n > 0 && (d = toupper(*s2) - toupper(*s1)) == 0 && *s1) { s1+= 1; s2+= 1; n-= 1; } return d; }
    static char* Strdup(s: *const c_char)                           { IM_ASSERT(s); len: size_t = strlen(s) + 1; buf: *mut c_void = malloc(len); IM_ASSERT(buf); return memcpy(buf, (*const c_void)s, len); }
    static c_void  Strtrim(char* s)                                { char* str_end = s + strlen(s); while (str_end > s && str_end[-1] == ' ') str_end-= 1; *str_end = 0; }

    c_void    ClearLog()
    {
        for (let i: c_int = 0; i < Items.Size; i++)
            free(Items[i]);
        Items.clear();
    }

    c_void    AddLog(fmt: *const c_char, ...) IM_FMTARGS(2)
    {
        // FIXME-OPT
        buf: [c_char;1024];
        va_list args;
        va_start(args, fmt);
        vsnprintf(buf, buf.len(), fmt, args);
        buf[buf.len()-1] = 0;
        va_end(args);
        Items.push(Strdup(buf));
    }

    c_void    Draw(title: *const c_char,p_open: *mut bool)
    {
        SetNextWindowSize(ImVec2::new(520, 600), ImGuiCond_FirstUseEver);
        if (!Begin(title, p_open))
        {
            End();
            return;
        }

        // As a specific feature guaranteed by the library, after calling Begin() the last Item represent the title bar.
        // So e.g. IsItemHovered() will return true when hovering the title bar.
        // Here we create a context menu only available from the title bar.
        if (BeginPopupContextItem())
        {
            if (MenuItem("Close Console"))
                *p_open = false;
            EndPopup();
        }

        TextWrapped(
            "This example implements a console with basic coloring, completion (TAB key) and history (Up/Down keys). A more elaborate "
            "implementation may want to store entries along with extra data such as timestamp, emitter, etc.");
        TextWrapped("Enter 'HELP' for help.");

        // TODO: display items starting from the bottom

        if (SmallButton("Add Debug Text"))  { AddLog("{} some text", Items.Size); AddLog("some more text"); AddLog("display very important message here!"); }
        SameLine();
        if (SmallButton("Add Debug Error")) { AddLog("[error] something went wrong"); }
        SameLine();
        if (SmallButton("Clear"))           { ClearLog(); }
        SameLine();
        let mut copy_to_clipboard: bool =  SmallButton("Copy");
        //static float t = 0.0; if (GetTime() - t > 0.020f32) { t = GetTime(); AddLog("Spam {}", t); }

        Separator();

        // Options menu
        if (BeginPopup("Options"))
        {
            Checkbox("Auto-scroll", &AutoScroll);
            EndPopup();
        }

        // Options, Filter
        if (Button("Options"))
            OpenPopup("Options");
        SameLine();
        Filter.Draw("Filter (\"incl,-excl\") (\"error\")", 180);
        Separator();

        // Reserve enough left-over height for 1 separator + 1 input text
        let footer_height_to_reserve: c_float =  GetStyle().ItemSpacing.y + GetFrameHeightWithSpacing();
        BeginChild("ScrollingRegion", ImVec2::new(0, -footer_height_to_reserve), false, ImGuiWindowFlags_HorizontalScrollbar);
        if (BeginPopupContextWindow())
        {
            if (Selectable("Clear")) ClearLog();
            EndPopup();
        }

        // Display every line as a separate entry so we can change their color or add custom widgets.
        // If you only want raw text you can use TextUnformatted(log.begin(), log.end());
        // NB- if you have thousands of entries this approach may be too inefficient and may require user-side clipping
        // to only process visible items. The clipper will automatically measure the height of your first item and then
        // "seek" to display only items in the visible area.
        // To use the clipper we can replace your standard loop:
        //      for (int i = 0; i < Items.Size; i++)
        //   With:
        //      ImGuiListClipper clipper;
        //      clipper.Begin(Items.Size);
        //      while (clipper.Step())
        //         for (int i = clipper.DisplayStart; i < clipper.DisplayEnd; i++)
        // - That your items are evenly spaced (same height)
        // - That you have cheap random access to your elements (you can access them given their index,
        //   without processing all the ones before)
        // You cannot this code as-is if a filter is active because it breaks the 'cheap random-access' property.
        // We would need random-access on the post-filtered list.
        // A typical application wanting coarse clipping and filtering may want to pre-compute an array of indices
        // or offsets of items that passed the filtering test, recomputing this array when user changes the filter,
        // and appending newly elements as they are inserted. This is left as a task to the user until we can manage
        // to improve this example code!
        // If your items are of variable height:
        // - Split them into same height items would be simpler and facilitate random-seeking into your list.
        // - Consider using manual call to IsRectVisible() and skipping extraneous decoration from your items.
        PushStyleVar(ImGuiStyleVar_ItemSpacing, ImVec2::new(4, 1)); // Tighten spacing
        if copy_to_clipboard {
            LogToClipboard(); }
        for (let i: c_int = 0; i < Items.Size; i++)
        {
            let mut  item: *const c_char = Items[i];
            if (!Filter.PassFilter(item))
                continue;

            // Normally you would store more information in your item than just a string.
            // (e.g. make Items[] an array of structure, store color/type etc.)
            color: ImVec4;
            let mut has_color: bool =  false;
            if (strstr(item, "[error]"))          { color = ImVec4(1.0, 0.4f, 0.4f, 1.0); has_color = true; }
            else if (strncmp(item, "# ", 2) == 0) { color = ImVec4(1.0, 0.8f, 0.6f, 1.0); has_color = true; }
            if (has_color)
                PushStyleColor(ImGuiCol_Text, color);
            TextUnformatted(item);
            if has_color {
                PopStyleColor(); }
        }
        if copy_to_clipboard {
            LogFinish(); }

        if (ScrollToBottom || (AutoScroll && GetScrollY() >= GetScrollMaxY()))
            SetScrollHereY(1.0);
        ScrollToBottom = false;

        PopStyleVar();
        EndChild();
        Separator();

        // Command-line
        let mut reclaim_focus: bool =  false;
        input_text_flags: ImGuiInputTextFlags = ImGuiInputTextFlags_EnterReturnsTrue | ImGuiInputTextFlags_CallbackCompletion | ImGuiInputTextFlags_CallbackHistory;
        if (InputText("Input", InputBuf, InputBu0f32.len(), input_text_flags, &TextEditCallbackStub, this))
        {
            char* s = InputBuf;
            Strtrim(s);
            if (s[0])
                ExecCommand(s);
            strcpy(s, "");
            reclaim_focus = true;
        }

        // Auto-focus on window apparition
        SetItemDefaultFocus();
        if (reclaim_focus)
            SetKeyboardFocusHere(-1); // Auto focus previous widget

        End();
    }

    c_void    ExecCommand(command_line: *const c_char)
    {
        AddLog("# {}\n", command_line);

        // Insert into history. First find match and delete it so it can be pushed to the back.
        // This isn't trying to be smart or optimal.
        HistoryPos = -1;
        for (let i: c_int = History.Size - 1; i >= 0; i--)
            if (Stricmp(History[i], command_line) == 0)
            {
                free(History[i]);
                History.erase(History.begin() + i);
                break;
            }
        History.push(Strdup(command_line));

        // Process command
        if (Stricmp(command_line, "CLEAR") == 0)
        {
            ClearLog();
        }
        else if (Stricmp(command_line, "HELP") == 0)
        {
            AddLog("Commands:");
            for (let i: c_int = 0; i < Commands.Size; i++)
                AddLog("- {}", Commands[i]);
        }
        else if (Stricmp(command_line, "HISTORY") == 0)
        {
            let first: c_int = History.Size - 10;
            for (let i: c_int = if first > 0 { first }else {0}; i < History.Size; i++)
                AddLog("%3d: {}\n", i, History[i]);
        }
        else
        {
            AddLog("Unknown command: '{}'\n", command_line);
        }

        // On command input, we scroll to bottom even if AutoScroll==false
        ScrollToBottom = true;
    }

    // In C++11 you'd be better off using lambdas for this sort of forwarding callbacks
    pub fn TextEditCallbackStub(ImGuiInputTextCallbackData* data) -> c_int
    {
        ExampleAppConsole* console = (ExampleAppConsole*)data.UserData;
        return console.TextEditCallback(data);
    }

    c_int     TextEditCallback(ImGuiInputTextCallbackData* data)
    {
        //AddLog("cursor: {}, selection: {}-{}", data->CursorPos, data->SelectionStart, data->SelectionEnd);
        switch (data.EventFlag)
        {
        ImGuiInputTextFlags_CallbackCompletion =>
            {
                // Example of TEXT COMPLETION

                // Locate beginning of current word
                let mut  word_end: *const c_char = data.Buf + data.CursorPos;
                let mut  word_start: *const c_char = word_end;
                while (word_start > data.Bu0f32)
                {
                    const  c: c_char = word_start[-1];
                    if (c == ' ' || c == '\t' || c == ',' || c == ';')
                        break;
                    word_start-= 1;
                }

                // Build a list of candidates
                Vec<*const char> candidates;
                for (let i: c_int = 0; i < Commands.Size; i++)
                    if (Strnicmp(Commands[i], word_start, (word_end - word_start)) == 0)
                        candidates.push(Commands[i]);

                if (candidates.Size == 0)
                {
                    // No match
                    AddLog("No match for \"%.*s\"!\n", (word_end - word_start), word_start);
                }
                else if (candidates.Size == 1)
                {
                    // Single match. Delete the beginning of the word and replace it entirely so we've got nice casing.
                    data.DeleteChars((word_start - data.Bu0f32), (word_end - word_start));
                    data.InsertChars(data.CursorPos, candidates[0]);
                    data.InsertChars(data.CursorPos, " ");
                }
                else
                {
                    // Multiple matches. Complete as much as we can..
                    // So inputing "C"+Tab will complete to "CL" then display "CLEAR" and "CLASSIFY" as matches.
                    let match_len: c_int = (word_end - word_start);
                    loop
                    {
                        let c: c_int = 0;
                        let mut all_candidates_matches: bool =  true;
                        for (let i: c_int = 0; i < candidates.Size && all_candidates_matches; i++)
                            if (i == 0)
                                c = toupper(candidates[i][match_len]);
                            else if c == 0 || c != toupper(candidates[i][match_len]) {
                                all_candidates_matches = false;}
                        if (!all_candidates_matches)
                            break;
                        match_len+= 1;
                    }

                    if (match_len > 0)
                    {
                        data.DeleteChars((word_start - data.Bu0f32), (word_end - word_start));
                        data.InsertChars(data.CursorPos, candidates[0], candidates[0] + match_len);
                    }

                    // List matches
                    AddLog("Possible matches:\n");
                    for (let i: c_int = 0; i < candidates.Size; i++)
                        AddLog("- {}\n", candidates[i]);
                }

                break;
            }
        ImGuiInputTextFlags_CallbackHistory =>
            {
                // Example of HISTORY
                let prev_history_pos: c_int = HistoryPos;
                if (data.EventKey == ImGuiKey_UpArrow)
                {
                    if (HistoryPos == -1)
                        HistoryPos = History.Size - 1;
                    else if (HistoryPos > 0)
                        HistoryPos-= 1;
                }
                else if (data.EventKey == ImGuiKey_DownArrow)
                {
                    if (HistoryPos != -1)
                        if (++HistoryPos >= History.Size)
                            HistoryPos = -1;
                }

                // A better implementation would preserve the data on the current input line along with cursor position.
                if (prev_history_pos != HistoryPos)
                {
                    let mut  history_str: *const c_char = if HistoryPos >= 0 { History[HistoryPos]} else { ""};
                    data.DeleteChars(0, data.BufTextLen);
                    data.InsertChars(0, history_str);
                }
            }
        }
        return 0;
    }
};

pub unsafe fn ShowExampleAppConsole(bool* p_open)
{
    static ExampleAppConsole console;
    console.Draw("Example: Console", p_open);
}

//-----------------------------------------------------------------------------
// [SECTION] Example App: Debug Log / ShowExampleAppLog()
//-----------------------------------------------------------------------------

// Usage:
//  static ExampleAppLog my_log;
//  my_log.AddLog("Hello {} world\n", 123);
//  my_log.Draw("title");
struct ExampleAppLog
{
    ImGuiTextBuffer     Buf;
    ImGuiTextFilter     Filter;
    Vec<c_int>       LineOffsets; // Index to lines offset. We maintain this with AddLog() calls.
    bool                AutoScroll;  // Keep scrolling if already at the bottom.

    ExampleAppLog()
    {
        AutoScroll = true;
        Clear();
    }

    c_void    Clear()
    {
        Buf.clear();
        LineOffsets.clear();
        LineOffsets.push(0);
    }

    c_void    AddLog(fmt: *const c_char, ...) IM_FMTARGS(2)
    {
        let old_size: c_int = Buf.size();
        va_list args;
        va_start(args, fmt);
        Buf.appendfv(fmt, args);
        va_end(args);
        for (let new_size: c_int = Buf.size(); old_size < new_size; old_size++)
            if (Buf[old_size] == '\n')
                LineOffsets.push(old_size + 1);
    }

    c_void    Draw(title: *const c_char,p_open: *mut bool = null_mut())
    {
        if (!Begin(title, p_open))
        {
            End();
            return;
        }

        // Options menu
        if (BeginPopup("Options"))
        {
            Checkbox("Auto-scroll", &AutoScroll);
            EndPopup();
        }

        // Main window
        if (Button("Options"))
            OpenPopup("Options");
        SameLine();
        let mut clear: bool =  Button("Clear");
        SameLine();
        let mut copy: bool =  Button("Copy");
        SameLine();
        Filter.Draw("Filter", -100);

        Separator();
        BeginChild("scrolling", ImVec2::new(0, 0), false, ImGuiWindowFlags_HorizontalScrollbar);

        if clear {
            Clear(); }
        if copy {
            LogToClipboard(); }

        PushStyleVar(ImGuiStyleVar_ItemSpacing, ImVec2::new(0, 0));
        let mut  buf: *const c_char = Buf.begin();
        let mut  buf_end: *const c_char = Buf.end();
        if (Filter.IsActive())
        {
            // In this example we don't use the clipper when Filter is enabled.
            // This is because we don't have a random access on the result on our filter.
            // A real application processing logs with ten of thousands of entries may want to store the result of
            // search/filter.. especially if the filtering function is not trivial (e.g. reg-exp).
            for (let line_no: c_int = 0; line_no < LineOffsets.Size; line_no++)
            {
                let mut  line_start: *const c_char = buf + LineOffsets[line_no];
                let mut  line_end: *const c_char = if (line_no + 1 < LineOffsets.Size) { (buf + LineOffsets[line_no + 1] - 1)} else {buf_end};
                if (Filter.PassFilter(line_start, line_end))
                    TextUnformatted(line_start, line_end);
            }
        }
        else
        {
            // The simplest and easy way to display the entire buffer:
            //   TextUnformatted(buf_begin, buf_end);
            // And it'll just work. TextUnformatted() has specialization for large blob of text and will fast-forward
            // to skip non-visible lines. Here we instead demonstrate using the clipper to only process lines that are
            // within the visible area.
            // If you have tens of thousands of items and their processing cost is non-negligible, coarse clipping them
            // on your side is recommended. Using ImGuiListClipper requires
            // - A) random access into your data
            // - B) items all being the  same height,
            // both of which we can handle since we an array pointing to the beginning of each line of text.
            // When using the filter (in the block of code above) we don't have random access into the data to display
            // anymore, which is why we don't use the clipper. Storing or skimming through the search result would make
            // it possible (and would be recommended if you want to search through tens of thousands of entries).
            ImGuiListClipper clipper;
            clipper.Begin(LineOffsets.Size);
            while (clipper.Step())
            {
                for (let line_no: c_int = clipper.DisplayStart; line_no < clipper.DisplayEnd; line_no++)
                {
                    let mut  line_start: *const c_char = buf + LineOffsets[line_no];
                    let mut  line_end: *const c_char = if (line_no + 1 < LineOffsets.Size) { (buf + LineOffsets[line_no + 1] - 1)} else {buf_end};
                    TextUnformatted(line_start, line_end);
                }
            }
            clipper.End();
        }
        PopStyleVar();

        if (AutoScroll && GetScrollY() >= GetScrollMaxY())
            SetScrollHereY(1.0);

        EndChild();
        End();
    }
};

// Demonstrate creating a simple log window with basic filtering.
pub unsafe fn ShowExampleAppLog(bool* p_open)
{
    static ExampleAppLog log;

    // For the demo: add a debug button _BEFORE_ the normal log window contents
    // We take advantage of a rarely used feature: multiple calls to Begin()/End() are appending to the _same_ window.
    // Most of the contents of the window will be added by the log.Draw() call.
    SetNextWindowSize(ImVec2::new(500, 400), ImGuiCond_FirstUseEver);
    Begin("Example: Log", p_open);
    IMGUI_DEMO_MARKER("Examples/Log");
    if (SmallButton("[Debug] Add 5 entries"))
    {
        static let counter: c_int = 0;
        *const categories: [c_char;3] = { "info", "warn", "error" };
        words: *const c_char[] = { "Bumfuzzled", "Cattywampus", "Snickersnee", "Abibliophobia", "Absquatulate", "Nincompoop", "Pauciloquent" };
        for (let n: c_int = 0; n < 5; n++)
        {
            let mut  category: *const c_char = categories[counter % categories.len()];
            let mut  word: *const c_char = words[counter % words.len()];
            log.AddLog("[%05d] [{}] Hello, current time is {}, here's a word: '{}'\n",
                GetFrameCount(), category, GetTime(), word);
            counter+= 1;
        }
    }
    End();

    // Actually call in the regular Log helper (which will Begin() into the same window as we just did)
    log.Draw("Example: Log", p_open);
}

//-----------------------------------------------------------------------------
// [SECTION] Example App: Simple Layout / ShowExampleAppLayout()
//-----------------------------------------------------------------------------

// Demonstrate create a window with multiple child windows.
pub unsafe fn ShowExampleAppLayout(bool* p_open)
{
    SetNextWindowSize(ImVec2::new(500, 440), ImGuiCond_FirstUseEver);
    if (Begin("Example: Simple layout", p_open, ImGuiWindowFlags_MenuBar))
    {
        IMGUI_DEMO_MARKER("Examples/Simple layout");
        if (BeginMenuBar())
        {
            if (BeginMenu("File"))
            {
                if (MenuItem("Close")) *p_open = false;
                EndMenu();
            }
            EndMenuBar();
        }

        // Left
        static let selected: c_int = 0;
        {
            BeginChild("left pane", ImVec2::new(150, 0), true);
            for (let i: c_int = 0; i < 100; i++)
            {
                // FIXME: Good candidate to use ImGuiSelectableFlags_SelectOnNav
                label: [c_char;128];
                sprintf(label, "MyObject {}", i);
                if Selectable(label, selected == i) {
                    selected = i;}
            }
            EndChild();
        }
        SameLine();

        // Right
        {
            BeginGroup();
            BeginChild("item view", ImVec2::new(0, -GetFrameHeightWithSpacing())); // Leave room for 1 line below us
            Text("MyObject: {}", selected);
            Separator();
            if (BeginTabBar("##Tabs", ImGuiTabBarFlags_None))
            {
                if (BeginTabItem("Description"))
                {
                    TextWrapped("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ");
                    EndTabItem();
                }
                if (BeginTabItem("Details"))
                {
                    Text("ID: 0123456789");
                    EndTabItem();
                }
                EndTabBar();
            }
            EndChild();
            if (Button("Revert")) {}
            SameLine();
            if (Button("Save")) {}
            EndGroup();
        }
    }
    End();
}

//-----------------------------------------------------------------------------
// [SECTION] Example App: Property Editor / ShowExampleAppPropertyEditor()
//-----------------------------------------------------------------------------

pub unsafe fn ShowPlaceholderObject(prefix: *const c_char, uid: c_int)
{
    // Use object uid as identifier. Most commonly you could also use the object pointer as a base ID.
    PushID(uid);

    // Text and Tree nodes are less high than framed widgets, using AlignTextToFramePadding() we add vertical spacing to make the tree lines equal high.
    TableNextRow();
    TableSetColumnIndex(0);
    AlignTextToFramePadding();
    let mut node_open: bool =  TreeNode("Object", "{}_%u", prefix, uid);
    TableSetColumnIndex(1);
    Text("my sailor is rich");

    if (node_open)
    {
        staticplaceholder_members: c_float[8] = { 0.0, 0.0, 1.0, 3.1416f, 100, 999.0 };
        for (let i: c_int = 0; i < 8; i++)
        {
            PushID(i); // Use field index as identifier.
            if (i < 2)
            {
                ShowPlaceholderObject("Child", 424242);
            }
            else
            {
                // Here we use a TreeNode to highlight on hover (we could use e.g. Selectable as well)
                TableNextRow();
                TableSetColumnIndex(0);
                AlignTextToFramePadding();
                flags: ImGuiTreeNodeFlags = ImGuiTreeNodeFlags_Leaf | ImGuiTreeNodeFlags_NoTreePushOnOpen | ImGuiTreeNodeFlags_Bullet;
                TreeNodeEx("Field", flags, "Field_{}", i);

                TableSetColumnIndex(1);
                SetNextItemWidth(-FLT_MIN);
                if (i >= 5)
                    InputFloat("##value", &placeholder_members[i], 1.0);
                else
                    DragFloat("##value", &placeholder_members[i], 0.010f32);
                NextColumn();
            }
            PopID();
        }
        TreePop();
    }
    PopID();
}

// Demonstrate create a simple property editor.
pub unsafe fn ShowExampleAppPropertyEditor(bool* p_open)
{
    SetNextWindowSize(ImVec2::new(430, 450), ImGuiCond_FirstUseEver);
    if (!Begin("Example: Property editor", p_open))
    {
        End();
        return;
    }
    IMGUI_DEMO_MARKER("Examples/Property Editor");

    HelpMarker(
        "This example shows how you may implement a property editor using two columns.\n"
        "All objects/fields data are dummies here.\n"
        "Remember that in many simple cases, you can use SameLine(xxx) to position\n"
        "your cursor horizontally instead of using the Columns() API.");

    PushStyleVar(ImGuiStyleVar_FramePadding, ImVec2::new(2, 2));
    if (BeginTable("split", 2, ImGuiTableFlags_BordersOuter | ImGuiTableFlags_Resizable))
    {
        // Iterate placeholder objects (all the same data)
        for (let obj_i: c_int = 0; obj_i < 4; obj_i++)
        {
            ShowPlaceholderObject("Object", obj_i);
            //Separator();
        }
        EndTable();
    }
    PopStyleVar();
    End();
}

//-----------------------------------------------------------------------------
// [SECTION] Example App: Long Text / ShowExampleAppLongText()
//-----------------------------------------------------------------------------

// Demonstrate/test rendering huge amount of text, and the incidence of clipping.
pub unsafe fn ShowExampleAppLongText(bool* p_open)
{
    SetNextWindowSize(ImVec2::new(520, 600), ImGuiCond_FirstUseEver);
    if (!Begin("Example: Long text display", p_open))
    {
        End();
        return;
    }
    IMGUI_DEMO_MARKER("Examples/Long text display");

    static let test_type: c_int = 0;
    static ImGuiTextBuffer log;
    static let lines: c_int = 0;
    Text("Printing unusually long amount of text.");
    Combo("Test type", &test_type,
        "Single call to TextUnformatted()\0"
        "Multiple calls to Text(), clipped\0"
        "Multiple calls to Text(), not clipped (slow)\0");
    Text("Buffer contents: {} lines, {} bytes", lines, log.size());
    if (Button("Clear")) { log.clear(); lines = 0; }
    SameLine();
    if (Button("Add 1000 lines"))
    {
        for (let i: c_int = 0; i < 1000; i++)
            log.appendf("%i The quick brown fox jumps over the lazy dog\n", lines + i);
        lines += 1000;
    }
    BeginChild("Log");
    switch (test_type)
    {
    0 =>
        // Single call to TextUnformatted() with a big buffer
        TextUnformatted(log.begin(), log.end());
        break;
    1 =>
        {
            // Multiple calls to Text(), manually coarsely clipped - demonstrate how to use the ImGuiListClipper helper.
            PushStyleVar(ImGuiStyleVar_ItemSpacing, ImVec2::new(0, 0));
            ImGuiListClipper clipper;
            clipper.Begin(lines);
            while (clipper.Step())
                for (let i: c_int = clipper.DisplayStart; i < clipper.DisplayEnd; i++)
                    Text("%i The quick brown fox jumps over the lazy dog", i);
            PopStyleVar();
            break;
        }
    2 =>
        // Multiple calls to Text(), not clipped (slow)
        PushStyleVar(ImGuiStyleVar_ItemSpacing, ImVec2::new(0, 0));
        for (let i: c_int = 0; i < lines; i++)
            Text("%i The quick brown fox jumps over the lazy dog", i);
        PopStyleVar();
        break;
    }
    EndChild();
    End();
}

//-----------------------------------------------------------------------------
// [SECTION] Example App: Auto Resize / ShowExampleAppAutoResize()
//-----------------------------------------------------------------------------

// Demonstrate creating a window which gets auto-resized according to its content.
pub unsafe fn ShowExampleAppAutoResize(bool* p_open)
{
    if (!Begin("Example: Auto-resizing window", p_open, ImGuiWindowFlags_AlwaysAutoResize))
    {
        End();
        return;
    }
    IMGUI_DEMO_MARKER("Examples/Auto-resizing window");

    static let lines: c_int = 10;
    TextUnformatted(
        "Window will resize every-frame to the size of its content.\n"
        "Note that you probably don't want to query the window size to\n"
        "output your content because that would create a feedback loop.");
    SliderInt("Number of lines", &lines, 1, 20);
    for (let i: c_int = 0; i < lines; i++)
        Text("%*sThis is line {}", i * 4, "", i); // Pad with space to extend size horizontally
    End();
}

//-----------------------------------------------------------------------------
// [SECTION] Example App: Constrained Resize / ShowExampleAppConstrainedResize()
//-----------------------------------------------------------------------------

// Demonstrate creating a window with custom resize constraints.
// Note that size constraints currently don't work on a docked window (when in 'docking' branch)
pub unsafe fn ShowExampleAppConstrainedResize(bool* p_open)
{
    struct CustomConstraints
    {
        // Helper functions to demonstrate programmatic constraints
        // FIXME: This doesn't take account of decoration size (e.g. title bar), library should make this easier.
        static c_void AspectRatio(ImGuiSizeCallbackData* data)    { let aspect_ratio: c_float =  *data.UserData; data.DesiredSize.x = IM_MAX(data.CurrentSize.x, data.CurrentSize.y); data.DesiredSize.y = (data.DesiredSize.x / aspect_ratio); }
        static c_void Square(ImGuiSizeCallbackData* data)         { data.DesiredSize.x = data.DesiredSize.y = IM_MAX(data.CurrentSize.x, data.CurrentSize.y); }
        static c_void Step(ImGuiSizeCallbackData* data)           { let step: c_float =  *data.UserData; data.DesiredSize = ImVec2::new((data.CurrentSize.x / step + 0.5) * step, (data.CurrentSize.y / step + 0.5) * step); }
    };

    test_desc: *const c_char[] =
    {
        "Between 100x100 and 500x500",
        "At least 100x100",
        "Resize vertical only",
        "Resize horizontal only",
        "Width Between 400 and 500",
        "Custom: Aspect Ratio 16:9",
        "Custom: Always Square",
        "Custom: Fixed Steps (100)",
    };

    // Options
    static let mut auto_resize: bool =  false;
    static let mut window_padding: bool =  true;
    static let type: c_int = 5; // Aspect Ratio
    static let display_lines: c_int = 10;

    // Submit constraint
    let aspect_ratio: c_float =  16f32 / 9.0;
    let fixed_step: c_float =  100;
    if (type == 0) SetNextWindowSizeConstraints(ImVec2::new(100, 100), ImVec2::new(500, 500));         // Between 100x100 and 500x500
    if (type == 1) SetNextWindowSizeConstraints(ImVec2::new(100, 100), ImVec2::new(f32::MAX, f32::MAX)); // Width > 100, Height > 100
    if (type == 2) SetNextWindowSizeConstraints(ImVec2::new(-1, 0),    ImVec2::new(-1, f32::MAX));      // Vertical only
    if (type == 3) SetNextWindowSizeConstraints(ImVec2::new(0, -1),    ImVec2::new(f32::MAX, -1));      // Horizontal only
    if (type == 4) SetNextWindowSizeConstraints(ImVec2::new(400, -1),  ImVec2::new(500, -1));          // Width Between and 400 and 500
    if (type == 5) SetNextWindowSizeConstraints(ImVec2::new(0, 0),     ImVec2::new(f32::MAX, f32::MAX), CustomConstraints::AspectRatio, &aspect_ratio);   // Aspect ratio
    if (type == 6) SetNextWindowSizeConstraints(ImVec2::new(0, 0),     ImVec2::new(f32::MAX, f32::MAX), CustomConstraints::Square);                              // Always Square
    if (type == 7) SetNextWindowSizeConstraints(ImVec2::new(0, 0),     ImVec2::new(f32::MAX, f32::MAX), CustomConstraints::Step, &fixed_step);            // Fixed Step

    // Submit window
    if (!window_padding)
        PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2::new(0.0, 0.0));
    const window_flags: ImGuiWindowFlags = if auto_resize { ImGuiWindowFlags_AlwaysAutoResize} else {0};
    let window_open: bool = Begin("Example: Constrained Resize", p_open, window_flags);
    if (!window_padding)
        PopStyleVar();
    if (window_open)
    {
        IMGUI_DEMO_MARKER("Examples/Constrained Resizing window");
        if (GetIO().KeyShift)
        {
            // Display a dummy viewport (in your real app you would likely use ImageButton() to display a texture.
            let avail_size: ImVec2 = GetContentRegionAvail();
            let pos: ImVec2 = GetCursorScreenPos();
            ColorButton("viewport", ImVec4(0.5, 0.2f, 0.5, 1.0), ImGuiColorEditFlags_NoTooltip | ImGuiColorEditFlags_NoDragDrop, avail_size);
            SetCursorScreenPos(ImVec2::new(pos.x + 10, pos.y + 10));
            Text("{} x {}", avail_size.x, avail_size.y);
        }
        else
        {
            Text("(Hold SHIFT to display a dummy viewport)");
            if (IsWindowDocked())
                Text("Warning: Sizing Constraints won't work if the window is docked!");
            if (Button("Set 200x200")) { SetWindowSize(ImVec2::new(200, 200)); } SameLine();
            if (Button("Set 500x500")) { SetWindowSize(ImVec2::new(500, 500)); } SameLine();
            if (Button("Set 800x200")) { SetWindowSize(ImVec2::new(800, 200)); }
            SetNextItemWidth(GetFontSize() * 20);
            Combo("Constraint", &type, test_desc, test_desc.len());
            SetNextItemWidth(GetFontSize() * 20);
            DragInt("Lines", &display_lines, 0.2f, 1, 100);
            Checkbox("Auto-resize", &auto_resize);
            Checkbox("Window padding", &window_padding);
            for (let i: c_int = 0; i < display_lines; i++)
                Text("%*sHello, sailor! Making this line long enough for the example.", i * 4, "");
        }
    }
    End();
}

//-----------------------------------------------------------------------------
// [SECTION] Example App: Simple overlay / ShowExampleAppSimpleOverlay()
//-----------------------------------------------------------------------------

// Demonstrate creating a simple static window with no decoration
// + a context-menu to choose which corner of the screen to use.
pub unsafe fn ShowExampleAppSimpleOverlay(bool* p_open)
{
    static let location: c_int = 0;
    ImGuiIO& io = GetIO();
    window_flags: ImGuiWindowFlags = ImGuiWindowFlags_NoDecoration | ImGuiWindowFlags_NoDocking | ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_NoFocusOnAppearing | ImGuiWindowFlags_NoNav;
    if (location >= 0)
    {
        let PAD: c_float =  10.0;
        let viewport: *const ImGuiViewport = GetMainViewport();
        let work_pos: ImVec2 = viewport.WorkPos; // Use work area to avoid menu-bar/task-bar, if any!
        let work_size: ImVec2 = viewport.WorkSize;
        window_pos: ImVec2, window_pos_pivot;
        window_pos.x = if (location & 1) { (work_pos.x + work_size.x - PAD)} else {work_pos.x + PAD};
        window_pos.y = if (location & 2) { (work_pos.y + work_size.y - PAD)} else {work_pos.y + PAD};
        window_pos_pivot.x = if (location & 1) { 1.0} else {0.0};
        window_pos_pivot.y = if (location & 2) { 1.0} else {0.0};
        SetNextWindowPos(window_pos, ImGuiCond_Always, window_pos_pivot);
        SetNextWindowViewport(viewport.ID);
        window_flags |= ImGuiWindowFlags_NoMove;
    }
    else if (location == -2)
    {
        // Center window
        SetNextWindowPos(GetMainViewport()->GetCenter(), ImGuiCond_Always, ImVec2::new(0.5, 0.5));
        window_flags |= ImGuiWindowFlags_NoMove;
    }
    SetNextWindowBgAlpha(0.350f32); // Transparent background
    if (Begin("Example: Simple overlay", p_open, window_flags))
    {
        IMGUI_DEMO_MARKER("Examples/Simple Overlay");
        Text("Simple overlay\n" "(right-click to change position)");
        Separator();
        if (IsMousePosValid())
            Text("Mouse Position: ({},{})", io.MousePos.x, io.MousePos.y);
        else
            Text("Mouse Position: <invalid>");
        if (BeginPopupContextWindow())
        {
            if (MenuItem("Custom",       None, location == -1)) location = -1;
            if (MenuItem("Center",       None, location == -2)) location = -2;
            if MenuItem("Top-left",     None, location == 0) {  location = 0;}
            if MenuItem("Top-right",    None, location == 1) {  location = 1;}
            if MenuItem("Bottom-left",  None, location == 2) {  location = 2;}
            if MenuItem("Bottom-right", None, location == 3) {  location = 3;}
            if (p_open && MenuItem("Close")) *p_open = false;
            EndPopup();
        }
    }
    End();
}

//-----------------------------------------------------------------------------
// [SECTION] Example App: Fullscreen window / ShowExampleAppFullscreen()
//-----------------------------------------------------------------------------

// Demonstrate creating a window covering the entire screen/viewport
pub unsafe fn ShowExampleAppFullscreen(bool* p_open)
{
    static let mut use_work_area: bool =  true;
    static flags: ImGuiWindowFlags = ImGuiWindowFlags_NoDecoration | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoSavedSettings;

    // We demonstrate using the full viewport area or the work area (without menu-bars, task-bars etc.)
    // Based on your use case you may want one of the other.
    let viewport: *const ImGuiViewport = GetMainViewport();
    SetNextWindowPos(if use_work_area { viewport.WorkPos} else {viewport.Pos});
    SetNextWindowSize(if use_work_area { viewport.WorkSize} else {viewport.Size});

    if (Begin("Example: Fullscreen window", p_open, flags))
    {
        Checkbox("Use work area instead of main area", &use_work_area);
        SameLine();
        HelpMarker("Main Area = entire viewport,\nWork Area = entire viewport minus sections used by the main menu bars, task bars etc.\n\nEnable the main-menu bar in Examples menu to see the difference.");

        CheckboxFlags("ImGuiWindowFlags_NoBackground", &flags, ImGuiWindowFlags_NoBackground);
        CheckboxFlags("ImGuiWindowFlags_NoDecoration", &flags, ImGuiWindowFlags_NoDecoration);
        Indent();
        CheckboxFlags("ImGuiWindowFlags_NoTitleBar", &flags, ImGuiWindowFlags_NoTitleBar);
        CheckboxFlags("ImGuiWindowFlags_NoCollapse", &flags, ImGuiWindowFlags_NoCollapse);
        CheckboxFlags("ImGuiWindowFlags_NoScrollbar", &flags, ImGuiWindowFlags_NoScrollbar);
        Unindent();

        if (p_open && Button("Close this window"))
            *p_open = false;
    }
    End();
}

//-----------------------------------------------------------------------------
// [SECTION] Example App: Manipulating Window Titles / ShowExampleAppWindowTitles()
//-----------------------------------------------------------------------------

// Demonstrate using "##" and "###" in identifiers to manipulate ID generation.
// This apply to all regular items as well.
// Read FAQ section "How can I have multiple widgets with the same label?" for details.
pub unsafe fn ShowExampleAppWindowTitles(bool*)
{
    let viewport: *const ImGuiViewport = GetMainViewport();
    let base_pos: ImVec2 = viewport.Pos;

    // By default, Windows are uniquely identified by their title.
    // You can use the "##" and "###" markers to manipulate the display/ID.

    // Using "##" to display same title but have unique identifier.
    SetNextWindowPos(ImVec2::new(base_pos.x + 100, base_pos.y + 100), ImGuiCond_FirstUseEver);
    Begin("Same title as another window##1");
    IMGUI_DEMO_MARKER("Examples/Manipulating window titles");
    Text("This is window 1.\nMy title is the same as window 2, but my identifier is unique.");
    End();

    SetNextWindowPos(ImVec2::new(base_pos.x + 100, base_pos.y + 200), ImGuiCond_FirstUseEver);
    Begin("Same title as another window##2");
    Text("This is window 2.\nMy title is the same as window 1, but my identifier is unique.");
    End();

    // Using "###" to display a changing title but keep a static identifier "AnimatedTitle"
    buf: [c_char;128];
    sprintf(buf, "Animated title {} {}###AnimatedTitle", "|/-\\"[(GetTime() / 0.250f32) & 3], GetFrameCount());
    SetNextWindowPos(ImVec2::new(base_pos.x + 100, base_pos.y + 300), ImGuiCond_FirstUseEver);
    Begin(buf);
    Text("This window has a changing title.");
    End();
}

//-----------------------------------------------------------------------------
// [SECTION] Example App: Custom Rendering using ImDrawList API / ShowExampleAppCustomRendering()
//-----------------------------------------------------------------------------

// Demonstrate using the low-level ImDrawList to draw custom shapes.
pub unsafe fn ShowExampleAppCustomRendering(bool* p_open)
{
    if (!Begin("Example: Custom rendering", p_open))
    {
        End();
        return;
    }
    IMGUI_DEMO_MARKER("Examples/Custom Rendering");

    // Tip: If you do a lot of custom rendering, you probably want to use your own geometrical types and benefit of
    // overloaded operators, etc. Define IM_VEC2_CLASS_EXTRA in imconfig.h to create implicit conversions between your
    // types and ImVec2/ImVec4. Dear ImGui defines overloaded operators but they are internal to imgui.cpp and not
    // exposed outside (to avoid messing with your types) In this example we are not using the maths operators!

    if (BeginTabBar("##TabBar"))
    {
        if (BeginTabItem("Primitives"))
        {
            PushItemWidth(-GetFontSize() * 15);
            let mut  draw_list: *mut ImDrawList =  GetWindowDrawList();

            // Draw gradients
            // (note that those are currently exacerbating our sRGB/Linear issues)
            // Calling GetColorU32() multiplies the given colors by the current Style Alpha, but you may pass the IM_COL32() directly as well..
            Text("Gradients");
            let gradient_size: ImVec2 = ImVec2::new(CalcItemWidth(), GetFrameHeight());
            {
                let p0: ImVec2 = GetCursorScreenPos();
                let p1: ImVec2 = ImVec2::new(p0.x + gradient_size.x, p0.y + gradient_size.y);
                col_a: u32 = GetColorU32(IM_COL32(0, 0, 0, 255));
                col_b: u32 = GetColorU32(IM_COL32(255, 255, 255, 255));
                draw_list.AddRectFilledMultiColor(p0, p1, col_a, col_b, col_b, col_a);
                InvisibleButton("##gradient1", gradient_size);
            }
            {
                let p0: ImVec2 = GetCursorScreenPos();
                let p1: ImVec2 = ImVec2::new(p0.x + gradient_size.x, p0.y + gradient_size.y);
                col_a: u32 = GetColorU32(IM_COL32(0, 255, 0, 255));
                col_b: u32 = GetColorU32(IM_COL32(255, 0, 0, 255));
                draw_list.AddRectFilledMultiColor(p0, p1, col_a, col_b, col_b, col_a);
                InvisibleButton("##gradient2", gradient_size);
            }

            // Draw a bunch of primitives
            Text("All primitives");
            static let sz: c_float =  36f32;
            static let thickness: c_float =  3.0;
            static let ngon_sides: c_int = 6;
            static let mut circle_segments_override: bool =  false;
            static let circle_segments_override_v: c_int = 12;
            static let mut curve_segments_override: bool =  false;
            static let curve_segments_override_v: c_int = 8;
            static colf: ImVec4 = ImVec4(1.0, 1.0, 0.4f, 1.0);
            DragFloat("Size", &sz, 0.2f, 2.0, 100, "{}f");
            DragFloat("Thickness", &thickness, 0.05f, 1.0, 8.0, "{}2f");
            SliderInt("N-gon sides", &ngon_sides, 3, 12);
            Checkbox("##circlesegmentoverride", &circle_segments_override);
            SameLine(0.0, GetStyle().ItemInnerSpacing.x);
            circle_segments_override |= SliderInt("Circle segments override", &circle_segments_override_v, 3, 40);
            Checkbox("##curvessegmentoverride", &curve_segments_override);
            SameLine(0.0, GetStyle().ItemInnerSpacing.x);
            curve_segments_override |= SliderInt("Curves segments override", &curve_segments_override_v, 3, 40);
            ColorEdit4("Color", &colf.x);

            let p: ImVec2 = GetCursorScreenPos();
            col: u32 = ImColor(col0f32);
            let spacing: c_float =  10.0;
            const corners_tl_br: ImDrawFlags = ImDrawFlags_RoundCornersTopLeft | ImDrawFlags_RoundCornersBottomRight;
            let rounding: c_float =  sz / 5f32;
            let circle_segments: c_int = if circle_segments_override { circle_segments_override_v} else {0};
            let curve_segments: c_int = if curve_segments_override { curve_segments_override_v} else {0};
            let x: c_float =  p.x + 4.0;
            let y: c_float =  p.y + 4.0;
            for (let n: c_int = 0; n < 2; n++)
            {
                // First line uses a thickness of 1.0, second line uses the configurable thickness
                let th: c_float = if (n == 0) { 1.0} else {thickness};
                draw_list.AddNgon(ImVec2::new(x + sz*0.5, y + sz*0.5), sz*0.5, col, ngon_sides, th);                 x += sz + spacing;  // N-gon
                draw_list.AddCircle(ImVec2::new(x + sz*0.5, y + sz*0.5), sz*0.5, col, circle_segments, th);          x += sz + spacing;  // Circle
                draw_list.AddRect(ImVec2::new(x, y), ImVec2::new(x + sz, y + sz), col, 0.0, ImDrawFlags_None, th);          x += sz + spacing;  // Square
                draw_list.AddRect(ImVec2::new(x, y), ImVec2::new(x + sz, y + sz), col, rounding, ImDrawFlags_None, th);      x += sz + spacing;  // Square with all rounded corners
                draw_list.AddRect(ImVec2::new(x, y), ImVec2::new(x + sz, y + sz), col, rounding, corners_tl_br, th);         x += sz + spacing;  // Square with two rounded corners
                draw_list.AddTriangle(ImVec2::new(x+sz*0.5,y), ImVec2::new(x+sz, y+sz-0.5), ImVec2::new(x, y+sz-0.5), col, th);x += sz + spacing;  // Triangle
                //draw_list.AddTriangle(ImVec2::new(x+sz*0.2f,y), ImVec2::new(x, y+sz-0.5), ImVec2::new(x+sz*0.4f, y+sz-0.5), col, th);x+= sz*0.4f + spacing; // Thin triangle
                draw_list.AddLine(ImVec2::new(x, y), ImVec2::new(x + sz, y), col, th);                                       x += sz + spacing;  // Horizontal line (note: drawing a filled rectangle will be faster!)
                draw_list.AddLine(ImVec2::new(x, y), ImVec2::new(x, y + sz), col, th);                                       x += spacing;       // Vertical line (note: drawing a filled rectangle will be faster!)
                draw_list.AddLine(ImVec2::new(x, y), ImVec2::new(x + sz, y + sz), col, th);                                  x += sz + spacing;  // Diagonal line

                // Quadratic Bezier Curve (3 control points)
                cp3: ImVec2[3] = { ImVec2::new(x, y + sz * 0.60), ImVec2::new(x + sz * 0.5, y - sz * 0.40f32), ImVec2::new(x + sz, y + sz) };
                draw_list.AddBezierQuadratic(cp3[0], cp3[1], cp3[2], col, th, curve_segments); x += sz + spacing;

                // Cubic Bezier Curve (4 control points)
                cp4: ImVec2[4] = { ImVec2::new(x, y), ImVec2::new(x + sz * 1.3f, y + sz * 0.3f32), ImVec2::new(x + sz - sz * 1.3f, y + sz - sz * 0.3f32), ImVec2::new(x + sz, y + sz) };
                draw_list.AddBezierCubic(cp4[0], cp4[1], cp4[2], cp4[3], col, th, curve_segments);

                x = p.x + 4;
                y += sz + spacing;
            }
            draw_list.AddNgonFilled(ImVec2::new(x + sz * 0.5, y + sz * 0.5), sz*0.5, col, ngon_sides);               x += sz + spacing;  // N-gon
            draw_list.AddCircleFilled(ImVec2::new(x + sz*0.5, y + sz*0.5), sz*0.5, col, circle_segments);            x += sz + spacing;  // Circle
            draw_list.AddRectFilled(ImVec2::new(x, y), ImVec2::new(x + sz, y + sz), col);                                    x += sz + spacing;  // Square
            draw_list.AddRectFilled(ImVec2::new(x, y), ImVec2::new(x + sz, y + sz), col, 10.0);                             x += sz + spacing;  // Square with all rounded corners
            draw_list.AddRectFilled(ImVec2::new(x, y), ImVec2::new(x + sz, y + sz), col, 10.0, corners_tl_br);              x += sz + spacing;  // Square with two rounded corners
            draw_list.AddTriangleFilled(ImVec2::new(x+sz*0.5,y), ImVec2::new(x+sz, y+sz-0.5), ImVec2::new(x, y+sz-0.5), col);  x += sz + spacing;  // Triangle
            //draw_list.AddTriangleFilled(ImVec2::new(x+sz*0.2f,y), ImVec2::new(x, y+sz-0.5), ImVec2::new(x+sz*0.4f, y+sz-0.5), col); x += sz*0.4f + spacing; // Thin triangle
            draw_list.AddRectFilled(ImVec2::new(x, y), ImVec2::new(x + sz, y + thickness), col);                             x += sz + spacing;  // Horizontal line (faster than AddLine, but only handle integer thickness)
            draw_list.AddRectFilled(ImVec2::new(x, y), ImVec2::new(x + thickness, y + sz), col);                             x += spacing * 2.0;// Vertical line (faster than AddLine, but only handle integer thickness)
            draw_list.AddRectFilled(ImVec2::new(x, y), ImVec2::new(x + 1, y + 1), col);                                      x += sz;            // Pixel (faster than AddLine)
            draw_list.AddRectFilledMultiColor(ImVec2::new(x, y), ImVec2::new(x + sz, y + sz), IM_COL32(0, 0, 0, 255), IM_COL32(255, 0, 0, 255), IM_COL32(255, 255, 0, 255), IM_COL32(0, 255, 0, 255));

            Dummy(ImVec2::new((sz + spacing) * 10.2f, (sz + spacing) * 3.0));
            PopItemWidth();
            EndTabItem();
        }

        if (BeginTabItem("Canvas"))
        {
            static Vec<ImVec2> points;
            static scrolling: ImVec2::new(0.0, 0.0);
            static let mut opt_enable_grid: bool =  true;
            static let mut opt_enable_context_menu: bool =  true;
            static let mut adding_line: bool =  false;

            Checkbox("Enable grid", &opt_enable_grid);
            Checkbox("Enable context menu", &opt_enable_context_menu);
            Text("Mouse Left: drag to add lines,\nMouse Right: drag to scroll, click for context menu.");

            // Typically you would use a BeginChild()/EndChild() pair to benefit from a clipping region + own scrolling.
            // Here we demonstrate that this can be replaced by simple offsetting + custom drawing + PushClipRect/PopClipRect() calls.
            // To use a child window instead we could use, e.g:
            //      PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2::new(0, 0));      // Disable padding
            //      PushStyleColor(ImGuiCol_ChildBg, IM_COL32(50, 50, 50, 255));  // Set a background color
            //      BeginChild("canvas", ImVec2::new(0.0, 0.0), true, ImGuiWindowFlags_NoMove);
            //      PopStyleColor();
            //      PopStyleVar();
            //      [...]
            //      EndChild();

            // Using InvisibleButton() as a convenience 1) it will advance the layout cursor and 2) allows us to use IsItemHovered()/IsItemActive()
            let canvas_p0: ImVec2 = GetCursorScreenPos();      // ImDrawList API uses screen coordinates!
            let canvas_sz: ImVec2 = GetContentRegionAvail();   // Resize canvas to what's available
            if canvas_sz.x < 50f32{ canvas_sz.x = 50f32;}
            if canvas_sz.y < 50f32{ canvas_sz.y = 50f32;}
            let canvas_p1: ImVec2 = ImVec2::new(canvas_p0.x + canvas_sz.x, canvas_p0.y + canvas_sz.y);

            // Draw border and background color
            ImGuiIO& io = GetIO();
            let mut  draw_list: *mut ImDrawList =  GetWindowDrawList();
            draw_list.AddRectFilled(canvas_p0, canvas_p1, IM_COL32(50, 50, 50, 255));
            draw_list.AddRect(canvas_p0, canvas_p1, IM_COL32(255, 255, 255, 255));

            // This will catch our interactions
            InvisibleButton("canvas", canvas_sz, ImGuiButtonFlags_MouseButtonLeft | ImGuiButtonFlags_MouseButtonRight);
            let is_hovered: bool = IsItemHovered(); // Hovered
            let is_active: bool = IsItemActive();   // Held
            const origin: ImVec2(canvas_p0.x + scrolling.x, canvas_p0.y + scrolling.y); // Lock scrolled origin
            const mouse_pos_in_canvas: ImVec2(io.MousePos.x - origin.x, io.MousePos.y - origin.y);

            // Add first and second point
            if (is_hovered && !adding_line && IsMouseClicked(ImGuiMouseButton_Left))
            {
                points.push(mouse_pos_in_canvas);
                points.push(mouse_pos_in_canvas);
                adding_line = true;
            }
            if (adding_line)
            {
                points.last().unwrap() = mouse_pos_in_canvas;
                if !IsMouseDown(ImGuiMouseButton_Left) {
                    adding_line = false;}
            }

            // Pan (we use a zero mouse threshold when there's no context menu)
            // You may decide to make that threshold dynamic based on whether the mouse is hovering something etc.
            let mouse_threshold_for_pan: c_float =  if opt_enable_context_menu { - 1.0} else {0.0};
            if (is_active && IsMouseDragging(ImGuiMouseButton_Right, mouse_threshold_for_pan))
            {
                scrolling.x += io.MouseDelta.x;
                scrolling.y += io.MouseDelta.y;
            }

            // Context menu (under default mouse threshold)
            let drag_delta: ImVec2 = GetMouseDragDelta(ImGuiMouseButton_Right);
            if (opt_enable_context_menu && drag_delta.x == 0.0 && drag_delta.y == 0.0)
                OpenPopupOnItemClick("context", ImGuiPopupFlags_MouseButtonRight);
            if (BeginPopup("context"))
            {
                if (adding_line)
                    points.resize(points.size() - 2);
                adding_line = false;
                if (MenuItem("Remove one", None, false, points.Size > 0)) { points.resize(points.size() - 2); }
                if (MenuItem("Remove all", None, false, points.Size > 0)) { points.clear(); }
                EndPopup();
            }

            // Draw grid + all lines in the canvas
            draw_list.PushClipRect(canvas_p0, canvas_p1, true);
            if (opt_enable_grid)
            {
                let GRID_STEP: c_float =  64.0;
                for (let x: c_float =  fmodf(scrolling.x, GRID_STEP); x < canvas_sz.x; x += GRID_STEP)
                    draw_list.AddLine(ImVec2::new(canvas_p0.x + x, canvas_p0.y), ImVec2::new(canvas_p0.x + x, canvas_p1.y), IM_COL32(200, 200, 200, 40));
                for (let y: c_float =  fmodf(scrolling.y, GRID_STEP); y < canvas_sz.y; y += GRID_STEP)
                    draw_list.AddLine(ImVec2::new(canvas_p0.x, canvas_p0.y + y), ImVec2::new(canvas_p1.x, canvas_p0.y + y), IM_COL32(200, 200, 200, 40));
            }
            for (let n: c_int = 0; n < points.Size; n += 2)
                draw_list.AddLine(ImVec2::new(origin.x + points[n].x, origin.y + points[n].y), ImVec2::new(origin.x + points[n + 1].x, origin.y + points[n + 1].y), IM_COL32(255, 255, 0, 255), 2.0);
            draw_list.PopClipRect();

            EndTabItem();
        }

        if (BeginTabItem("BG/FG draw lists"))
        {
            static let mut draw_bg: bool =  true;
            static let mut draw_fg: bool =  true;
            Checkbox("Draw in Background draw list", &draw_bg);
            SameLine(); HelpMarker("The Background draw list will be rendered below every Dear ImGui windows.");
            Checkbox("Draw in Foreground draw list", &draw_fg);
            SameLine(); HelpMarker("The Foreground draw list will be rendered over every Dear ImGui windows.");
            let window_pos: ImVec2 = GetWindowPos();
            let window_size: ImVec2 = GetWindowSize();
            let window_center: ImVec2 = ImVec2::new(window_pos.x + window_size.x * 0.5, window_pos.y + window_size.y * 0.5);
            if (draw_bg)
                GetBackgroundDrawList()->AddCircle(window_center, window_size.x * 0.6f, IM_COL32(255, 0, 0, 200), 0, 10 + 4);
            if (draw_fg)
                GetForegroundDrawList()->AddCircle(window_center, window_size.y * 0.6f, IM_COL32(0, 255, 0, 200), 0, 10);
            EndTabItem();
        }

        EndTabBar();
    }

    End();
}

//-----------------------------------------------------------------------------
// [SECTION] Example App: Docking, DockSpace / ShowExampleAppDockSpace()
//-----------------------------------------------------------------------------

// Demonstrate using DockSpace() to create an explicit docking node within an existing window.
// Note: You can use most Docking facilities without calling any API. You DO NOT need to call DockSpace() to use Docking!
// - Drag from window title bar or their tab to dock/undock. Hold SHIFT to disable docking.
// - Drag from window menu button (upper-left button) to undock an entire node (all windows).
// - When io.ConfigDockingWithShift == true, you instead need to hold SHIFT to _enable_ docking/undocking.
// About dockspaces:
// - Use DockSpace() to create an explicit dock node _within_ an existing window.
// - Use DockSpaceOverViewport() to create an explicit dock node covering the screen or a specific viewport.
//   This is often used with ImGuiDockNodeFlags_PassthruCentralNode.
// - Important: Dockspaces need to be submitted _before_ any window they can host. Submit it early in your frame! (*)
// - Important: Dockspaces need to be kept alive if hidden, otherwise windows docked into it will be undocked.
//   e.g. if you have multiple tabs with a dockspace inside each tab: submit the non-visible dockspaces with ImGuiDockNodeFlags_KeepAliveOnly.
// (*) because of this constraint, the implicit \"Debug\" window can not be docked into an explicit DockSpace() node,
// because that window is submitted as part of the part of the NewFrame() call. An easy workaround is that you can create
// your own implicit "Debug##2" window after calling DockSpace() and leave it in the window stack for anyone to use.
pub unsafe fn ShowExampleAppDockSpace(bool* p_open)
{
    // If you strip some features of, this demo is pretty much equivalent to calling DockSpaceOverViewport()!
    // In most cases you should be able to just call DockSpaceOverViewport() and ignore all the code below!
    // In this specific demo, we are not using DockSpaceOverViewport() because:
    // - we allow the host window to be floating/moveable instead of filling the viewport (when opt_fullscreen == false)
    // - we allow the host window to have padding (when opt_padding == true)
    // - we have a local menu bar in the host window (vs. you could use BeginMainMenuBar() + DockSpaceOverViewport() in your code!)
    // TL;DR; this demo is more complicated than what you would normally use.
    // If we removed all the options we are showcasing, this demo would become:
    //     void ShowExampleAppDockSpace()
    //     {
    //         DockSpaceOverViewport(GetMainViewport());
    //     }

    static let mut opt_fullscreen: bool =  true;
    static let mut opt_padding: bool =  false;
    static ImGuiDockNodeFlags dockspace_flags = ImGuiDockNodeFlags_None;

    // We are using the ImGuiWindowFlags_NoDocking flag to make the parent window not dockable into,
    // because it would be confusing to have two docking targets within each others.
    window_flags: ImGuiWindowFlags = ImGuiWindowFlags_MenuBar | ImGuiWindowFlags_NoDocking;
    if (opt_fullscreen)
    {
        let viewport: *const ImGuiViewport = GetMainViewport();
        SetNextWindowPos(viewport.WorkPos);
        SetNextWindowSize(viewport.WorkSize);
        SetNextWindowViewport(viewport.ID);
        PushStyleVar(ImGuiStyleVar_WindowRounding, 0.0);
        PushStyleVar(ImGuiStyleVar_WindowBorderSize, 0.0);
        window_flags |= ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove;
        window_flags |= ImGuiWindowFlags_NoBringToFrontOnFocus | ImGuiWindowFlags_NoNavFocus;
    }
    else
    {
        dockspace_flags &= !ImGuiDockNodeFlags_PassthruCentralNode;
    }

    // When using ImGuiDockNodeFlags_PassthruCentralNode, DockSpace() will render our background
    // and handle the pass-thru hole, so we ask Begin() to not render a background.
    if (dockspace_flags & ImGuiDockNodeFlags_PassthruCentralNode)
        window_flags |= ImGuiWindowFlags_NoBackground;

    // Important: note that we proceed even if Begin() returns false (aka window is collapsed).
    // This is because we want to keep our DockSpace() active. If a DockSpace() is inactive,
    // all active windows docked into it will lose their parent and become undocked.
    // We cannot preserve the docking relationship between an active window and an inactive docking, otherwise
    // any change of dockspace/settings would lead to windows being stuck in limbo and never being visible.
    if (!opt_padding)
        PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2::new(0.0, 0.0));
    Begin("DockSpace Demo", p_open, window_flags);
    if (!opt_padding)
        PopStyleVar();

    if opt_fullscreen {
        PopStyleVar(2)(); }

    // Submit the DockSpace
    ImGuiIO& io = GetIO();
    if (io.ConfigFlags & ImGuiConfigFlags_DockingEnable)
    {
        let mut dockspace_id: ImguiHandle =  GetID("MyDockSpace");
        DockSpace(dockspace_id, ImVec2::new(0.0, 0.0), dockspace_flags);
    }
    else
    {
        ShowDockingDisabledMessage();
    }

    if (BeginMenuBar())
    {
        if (BeginMenu("Options"))
        {
            // Disabling fullscreen would allow the window to be moved to the front of other windows,
            // which we can't undo at the moment without finer window depth/z control.
            MenuItem("Fullscreen", None, &opt_fullscreen);
            MenuItem("Padding", None, &opt_padding);
            Separator();

            if (MenuItem("Flag: NoSplit",                "", (dockspace_flags & ImGuiDockNodeFlags_NoSplit) != 0))                 { dockspace_flags ^= ImGuiDockNodeFlags_NoSplit; }
            if (MenuItem("Flag: NoResize",               "", (dockspace_flags & ImGuiDockNodeFlags_NoResize) != 0))                { dockspace_flags ^= ImGuiDockNodeFlags_NoResize; }
            if (MenuItem("Flag: NoDockingInCentralNode", "", (dockspace_flags & ImGuiDockNodeFlags_NoDockingInCentralNode) != 0))  { dockspace_flags ^= ImGuiDockNodeFlags_NoDockingInCentralNode; }
            if (MenuItem("Flag: AutoHideTabBar",         "", (dockspace_flags & ImGuiDockNodeFlags_AutoHideTabBar) != 0))          { dockspace_flags ^= ImGuiDockNodeFlags_AutoHideTabBar; }
            if (MenuItem("Flag: PassthruCentralNode",    "", (dockspace_flags & ImGuiDockNodeFlags_PassthruCentralNode) != 0, opt_fullscreen)) { dockspace_flags ^= ImGuiDockNodeFlags_PassthruCentralNode; }
            Separator();

            if (MenuItem("Close", None, false, p_open != null_mut()))
                *p_open = false;
            EndMenu();
        }
        HelpMarker(
            "When docking is enabled, you can ALWAYS dock MOST window into another! Try it now!" "\n"
            "- Drag from window title bar or their tab to dock/undock." "\n"
            "- Drag from window menu button (upper-left button) to undock an entire node (all windows)." "\n"
            "- Hold SHIFT to disable docking (if io.ConfigDockingWithShift == false, default)" "\n"
            "- Hold SHIFT to enable docking (if io.ConfigDockingWithShift == true)" "\n"
            "This demo app has nothing to do with enabling docking!" "\n\n"
            "This demo app only demonstrate the use of DockSpace() which allows you to manually create a docking node _within_ another window." "\n\n"
            "Read comments in ShowExampleAppDockSpace() for more details.");

        EndMenuBar();
    }

    End();
}

//-----------------------------------------------------------------------------
// [SECTION] Example App: Documents Handling / ShowExampleAppDocuments()
//-----------------------------------------------------------------------------

// Simplified structure to mimic a Document model
struct MyDocument
{
let Name: *const c_char;       // Document title
    bool        Open;       // Set when open (we keep an array of all available documents to simplify demo code!)
    bool        OpenPrev;   // Copy of Open from last update.
    bool        Dirty;      // Set when the document has been modified
    bool        WantClose;  // Set when the document
    ImVec4      Color;      // An arbitrary variable associated to the document

    MyDocument(name: *const c_char, let mut open: bool =  true, color: &ImVec4 = ImVec4(1.0, 1.0, 1.0, 1.0))
    {
        Name = name;
        Open = OpenPrev = open;
        Dirty = false;
        WantClose = false;
        Color = color;
    }
    c_void DoOpen()       { Open = true; }
    c_void DoQueueClose() { WantClose = true; }
    c_void DoForceClose() { Open = false; Dirty = false; }
    c_void DoSave()       { Dirty = false; }

    // Display placeholder contents for the Document
    static c_void DisplayContents(MyDocument* doc)
    {
        PushID(doc);
        Text("Document \"{}\"", doc.Name);
        PushStyleColor(ImGuiCol_Text, doc->Color);
        TextWrapped("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.");
        PopStyleColor();
        if (Button("Modify", ImVec2::new(100, 0)))
            doc->Dirty = true;
        SameLine();
        if (Button("Save", ImVec2::new(100, 0)))
            doc->DoSave();
        ColorEdit3("color", &doc->Color.x);  // Useful to test drag and drop and hold-dragged-to-open-tab behavior.
        PopID();
    }

    // Display context menu for the Document
    static c_void DisplayContextMenu(MyDocument* doc)
    {
        if !BeginPopupContextItem() { return ; }

        buf: [c_char;256];
        sprintf(buf, "Save {}", doc.Name);
        if (MenuItem(buf, "CTRL+S", false, doc->Open))
            doc->DoSave();
        if (MenuItem("Close", "CTRL+W", false, doc->Open))
            doc->DoQueueClose();
        EndPopup();
    }
};

struct ExampleAppDocuments
{
    Vec<MyDocument> Documents;

    ExampleAppDocuments()
    {
        Documents.push(MyDocument("Lettuce",             true,  ImVec4(0.4f, 0.8f, 0.4f, 1.0)));
        Documents.push(MyDocument("Eggplant",            true,  ImVec4(0.8f, 0.5, 1.0, 1.0)));
        Documents.push(MyDocument("Carrot",              true,  ImVec4(1.0, 0.8f, 0.5, 1.0)));
        Documents.push(MyDocument("Tomato",              false, ImVec4(1.0, 0.3f, 0.4f, 1.0)));
        Documents.push(MyDocument("A Rather Long Title", false));
        Documents.push(MyDocument("Some Document",       false));
    }
};

// [Optional] Notify the system of Tabs/Windows closure that happened outside the regular tab interface.
// If a tab has been closed programmatically (aka closed from another source such as the Checkbox() in the demo,
// as opposed to clicking on the regular tab closing button) and stops being submitted, it will take a frame for
// the tab bar to notice its absence. During this frame there will be a gap in the tab bar, and if the tab that has
// disappeared was the selected one, the tab bar will report no selected tab during the frame. This will effectively
// give the impression of a flicker for one frame.
// We call SetTabItemClosed() to manually notify the Tab Bar or Docking system of removed tabs to avoid this glitch.
// Note that this completely optional, and only affect tab bars with the ImGuiTabBarFlags_Reorderable flag.
pub unsafe fn NotifyOfDocumentsClosedElsewhere(ExampleAppDocuments& app)
{
    for (let doc_n: c_int = 0; doc_n < app.Documents.Size; doc_n++)
    {
        MyDocument* doc = &app.Documents[doc_n];
        if (!doc->Open && doc->OpenPrev)
            SetTabItemClosed(doc.Name);
        doc->OpenPrev = doc->Open;
    }
}

pub unsafe fn ShowExampleAppDocuments(bool* p_open)
{
    static ExampleAppDocuments app;

    // Options
    enum Target
    {
        Target_None,
        Target_Tab,                 // Create documents as local tab into a local tab bar
        Target_DockSpaceAndWindow   // Create documents as regular windows, and create an embedded dockspace
    };
    static Target opt_target = Target_Tab;
    static let mut opt_reorderable: bool =  true;
    static opt_fitting_flags: ImGuiTabBarFlags = ImGuiTabBarFlags_FittingPolicyDefault_;

    // When (opt_target == Target_DockSpaceAndWindow) there is the possibily that one of our child Document window (e.g. "Eggplant")
    // that we emit gets docked into the same spot as the parent window ("Example: Documents").
    // This would create a problematic feedback loop because selecting the "Eggplant" tab would make the "Example: Documents" tab
    // not visible, which in turn would stop submitting the "Eggplant" window.
    // We avoid this problem by submitting our documents window even if our parent window is not currently visible.
    // Another solution may be to make the "Example: Documents" window use the ImGuiWindowFlags_NoDocking.

    let mut window_contents_visible: bool =  Begin("Example: Documents", p_open, ImGuiWindowFlags_MenuBar);
    if (!window_contents_visible && opt_target != Target_DockSpaceAndWindow)
    {
        End();
        return;
    }

    // Menu
    if (BeginMenuBar())
    {
        if (BeginMenu("File"))
        {
            let open_count: c_int = 0;
            for (let doc_n: c_int = 0; doc_n < app.Documents.Size; doc_n++)
                open_count += if app.Documents[doc_n].Open { 1} else {0};

            if (BeginMenu("Open", open_count < app.Documents.Size))
            {
                for (let doc_n: c_int = 0; doc_n < app.Documents.Size; doc_n++)
                {
                    MyDocument* doc = &app.Documents[doc_n];
                    if (!doc->Open)
                        if (MenuItem(doc.Name))
                            doc->DoOpen();
                }
                EndMenu();
            }
            if (MenuItem("Close All Documents", None, false, open_count > 0))
                for (let doc_n: c_int = 0; doc_n < app.Documents.Size; doc_n++)
                    app.Documents[doc_n].DoQueueClose();
            if (MenuItem("Exit", "Ctrl+F4") && p_open)
                *p_open = false;
            EndMenu();
        }
        EndMenuBar();
    }

    // [Debug] List documents with one checkbox for each
    for (let doc_n: c_int = 0; doc_n < app.Documents.Size; doc_n++)
    {
        MyDocument* doc = &app.Documents[doc_n];
        if doc_n > 0 {
            SameLine(); }
        PushID(doc);
        if (Checkbox(doc.Name, &doc->Open))
            if (!doc->Open)
                doc->DoForceClose();
        PopID();
    }
    PushItemWidth(GetFontSize() * 12);
    Combo("Output", (c_int*)&opt_target, "None\0TabBar+Tabs\0DockSpace+Window\0");
    PopItemWidth();
    let mut redock_all: bool =  false;
    if (opt_target == Target_Tab)                { SameLine(); Checkbox("Reorderable Tabs", &opt_reorderable); }
    if (opt_target == Target_DockSpaceAndWindow) { SameLine(); redock_all = Button("Redock all"); }

    Separator();

    // About the ImGuiWindowFlags_UnsavedDocument / ImGuiTabItemFlags_UnsavedDocument flags.
    // They have multiple effects:
    // - Display a dot next to the title.
    // - Tab is selected when clicking the X close button.
    // - Closure is not assumed (will wait for user to stop submitting the tab).
    //   Otherwise closure is assumed when pressing the X, so if you keep submitting the tab may reappear at end of tab bar.
    //   We need to assume closure by default otherwise waiting for "lack of submission" on the next frame would leave an empty
    //   hole for one-frame, both in the tab-bar and in tab-contents when closing a tab/window.
    //   The rarely used SetTabItemClosed() function is a way to notify of programmatic closure to avoid the one-frame hole.

    // Tabs
    if (opt_target == Target_Tab)
    {
        tab_bar_flags: ImGuiTabBarFlags = (opt_fitting_flags) | (if opt_reorderable { ImGuiTabBarFlags_Reorderable} else {0});
        if (BeginTabBar("##tabs", tab_bar_flags))
        {
            if opt_reorderable {
                NotifyOfDocumentsClosedElsewhere(app)(); }

            // [DEBUG] Stress tests
            //if ((GetFrameCount() % 30) == 0) docs[1].Open ^= 1;            // [DEBUG] Automatically show/hide a tab. Test various interactions e.g. dragging with this on.
            //if (GetIO().KeyCtrl) SetTabItemSelected(docs[1].Name);  // [DEBUG] Test SetTabItemSelected(), probably not very useful as-is anyway..

            // Submit Tabs
            for (let doc_n: c_int = 0; doc_n < app.Documents.Size; doc_n++)
            {
                MyDocument* doc = &app.Documents[doc_n];
                if (!doc->Open)
                    continue;

                tab_flags: ImGuiTabItemFlags = (if doc->Dirty { ImGuiTabItemFlags_UnsavedDocument} else {0});
                let mut visible: bool =  BeginTabItem(doc.Name, &doc->Open, tab_flags);

                // Cancel attempt to close when unsaved add to save queue so we can display a popup.
                if (!doc->Open && doc->Dirty)
                {
                    doc->Open = true;
                    doc->DoQueueClose();
                }

                MyDocument::DisplayContextMenu(doc);
                if (visible)
                {
                    MyDocument::DisplayContents(doc);
                    EndTabItem();
                }
            }

            EndTabBar();
        }
    }
    else if (opt_target == Target_DockSpaceAndWindow)
    {
        if (GetIO().ConfigFlags & ImGuiConfigFlags_DockingEnable)
        {
            NotifyOfDocumentsClosedElsewhere(app);

            // Create a DockSpace node where any window can be docked
            let mut dockspace_id: ImguiHandle =  GetID("MyDockSpace");
            DockSpace(dockspace_id);

            // Create Windows
            for (let doc_n: c_int = 0; doc_n < app.Documents.Size; doc_n++)
            {
                MyDocument* doc = &app.Documents[doc_n];
                if (!doc->Open)
                    continue;

                SetNextWindowDockID(dockspace_id, if redock_all {ImGuiCond_Always} else { ImGuiCond_FirstUseEver });
                window_flags: ImGuiWindowFlags = (if doc->Dirty { ImGuiWindowFlags_UnsavedDocument} else {0});
                let mut visible: bool =  Begin(doc.Name, &doc->Open, window_flags);

                // Cancel attempt to close when unsaved add to save queue so we can display a popup.
                if (!doc->Open && doc->Dirty)
                {
                    doc->Open = true;
                    doc->DoQueueClose();
                }

                MyDocument::DisplayContextMenu(doc);
                if (visible)
                    MyDocument::DisplayContents(doc);

                End();
            }
        }
        else
        {
            ShowDockingDisabledMessage();
        }
    }

    // Early out other contents
    if (!window_contents_visible)
    {
        End();
        return;
    }

    // Update closing queue
    static Vec<MyDocument*> close_queue;
    if (close_queue.empty())
    {
        // Close queue is locked once we started a popup
        for (let doc_n: c_int = 0; doc_n < app.Documents.Size; doc_n++)
        {
            MyDocument* doc = &app.Documents[doc_n];
            if (doc->WantClose)
            {
                doc->WantClose = false;
                close_queue.push(doc);
            }
        }
    }

    // Display closing confirmation UI
    if (!close_queue.empty())
    {
        let close_queue_unsaved_documents: c_int = 0;
        for (let n: c_int = 0; n < close_queue.Size; n++)
            if (close_queue[n]->Dirty)
                close_queue_unsaved_documents+= 1;

        if (close_queue_unsaved_documents == 0)
        {
            // Close documents when all are unsaved
            for (let n: c_int = 0; n < close_queue.Size; n++)
                close_queue[n]->DoForceClose();
            close_queue.clear();
        }
        else
        {
            if (!IsPopupOpen("Save?"))
                OpenPopup("Save?");
            if (BeginPopupModal("Save?", None, ImGuiWindowFlags_AlwaysAutoResize))
            {
                Text("Save change to the following items?");
                let item_height: c_float =  GetTextLineHeightWithSpacing();
                if (BeginChildFrame(GetID("frame"), ImVec2::new(-FLT_MIN, 6.25f * item_height)))
                {
                    for (let n: c_int = 0; n < close_queue.Size; n++)
                        if (close_queue[n]->Dirty)
                            Text("{}", close_queue[n].Name);
                    EndChildFrame();
                }

                button_size: ImVec2(GetFontSize() * 7.0, 0.0);
                if (Button("Yes", button_size))
                {
                    for (let n: c_int = 0; n < close_queue.Size; n++)
                    {
                        if (close_queue[n]->Dirty)
                            close_queue[n]->DoSave();
                        close_queue[n]->DoForceClose();
                    }
                    close_queue.clear();
                    CloseCurrentPopup();
                }
                SameLine();
                if (Button("No", button_size))
                {
                    for (let n: c_int = 0; n < close_queue.Size; n++)
                        close_queue[n]->DoForceClose();
                    close_queue.clear();
                    CloseCurrentPopup();
                }
                SameLine();
                if (Button("Cancel", button_size))
                {
                    close_queue.clear();
                    CloseCurrentPopup();
                }
                EndPopup();
            }
        }
    }

    End();
}

// End of Demo code
// #else

pub unsafe fn ShowAboutWindow(bool*) {}
pub unsafe fn ShowDemoWindow(bool*) {}
pub unsafe fn ShowUserGuide() {}
pub unsafe fn ShowStyleEditor(ImGuiStyle*) {}

// #endif

// #endif // #ifndef IMGUI_DISABLE
