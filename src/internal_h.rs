// dear imgui, v1.88
// (internal structures/api)

// You may use this file to debug, understand or extend ImGui features but we don't provide any guarantee of forward compatibility!
// Set:
//   #define IMGUI_DEFINE_MATH_OPERATORS
// To implement maths operators for Vector2D (disabled by default to not collide with using IM_VEC2_CLASS_EXTRA along with your own math types+operators)

/*

index of this file:

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
// [SECTION] viewport support
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

use std::collections::HashSet;
use std::f32::consts::PI;
use crate::context::{Context, ContextHookType};
use crate::types::{Id32, INVALID_ID};
use crate::input::DimgKey::{GamepadRStickRight, GamepadStart};
use crate::dock_node::{DockNode, DockNodeFlags};
use crate::draw_data_builder::DrawDataBuilder;
use crate::draw_list::{DIMG_DRAW_LIST_CIRCLE_AUTO_SEGMENT_MAX, DIMG_DRAW_LIST_CIRCLE_AUTO_SEGMENT_MIN, DRAW_LIST_ARCFAST_TABLE_SIZE, DrawList};
use crate::input::DimgKey;
use crate::rect::Rect;
use crate::vectors::two_d::Vector2D;


// Use your programming IDE "Go to definition" facility on the names of the center columns to find the actual flags/enum lists.
// typedef int ImGuiDataAuthority;         // -> enum ImGuiDataAuthority_      // Enum: for storing the source authority (dock node vs window) of a field
// typedef int ImGuiLayoutType;            // -> enum ImGuiLayoutType_         // Enum: Horizontal or vertical
// typedef int ImGuiActivateFlags;         // -> enum ImGuiActivateFlags_      // flags: for navigation/focus function (will be for ActivateItem() later)
// typedef int ImGuiDebugLogFlags;         // -> enum ImGuiDebugLogFlags_      // flags: for ShowDebugLogWindow(), g.DebugLogFlags
// typedef int ImGuiItemFlags;             // -> enum ImGuiItemFlags_          // flags: for PushItemFlag()
// typedef int ImGuiItemStatusFlags;       // -> enum ImGuiItemStatusFlags_    // flags: for dc.LastItemStatusFlags
// typedef int ImGuiOldColumnFlags;        // -> enum ImGuiOldColumnFlags_     // flags: for BeginColumns()
// typedef int ImGuiNavHighlightFlags;     // -> enum ImGuiNavHighlightFlags_  // flags: for RenderNavHighlight()
// typedef int ImGuiNavDirSourceFlags;     // -> enum NavDirSourceFlags::  // flags: for get_nav_input_amount_2d()
// typedef int ImGuiNavMoveFlags;          // -> enum ImGuiNavMoveFlags_       // flags: for navigation requests
// typedef int ImGuiNextItemDataFlags;     // -> enum ImGuiNextItemDataFlags_  // flags: for SetNextItemXXX() functions
// typedef int ImGuiNextWindowDataFlags;   // -> enum ImGuiNextWindowDataFlags_// flags: for SetNextWindowXXX() functions
// typedef int ImGuiScrollFlags;           // -> enum ImGuiScrollFlags_        // flags: for ScrollToItem() and navigation requests
// typedef int ImGuiSeparatorFlags;        // -> enum      // flags: for SeparatorEx()
// typedef int ImGuiTextFlags;             // -> enum           // flags: for TextEx()
// typedef int ImGuiTooltipFlags;          // -> enum        // flags: for BeginTooltipEx()
//
// typedef void (*ImGuiErrorLogCallback)(void* user_data, const char* fmt, ...);

//-----------------------------------------------------------------------------
// [SECTION] Context pointer
// See implementation of this variable in imgui.cpp for comments and details.
//-----------------------------------------------------------------------------

// #ifndef GImGui
// extern  ImGuiContext* GImGui;  // current implicit context pointer
// #endif

//-------------------------------------------------------------------------
// [SECTION] STB libraries includes
//-------------------------------------------------------------------------



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
// down the line in some frameworks/languages we would like to have a way to redirect those to the programmer and recover from more faults.
// #ifndef IM_ASSERT_USER_ERROR
// #define IM_ASSERT_USER_ERROR(_EXP,_MSG) IM_ASSERT((_EXP) && _MSG)   // Recoverable User Error
// #endif

// Misc Macros
// #define f32::PI                           3.14159265358979323846
// #ifdef _WIN32
// #define IM_NEWLINE                      "\r\n"   // Play it nice with windows users (Update: since 2018-05, Notepad finally appears to support Unix-style carriage returns!)
// #else
// #define IM_NEWLINE                      "\n"
// #endif
// #define IM_TABSIZE                      (4)
// #define IM_MEMALIGN(_OFF,_ALIGN)        (((_OFF) + ((_ALIGN) - 1)) & ~((_ALIGN) - 1))           // Memory align e.g. IM_ALIGN(0,4)=0, IM_ALIGN(1,4)=4, IM_ALIGN(4,4)=4, IM_ALIGN(5,4)=8

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
// - Helpers: Vector2D/Vector4D operators
// - Helpers: Maths
// - Helpers: Geometry
// - Helper: ImVec1
// - Helper: Vector2Dih
// - Helper: ImRect
// - Helper: ImBitArray
// - Helper: ImBitVector
// - Helper: ImSpan<>, ImSpanAllocator<>
// - Helper: ImPool<>
// - Helper: ImChunkStream<>
//-----------------------------------------------------------------------------

// Helpers: Hashing
//  ImGuiID       ImHashData(const void* data, size_t data_size, ImU32 seed = 0);
//  ImGuiID       ImHashStr(const char* data, size_t data_size = 0, ImU32 seed = 0);



// Helpers: Color Blending
//  ImU32         im_alpha_blend_colors(ImU32 col_a, ImU32 col_b);

// Helpers: Bit manipulation

// Helpers: String
//  int           ImStricmp(const char* str1, const char* str2);
//  int           ImStrnicmp(const char* str1, const char* str2, size_t count);
//  void          ImStrncpy(char* dst, const char* src, size_t count);
//  char*         ImStrdup(const char* str);
//  char*         ImStrdupcpy(char* dst, size_t* p_dst_size, const char* str);
//  const char*   ImStrchrRange(const char* str_begin, const char* str_end, char c);
//  int           ImStrlenW(const ImWchar* str);
//  const char*   ImStreolRange(const char* str, const char* str_end);                // End end-of-line
//  const ImWchar*ImStrbolW(const ImWchar* buf_mid_line, const ImWchar* buf_begin);   // Find beginning-of-line
//  const char*   ImStristr(const char* haystack, const char* haystack_end, const char* needle, const char* needle_end);
//  void          ImStrTrimBlanks(char* str);
//  const char*   ImStrSkipBlank(const char* str);
// static inline bool      ImCharIsBlankA(char c)          { return c == ' ' || c == '\t'; }


// Helpers: Formatting
//  int           ImFormatString(char* buf, size_t buf_size, const char* fmt, ...) IM_FMTARGS(3);
//  int           ImFormatStringV(char* buf, size_t buf_size, const char* fmt, va_list args) IM_FMTLIST(3);
//  void          ImFormatStringToTempBuffer(const char** out_buf, const char** out_buf_end, const char* fmt, ...) IM_FMTARGS(3);
//  void          ImFormatStringToTempBufferV(const char** out_buf, const char** out_buf_end, const char* fmt, va_list args) IM_FMTLIST(3);
//  const char*   ImParseFormatFindStart(const char* format);
//  const char*   ImParseFormatFindEnd(const char* format);
//  const char*   ImParseFormatTrimDecorations(const char* format, char* buf, size_t buf_size);
//  void          ImParseFormatSanitizeForPrinting(const char* fmt_in, char* fmt_out, size_t fmt_out_size);
//  const char*   ImParseFormatSanitizeForScanning(const char* fmt_in, char* fmt_out, size_t fmt_out_size);
//  int           ImParseFormatPrecision(const char* format, int default_value);

// Helpers: UTF-8 <> wchar conversions
//  const char*   ImTextCharToUtf8(char out_buf[5], unsigned int c);                                                      // return out_buf
//  int           ImTextStrToUtf8(char* out_buf, int out_buf_size, const ImWchar* in_text, const ImWchar* in_text_end);   // return output UTF-8 bytes count
//  int           ImTextCharFromUtf8(unsigned int* out_char, const char* in_text, const char* in_text_end);               // read one character. return input UTF-8 bytes count
//  int           ImTextStrFromUtf8(ImWchar* out_buf, int out_buf_size, const char* in_text, const char* in_text_end, const char** in_remaining = NULL);   // return input UTF-8 bytes count
//  int           ImTextCountCharsFromUtf8(const char* in_text, const char* in_text_end);                                 // return number of UTF-8 code-points (NOT bytes count)
//  int           ImTextCountUtf8BytesFromChar(const char* in_text, const char* in_text_end);                             // return number of bytes to express one char in UTF-8
//  int           ImTextCountUtf8BytesFromStr(const ImWchar* in_text, const ImWchar* in_text_end);                        // return number of bytes to express string in UTF-8

// Helpers: Vector2D/Vector4D operators
// We are keeping those disabled by default so they don't leak in user space, to allow user enabling implicit cast operators between Vector2D and their own types (using IM_VEC2_CLASS_EXTRA etc.)
// We unfortunately don't have a unary- operator for Vector2D because this would needs to be defined inside the class itself.
// #ifdef IMGUI_DEFINE_MATH_OPERATORS
// IM_MSVC_RUNTIME_CHECKS_OFF

// IM_MSVC_RUNTIME_CHECKS_RESTORE
// #endif

// Helpers: File System
// #ifdef IMGUI_DISABLE_FILE_FUNCTIONS
// #define IMGUI_DISABLE_DEFAULT_FILE_FUNCTIONS
// typedef void* ImFileHandle;
// static inline ImFileHandle  ImFileOpen(const char*, const char*)                    { return NULL; }
// static inline bool          ImFileClose(ImFileHandle)                               { return false; }
// static inline ImU64         ImFileGetSize(ImFileHandle)                             { return -1; }
// static inline ImU64         ImFileRead(void*, ImU64, ImU64, ImFileHandle)           { return 0; }
// static inline ImU64         ImFileWrite(const void*, ImU64, ImU64, ImFileHandle)    { return 0; }
// #endif
// #ifndef IMGUI_DISABLE_DEFAULT_FILE_FUNCTIONS
// typedef FILE* ImFileHandle;
//  ImFileHandle      ImFileOpen(const char* filename, const char* mode);
//  bool              ImFileClose(ImFileHandle file);
//  ImU64             ImFileGetSize(ImFileHandle file);
//  ImU64             ImFileRead(void* data, ImU64 size, ImU64 count, ImFileHandle file);
//  ImU64             ImFileWrite(const void* data, ImU64 size, ImU64 count, ImFileHandle file);
// #else
// #define IMGUI_DISABLE_TTY_FUNCTIONS // Can't use stdout, fflush if we are not using default file functions
// #endif
//  void*             ImFileLoadToMemory(const char* filename, const char* mode, size_t* out_file_size = NULL, int padding_bytes = 0);


// Helpers: Geometry

// Helper: ImVec1 (1D vector)
// (this odd construct is used to facilitate the transition between 1D and 2D, and the maintenance of some branches/patches)
// IM_MSVC_RUNTIME_CHECKS_OFF


// Helper: Vector2Dih (2D vector, half-size integer, for long-term packed storage)
// struct Vector2Dih
// {
//     short   x, y;
//     constexpr Vector2Dih()                           : x(0), y(0) {}
//     constexpr Vector2Dih(short _x, short _y)         : x(_x), y(_y) {}
//     constexpr explicit Vector2Dih(const Vector2D& rhs) : x((short)rhs.x), y((short)rhs.y) {}
// };

// Helper: ImBitArray
// inline bool     ImBitArrayTestBit(const ImU32* arr, int n)      { ImU32 mask = 1 << (n & 31); return (arr[n >> 5] & mask) != 0; }
// inline void     ImBitArrayClearBit(ImU32* arr, int n)           { ImU32 mask = 1 << (n & 31); arr[n >> 5] &= ~mask; }
// inline void     ImBitArraySetBit(ImU32* arr, int n)             { ImU32 mask = 1 << (n & 31); arr[n >> 5] |= mask; }
// inline void     ImBitArraySetBitRange(ImU32* arr, int n, int n2) // Works on range [n..n2)
// {
//     n2--;
//     while (n <= n2)
//     {
//         int a_mod = (n & 31);
//         int b_mod = (n2 > (n | 31) ? 31 : (n2 & 31)) + 1;
//         ImU32 mask = ((1 << b_mod) - 1) & ~((1 << a_mod) - 1);
//         arr[n >> 5] |= mask;
//         n = (n + 32) & ~31;
//     }
// }

// Helper: ImBitArray class (wrapper over ImBitArray functions)
// Store 1-bit per value.
// template<int BITCOUNT, int OFFSET = 0>
// struct ImBitArray
// {
//     ImU32           Storage[(BITCOUNT + 31) >> 5];
//     ImBitArray()                                { ClearAllBits(); }
//     void            ClearAllBits()              { memset(Storage, 0, sizeof(Storage)); }
//     void            SetAllBits()                { memset(Storage, 255, sizeof(Storage)); }
//     bool            TestBit(int n) const        { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); return ImBitArrayTestBit(Storage, n); }
//     void            set_bit(int n)               { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); ImBitArraySetBit(Storage, n); }
//     void            ClearBit(int n)             { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); ImBitArrayClearBit(Storage, n); }
//     void            SetBitRange(int n, int n2)  { n += OFFSET; n2 += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT && n2 > n && n2 <= BITCOUNT); ImBitArraySetBitRange(Storage, n, n2); } // Works on range [n..n2)
//     bool            operator[](int n) const     { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); return ImBitArrayTestBit(Storage, n); }
// };

// Helper: ImBitVector
// Store 1-bit per value.
// struct  ImBitVector
// {
//     ImVector<ImU32> Storage;
//     void            Create(int sz)              { Storage.resize((sz + 31) >> 5); memset(Storage.Data, 0, Storage.size * sizeof(Storage.Data[0])); }
//     void            clear()                     { Storage.clear(); }
//     bool            TestBit(int n) const        { IM_ASSERT(n < (Storage.size << 5)); return ImBitArrayTestBit(Storage.Data, n); }
//     void            set_bit(int n)               { IM_ASSERT(n < (Storage.size << 5)); ImBitArraySetBit(Storage.Data, n); }
//     void            ClearBit(int n)             { IM_ASSERT(n < (Storage.size << 5)); ImBitArrayClearBit(Storage.Data, n); }
// };

// Helper: ImSpan<>
// Pointing to a span of data we don't own.
// template<typename T>
// struct ImSpan
// {
//     T*                  Data;
//     T*                  DataEnd;
//
//     // Constructors, destructor
//     inline ImSpan()                                 { Data = DataEnd = NULL; }
//     inline ImSpan(T* data, int size)                { Data = data; DataEnd = data + size; }
//     inline ImSpan(T* data, T* data_end)             { Data = data; DataEnd = data_end; }
//
//     inline void         set(T* data, int size)      { Data = data; DataEnd = data + size; }
//     inline void         set(T* data, T* data_end)   { Data = data; DataEnd = data_end; }
//     inline int          size() const                { return (ptrdiff_t)(DataEnd - Data); }
//     inline int          size_in_bytes() const       { return (ptrdiff_t)(DataEnd - Data) * sizeof(T); }
//     inline T&           operator[](int i)           { T* p = Data + i; IM_ASSERT(p >= Data && p < DataEnd); return *p; }
//     inline const T&     operator[](int i) const     { const T* p = Data + i; IM_ASSERT(p >= Data && p < DataEnd); return *p; }
//
//     inline T*           begin()                     { return Data; }
//     inline const T*     begin() const               { return Data; }
//     inline T*           end()                       { return DataEnd; }
//     inline const T*     end() const                 { return DataEnd; }
//
//     // Utilities
//     inline int  index_from_ptr(const T* it) const   { IM_ASSERT(it >= Data && it < DataEnd); const ptrdiff_t off = it - Data; return off; }
// };

// Helper: ImSpanAllocator<>
// Facilitate storing multiple chunks into a single large block (the "arena")
// - Usage: call Reserve() N times, allocate GetArenaSizeInBytes() worth, pass it to SetArenaBasePtr(), call GetSpan() N times to retrieve the aligned ranges.
// template<int CHUNKS>
// struct ImSpanAllocator
// {
//     char*   BasePtr;
//     int     CurrOff;
//     int     CurrIdx;
//     int     Offsets[CHUNKS];
//     int     Sizes[CHUNKS];
//
//     ImSpanAllocator()                               { memset(this, 0, sizeof(*this)); }
//     inline void  Reserve(int n, size_t sz, int a=4) { IM_ASSERT(n == CurrIdx && n < CHUNKS); CurrOff = IM_MEMALIGN(CurrOff, a); Offsets[n] = CurrOff; Sizes[n] = sz; CurrIdx += 1; CurrOff += sz; }
//     inline int   GetArenaSizeInBytes()              { return CurrOff; }
//     inline void  SetArenaBasePtr(void* base_ptr)    { BasePtr = (char*)base_ptr; }
//     inline void* GetSpanPtrBegin(int n)             { IM_ASSERT(n >= 0 && n < CHUNKS && CurrIdx == CHUNKS); return (void*)(BasePtr + Offsets[n]); }
//     inline void* GetSpanPtrEnd(int n)               { IM_ASSERT(n >= 0 && n < CHUNKS && CurrIdx == CHUNKS); return (void*)(BasePtr + Offsets[n] + Sizes[n]); }
//     template<typename T>
//     inline void  GetSpan(int n, ImSpan<T>* span)    { span->set((T*)GetSpanPtrBegin(n), (T*)GetSpanPtrEnd(n)); }
// };

// Helper: ImChunkStream<>
// build and iterate a contiguous stream of variable-sized structures.
// This is used by Settings to store persistent data while reducing allocation count.
// We store the chunk size first, and align the final size on 4 bytes boundaries.
// The tedious/zealous amount of casting is to avoid -Wcast-align warnings.
// template<typename T>
// struct ImChunkStream
// {
//     ImVector<char>  Buf;
//
//     void    clear()                     { Buf.clear(); }
//     bool    empty() const               { return Buf.size == 0; }
//     int     size() const                { return Buf.size; }
//     T*      alloc_chunk(size_t sz)      { size_t HDR_SZ = 4; sz = IM_MEMALIGN(HDR_SZ + sz, 4u); int off = Buf.size; Buf.resize(off + sz); ((int*)(void*)(Buf.Data + off))[0] = sz; return (T*)(void*)(Buf.Data + off + HDR_SZ); }
//     T*      begin()                     { size_t HDR_SZ = 4; if (!Buf.Data) return NULL; return (T*)(void*)(Buf.Data + HDR_SZ); }
//     T*      next_chunk(T* p)            { size_t HDR_SZ = 4; IM_ASSERT(p >= begin() && p < end()); p = (T*)(void*)((char*)(void*)p + chunk_size(p)); if (p == (T*)(void*)((char*)end() + HDR_SZ)) return (T*)0; IM_ASSERT(p < end()); return p; }
//     int     chunk_size(const T* p)      { return ((const int*)p)[-1]; }
//     T*      end()                       { return (T*)(void*)(Buf.Data + Buf.size); }
//     int     offset_from_ptr(const T* p) { IM_ASSERT(p >= begin() && p < end()); const ptrdiff_t off = (const char*)p - Buf.Data; return off; }
//     T*      ptr_from_offset(int off)    { IM_ASSERT(off >= 4 && off < Buf.size); return (T*)(void*)(Buf.Data + off); }
//     void    swap(ImChunkStream<T>& rhs) { rhs.Buf.swap(Buf); }
//
// };

//-----------------------------------------------------------------------------
// [SECTION] ImDrawList support
//-----------------------------------------------------------------------------
// data shared between all ImDrawList instances
// You may want to create your own instance of this if you want to use ImDrawList completely without ImGui. In that case, watch out for future changes to this structure.

//-----------------------------------------------------------------------------
// [SECTION] Widgets support: flags, enums, data structures
//-----------------------------------------------------------------------------

// Storage for LastItem data

// PressedOnMask_         = PressedOnClick | PressedOnClickRelease | PressedOnClickReleaseAnywhere | PressedOnRelease | PressedOnDoubleClick | PressedOnDragDropHold,
// PressedOnDefault_      = PressedOnClickRelease


//-----------------------------------------------------------------------------
// [SECTION] Inputs support
//-----------------------------------------------------------------------------

// typedef ImBitArray<NamedKey_COUNT, -NamedKey_BEGIN>    ImBitArrayForNamedKeys;

//-----------------------------------------------------------------------------
// [SECTION] Clipper support
//-----------------------------------------------------------------------------


//-----------------------------------------------------------------------------
// [SECTION] Navigation support
//-----------------------------------------------------------------------------



//-----------------------------------------------------------------------------
// [SECTION] Columns support
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] Multi-select support
//-----------------------------------------------------------------------------

//#ifdef IMGUI_HAS_MULTI_SELECT
// <this is filled in 'range_select' branch>
//#endif // #ifdef IMGUI_HAS_MULTI_SELECT

//-----------------------------------------------------------------------------
// [SECTION] Docking support
//-----------------------------------------------------------------------------


// sizeof() 156~192

// #endif // #ifdef IMGUI_HAS_DOCK

//-----------------------------------------------------------------------------
// [SECTION] viewport support
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] Metrics, Debug Tools
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] ImGuiWindowTempData, ImGuiWindow
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] Tab bar, Tab item support
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] Table support
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] ImGui internal API
// No guarantee of forward compatibility here!
//-----------------------------------------------------------------------------

// namespace ImGui
// {
    // windows
    // We should always have a current_window in the stack (there is an implicit "Debug" window)
    // If this ever crash because g.current_window is NULL it means that either
    // - ImGui::NewFrame() has never been called, which is illegal.
    // - You are calling ImGui functions after ImGui::EndFrame()/ImGui::Render() and before the next ImGui::NewFrame(), which is also illegal.
    // inline    ImGuiWindow*  GetCurrentWindowRead()      { ImGuiContext& g = *GImGui; return g.CurrentWindow; }
    // inline    ImGuiWindow*  GetCurrentWindow()          { ImGuiContext& g = *GImGui; g.CurrentWindow->WriteAccessed = true; return g.CurrentWindow; }
    //  ImGuiWindow*  FindWindowByID(ImGuiID id);
    //  ImGuiWindow*  FindWindowByName(const char* name);
    //  void          UpdateWindowParentAndRootLinks(ImGuiWindow* window, ImGuiWindowFlags flags, ImGuiWindow* parent_window);
    //  Vector2D        CalcWindowNextAutoFitSize(ImGuiWindow* window);
    //  bool          IsWindowChildOf(ImGuiWindow* window, ImGuiWindow* potential_parent, bool popup_hierarchy, bool dock_hierarchy);
    //  bool          IsWindowWithinBeginStackOf(ImGuiWindow* window, ImGuiWindow* potential_parent);
    //  bool          is_window_above(ImGuiWindow* potential_above, ImGuiWindow* potential_below);
    //  bool          IsWindowNavFocusable(ImGuiWindow* window);
    //  void          set_window_pos(ImGuiWindow* window, const Vector2D& pos, ImGuiCond cond = 0);
    //  void          SetWindowSize(ImGuiWindow* window, const Vector2D& size, ImGuiCond cond = 0);
    //  void          SetWindowCollapsed(ImGuiWindow* window, bool collapsed, ImGuiCond cond = 0);
    //  void          SetWindowHitTestHole(ImGuiWindow* window, const Vector2D& pos, const Vector2D& size);
    // inline ImRect           WindowRectAbsToRel(ImGuiWindow* window, const ImRect& r) { Vector2D off = window.dc.CursorStartPos; return ImRect(r.min.x - off.x, r.min.y - off.y, r.max.x - off.x, r.max.y - off.y); }
    // inline ImRect           WindowRectRelToAbs(ImGuiWindow* window, const ImRect& r) { Vector2D off = window.dc.CursorStartPos; return ImRect(r.min.x + off.x, r.min.y + off.y, r.max.x + off.x, r.max.y + off.y); }

    // windows: Display Order and Focus Order
    //  void          focus_window(ImGuiWindow* window);
    //  void          FocusTopMostWindowUnderOne(ImGuiWindow* under_this_window, ImGuiWindow* ignore_window);
    //  void          BringWindowToFocusFront(ImGuiWindow* window);
    //  void          BringWindowToDisplayFront(ImGuiWindow* window);
    //  void          BringWindowToDisplayBack(ImGuiWindow* window);
    //  void          BringWindowToDisplayBehind(ImGuiWindow* window, ImGuiWindow* above_window);
    //  int           FindWindowDisplayIndex(ImGuiWindow* window);
    //  ImGuiWindow*  FindBottomMostVisibleWindowWithinBeginStack(ImGuiWindow* window);

    // fonts, drawing
    //  void          SetCurrentFont(ImFont* font);
    // inline ImFont*          GetDefaultFont() { ImGuiContext& g = *GImGui; return g.io.font_default ? g.io.font_default : g.io.fonts->fonts[0]; }
    // inline ImDrawList*      GetForegroundDrawList(ImGuiWindow* window) { return GetForegroundDrawList(window.viewport); }

    // Init
    //  void          Initialize();
    //  void          Shutdown();    // Since 1.60 this is a _private_ function. You can call DestroyContext() to destroy the context created by CreateContext().

    // NewFrame
    //  void          UpdateInputEvents(bool trickle_fast_inputs);
    //  void          UpdateHoveredWindowAndCaptureFlags();
    //  void          start_mouse_moving_window(ImGuiWindow* window);
    //  void          StartMouseMovingWindowOrNode(ImGuiWindow* window, ImGuiDockNode* node, bool undock_floating_node);
    //  void          UpdateMouseMovingWindowNewFrame();
    //  void          UpdateMouseMovingWindowEndFrame();

    // Generic context hooks
    //  ImGuiID       AddContextHook(ImGuiContext* context, const ImGuiContextHook* hook);
    //  void          RemoveContextHook(ImGuiContext* context, ImGuiID hook_to_remove);
    //  void          CallContextHooks(ImGuiContext* context, ImGuiContextHookType type);

    // viewports
    //  void          TranslateWindowsInViewport(ImGuiViewportP* viewport, const Vector2D& old_pos, const Vector2D& new_pos);
    //  void          ScaleWindowsInViewport(ImGuiViewportP* viewport, float scale);
    //  void          DestroyPlatformWindow(ImGuiViewportP* viewport);
    //  void          SetWindowViewport(ImGuiWindow* window, ImGuiViewportP* viewport);
    //  void          SetCurrentViewport(ImGuiWindow* window, ImGuiViewportP* viewport);
    //  const ImGuiPlatformMonitor*   GetViewportPlatformMonitor(ImGuiViewport* viewport);
    //  ImGuiViewportP*               FindHoveredViewportFromPlatformWindowStack(const Vector2D& mouse_platform_pos);

    // Settings
    //  void                  MarkIniSettingsDirty();
    //  void                  MarkIniSettingsDirty(ImGuiWindow* window);
    //  void                  ClearIniSettings();
    //  ImGuiWindowSettings*  CreateNewWindowSettings(const char* name);
    //  ImGuiWindowSettings*  FindWindowSettings(ImGuiID id);
    //  ImGuiWindowSettings*  FindOrCreateWindowSettings(const char* name);
    //  void                  AddSettingsHandler(const ImGuiSettingsHandler* handler);
    //  void                  RemoveSettingsHandler(const char* type_name);
    //  ImGuiSettingsHandler* FindSettingsHandler(const char* type_name);

    // Scrolling
    //  void          SetNextWindowScroll(const Vector2D& scroll); // Use -1.0 on one axis to leave as-is
    //  void          SetScrollX(ImGuiWindow* window, float scroll_x);
    //  void          SetScrollY(ImGuiWindow* window, float scroll_y);
    //  void          SetScrollFromPosX(ImGuiWindow* window, float local_x, float center_x_ratio);
    //  void          SetScrollFromPosY(ImGuiWindow* window, float local_y, float center_y_ratio);

    // Early work-in-progress API (ScrollToItem() will become public)
    //  void          ScrollToItem(ImGuiScrollFlags flags = 0);
    //  void          ScrollToRect(ImGuiWindow* window, const ImRect& rect, ImGuiScrollFlags flags = 0);
    //  Vector2D        ScrollToRectEx(ImGuiWindow* window, const ImRect& rect, ImGuiScrollFlags flags = 0);
//#ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     inline void             ScrollToBringRectIntoView(ImGuiWindow* window, const ImRect& rect) { ScrollToRect(window, rect, ImGuiScrollFlags_KeepVisibleEdgeY); }
//#endif

    // Basic Accessors
    // inline ImGuiID          GetItemID()     { ImGuiContext& g = *GImGui; return g.LastItemData.id; }   // Get id of last item (~~ often same ImGui::GetID(label) beforehand)
    // inline ImGuiItemStatusFlags GetItemStatusFlags(){ ImGuiContext& g = *GImGui; return g.LastItemData.status_flags; }
    // inline ImGuiItemFlags   GetItemFlags()  { ImGuiContext& g = *GImGui; return g.LastItemData.in_flags; }
    // inline ImGuiID          GetActiveID()   { ImGuiContext& g = *GImGui; return g.active_id; }
    // inline ImGuiID          GetFocusID()    { ImGuiContext& g = *GImGui; return g.nav_id; }
    //  void          set_active_id(ImGuiID id, ImGuiWindow* window);
    //  void          SetFocusID(ImGuiID id, ImGuiWindow* window);
    //  void          clear_active_id();
    //  ImGuiID       GetHoveredID();
    //  void          SetHoveredID(ImGuiID id);
    //  void          keep_alive_id(ImGuiID id);
    //  void          MarkItemEdited(ImGuiID id);     // Mark data associated to given item as "edited", used by IsItemDeactivatedAfterEdit() function.
    //  void          PushOverrideID(ImGuiID id);     // Push given value as-is at the top of the id stack (whereas PushID combines old and new hashes)
    //  ImGuiID       GetIDWithSeed(const char* str_id_begin, const char* str_id_end, ImGuiID seed);

    // Basic Helpers for widget code
    //  void          ItemSize(const Vector2D& size, float text_baseline_y = -1.0);
    // inline void             ItemSize(const ImRect& bb, float text_baseline_y = -1.0) { ItemSize(bb.get_size(), text_baseline_y); } // FIXME: This is a misleading API since we expect CursorPos to be bb.min.
    //  bool          ItemAdd(const ImRect& bb, ImGuiID id, const ImRect* nav_bb = NULL, ImGuiItemFlags extra_flags = 0);
    //  bool          ItemHoverable(const ImRect& bb, ImGuiID id);
    //  bool          IsClippedEx(const ImRect& bb, ImGuiID id);
    //  void          SetLastItemData(ImGuiID item_id, ImGuiItemFlags in_flags, ImGuiItemStatusFlags status_flags, const ImRect& item_rect);
    //  Vector2D        CalcItemSize(Vector2D size, float default_w, float default_h);
    //  float         CalcWrapWidthForPos(const Vector2D& pos, float wrap_pos_x);
    //  void          PushMultiItemsWidths(int components, float width_full);
    //  bool          IsItemToggledSelection();                                   // Was the last item selection toggled? (after Selectable(), TreeNode() etc. We only returns toggle _event_ in order to handle clipping correctly)
    //  Vector2D        GetContentRegionMaxAbs();
    //  void          ShrinkWidths(ImGuiShrinkWidthItem* items, int count, float width_excess);

    // Parameter stacks
    //  void          PushItemFlag(ImGuiItemFlags option, bool enabled);
    //  void          PopItemFlag();

// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    // Currently refactoring focus/nav/tabbing system
    // If you have old/custom copy-and-pasted widgets that used FocusableItemRegister():
    //  (Old) IMGUI_VERSION_NUM  < 18209: using 'ItemAdd(....)'                              and 'bool tab_focused = FocusableItemRegister(...)'
    //  (Old) IMGUI_VERSION_NUM >= 18209: using 'ItemAdd(..., ImGuiItemAddFlags_Focusable)'  and 'bool tab_focused = (GetItemStatusFlags() & ImGuiItemStatusFlags_Focused) != 0'
    //  (New) IMGUI_VERSION_NUM >= 18413: using 'ItemAdd(..., ImGuiItemFlags_Inputable)'     and 'bool tab_focused = (GetItemStatusFlags() & ImGuiItemStatusFlags_FocusedTabbing) != 0 || g.nav_activate_input_id == id' (WIP)
    // Widget code are simplified as there's no need to call FocusableItemUnregister() while managing the transition from regular widget to TempInputText()
    // inline bool FocusableItemRegister(ImGuiWindow* window, ImGuiID id)  { IM_ASSERT(0); IM_UNUSED(window); IM_UNUSED(id); return false; } // -> pass ImGuiItemAddFlags_Inputable flag to ItemAdd()
    // inline void FocusableItemUnregister(ImGuiWindow* window)            { IM_ASSERT(0); IM_UNUSED(window); }                              // -> unnecessary: TempInputText() uses ImGuiInputTextFlags_MergedItem
// #endif

    // Logging/Capture
    //  void          LogBegin(ImGuiLogType type, int auto_open_depth);           // -> BeginCapture() when we design v2 api, for now stay under the radar by using the old name.
    //  void          LogToBuffer(int auto_open_depth = -1);                      // Start logging/capturing to internal buffer
    //  void          LogRenderedText(const Vector2D* ref_pos, const char* text, const char* text_end = NULL);
     // void          LogSetNextTextDecoration(const char* prefix, const char* suffix);
    //
    // Popups, Modals, Tooltips
     // bool          begin_child_ex(const char* name, ImGuiID id, const Vector2D& size_arg, bool border, ImGuiWindowFlags flags);
     // void          OpenPopupEx(ImGuiID id, ImGuiPopupFlags popup_flags = ImGuiPopupFlags_None);
     // void          ClosePopupToLevel(int remaining, bool restore_focus_to_window_under_popup);
     // void          close_popups_over_window(ImGuiWindow* ref_window, bool restore_focus_to_window_under_popup);
     // void          ClosePopupsExceptModals();
     // bool          IsPopupOpen(ImGuiID id, ImGuiPopupFlags popup_flags);
     // bool          BeginPopupEx(ImGuiID id, ImGuiWindowFlags extra_flags);
     // void          BeginTooltipEx(ImGuiTooltipFlags tooltip_flags, ImGuiWindowFlags extra_window_flags);
     // ImRect        GetPopupAllowedExtentRect(ImGuiWindow* window);
     // ImGuiWindow*  get_top_most_popup_modal();
     // ImGuiWindow*  GetTopMostAndVisiblePopupModal();
     // Vector2D        FindBestWindowPosForPopup(ImGuiWindow* window);
     // Vector2D        FindBestWindowPosForPopupEx(const Vector2D& ref_pos, const Vector2D& size, ImGuiDir* last_dir, const ImRect& r_outer, const ImRect& r_avoid, ImGuiPopupPositionPolicy policy);
    //
    // Menus
    //  bool          BeginViewportSideBar(const char* name, ImGuiViewport* viewport, ImGuiDir dir, float size, ImGuiWindowFlags window_flags);
    //  bool          BeginMenuEx(const char* label, const char* icon, bool enabled = true);
    //  bool          MenuItemEx(const char* label, const char* icon, const char* shortcut = NULL, bool selected = false, bool enabled = true);

    // Combos
    //  bool          BeginComboPopup(ImGuiID popup_id, const ImRect& bb, ImGuiComboFlags flags);
    //  bool          BeginComboPreview();
    //  void          EndComboPreview();

    // Gamepad/Keyboard Navigation
    //  void          nav_init_window(ImGuiWindow* window, bool force_reinit);
     // void          NavInitRequestApplyResult();
     // bool          NavMoveRequestButNoResultYet();
     // void          NavMoveRequestSubmit(ImGuiDir move_dir, ImGuiDir clip_dir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags);
     // void          NavMoveRequestForward(ImGuiDir move_dir, ImGuiDir clip_dir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags);
     // void          NavMoveRequestResolveWithLastItem(ImGuiNavItemData* result);
     // void          NavMoveRequestCancel();
     // void          NavMoveRequestApplyResult();
     // void          NavMoveRequestTryWrapping(ImGuiWindow* window, ImGuiNavMoveFlags move_flags);
     // const char*   GetNavInputName(ImGuiNavInput n);
     // float         GetNavInputAmount(ImGuiNavInput n, ImGuiNavReadMode mode);
     // Vector2D        get_nav_input_amount_2d(ImGuiNavDirSourceFlags dir_sources, ImGuiNavReadMode mode, float slow_factor = 0.0, float fast_factor = 0.0);
     // int           CalcTypematicRepeatAmount(float t0, float t1, float repeat_delay, float repeat_rate);
     // void          ActivateItem(ImGuiID id);   // Remotely activate a button, checkbox, tree node etc. given its unique id. activation is queued and processed on the next frame when the item is encountered again.
     // void          SetNavWindow(ImGuiWindow* window);
     // void          SetNavID(ImGuiID id, ImGuiNavLayer nav_layer, ImGuiID focus_scope_id, const ImRect& rect_rel);

    // Focus Scope (WIP)
    // This is generally used to identify a selection set (multiple of which may be in the same window), as selection
    // patterns generally need to react (e.g. clear selection) when landing on an item of the set.
    //  void          PushFocusScope(ImGuiID id);
    //  void          PopFocusScope();
    // inline ImGuiID          GetFocusedFocusScope()          { ImGuiContext& g = *GImGui; return g.NavFocusScopeId; }                            // Focus scope which is actually active
    // inline ImGuiID          GetFocusScope()                 { ImGuiContext& g = *GImGui; return g.CurrentWindow->DC.nav_focus_scope_id_current; }   // Focus scope we are outputting into, set by PushFocusScope()

    // Inputs
    // FIXME: Eventually we should aim to move e.g. IsActiveIdUsingKey() into IsKeyXXX functions.
    // inline bool             IsNamedKey(ImGuiKey key)                                    { return key >= NamedKey_BEGIN && key < NamedKey_END; }
    // inline bool             IsLegacyKey(ImGuiKey key)                                   { return key >= LegacyNativeKey_BEGIN && key < LegacyNativeKey_END; }
    // inline bool             IsGamepadKey(ImGuiKey key)                                  { return key >= Gamepad_BEGIN && key < Gamepad_END; }
    //  ImGuiKeyData* GetKeyData(ImGuiKey key);
    //  void          SetItemUsingMouseWheel();
    //  void          SetActiveIdUsingNavAndKeys();
    // inline bool             IsActiveIdUsingNavDir(ImGuiDir dir)                         { ImGuiContext& g = *GImGui; return (g.ActiveIdUsingNavDirMask & (1 << dir)) != 0; }
    // inline bool             IsActiveIdUsingNavInput(ImGuiNavInput input)                { ImGuiContext& g = *GImGui; return (g.ActiveIdUsingNavInputMask & (1 << input)) != 0; }
    // inline bool             IsActiveIdUsingKey(ImGuiKey key)                            { ImGuiContext& g = *GImGui; return g.ActiveIdUsingKeyInputMask[key]; }
    // inline void             SetActiveIdUsingKey(ImGuiKey key)                           { ImGuiContext& g = *GImGui; g.ActiveIdUsingKeyInputMask.set_bit(key); }
    //  bool          IsMouseDragPastThreshold(ImGuiMouseButton button, float lock_threshold = -1.0);
    // inline bool             IsNavInputDown(ImGuiNavInput n)                             { ImGuiContext& g = *GImGui; return g.io.nav_inputs[n] > 0.0; }
    // inline bool             IsNavInputTest(ImGuiNavInput n, ImGuiNavReadMode rm)        { return (GetNavInputAmount(n, rm) > 0.0); }
    //  ImGuiModFlags get_merged_mod_flags();
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//     inline bool             IsKeyPressedMap(ImGuiKey key, bool repeat = true)           { IM_ASSERT(IsNamedKey(key)); return IsKeyPressed(key, repeat); } // [removed in 1.87]
// #endif

// inline bool             DockNodeIsInHierarchyOf(ImGuiDockNode* node, ImGuiDockNode* parent) { while (node) { if (node == parent) return true; node = node->parent_node; } return false; }
    // inline int              DockNodeGetDepth(const ImGuiDockNode* node)              { int depth = 0; while (node->parent_node) { node = node->parent_node; depth += 1; } return depth; }
    // inline ImGuiID          DockNodeGetWindowMenuButtonId(const ImGuiDockNode* node) { return ImHashStr("#COLLAPSE", 0, node->id); }
    // inline ImGuiDockNode*   GetWindowDockNode()                                      { ImGuiContext& g = *GImGui; return g.CurrentWindow->DockNode; }
    //  bool          GetWindowAlwaysWantOwnTabBar(ImGuiWindow* window);
    //  void          BeginDocked(ImGuiWindow* window, bool* p_open);
    //  void          BeginDockableDragDropSource(ImGuiWindow* window);
    //  void          BeginDockableDragDropTarget(ImGuiWindow* window);
    //  void          SetWindowDock(ImGuiWindow* window, ImGuiID dock_id, ImGuiCond cond);

    // Docking - Builder function needs to be generally called before the node is used/submitted.
    // - The DockBuilderXXX functions are designed to _eventually_ become a public API, but it is too early to expose it and guarantee stability.
    // - Do not hold on ImGuiDockNode* pointers! They may be invalidated by any split/merge/remove operation and every frame.
    // - To create a DockSpace() node, make sure to set the DockSpace flag when calling DockBuilderAddNode().
    //   You can create dockspace nodes (attached to a window) _or_ floating nodes (carry its own window) with this API.
    // - DockBuilderSplitNode() create 2 child nodes within 1 node. The initial node becomes a parent node.
    // - If you intend to split the node immediately after creation using DockBuilderSplitNode(), make sure
    //   to call DockBuilderSetNodeSize() beforehand. If you don't, the resulting split sizes may not be reliable.
    // - Call DockBuilderFinish() after you are done.
    //  void          DockBuilderDockWindow(const char* window_name, ImGuiID node_id);
    //  ImGuiDockNode*DockBuilderGetNode(ImGuiID node_id);
    // inline ImGuiDockNode*   DockBuilderGetCentralNode(ImGuiID node_id)              { ImGuiDockNode* node = DockBuilderGetNode(node_id); if (!node) return NULL; return DockNodeGetRootNode(node)->central_node; }
    //  ImGuiID       DockBuilderAddNode(ImGuiID node_id = 0, ImGuiDockNodeFlags flags = 0);
    //  void          DockBuilderRemoveNode(ImGuiID node_id);                 // Remove node and all its child, undock all windows
    //  void          DockBuilderRemoveNodeDockedWindows(ImGuiID node_id, bool clear_settings_refs = true);
    //  void          DockBuilderRemoveNodeChildNodes(ImGuiID node_id);       // Remove all split/hierarchy. All remaining docked windows will be re-docked to the remaining root node (node_id).
    //  void          DockBuilderSetNodePos(ImGuiID node_id, Vector2D pos);
    //  void          DockBuilderSetNodeSize(ImGuiID node_id, Vector2D size);
    //  ImGuiID       DockBuilderSplitNode(ImGuiID node_id, ImGuiDir split_dir, float size_ratio_for_node_at_dir, ImGuiID* out_id_at_dir, ImGuiID* out_id_at_opposite_dir); // Create 2 child nodes in this parent node.
    //  void          DockBuilderCopyDockSpace(ImGuiID src_dockspace_id, ImGuiID dst_dockspace_id, ImVector<const char*>* in_window_remap_pairs);
    //  void          DockBuilderCopyNode(ImGuiID src_node_id, ImGuiID dst_node_id, ImVector<ImGuiID>* out_node_remap_pairs);
    //  void          DockBuilderCopyWindowSettings(const char* src_name, const char* dst_name);
    //  void          DockBuilderFinish(ImGuiID node_id);

    // Drag and Drop
    //  bool          IsDragDropActive();
    //  bool          BeginDragDropTargetCustom(const ImRect& bb, ImGuiID id);
    //  void          ClearDragDrop();
    //  bool          is_drag_drop_payload_being_accepted();

    // Internal Columns API (this is not exposed because we will encourage transitioning to the tables API)
    //  void          SetWindowClipRectBeforeSetChannel(ImGuiWindow* window, const ImRect& clip_rect);
    //  void          BeginColumns(const char* str_id, int count, ImGuiOldColumnFlags flags = 0); // setup number of columns. use an identifier to distinguish multiple column sets. close with EndColumns().
    //  void          EndColumns();                                                               // close columns
    //  void          PushColumnClipRect(int column_index);
    //  void          PushColumnsBackground();
    //  void          PopColumnsBackground();
    //  ImGuiID       GetColumnsID(const char* str_id, int count);
    //  ImGuiOldColumns* FindOrCreateColumns(ImGuiWindow* window, ImGuiID id);
    //  float         GetColumnOffsetFromNorm(const ImGuiOldColumns* columns, float offset_norm);
    //  float         GetColumnNormFromOffset(const ImGuiOldColumns* columns, float offset);

    // tables: Candidates for public API
    //  void          TableOpenContextMenu(int column_n = -1);
    //  void          TableSetColumnWidth(int column_n, float width);
    //  void          TableSetColumnSortDirection(int column_n, ImGuiSortDirection sort_direction, bool append_to_sort_specs);
    //  int           TableGetHoveredColumn(); // May use (TableGetColumnFlags() & ImGuiTableColumnFlags_IsHovered) instead. Return hovered column. return -1 when table is not hovered. return columns_count if the unused space at the right of visible columns is hovered.
    //  float         TableGetHeaderRowHeight();
    //  void          TablePushBackgroundChannel();
    //  void          TablePopBackgroundChannel();

    // tables: Internals
    // inline    ImGuiTable*   GetCurrentTable() { ImGuiContext& g = *GImGui; return g.CurrentTable; }
    //  ImGuiTable*   TableFindByID(ImGuiID id);
    //  bool          BeginTableEx(const char* name, ImGuiID id, int columns_count, ImGuiTableFlags flags = 0, const Vector2D& outer_size = Vector2D(0, 0), float inner_width = 0.0);
    //  void          TableBeginInitMemory(ImGuiTable* table, int columns_count);
    //  void          TableBeginApplyRequests(ImGuiTable* table);
    //  void          TableSetupDrawChannels(ImGuiTable* table);
    //  void          TableUpdateLayout(ImGuiTable* table);
    //  void          TableUpdateBorders(ImGuiTable* table);
    //  void          TableUpdateColumnsWeightFromWidth(ImGuiTable* table);
    //  void          TableDrawBorders(ImGuiTable* table);
    //  void          TableDrawContextMenu(ImGuiTable* table);
    //  void          TableMergeDrawChannels(ImGuiTable* table);
    // inline ImGuiTableInstanceData*   TableGetInstanceData(ImGuiTable* table, int instance_no) { if (instance_no == 0) return &table->InstanceDataFirst; return &table->InstanceDataExtra[instance_no - 1]; }
    //  void          TableSortSpecsSanitize(ImGuiTable* table);
    //  void          TableSortSpecsBuild(ImGuiTable* table);
    //  ImGuiSortDirection TableGetColumnNextSortDirection(ImGuiTableColumn* column);
    //  void          TableFixColumnSortDirection(ImGuiTable* table, ImGuiTableColumn* column);
    //  float         TableGetColumnWidthAuto(ImGuiTable* table, ImGuiTableColumn* column);
    //  void          TableBeginRow(ImGuiTable* table);
    //  void          TableEndRow(ImGuiTable* table);
    //  void          TableBeginCell(ImGuiTable* table, int column_n);
    //  void          TableEndCell(ImGuiTable* table);
    //  ImRect        TableGetCellBgRect(const ImGuiTable* table, int column_n);
    //  const char*   TableGetColumnName(const ImGuiTable* table, int column_n);
    //  ImGuiID       TableGetColumnResizeID(const ImGuiTable* table, int column_n, int instance_no = 0);
    //  float         TableGetMaxColumnWidth(const ImGuiTable* table, int column_n);
    //  void          TableSetColumnWidthAutoSingle(ImGuiTable* table, int column_n);
    //  void          TableSetColumnWidthAutoAll(ImGuiTable* table);
    //  void          TableRemove(ImGuiTable* table);
    //  void          TableGcCompactTransientBuffers(ImGuiTable* table);
    //  void          TableGcCompactTransientBuffers(ImGuiTableTempData* table);
    //  void          TableGcCompactSettings();

    // tables: Settings
    //  void                  TableLoadSettings(ImGuiTable* table);
    //  void                  TableSaveSettings(ImGuiTable* table);
    //  void                  TableResetSettings(ImGuiTable* table);
    //  ImGuiTableSettings*   TableGetBoundSettings(ImGuiTable* table);
    //  void                  TableSettingsAddSettingsHandler();
    //  ImGuiTableSettings*   TableSettingsCreate(ImGuiID id, int columns_count);
    //  ImGuiTableSettings*   TableSettingsFindByID(ImGuiID id);
    //
    // Tab Bars
    //  bool          BeginTabBarEx(ImGuiTabBar* tab_bar, const ImRect& bb, ImGuiTabBarFlags flags, ImGuiDockNode* dock_node);
    //  ImGuiTabItem* TabBarFindTabByID(ImGuiTabBar* tab_bar, ImGuiID tab_id);
    //  ImGuiTabItem* TabBarFindMostRecentlySelectedTabForActiveWindow(ImGuiTabBar* tab_bar);
    //  void          TabBarAddTab(ImGuiTabBar* tab_bar, ImGuiTabItemFlags tab_flags, ImGuiWindow* window);
    //  void          TabBarRemoveTab(ImGuiTabBar* tab_bar, ImGuiID tab_id);
    //  void          TabBarCloseTab(ImGuiTabBar* tab_bar, ImGuiTabItem* tab);
    //  void          TabBarQueueReorder(ImGuiTabBar* tab_bar, const ImGuiTabItem* tab, int offset);
    //  void          TabBarQueueReorderFromMousePos(ImGuiTabBar* tab_bar, const ImGuiTabItem* tab, Vector2D mouse_pos);
    //  bool          TabBarProcessReorder(ImGuiTabBar* tab_bar);
    //  bool          TabItemEx(ImGuiTabBar* tab_bar, const char* label, bool* p_open, ImGuiTabItemFlags flags, ImGuiWindow* docked_window);
    //  Vector2D        TabItemCalcSize(const char* label, bool has_close_button);
    //  void          TabItemBackground(ImDrawList* draw_list, const ImRect& bb, ImGuiTabItemFlags flags, ImU32 col);
    //  void          TabItemLabelAndCloseButton(ImDrawList* draw_list, const ImRect& bb, ImGuiTabItemFlags flags, Vector2D frame_padding, const char* label, ImGuiID tab_id, ImGuiID close_button_id, bool is_contents_visible, bool* out_just_closed, bool* out_text_clipped);
    //
    // Render helpers
    // AVOID USING OUTSIDE OF IMGUI.CPP! NOT FOR PUBLIC CONSUMPTION. THOSE FUNCTIONS ARE A MESS. THEIR SIGNATURE AND BEHAVIOR WILL CHANGE, THEY NEED TO BE REFACTORED INTO SOMETHING DECENT.
    // NB: All position are in absolute pixels coordinates (we are never using window coordinates internally)
    //  void          render_text(Vector2D pos, const char* text, const char* text_end = NULL, bool hide_text_after_hash = true);
    //  void          RenderTextWrapped(Vector2D pos, const char* text, const char* text_end, float wrap_width);
    //  void          RenderTextClipped(const Vector2D& pos_min, const Vector2D& pos_max, const char* text, const char* text_end, const Vector2D* text_size_if_known, const Vector2D& align = Vector2D(0, 0), const ImRect* clip_rect = NULL);
    //  void          RenderTextClippedEx(ImDrawList* draw_list, const Vector2D& pos_min, const Vector2D& pos_max, const char* text, const char* text_end, const Vector2D* text_size_if_known, const Vector2D& align = Vector2D(0, 0), const ImRect* clip_rect = NULL);
    //  void          RenderTextEllipsis(ImDrawList* draw_list, const Vector2D& pos_min, const Vector2D& pos_max, float clip_max_x, float ellipsis_max_x, const char* text, const char* text_end, const Vector2D* text_size_if_known);
    //  void          RenderFrame(Vector2D p_min, Vector2D p_max, ImU32 fill_col, bool border = true, float rounding = 0.0);
    //  void          RenderFrameBorder(Vector2D p_min, Vector2D p_max, float rounding = 0.0);
    //  void          RenderColorRectWithAlphaCheckerboard(ImDrawList* draw_list, Vector2D p_min, Vector2D p_max, ImU32 fill_col, float grid_step, Vector2D grid_off, float rounding = 0.0, ImDrawFlags flags = 0);
    //  void          RenderNavHighlight(const ImRect& bb, ImGuiID id, ImGuiNavHighlightFlags flags = ImGuiNavHighlightFlags_TypeDefault); // Navigation highlight
    //  const char*   FindRenderedTextEnd(const char* text, const char* text_end = NULL); // Find the optional ## from which we stop displaying text.
    //  void          RenderMouseCursor(Vector2D pos, float scale, ImGuiMouseCursor mouse_cursor, ImU32 col_fill, ImU32 col_border, ImU32 col_shadow);

    // Render helpers (those functions don't access any ImGui state!)
    //  void          RenderArrow(ImDrawList* draw_list, Vector2D pos, ImU32 col, ImGuiDir dir, float scale = 1.0);
    //  void          RenderBullet(ImDrawList* draw_list, Vector2D pos, ImU32 col);
    //  void          RenderCheckMark(ImDrawList* draw_list, Vector2D pos, ImU32 col, float sz);
    //  void          RenderArrowPointingAt(ImDrawList* draw_list, Vector2D pos, Vector2D half_sz, ImGuiDir direction, ImU32 col);
    //  void          RenderArrowDockMenu(ImDrawList* draw_list, Vector2D p_min, float sz, ImU32 col);
    //  void          RenderRectFilledRangeH(ImDrawList* draw_list, const ImRect& rect, ImU32 col, float x_start_norm, float x_end_norm, float rounding);
    //  void          RenderRectFilledWithHole(ImDrawList* draw_list, const ImRect& outer, const ImRect& inner, ImU32 col, float rounding);
    //  ImDrawFlags   CalcRoundingFlagsForRectInRect(const ImRect& r_in, const ImRect& r_outer, float threshold);

    // Widgets
    //  void          TextEx(const char* text, const char* text_end = NULL, ImGuiTextFlags flags = 0);
    //  bool          ButtonEx(const char* label, const Vector2D& size_arg = Vector2D(0, 0), ImGuiButtonFlags flags = 0);
    //  bool          CloseButton(ImGuiID id, const Vector2D& pos);
    //  bool          CollapseButton(ImGuiID id, const Vector2D& pos, ImGuiDockNode* dock_node);
    //  bool          ArrowButtonEx(const char* str_id, ImGuiDir dir, Vector2D size_arg, ImGuiButtonFlags flags = 0);
    //  void          Scrollbar(ImGuiAxis axis);
    //  bool          ScrollbarEx(const ImRect& bb, ImGuiID id, ImGuiAxis axis, ImS64* p_scroll_v, ImS64 avail_v, ImS64 contents_v, ImDrawFlags flags);
    //  bool          ImageButtonEx(ImGuiID id, ImTextureID texture_id, const Vector2D& size, const Vector2D& uv0, const Vector2D& uv1, const Vector2D& padding, const Vector4D& bg_col, const Vector4D& tint_col);
    //  ImRect        GetWindowScrollbarRect(ImGuiWindow* window, ImGuiAxis axis);
    //  ImGuiID       GetWindowScrollbarID(ImGuiWindow* window, ImGuiAxis axis);
    //  ImGuiID       GetWindowResizeCornerID(ImGuiWindow* window, int n); // 0..3: corners
    //  ImGuiID       GetWindowResizeBorderID(ImGuiWindow* window, ImGuiDir dir);
    //  void          SeparatorEx(ImGuiSeparatorFlags flags);
    //  bool          CheckboxFlags(const char* label, ImS64* flags, ImS64 flags_value);
    //  bool          CheckboxFlags(const char* label, ImU64* flags, ImU64 flags_value);

    // Widgets low-level behaviors
    //  bool          ButtonBehavior(const ImRect& bb, ImGuiID id, bool* out_hovered, bool* out_held, ImGuiButtonFlags flags = 0);
    //  bool          DragBehavior(ImGuiID id, ImGuiDataType data_type, void* p_v, float v_speed, const void* p_min, const void* p_max, const char* format, ImGuiSliderFlags flags);
    //  bool          SliderBehavior(const ImRect& bb, ImGuiID id, ImGuiDataType data_type, void* p_v, const void* p_min, const void* p_max, const char* format, ImGuiSliderFlags flags, ImRect* out_grab_bb);
    //  bool          SplitterBehavior(const ImRect& bb, ImGuiID id, ImGuiAxis axis, float* size1, float* size2, float min_size1, float min_size2, float hover_extend = 0.0, float hover_visibility_delay = 0.0, ImU32 bg_col = 0);
    //  bool          TreeNodeBehavior(ImGuiID id, ImGuiTreeNodeFlags flags, const char* label, const char* label_end = NULL);
    //  bool          TreeNodeBehaviorIsOpen(ImGuiID id, ImGuiTreeNodeFlags flags = 0);                     // Consume previous SetNextItemOpen() data, if any. May return true when logging
    //  void          TreePushOverrideID(ImGuiID id);

    // Template functions are instantiated in imgui_widgets.cpp for a finite number of types.
    // To use them externally (for custom widget) you may need an "extern template" statement in your code in order to link to existing instances and silence Clang warnings (see #2036).
    // e.g. " extern template  float RoundScalarWithFormatT<float, float>(const char* format, ImGuiDataType data_type, float v); "
    // template<typename T, typename SIGNED_T, typename FLOAT_T>    float ScaleRatioFromValueT(ImGuiDataType data_type, T v, T v_min, T v_max, bool is_logarithmic, float logarithmic_zero_epsilon, float zero_deadzone_size);
    // template<typename T, typename SIGNED_T, typename FLOAT_T>    T     ScaleValueFromRatioT(ImGuiDataType data_type, float t, T v_min, T v_max, bool is_logarithmic, float logarithmic_zero_epsilon, float zero_deadzone_size);
    // template<typename T, typename SIGNED_T, typename FLOAT_T>    bool  DragBehaviorT(ImGuiDataType data_type, T* v, float v_speed, T v_min, T v_max, const char* format, ImGuiSliderFlags flags);
    // template<typename T, typename SIGNED_T, typename FLOAT_T>    bool  SliderBehaviorT(const ImRect& bb, ImGuiID id, ImGuiDataType data_type, T* v, T v_min, T v_max, const char* format, ImGuiSliderFlags flags, ImRect* out_grab_bb);
    // template<typename T>                                         T     RoundScalarWithFormatT(const char* format, ImGuiDataType data_type, T v);
    // template<typename T>                                         bool  CheckboxFlagsT(const char* label, T* flags, T flags_value);

    // data type helpers
    //  const ImGuiDataTypeInfo*  DataTypeGetInfo(ImGuiDataType data_type);
    //  int           DataTypeFormatString(char* buf, int buf_size, ImGuiDataType data_type, const void* p_data, const char* format);
    //  void          DataTypeApplyOp(ImGuiDataType data_type, int op, void* output, const void* arg_1, const void* arg_2);
    //  bool          DataTypeApplyFromText(const char* buf, ImGuiDataType data_type, void* p_data, const char* format);
    //  int           DataTypeCompare(ImGuiDataType data_type, const void* arg_1, const void* arg_2);
    //  bool          DataTypeClamp(ImGuiDataType data_type, void* p_data, const void* p_min, const void* p_max);
    //
    // InputText
    //  bool          InputTextEx(const char* label, const char* hint, char* buf, int buf_size, const Vector2D& size_arg, ImGuiInputTextFlags flags, ImGuiInputTextCallback callback = NULL, void* user_data = NULL);
    //  bool          TempInputText(const ImRect& bb, ImGuiID id, const char* label, char* buf, int buf_size, ImGuiInputTextFlags flags);
    //  bool          TempInputScalar(const ImRect& bb, ImGuiID id, const char* label, ImGuiDataType data_type, void* p_data, const char* format, const void* p_clamp_min = NULL, const void* p_clamp_max = NULL);
    // inline bool             TempInputIsActive(ImGuiID id)       { ImGuiContext& g = *GImGui; return (g.active_id == id && g.TempInputId == id); }
    // inline ImGuiInputTextState* GetInputTextState(ImGuiID id)   { ImGuiContext& g = *GImGui; return (g.InputTextState.id == id) ? &g.InputTextState : NULL; } // Get input text state if active

    // Color
    //  void          ColorTooltip(const char* text, const float* col, ImGuiColorEditFlags flags);
    //  void          ColorEditOptionsPopup(const float* col, ImGuiColorEditFlags flags);
    //  void          ColorPickerOptionsPopup(const float* ref_col, ImGuiColorEditFlags flags);

    // Plot
    //  int           PlotEx(ImGuiPlotType plot_type, const char* label, float (*values_getter)(void* data, int idx), void* data, int values_count, int values_offset, const char* overlay_text, float scale_min, float scale_max, Vector2D frame_size);
    //
    // Shade functions (write over already created vertices)
    //  void          ShadeVertsLinearColorGradientKeepAlpha(ImDrawList* draw_list, int vert_start_idx, int vert_end_idx, Vector2D gradient_p0, Vector2D gradient_p1, ImU32 col0, ImU32 col1);
    //  void          ShadeVertsLinearUV(ImDrawList* draw_list, int vert_start_idx, int vert_end_idx, const Vector2D& a, const Vector2D& b, const Vector2D& uv_a, const Vector2D& uv_b, bool clamp);

    // Garbage collection
    //  void          GcCompactTransientMiscBuffers();
    //  void          GcCompactTransientWindowBuffers(ImGuiWindow* window);
    //  void          GcAwakeTransientWindowBuffers(ImGuiWindow* window);

    // Debug Log
    //  void          DebugLog(const char* fmt, ...) IM_FMTARGS(1);
    //  void          DebugLogV(const char* fmt, va_list args) IM_FMTLIST(1);
    
    // Debug Tools
    //  void          ErrorCheckEndFrameRecover(ImGuiErrorLogCallback log_callback, void* user_data = NULL);
    //  void          ErrorCheckEndWindowRecover(ImGuiErrorLogCallback log_callback, void* user_data = NULL);
    // inline void             DebugDrawItemRect(ImU32 col = IM_COL32(255,0,0,255))    { ImGuiContext& g = *GImGui; ImGuiWindow* window = g.CurrentWindow; GetForegroundDrawList(window)->add_rect(g.LastItemData.rect.min, g.LastItemData.rect.max, col); }
    // inline void             DebugStartItemPicker()                                  { ImGuiContext& g = *GImGui; g.DebugItemPickerActive = true; }
    //  void          ShowFontAtlas(ImFontAtlas* atlas);
    //  void          DebugHookIdInfo(ImGuiID id, ImGuiDataType data_type, const void* data_id, const void* data_id_end);
    //  void          DebugNodeColumns(ImGuiOldColumns* columns);
    //  void          DebugNodeDockNode(ImGuiDockNode* node, const char* label);
    //  void          DebugNodeDrawList(ImGuiWindow* window, ImGuiViewportP* viewport, const ImDrawList* draw_list, const char* label);
    //  void          DebugNodeDrawCmdShowMeshAndBoundingBox(ImDrawList* out_draw_list, const ImDrawList* draw_list, const ImDrawCmd* draw_cmd, bool show_mesh, bool show_aabb);
    //  void          DebugNodeFont(ImFont* font);
    //  void          DebugNodeFontGlyph(ImFont* font, const ImFontGlyph* glyph);
    //  void          DebugNodeStorage(ImGuiStorage* storage, const char* label);
    //  void          DebugNodeTabBar(ImGuiTabBar* tab_bar, const char* label);
    //  void          DebugNodeTable(ImGuiTable* table);
    //  void          DebugNodeTableSettings(ImGuiTableSettings* settings);
    //  void          DebugNodeInputTextState(ImGuiInputTextState* state);
    //  void          DebugNodeWindow(ImGuiWindow* window, const char* label);
    //  void          DebugNodeWindowSettings(ImGuiWindowSettings* settings);
    //  void          DebugNodeWindowsList(ImVector<ImGuiWindow*>* windows, const char* label);
    //  void          DebugNodeWindowsListByBeginStackParent(ImGuiWindow** windows, int windows_size, ImGuiWindow* parent_in_begin_stack);
    //  void          DebugNodeViewport(ImGuiViewportP* viewport);
    //  void          DebugRenderViewportThumbnail(ImDrawList* draw_list, ImGuiViewportP* viewport, const ImRect& bb);

// } // namespace ImGui


//-----------------------------------------------------------------------------
// [SECTION] ImFontAtlas internal API
//-----------------------------------------------------------------------------

// Helper for font builder
// #ifdef IMGUI_ENABLE_STB_TRUETYPE
//  const ImFontBuilderIO* ImFontAtlasGetBuilderForStbTruetype();
// #endif
//  void      ImFontAtlasBuildInit(ImFontAtlas* atlas);
//  void      ImFontAtlasBuildSetupFont(ImFontAtlas* atlas, ImFont* font, ImFontConfig* font_config, float ascent, float descent);
//  void      ImFontAtlasBuildPackCustomRects(ImFontAtlas* atlas, void* stbrp_context_opaque);
//  void      ImFontAtlasBuildFinish(ImFontAtlas* atlas);
//  void      ImFontAtlasBuildRender8bppRectFromString(ImFontAtlas* atlas, int x, int y, int w, int h, const char* in_str, char in_marker_char, unsigned char in_marker_pixel_value);
//  void      ImFontAtlasBuildRender32bppRectFromString(ImFontAtlas* atlas, int x, int y, int w, int h, const char* in_str, char in_marker_char, unsigned int in_marker_pixel_value);
//  void      ImFontAtlasBuildMultiplyCalcLookupTable(unsigned char out_table[256], float in_multiply_factor);
//  void      ImFontAtlasBuildMultiplyRectAlpha8(const unsigned char table[256], unsigned char* pixels, int x, int y, int w, int h, int stride);

//-----------------------------------------------------------------------------
// [SECTION] Test Engine specific hooks (imgui_test_engine)
//-----------------------------------------------------------------------------

// #ifdef IMGUI_ENABLE_TEST_ENGINE
// extern void         ImGuiTestEngineHook_ItemAdd(ImGuiContext* ctx, const ImRect& bb, ImGuiID id);
// extern void         ImGuiTestEngineHook_ItemInfo(ImGuiContext* ctx, ImGuiID id, const char* label, ImGuiItemStatusFlags flags);
// extern void         ImGuiTestEngineHook_Log(ImGuiContext* ctx, const char* fmt, ...);
// extern const char*  ImGuiTestEngine_FindItemDebugLabel(ImGuiContext* ctx, ImGuiID id);
//
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
//
// #ifdef _MSC_VER
// #pragma warning (pop)
// #endif
//
// #endif // #ifndef IMGUI_DISABLE
