// dear imgui, v1.89 WIP
// (internal structures/api)

// You may use this file to debug, understand or extend ImGui features but we don't provide any guarantee of forward compatibility!
// Set:
//   #define IMGUI_DEFINE_MATH_OPERATORS
// To implement maths operators for ImVec2 (disabled by default to not collide with using IM_VEC2_CLASS_EXTRA along with your own math types+operators)

/*

Index of this file:

// [SECTION] Header mess
// [SECTION] Forward declarations
// [SECTION] Context pointer
// [SECTION] STB libraries includes
// [SECTION] Macros
// [SECTION] Generic helpers
// [SECTION] ImDrawList support
// [SECTION] Widgets support: flags, enums, data structures
// [SECTION] Inputs support
// [SECTION] Clipper support
// [SECTION] Navigation support
// [SECTION] Columns support
// [SECTION] Multi-select support
// [SECTION] Docking support
// [SECTION] Viewport support
// [SECTION] Settings support
// [SECTION] Metrics, Debug tools
// [SECTION] Generic context hooks
// [SECTION] ImGuiContext (main imgui context)
// [SECTION] ImGuiWindowTempData, ImGuiWindow
// [SECTION] Tab bar, Tab item support
// [SECTION] Table support
// [SECTION] ImGui internal API
// [SECTION] ImFontAtlas internal API
// [SECTION] Test Engine specific hooks (imgui_test_engine)

*/

// #pragma once
// #ifndef IMGUI_DISABLE

//-----------------------------------------------------------------------------
// [SECTION] Header mess
//-----------------------------------------------------------------------------

// #ifndef IMGUI_VERSION
// #include "imgui.h"
// #endif

// #include <stdio.h>      // FILE*, sscanf
// #include <stdlib.h>     // NULL, malloc, free, qsort, atoi, atof
// #include <math.h>       // sqrtf, fabsf, fmodf, powf, floorf, ceilf, cosf, sinf
// #include <limits.h>     // INT_MIN, INT_MAX

// Enable SSE intrinsics if available
// #if (defined __SSE__ || defined __x86_64__ || defined _M_X64) && !defined(IMGUI_DISABLE_SSE)
// #define IMGUI_ENABLE_SSE
// #include <immintrin.h>
// #endif

// Visual Studio warnings
// #ifdef _MSC_VER
// #pragma warning (push)
// #pragma warning (disable: 4251)     // class 'xxx' needs to have dll-interface to be used by clients of struct 'xxx' // when IMGUI_API is set to__declspec(dllexport)
// #pragma warning (disable: 26812)    // The enum type 'xxx' is unscoped. Prefer 'enum class' over 'enum' (Enum.3). [MSVC Static Analyzer)
// #pragma warning (disable: 26495)    // [Static Analyzer] Variable 'XXX' is uninitialized. Always initialize a member variable (type.6).
// #if defined(_MSC_VER) && _MSC_VER >= 1922 // MSVC 2019 16.2 or later
// #pragma warning (disable: 5054)     // operator '|': deprecated between enumerations of different types
// #endif
// #endif

// Clang/GCC warnings with -Weverything
// #if defined(__clang__)
// #pragma clang diagnostic push
// #if __has_warning("-Wunknown-warning-option")
// #pragma clang diagnostic ignored "-Wunknown-warning-option"         // warning: unknown warning group 'xxx'
// #endif
// #pragma clang diagnostic ignored "-Wunknown-pragmas"                // warning: unknown warning group 'xxx'
// #pragma clang diagnostic ignored "-Wfloat-equal"                    // warning: comparing floating point with == or != is unsafe // storing and comparing against same constants ok, for ImFloorSigned()
// #pragma clang diagnostic ignored "-Wunused-function"                // for stb_textedit.h
// #pragma clang diagnostic ignored "-Wmissing-prototypes"             // for stb_textedit.h
// #pragma clang diagnostic ignored "-Wold-style-cast"
// #pragma clang diagnostic ignored "-Wzero-as-null-pointer-constant"
// #pragma clang diagnostic ignored "-Wdouble-promotion"
// #pragma clang diagnostic ignored "-Wimplicit-int-float-conversion"  // warning: implicit conversion from 'xxx' to 'float' may lose precision
// #pragma clang diagnostic ignored "-Wmissing-noreturn"               // warning: function 'xxx' could be declared with attribute 'noreturn'
// #elif defined(__GNUC__)
// #pragma GCC diagnostic push
// #pragma GCC diagnostic ignored "-Wpragmas"              // warning: unknown option after '#pragma GCC diagnostic' kind
// #pragma GCC diagnostic ignored "-Wclass-memaccess"      // [__GNUC__ >= 8] warning: 'memset/memcpy' clearing/writing an object of type 'xxxx' with no trivial copy-assignment; use assignment or value-initialization instead
// #endif

// Legacy defines
// #ifdef IMGUI_DISABLE_FORMAT_STRING_FUNCTIONS            // Renamed in 1.74
// #error Use IMGUI_DISABLE_DEFAULT_FORMAT_FUNCTIONS
// #endif
// #ifdef IMGUI_DISABLE_MATH_FUNCTIONS                     // Renamed in 1.74
// #error Use IMGUI_DISABLE_DEFAULT_MATH_FUNCTIONS
// #endif

// Enable stb_truetype by default unless FreeType is enabled.
// You can compile with both by defining both IMGUI_ENABLE_FREETYPE and IMGUI_ENABLE_STB_TRUETYPE together.
// #ifndef IMGUI_ENABLE_FREETYPE
// #define IMGUI_ENABLE_STB_TRUETYPE
// #endif

//-----------------------------------------------------------------------------
// [SECTION] Forward declarations
//-----------------------------------------------------------------------------

// struct ImBitVector;                 // Store 1-bit per value
// struct ImRect;                      // An axis-aligned rectangle (2 points)
// struct ImDrawDataBuilder;           // Helper to build a ImDrawData instance
// struct ImDrawListSharedData;        // Data shared between all ImDrawList instances
// struct ImGuiColorMod;               // Stacked color modifier, backup of modified data so we can restore it
// struct ImGuiContext;                // Main Dear ImGui context
// struct ImGuiContextHook;            // Hook for extensions like ImGuiTestEngine
// struct ImGuiDataTypeInfo;           // Type information associated to a ImGuiDataType enum
// struct ImGuiDockContext;            // Docking system context
// struct ImGuiDockRequest;            // Docking system dock/undock queued request
// struct ImGuiDockNode;               // Docking system node (hold a list of Windows OR two child dock nodes)
// struct ImGuiDockNodeSettings;       // Storage for a dock node in .ini file (we preserve those even if the associated dock node isn't active during the session)
// struct ImGuiGroupData;              // Stacked storage data for BeginGroup()/EndGroup()
// struct ImGuiInputTextState;         // Internal state of the currently focused/edited text input box
// struct ImGuiLastItemData;           // Status storage for last submitted items
// struct ImGuiMenuColumns;            // Simple column measurement, currently used for MenuItem() only
// struct ImGuiNavItemData;            // Result of a gamepad/keyboard directional navigation move query result
// struct ImGuiMetricsConfig;          // Storage for ShowMetricsWindow() and DebugNodeXXX() functions
// struct ImGuiNextWindowData;         // Storage for SetNextWindow** functions
// struct ImGuiNextItemData;           // Storage for SetNextItem** functions
// struct ImGuiOldColumnData;          // Storage data for a single column for legacy Columns() api
// struct ImGuiOldColumns;             // Storage data for a columns set for legacy Columns() api
// struct ImGuiPopupData;              // Storage for current popup stack
// struct ImGuiSettingsHandler;        // Storage for one type registered in the .ini file
// struct ImGuiStackSizes;             // Storage of stack sizes for debugging/asserting
// struct ImGuiStyleMod;               // Stacked style modifier, backup of modified data so we can restore it
// struct ImGuiTabBar;                 // Storage for a tab bar
// struct ImGuiTabItem;                // Storage for a tab item (within a tab bar)
// struct ImGuiTable;                  // Storage for a table
// struct ImGuiTableColumn;            // Storage for one column of a table
// struct ImGuiTableInstanceData;      // Storage for one instance of a same table
// struct ImGuiTableTempData;          // Temporary storage for one table (one per table in the stack), shared between tables.
// struct ImGuiTableSettings;          // Storage for a table .ini settings
// struct ImGuiTableColumnsSettings;   // Storage for a column .ini settings
// struct ImGuiWindow;                 // Storage for one window
// struct ImGuiWindowTempData;         // Temporary storage for one window (that's the data which in theory we could ditch at the end of the frame, in practice we currently keep it for each window)
// struct ImGuiWindowSettings;         // Storage for a window .ini settings (we keep one of those even if the actual window wasn't instanced during this session)

//-----------------------------------------------------------------------------
// [SECTION] Context pointer
// See implementation of this variable in imgui.cpp for comments and details.
//-----------------------------------------------------------------------------

// #ifndef GImGui
// extern IMGUI_API ImGuiContext* GImGui;  // Current implicit context pointer
// #endif

//-------------------------------------------------------------------------
// [SECTION] STB libraries includes
//-------------------------------------------------------------------------

// namespace ImStb
// {
//
// #undef STB_TEXTEDIT_STRING
// #undef STB_TEXTEDIT_CHARTYPE
// // #define STB_TEXTEDIT_STRING             ImGuiInputTextState
// // #define STB_TEXTEDIT_CHARTYPE           ImWchar
// // #define STB_TEXTEDIT_GETWIDTH_NEWLINE   (-1f32)
// // #define STB_TEXTEDIT_UNDOSTATECOUNT     99
// // #define STB_TEXTEDIT_UNDOCHARCOUNT      999
// // #include "imstb_textedit.h"
//
// } // namespace ImStb

//-----------------------------------------------------------------------------
// [SECTION] Macros
//-----------------------------------------------------------------------------

// Internal Drag and Drop payload types. String starting with '_' are reserved for Dear ImGui.
// #define IMGUI_PAYLOAD_TYPE_WINDOW       "_IMWINDOW"     // Payload == ImGuiWindow*

// Debug Printing Into TTY
// (since IMGUI_VERSION_NUM >= 18729: IMGUI_DEBUG_LOG was reworked into IMGUI_DEBUG_PRINTF (and removed framecount from it). If you were using a #define IMGUI_DEBUG_LOG please rename)
// #ifndef IMGUI_DEBUG_PRINTF
// #ifndef IMGUI_DISABLE_DEFAULT_FORMAT_FUNCTIONS
// #define IMGUI_DEBUG_PRINTF(_FMT,...)    printf(_FMT, __VA_ARGS__)
// #else
// #define IMGUI_DEBUG_PRINTF(_FMT,...)
// #endif
// #endif

// Debug Logging for ShowDebugLogWindow(). This is designed for relatively rare events so please don't spam.
// #define IMGUI_DEBUG_LOG(...)            ImGui::DebugLog(__VA_ARGS__);
// #define IMGUI_DEBUG_LOG_ACTIVEID(...)   do { if (g.DebugLogFlags & ImGuiDebugLogFlags_EventActiveId) IMGUI_DEBUG_LOG(__VA_ARGS__); } while (0)
// #define IMGUI_DEBUG_LOG_FOCUS(...)      do { if (g.DebugLogFlags & ImGuiDebugLogFlags_EventFocus)    IMGUI_DEBUG_LOG(__VA_ARGS__); } while (0)
// #define IMGUI_DEBUG_LOG_POPUP(...)      do { if (g.DebugLogFlags & ImGuiDebugLogFlags_EventPopup)    IMGUI_DEBUG_LOG(__VA_ARGS__); } while (0)
// #define IMGUI_DEBUG_LOG_NAV(...)        do { if (g.DebugLogFlags & ImGuiDebugLogFlags_EventNav)      IMGUI_DEBUG_LOG(__VA_ARGS__); } while (0)
// #define IMGUI_DEBUG_LOG_CLIPPER(...)    do { if (g.DebugLogFlags & ImGuiDebugLogFlags_EventClipper)  IMGUI_DEBUG_LOG(__VA_ARGS__); } while (0)
// #define IMGUI_DEBUG_LOG_IO(...)         do { if (g.DebugLogFlags & ImGuiDebugLogFlags_EventIO)       IMGUI_DEBUG_LOG(__VA_ARGS__); } while (0)
// #define IMGUI_DEBUG_LOG_DOCKING(...)    do { if (g.DebugLogFlags & ImGuiDebugLogFlags_EventDocking)  IMGUI_DEBUG_LOG(__VA_ARGS__); } while (0)
// #define IMGUI_DEBUG_LOG_VIEWPORT(...)   do { if (g.DebugLogFlags & ImGuiDebugLogFlags_EventViewport) IMGUI_DEBUG_LOG(__VA_ARGS__); } while (0)

// Static Asserts
// #define IM_STATIC_ASSERT(_COND)         static_assert(_COND, "")

// "Paranoid" Debug Asserts are meant to only be enabled during specific debugging/work, otherwise would slow down the code too much.
// We currently don't have many of those so the effect is currently negligible, but onward intent to add more aggressive ones in the code.
//#define IMGUI_DEBUG_PARANOID
// #ifdef IMGUI_DEBUG_PARANOID
// #define IM_ASSERT_PARANOID(_EXPR)       IM_ASSERT(_EXPR)
// #else
// #define IM_ASSERT_PARANOID(_EXPR)
// #endif

// Error handling
// Down the line in some frameworks/languages we would like to have a way to redirect those to the programmer and recover from more faults.
// #ifndef IM_ASSERT_USER_ERROR
// #define IM_ASSERT_USER_ERROR(_EXP,_MSG) IM_ASSERT((_EXP) && _MSG)   // Recoverable User Error
// #endif

// Misc Macros
// #define IM_PI                           3.14159265358979323846f
// #ifdef _WIN32
// #define IM_NEWLINE                      "\r\n"   // Play it nice with Windows users (Update: since 2018-05, Notepad finally appears to support Unix-style carriage returns!)
// #else
// #define IM_NEWLINE                      "\n"
// #endif
// #ifndef IM_TABSIZE                      // Until we move this to runtime and/or add proper tab support, at least allow users to compile-time override
// #define IM_TABSIZE                      (4)
// #endif
// #define IM_MEMALIGN(_OFF,_ALIGN)        (((_OF0f32) + ((_ALIGN) - 1)) & ~((_ALIGN) - 1))           // Memory align e.g. IM_ALIGN(0,4)=0, IM_ALIGN(1,4)=4, IM_ALIGN(4,4)=4, IM_ALIGN(5,4)=8
// #define IM_F32_TO_INT8_UNBOUND(_VAL)    (((_VAL) * 255f32 + ((_VAL)>=0 ? 0.5f32 : -0.5f32)))   // Unsaturated, for display purpose
// #define IM_F32_TO_INT8_SAT(_VAL)        ((ImSaturate(_VAL) * 255f32 + 0.5f32))               // Saturated, always output 0..255
// #define IM_FLOOR(_VAL)                  ((_VAL))                                    // ImFloor() is not inlined in MSVC debug builds
// #define IM_ROUND(_VAL)                  (((_VAL) + 0.5f32))                           //

// Enforce cdecl calling convention for functions called by the standard library, in case compilation settings changed the default to e.g. __vectorcall
// #ifdef _MSC_VER
// #define IMGUI_CDECL __cdecl
// #else
// #define IMGUI_CDECL
// #endif

// Warnings
// #if defined(_MSC_VER) && !defined(__clang__)
// #define IM_MSVC_WARNING_SUPPRESS(XXXX)  __pragma(warning(suppress: XXXX))
// #else
// #define IM_MSVC_WARNING_SUPPRESS(XXXX)
// #endif

// Debug Tools
// Use 'Metrics/Debugger->Tools->Item Picker' to break into the call-stack of a specific item.
// This will call IM_DEBUG_BREAK() which you may redefine yourself. See https://github.com/scottt/debugbreak for more reference.
// #ifndef IM_DEBUG_BREAK
// #if defined (_MSC_VER)
// #define IM_DEBUG_BREAK()    __debugbreak()
// #elif defined(__clang__)
// #define IM_DEBUG_BREAK()    __builtin_debugtrap()
// #elif defined(__GNUC__) && (defined(__i386__) || defined(__x86_64__))
// #define IM_DEBUG_BREAK()    __asm__ volatile("int $0x03")
// #elif defined(__GNUC__) && defined(__thumb__)
// #define IM_DEBUG_BREAK()    __asm__ volatile(".inst 0xde01")
// #elif defined(__GNUC__) && defined(__arm__) && !defined(__thumb__)
// #define IM_DEBUG_BREAK()    __asm__ volatile(".inst 0xe7f001f0");
// #else
// #define IM_DEBUG_BREAK()    IM_ASSERT(0)    // It is expected that you define IM_DEBUG_BREAK() into something that will break nicely in a debugger!
// #endif
// #endif // #ifndef IM_DEBUG_BREAK

//-----------------------------------------------------------------------------
// [SECTION] Generic helpers
// Note that the ImXXX helpers functions are lower-level than ImGui functions.
// ImGui functions or the ImGui context are never called/used from other ImXXX functions.
//-----------------------------------------------------------------------------
// - Helpers: Hashing
// - Helpers: Sorting
// - Helpers: Bit manipulation
// - Helpers: String
// - Helpers: Formatting
// - Helpers: UTF-8 <> wchar conversions
// - Helpers: ImVec2/ImVec4 operators
// - Helpers: Maths
// - Helpers: Geometry
// - Helper: ImVec1
// - Helper: ImVec2ih
// - Helper: ImRect
// - Helper: ImBitArray
// - Helper: ImBitVector
// - Helper: ImSpan<>, ImSpanAllocator<>
// - Helper: ImPool<>
// - Helper: ImChunkStream<>
//-----------------------------------------------------------------------------

// Helpers: Hashing
 ImGuiID       ImHashData(*const void data, size_t data_size, u32 seed = 0);
 ImGuiID       ImHashStr(*const char data, size_t data_size = 0, u32 seed = 0);

// Helpers: Sorting
// #ifndef ImQsort
static inline void      ImQsort(*mut void base, size_t count, size_t size_of_element, c_int(IMGUI_CDECL *compare_func)(void *mut const, void *mut const)) { if (count > 1) qsort(base, count, size_of_element, compare_func); }
// #endif

// Helpers: Color Blending
 u32         ImAlphaBlendColors(u32 col_a, u32 col_b);

// Helpers: Bit manipulation
static inline bool      ImIsPowerOfTwo(c_int v)           { return v != 0 && (v & (v - 1)) == 0; }
static inline bool      ImIsPowerOfTwo(ImU64 v)         { return v != 0 && (v & (v - 1)) == 0; }
static inline c_int       ImUpperPowerOfTwo(c_int v)        { v-= 1; v |= v >> 1; v |= v >> 2; v |= v >> 4; v |= v >> 8; v |= v >> 16; v+= 1; return v; }

// Helpers: String
 c_int           ImStricmp(*const char str1, *const char str2);
 c_int           ImStrnicmp(*const char str1, *const char str2, size_t count);
 void          ImStrncpy(*mut char dst, *const char src, size_t count);
 *mut char         ImStrdup(*const char str);
 *mut char         ImStrdupcpy(*mut char dst, *mut size_t p_dst_size, *const char str);
 *const char   ImStrchrRange(*const char str_begin, *const char str_end, char c);
 c_int           ImStrlenW(*const ImWchar str);
 *const char   ImStreolRange(*const char str, *const char str_end);                // End end-of-line
 *const ImWcharImStrbolW(*const ImWchar buf_mid_line, *const ImWchar buf_begin);   // Find beginning-of-line
 *const char   ImStristr(*const char haystack, *const char haystack_end, *const char needle, *const char needle_end);
 void          ImStrTrimBlanks(*mut char str);
 *const char   ImStrSkipBlank(*const char str);
static inline bool      ImCharIsBlankA(char c)          { return c == ' ' || c == '\t'; }
static inline bool      ImCharIsBlankW(c_uint c)  { return c == ' ' || c == '\t' || c == 0x3000; }

// Helpers: Formatting
 c_int           ImFormatString(*mut char buf, size_t buf_size, *const char fmt, ...) IM_FMTARGS(3);
 c_int           ImFormatStringV(*mut char buf, size_t buf_size, *const char fmt, va_list args) IM_FMTLIST(3);
 void          ImFormatStringToTempBuffer(*const *mut char out_buf, *const *mut char out_buf_end, *const char fmt, ...) IM_FMTARGS(3);
 void          ImFormatStringToTempBufferV(*const *mut char out_buf, *const *mut char out_buf_end, *const char fmt, va_list args) IM_FMTLIST(3);
 *const char   ImParseFormatFindStart(*const char format);
 *const char   ImParseFormatFindEnd(*const char format);
 *const char   ImParseFormatTrimDecorations(*const char format, *mut char buf, size_t buf_size);
 void          ImParseFormatSanitizeForPrinting(*const char fmt_in, *mut char fmt_out, size_t fmt_out_size);
 *const char   ImParseFormatSanitizeForScanning(*const char fmt_in, *mut char fmt_out, size_t fmt_out_size);
 c_int           ImParseFormatPrecision(*const char format, c_int default_value);

// Helpers: UTF-8 <> wchar conversions
 *const char   ImTextCharToUtf8(out_buf: [c_char;5], c_uint c);                                                      // return out_buf
 c_int           ImTextStrToUtf8(*mut char out_buf, c_int out_buf_size, *const ImWchar in_text, *const ImWchar in_text_end);   // return output UTF-8 bytes count
 c_int           ImTextCharFromUtf8(*mut c_uint out_char, *const char in_text, *const char in_text_end);               // read one character. return input UTF-8 bytes count
 c_int           ImTextStrFromUtf8(*mut ImWchar out_buf, c_int out_buf_size, *const char in_text, *const char in_text_end, *const *mut char in_remaining = NULL);   // return input UTF-8 bytes count
 c_int           ImTextCountCharsFromUtf8(*const char in_text, *const char in_text_end);                                 // return number of UTF-8 code-points (NOT bytes count)
 c_int           ImTextCountUtf8BytesFromChar(*const char in_text, *const char in_text_end);                             // return number of bytes to express one char in UTF-8
 c_int           ImTextCountUtf8BytesFromStr(*const ImWchar in_text, *const ImWchar in_text_end);                        // return number of bytes to express string in UTF-8

// Helpers: ImVec2/ImVec4 operators
// We are keeping those disabled by default so they don't leak in user space, to allow user enabling implicit cast operators between ImVec2 and their own types (using IM_VEC2_CLASS_EXTRA etc.)
// We unfortunately don't have a unary- operator for ImVec2 because this would needs to be defined inside the class itself.
// #ifdef IMGUI_DEFINE_MATH_OPERATORS
IM_MSVC_RUNTIME_CHECKS_OFF
static inline ImVec2 *mut operator(const ImVec2& lhs, const c_float rhs)              { return ImVec2(lhs.x * rhs, lhs.y * rhs); }
static inline ImVec2 operator/(const ImVec2& lhs, const c_float rhs)              { return ImVec2(lhs.x / rhs, lhs.y / rhs); }
static inline ImVec2 operator+(const ImVec2& lhs, const ImVec2& rhs)            { return ImVec2(lhs.x + rhs.x, lhs.y + rhs.y); }
static inline ImVec2 operator-(const ImVec2& lhs, const ImVec2& rhs)            { return ImVec2(lhs.x - rhs.x, lhs.y - rhs.y); }
static inline ImVec2 *mut operator(const ImVec2& lhs, const ImVec2& rhs)            { return ImVec2(lhs.x * rhs.x, lhs.y * rhs.y); }
static inline ImVec2 operator/(const ImVec2& lhs, const ImVec2& rhs)            { return ImVec2(lhs.x / rhs.x, lhs.y / rhs.y); }
static inline ImVec2& *mut operator=(ImVec2& lhs, const c_float rhs)                  { lhs.x *= rhs; lhs.y *= rhs; return lhs; }
static inline ImVec2& operator/=(ImVec2& lhs, const c_float rhs)                  { lhs.x /= rhs; lhs.y /= rhs; return lhs; }
static inline ImVec2& operator+=(ImVec2& lhs, const ImVec2& rhs)                { lhs.x += rhs.x; lhs.y += rhs.y; return lhs; }
static inline ImVec2& operator-=(ImVec2& lhs, const ImVec2& rhs)                { lhs.x -= rhs.x; lhs.y -= rhs.y; return lhs; }
static inline ImVec2& *mut operator=(ImVec2& lhs, const ImVec2& rhs)                { lhs.x *= rhs.x; lhs.y *= rhs.y; return lhs; }
static inline ImVec2& operator/=(ImVec2& lhs, const ImVec2& rhs)                { lhs.x /= rhs.x; lhs.y /= rhs.y; return lhs; }
static inline ImVec4 operator+(const ImVec4& lhs, const ImVec4& rhs)            { return ImVec4(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z, lhs.w + rhs.w); }
static inline ImVec4 operator-(const ImVec4& lhs, const ImVec4& rhs)            { return ImVec4(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z, lhs.w - rhs.w); }
static inline ImVec4 *mut operator(const ImVec4& lhs, const ImVec4& rhs)            { return ImVec4(lhs.x * rhs.x, lhs.y * rhs.y, lhs.z * rhs.z, lhs.w * rhs.w); }
IM_MSVC_RUNTIME_CHECKS_RESTORE
// #endif

// Helpers: File System
// #ifdef IMGUI_DISABLE_FILE_FUNCTIONS
// #define IMGUI_DISABLE_DEFAULT_FILE_FUNCTIONS
typedef *mut void ImFileHandle;
static inline ImFileHandle  ImFileOpen(*const char, *const char)                    { return NULL; }
static inline bool          ImFileClose(ImFileHandle)                               { return false; }
static inline ImU64         ImFileGetSize(ImFileHandle)                             { return (ImU64)-1; }
static inline ImU64         ImFileRead(*mut void, ImU64, ImU64, ImFileHandle)           { return 0; }
static inline ImU64         ImFileWrite(*const void, ImU64, ImU64, ImFileHandle)    { return 0; }
// #endif
// #ifndef IMGUI_DISABLE_DEFAULT_FILE_FUNCTIONS
typedef *mut FILE ImFileHandle;
 ImFileHandle      ImFileOpen(*const char filename, *const char mode);
 bool              ImFileClose(ImFileHandle file);
 ImU64             ImFileGetSize(ImFileHandle file);
 ImU64             ImFileRead(*mut void data, ImU64 size, ImU64 count, ImFileHandle file);
 ImU64             ImFileWrite(*const void data, ImU64 size, ImU64 count, ImFileHandle file);
// #else
// #define IMGUI_DISABLE_TTY_FUNCTIONS // Can't use stdout, fflush if we are not using default file functions
// #endif
 *mut void             ImFileLoadToMemory(*const char filename, *const char mode, *mut size_t out_file_size = NULL, c_int padding_bytes = 0);

// Helpers: Maths
IM_MSVC_RUNTIME_CHECKS_OFF
// - Wrapper for standard libs functions. (Note that imgui_demo.cpp does _not_ use them to keep the code easy to copy)
// #ifndef IMGUI_DISABLE_DEFAULT_MATH_FUNCTIONS
// #define ImFabs(X)           fabsf(X)
// #define ImSqrt(X)           sqrtf(X)
// #define ImFmod(X, Y)        fmodf((X), (Y))
// #define ImCos(X)            cosf(X)
// #define ImSin(X)            sinf(X)
// #define ImAcos(X)           acosf(X)
// #define ImAtan2(Y, X)       atan2f((Y), (X))
// #define ImAtof(STR)         atof(STR)
//#define ImFloorStd(X)     floorf(X)           // We use our own, see ImFloor() and ImFloorSigned()
// #define ImCeil(X)           ceilf(X)
static inline c_float  ImPow(c_float x, c_float y)    { return powf(x, y); }          // DragBehaviorT/SliderBehaviorT uses ImPow with either float/double and need the precision
static inline double ImPow(double x, double y)  { return pow(x, y); }
static inline c_float  ImLog(c_float x)             { return logf(x); }             // DragBehaviorT/SliderBehaviorT uses ImLog with either float/double and need the precision
static inline double ImLog(double x)            { return log(x); }
static inline c_int    ImAbs(c_int x)               { return x < 0 ? -x : x; }
static inline c_float  ImAbs(c_float x)             { return fabsf(x); }
static inline double ImAbs(double x)            { return fabs(x); }
static inline c_float  ImSign(c_float x)            { return (x < 0f32) ? -1f32 : (x > 0f32) ? 1f32 : 0f32; } // Sign operator - returns -1, 0 or 1 based on sign of argument
static inline double ImSign(double x)           { return (x < 0.0) ? -1.0 : (x > 0.0) ? 1.0 : 0.0; }
// #ifdef IMGUI_ENABLE_SSE
static inline c_float  ImRsqrt(c_float x)           { return _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x))); }
// #else
static inline c_float  ImRsqrt(c_float x)           { return 1f32 / sqrtf(x); }
// #endif
static inline double ImRsqrt(double x)          { return 1.0 / sqrt(x); }
// #endif
// - ImMin/ImMax/ImClamp/ImLerp/ImSwap are used by widgets which support variety of types: signed/unsigned int/long long float/double
// (Exceptionally using templates here but we could also redefine them for those types)
template<typename T> static inline T ImMin(T lhs, T rhs)                        { return lhs < rhs ? lhs : rhs; }
template<typename T> static inline T ImMax(T lhs, T rhs)                        { return lhs >= rhs ? lhs : rhs; }
template<typename T> static inline T ImClamp(T v, T mn, T mx)                   { return (v < mn) ? mn : (v > mx) ? mx : v; }
template<typename T> static inline T ImLerp(T a, T b, c_float t)                  { return (T)(a + (b - a) * t); }
template<typename T> static inline void ImSwap(T& a, T& b)                      { T tmp = a; a = b; b = tmp; }
template<typename T> static inline T ImAddClampOverflow(T a, T b, T mn, T mx)   { if (b < 0 && (a < mn - b)) return mn; if (b > 0 && (a > mx - b)) return mx; return a + b; }
template<typename T> static inline T ImSubClampOverflow(T a, T b, T mn, T mx)   { if (b > 0 && (a < mn + b)) return mn; if (b < 0 && (a > mx + b)) return mx; return a - b; }
// - Misc maths helpers
static inline ImVec2 ImMin(const ImVec2& lhs, const ImVec2& rhs)                { return ImVec2(lhs.x < rhs.x ? lhs.x : rhs.x, lhs.y < rhs.y ? lhs.y : rhs.y); }
static inline ImVec2 ImMax(const ImVec2& lhs, const ImVec2& rhs)                { return ImVec2(lhs.x >= rhs.x ? lhs.x : rhs.x, lhs.y >= rhs.y ? lhs.y : rhs.y); }
static inline ImVec2 ImClamp(const ImVec2& v, const ImVec2& mn, ImVec2 mx)      { return ImVec2((v.x < mn.x) ? mn.x : (v.x > mx.x) ? mx.x : v.x, (v.y < mn.y) ? mn.y : (v.y > mx.y) ? mx.y : v.y); }
static inline ImVec2 ImLerp(const ImVec2& a, const ImVec2& b, c_float t)          { return ImVec2(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t); }
static inline ImVec2 ImLerp(const ImVec2& a, const ImVec2& b, const ImVec2& t)  { return ImVec2(a.x + (b.x - a.x) * t.x, a.y + (b.y - a.y) * t.y); }
static inline ImVec4 ImLerp(const ImVec4& a, const ImVec4& b, c_float t)          { return ImVec4(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t, a.z + (b.z - a.z) * t, a.w + (b.w - a.w) * t); }
static inline c_float  ImSaturate(c_float 0f32)                                        { return (f < 0f32) ? 0f32 : (f > 1f32) ? 1f32 : f; }
static inline c_float  ImLengthSqr(const ImVec2& lhs)                             { return (lhs.x * lhs.x) + (lhs.y * lhs.y); }
static inline c_float  ImLengthSqr(const ImVec4& lhs)                             { return (lhs.x * lhs.x) + (lhs.y * lhs.y) + (lhs.z * lhs.z) + (lhs.w * lhs.w); }
static inline c_float  ImInvLength(const ImVec2& lhs, c_float fail_value)           { c_float d = (lhs.x * lhs.x) + (lhs.y * lhs.y); if (d > 0f32) return ImRsqrt(d); return fail_value; }
static inline c_float  ImFloor(c_float 0f32)                                           { return (0f32); }
static inline c_float  ImFloorSigned(c_float 0f32)                                     { return ((f >= 0 || f == 0f32) ? f : f - 1); } // Decent replacement for floorf()
static inline ImVec2 ImFloor(const ImVec2& v)                                   { return ImVec2((v.x), (v.y)); }
static inline ImVec2 ImFloorSigned(const ImVec2& v)                             { return ImVec2(ImFloorSigned(v.x), ImFloorSigned(v.y)); }
static inline c_int    ImModPositive(c_int a, c_int b)                                { return (a + b) % b; }
static inline c_float  ImDot(const ImVec2& a, const ImVec2& b)                    { return a.x * b.x + a.y * b.y; }
static inline ImVec2 ImRotate(const ImVec2& v, c_float cos_a, c_float sin_a)        { return ImVec2(v.x * cos_a - v.y * sin_a, v.x * sin_a + v.y * cos_a); }
static inline c_float  ImLinearSweep(c_float current, c_float target, c_float speed)    { if (current < target) return ImMin(current + speed, target); if (current > target) return ImMax(current - speed, target); return current; }
static inline ImVec2 ImMul(const ImVec2& lhs, const ImVec2& rhs)                { return ImVec2(lhs.x * rhs.x, lhs.y * rhs.y); }
static inline bool   ImIsFloatAboveGuaranteedIntegerPrecision(c_float 0f32)          { return f <= -16777216 || f >= 16777216; }
IM_MSVC_RUNTIME_CHECKS_RESTORE

// Helpers: Geometry
 ImVec2     ImBezierCubicCalc(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, c_float t);
 ImVec2     ImBezierCubicClosestPoint(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, const ImVec2& p, c_int num_segments);       // For curves with explicit number of segments
 ImVec2     ImBezierCubicClosestPointCasteljau(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, const ImVec2& p, c_float tess_tol);// For auto-tessellated curves you can use tess_tol = style.CurveTessellationTol
 ImVec2     ImBezierQuadraticCalc(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, c_float t);
 ImVec2     ImLineClosestPoint(const ImVec2& a, const ImVec2& b, const ImVec2& p);
 bool       ImTriangleContainsPoint(const ImVec2& a, const ImVec2& b, const ImVec2& c, const ImVec2& p);
 ImVec2     ImTriangleClosestPoint(const ImVec2& a, const ImVec2& b, const ImVec2& c, const ImVec2& p);
 void       ImTriangleBarycentricCoords(const ImVec2& a, const ImVec2& b, const ImVec2& c, const ImVec2& p, c_float& out_u, c_float& out_v, c_float& out_w);
inline c_float         ImTriangleArea(const ImVec2& a, const ImVec2& b, const ImVec2& c) { return ImFabs((a.x * (b.y - c.y)) + (b.x * (c.y - a.y)) + (c.x * (a.y - b.y))) * 0.5f32; }
 ImGuiDir   ImGetDirQuadrantFromDelta(c_float dx, c_float dy);

// Helper: ImVec1 (1D vector)
// (this odd construct is used to facilitate the transition between 1D and 2D, and the maintenance of some branches/patches)
IM_MSVC_RUNTIME_CHECKS_OFF
struct ImVec1
{
    c_float   x;
    constexpr ImVec1()         : x(0f32) { }
    constexpr ImVec1(c_float _x) : x(_x) { }
};



IM_MSVC_RUNTIME_CHECKS_RESTORE

// Helper: ImBitArray
inline bool     ImBitArrayTestBit(*const u32 arr, c_int n)      { u32 mask = 1 << (n & 31); return (arr[n >> 5] & mask) != 0; }
inline void     ImBitArrayClearBit(*mut u32 arr, c_int n)           { u32 mask = 1 << (n & 31); arr[n >> 5] &= ~mask; }
inline void     ImBitArraySetBit(*mut u32 arr, c_int n)             { u32 mask = 1 << (n & 31); arr[n >> 5] |= mask; }
inline void     ImBitArraySetBitRange(*mut u32 arr, c_int n, c_int n2) // Works on range [n..n2)
{
    n2-= 1;
    while (n <= n2)
    {
        c_int a_mod = (n & 31);
        c_int b_mod = (n2 > (n | 31) ? 31 : (n2 & 31)) + 1;
        u32 mask = (((ImU64)1 << b_mod) - 1) & ~(((ImU64)1 << a_mod) - 1);
        arr[n >> 5] |= mask;
        n = (n + 32) & ~31;
    }
}

// Helper: ImBitArray class (wrapper over ImBitArray functions)
// Store 1-bit per value.
template<c_int BITCOUNT, c_int OFFSET = 0>
struct ImBitArray
{
    u32           Storage[(BITCOUNT + 31) >> 5];
    ImBitArray()                                { ClearAllBits(); }
    void            ClearAllBits()              { memset(Storage, 0, sizeof(Storage)); }
    void            SetAllBits()                { memset(Storage, 255, sizeof(Storage)); }
    bool            TestBit(c_int n) const        { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); return ImBitArrayTestBit(Storage, n); }
    void            SetBit(c_int n)               { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); ImBitArraySetBit(Storage, n); }
    void            ClearBit(c_int n)             { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); ImBitArrayClearBit(Storage, n); }
    void            SetBitRange(c_int n, c_int n2)  { n += OFFSET; n2 += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT && n2 > n && n2 <= BITCOUNT); ImBitArraySetBitRange(Storage, n, n2); } // Works on range [n..n2)
    bool            operator[](c_int n) const     { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); return ImBitArrayTestBit(Storage, n); }
};

// Helper: ImBitVector
// Store 1-bit per value.
struct  ImBitVector
{
    Vec<u32> Storage;
    void            Create(c_int sz)              { Storage.resize((sz + 31) >> 5); memset(Storage.Data, 0, Storage.Size * sizeof(Storage.Data[0])); }
    void            Clear()                     { Storage.clear(); }
    bool            TestBit(c_int n) const        { IM_ASSERT(n < (Storage.Size << 5)); return ImBitArrayTestBit(Storage.Data, n); }
    void            SetBit(c_int n)               { IM_ASSERT(n < (Storage.Size << 5)); ImBitArraySetBit(Storage.Data, n); }
    void            ClearBit(c_int n)             { IM_ASSERT(n < (Storage.Size << 5)); ImBitArrayClearBit(Storage.Data, n); }
};

// Helper: ImSpan<>
// Pointing to a span of data we don't own.
template<typename T>
struct ImSpan
{
    *mut T                  Data;
    *mut T                  DataEnd;

    // Constructors, destructor
    inline ImSpan()                                 { Data = DataEnd = None; }
    inline ImSpan(*mut T data, c_int size)                { Data = data; DataEnd = data + size; }
    inline ImSpan(*mut T data, *mut T data_end)             { Data = data; DataEnd = data_end; }

    inline void         set(*mut T data, c_int size)      { Data = data; DataEnd = data + size; }
    inline void         set(*mut T data, *mut T data_end)   { Data = data; DataEnd = data_end; }
    inline c_int          size() const                { return (ptrdiff_t)(DataEnd - Data); }
    inline c_int          size_in_bytes() const       { return (ptrdiff_t)(DataEnd - Data) * sizeof(T); }
    inline T&           operator[](c_int i)           { *mut T p = Data + i; IM_ASSERT(p >= Data && p < DataEnd); return *p; }
    inline const T&     operator[](c_int i) const     { *const T p = Data + i; IM_ASSERT(p >= Data && p < DataEnd); return *p; }

    inline *mut T           begin()                     { return Data; }
    inline *const T     begin() const               { return Data; }
    inline *mut T           end()                       { return DataEnd; }
    inline *const T     end() const                 { return DataEnd; }

    // Utilities
    inline c_int  index_from_ptr(*const T it) const   { IM_ASSERT(it >= Data && it < DataEnd); const ptrdiff_t off = it - Data; return off; }
};

// Helper: ImSpanAllocator<>
// Facilitate storing multiple chunks into a single large block (the "arena")
// - Usage: call Reserve() N times, allocate GetArenaSizeInBytes() worth, pass it to SetArenaBasePtr(), call GetSpan() N times to retrieve the aligned ranges.
template<c_int CHUNKS>
struct ImSpanAllocator
{
    *mut char   BasePtr;
    c_int     CurrOff;
    c_int     CurrIdx;
    c_int     Offsets[CHUNKS];
    c_int     Sizes[CHUNKS];

    ImSpanAllocator()                               { memset(this, 0, sizeof(*this)); }
    inline void  Reserve(c_int n, size_t sz, c_int a=4) { IM_ASSERT(n == CurrIdx && n < CHUNKS); CurrOff = IM_MEMALIGN(CurrOff, a); Offsets[n] = CurrOff; Sizes[n] = sz; CurrIdx+= 1; CurrOff += sz; }
    inline c_int   GetArenaSizeInBytes()              { return CurrOff; }
    inline void  SetArenaBasePtr(*mut void base_ptr)    { BasePtr = (*mut char)base_ptr; }
    inline *mut void GetSpanPtrBegin(c_int n)             { IM_ASSERT(n >= 0 && n < CHUNKS && CurrIdx == CHUNKS); return (*mut void)(BasePtr + Offsets[n]); }
    inline *mut void GetSpanPtrEnd(c_int n)               { IM_ASSERT(n >= 0 && n < CHUNKS && CurrIdx == CHUNKS); return (*mut void)(BasePtr + Offsets[n] + Sizes[n]); }
    template<typename T>
    inline void  GetSpan(c_int n, ImSpan<T>* span)    { span->set((*mut T)GetSpanPtrBegin(n), (*mut T)GetSpanPtrEnd(n)); }
};

// Helper: ImChunkStream<>
// Build and iterate a contiguous stream of variable-sized structures.
// This is used by Settings to store persistent data while reducing allocation count.
// We store the chunk size first, and align the final size on 4 bytes boundaries.
// The tedious/zealous amount of casting is to avoid -Wcast-align warnings.
template<typename T>
struct ImChunkStream
{
    Vec<char>  Buf;

    void    clear()                     { Buf.clear(); }
    bool    empty() const               { return Buf.Size == 0; }
    c_int     size() const                { return Buf.Size; }
    *mut T      alloc_chunk(size_t sz)      { size_t HDR_SZ = 4; sz = IM_MEMALIGN(HDR_SZ + sz, 4u); c_int off = Buf.Size; Buf.resize(off + sz); ((*mut c_int)(*mut void)(Buf.Data + of0f32))[0] = sz; return (*mut T)(*mut void)(Buf.Data + off + HDR_SZ); }
    *mut T      begin()                     { size_t HDR_SZ = 4; if (!Buf.Data) return NULL; return (*mut T)(*mut void)(Buf.Data + HDR_SZ); }
    *mut T      next_chunk(*mut T p)            { size_t HDR_SZ = 4; IM_ASSERT(p >= begin() && p < end()); p = (*mut T)(*mut void)((*mut char)(*mut void)p + chunk_size(p)); if (p == (*mut T)(*mut void)((*mut char)end() + HDR_SZ)) return (*mut T)0; IM_ASSERT(p < end()); return p; }
    c_int     chunk_size(*const T p)      { return ((*const c_int)p)[-1]; }
    *mut T      end()                       { return (*mut T)(*mut void)(Buf.Data + Buf.Size); }
    c_int     offset_from_ptr(*const T p) { IM_ASSERT(p >= begin() && p < end()); const ptrdiff_t off = (*const char)p - Buf.Data; return off; }
    *mut T      ptr_from_offset(c_int of0f32)    { IM_ASSERT(off >= 4 && off < Buf.Size); return (*mut T)(*mut void)(Buf.Data + of0f32); }
    void    swap(ImChunkStream<T>& rhs) { rhs.Buf.swap(Bu0f32); }

};

//-----------------------------------------------------------------------------
// [SECTION] ImDrawList support
//-----------------------------------------------------------------------------

// ImDrawList: Helper function to calculate a circle's segment count given its radius and a "maximum error" value.
// Estimation of number of circle segment based on error is derived using method described in https://stackoverflow.com/a/2244088/15194693
// Number of segments (N) is calculated using equation:
//   N = ceil ( pi / acos(1 - error / r) )     where r > 0, error <= r
// Our equation is significantly simpler that one in the post thanks for choosing segment that is
// perpendicular to X axis. Follow steps in the article from this starting condition and you will
// will get this result.
//
// Rendering circles with an odd number of segments, while mathematically correct will produce
// asymmetrical results on the raster grid. Therefore we're rounding N to next even number (7->8, 8->8, 9->10 etc.)
// #define IM_ROUNDUP_TO_EVEN(_V)                                  ((((_V) + 1) / 2) * 2)
// #define IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MIN                     4
// #define IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MAX                     512
// #define IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC(_RAD,_MAXERROR)    ImClamp(IM_ROUNDUP_TO_EVEN(ImCeil(IM_PI / ImAcos(1 - ImMin((_MAXERROR), (_RAD)) / (_RAD)))), IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MIN, IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MAX)

// Raw equation from IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC rewritten for 'r' and 'error'.
// #define IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC_R(_N,_MAXERROR)    ((_MAXERROR) / (1 - ImCos(IM_PI / ImMax((_N), IM_PI))))
// #define IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC_ERROR(_N,_RAD)     ((1 - ImCos(IM_PI / ImMax((_N), IM_PI))) / (_RAD))

// ImDrawList: Lookup table size for adaptive arc drawing, cover full circle.
// #ifndef IM_DRAWLIST_ARCFAST_TABLE_SIZE
// #define IM_DRAWLIST_ARCFAST_TABLE_SIZE                          48 // Number of samples in lookup table.
// #endif
// #define IM_DRAWLIST_ARCFAST_SAMPLE_MAX                          IM_DRAWLIST_ARCFAST_TABLE_SIZE // Sample index _PathArcToFastEx() for 360 angle.

//-----------------------------------------------------------------------------
// [SECTION] Widgets support: flags, enums, data structures
//-----------------------------------------------------------------------------

// Transient per-window flags, reset at the beginning of the frame. For child window, inherited from parent on first Begin().
// This is going to be exposed in imgui.h when stabilized enough.
enum ImGuiItemFlags_
{
    // Controlled by user
    ImGuiItemFlags_None                     = 0,
    ImGuiItemFlags_NoTabStop                = 1 << 0,  // false     // Disable keyboard tabbing (FIXME: should merge with _NoNav)
    ImGuiItemFlags_ButtonRepeat             = 1 << 1,  // false     // Button() will return true multiple times based on io.KeyRepeatDelay and io.KeyRepeatRate settings.
    ImGuiItemFlags_Disabled                 = 1 << 2,  // false     // Disable interactions but doesn't affect visuals. See BeginDisabled()/EndDisabled(). See github.com/ocornut/imgui/issues/211
    ImGuiItemFlags_NoNav                    = 1 << 3,  // false     // Disable keyboard/gamepad directional navigation (FIXME: should merge with _NoTabStop)
    ImGuiItemFlags_NoNavDefaultFocus        = 1 << 4,  // false     // Disable item being a candidate for default focus (e.g. used by title bar items)
    ImGuiItemFlags_SelectableDontClosePopup = 1 << 5,  // false     // Disable MenuItem/Selectable() automatically closing their popup window
    ImGuiItemFlags_MixedValue               = 1 << 6,  // false     // [BETA] Represent a mixed/indeterminate value, generally multi-selection where values differ. Currently only supported by Checkbox() (later should support all sorts of widgets)
    ImGuiItemFlags_ReadOnly                 = 1 << 7,  // false     // [ALPHA] Allow hovering interactions but underlying value is not changed.

    // Controlled by widget code
    ImGuiItemFlags_Inputable                = 1 << 8,  // false     // [WIP] Auto-activate input mode when tab focused. Currently only used and supported by a few items before it becomes a generic feature.
};

// Storage for LastItem data
enum ImGuiItemStatusFlags_
{
    ImGuiItemStatusFlags_None               = 0,
    ImGuiItemStatusFlags_HoveredRect        = 1 << 0,   // Mouse position is within item rectangle (does NOT mean that the window is in correct z-order and can be hovered!, this is only one part of the most-common IsItemHovered test)
    ImGuiItemStatusFlags_HasDisplayRect     = 1 << 1,   // g.LastItemData.DisplayRect is valid
    ImGuiItemStatusFlags_Edited             = 1 << 2,   // Value exposed by item was edited in the current frame (should match the bool return value of most widgets)
    ImGuiItemStatusFlags_ToggledSelection   = 1 << 3,   // Set when Selectable(), TreeNode() reports toggling a selection. We can't report "Selected", only state changes, in order to easily handle clipping with less issues.
    ImGuiItemStatusFlags_ToggledOpen        = 1 << 4,   // Set when TreeNode() reports toggling their open state.
    ImGuiItemStatusFlags_HasDeactivated     = 1 << 5,   // Set if the widget/group is able to provide data for the ImGuiItemStatusFlags_Deactivated flag.
    ImGuiItemStatusFlags_Deactivated        = 1 << 6,   // Only valid if ImGuiItemStatusFlags_HasDeactivated is set.
    ImGuiItemStatusFlags_HoveredWindow      = 1 << 7,   // Override the HoveredWindow test to allow cross-window hover testing.
    ImGuiItemStatusFlags_FocusedByTabbing   = 1 << 8,   // Set when the Focusable item just got focused by Tabbing (FIXME: to be removed soon)

// #ifdef IMGUI_ENABLE_TEST_ENGINE
    ImGuiItemStatusFlags_Openable           = 1 << 20,  // Item is an openable (e.g. TreeNode)
    ImGuiItemStatusFlags_Opened             = 1 << 21,  //
    ImGuiItemStatusFlags_Checkable          = 1 << 22,  // Item is a checkable (e.g. CheckBox, MenuItem)
    ImGuiItemStatusFlags_Checked            = 1 << 23,  //
// #endif
};

// Extend ImGuiInputTextFlags_
enum ImGuiInputTextFlagsPrivate_
{
    // [Internal]
    ImGuiInputTextFlags_Multiline           = 1 << 26,  // For internal use by InputTextMultiline()
    ImGuiInputTextFlags_NoMarkEdited        = 1 << 27,  // For internal use by functions using InputText() before reformatting data
    ImGuiInputTextFlags_MergedItem          = 1 << 28,  // For internal use by TempInputText(), will skip calling ItemAdd(). Require bounding-box to strictly match.
};

// Extend ImGuiButtonFlags_
enum ImGuiButtonFlagsPrivate_
{
    ImGuiButtonFlags_PressedOnClick         = 1 << 4,   // return true on click (mouse down event)
    ImGuiButtonFlags_PressedOnClickRelease  = 1 << 5,   // [Default] return true on click + release on same item <-- this is what the majority of Button are using
    ImGuiButtonFlags_PressedOnClickReleaseAnywhere = 1 << 6, // return true on click + release even if the release event is not done while hovering the item
    ImGuiButtonFlags_PressedOnRelease       = 1 << 7,   // return true on release (default requires click+release)
    ImGuiButtonFlags_PressedOnDoubleClick   = 1 << 8,   // return true on double-click (default requires click+release)
    ImGuiButtonFlags_PressedOnDragDropHold  = 1 << 9,   // return true when held into while we are drag and dropping another item (used by e.g. tree nodes, collapsing headers)
    ImGuiButtonFlags_Repeat                 = 1 << 10,  // hold to repeat
    ImGuiButtonFlags_FlattenChildren        = 1 << 11,  // allow interactions even if a child window is overlapping
    ImGuiButtonFlags_AllowItemOverlap       = 1 << 12,  // require previous frame HoveredId to either match id or be null before being usable, use along with SetItemAllowOverlap()
    ImGuiButtonFlags_DontClosePopups        = 1 << 13,  // disable automatically closing parent popup on press // [UNUSED]
    //ImGuiButtonFlags_Disabled             = 1 << 14,  // disable interactions -> use BeginDisabled() or ImGuiItemFlags_Disabled
    ImGuiButtonFlags_AlignTextBaseLine      = 1 << 15,  // vertically align button to match text baseline - ButtonEx() only // FIXME: Should be removed and handled by SmallButton(), not possible currently because of DC.CursorPosPrevLine
    ImGuiButtonFlags_NoKeyModifiers         = 1 << 16,  // disable mouse interaction if a key modifier is held
    ImGuiButtonFlags_NoHoldingActiveId      = 1 << 17,  // don't set ActiveId while holding the mouse (ImGuiButtonFlags_PressedOnClick only)
    ImGuiButtonFlags_NoNavFocus             = 1 << 18,  // don't override navigation focus when activated
    ImGuiButtonFlags_NoHoveredOnFocus       = 1 << 19,  // don't report as hovered when nav focus is on this item
    ImGuiButtonFlags_PressedOnMask_         = ImGuiButtonFlags_PressedOnClick | ImGuiButtonFlags_PressedOnClickRelease | ImGuiButtonFlags_PressedOnClickReleaseAnywhere | ImGuiButtonFlags_PressedOnRelease | ImGuiButtonFlags_PressedOnDoubleClick | ImGuiButtonFlags_PressedOnDragDropHold,
    ImGuiButtonFlags_PressedOnDefault_      = ImGuiButtonFlags_PressedOnClickRelease,
};

// Extend ImGuiComboFlags_
enum ImGuiComboFlagsPrivate_
{
    ImGuiComboFlags_CustomPreview           = 1 << 20,  // enable BeginComboPreview()
};

// Extend ImGuiSliderFlags_
enum ImGuiSliderFlagsPrivate_
{
    ImGuiSliderFlags_Vertical               = 1 << 20,  // Should this slider be orientated vertically?
    ImGuiSliderFlags_ReadOnly               = 1 << 21,
};

// Extend ImGuiSelectableFlags_
enum ImGuiSelectableFlagsPrivate_
{
    // NB: need to be in sync with last value of ImGuiSelectableFlags_
    ImGuiSelectableFlags_NoHoldingActiveID      = 1 << 20,
    ImGuiSelectableFlags_SelectOnNav            = 1 << 21,  // (WIP) Auto-select when moved into. This is not exposed in public API as to handle multi-select and modifiers we will need user to explicitly control focus scope. May be replaced with a BeginSelection() API.
    ImGuiSelectableFlags_SelectOnClick          = 1 << 22,  // Override button behavior to react on Click (default is Click+Release)
    ImGuiSelectableFlags_SelectOnRelease        = 1 << 23,  // Override button behavior to react on Release (default is Click+Release)
    ImGuiSelectableFlags_SpanAvailWidth         = 1 << 24,  // Span all avail width even if we declared less for layout purpose. FIXME: We may be able to remove this (added in 6251d379, 2bcafc86 for menus)
    ImGuiSelectableFlags_DrawHoveredWhenHeld    = 1 << 25,  // Always show active when held, even is not hovered. This concept could probably be renamed/formalized somehow.
    ImGuiSelectableFlags_SetNavIdOnHover        = 1 << 26,  // Set Nav/Focus ID on mouse hover (used by MenuItem)
    ImGuiSelectableFlags_NoPadWithHalfSpacing   = 1 << 27,  // Disable padding each side with ItemSpacing * 0.5f32
};

// Extend ImGuiTreeNodeFlags_
enum ImGuiTreeNodeFlagsPrivate_
{
    ImGuiTreeNodeFlags_ClipLabelForTrailingButton = 1 << 20,
};

enum ImGuiSeparatorFlags_
{
    ImGuiSeparatorFlags_None                    = 0,
    ImGuiSeparatorFlags_Horizontal              = 1 << 0,   // Axis default to current layout type, so generally Horizontal unless e.g. in a menu bar
    ImGuiSeparatorFlags_Vertical                = 1 << 1,
    ImGuiSeparatorFlags_SpanAllColumns          = 1 << 2,
};

enum ImGuiTextFlags_
{
    ImGuiTextFlags_None                         = 0,
    ImGuiTextFlags_NoWidthForLargeClippedText   = 1 << 0,
};

enum ImGuiTooltipFlags_
{
    ImGuiTooltipFlags_None                      = 0,
    ImGuiTooltipFlags_OverridePreviousTooltip   = 1 << 0,   // Override will clear/ignore previously submitted tooltip (defaults to append)
};

// FIXME: this is in development, not exposed/functional as a generic feature yet.
// Horizontal/Vertical enums are fixed to 0/1 so they may be used to index ImVec2
enum ImGuiLayoutType_
{
    ImGuiLayoutType_Horizontal = 0,
    ImGuiLayoutType_Vertical = 1
};

enum ImGuiLogType
{
    ImGuiLogType_None = 0,
    ImGuiLogType_TTY,
    ImGuiLogType_File,
    ImGuiLogType_Buffer,
    ImGuiLogType_Clipboard,
};

// X/Y enums are fixed to 0/1 so they may be used to index ImVec2
enum ImGuiAxis
{
    ImGuiAxis_None = -1,
    ImGuiAxis_X = 0,
    ImGuiAxis_Y = 1
};

enum ImGuiPlotType
{
    ImGuiPlotType_Lines,
    ImGuiPlotType_Histogram,
};

enum ImGuiPopupPositionPolicy
{
    ImGuiPopupPositionPolicy_Default,
    ImGuiPopupPositionPolicy_ComboBox,
    ImGuiPopupPositionPolicy_Tooltip,
};

struct ImGuiDataTypeTempStorage
{
    ImU8        Data[8];        // Can fit any data up to ImGuiDataType_COUNT
};

// Type information associated to one ImGuiDataType. Retrieve with DataTypeGetInfo().
struct ImGuiDataTypeInfo
{
    size_t      Size;           // Size in bytes
    *const char Name;           // Short descriptive name for the type, for debugging
    *const char PrintFmt;       // Default printf format for the type
    *const char ScanFmt;        // Default scanf format for the type
};

// Extend ImGuiDataType_
enum ImGuiDataTypePrivate_
{
    ImGuiDataType_String = ImGuiDataType_COUNT + 1,
    ImGuiDataType_Pointer,
    ImGuiDataType_ID,
};





// Storage data for BeginComboPreview()/EndComboPreview()
struct  ImGuiComboPreviewData
{
    ImRect          PreviewRect;
    ImVec2          BackupCursorPos;
    ImVec2          BackupCursorMaxPos;
    ImVec2          BackupCursorPosPrevLine;
    c_float           BackupPrevLineTextBaseOffset;
    ImGuiLayoutType BackupLayout;

    ImGuiComboPreviewData() { memset(this, 0, sizeof(*this)); }
};



// Simple column measurement, currently used for MenuItem() only.. This is very short-sighted/throw-away code and NOT a generic helper.
struct  ImGuiMenuColumns
{
    u32       TotalWidth;
    u32       NextTotalWidth;
    ImU16       Spacing;
    ImU16       OffsetIcon;         // Always zero for now
    ImU16       OffsetLabel;        // Offsets are locked in Update()
    ImU16       OffsetShortcut;
    ImU16       OffsetMark;
    ImU16       Widths[4];          // Width of:   Icon, Label, Shortcut, Mark  (accumulators for current frame)

    ImGuiMenuColumns() { memset(this, 0, sizeof(*this)); }
    void        Update(c_float spacing, bool window_reappearing);
    c_float       DeclColumns(c_float w_icon, c_float w_label, c_float w_shortcut, c_float w_mark);
    void        CalcNextTotalWidth(bool update_offsets);
};

// Internal state of the currently focused/edited text input box
// For a given item ID, access with ImGui::GetInputTextState()
struct  ImGuiInputTextState
{
    ImGuiID                 ID;                     // widget id owning the text state
    c_int                     CurLenW, CurLenA;       // we need to maintain our buffer length in both UTF-8 and wchar format. UTF-8 length is valid even if TextA is not.
    Vec<ImWchar>       TextW;                  // edit buffer, we need to persist but can't guarantee the persistence of the user-provided buffer. so we copy into own buffer.
    Vec<char>          TextA;                  // temporary UTF8 buffer for callbacks and other operations. this is not updated in every code-path! size=capacity.
    Vec<char>          InitialTextA;           // backup of end-user buffer at the time of focus (in UTF-8, unaltered)
    bool                    TextAIsValid;           // temporary UTF8 buffer is not initially valid before we make the widget active (until then we pull the data from user argument)
    c_int                     BufCapacityA;           // end-user buffer capacity
    c_float                   ScrollX;                // horizontal scrolling/offset
    ImStb::STB_TexteditState Stb;                   // state for stb_textedit.h
    c_float                   CursorAnim;             // timer for cursor blink, reset on every user action so the cursor reappears immediately
    bool                    CursorFollow;           // set when we want scrolling to follow the current cursor position (not always!)
    bool                    SelectedAllMouseLock;   // after a double-click to select all, we ignore further mouse drags to update selection
    bool                    Edited;                 // edited this frame
    ImGuiInputTextFlags     Flags;                  // copy of InputText() flags

    ImGuiInputTextState()                   { memset(this, 0, sizeof(*this)); }
    void        ClearText()                 { CurLenW = CurLenA = 0; TextW[0] = 0; TextA[0] = 0; CursorClamp(); }
    void        ClearFreeMemory()           { TextW.clear(); TextA.clear(); InitialTextA.clear(); }
    c_int         GetUndoAvailCount() const   { return Stb.undostate.undo_point; }
    c_int         GetRedoAvailCount() const   { return STB_TEXTEDIT_UNDOSTATECOUNT - Stb.undostate.redo_point; }
    void        OnKeyPressed(c_int key);      // Cannot be inline because we call in code in stb_textedit.h implementation

    // Cursor & Selection
    void        CursorAnimReset()           { CursorAnim = -0.3f32; }                                   // After a user-input the cursor stays on for a while without blinking
    void        CursorClamp()               { Stb.cursor = ImMin(Stb.cursor, CurLenW); Stb.select_start = ImMin(Stb.select_start, CurLenW); Stb.select_end = ImMin(Stb.select_end, CurLenW); }
    bool        HasSelection() const        { return Stb.select_start != Stb.select_end; }
    void        ClearSelection()            { Stb.select_start = Stb.select_end = Stb.cursor; }
    c_int         GetCursorPos() const        { return Stb.cursor; }
    c_int         GetSelectionStart() const   { return Stb.select_start; }
    c_int         GetSelectionEnd() const     { return Stb.select_end; }
    void        SelectAll()                 { Stb.select_start = 0; Stb.cursor = Stb.select_end = CurLenW; Stb.has_preferred_x = 0; }
};


enum ImGuiNextWindowDataFlags_
{
    ImGuiNextWindowDataFlags_None               = 0,
    ImGuiNextWindowDataFlags_HasPos             = 1 << 0,
    ImGuiNextWindowDataFlags_HasSize            = 1 << 1,
    ImGuiNextWindowDataFlags_HasContentSize     = 1 << 2,
    ImGuiNextWindowDataFlags_HasCollapsed       = 1 << 3,
    ImGuiNextWindowDataFlags_HasSizeConstraint  = 1 << 4,
    ImGuiNextWindowDataFlags_HasFocus           = 1 << 5,
    ImGuiNextWindowDataFlags_HasBgAlpha         = 1 << 6,
    ImGuiNextWindowDataFlags_HasScroll          = 1 << 7,
    ImGuiNextWindowDataFlags_HasViewport        = 1 << 8,
    ImGuiNextWindowDataFlags_HasDock            = 1 << 9,
    ImGuiNextWindowDataFlags_HasWindowClass     = 1 << 10,
};

enum ImGuiNextItemDataFlags_
{
    ImGuiNextItemDataFlags_None     = 0,
    ImGuiNextItemDataFlags_HasWidth = 1 << 0,
    ImGuiNextItemDataFlags_HasOpen  = 1 << 1,
};



struct  ImGuiStackSizes
{
    c_short   SizeOfIDStack;
    c_short   SizeOfColorStack;
    c_short   SizeOfStyleVarStack;
    c_short   SizeOfFontStack;
    c_short   SizeOfFocusScopeStack;
    c_short   SizeOfGroupStack;
    c_short   SizeOfItemFlagsStack;
    c_short   SizeOfBeginPopupStack;
    c_short   SizeOfDisabledStack;

    ImGuiStackSizes() { memset(this, 0, sizeof(*this)); }
    void SetToCurrentState();
    void CompareWithCurrentState();
};

struct ImGuiShrinkWidthItem
{
    c_int         Index;
    c_float       Width;
    c_float       InitialWidth;
};

struct ImGuiPtrOrIndex
{
    *mut void       Ptr;            // Either field can be set, not both. e.g. Dock node tab bars are loose while BeginTabBar() ones are in a pool.
    c_int         Index;          // Usually index in a main pool.

    ImGuiPtrOrIndex(*mut void ptr)  { Ptr = ptr; Index = -1; }
    ImGuiPtrOrIndex(c_int index)  { Ptr = None; Index = index; }
};

//-----------------------------------------------------------------------------
// [SECTION] Inputs support
//-----------------------------------------------------------------------------



// Extend ImGuiKey_
enum ImGuiKeyPrivate_
{
    ImGuiKey_LegacyNativeKey_BEGIN  = 0,
    ImGuiKey_LegacyNativeKey_END    = 512,
    ImGuiKey_Keyboard_BEGIN         = ImGuiKey_NamedKey_BEGIN,
    ImGuiKey_Keyboard_END           = ImGuiKey_GamepadStart,
    ImGuiKey_Gamepad_BEGIN          = ImGuiKey_GamepadStart,
    ImGuiKey_Gamepad_END            = ImGuiKey_GamepadRStickDown + 1,
    ImGuiKey_Aliases_BEGIN          = ImGuiKey_MouseLeft,
    ImGuiKey_Aliases_END            = ImGuiKey_COUNT,

    // [Internal] Named shortcuts for Navigation
    ImGuiKey_NavKeyboardTweakSlow   = ImGuiKey_ModCtrl,
    ImGuiKey_NavKeyboardTweakFast   = ImGuiKey_ModShift,
    ImGuiKey_NavGamepadTweakSlow    = ImGuiKey_GamepadL1,
    ImGuiKey_NavGamepadTweakFast    = ImGuiKey_GamepadR1,
    ImGuiKey_NavGamepadActivate     = ImGuiKey_GamepadFaceDown,
    ImGuiKey_NavGamepadCancel       = ImGuiKey_GamepadFaceRight,
    ImGuiKey_NavGamepadMenu         = ImGuiKey_GamepadFaceLeft,
    ImGuiKey_NavGamepadInput        = ImGuiKey_GamepadFaceUp,
};

enum ImGuiInputEventType
{
    ImGuiInputEventType_None = 0,
    ImGuiInputEventType_MousePos,
    ImGuiInputEventType_MouseWheel,
    ImGuiInputEventType_MouseButton,
    ImGuiInputEventType_MouseViewport,
    ImGuiInputEventType_Key,
    ImGuiInputEventType_Text,
    ImGuiInputEventType_Focus,
    ImGuiInputEventType_COUNT
};



// FIXME: Structures in the union below need to be declared as anonymous unions appears to be an extension?
// Using ImVec2() would fail on Clang 'union member 'MousePos' has a non-trivial default constructor'
struct ImGuiInputEventMousePos      { c_float PosX, PosY; };
struct ImGuiInputEventMouseWheel    { c_float WheelX, WheelY; };
struct ImGuiInputEventMouseButton   { c_int Button; bool Down; };
struct ImGuiInputEventMouseViewport { ImGuiID HoveredViewportID; };
struct ImGuiInputEventKey           { ImGuiKey Key; bool Down; c_float AnalogValue; };
struct ImGuiInputEventText          { c_uint Char; };
struct ImGuiInputEventAppFocused    { bool Focused; };

// Flags for IsKeyPressedEx(). In upcoming feature this will be used more (and IsKeyPressedEx() renamed)
// Don't mistake with ImGuiInputTextFlags! (for ImGui::InputText() function)
enum ImGuiInputFlags_
{
    // Flags for IsKeyPressedEx()
    ImGuiInputFlags_None                = 0,
    ImGuiInputFlags_Repeat              = 1 << 0,   // Return true on successive repeats. Default for legacy IsKeyPressed(). NOT Default for legacy IsMouseClicked(). MUST BE == 1.
    ImGuiInputFlags_RepeatRateDefault   = 1 << 1,   // Repeat rate: Regular (default)
    ImGuiInputFlags_RepeatRateNavMove   = 1 << 2,   // Repeat rate: Fast
    ImGuiInputFlags_RepeatRateNavTweak  = 1 << 3,   // Repeat rate: Faster
    ImGuiInputFlags_RepeatRateMask_     = ImGuiInputFlags_RepeatRateDefault | ImGuiInputFlags_RepeatRateNavMove | ImGuiInputFlags_RepeatRateNavTweak,
};

//-----------------------------------------------------------------------------
// [SECTION] Clipper support
//-----------------------------------------------------------------------------

struct ImGuiListClipperRange
{
    c_int     Min;
    c_int     Max;
    bool    PosToIndexConvert;      // Begin/End are absolute position (will be converted to indices later)
    ImS8    PosToIndexOffsetMin;    // Add to Min after converting to indices
    ImS8    PosToIndexOffsetMax;    // Add to Min after converting to indices

    static ImGuiListClipperRange    FromIndices(c_int min, c_int max)                               { ImGuiListClipperRange r = { min, max, false, 0, 0 }; return r; }
    static ImGuiListClipperRange    FromPositions(c_float y1, c_float y2, c_int off_min, c_int off_max) { ImGuiListClipperRange r = { y1, y2, true, (ImS8)off_min, (ImS8)off_max }; return r; }
};



//-----------------------------------------------------------------------------
// [SECTION] Navigation support
//-----------------------------------------------------------------------------

enum ImGuiActivateFlags_
{
    ImGuiActivateFlags_None                 = 0,
    ImGuiActivateFlags_PreferInput          = 1 << 0,       // Favor activation that requires keyboard text input (e.g. for Slider/Drag). Default if keyboard is available.
    ImGuiActivateFlags_PreferTweak          = 1 << 1,       // Favor activation for tweaking with arrows or gamepad (e.g. for Slider/Drag). Default if keyboard is not available.
    ImGuiActivateFlags_TryToPreserveState   = 1 << 2,       // Request widget to preserve state if it can (e.g. InputText will try to preserve cursor/selection)
};

// Early work-in-progress API for ScrollToItem()
enum ImGuiScrollFlags_
{
    ImGuiScrollFlags_None                   = 0,
    ImGuiScrollFlags_KeepVisibleEdgeX       = 1 << 0,       // If item is not visible: scroll as little as possible on X axis to bring item back into view [default for X axis]
    ImGuiScrollFlags_KeepVisibleEdgeY       = 1 << 1,       // If item is not visible: scroll as little as possible on Y axis to bring item back into view [default for Y axis for windows that are already visible]
    ImGuiScrollFlags_KeepVisibleCenterX     = 1 << 2,       // If item is not visible: scroll to make the item centered on X axis [rarely used]
    ImGuiScrollFlags_KeepVisibleCenterY     = 1 << 3,       // If item is not visible: scroll to make the item centered on Y axis
    ImGuiScrollFlags_AlwaysCenterX          = 1 << 4,       // Always center the result item on X axis [rarely used]
    ImGuiScrollFlags_AlwaysCenterY          = 1 << 5,       // Always center the result item on Y axis [default for Y axis for appearing window)
    ImGuiScrollFlags_NoScrollParent         = 1 << 6,       // Disable forwarding scrolling to parent window if required to keep item/rect visible (only scroll window the function was applied to).
    ImGuiScrollFlags_MaskX_                 = ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_KeepVisibleCenterX | ImGuiScrollFlags_AlwaysCenterX,
    ImGuiScrollFlags_MaskY_                 = ImGuiScrollFlags_KeepVisibleEdgeY | ImGuiScrollFlags_KeepVisibleCenterY | ImGuiScrollFlags_AlwaysCenterY,
};

enum ImGuiNavHighlightFlags_
{
    ImGuiNavHighlightFlags_None             = 0,
    ImGuiNavHighlightFlags_TypeDefault      = 1 << 0,
    ImGuiNavHighlightFlags_TypeThin         = 1 << 1,
    ImGuiNavHighlightFlags_AlwaysDraw       = 1 << 2,       // Draw rectangular highlight if (g.NavId == id) _even_ when using the mouse.
    ImGuiNavHighlightFlags_NoRounding       = 1 << 3,
};

enum ImGuiNavMoveFlags_
{
    ImGuiNavMoveFlags_None                  = 0,
    ImGuiNavMoveFlags_LoopX                 = 1 << 0,   // On failed request, restart from opposite side
    ImGuiNavMoveFlags_LoopY                 = 1 << 1,
    ImGuiNavMoveFlags_WrapX                 = 1 << 2,   // On failed request, request from opposite side one line down (when NavDir==right) or one line up (when NavDir==left)
    ImGuiNavMoveFlags_WrapY                 = 1 << 3,   // This is not super useful but provided for completeness
    ImGuiNavMoveFlags_AllowCurrentNavId     = 1 << 4,   // Allow scoring and considering the current NavId as a move target candidate. This is used when the move source is offset (e.g. pressing PageDown actually needs to send a Up move request, if we are pressing PageDown from the bottom-most item we need to stay in place)
    ImGuiNavMoveFlags_AlsoScoreVisibleSet   = 1 << 5,   // Store alternate result in NavMoveResultLocalVisible that only comprise elements that are already fully visible (used by PageUp/PageDown)
    ImGuiNavMoveFlags_ScrollToEdgeY         = 1 << 6,   // Force scrolling to min/max (used by Home/End) // FIXME-NAV: Aim to remove or reword, probably unnecessary
    ImGuiNavMoveFlags_Forwarded             = 1 << 7,
    ImGuiNavMoveFlags_DebugNoResult         = 1 << 8,   // Dummy scoring for debug purpose, don't apply result
    ImGuiNavMoveFlags_FocusApi              = 1 << 9,
    ImGuiNavMoveFlags_Tabbing               = 1 << 10,  // == Focus + Activate if item is Inputable + DontChangeNavHighlight
    ImGuiNavMoveFlags_Activate              = 1 << 11,
    ImGuiNavMoveFlags_DontSetNavHighlight   = 1 << 12,  // Do not alter the visible state of keyboard vs mouse nav highlight
};





//-----------------------------------------------------------------------------
// [SECTION] Columns support
//-----------------------------------------------------------------------------

// Flags for internal's BeginColumns(). Prefix using BeginTable() nowadays!
enum ImGuiOldColumnFlags_
{
    ImGuiOldColumnFlags_None                    = 0,
    ImGuiOldColumnFlags_NoBorder                = 1 << 0,   // Disable column dividers
    ImGuiOldColumnFlags_NoResize                = 1 << 1,   // Disable resizing columns when clicking on the dividers
    ImGuiOldColumnFlags_NoPreserveWidths        = 1 << 2,   // Disable column width preservation when adjusting columns
    ImGuiOldColumnFlags_NoForceWithinWindow     = 1 << 3,   // Disable forcing columns to fit within window
    ImGuiOldColumnFlags_GrowParentContentsSize  = 1 << 4,   // (WIP) Restore pre-1.51 behavior of extending the parent window contents size but _without affecting the columns width at all_. Will eventually remove.

    // Obsolete names (will be removed)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    ImGuiColumnsFlags_None                      = ImGuiOldColumnFlags_None,
    ImGuiColumnsFlags_NoBorder                  = ImGuiOldColumnFlags_NoBorder,
    ImGuiColumnsFlags_NoResize                  = ImGuiOldColumnFlags_NoResize,
    ImGuiColumnsFlags_NoPreserveWidths          = ImGuiOldColumnFlags_NoPreserveWidths,
    ImGuiColumnsFlags_NoForceWithinWindow       = ImGuiOldColumnFlags_NoForceWithinWindow,
    ImGuiColumnsFlags_GrowParentContentsSize    = ImGuiOldColumnFlags_GrowParentContentsSize,
// #endif
};

struct ImGuiOldColumnData
{
    c_float               OffsetNorm;             // Column start offset, normalized 0.0 (far left) -> 1.0 (far right)
    c_float               OffsetNormBeforeResize;
    ImGuiOldColumnFlags Flags;                  // Not exposed
    ImRect              ClipRect;

    ImGuiOldColumnData() { memset(this, 0, sizeof(*this)); }
};

//-----------------------------------------------------------------------------
// [SECTION] Multi-select support
//-----------------------------------------------------------------------------

// #ifdef IMGUI_HAS_MULTI_SELECT
// <this is filled in 'range_select' branch>
// #endif // #ifdef IMGUI_HAS_MULTI_SELECT

//-----------------------------------------------------------------------------
// [SECTION] Docking support
//-----------------------------------------------------------------------------

// #define DOCKING_HOST_DRAW_CHANNEL_BG 0  // Dock host: background fill
// #define DOCKING_HOST_DRAW_CHANNEL_FG 1  // Dock host: decorations and contents

// #ifdef IMGUI_HAS_DOCK

// Extend ImGuiDockNodeFlags_
enum ImGuiDockNodeFlagsPrivate_
{
    // [Internal]
    ImGuiDockNodeFlags_DockSpace                = 1 << 10,  // Local, Saved  // A dockspace is a node that occupy space within an existing user window. Otherwise the node is floating and create its own window.
    ImGuiDockNodeFlags_CentralNode              = 1 << 11,  // Local, Saved  // The central node has 2 main properties: stay visible when empty, only use "remaining" spaces from its neighbor.
    ImGuiDockNodeFlags_NoTabBar                 = 1 << 12,  // Local, Saved  // Tab bar is completely unavailable. No triangle in the corner to enable it back.
    ImGuiDockNodeFlags_HiddenTabBar             = 1 << 13,  // Local, Saved  // Tab bar is hidden, with a triangle in the corner to show it again (NB: actual tab-bar instance may be destroyed as this is only used for single-window tab bar)
    ImGuiDockNodeFlags_NoWindowMenuButton       = 1 << 14,  // Local, Saved  // Disable window/docking menu (that one that appears instead of the collapse button)
    ImGuiDockNodeFlags_NoCloseButton            = 1 << 15,  // Local, Saved  //
    ImGuiDockNodeFlags_NoDocking                = 1 << 16,  // Local, Saved  // Disable any form of docking in this dockspace or individual node. (On a whole dockspace, this pretty much defeat the purpose of using a dockspace at all). Note: when turned on, existing docked nodes will be preserved.
    ImGuiDockNodeFlags_NoDockingSplitMe         = 1 << 17,  // [EXPERIMENTAL] Prevent another window/node from splitting this node.
    ImGuiDockNodeFlags_NoDockingSplitOther      = 1 << 18,  // [EXPERIMENTAL] Prevent this node from splitting another window/node.
    ImGuiDockNodeFlags_NoDockingOverMe          = 1 << 19,  // [EXPERIMENTAL] Prevent another window/node to be docked over this node.
    ImGuiDockNodeFlags_NoDockingOverOther       = 1 << 20,  // [EXPERIMENTAL] Prevent this node to be docked over another window or non-empty node.
    ImGuiDockNodeFlags_NoDockingOverEmpty       = 1 << 21,  // [EXPERIMENTAL] Prevent this node to be docked over an empty node (e.g. DockSpace with no other windows)
    ImGuiDockNodeFlags_NoResizeX                = 1 << 22,  // [EXPERIMENTAL]
    ImGuiDockNodeFlags_NoResizeY                = 1 << 23,  // [EXPERIMENTAL]
    ImGuiDockNodeFlags_SharedFlagsInheritMask_  = ~0,
    ImGuiDockNodeFlags_NoResizeFlagsMask_       = ImGuiDockNodeFlags_NoResize | ImGuiDockNodeFlags_NoResizeX | ImGuiDockNodeFlags_NoResizeY,
    ImGuiDockNodeFlags_LocalFlagsMask_          = ImGuiDockNodeFlags_NoSplit | ImGuiDockNodeFlags_NoResizeFlagsMask_ | ImGuiDockNodeFlags_AutoHideTabBar | ImGuiDockNodeFlags_DockSpace | ImGuiDockNodeFlags_CentralNode | ImGuiDockNodeFlags_NoTabBar | ImGuiDockNodeFlags_HiddenTabBar | ImGuiDockNodeFlags_NoWindowMenuButton | ImGuiDockNodeFlags_NoCloseButton | ImGuiDockNodeFlags_NoDocking,
    ImGuiDockNodeFlags_LocalFlagsTransferMask_  = ImGuiDockNodeFlags_LocalFlagsMask_ & ~ImGuiDockNodeFlags_DockSpace,  // When splitting those flags are moved to the inheriting child, never duplicated
    ImGuiDockNodeFlags_SavedFlagsMask_          = ImGuiDockNodeFlags_NoResizeFlagsMask_ | ImGuiDockNodeFlags_DockSpace | ImGuiDockNodeFlags_CentralNode | ImGuiDockNodeFlags_NoTabBar | ImGuiDockNodeFlags_HiddenTabBar | ImGuiDockNodeFlags_NoWindowMenuButton | ImGuiDockNodeFlags_NoCloseButton | ImGuiDockNodeFlags_NoDocking
};

// Store the source authority (dock node vs window) of a field
enum ImGuiDataAuthority_
{
    ImGuiDataAuthority_Auto,
    ImGuiDataAuthority_DockNode,
    ImGuiDataAuthority_Window,
};

enum ImGuiDockNodeState
{
    ImGuiDockNodeState_Unknown,
    ImGuiDockNodeState_HostWindowHiddenBecauseSingleWindow,
    ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing,
    ImGuiDockNodeState_HostWindowVisible,
};

// List of colors that are stored at the time of Begin() into Docked Windows.
// We currently store the packed colors in a simple array window.DockStyle.Colors[].
// A better solution may involve appending into a log of colors in ImGuiContext + store offsets into those arrays in ImGuiWindow,
// but it would be more complex as we'd need to double-buffer both as e.g. drop target may refer to window from last frame.
enum ImGuiWindowDockStyleCol
{
    ImGuiWindowDockStyleCol_Text,
    ImGuiWindowDockStyleCol_Tab,
    ImGuiWindowDockStyleCol_TabHovered,
    ImGuiWindowDockStyleCol_TabActive,
    ImGuiWindowDockStyleCol_TabUnfocused,
    ImGuiWindowDockStyleCol_TabUnfocusedActive,
    ImGuiWindowDockStyleCol_COUNT
};



struct ImGuiDockContext
{
    ImGuiStorage                    Nodes;          // Map ID -> ImGuiDockNode*: Active nodes
    Vec<ImGuiDockRequest>      Requests;
    Vec<ImGuiDockNodeSettings> NodesSettings;
    bool                            WantFullRebuild;
    ImGuiDockContext()              { memset(this, 0, sizeof(*this)); }
};

// #endif // #ifdef IMGUI_HAS_DOCK

//-----------------------------------------------------------------------------
// [SECTION] Viewport support
//-----------------------------------------------------------------------------

// ImGuiViewport Private/Internals fields (cardinal sin: we are using inheritance!)
// Every instance of ImGuiViewport is in fact a ImGuiViewportP.

//-----------------------------------------------------------------------------
// [SECTION] Settings support
//-----------------------------------------------------------------------------

// Windows data saved in imgui.ini file
// Because we never destroy or rename ImGuiWindowSettings, we can store the names in a separate buffer easily.
// (this is designed to be stored in a ImChunkStream buffer, with the variable-length Name following our structure)
struct ImGuiWindowSettings
{
    ImGuiID     ID;
    ImVec2ih    Pos;            // NB: Settings position are stored RELATIVE to the viewport! Whereas runtime ones are absolute positions.
    ImVec2ih    Size;
    ImVec2ih    ViewportPos;
    ImGuiID     ViewportId;
    ImGuiID     DockId;         // ID of last known DockNode (even if the DockNode is invisible because it has only 1 active window), or 0 if none.
    ImGuiID     ClassId;        // ID of window class if specified
    c_short       DockOrder;      // Order of the last time the window was visible within its DockNode. This is used to reorder windows that are reappearing on the same frame. Same value between windows that were active and windows that were none are possible.
    bool        Collapsed;
    bool        WantApply;      // Set when loaded from .ini data (to enable merging/loading .ini data into an already running context)

    ImGuiWindowSettings()       { memset(this, 0, sizeof(*this)); DockOrder = -1; }
    *mut char GetName()             { return (*mut char)(this + 1); }
};

struct ImGuiSettingsHandler
{
    *const char TypeName;       // Short description stored in .ini file. Disallowed characters: '[' ']'
    ImGuiID     TypeHash;       // == ImHashStr(TypeName)
    void        (*ClearAllFn)(*mut ImGuiContext ctx, *mut ImGuiSettingsHandler handler);                                // Clear all settings data
    void        (*ReadInitFn)(*mut ImGuiContext ctx, *mut ImGuiSettingsHandler handler);                                // Read: Called before reading (in registration order)
    *mut void       (*ReadOpenFn)(*mut ImGuiContext ctx, *mut ImGuiSettingsHandler handler, *const char name);              // Read: Called when entering into a new ini entry e.g. "[Window][Name]"
    void        (*ReadLineFn)(*mut ImGuiContext ctx, *mut ImGuiSettingsHandler handler, *mut void entry, *const char line); // Read: Called for every line of text within an ini entry
    void        (*ApplyAllFn)(*mut ImGuiContext ctx, *mut ImGuiSettingsHandler handler);                                // Read: Called after reading (in registration order)
    void        (*WriteAllFn)(*mut ImGuiContext ctx, *mut ImGuiSettingsHandler handler, *mut ImGuiTextBuffer out_bu0f32);      // Write: Output every entries into 'out_buf'
    *mut void       UserData;

    ImGuiSettingsHandler() { memset(this, 0, sizeof(*this)); }
};

//-----------------------------------------------------------------------------
// [SECTION] Metrics, Debug Tools
//-----------------------------------------------------------------------------

enum ImGuiDebugLogFlags_
{
    // Event types
    ImGuiDebugLogFlags_None             = 0,
    ImGuiDebugLogFlags_EventActiveId    = 1 << 0,
    ImGuiDebugLogFlags_EventFocus       = 1 << 1,
    ImGuiDebugLogFlags_EventPopup       = 1 << 2,
    ImGuiDebugLogFlags_EventNav         = 1 << 3,
    ImGuiDebugLogFlags_EventClipper     = 1 << 4,
    ImGuiDebugLogFlags_EventIO          = 1 << 5,
    ImGuiDebugLogFlags_EventDocking     = 1 << 6,
    ImGuiDebugLogFlags_EventViewport    = 1 << 7,
    ImGuiDebugLogFlags_EventMask_       = ImGuiDebugLogFlags_EventActiveId | ImGuiDebugLogFlags_EventFocus | ImGuiDebugLogFlags_EventPopup | ImGuiDebugLogFlags_EventNav | ImGuiDebugLogFlags_EventClipper | ImGuiDebugLogFlags_EventIO | ImGuiDebugLogFlags_EventDocking | ImGuiDebugLogFlags_EventViewport,
    ImGuiDebugLogFlags_OutputToTTY      = 1 << 10,  // Also send output to TTY
};

struct ImGuiMetricsConfig
{
    bool        ShowDebugLog;
    bool        ShowStackTool;
    bool        ShowWindowsRects;
    bool        ShowWindowsBeginOrder;
    bool        ShowTablesRects;
    bool        ShowDrawCmdMesh;
    bool        ShowDrawCmdBoundingBoxes;
    bool        ShowDockingNodes;
    c_int         ShowWindowsRectsType;
    c_int         ShowTablesRectsType;

    ImGuiMetricsConfig()
    {
        ShowDebugLog = ShowStackTool = ShowWindowsRects = ShowWindowsBeginOrder = ShowTablesRects = false;
        ShowDrawCmdMesh = true;
        ShowDrawCmdBoundingBoxes = true;
        ShowDockingNodes = false;
        ShowWindowsRectsType = ShowTablesRectsType = -1;
    }
};

struct ImGuiStackLevelInfo
{
    ImGuiID                 ID;
    ImS8                    QueryFrameCount;            // >= 1: Query in progress
    bool                    QuerySuccess;               // Obtained result from DebugHookIdInfo()
    ImGuiDataType           DataType : 8;
    Desc: [c_char;57];                   // Arbitrarily sized buffer to hold a result (FIXME: could replace Results[] with a chunk stream?) FIXME: Now that we added CTRL+C this should be fixed.

    ImGuiStackLevelInfo()   { memset(this, 0, sizeof(*this)); }
};

// State for Stack tool queries
struct ImGuiStackTool
{
    c_int                     LastActiveFrame;
    c_int                     StackLevel;                 // -1: query stack and resize Results, >= 0: individual stack level
    ImGuiID                 QueryId;                    // ID to query details for
    Vec<ImGuiStackLevelInfo> Results;
    bool                    CopyToClipboardOnCtrlC;
    c_float                   CopyToClipboardLastTime;

    ImGuiStackTool()        { memset(this, 0, sizeof(*this)); CopyToClipboardLastTime = -f32::MAX; }
};

//-----------------------------------------------------------------------------
// [SECTION] Generic context hooks
//-----------------------------------------------------------------------------

typedef void (*ImGuiContextHookCallback)(*mut ImGuiContext ctx, *mut ImGuiContextHook hook);
enum ImGuiContextHookType { ImGuiContextHookType_NewFramePre, ImGuiContextHookType_NewFramePost, ImGuiContextHookType_EndFramePre, ImGuiContextHookType_EndFramePost, ImGuiContextHookType_RenderPre, ImGuiContextHookType_RenderPost, ImGuiContextHookType_Shutdown, ImGuiContextHookType_PendingRemoval_ };

struct ImGuiContextHook
{
    ImGuiID                     HookId;     // A unique ID assigned by AddContextHook()
    ImGuiContextHookType        Type;
    ImGuiID                     Owner;
    ImGuiContextHookCallback    Callback;
    *mut void                       UserData;

    ImGuiContextHook()          { memset(this, 0, sizeof(*this)); }
};


//-----------------------------------------------------------------------------
// [SECTION] ImGuiWindowTempData, ImGuiWindow
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] Tab bar, Tab item support
//-----------------------------------------------------------------------------

// Extend ImGuiTabBarFlags_
enum ImGuiTabBarFlagsPrivate_
{
    ImGuiTabBarFlags_DockNode                   = 1 << 20,  // Part of a dock node [we don't use this in the master branch but it facilitate branch syncing to keep this around]
    ImGuiTabBarFlags_IsFocused                  = 1 << 21,
    ImGuiTabBarFlags_SaveSettings               = 1 << 22,  // FIXME: Settings are handled by the docking system, this only request the tab bar to mark settings dirty when reordering tabs
};

// Extend ImGuiTabItemFlags_
enum ImGuiTabItemFlagsPrivate_
{
    ImGuiTabItemFlags_SectionMask_              = ImGuiTabItemFlags_Leading | ImGuiTabItemFlags_Trailing,
    ImGuiTabItemFlags_NoCloseButton             = 1 << 20,  // Track whether p_open was set or not (we'll need this info on the next frame to recompute ContentWidth during layout)
    ImGuiTabItemFlags_Button                    = 1 << 21,  // Used by TabItemButton, change the tab item behavior to mimic a button
    ImGuiTabItemFlags_Unsorted                  = 1 << 22,  // [Docking] Trailing tabs with the _Unsorted flag will be sorted based on the DockOrder of their Window.
    ImGuiTabItemFlags_Preview                   = 1 << 23,  // [Docking] Display tab shape for docking preview (height is adjusted slightly to compensate for the yet missing tab bar)
};

// Storage for one active tab item (sizeof() 48 bytes)
struct ImGuiTabItem
{
    ImGuiID             ID;
    ImGuiTabItemFlags   Flags;
    *mut ImGuiWindow        Window;                 // When TabItem is part of a DockNode's TabBar, we hold on to a window.
    c_int                 LastFrameVisible;
    c_int                 LastFrameSelected;      // This allows us to infer an ordered list of the last activated tabs with little maintenance
    c_float               Offset;                 // Position relative to beginning of tab
    c_float               Width;                  // Width currently displayed
    c_float               ContentWidth;           // Width of label, stored during BeginTabItem() call
    c_float               RequestedWidth;         // Width optionally requested by caller, -1f32 is unused
    ImS32               NameOffset;             // When Window==NULL, offset to name within parent ImGuiTabBar::TabsNames
    ImS16               BeginOrder;             // BeginTabItem() order, used to re-order tabs after toggling ImGuiTabBarFlags_Reorderable
    ImS16               IndexDuringLayout;      // Index only used during TabBarLayout()
    bool                WantClose;              // Marked as closed by SetTabItemClosed()

    ImGuiTabItem()      { memset(this, 0, sizeof(*this)); LastFrameVisible = LastFrameSelected = -1; RequestedWidth = -1f32; NameOffset = -1; BeginOrder = IndexDuringLayout = -1; }
};

// Storage for a tab bar (sizeof() 152 bytes)
struct  ImGuiTabBar
{
    Vec<ImGuiTabItem> Tabs;
    ImGuiTabBarFlags    Flags;
    ImGuiID             ID;                     // Zero for tab-bars used by docking
    ImGuiID             SelectedTabId;          // Selected tab/window
    ImGuiID             NextSelectedTabId;      // Next selected tab/window. Will also trigger a scrolling animation
    ImGuiID             VisibleTabId;           // Can occasionally be != SelectedTabId (e.g. when previewing contents for CTRL+TAB preview)
    c_int                 CurrFrameVisible;
    c_int                 PrevFrameVisible;
    ImRect              BarRect;
    c_float               CurrTabsContentsHeight;
    c_float               PrevTabsContentsHeight; // Record the height of contents submitted below the tab bar
    c_float               WidthAllTabs;           // Actual width of all tabs (locked during layout)
    c_float               WidthAllTabsIdeal;      // Ideal width if all tabs were visible and not clipped
    c_float               ScrollingAnim;
    c_float               ScrollingTarget;
    c_float               ScrollingTargetDistToVisibility;
    c_float               ScrollingSpeed;
    c_float               ScrollingRectMinX;
    c_float               ScrollingRectMaxX;
    ImGuiID             ReorderRequestTabId;
    ImS16               ReorderRequestOffset;
    ImS8                BeginCount;
    bool                WantLayout;
    bool                VisibleTabWasSubmitted;
    bool                TabsAddedNew;           // Set to true when a new tab item or button has been added to the tab bar during last frame
    ImS16               TabsActiveCount;        // Number of tabs submitted this frame.
    ImS16               LastTabItemIdx;         // Index of last BeginTabItem() tab for use by EndTabItem()
    c_float               ItemSpacingY;
    ImVec2              FramePadding;           // style.FramePadding locked at the time of BeginTabBar()
    ImVec2              BackupCursorPos;
    ImGuiTextBuffer     TabsNames;              // For non-docking tab bar we re-append names in a contiguous buffer.

    ImGuiTabBar();
    c_int                 GetTabOrder(*const ImGuiTabItem tab) const  { return Tabs.index_from_ptr(tab); }
    *const char         GetTabName(*const ImGuiTabItem tab) const
    {
        if (tab->Window)
            return tab->window.Name;
        IM_ASSERT(tab->NameOffset != -1 && tab->NameOffset < TabsNames.Buf.Size);
        return TabsNames.Buf.Data + tab->NameOffset;
    }
};

//-----------------------------------------------------------------------------
// [SECTION] Table support
//-----------------------------------------------------------------------------

// #define IM_COL32_DISABLE                IM_COL32(0,0,0,1)   // Special sentinel code which cannot be used as a regular color.
// #define IMGUI_TABLE_MAX_COLUMNS         64                  // sizeof(ImU64) * 8. This is solely because we frequently encode columns set in a ImU64.
// #define IMGUI_TABLE_MAX_DRAW_CHANNELS   (4 + 64 * 2)        // See TableSetupDrawChannels()

// Our current column maximum is 64 but we may raise that in the future.
typedef ImS8 ImGuiTableColumnIdx;
typedef ImU8 ImGuiTableDrawChannelIdx;

// [Internal] sizeof() ~ 104
// We use the terminology "Enabled" to refer to a column that is not Hidden by user/api.
// We use the terminology "Clipped" to refer to a column that is out of sight because of scrolling/clipping.
// This is in contrast with some user-facing api such as IsItemVisible() / IsRectVisible() which use "Visible" to mean "not clipped".
struct ImGuiTableColumn
{
    ImGuiTableColumnFlags   Flags;                          // Flags after some patching (not directly same as provided by user). See ImGuiTableColumnFlags_
    c_float                   WidthGiven;                     // Final/actual width visible == (MaxX - MinX), locked in TableUpdateLayout(). May be > WidthRequest to honor minimum width, may be < WidthRequest to honor shrinking columns down in tight space.
    c_float                   MinX;                           // Absolute positions
    c_float                   MaxX;
    c_float                   WidthRequest;                   // Master width absolute value when !(Flags & _WidthStretch). When Stretch this is derived every frame from StretchWeight in TableUpdateLayout()
    c_float                   WidthAuto;                      // Automatic width
    c_float                   StretchWeight;                  // Master width weight when (Flags & _WidthStretch). Often around ~1f32 initially.
    c_float                   InitStretchWeightOrWidth;       // Value passed to TableSetupColumn(). For Width it is a content width (_without padding_).
    ImRect                  ClipRect;                       // Clipping rectangle for the column
    ImGuiID                 UserID;                         // Optional, value passed to TableSetupColumn()
    c_float                   WorkMinX;                       // Contents region min ~(MinX + CellPaddingX + CellSpacingX1) == cursor start position when entering column
    c_float                   WorkMaxX;                       // Contents region max ~(MaxX - CellPaddingX - CellSpacingX2)
    c_float                   ItemWidth;                      // Current item width for the column, preserved across rows
    c_float                   ContentMaxXFrozen;              // Contents maximum position for frozen rows (apart from headers), from which we can infer content width.
    c_float                   ContentMaxXUnfrozen;
    c_float                   ContentMaxXHeadersUsed;         // Contents maximum position for headers rows (regardless of freezing). TableHeader() automatically softclip itself + report ideal desired size, to avoid creating extraneous draw calls
    c_float                   ContentMaxXHeadersIdeal;
    ImS16                   NameOffset;                     // Offset into parent ColumnsNames[]
    ImGuiTableColumnIdx     DisplayOrder;                   // Index within Table's IndexToDisplayOrder[] (column may be reordered by users)
    ImGuiTableColumnIdx     IndexWithinEnabledSet;          // Index within enabled/visible set (<= IndexToDisplayOrder)
    ImGuiTableColumnIdx     PrevEnabledColumn;              // Index of prev enabled/visible column within Columns[], -1 if first enabled/visible column
    ImGuiTableColumnIdx     NextEnabledColumn;              // Index of next enabled/visible column within Columns[], -1 if last enabled/visible column
    ImGuiTableColumnIdx     SortOrder;                      // Index of this column within sort specs, -1 if not sorting on this column, 0 for single-sort, may be >0 on multi-sort
    ImGuiTableDrawChannelIdx DrawChannelCurrent;            // Index within DrawSplitter.Channels[]
    ImGuiTableDrawChannelIdx DrawChannelFrozen;             // Draw channels for frozen rows (often headers)
    ImGuiTableDrawChannelIdx DrawChannelUnfrozen;           // Draw channels for unfrozen rows
    bool                    IsEnabled;                      // IsUserEnabled && (Flags & ImGuiTableColumnFlags_Disabled) == 0
    bool                    IsUserEnabled;                  // Is the column not marked Hidden by the user? (unrelated to being off view, e.g. clipped by scrolling).
    bool                    IsUserEnabledNextFrame;
    bool                    IsVisibleX;                     // Is actually in view (e.g. overlapping the host window clipping rectangle, not scrolled).
    bool                    IsVisibleY;
    bool                    IsRequestOutput;                // Return value for TableSetColumnIndex() / TableNextColumn(): whether we request user to output contents or not.
    bool                    IsSkipItems;                    // Do we want item submissions to this column to be completely ignored (no layout will happen).
    bool                    IsPreserveWidthAuto;
    ImS8                    NavLayerCurrent;                // ImGuiNavLayer in 1 byte
    ImU8                    AutoFitQueue;                   // Queue of 8 values for the next 8 frames to request auto-fit
    ImU8                    CannotSkipItemsQueue;           // Queue of 8 values for the next 8 frames to disable Clipped/SkipItem
    ImU8                    SortDirection : 2;              // ImGuiSortDirection_Ascending or ImGuiSortDirection_Descending
    ImU8                    SortDirectionsAvailCount : 2;   // Number of available sort directions (0 to 3)
    ImU8                    SortDirectionsAvailMask : 4;    // Mask of available sort directions (1-bit each)
    ImU8                    SortDirectionsAvailList;        // Ordered of available sort directions (2-bits each)

    ImGuiTableColumn()
    {
        memset(this, 0, sizeof(*this));
        StretchWeight = WidthRequest = -1f32;
        NameOffset = -1;
        DisplayOrder = IndexWithinEnabledSet = -1;
        PrevEnabledColumn = NextEnabledColumn = -1;
        SortOrder = -1;
        SortDirection = ImGuiSortDirection_None;
        DrawChannelCurrent = DrawChannelFrozen = DrawChannelUnfrozen = (ImU8)-1;
    }
};

// Transient cell data stored per row.
// sizeof() ~ 6
struct ImGuiTableCellData
{
    u32                       BgColor;    // Actual color
    ImGuiTableColumnIdx         Column;     // Column number
};

// Per-instance data that needs preserving across frames (seemingly most others do not need to be preserved aside from debug needs, does that needs they could be moved to ImGuiTableTempData ?)
struct ImGuiTableInstanceData
{
    c_float                       LastOuterHeight;            // Outer height from last frame // FIXME: multi-instance issue (#3955)
    c_float                       LastFirstRowHeight;         // Height of first row from last frame // FIXME: possible multi-instance issue?

    ImGuiTableInstanceData()    { LastOuterHeight = LastFirstRowHeight = 0f32; }
};

// sizeof() ~ 12
struct ImGuiTableColumnSettings
{
    c_float                   WidthOrWeight;
    ImGuiID                 UserID;
    ImGuiTableColumnIdx     Index;
    ImGuiTableColumnIdx     DisplayOrder;
    ImGuiTableColumnIdx     SortOrder;
    ImU8                    SortDirection : 2;
    ImU8                    IsEnabled : 1; // "Visible" in ini file
    ImU8                    IsStretch : 1;

    ImGuiTableColumnSettings()
    {
        WidthOrWeight = 0f32;
        UserID = 0;
        Index = -1;
        DisplayOrder = SortOrder = -1;
        SortDirection = ImGuiSortDirection_None;
        IsEnabled = 1;
        IsStretch = 0;
    }
};

// This is designed to be stored in a single ImChunkStream (1 header followed by N ImGuiTableColumnSettings, etc.)
struct ImGuiTableSettings
{
    ImGuiID                     ID;                     // Set to 0 to invalidate/delete the setting
    ImGuiTableFlags             SaveFlags;              // Indicate data we want to save using the Resizable/Reorderable/Sortable/Hideable flags (could be using its own flags..)
    c_float                       RefScale;               // Reference scale to be able to rescale columns on font/dpi changes.
    ImGuiTableColumnIdx         ColumnsCount;
    ImGuiTableColumnIdx         ColumnsCountMax;        // Maximum number of columns this settings instance can store, we can recycle a settings instance with lower number of columns but not higher
    bool                        WantApply;              // Set when loaded from .ini data (to enable merging/loading .ini data into an already running context)

    ImGuiTableSettings()        { memset(this, 0, sizeof(*this)); }
    *mut ImGuiTableColumnSettings   GetColumnSettings()     { return (*mut ImGuiTableColumnSettings)(this + 1); }
};

//-----------------------------------------------------------------------------
// [SECTION] ImGui internal API
// No guarantee of forward compatibility here!
//-----------------------------------------------------------------------------

namespace ImGui
{
    // Windows
    // We should always have a CurrentWindow in the stack (there is an implicit "Debug" window)
    // If this ever crash because g.CurrentWindow is NULL it means that either
    // - ImGui::NewFrame() has never been called, which is illegal.
    // - You are calling ImGui functions after ImGui::EndFrame()/ImGui::Render() and before the next ImGui::NewFrame(), which is also illegal.
    inline    *mut ImGuiWindow  GetCurrentWindowRead()      { let g = GImGui; // ImGuiContext& g = *GImGui; return g.CurrentWindow; }
    inline    *mut ImGuiWindow  GetCurrentWindow()          { let g = GImGui; // ImGuiContext& g = *GImGui; g.Currentwindow.WriteAccessed = true; return g.CurrentWindow; }
     *mut ImGuiWindow  FindWindowByID(ImGuiID id);
     *mut ImGuiWindow  FindWindowByName(*const char name);
     void          UpdateWindowParentAndRootLinks(*mut ImGuiWindow window, ImGuiWindowFlags flags, *mut ImGuiWindow parent_window);
     ImVec2        CalcWindowNextAutoFitSize(*mut ImGuiWindow window);
     bool          IsWindowChildOf(*mut ImGuiWindow window, *mut ImGuiWindow potential_parent, bool popup_hierarchy, bool dock_hierarchy);
     bool          IsWindowWithinBeginStackOf(*mut ImGuiWindow window, *mut ImGuiWindow potential_parent);
     bool          IsWindowAbove(*mut ImGuiWindow potential_above, *mut ImGuiWindow potential_below);
     bool          IsWindowNavFocusable(*mut ImGuiWindow window);
     void          SetWindowPos(*mut ImGuiWindow window, const ImVec2& pos, ImGuiCond cond = 0);
     void          SetWindowSize(*mut ImGuiWindow window, const ImVec2& size, ImGuiCond cond = 0);
     void          SetWindowCollapsed(*mut ImGuiWindow window, bool collapsed, ImGuiCond cond = 0);
     void          SetWindowHitTestHole(*mut ImGuiWindow window, const ImVec2& pos, const ImVec2& size);
    inline ImRect           WindowRectAbsToRel(*mut ImGuiWindow window, const ImRect& r) { ImVec2 off = window.DC.CursorStartPos; return ImRect(r.Min.x - off.x, r.Min.y - off.y, r.Max.x - off.x, r.Max.y - off.y); }
    inline ImRect           WindowRectRelToAbs(*mut ImGuiWindow window, const ImRect& r) { ImVec2 off = window.DC.CursorStartPos; return ImRect(r.Min.x + off.x, r.Min.y + off.y, r.Max.x + off.x, r.Max.y + off.y); }

    // Windows: Display Order and Focus Order
     void          FocusWindow(*mut ImGuiWindow window);
     void          FocusTopMostWindowUnderOne(*mut ImGuiWindow under_this_window, *mut ImGuiWindow ignore_window);
     void          BringWindowToFocusFront(*mut ImGuiWindow window);
     void          BringWindowToDisplayFront(*mut ImGuiWindow window);
     void          BringWindowToDisplayBack(*mut ImGuiWindow window);
     void          BringWindowToDisplayBehind(*mut ImGuiWindow window, *mut ImGuiWindow above_window);
     c_int           FindWindowDisplayIndex(*mut ImGuiWindow window);
     *mut ImGuiWindow  FindBottomMostVisibleWindowWithinBeginStack(*mut ImGuiWindow window);

    // Fonts, drawing
     void          SetCurrentFont(*mut ImFont font);
    inline *mut ImFont          GetDefaultFont() { let g = GImGui; // ImGuiContext& g = *GImGui; return g.IO.FontDefault ? g.IO.FontDefault : g.IO.Fonts->Fonts[0]; }
    inline *mut ImDrawList      GetForegroundDrawList(*mut ImGuiWindow window) { return GetForegroundDrawList(window.Viewport); }

    // Init
     void          Initialize();
     void          Shutdown();    // Since 1.60 this is a _private_ function. You can call DestroyContext() to destroy the context created by CreateContext().

    // NewFrame
     void          UpdateInputEvents(bool trickle_fast_inputs);
     void          UpdateHoveredWindowAndCaptureFlags();
     void          StartMouseMovingWindow(*mut ImGuiWindow window);
     void          StartMouseMovingWindowOrNode(*mut ImGuiWindow window, *mut ImGuiDockNode node, bool undock_floating_node);
     void          UpdateMouseMovingWindowNewFrame();
     void          UpdateMouseMovingWindowEndFrame();

    // Generic context hooks
     ImGuiID       AddContextHook(*mut ImGuiContext context, *const ImGuiContextHook hook);
     void          RemoveContextHook(*mut ImGuiContext context, ImGuiID hook_to_remove);
     void          CallContextHooks(*mut ImGuiContext context, ImGuiContextHookType type);

    // Viewports
     void          TranslateWindowsInViewport(*mut ImGuiViewportP viewport, const ImVec2& old_pos, const ImVec2& new_pos);
     void          ScaleWindowsInViewport(*mut ImGuiViewportP viewport, c_float scale);
     void          DestroyPlatformWindow(*mut ImGuiViewportP viewport);
     void          SetWindowViewport(*mut ImGuiWindow window, *mut ImGuiViewportP viewport);
     void          SetCurrentViewport(*mut ImGuiWindow window, *mut ImGuiViewportP viewport);
     *const ImGuiPlatformMonitor   GetViewportPlatformMonitor(*mut ImGuiViewport viewport);
     *mut ImGuiViewportP               FindHoveredViewportFromPlatformWindowStack(const ImVec2& mouse_platform_pos);

    // Settings
     void                  MarkIniSettingsDirty();
     void                  MarkIniSettingsDirty(*mut ImGuiWindow window);
     void                  ClearIniSettings();
     *mut ImGuiWindowSettings  CreateNewWindowSettings(*const char name);
     *mut ImGuiWindowSettings  FindWindowSettings(ImGuiID id);
     *mut ImGuiWindowSettings  FindOrCreateWindowSettings(*const char name);
     void                  AddSettingsHandler(*const ImGuiSettingsHandler handler);
     void                  RemoveSettingsHandler(*const char type_name);
     *mut ImGuiSettingsHandler FindSettingsHandler(*const char type_name);

    // Scrolling
     void          SetNextWindowScroll(const ImVec2& scroll); // Use -1f32 on one axis to leave as-is
     void          SetScrollX(*mut ImGuiWindow window, c_float scroll_x);
     void          SetScrollY(*mut ImGuiWindow window, c_float scroll_y);
     void          SetScrollFromPosX(*mut ImGuiWindow window, c_float local_x, c_float center_x_ratio);
     void          SetScrollFromPosY(*mut ImGuiWindow window, c_float local_y, c_float center_y_ratio);

    // Early work-in-progress API (ScrollToItem() will become public)
     void          ScrollToItem(ImGuiScrollFlags flags = 0);
     void          ScrollToRect(*mut ImGuiWindow window, const ImRect& rect, ImGuiScrollFlags flags = 0);
     ImVec2        ScrollToRectEx(*mut ImGuiWindow window, const ImRect& rect, ImGuiScrollFlags flags = 0);
//#ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    inline void             ScrollToBringRectIntoView(*mut ImGuiWindow window, const ImRect& rect) { ScrollToRect(window, rect, ImGuiScrollFlags_KeepVisibleEdgeY); }
//#endif

    // Basic Accessors
    inline ImGuiID          GetItemID()     { let g = GImGui; // ImGuiContext& g = *GImGui; return g.LastItemData.ID; }   // Get ID of last item (~~ often same ImGui::GetID(label) beforehand)
    inline ImGuiItemStatusFlags GetItemStatusFlags(){ let g = GImGui; // ImGuiContext& g = *GImGui; return g.LastItemData.StatusFlags; }
    inline ImGuiItemFlags   GetItemFlags()  { let g = GImGui; // ImGuiContext& g = *GImGui; return g.LastItemData.InFlags; }
    inline ImGuiID          GetActiveID()   { let g = GImGui; // ImGuiContext& g = *GImGui; return g.ActiveId; }
    inline ImGuiID          GetFocusID()    { let g = GImGui; // ImGuiContext& g = *GImGui; return g.NavId; }
     void          SetActiveID(ImGuiID id, *mut ImGuiWindow window);
     void          SetFocusID(ImGuiID id, *mut ImGuiWindow window);
     void          ClearActiveID();
     ImGuiID       GetHoveredID();
     void          SetHoveredID(ImGuiID id);
     void          KeepAliveID(ImGuiID id);
     void          MarkItemEdited(ImGuiID id);     // Mark data associated to given item as "edited", used by IsItemDeactivatedAfterEdit() function.
     void          PushOverrideID(ImGuiID id);     // Push given value as-is at the top of the ID stack (whereas PushID combines old and new hashes)
     ImGuiID       GetIDWithSeed(*const char str_id_begin, *const char str_id_end, ImGuiID seed);

    // Basic Helpers for widget code
     void          ItemSize(const ImVec2& size, c_float text_baseline_y = -1f32);
    inline void             ItemSize(const ImRect& bb, c_float text_baseline_y = -1f32) { ItemSize(bb.GetSize(), text_baseline_y); } // FIXME: This is a misleading API since we expect CursorPos to be bb.Min.
     bool          ItemAdd(const ImRect& bb, ImGuiID id, *const ImRect nav_bb = NULL, ImGuiItemFlags extra_flags = 0);
     bool          ItemHoverable(const ImRect& bb, ImGuiID id);
     bool          IsClippedEx(const ImRect& bb, ImGuiID id);
     void          SetLastItemData(ImGuiID item_id, ImGuiItemFlags in_flags, ImGuiItemStatusFlags status_flags, const ImRect& item_rect);
     ImVec2        CalcItemSize(ImVec2 size, c_float default_w, c_float default_h);
     c_float         CalcWrapWidthForPos(const ImVec2& pos, c_float wrap_pos_x);
     void          PushMultiItemsWidths(c_int components, c_float width_full);
     bool          IsItemToggledSelection();                                   // Was the last item selection toggled? (after Selectable(), TreeNode() etc. We only returns toggle _event_ in order to handle clipping correctly)
     ImVec2        GetContentRegionMaxAbs();
     void          ShrinkWidths(*mut ImGuiShrinkWidthItem items, c_int count, c_float width_excess);

    // Parameter stacks
     void          PushItemFlag(ImGuiItemFlags option, bool enabled);
     void          PopItemFlag();

    // Logging/Capture
     void          LogBegin(ImGuiLogType type, c_int auto_open_depth);           // -> BeginCapture() when we design v2 api, for now stay under the radar by using the old name.
     void          LogToBuffer(c_int auto_open_depth = -1);                      // Start logging/capturing to internal buffer
     void          LogRenderedText(*const ImVec2 ref_pos, *const char text, *const char text_end = NULL);
     void          LogSetNextTextDecoration(*const char prefix, *const char suffix);

    // Popups, Modals, Tooltips
     bool          BeginChildEx(*const char name, ImGuiID id, const ImVec2& size_arg, bool border, ImGuiWindowFlags flags);
     void          OpenPopupEx(ImGuiID id, ImGuiPopupFlags popup_flags = ImGuiPopupFlags_None);
     void          ClosePopupToLevel(c_int remaining, bool restore_focus_to_window_under_popup);
     void          ClosePopupsOverWindow(*mut ImGuiWindow ref_window, bool restore_focus_to_window_under_popup);
     void          ClosePopupsExceptModals();
     bool          IsPopupOpen(ImGuiID id, ImGuiPopupFlags popup_flags);
     bool          BeginPopupEx(ImGuiID id, ImGuiWindowFlags extra_flags);
     void          BeginTooltipEx(ImGuiTooltipFlags tooltip_flags, ImGuiWindowFlags extra_window_flags);
     ImRect        GetPopupAllowedExtentRect(*mut ImGuiWindow window);
     *mut ImGuiWindow  GetTopMostPopupModal();
     *mut ImGuiWindow  GetTopMostAndVisiblePopupModal();
     ImVec2        FindBestWindowPosForPopup(*mut ImGuiWindow window);
     ImVec2        FindBestWindowPosForPopupEx(const ImVec2& ref_pos, const ImVec2& size, *mut ImGuiDir last_dir, const ImRect& r_outer, const ImRect& r_avoid, ImGuiPopupPositionPolicy policy);

    // Menus
     bool          BeginViewportSideBar(*const char name, *mut ImGuiViewport viewport, ImGuiDir dir, c_float size, ImGuiWindowFlags window_flags);
     bool          BeginMenuEx(*const char label, *const char icon, bool enabled = true);
     bool          MenuItemEx(*const char label, *const char icon, *const char shortcut = NULL, bool selected = false, bool enabled = true);

    // Combos
     bool          BeginComboPopup(ImGuiID popup_id, const ImRect& bb, ImGuiComboFlags flags);
     bool          BeginComboPreview();
     void          EndComboPreview();

    // Gamepad/Keyboard Navigation
     void          NavInitWindow(*mut ImGuiWindow window, bool force_reinit);
     void          NavInitRequestApplyResult();
     bool          NavMoveRequestButNoResultYet();
     void          NavMoveRequestSubmit(ImGuiDir move_dir, ImGuiDir clip_dir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags);
     void          NavMoveRequestForward(ImGuiDir move_dir, ImGuiDir clip_dir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags);
     void          NavMoveRequestResolveWithLastItem(*mut ImGuiNavItemData result);
     void          NavMoveRequestCancel();
     void          NavMoveRequestApplyResult();
     void          NavMoveRequestTryWrapping(*mut ImGuiWindow window, ImGuiNavMoveFlags move_flags);
     void          ActivateItem(ImGuiID id);   // Remotely activate a button, checkbox, tree node etc. given its unique ID. activation is queued and processed on the next frame when the item is encountered again.
     void          SetNavWindow(*mut ImGuiWindow window);
     void          SetNavID(ImGuiID id, ImGuiNavLayer nav_layer, ImGuiID focus_scope_id, const ImRect& rect_rel);

    // Focus Scope (WIP)
    // This is generally used to identify a selection set (multiple of which may be in the same window), as selection
    // patterns generally need to react (e.g. clear selection) when landing on an item of the set.
     void          PushFocusScope(ImGuiID id);
     void          PopFocusScope();
    inline ImGuiID          GetFocusedFocusScope()          { let g = GImGui; // ImGuiContext& g = *GImGui; return g.NavFocusScopeId; }                            // Focus scope which is actually active
    inline ImGuiID          GetFocusScope()                 { let g = GImGui; // ImGuiContext& g = *GImGui; return g.Currentwindow.DC.NavFocusScopeIdCurrent; }   // Focus scope we are outputting into, set by PushFocusScope()

    // Inputs
    // FIXME: Eventually we should aim to move e.g. IsActiveIdUsingKey() into IsKeyXXX functions.
    inline bool             IsNamedKey(ImGuiKey key)                                    { return key >= ImGuiKey_NamedKey_BEGIN && key < ImGuiKey_NamedKey_END; }
    inline bool             IsLegacyKey(ImGuiKey key)                                   { return key >= ImGuiKey_LegacyNativeKey_BEGIN && key < ImGuiKey_LegacyNativeKey_END; }
    inline bool             IsGamepadKey(ImGuiKey key)                                  { return key >= ImGuiKey_Gamepad_BEGIN && key < ImGuiKey_Gamepad_END; }
    inline bool             IsAliasKey(ImGuiKey key)                                    { return key >= ImGuiKey_Aliases_BEGIN && key < ImGuiKey_Aliases_END; }
     *mut ImGuiKeyData GetKeyData(ImGuiKey key);
     void          GetKeyChordName(ImGuiModFlags mods, ImGuiKey key, *mut char out_buf, c_int out_buf_size);
     void          SetItemUsingMouseWheel();
     void          SetActiveIdUsingAllKeyboardKeys();
    inline bool             IsActiveIdUsingNavDir(ImGuiDir dir)                         { let g = GImGui; // ImGuiContext& g = *GImGui; return (g.ActiveIdUsingNavDirMask & (1 << dir)) != 0; }
    inline bool             IsActiveIdUsingKey(ImGuiKey key)                            { let g = GImGui; // ImGuiContext& g = *GImGui; return g.ActiveIdUsingKeyInputMask[key]; }
    inline void             SetActiveIdUsingKey(ImGuiKey key)                           { let g = GImGui; // ImGuiContext& g = *GImGui; g.ActiveIdUsingKeyInputMask.SetBit(key); }
    inline ImGuiKey         MouseButtonToKey(ImGuiMouseButton button)                   { IM_ASSERT(button >= 0 && button < ImGuiMouseButton_COUNT); return ImGuiKey_MouseLeft + button; }
     bool          IsMouseDragPastThreshold(ImGuiMouseButton button, c_float lock_threshold = -1f32);
     ImGuiModFlags GetMergedModFlags();
     ImVec2        GetKeyVector2d(ImGuiKey key_left, ImGuiKey key_right, ImGuiKey key_up, ImGuiKey key_down);
     c_float         GetNavTweakPressedAmount(ImGuiAxis axis);
     c_int           CalcTypematicRepeatAmount(c_float t0, c_float t1, c_float repeat_delay, c_float repeat_rate);
     void          GetTypematicRepeatRate(ImGuiInputFlags flags, *mut c_float repeat_delay, *mut c_float repeat_rate);
     bool          IsKeyPressedEx(ImGuiKey key, ImGuiInputFlags flags = 0);
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    inline bool             IsKeyPressedMap(ImGuiKey key, bool repeat = true)           { IM_ASSERT(IsNamedKey(key)); return IsKeyPressed(key, repeat); } // [removed in 1.87]
// #endif

    // Docking
    // (some functions are only declared in imgui.cpp, see Docking section)
     void          DockContextInitialize(*mut ImGuiContext ctx);
     void          DockContextShutdown(*mut ImGuiContext ctx);
     void          DockContextClearNodes(*mut ImGuiContext ctx, ImGuiID root_id, bool clear_settings_refs); // Use root_id==0 to clear all
     void          DockContextRebuildNodes(*mut ImGuiContext ctx);
     void          DockContextNewFrameUpdateUndocking(*mut ImGuiContext ctx);
     void          DockContextNewFrameUpdateDocking(*mut ImGuiContext ctx);
     void          DockContextEndFrame(*mut ImGuiContext ctx);
     ImGuiID       DockContextGenNodeID(*mut ImGuiContext ctx);
     void          DockContextQueueDock(*mut ImGuiContext ctx, *mut ImGuiWindow target, *mut ImGuiDockNode target_node, *mut ImGuiWindow payload, ImGuiDir split_dir, c_float split_ratio, bool split_outer);
     void          DockContextQueueUndockWindow(*mut ImGuiContext ctx, *mut ImGuiWindow window);
     void          DockContextQueueUndockNode(*mut ImGuiContext ctx, *mut ImGuiDockNode node);
     bool          DockContextCalcDropPosForDocking(*mut ImGuiWindow target, *mut ImGuiDockNode target_node, *mut ImGuiWindow payload_window, *mut ImGuiDockNode payload_node, ImGuiDir split_dir, bool split_outer, *mut ImVec2 out_pos);
     *mut ImGuiDockNodeDockContextFindNodeByID(*mut ImGuiContext ctx, ImGuiID id);
     bool          DockNodeBeginAmendTabBar(*mut ImGuiDockNode node);
     void          DockNodeEndAmendTabBar();
    inline *mut ImGuiDockNode   DockNodeGetRootNode(*mut ImGuiDockNode node)                 { while (node->ParentNode) node = node->ParentNode; return node; }
    inline bool             DockNodeIsInHierarchyOf(*mut ImGuiDockNode node, *mut ImGuiDockNode parent) { while (node) { if (node == parent) return true; node = node->ParentNode; } return false; }
    inline c_int              DockNodeGetDepth(*const ImGuiDockNode node)              { c_int depth = 0; while (node->ParentNode) { node = node->ParentNode; depth+= 1; } return depth; }
    inline ImGuiID          DockNodeGetWindowMenuButtonId(*const ImGuiDockNode node) { return ImHashStr("#COLLAPSE", 0, node->ID); }
    inline *mut ImGuiDockNode   GetWindowDockNode()                                      { let g = GImGui; // ImGuiContext& g = *GImGui; return g.Currentwindow.DockNode; }
     bool          GetWindowAlwaysWantOwnTabBar(*mut ImGuiWindow window);
     void          BeginDocked(*mut ImGuiWindow window, *mut bool p_open);
     void          BeginDockableDragDropSource(*mut ImGuiWindow window);
     void          BeginDockableDragDropTarget(*mut ImGuiWindow window);
     void          SetWindowDock(*mut ImGuiWindow window, ImGuiID dock_id, ImGuiCond cond);

    // Docking - Builder function needs to be generally called before the node is used/submitted.
    // - The DockBuilderXXX functions are designed to _eventually_ become a public API, but it is too early to expose it and guarantee stability.
    // - Do not hold on ImGuiDockNode* pointers! They may be invalidated by any split/merge/remove operation and every frame.
    // - To create a DockSpace() node, make sure to set the ImGuiDockNodeFlags_DockSpace flag when calling DockBuilderAddNode().
    //   You can create dockspace nodes (attached to a window) _or_ floating nodes (carry its own window) with this API.
    // - DockBuilderSplitNode() create 2 child nodes within 1 node. The initial node becomes a parent node.
    // - If you intend to split the node immediately after creation using DockBuilderSplitNode(), make sure
    //   to call DockBuilderSetNodeSize() beforehand. If you don't, the resulting split sizes may not be reliable.
    // - Call DockBuilderFinish() after you are done.
     void          DockBuilderDockWindow(*const char window_name, ImGuiID node_id);
     *mut ImGuiDockNodeDockBuilderGetNode(ImGuiID node_id);
    inline *mut ImGuiDockNode   DockBuilderGetCentralNode(ImGuiID node_id)              { *mut ImGuiDockNode node = DockBuilderGetNode(node_id); if (!node) return NULL; return DockNodeGetRootNode(node)->CentralNode; }
     ImGuiID       DockBuilderAddNode(ImGuiID node_id = 0, ImGuiDockNodeFlags flags = 0);
     void          DockBuilderRemoveNode(ImGuiID node_id);                 // Remove node and all its child, undock all windows
     void          DockBuilderRemoveNodeDockedWindows(ImGuiID node_id, bool clear_settings_refs = true);
     void          DockBuilderRemoveNodeChildNodes(ImGuiID node_id);       // Remove all split/hierarchy. All remaining docked windows will be re-docked to the remaining root node (node_id).
     void          DockBuilderSetNodePos(ImGuiID node_id, ImVec2 pos);
     void          DockBuilderSetNodeSize(ImGuiID node_id, ImVec2 size);
     ImGuiID       DockBuilderSplitNode(ImGuiID node_id, ImGuiDir split_dir, c_float size_ratio_for_node_at_dir, *mut ImGuiID out_id_at_dir, *mut ImGuiID out_id_at_opposite_dir); // Create 2 child nodes in this parent node.
     void          DockBuilderCopyDockSpace(ImGuiID src_dockspace_id, ImGuiID dst_dockspace_id, Vec<*const char>* in_window_remap_pairs);
     void          DockBuilderCopyNode(ImGuiID src_node_id, ImGuiID dst_node_id, Vec<ImGuiID>* out_node_remap_pairs);
     void          DockBuilderCopyWindowSettings(*const char src_name, *const char dst_name);
     void          DockBuilderFinish(ImGuiID node_id);

    // Drag and Drop
     bool          IsDragDropActive();
     bool          BeginDragDropTargetCustom(const ImRect& bb, ImGuiID id);
     void          ClearDragDrop();
     bool          IsDragDropPayloadBeingAccepted();

    // Internal Columns API (this is not exposed because we will encourage transitioning to the Tables API)
     void          SetWindowClipRectBeforeSetChannel(*mut ImGuiWindow window, const ImRect& clip_rect);
     void          BeginColumns(*const char str_id, c_int count, ImGuiOldColumnFlags flags = 0); // setup number of columns. use an identifier to distinguish multiple column sets. close with EndColumns().
     void          EndColumns();                                                               // close columns
     void          PushColumnClipRect(c_int column_index);
     void          PushColumnsBackground();
     void          PopColumnsBackground();
     ImGuiID       GetColumnsID(*const char str_id, c_int count);
     *mut ImGuiOldColumns FindOrCreateColumns(*mut ImGuiWindow window, ImGuiID id);
     c_float         GetColumnOffsetFromNorm(*const ImGuiOldColumns columns, c_float offset_norm);
     c_float         GetColumnNormFromOffset(*const ImGuiOldColumns columns, c_float offset);

    // Tables: Candidates for public API
     void          TableOpenContextMenu(c_int column_n = -1);
     void          TableSetColumnWidth(c_int column_n, c_float width);
     void          TableSetColumnSortDirection(c_int column_n, ImGuiSortDirection sort_direction, bool append_to_sort_specs);
     c_int           TableGetHoveredColumn(); // May use (TableGetColumnFlags() & ImGuiTableColumnFlags_IsHovered) instead. Return hovered column. return -1 when table is not hovered. return columns_count if the unused space at the right of visible columns is hovered.
     c_float         TableGetHeaderRowHeight();
     void          TablePushBackgroundChannel();
     void          TablePopBackgroundChannel();

    // Tables: Internals
    inline    *mut ImGuiTable   GetCurrentTable() { let g = GImGui; // ImGuiContext& g = *GImGui; return g.CurrentTable; }
     *mut ImGuiTable   TableFindByID(ImGuiID id);
     bool          BeginTableEx(*const char name, ImGuiID id, c_int columns_count, ImGuiTableFlags flags = 0, const ImVec2& outer_size = ImVec2(0, 0), c_float inner_width = 0f32);
     void          TableBeginInitMemory(*mut ImGuiTable table, c_int columns_count);
     void          TableBeginApplyRequests(*mut ImGuiTable table);
     void          TableSetupDrawChannels(*mut ImGuiTable table);
     void          TableUpdateLayout(*mut ImGuiTable table);
     void          TableUpdateBorders(*mut ImGuiTable table);
     void          TableUpdateColumnsWeightFromWidth(*mut ImGuiTable table);
     void          TableDrawBorders(*mut ImGuiTable table);
     void          TableDrawContextMenu(*mut ImGuiTable table);
     bool          TableBeginContextMenuPopup(*mut ImGuiTable table);
     void          TableMergeDrawChannels(*mut ImGuiTable table);
    inline *mut ImGuiTableInstanceData   TableGetInstanceData(*mut ImGuiTable table, c_int instance_no) { if (instance_no == 0) return &table->InstanceDataFirst; return &table->InstanceDataExtra[instance_no - 1]; }
     void          TableSortSpecsSanitize(*mut ImGuiTable table);
     void          TableSortSpecsBuild(*mut ImGuiTable table);
     ImGuiSortDirection TableGetColumnNextSortDirection(*mut ImGuiTableColumn column);
     void          TableFixColumnSortDirection(*mut ImGuiTable table, *mut ImGuiTableColumn column);
     c_float         TableGetColumnWidthAuto(*mut ImGuiTable table, *mut ImGuiTableColumn column);
     void          TableBeginRow(*mut ImGuiTable table);
     void          TableEndRow(*mut ImGuiTable table);
     void          TableBeginCell(*mut ImGuiTable table, c_int column_n);
     void          TableEndCell(*mut ImGuiTable table);
     ImRect        TableGetCellBgRect(*const ImGuiTable table, c_int column_n);
     *const char   TableGetColumnName(*const ImGuiTable table, c_int column_n);
     ImGuiID       TableGetColumnResizeID(*const ImGuiTable table, c_int column_n, c_int instance_no = 0);
     c_float         TableGetMaxColumnWidth(*const ImGuiTable table, c_int column_n);
     void          TableSetColumnWidthAutoSingle(*mut ImGuiTable table, c_int column_n);
     void          TableSetColumnWidthAutoAll(*mut ImGuiTable table);
     void          TableRemove(*mut ImGuiTable table);
     void          TableGcCompactTransientBuffers(*mut ImGuiTable table);
     void          TableGcCompactTransientBuffers(*mut ImGuiTableTempData table);
     void          TableGcCompactSettings();

    // Tables: Settings
     void                  TableLoadSettings(*mut ImGuiTable table);
     void                  TableSaveSettings(*mut ImGuiTable table);
     void                  TableResetSettings(*mut ImGuiTable table);
     *mut ImGuiTableSettings   TableGetBoundSettings(*mut ImGuiTable table);
     void                  TableSettingsAddSettingsHandler();
     *mut ImGuiTableSettings   TableSettingsCreate(ImGuiID id, c_int columns_count);
     *mut ImGuiTableSettings   TableSettingsFindByID(ImGuiID id);

    // Tab Bars
     bool          BeginTabBarEx(*mut ImGuiTabBar tab_bar, const ImRect& bb, ImGuiTabBarFlags flags, *mut ImGuiDockNode dock_node);
     *mut ImGuiTabItem TabBarFindTabByID(*mut ImGuiTabBar tab_bar, ImGuiID tab_id);
     *mut ImGuiTabItem TabBarFindMostRecentlySelectedTabForActiveWindow(*mut ImGuiTabBar tab_bar);
     void          TabBarAddTab(*mut ImGuiTabBar tab_bar, ImGuiTabItemFlags tab_flags, *mut ImGuiWindow window);
     void          TabBarRemoveTab(*mut ImGuiTabBar tab_bar, ImGuiID tab_id);
     void          TabBarCloseTab(*mut ImGuiTabBar tab_bar, *mut ImGuiTabItem tab);
     void          TabBarQueueReorder(*mut ImGuiTabBar tab_bar, *const ImGuiTabItem tab, c_int offset);
     void          TabBarQueueReorderFromMousePos(*mut ImGuiTabBar tab_bar, *const ImGuiTabItem tab, ImVec2 mouse_pos);
     bool          TabBarProcessReorder(*mut ImGuiTabBar tab_bar);
     bool          TabItemEx(*mut ImGuiTabBar tab_bar, *const char label, *mut bool p_open, ImGuiTabItemFlags flags, *mut ImGuiWindow docked_window);
     ImVec2        TabItemCalcSize(*const char label, bool has_close_button);
     void          TabItemBackground(*mut ImDrawList draw_list, const ImRect& bb, ImGuiTabItemFlags flags, u32 col);
     void          TabItemLabelAndCloseButton(*mut ImDrawList draw_list, const ImRect& bb, ImGuiTabItemFlags flags, ImVec2 frame_padding, *const char label, ImGuiID tab_id, ImGuiID close_button_id, bool is_contents_visible, *mut bool out_just_closed, *mut bool out_text_clipped);

    // Render helpers
    // AVOID USING OUTSIDE OF IMGUI.CPP! NOT FOR PUBLIC CONSUMPTION. THOSE FUNCTIONS ARE A MESS. THEIR SIGNATURE AND BEHAVIOR WILL CHANGE, THEY NEED TO BE REFACTORED INTO SOMETHING DECENT.
    // NB: All position are in absolute pixels coordinates (we are never using window coordinates internally)
     void          RenderText(ImVec2 pos, *const char text, *const char text_end = NULL, bool hide_text_after_hash = true);
     void          RenderTextWrapped(ImVec2 pos, *const char text, *const char text_end, c_float wrap_width);
     void          RenderTextClipped(const ImVec2& pos_min, const ImVec2& pos_max, *const char text, *const char text_end, *const ImVec2 text_size_if_known, const ImVec2& align = ImVec2(0, 0), *const ImRect clip_rect = NULL);
     void          RenderTextClippedEx(*mut ImDrawList draw_list, const ImVec2& pos_min, const ImVec2& pos_max, *const char text, *const char text_end, *const ImVec2 text_size_if_known, const ImVec2& align = ImVec2(0, 0), *const ImRect clip_rect = NULL);
     void          RenderTextEllipsis(*mut ImDrawList draw_list, const ImVec2& pos_min, const ImVec2& pos_max, c_float clip_max_x, c_float ellipsis_max_x, *const char text, *const char text_end, *const ImVec2 text_size_if_known);
     void          RenderFrame(ImVec2 p_min, ImVec2 p_max, u32 fill_col, bool border = true, c_float rounding = 0f32);
     void          RenderFrameBorder(ImVec2 p_min, ImVec2 p_max, c_float rounding = 0f32);
     void          RenderColorRectWithAlphaCheckerboard(*mut ImDrawList draw_list, ImVec2 p_min, ImVec2 p_max, u32 fill_col, c_float grid_step, ImVec2 grid_off, c_float rounding = 0f32, ImDrawFlags flags = 0);
     void          RenderNavHighlight(const ImRect& bb, ImGuiID id, ImGuiNavHighlightFlags flags = ImGuiNavHighlightFlags_TypeDefault); // Navigation highlight
     *const char   FindRenderedTextEnd(*const char text, *const char text_end = NULL); // Find the optional ## from which we stop displaying text.
     void          RenderMouseCursor(ImVec2 pos, c_float scale, ImGuiMouseCursor mouse_cursor, u32 col_fill, u32 col_border, u32 col_shadow);

    // Render helpers (those functions don't access any ImGui state!)
     void          RenderArrow(*mut ImDrawList draw_list, ImVec2 pos, u32 col, ImGuiDir dir, c_float scale = 1f32);
     void          RenderBullet(*mut ImDrawList draw_list, ImVec2 pos, u32 col);
     void          RenderCheckMark(*mut ImDrawList draw_list, ImVec2 pos, u32 col, c_float sz);
     void          RenderArrowPointingAt(*mut ImDrawList draw_list, ImVec2 pos, ImVec2 half_sz, ImGuiDir direction, u32 col);
     void          RenderArrowDockMenu(*mut ImDrawList draw_list, ImVec2 p_min, c_float sz, u32 col);
     void          RenderRectFilledRangeH(*mut ImDrawList draw_list, const ImRect& rect, u32 col, c_float x_start_norm, c_float x_end_norm, c_float rounding);
     void          RenderRectFilledWithHole(*mut ImDrawList draw_list, const ImRect& outer, const ImRect& inner, u32 col, c_float rounding);
     ImDrawFlags   CalcRoundingFlagsForRectInRect(const ImRect& r_in, const ImRect& r_outer, c_float threshold);

    // Widgets
     void          TextEx(*const char text, *const char text_end = NULL, ImGuiTextFlags flags = 0);
     bool          ButtonEx(*const char label, const ImVec2& size_arg = ImVec2(0, 0), ImGuiButtonFlags flags = 0);
     bool          CloseButton(ImGuiID id, const ImVec2& pos);
     bool          CollapseButton(ImGuiID id, const ImVec2& pos, *mut ImGuiDockNode dock_node);
     bool          ArrowButtonEx(*const char str_id, ImGuiDir dir, ImVec2 size_arg, ImGuiButtonFlags flags = 0);
     void          Scrollbar(ImGuiAxis axis);
     bool          ScrollbarEx(const ImRect& bb, ImGuiID id, ImGuiAxis axis, *mut ImS64 p_scroll_v, ImS64 avail_v, ImS64 contents_v, ImDrawFlags flags);
     bool          ImageButtonEx(ImGuiID id, ImTextureID texture_id, const ImVec2& size, const ImVec2& uv0, const ImVec2& uv1, const ImVec4& bg_col, const ImVec4& tint_col);
     ImRect        GetWindowScrollbarRect(*mut ImGuiWindow window, ImGuiAxis axis);
     ImGuiID       GetWindowScrollbarID(*mut ImGuiWindow window, ImGuiAxis axis);
     ImGuiID       GetWindowResizeCornerID(*mut ImGuiWindow window, c_int n); // 0..3: corners
     ImGuiID       GetWindowResizeBorderID(*mut ImGuiWindow window, ImGuiDir dir);
     void          SeparatorEx(ImGuiSeparatorFlags flags);
     bool          CheckboxFlags(*const char label, *mut ImS64 flags, ImS64 flags_value);
     bool          CheckboxFlags(*const char label, *mut ImU64 flags, ImU64 flags_value);

    // Widgets low-level behaviors
     bool          ButtonBehavior(const ImRect& bb, ImGuiID id, *mut bool out_hovered, *mut bool out_held, ImGuiButtonFlags flags = 0);
     bool          DragBehavior(ImGuiID id, ImGuiDataType data_type, *mut void p_v, c_float v_speed, *const void p_min, *const void p_max, *const char format, ImGuiSliderFlags flags);
     bool          SliderBehavior(const ImRect& bb, ImGuiID id, ImGuiDataType data_type, *mut void p_v, *const void p_min, *const void p_max, *const char format, ImGuiSliderFlags flags, *mut ImRect out_grab_bb);
     bool          SplitterBehavior(const ImRect& bb, ImGuiID id, ImGuiAxis axis, *mut c_float size1, *mut c_float size2, c_float min_size1, c_float min_size2, c_float hover_extend = 0f32, c_float hover_visibility_delay = 0f32, u32 bg_col = 0);
     bool          TreeNodeBehavior(ImGuiID id, ImGuiTreeNodeFlags flags, *const char label, *const char label_end = NULL);
     void          TreePushOverrideID(ImGuiID id);
     void          TreeNodeSetOpen(ImGuiID id, bool open);
     bool          TreeNodeUpdateNextOpen(ImGuiID id, ImGuiTreeNodeFlags flags);   // Return open state. Consume previous SetNextItemOpen() data, if any. May return true when logging.

    // Template functions are instantiated in imgui_widgets.cpp for a finite number of types.
    // To use them externally (for custom widget) you may need an "extern template" statement in your code in order to link to existing instances and silence Clang warnings (see #2036).
    // e.g. " extern template IMGUI_API float RoundScalarWithFormatT<float, float>(const char* format, ImGuiDataType data_type, float v); "
    template<typename T, typename SIGNED_T, typename FLOAT_T>    c_float ScaleRatioFromValueT(ImGuiDataType data_type, T v, T v_min, T v_max, bool is_logarithmic, c_float logarithmic_zero_epsilon, c_float zero_deadzone_size);
    template<typename T, typename SIGNED_T, typename FLOAT_T>    T     ScaleValueFromRatioT(ImGuiDataType data_type, c_float t, T v_min, T v_max, bool is_logarithmic, c_float logarithmic_zero_epsilon, c_float zero_deadzone_size);
    template<typename T, typename SIGNED_T, typename FLOAT_T>    bool  DragBehaviorT(ImGuiDataType data_type, *mut T v, c_float v_speed, T v_min, T v_max, *const char format, ImGuiSliderFlags flags);
    template<typename T, typename SIGNED_T, typename FLOAT_T>    bool  SliderBehaviorT(const ImRect& bb, ImGuiID id, ImGuiDataType data_type, *mut T v, T v_min, T v_max, *const char format, ImGuiSliderFlags flags, *mut ImRect out_grab_bb);
    template<typename T>                                         T     RoundScalarWithFormatT(*const char format, ImGuiDataType data_type, T v);
    template<typename T>                                         bool  CheckboxFlagsT(*const char label, *mut T flags, T flags_value);

    // Data type helpers
     *const ImGuiDataTypeInfo  DataTypeGetInfo(ImGuiDataType data_type);
     c_int           DataTypeFormatString(*mut char buf, c_int buf_size, ImGuiDataType data_type, *const void p_data, *const char format);
     void          DataTypeApplyOp(ImGuiDataType data_type, c_int op, *mut void output, *const void arg_1, *const void arg_2);
     bool          DataTypeApplyFromText(*const char buf, ImGuiDataType data_type, *mut void p_data, *const char format);
     c_int           DataTypeCompare(ImGuiDataType data_type, *const void arg_1, *const void arg_2);
     bool          DataTypeClamp(ImGuiDataType data_type, *mut void p_data, *const void p_min, *const void p_max);

    // InputText
     bool          InputTextEx(*const char label, *const char hint, *mut char buf, c_int buf_size, const ImVec2& size_arg, ImGuiInputTextFlags flags, ImGuiInputTextCallback callback = NULL, *mut void user_data = NULL);
     bool          TempInputText(const ImRect& bb, ImGuiID id, *const char label, *mut char buf, c_int buf_size, ImGuiInputTextFlags flags);
     bool          TempInputScalar(const ImRect& bb, ImGuiID id, *const char label, ImGuiDataType data_type, *mut void p_data, *const char format, *const void p_clamp_min = NULL, *const void p_clamp_max = NULL);
    inline bool             TempInputIsActive(ImGuiID id)       { let g = GImGui; // ImGuiContext& g = *GImGui; return (g.ActiveId == id && g.TempInputId == id); }
    inline *mut ImGuiInputTextState GetInputTextState(ImGuiID id)   { let g = GImGui; // ImGuiContext& g = *GImGui; return (id != 0 && g.InputTextState.ID == id) ? &g.InputTextState : NULL; } // Get input text state if active

    // Color
     void          ColorTooltip(*const char text, *const c_float col, ImGuiColorEditFlags flags);
     void          ColorEditOptionsPopup(*const c_float col, ImGuiColorEditFlags flags);
     void          ColorPickerOptionsPopup(*const c_float ref_col, ImGuiColorEditFlags flags);

    // Plot
     c_int           PlotEx(ImGuiPlotType plot_type, *const char label, c_float (*values_getter)(*mut void data, c_int idx), *mut void data, c_int values_count, c_int values_offset, *const char overlay_text, c_float scale_min, c_float scale_max, ImVec2 frame_size);

    // Shade functions (write over already created vertices)
     void          ShadeVertsLinearColorGradientKeepAlpha(*mut ImDrawList draw_list, c_int vert_start_idx, c_int vert_end_idx, ImVec2 gradient_p0, ImVec2 gradient_p1, u32 col0, u32 col1);
     void          ShadeVertsLinearUV(*mut ImDrawList draw_list, c_int vert_start_idx, c_int vert_end_idx, const ImVec2& a, const ImVec2& b, const ImVec2& uv_a, const ImVec2& uv_b, bool clamp);

    // Garbage collection
     void          GcCompactTransientMiscBuffers();
     void          GcCompactTransientWindowBuffers(*mut ImGuiWindow window);
     void          GcAwakeTransientWindowBuffers(*mut ImGuiWindow window);

    // Debug Log
     void          DebugLog(*const char fmt, ...) IM_FMTARGS(1);
     void          DebugLogV(*const char fmt, va_list args) IM_FMTLIST(1);

    // Debug Tools
     void          ErrorCheckEndFrameRecover(ImGuiErrorLogCallback log_callback, *mut void user_data = NULL);
     void          ErrorCheckEndWindowRecover(ImGuiErrorLogCallback log_callback, *mut void user_data = NULL);
     void          ErrorCheckUsingSetCursorPosToExtendParentBoundaries();
    inline void             DebugDrawItemRect(u32 col = IM_COL32(255,0,0,255))    { let g = GImGui; // ImGuiContext& g = *GImGui; ImGuiWindow* window = g.CurrentWindow; GetForegroundDrawList(window)->AddRect(g.LastItemData.Rect.Min, g.LastItemData.Rect.Max, col); }
    inline void             DebugStartItemPicker()                                  { let g = GImGui; // ImGuiContext& g = *GImGui; g.DebugItemPickerActive = true; }
     void          ShowFontAtlas(*mut ImFontAtlas atlas);
     void          DebugHookIdInfo(ImGuiID id, ImGuiDataType data_type, *const void data_id, *const void data_id_end);
     void          DebugNodeColumns(*mut ImGuiOldColumns columns);
     void          DebugNodeDockNode(*mut ImGuiDockNode node, *const char label);
     void          DebugNodeDrawList(*mut ImGuiWindow window, *mut ImGuiViewportP viewport, *const ImDrawList draw_list, *const char label);
     void          DebugNodeDrawCmdShowMeshAndBoundingBox(*mut ImDrawList out_draw_list, *const ImDrawList draw_list, *const ImDrawCmd draw_cmd, bool show_mesh, bool show_aabb);
     void          DebugNodeFont(*mut ImFont font);
     void          DebugNodeFontGlyph(*mut ImFont font, *const ImFontGlyph glyph);
     void          DebugNodeStorage(*mut ImGuiStorage storage, *const char label);
     void          DebugNodeTabBar(*mut ImGuiTabBar tab_bar, *const char label);
     void          DebugNodeTable(*mut ImGuiTable table);
     void          DebugNodeTableSettings(*mut ImGuiTableSettings settings);
     void          DebugNodeInputTextState(*mut ImGuiInputTextState state);
     void          DebugNodeWindow(*mut ImGuiWindow window, *const char label);
     void          DebugNodeWindowSettings(*mut ImGuiWindowSettings settings);
     void          DebugNodeWindowsList(Vec<*mut ImGuiWindow>* windows, *const char label);
     void          DebugNodeWindowsListByBeginStackParent(*mut ImGuiWindow* windows, c_int windows_size, *mut ImGuiWindow parent_in_begin_stack);
     void          DebugNodeViewport(*mut ImGuiViewportP viewport);
     void          DebugRenderViewportThumbnail(*mut ImDrawList draw_list, *mut ImGuiViewportP viewport, const ImRect& bb);

    // Obsolete functions
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    inline bool     TreeNodeBehaviorIsOpen(ImGuiID id, ImGuiTreeNodeFlags flags = 0)    { return TreeNodeUpdateNextOpen(id, flags); }   // Renamed in 1.89

    // Refactored focus/nav/tabbing system in 1.82 and 1.84. If you have old/custom copy-and-pasted widgets that used FocusableItemRegister():
    //  (Old) IMGUI_VERSION_NUM  < 18209: using 'ItemAdd(....)'                              and 'bool tab_focused = FocusableItemRegister(...)'
    //  (Old) IMGUI_VERSION_NUM >= 18209: using 'ItemAdd(..., ImGuiItemAddFlags_Focusable)'  and 'bool tab_focused = (GetItemStatusFlags() & ImGuiItemStatusFlags_Focused) != 0'
    //  (New) IMGUI_VERSION_NUM >= 18413: using 'ItemAdd(..., ImGuiItemFlags_Inputable)'     and 'bool tab_focused = (GetItemStatusFlags() & ImGuiItemStatusFlags_FocusedTabbing) != 0 || g.NavActivateInputId == id' (WIP)
    // Widget code are simplified as there's no need to call FocusableItemUnregister() while managing the transition from regular widget to TempInputText()
    inline bool     FocusableItemRegister(*mut ImGuiWindow window, ImGuiID id)              { IM_ASSERT(0); IM_UNUSED(window); IM_UNUSED(id); return false; } // -> pass ImGuiItemAddFlags_Inputable flag to ItemAdd()
    inline void     FocusableItemUnregister(*mut ImGuiWindow window)                        { IM_ASSERT(0); IM_UNUSED(window); }                              // -> unnecessary: TempInputText() uses ImGuiInputTextFlags_MergedItem
// #endif

} // namespace ImGui


//-----------------------------------------------------------------------------
// [SECTION] ImFontAtlas internal API
//-----------------------------------------------------------------------------

// This structure is likely to evolve as we add support for incremental atlas updates
struct ImFontBuilderIO
{
    bool    (*FontBuilder_Build)(*mut ImFontAtlas atlas);
};

// Helper for font builder
// #ifdef IMGUI_ENABLE_STB_TRUETYPE
 *const ImFontBuilderIO ImFontAtlasGetBuilderForStbTruetype();
// #endif
 void      ImFontAtlasBuildInit(*mut ImFontAtlas atlas);
 void      ImFontAtlasBuildSetupFont(*mut ImFontAtlas atlas, *mut ImFont font, *mut ImFontConfig font_config, c_float ascent, c_float descent);
 void      ImFontAtlasBuildPackCustomRects(*mut ImFontAtlas atlas, *mut void stbrp_context_opaque);
 void      ImFontAtlasBuildFinish(*mut ImFontAtlas atlas);
 void      ImFontAtlasBuildRender8bppRectFromString(*mut ImFontAtlas atlas, c_int x, c_int y, c_int w, c_int h, *const char in_str, char in_marker_char, unsigned char in_marker_pixel_value);
 void      ImFontAtlasBuildRender32bppRectFromString(*mut ImFontAtlas atlas, c_int x, c_int y, c_int w, c_int h, *const char in_str, char in_marker_char, c_uint in_marker_pixel_value);
 void      ImFontAtlasBuildMultiplyCalcLookupTable(unsigned out_table: [c_char;256], c_float in_multiply_factor);
 void      ImFontAtlasBuildMultiplyRectAlpha8(const unsigned table: [c_char;256], unsigned *mut char pixels, c_int x, c_int y, c_int w, c_int h, c_int stride);

//-----------------------------------------------------------------------------
// [SECTION] Test Engine specific hooks (imgui_test_engine)
//-----------------------------------------------------------------------------

// #ifdef IMGUI_ENABLE_TEST_ENGINE
extern void         ImGuiTestEngineHook_ItemAdd(*mut ImGuiContext ctx, const ImRect& bb, ImGuiID id);
extern void         ImGuiTestEngineHook_ItemInfo(*mut ImGuiContext ctx, ImGuiID id, *const char label, ImGuiItemStatusFlags flags);
extern void         ImGuiTestEngineHook_Log(*mut ImGuiContext ctx, *const char fmt, ...);
extern *const char  ImGuiTestEngine_FindItemDebugLabel(*mut ImGuiContext ctx, ImGuiID id);

// #define IMGUI_TEST_ENGINE_ITEM_ADD(_BB,_ID)                 if (g.TestEngineHookItems) ImGuiTestEngineHook_ItemAdd(&g, _BB, _ID)               // Register item bounding box
// #define IMGUI_TEST_ENGINE_ITEM_INFO(_ID,_LABEL,_FLAGS)      if (g.TestEngineHookItems) ImGuiTestEngineHook_ItemInfo(&g, _ID, _LABEL, _FLAGS)   // Register item label and status flags (optional)
// #define IMGUI_TEST_ENGINE_LOG(_FMT,...)                     if (g.TestEngineHookItems) ImGuiTestEngineHook_Log(&g, _FMT, __VA_ARGS__)          // Custom log entry from user land into test log
// #else
// #define IMGUI_TEST_ENGINE_ITEM_ADD(_BB,_ID)                 ((void)0)
// #define IMGUI_TEST_ENGINE_ITEM_INFO(_ID,_LABEL,_FLAGS)      ((void)g)
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

// #endif // #ifndef IMGUI_DISABLE
