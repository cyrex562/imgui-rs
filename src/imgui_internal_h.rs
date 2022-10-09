
static inline c_void      ImQsort(*mut c_void base, size_t count, size_t size_of_element, c_int(IMGUI_CDECL *compare_func)(c_void *mut const, c_void *mut const)) { if (count > 1) qsort(base, count, size_of_element, compare_func); }
// #endif

// Helpers: Color Blending
 u32         ImAlphaBlendColors(u32 col_a, u32 col_b);

// Helpers: Bit manipulation
static inline bool      ImIsPowerOfTwo(c_int v)           { return v != 0 && (v & (v - 1)) == 0; }
static inline bool      ImIsPowerOfTwo(u64 v)         { return v != 0 && (v & (v - 1)) == 0; }
static inline c_int       ImUpperPowerOfTwo(c_int v)        { v-= 1; v |= v >> 1; v |= v >> 2; v |= v >> 4; v |= v >> 8; v |= v >> 16; v+= 1; return v; }

// Helpers: String
 c_int           ImStricmp(*const char str1, *const char str2);
 c_int           ImStrnicmp(*const char str1, *const char str2, size_t count);
 c_void          ImStrncpy(*mut char dst, *const char src, size_t count);
 *mut char         ImStrdup(*const char str);
 *mut char         ImStrdupcpy(*mut char dst, *mut size_t p_dst_size, *const char str);
 *const char   ImStrchrRange(*const char str_begin, *const char str_end, char c);
 c_int           ImStrlenW(*const ImWchar str);
 *const char   ImStreolRange(*const char str, *const char str_end);                // End end-of-line
 *const ImWcharImStrbolW(*const ImWchar buf_mid_line, *const ImWchar buf_begin);   // Find beginning-of-line
 *const char   ImStristr(*const char haystack, *const char haystack_end, *const char needle, *const char needle_end);
 c_void          ImStrTrimBlanks(*mut char str);
 *const char   ImStrSkipBlank(*const char str);
static inline bool      ImCharIsBlankA(char c)          { return c == ' ' || c == '\t'; }
static inline bool      ImCharIsBlankW(c_uint c)  { return c == ' ' || c == '\t' || c == 0x3000; }

// Helpers: Formatting
 c_int           ImFormatString(*mut char buf, size_t buf_size, *const char fmt, ...) IM_FMTARGS(3);
 c_int           ImFormatStringV(*mut char buf, size_t buf_size, *const char fmt, va_list args) IM_FMTLIST(3);
 c_void          ImFormatStringToTempBuffer(*const *mut char out_buf, *const *mut char out_buf_end, *const char fmt, ...) IM_FMTARGS(3);
 c_void          ImFormatStringToTempBufferV(*const *mut char out_buf, *const *mut char out_buf_end, *const char fmt, va_list args) IM_FMTLIST(3);
 *const char   ImParseFormatFindStart(*const char format);
 *const char   ImParseFormatFindEnd(*const char format);
 *const char   ImParseFormatTrimDecorations(*const char format, *mut char buf, size_t buf_size);
 c_void          ImParseFormatSanitizeForPrinting(*const char fmt_in, *mut char fmt_out, size_t fmt_out_size);
 *const char   ImParseFormatSanitizeForScanning(*const char fmt_in, *mut char fmt_out, size_t fmt_out_size);
 c_int           ImParseFormatPrecision(*const char format, c_int default_value);

// Helpers: UTF-8 <> wchar conversions
 *const char   ImTextCharToUtf8(out_buf: [c_char;5], c_uint c);                                                      // return out_buf
 c_int           ImTextStrToUtf8(*mut char out_buf, c_int out_buf_size, *const ImWchar in_text, *const ImWchar in_text_end);   // return output UTF-8 bytes count
 c_int           ImTextCharFromUtf8(*mut c_uint out_char, *const char in_text, *const char in_text_end);               // read one character. return input UTF-8 bytes count
 c_int           ImTextStrFromUtf8(*mut ImWchar out_buf, c_int out_buf_size, *const char in_text, *const char in_text_end, *const *mut char in_remaining = null_mut());   // return input UTF-8 bytes count
 c_int           ImTextCountCharsFromUtf8(*const char in_text, *const char in_text_end);                                 // return number of UTF-8 code-points (NOT bytes count)
 c_int           ImTextCountUtf8BytesFromChar(*const char in_text, *const char in_text_end);                             // return number of bytes to express one char in UTF-8
 c_int           ImTextCountUtf8BytesFromStr(*const ImWchar in_text, *const ImWchar in_text_end);                        // return number of bytes to express string in UTF-8

// Helpers: ImVec2/ImVec4 operators
// We are keeping those disabled by default so they don't leak in user space, to allow user enabling implicit cast operators between ImVec2 and their own types (using IM_VEC2_CLASS_EXTRA etc.)
// We unfortunately don't have a unary- operator for ImVec2 because this would needs to be defined inside the class itself.
// #ifdef IMGUI_DEFINE_MATH_OPERATORS
IM_MSVC_RUNTIME_CHECKS_OFF
static inline ImVec2 *mut operator(lhs: &ImVec2, rhs: c_float)              { return ImVec2(lhs.x * rhs, lhs.y * rhs); }
static inline ImVec2 operator/(lhs: &ImVec2, rhs: c_float)              { return ImVec2(lhs.x / rhs, lhs.y / rhs); }
static inline ImVec2 operator+(lhs: &ImVec2, rhs: &ImVec2)            { return ImVec2(lhs.x + rhs.x, lhs.y + rhs.y); }
static inline ImVec2 operator-(lhs: &ImVec2, rhs: &ImVec2)            { return ImVec2(lhs.x - rhs.x, lhs.y - rhs.y); }
static inline ImVec2 *mut operator(lhs: &ImVec2, rhs: &ImVec2)            { return ImVec2(lhs.x * rhs.x, lhs.y * rhs.y); }
static inline ImVec2 operator/(lhs: &ImVec2, rhs: &ImVec2)            { return ImVec2(lhs.x / rhs.x, lhs.y / rhs.y); }
static inline ImVec2& *mut operator=(ImVec2& lhs, rhs: c_float)                  { lhs.x *= rhs; lhs.y *= rhs; return lhs; }
static inline ImVec2& operator/=(ImVec2& lhs, rhs: c_float)                  { lhs.x /= rhs; lhs.y /= rhs; return lhs; }
static inline ImVec2& operator+=(ImVec2& lhs, rhs: &ImVec2)                { lhs.x += rhs.x; lhs.y += rhs.y; return lhs; }
static inline ImVec2& operator-=(ImVec2& lhs, rhs: &ImVec2)                { lhs.x -= rhs.x; lhs.y -= rhs.y; return lhs; }
static inline ImVec2& *mut operator=(ImVec2& lhs, rhs: &ImVec2)                { lhs.x *= rhs.x; lhs.y *= rhs.y; return lhs; }
static inline ImVec2& operator/=(ImVec2& lhs, rhs: &ImVec2)                { lhs.x /= rhs.x; lhs.y /= rhs.y; return lhs; }
static inline ImVec4 operator+(const ImVec4& lhs, const ImVec4& rhs)            { return ImVec4(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z, lhs.w + rhs.w); }
static inline ImVec4 operator-(const ImVec4& lhs, const ImVec4& rhs)            { return ImVec4(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z, lhs.w - rhs.w); }
static inline ImVec4 *mut operator(const ImVec4& lhs, const ImVec4& rhs)            { return ImVec4(lhs.x * rhs.x, lhs.y * rhs.y, lhs.z * rhs.z, lhs.w * rhs.w); }

typedef *mut c_void ImFileHandle;
static inline ImFileHandle  ImFileOpen(*const char, *const char)                    { return null_mut(); }
static inline bool          ImFileClose(ImFileHandle)                               { return false; }
static inline u64         ImFileGetSize(ImFileHandle)                             { return (u64)-1; }
static inline u64         ImFileRead(*mut c_void, u64, u64, ImFileHandle)           { return 0; }
static inline u64         ImFileWrite(*const c_void, u64, u64, ImFileHandle)    { return 0; }

// Helper: ImBitArray
inline bool     ImBitArrayTestBit(*const u32 arr, c_int n)      { u32 mask = 1 << (n & 31); return (arr[n >> 5] & mask) != 0; }
inline c_void     ImBitArrayClearBit(*mut u32 arr, c_int n)           { u32 mask = 1 << (n & 31); arr[n >> 5] &= ~mask; }
inline c_void     ImBitArraySetBit(*mut u32 arr, c_int n)             { u32 mask = 1 << (n & 31); arr[n >> 5] |= mask; }
inline c_void     ImBitArraySetBitRange(*mut u32 arr, c_int n, c_int n2) // Works on range [n..n2)
{
    n2-= 1;
    while (n <= n2)
    {
        let a_mod: c_int = (n & 31);
        let b_mod: c_int = (n2 > (n | 31) ? 31 : (n2 & 31)) + 1;
        u32 mask = (((u64)1 << b_mod) - 1) & ~(((u64)1 << a_mod) - 1);
        arr[n >> 5] |= mask;
        n = (n + 32) & ~31;
    }
}

// Helper: ImBitArray class (wrapper over ImBitArray functions)
// Store 1-bit per value.
template<c_int BITCOUNT, let OFFSET: c_int = 0>
struct ImBitArray
{
    u32           Storage[(BITCOUNT + 31) >> 5];
    ImBitArray()                                { ClearAllBits(); }
    c_void            ClearAllBits()              { memset(Storage, 0, sizeof(Storage)); }
    c_void            SetAllBits()                { memset(Storage, 255, sizeof(Storage)); }
    bool            TestBit(c_int n) const        { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); return ImBitArrayTestBit(Storage, n); }
    c_void            SetBit(c_int n)               { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); ImBitArraySetBit(Storage, n); }
    c_void            ClearBit(c_int n)             { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); ImBitArrayClearBit(Storage, n); }
    c_void            SetBitRange(c_int n, c_int n2)  { n += OFFSET; n2 += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT && n2 > n && n2 <= BITCOUNT); ImBitArraySetBitRange(Storage, n, n2); } // Works on range [n..n2)
    bool            operator[](c_int n) const     { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); return ImBitArrayTestBit(Storage, n); }
};

// Helper: ImBitVector
// Store 1-bit per value.
struct  ImBitVector
{
    Vec<u32> Storage;
    c_void            Create(c_int sz)              { Storage.resize((sz + 31) >> 5); memset(Storage.Data, 0, Storage.Size * sizeof(Storage.Data[0])); }
    c_void            Clear()                     { Storage.clear(); }
    bool            TestBit(c_int n) const        { IM_ASSERT(n < (Storage.Size << 5)); return ImBitArrayTestBit(Storage.Data, n); }
    c_void            SetBit(c_int n)               { IM_ASSERT(n < (Storage.Size << 5)); ImBitArraySetBit(Storage.Data, n); }
    c_void            ClearBit(c_int n)             { IM_ASSERT(n < (Storage.Size << 5)); ImBitArrayClearBit(Storage.Data, n); }
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
    inline c_void  Reserve(c_int n, size_t sz, let mut a: c_int = 4) { IM_ASSERT(n == CurrIdx && n < CHUNKS); CurrOff = IM_MEMALIGN(CurrOff, a); Offsets[n] = CurrOff; Sizes[n] = sz; CurrIdx+= 1; CurrOff += sz; }
    inline c_int   GetArenaSizeInBytes()              { return CurrOff; }
    inline c_void  SetArenaBasePtr(*mut c_void base_ptr)    { BasePtr = (*mut char)base_ptr; }
    inline *mut c_void GetSpanPtrBegin(c_int n)             { IM_ASSERT(n >= 0 && n < CHUNKS && CurrIdx == CHUNKS); return (*mut c_void)(BasePtr + Offsets[n]); }
    inline *mut c_void GetSpanPtrEnd(c_int n)               { IM_ASSERT(n >= 0 && n < CHUNKS && CurrIdx == CHUNKS); return (*mut c_void)(BasePtr + Offsets[n] + Sizes[n]); }
    template<typename T>
    inline c_void  GetSpan(c_int n, ImSpan<T>* span)    { span.set((*mut T)GetSpanPtrBegin(n), (*mut T)GetSpanPtrEnd(n)); }
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
    u8        Data[8];        // Can fit any data up to ImGuiDataType_COUNT
};

// Type information associated to one ImGuiDataType. Retrieve with DataTypeGetInfo().
struct ImGuiDataTypeInfo
{
    size_t      Size;           // Size in bytes
let Name: *const c_char;           // Short descriptive name for the type, for debugging
let PrintFmt: *const c_char;       // Default printf format for the type
let ScanFmt: *const c_char;        // Default scanf format for the type
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
    c_void        Update(spacing: c_float, bool window_reappearing);DeclColumns: c_float(w_icon: c_float,w_label: c_float,w_shortcut: c_float,w_mark: c_float);
    c_void        CalcNextTotalWidth(bool update_offsets);
};




enum ImGuiNextItemDataFlags_
{
    ImGuiNextItemDataFlags_None     = 0,
    ImGuiNextItemDataFlags_HasWidth = 1 << 0,
    ImGuiNextItemDataFlags_HasOpen  = 1 << 1,
};








//-----------------------------------------------------------------------------
// [SECTION] Inputs support
//-----------------------------------------------------------------------------
















//-----------------------------------------------------------------------------
// [SECTION] Navigation support
//-----------------------------------------------------------------------------











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







// #endif // #ifdef IMGUI_HAS_DOCK

//-----------------------------------------------------------------------------
// [SECTION] Viewport support
//-----------------------------------------------------------------------------

// ImGuiViewport Private/Internals fields (cardinal sin: we are using inheritance!)
// Every instance of ImGuiViewport is in fact a ImGuiViewportP.

//-----------------------------------------------------------------------------
// [SECTION] Settings support
//-----------------------------------------------------------------------------





//-----------------------------------------------------------------------------
// [SECTION] Metrics, Debug Tools
//-----------------------------------------------------------------------------









//-----------------------------------------------------------------------------
// [SECTION] Generic context hooks
//-----------------------------------------------------------------------------




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

//-----------------------------------------------------------------------------
// [SECTION] Table support
//-----------------------------------------------------------------------------


// #define IMGUI_TABLE_MAX_COLUMNS         64                  // sizeof(u64) * 8. This is solely because we frequently encode columns set in a u64.
// #define IMGUI_TABLE_MAX_DRAW_CHANNELS   (4 + 64 * 2)        // See TableSetupDrawChannels()











//-----------------------------------------------------------------------------
// [SECTION] ImGui internal API
// No guarantee of forward compatibility here!
//-----------------------------------------------------------------------------

namespace ImGui
{
    // Windows
    // We should always have a CurrentWindow in the stack (there is an implicit "Debug" window)
    // If this ever crash because g.CurrentWindow is NULL it means that either
    // - NewFrame() has never been called, which is illegal.
    // - You are calling ImGui functions after EndFrame()/Render() and before the next NewFrame(), which is also illegal.
    inline    *mut ImGuiWindow  GetCurrentWindowRead()      { let g = GImGui; // ImGuiContext& g = *GImGui; return g.CurrentWindow; }
    inline    *mut ImGuiWindow  GetCurrentWindow()          { let g = GImGui; // ImGuiContext& g = *GImGui; g.Currentwindow.WriteAccessed = true; return g.CurrentWindow; }
     *mut ImGuiWindow  FindWindowByID(id: ImGuiID);
     *mut ImGuiWindow  FindWindowByName(*const char name);
     c_void          UpdateWindowParentAndRootLinks(*mut ImGuiWindow window, ImGuiWindowFlags flags, *mut ImGuiWindow parent_window);
     ImVec2        CalcWindowNextAutoFitSize(*mut ImGuiWindow window);
     bool          IsWindowChildOf(*mut ImGuiWindow window, *mut ImGuiWindow potential_parent, bool popup_hierarchy, bool dock_hierarchy);
     bool          IsWindowWithinBeginStackOf(*mut ImGuiWindow window, *mut ImGuiWindow potential_parent);
     bool          IsWindowAbove(*mut ImGuiWindow potential_above, *mut ImGuiWindow potential_below);
     bool          IsWindowNavFocusable(*mut ImGuiWindow window);
     c_void          SetWindowPos(*mut ImGuiWindow window, pos: &ImVec2, cond: ImGuiCond = 0);
     c_void          SetWindowSize(*mut ImGuiWindow window, size: &ImVec2, cond: ImGuiCond = 0);
     c_void          SetWindowCollapsed(*mut ImGuiWindow window, bool collapsed, cond: ImGuiCond = 0);
     c_void          SetWindowHitTestHole(*mut ImGuiWindow window, pos: &ImVec2, size: &ImVec2);



    // Windows: Display Order and Focus Order
     c_void          FocusWindow(*mut ImGuiWindow window);
     c_void          FocusTopMostWindowUnderOne(*mut ImGuiWindow under_this_window, *mut ImGuiWindow ignore_window);
     c_void          BringWindowToFocusFront(*mut ImGuiWindow window);
     c_void          BringWindowToDisplayFront(*mut ImGuiWindow window);
     c_void          BringWindowToDisplayBack(*mut ImGuiWindow window);
     c_void          BringWindowToDisplayBehind(*mut ImGuiWindow window, *mut ImGuiWindow above_window);
     c_int           FindWindowDisplayIndex(*mut ImGuiWindow window);
     *mut ImGuiWindow  FindBottomMostVisibleWindowWithinBeginStack(*mut ImGuiWindow window);

    // Fonts, drawing
     c_void          SetCurrentFont(*mut ImFont font);
    inline *mut ImFont          GetDefaultFont() { let g = GImGui; // ImGuiContext& g = *GImGui; return g.IO.FontDefault ? g.IO.FontDefault : g.IO.Fonts.Fonts[0]; }
    inline *mut ImDrawList      GetForegroundDrawList(*mut ImGuiWindow window) { return GetForegroundDrawList(window.Viewport); }

    // Init
     c_void          Initialize();
     c_void          Shutdown();    // Since 1.60 this is a _private_ function. You can call DestroyContext() to destroy the context created by CreateContext().

    // NewFrame
     c_void          UpdateInputEvents(bool trickle_fast_inputs);
     c_void          UpdateHoveredWindowAndCaptureFlags();
     c_void          StartMouseMovingWindow(*mut ImGuiWindow window);
     c_void          StartMouseMovingWindowOrNode(*mut ImGuiWindow window, *mut ImGuiDockNode node, bool undock_floating_node);
     c_void          UpdateMouseMovingWindowNewFrame();
     c_void          UpdateMouseMovingWindowEndFrame();

    // Generic context hooks
     ImGuiID       AddContextHook(*mut ImGuiContext context, *const ImGuiContextHook hook);
     c_void          RemoveContextHook(*mut ImGuiContext context, ImGuiID hook_to_remove);
     c_void          CallContextHooks(*mut ImGuiContext context, ImGuiContextHookType type);

    // Viewports
     c_void          TranslateWindowsInViewport(*mut ImGuiViewportP viewport, old_pos: &ImVec2, new_pos: &ImVec2);
     c_void          ScaleWindowsInViewport(*mut ImGuiViewportP viewport,scale: c_float);
     c_void          DestroyPlatformWindow(*mut ImGuiViewportP viewport);
     c_void          SetWindowViewport(*mut ImGuiWindow window, *mut ImGuiViewportP viewport);
     c_void          SetCurrentViewport(*mut ImGuiWindow window, *mut ImGuiViewportP viewport);
     *const ImGuiPlatformMonitor   GetViewportPlatformMonitor(*mut ImGuiViewport viewport);
     *mut ImGuiViewportP               FindHoveredViewportFromPlatformWindowStack(mouse_platform_pos: &ImVec2);

    // Settings
     c_void                  MarkIniSettingsDirty();
     c_void                  MarkIniSettingsDirty(*mut ImGuiWindow window);
     c_void                  ClearIniSettings();
     *mut ImGuiWindowSettings  CreateNewWindowSettings(*const char name);
     *mut ImGuiWindowSettings  FindWindowSettings(id: ImGuiID);
     *mut ImGuiWindowSettings  FindOrCreateWindowSettings(*const char name);
     c_void                  AddSettingsHandler(*const ImGuiSettingsHandler handler);
     c_void                  RemoveSettingsHandler(*const char type_name);
     *mut ImGuiSettingsHandler FindSettingsHandler(*const char type_name);

    // Scrolling
     c_void          SetNextWindowScroll(scroll: &ImVec2); // Use -1f32 on one axis to leave as-is
     c_void          SetScrollX(*mut ImGuiWindow window,scroll_x: c_float);
     c_void          SetScrollY(*mut ImGuiWindow window,scroll_y: c_float);
     c_void          SetScrollFromPosX(*mut ImGuiWindow window,local_x: c_float,center_x_ratio: c_float);
     c_void          SetScrollFromPosY(*mut ImGuiWindow window,local_y: c_float,center_y_ratio: c_float);

    // Early work-in-progress API (ScrollToItem() will become public)
     c_void          ScrollToItem(ImGuiScrollFlags flags = 0);
     c_void          ScrollToRect(*mut ImGuiWindow window, rect: &ImRect, ImGuiScrollFlags flags = 0);
     ImVec2        ScrollToRectEx(*mut ImGuiWindow window, rect: &ImRect, ImGuiScrollFlags flags = 0);
//#ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    inline c_void             ScrollToBringRectIntoView(*mut ImGuiWindow window, rect: &ImRect) { ScrollToRect(window, rect, ImGuiScrollFlags_KeepVisibleEdgeY); }
//#endif

    // Basic Accessors
    inline ImGuiID          GetItemID()     { let g = GImGui; // ImGuiContext& g = *GImGui; return g.LastItemData.ID; }   // Get ID of last item (~~ often same GetID(label) beforehand)
    inline ImGuiItemStatusFlags GetItemStatusFlags(){ let g = GImGui; // ImGuiContext& g = *GImGui; return g.LastItemData.StatusFlags; }
    inline ImGuiItemFlags   GetItemFlags()  { let g = GImGui; // ImGuiContext& g = *GImGui; return g.LastItemData.InFlags; }
    inline ImGuiID          GetActiveID()   { let g = GImGui; // ImGuiContext& g = *GImGui; return g.ActiveId; }
    inline ImGuiID          GetFocusID()    { let g = GImGui; // ImGuiContext& g = *GImGui; return g.NavId; }
     c_void          SetActiveID(id: ImGuiID, *mut ImGuiWindow window);
     c_void          SetFocusID(id: ImGuiID, *mut ImGuiWindow window);
     c_void          ClearActiveID();
     ImGuiID       GetHoveredID();
     c_void          SetHoveredID(id: ImGuiID);
     c_void          KeepAliveID(id: ImGuiID);
     c_void          MarkItemEdited(id: ImGuiID);     // Mark data associated to given item as "edited", used by IsItemDeactivatedAfterEdit() function.
     c_void          PushOverrideID(id: ImGuiID);     // Push given value as-is at the top of the ID stack (whereas PushID combines old and new hashes)
     ImGuiID       GetIDWithSeed(*const char str_id_begin, *const char str_id_end, ImGuiID seed);

    // Basic Helpers for widget code
     c_void          ItemSize(size: &ImVec2, let text_baseline_y: c_float =  -1f32);
    inline c_void             ItemSize(bb: &ImRect, let text_baseline_y: c_float =  -1f32) { ItemSize(bb.GetSize(), text_baseline_y); } // FIXME: This is a misleading API since we expect CursorPos to be bb.Min.
     bool          ItemAdd(bb: &ImRect, id: ImGuiID, *const let nav_bb: ImRect =  null_mut(), let mut extra_flags: ImGuiItemFlags =  0);
     bool          ItemHoverable(bb: &ImRect, id: ImGuiID);
     bool          IsClippedEx(bb: &ImRect, id: ImGuiID);
     c_void          SetLastItemData(ImGuiID item_id, in_flags: ImGuiItemFlags, ImGuiItemStatusFlags status_flags, item_rect: &ImRect);
     ImVec2        CalcItemSize(ImVec2 size,default_w: c_float,default_h: c_float);CalcWrapWidthForPos: c_float(pos: &ImVec2,wrap_pos_x: c_float);
     c_void          PushMultiItemsWidths(c_int components,width_full: c_float);
     bool          IsItemToggledSelection();                                   // Was the last item selection toggled? (after Selectable(), TreeNode() etc. We only returns toggle _event_ in order to handle clipping correctly)
     ImVec2        GetContentRegionMaxAbs();
     c_void          ShrinkWidths(*mut ImGuiShrinkWidthItem items, c_int count,width_excess: c_float);

    // Parameter stacks
     c_void          PushItemFlag(option: ImGuiItemFlags, bool enabled);
     c_void          PopItemFlag();

    // Logging/Capture
     c_void          LogBegin(ImGuiLogType type, c_int auto_open_depth);           // -> BeginCapture() when we design v2 api, for now stay under the radar by using the old name.
     c_void          LogToBuffer(let auto_open_depth: c_int = -1);                      // Start logging/capturing to internal buffer
     c_void          LogRenderedText(*const ImVec2 ref_pos, *const char text, *const char text_end = null_mut());
     c_void          LogSetNextTextDecoration(*const char prefix, *const char suffix);

    // Popups, Modals, Tooltips
     bool          BeginChildEx(*const char name, id: ImGuiID, size_arg: &ImVec2, bool border, ImGuiWindowFlags flags);
     c_void          OpenPopupEx(id: ImGuiID, ImGuiPopupFlags popup_flags = ImGuiPopupFlags_None);
     c_void          ClosePopupToLevel(c_int remaining, bool restore_focus_to_window_under_popup);
     c_void          ClosePopupsOverWindow(*mut ImGuiWindow ref_window, bool restore_focus_to_window_under_popup);
     c_void          ClosePopupsExceptModals();
     bool          IsPopupOpen(id: ImGuiID, ImGuiPopupFlags popup_flags);
     bool          BeginPopupEx(id: ImGuiID, ImGuiWindowFlags extra_flags);
     c_void          BeginTooltipEx(ImGuiTooltipFlags tooltip_flags, ImGuiWindowFlags extra_window_flags);
     ImRect        GetPopupAllowedExtentRect(*mut ImGuiWindow window);
     *mut ImGuiWindow  GetTopMostPopupModal();
     *mut ImGuiWindow  GetTopMostAndVisiblePopupModal();
     ImVec2        FindBestWindowPosForPopup(*mut ImGuiWindow window);
     ImVec2        FindBestWindowPosForPopupEx(ref_pos: &ImVec2, size: &ImVec2, *mut ImGuiDir last_dir, r_outer: &ImRect, r_avoid: &ImRect, ImGuiPopupPositionPolicy policy);

    // Menus
     bool          BeginViewportSideBar(*const char name, *mut ImGuiViewport viewport, ImGuiDir dir,size: c_float, ImGuiWindowFlags window_flags);
     bool          BeginMenuEx(*const char label, *const char icon, let mut enabled: bool =  true);
     bool          MenuItemEx(*const char label, *const char icon, *const char shortcut = null_mut(), let mut selected: bool =  false, let mut enabled: bool =  true);

    // Combos
     bool          BeginComboPopup(ImGuiID popup_id, bb: &ImRect, ImGuiComboFlags flags);
     bool          BeginComboPreview();
     c_void          EndComboPreview();

    // Gamepad/Keyboard Navigation
     c_void          NavInitWindow(*mut ImGuiWindow window, bool force_reinit);
     c_void          NavInitRequestApplyResult();
     bool          NavMoveRequestButNoResultYet();
     c_void          NavMoveRequestSubmit(ImGuiDir move_dir, ImGuiDir clip_dir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags);
     c_void          NavMoveRequestForward(ImGuiDir move_dir, ImGuiDir clip_dir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags);
     c_void          NavMoveRequestResolveWithLastItem(*mut ImGuiNavItemData result);
     c_void          NavMoveRequestCancel();
     c_void          NavMoveRequestApplyResult();
     c_void          NavMoveRequestTryWrapping(*mut ImGuiWindow window, ImGuiNavMoveFlags move_flags);
     c_void          ActivateItem(id: ImGuiID);   // Remotely activate a button, checkbox, tree node etc. given its unique ID. activation is queued and processed on the next frame when the item is encountered again.
     c_void          SetNavWindow(*mut ImGuiWindow window);
     c_void          SetNavID(id: ImGuiID, ImGuiNavLayer nav_layer, ImGuiID focus_scope_id, rect_rel: &ImRect);

    // Focus Scope (WIP)
    // This is generally used to identify a selection set (multiple of which may be in the same window), as selection
    // patterns generally need to react (e.g. clear selection) when landing on an item of the set.
     c_void          PushFocusScope(id: ImGuiID);
     c_void          PopFocusScope();
    inline ImGuiID          GetFocusedFocusScope()          { let g = GImGui; // ImGuiContext& g = *GImGui; return g.NavFocusScopeId; }                            // Focus scope which is actually active
    inline ImGuiID          GetFocusScope()                 { let g = GImGui; // ImGuiContext& g = *GImGui; return g.Currentwindow.DC.NavFocusScopeIdCurrent; }   // Focus scope we are outputting into, set by PushFocusScope()

    // Inputs
    // FIXME: Eventually we should aim to move e.g. IsActiveIdUsingKey() into IsKeyXXX functions.

     c_void          GetKeyChordName(ImGuiModFlags mods, ImGuiKey key, *mut char out_buf, c_int out_buf_size);
     c_void          SetItemUsingMouseWheel();
     c_void          SetActiveIdUsingAllKeyboardKeys();
    inline bool             IsActiveIdUsingNavDir(ImGuiDir dir)                         { let g = GImGui; // ImGuiContext& g = *GImGui; return (g.ActiveIdUsingNavDirMask & (1 << dir)) != 0; }
    inline bool             IsActiveIdUsingKey(ImGuiKey key)                            { let g = GImGui; // ImGuiContext& g = *GImGui; return g.ActiveIdUsingKeyInputMask[key]; }
    inline c_void             SetActiveIdUsingKey(ImGuiKey key)                           { let g = GImGui; // ImGuiContext& g = *GImGui; g.ActiveIdUsingKeyInputMask.SetBit(key); }
    inline ImGuiKey         MouseButtonToKey(ImGuiMouseButton button)                   { IM_ASSERT(button >= 0 && button < ImGuiMouseButton_COUNT); return ImGuiKey_MouseLeft + button; }
     bool          IsMouseDragPastThreshold(ImGuiMouseButton button, let lock_threshold: c_float =  -1f32);
     ImGuiModFlags GetMergedModFlags();
     ImVec2        GetKeyVector2d(ImGuiKey key_left, ImGuiKey key_right, ImGuiKey key_up, ImGuiKey key_down);GetNavTweakPressedAmount: c_float(ImGuiAxis axis);
     c_int           CalcTypematicRepeatAmount(t0: c_float,t1: c_float,repeat_delay: c_float,repeat_rate: c_float);
     c_void          GetTypematicRepeatRate(ImGuiInputFlags flags, *mutrepeat_delay: c_float, *mutrepeat_rate: c_float);
     bool          IsKeyPressedEx(ImGuiKey key, ImGuiInputFlags flags = 0);
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    inline bool             IsKeyPressedMap(ImGuiKey key, let mut repeat: bool =  true)           { IM_ASSERT(IsNamedKey(key)); return IsKeyPressed(key, repeat); } // [removed in 1.87]
// #endif

    // Docking
    // (some functions are only declared in imgui.cpp, see Docking section)
     c_void          DockContextInitialize(*mut ImGuiContext ctx);
     c_void          DockContextShutdown(*mut ImGuiContext ctx);
     c_void          DockContextClearNodes(*mut ImGuiContext ctx, ImGuiID root_id, bool clear_settings_refs); // Use root_id==0 to clear all
     c_void          DockContextRebuildNodes(*mut ImGuiContext ctx);
     c_void          DockContextNewFrameUpdateUndocking(*mut ImGuiContext ctx);
     c_void          DockContextNewFrameUpdateDocking(*mut ImGuiContext ctx);
     c_void          DockContextEndFrame(*mut ImGuiContext ctx);
     ImGuiID       DockContextGenNodeID(*mut ImGuiContext ctx);
     c_void          DockContextQueueDock(*mut ImGuiContext ctx, *mut ImGuiWindow target, *mut ImGuiDockNode target_node, *mut ImGuiWindow payload, ImGuiDir split_dir,split_ratio: c_float, bool split_outer);
     c_void          DockContextQueueUndockWindow(*mut ImGuiContext ctx, *mut ImGuiWindow window);
     c_void          DockContextQueueUndockNode(*mut ImGuiContext ctx, *mut ImGuiDockNode node);
     bool          DockContextCalcDropPosForDocking(*mut ImGuiWindow target, *mut ImGuiDockNode target_node, *mut ImGuiWindow payload_window, *mut ImGuiDockNode payload_node, ImGuiDir split_dir, bool split_outer, *mut ImVec2 out_pos);
     *mut ImGuiDockNodeDockContextFindNodeByID(*mut ImGuiContext ctx, id: ImGuiID);
     bool          DockNodeBeginAmendTabBar(*mut ImGuiDockNode node);
     c_void          DockNodeEndAmendTabBar();
    inline *mut ImGuiDockNode   DockNodeGetRootNode(*mut ImGuiDockNode node)                 { while (node.ParentNode) node = node.ParentNode; return node; }
    inline bool             DockNodeIsInHierarchyOf(*mut ImGuiDockNode node, *mut ImGuiDockNode parent) { while (node) { if (node == parent) return true; node = node.ParentNode; } return false; }
    inline c_int              DockNodeGetDepth(*const ImGuiDockNode node)              { let depth: c_int = 0; while (node.ParentNode) { node = node.ParentNode; depth+= 1; } return depth; }
    inline ImGuiID          DockNodeGetWindowMenuButtonId(*const ImGuiDockNode node) { return ImHashStr("#COLLAPSE", 0, node.ID); }
    inline *mut ImGuiDockNode   GetWindowDockNode()                                      { let g = GImGui; // ImGuiContext& g = *GImGui; return g.Currentwindow.DockNode; }
     bool          GetWindowAlwaysWantOwnTabBar(*mut ImGuiWindow window);
     c_void          BeginDocked(*mut ImGuiWindow window, *mut bool p_open);
     c_void          BeginDockableDragDropSource(*mut ImGuiWindow window);
     c_void          BeginDockableDragDropTarget(*mut ImGuiWindow window);
     c_void          SetWindowDock(*mut ImGuiWindow window, ImGuiID dock_id, cond: ImGuiCond);

    // Docking - Builder function needs to be generally called before the node is used/submitted.
    // - The DockBuilderXXX functions are designed to _eventually_ become a public API, but it is too early to expose it and guarantee stability.
    // - Do not hold on ImGuiDockNode* pointers! They may be invalidated by any split/merge/remove operation and every frame.
    // - To create a DockSpace() node, make sure to set the ImGuiDockNodeFlags_DockSpace flag when calling DockBuilderAddNode().
    //   You can create dockspace nodes (attached to a window) _or_ floating nodes (carry its own window) with this API.
    // - DockBuilderSplitNode() create 2 child nodes within 1 node. The initial node becomes a parent node.
    // - If you intend to split the node immediately after creation using DockBuilderSplitNode(), make sure
    //   to call DockBuilderSetNodeSize() beforehand. If you don't, the resulting split sizes may not be reliable.
    // - Call DockBuilderFinish() after you are done.
     c_void          DockBuilderDockWindow(*const char window_name, ImGuiID node_id);
     *mut ImGuiDockNodeDockBuilderGetNode(ImGuiID node_id);
    inline *mut ImGuiDockNode   DockBuilderGetCentralNode(ImGuiID node_id)              { *mut ImGuiDockNode node = DockBuilderGetNode(node_id); if (!node) return null_mut(); return DockNodeGetRootNode(node)->CentralNode; }
     ImGuiID       DockBuilderAddNode(let mut node_id: ImGuiID =  0, ImGuiDockNodeFlags flags = 0);
     c_void          DockBuilderRemoveNode(ImGuiID node_id);                 // Remove node and all its child, undock all windows
     c_void          DockBuilderRemoveNodeDockedWindows(ImGuiID node_id, let mut clear_settings_refs: bool =  true);
     c_void          DockBuilderRemoveNodeChildNodes(ImGuiID node_id);       // Remove all split/hierarchy. All remaining docked windows will be re-docked to the remaining root node (node_id).
     c_void          DockBuilderSetNodePos(ImGuiID node_id, ImVec2 pos);
     c_void          DockBuilderSetNodeSize(ImGuiID node_id, ImVec2 size);
     ImGuiID       DockBuilderSplitNode(ImGuiID node_id, ImGuiDir split_dir,size_ratio_for_node_at_dir: c_float, *mut ImGuiID out_id_at_dir, *mut ImGuiID out_id_at_opposite_dir); // Create 2 child nodes in this parent node.
     c_void          DockBuilderCopyDockSpace(ImGuiID src_dockspace_id, ImGuiID dst_dockspace_id, Vec<*const char>* in_window_remap_pairs);
     c_void          DockBuilderCopyNode(ImGuiID src_node_id, ImGuiID dst_node_id, Vec<ImGuiID>* out_node_remap_pairs);
     c_void          DockBuilderCopyWindowSettings(*const char src_name, *const char dst_name);
     c_void          DockBuilderFinish(ImGuiID node_id);

    // Drag and Drop
     bool          IsDragDropActive();
     bool          BeginDragDropTargetCustom(bb: &ImRect, id: ImGuiID);
     c_void          ClearDragDrop();
     bool          IsDragDropPayloadBeingAccepted();

    // Internal Columns API (this is not exposed because we will encourage transitioning to the Tables API)
     c_void          SetWindowClipRectBeforeSetChannel(*mut ImGuiWindow window, clip_rect: &ImRect);
     c_void          BeginColumns(*const char str_id, c_int count, ImGuiOldColumnFlags flags = 0); // setup number of columns. use an identifier to distinguish multiple column sets. close with EndColumns().
     c_void          EndColumns();                                                               // close columns
     c_void          PushColumnClipRect(c_int column_index);
     c_void          PushColumnsBackground();
     c_void          PopColumnsBackground();
     ImGuiID       GetColumnsID(*const char str_id, c_int count);
     *mut ImGuiOldColumns FindOrCreateColumns(*mut ImGuiWindow window, id: ImGuiID);GetColumnOffsetFromNorm: c_float(*const ImGuiOldColumns columns,offset_norm: c_float);GetColumnNormFromOffset: c_float(*const ImGuiOldColumns columns,offset: c_float);

    // Tables: Candidates for public API
     c_void          TableOpenContextMenu(let column_n: c_int = -1);
     c_void          TableSetColumnWidth(c_int column_n,width: c_float);
     c_void          TableSetColumnSortDirection(c_int column_n, ImGuiSortDirection sort_direction, bool append_to_sort_specs);
     c_int           TableGetHoveredColumn(); // May use (TableGetColumnFlags() & ImGuiTableColumnFlags_IsHovered) instead. Return hovered column. return -1 when table is not hovered. return columns_count if the unused space at the right of visible columns is hovered.TableGetHeaderRowHeight: c_float();
     c_void          TablePushBackgroundChannel();
     c_void          TablePopBackgroundChannel();

    // Tables: Internals
    inline    *mut ImGuiTable   GetCurrentTable() { let g = GImGui; // ImGuiContext& g = *GImGui; return g.CurrentTable; }
     *mut ImGuiTable   TableFindByID(id: ImGuiID);
     bool          BeginTableEx(*const char name, id: ImGuiID, c_int columns_count, ImGuiTableFlags flags = 0, outer_size: &ImVec2 = ImVec2(0, 0), let inner_width: c_float =  0f32);
     c_void          TableBeginInitMemory(*mut ImGuiTable table, c_int columns_count);
     c_void          TableBeginApplyRequests(*mut ImGuiTable table);
     c_void          TableSetupDrawChannels(*mut ImGuiTable table);
     c_void          TableUpdateLayout(*mut ImGuiTable table);
     c_void          TableUpdateBorders(*mut ImGuiTable table);
     c_void          TableUpdateColumnsWeightFromWidth(*mut ImGuiTable table);
     c_void          TableDrawBorders(*mut ImGuiTable table);
     c_void          TableDrawContextMenu(*mut ImGuiTable table);
     bool          TableBeginContextMenuPopup(*mut ImGuiTable table);
     c_void          TableMergeDrawChannels(*mut ImGuiTable table);

     c_void          TableSortSpecsSanitize(*mut ImGuiTable table);
     c_void          TableSortSpecsBuild(*mut ImGuiTable table);
     ImGuiSortDirection TableGetColumnNextSortDirection(*mut ImGuiTableColumn column);
     c_void          TableFixColumnSortDirection(*mut ImGuiTable table, *mut ImGuiTableColumn column);TableGetColumnWidthAuto: c_float(*mut ImGuiTable table, *mut ImGuiTableColumn column);
     c_void          TableBeginRow(*mut ImGuiTable table);
     c_void          TableEndRow(*mut ImGuiTable table);
     c_void          TableBeginCell(*mut ImGuiTable table, c_int column_n);
     c_void          TableEndCell(*mut ImGuiTable table);
     ImRect        TableGetCellBgRect(*const ImGuiTable table, c_int column_n);
     *const char   TableGetColumnName(*const ImGuiTable table, c_int column_n);
     ImGuiID       TableGetColumnResizeID(*const ImGuiTable table, c_int column_n, let instance_no: c_int = 0);TableGetMaxColumnWidth: c_float(*const ImGuiTable table, c_int column_n);
     c_void          TableSetColumnWidthAutoSingle(*mut ImGuiTable table, c_int column_n);
     c_void          TableSetColumnWidthAutoAll(*mut ImGuiTable table);
     c_void          TableRemove(*mut ImGuiTable table);
     c_void          TableGcCompactTransientBuffers(*mut ImGuiTable table);
     c_void          TableGcCompactTransientBuffers(*mut ImGuiTableTempData table);
     c_void          TableGcCompactSettings();

    // Tables: Settings
     c_void                  TableLoadSettings(*mut ImGuiTable table);
     c_void                  TableSaveSettings(*mut ImGuiTable table);
     c_void                  TableResetSettings(*mut ImGuiTable table);
     *mut ImGuiTableSettings   TableGetBoundSettings(*mut ImGuiTable table);
     c_void                  TableSettingsAddSettingsHandler();
     *mut ImGuiTableSettings   TableSettingsCreate(id: ImGuiID, c_int columns_count);
     *mut ImGuiTableSettings   TableSettingsFindByID(id: ImGuiID);

    // Tab Bars
     bool          BeginTabBarEx(*mut ImGuiTabBar tab_bar, bb: &ImRect, ImGuiTabBarFlags flags, *mut ImGuiDockNode dock_node);
     *mut ImGuiTabItem TabBarFindTabByID(*mut ImGuiTabBar tab_bar, ImGuiID tab_id);
     *mut ImGuiTabItem TabBarFindMostRecentlySelectedTabForActiveWindow(*mut ImGuiTabBar tab_bar);
     c_void          TabBarAddTab(*mut ImGuiTabBar tab_bar, ImGuiTabItemFlags tab_flags, *mut ImGuiWindow window);
     c_void          TabBarRemoveTab(*mut ImGuiTabBar tab_bar, ImGuiID tab_id);
     c_void          TabBarCloseTab(*mut ImGuiTabBar tab_bar, *mut ImGuiTabItem tab);
     c_void          TabBarQueueReorder(*mut ImGuiTabBar tab_bar, *const ImGuiTabItem tab, c_int offset);
     c_void          TabBarQueueReorderFromMousePos(*mut ImGuiTabBar tab_bar, *const ImGuiTabItem tab, ImVec2 mouse_pos);
     bool          TabBarProcessReorder(*mut ImGuiTabBar tab_bar);
     bool          TabItemEx(*mut ImGuiTabBar tab_bar, *const char label, *mut bool p_open, ImGuiTabItemFlags flags, *mut ImGuiWindow docked_window);
     ImVec2        TabItemCalcSize(*const char label, bool has_close_button);
     c_void          TabItemBackground(*mut ImDrawList draw_list, bb: &ImRect, ImGuiTabItemFlags flags, u32 col);
     c_void          TabItemLabelAndCloseButton(*mut ImDrawList draw_list, bb: &ImRect, ImGuiTabItemFlags flags, ImVec2 frame_padding, *const char label, ImGuiID tab_id, ImGuiID close_button_id, bool is_contents_visible, *mut bool out_just_closed, *mut bool out_text_clipped);

    // Render helpers
    // AVOID USING OUTSIDE OF IMGUI.CPP! NOT FOR PUBLIC CONSUMPTION. THOSE FUNCTIONS ARE A MESS. THEIR SIGNATURE AND BEHAVIOR WILL CHANGE, THEY NEED TO BE REFACTORED INTO SOMETHING DECENT.
    // NB: All position are in absolute pixels coordinates (we are never using window coordinates internally)
     c_void          RenderText(ImVec2 pos, *const char text, *const char text_end = null_mut(), let mut hide_text_after_hash: bool =  true);
     c_void          RenderTextWrapped(ImVec2 pos, *const char text, *const char text_end,wrap_width: c_float);
     c_void          RenderTextClipped(pos_min: &ImVec2, pos_max: &ImVec2, *const char text, *const char text_end, *const ImVec2 text_size_if_known, align: &ImVec2 = ImVec2(0, 0), *const let clip_rect: ImRect =  null_mut());
     c_void          RenderTextClippedEx(*mut ImDrawList draw_list, pos_min: &ImVec2, pos_max: &ImVec2, *const char text, *const char text_end, *const ImVec2 text_size_if_known, align: &ImVec2 = ImVec2(0, 0), *const let clip_rect: ImRect =  null_mut());
     c_void          RenderTextEllipsis(*mut ImDrawList draw_list, pos_min: &ImVec2, pos_max: &ImVec2,clip_max_x: c_float,ellipsis_max_x: c_float, *const char text, *const char text_end, *const ImVec2 text_size_if_known);
     c_void          RenderFrame(ImVec2 p_min, ImVec2 p_max, u32 fill_col, let mut border: bool =  true, let rounding: c_float =  0f32);
     c_void          RenderFrameBorder(ImVec2 p_min, ImVec2 p_max, let rounding: c_float =  0f32);
     c_void          RenderColorRectWithAlphaCheckerboard(*mut ImDrawList draw_list, ImVec2 p_min, ImVec2 p_max, u32 fill_col,grid_step: c_float, ImVec2 grid_off, let rounding: c_float =  0f32, ImDrawFlags flags = 0);
     c_void          RenderNavHighlight(bb: &ImRect, id: ImGuiID, ImGuiNavHighlightFlags flags = ImGuiNavHighlightFlags_TypeDefault); // Navigation highlight
     *const char   FindRenderedTextEnd(*const char text, *const char text_end = null_mut()); // Find the optional ## from which we stop displaying text.
     c_void          RenderMouseCursor(ImVec2 pos,scale: c_float, ImGuiMouseCursor mouse_cursor, u32 col_fill, u32 col_border, u32 col_shadow);

    // Render helpers (those functions don't access any ImGui state!)
     c_void          RenderArrow(*mut ImDrawList draw_list, ImVec2 pos, u32 col, ImGuiDir dir, let scale: c_float =  1f32);
     c_void          RenderBullet(*mut ImDrawList draw_list, ImVec2 pos, u32 col);
     c_void          RenderCheckMark(*mut ImDrawList draw_list, ImVec2 pos, u32 col,sz: c_float);
     c_void          RenderArrowPointingAt(*mut ImDrawList draw_list, ImVec2 pos, ImVec2 half_sz, ImGuiDir direction, u32 col);
     c_void          RenderArrowDockMenu(*mut ImDrawList draw_list, ImVec2 p_min,sz: c_float, u32 col);
     c_void          RenderRectFilledRangeH(*mut ImDrawList draw_list, rect: &ImRect, u32 col,x_start_norm: c_float,x_end_norm: c_float,rounding: c_float);
     c_void          RenderRectFilledWithHole(*mut ImDrawList draw_list, outer: &ImRect, inner: &ImRect, u32 col,rounding: c_float);
     ImDrawFlags   CalcRoundingFlagsForRectInRect(r_in: &ImRect, r_outer: &ImRect,threshold: c_float);

    // Widgets
     c_void          TextEx(*const char text, *const char text_end = null_mut(), ImGuiTextFlags flags = 0);
     bool          ButtonEx(*const char label, size_arg: &ImVec2 = ImVec2(0, 0), ImGuiButtonFlags flags = 0);
     bool          CloseButton(id: ImGuiID, pos: &ImVec2);
     bool          CollapseButton(id: ImGuiID, pos: &ImVec2, *mut ImGuiDockNode dock_node);
     bool          ArrowButtonEx(*const char str_id, ImGuiDir dir, ImVec2 size_arg, ImGuiButtonFlags flags = 0);
     c_void          Scrollbar(ImGuiAxis axis);
     bool          ScrollbarEx(bb: &ImRect, id: ImGuiID, ImGuiAxis axis, *mut ImS64 p_scroll_v, ImS64 avail_v, ImS64 contents_v, ImDrawFlags flags);
     bool          ImageButtonEx(id: ImGuiID, ImTextureID texture_id, size: &ImVec2, uv0: &ImVec2, uv1: &ImVec2, const ImVec4& bg_col, const ImVec4& tint_col);
     ImRect        GetWindowScrollbarRect(*mut ImGuiWindow window, ImGuiAxis axis);
     ImGuiID       GetWindowScrollbarID(*mut ImGuiWindow window, ImGuiAxis axis);
     ImGuiID       GetWindowResizeCornerID(*mut ImGuiWindow window, c_int n); // 0..3: corners
     ImGuiID       GetWindowResizeBorderID(*mut ImGuiWindow window, ImGuiDir dir);
     c_void          SeparatorEx(ImGuiSeparatorFlags flags);
     bool          CheckboxFlags(*const char label, *mut ImS64 flags, ImS64 flags_value);
     bool          CheckboxFlags(*const char label, *mut u64 flags, u64 flags_value);

    // Widgets low-level behaviors
     bool          ButtonBehavior(bb: &ImRect, id: ImGuiID, *mut bool out_hovered, *mut bool out_held, ImGuiButtonFlags flags = 0);
     bool          DragBehavior(id: ImGuiID, ImGuiDataType data_type, *mut c_void p_v,v_speed: c_float, *const c_void p_min, *const c_void p_max, *const char format, ImGuiSliderFlags flags);
     bool          SliderBehavior(bb: &ImRect, id: ImGuiID, ImGuiDataType data_type, *mut c_void p_v, *const c_void p_min, *const c_void p_max, *const char format, ImGuiSliderFlags flags, *mut ImRect out_grab_bb);
     bool          SplitterBehavior(bb: &ImRect, id: ImGuiID, ImGuiAxis axis, *mutsize1: c_float, *mutsize2: c_float,min_size1: c_float,min_size2: c_float, let hover_extend: c_float =  0f32, let hover_visibility_delay: c_float =  0f32, u32 bg_col = 0);
     bool          TreeNodeBehavior(id: ImGuiID, ImGuiTreeNodeFlags flags, *const char label, *const char label_end = null_mut());
     c_void          TreePushOverrideID(id: ImGuiID);
     c_void          TreeNodeSetOpen(id: ImGuiID, bool open);
     bool          TreeNodeUpdateNextOpen(id: ImGuiID, ImGuiTreeNodeFlags flags);   // Return open state. Consume previous SetNextItemOpen() data, if any. May return true when logging.

    // Template functions are instantiated in imgui_widgets.cpp for a finite number of types.
    // To use them externally (for custom widget) you may need an "extern template" statement in your code in order to link to existing instances and silence Clang warnings (see #2036).
    // e.g. " extern template IMGUI_API float RoundScalarWithFormatT<float, float>(const char* format, ImGuiDataType data_type, float v); "
    template<typename T, typename SIGNED_T, typename FLOAT_T>ScaleRatioFromValueT: c_float(ImGuiDataType data_type, T v, T v_min, T v_max, bool is_logarithmic,logarithmic_zero_epsilon: c_float,zero_deadzone_size: c_float);
    template<typename T, typename SIGNED_T, typename FLOAT_T>    T     ScaleValueFromRatioT(ImGuiDataType data_type,t: c_float, T v_min, T v_max, bool is_logarithmic,logarithmic_zero_epsilon: c_float,zero_deadzone_size: c_float);
    template<typename T, typename SIGNED_T, typename FLOAT_T>    bool  DragBehaviorT(ImGuiDataType data_type, *mut T v,v_speed: c_float, T v_min, T v_max, *const char format, ImGuiSliderFlags flags);
    template<typename T, typename SIGNED_T, typename FLOAT_T>    bool  SliderBehaviorT(bb: &ImRect, id: ImGuiID, ImGuiDataType data_type, *mut T v, T v_min, T v_max, *const char format, ImGuiSliderFlags flags, *mut ImRect out_grab_bb);
    template<typename T>                                         T     RoundScalarWithFormatT(*const char format, ImGuiDataType data_type, T v);
    template<typename T>                                         bool  CheckboxFlagsT(*const char label, *mut T flags, T flags_value);

    // Data type helpers
     *const ImGuiDataTypeInfo  DataTypeGetInfo(ImGuiDataType data_type);
     c_int           DataTypeFormatString(*mut char buf, c_int buf_size, ImGuiDataType data_type, *const c_void p_data, *const char format);
     c_void          DataTypeApplyOp(ImGuiDataType data_type, c_int op, *mut c_void output, *const c_void arg_1, *const c_void arg_2);
     bool          DataTypeApplyFromText(*const char buf, ImGuiDataType data_type, *mut c_void p_data, *const char format);
     c_int           DataTypeCompare(ImGuiDataType data_type, *const c_void arg_1, *const c_void arg_2);
     bool          DataTypeClamp(ImGuiDataType data_type, *mut c_void p_data, *const c_void p_min, *const c_void p_max);

    // InputText
     bool          InputTextEx(*const char label, *const char hint, *mut char buf, c_int buf_size, size_arg: &ImVec2, ImGuiInputTextFlags flags, ImGuiInputTextCallback callback = null_mut(), *mut c_void user_data = null_mut());
     bool          TempInputText(bb: &ImRect, id: ImGuiID, *const char label, *mut char buf, c_int buf_size, ImGuiInputTextFlags flags);
     bool          TempInputScalar(bb: &ImRect, id: ImGuiID, *const char label, ImGuiDataType data_type, *mut c_void p_data, *const char format, *const c_void p_clamp_min = null_mut(), *const c_void p_clamp_max = null_mut());
    inline bool             TempInputIsActive(id: ImGuiID)       { let g = GImGui; // ImGuiContext& g = *GImGui; return (g.ActiveId == id && g.TempInputId == id); }
    inline *mut ImGuiInputTextState GetInputTextState(id: ImGuiID)   { let g = GImGui; // ImGuiContext& g = *GImGui; return (id != 0 && g.InputTextState.ID == id) ? &g.InputTextState : NULL; } // Get input text state if active

    // Color
     c_void          ColorTooltip(*const char text, *col: c_float, ImGuiColorEditFlags flags);
     c_void          ColorEditOptionsPopup(*col: c_float, ImGuiColorEditFlags flags);
     c_void          ColorPickerOptionsPopup(*ref_col: c_float, ImGuiColorEditFlags flags);

    // Plot
     c_int           PlotEx(ImGuiPlotType plot_type, *const char label, c_float (*values_getter)(*mut c_void data, c_int idx), *mut c_void data, c_int values_count, c_int values_offset, *const char overlay_text,scale_min: c_float,scale_max: c_float, ImVec2 frame_size);

    // Shade functions (write over already created vertices)
     c_void          ShadeVertsLinearColorGradientKeepAlpha(*mut ImDrawList draw_list, c_int vert_start_idx, c_int vert_end_idx, ImVec2 gradient_p0, ImVec2 gradient_p1, u32 col0, u32 col1);
     c_void          ShadeVertsLinearUV(*mut ImDrawList draw_list, c_int vert_start_idx, c_int vert_end_idx, a: &ImVec2, b: &ImVec2, uv_a: &ImVec2, uv_b: &ImVec2, bool clamp);

    // Garbage collection
     c_void          GcCompactTransientMiscBuffers();
     c_void          GcCompactTransientWindowBuffers(*mut ImGuiWindow window);
     c_void          GcAwakeTransientWindowBuffers(*mut ImGuiWindow window);

    // Debug Log
     c_void          DebugLog(*const char fmt, ...) IM_FMTARGS(1);
     c_void          DebugLogV(*const char fmt, va_list args) IM_FMTLIST(1);

    // Debug Tools
     c_void          ErrorCheckEndFrameRecover(ImGuiErrorLogCallback log_callback, *mut c_void user_data = null_mut());
     c_void          ErrorCheckEndWindowRecover(ImGuiErrorLogCallback log_callback, *mut c_void user_data = null_mut());
     c_void          ErrorCheckUsingSetCursorPosToExtendParentBoundaries();
    inline c_void             DebugDrawItemRect(u32 col = IM_COL32(255,0,0,255))    { let g = GImGui; // ImGuiContext& g = *GImGui; ImGuiWindow* window = g.CurrentWindow; GetForegroundDrawList(window)->AddRect(g.LastItemData.Rect.Min, g.LastItemData.Rect.Max, col); }
    inline c_void             DebugStartItemPicker()                                  { let g = GImGui; // ImGuiContext& g = *GImGui; g.DebugItemPickerActive = true; }
     c_void          ShowFontAtlas(*mut ImFontAtlas atlas);
     c_void          DebugHookIdInfo(id: ImGuiID, ImGuiDataType data_type, *const c_void data_id, *const c_void data_id_end);
     c_void          DebugNodeColumns(*mut ImGuiOldColumns columns);
     c_void          DebugNodeDockNode(*mut ImGuiDockNode node, *const char label);
     c_void          DebugNodeDrawList(*mut ImGuiWindow window, *mut ImGuiViewportP viewport, *const ImDrawList draw_list, *const char label);
     c_void          DebugNodeDrawCmdShowMeshAndBoundingBox(*mut ImDrawList out_draw_list, *const ImDrawList draw_list, *const ImDrawCmd draw_cmd, bool show_mesh, bool show_aabb);
     c_void          DebugNodeFont(*mut ImFont font);
     c_void          DebugNodeFontGlyph(*mut ImFont font, *const ImFontGlyph glyph);
     c_void          DebugNodeStorage(*mut ImGuiStorage storage, *const char label);
     c_void          DebugNodeTabBar(*mut ImGuiTabBar tab_bar, *const char label);
     c_void          DebugNodeTable(*mut ImGuiTable table);
     c_void          DebugNodeTableSettings(*mut ImGuiTableSettings settings);
     c_void          DebugNodeInputTextState(*mut ImGuiInputTextState state);
     c_void          DebugNodeWindow(*mut ImGuiWindow window, *const char label);
     c_void          DebugNodeWindowSettings(*mut ImGuiWindowSettings settings);
     c_void          DebugNodeWindowsList(Vec<*mut ImGuiWindow>* windows, *const char label);
     c_void          DebugNodeWindowsListByBeginStackParent(*mut ImGuiWindow* windows, c_int windows_size, *mut ImGuiWindow parent_in_begin_stack);
     c_void          DebugNodeViewport(*mut ImGuiViewportP viewport);
     c_void          DebugRenderViewportThumbnail(*mut ImDrawList draw_list, *mut ImGuiViewportP viewport, bb: &ImRect);

    // Obsolete functions
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    inline bool     TreeNodeBehaviorIsOpen(id: ImGuiID, ImGuiTreeNodeFlags flags = 0)    { return TreeNodeUpdateNextOpen(id, flags); }   // Renamed in 1.89

    // Refactored focus/nav/tabbing system in 1.82 and 1.84. If you have old/custom copy-and-pasted widgets that used FocusableItemRegister():
    //  (Old) IMGUI_VERSION_NUM  < 18209: using 'ItemAdd(....)'                              and 'bool tab_focused = FocusableItemRegister(...)'
    //  (Old) IMGUI_VERSION_NUM >= 18209: using 'ItemAdd(..., ImGuiItemAddFlags_Focusable)'  and 'bool tab_focused = (GetItemStatusFlags() & ImGuiItemStatusFlags_Focused) != 0'
    //  (New) IMGUI_VERSION_NUM >= 18413: using 'ItemAdd(..., ImGuiItemFlags_Inputable)'     and 'bool tab_focused = (GetItemStatusFlags() & ImGuiItemStatusFlags_FocusedTabbing) != 0 || g.NavActivateInputId == id' (WIP)
    // Widget code are simplified as there's no need to call FocusableItemUnregister() while managing the transition from regular widget to TempInputText()
    inline bool     FocusableItemRegister(*mut ImGuiWindow window, id: ImGuiID)              { IM_ASSERT(0); IM_UNUSED(window); IM_UNUSED(id); return false; } // -> pass ImGuiItemAddFlags_Inputable flag to ItemAdd()
    inline c_void     FocusableItemUnregister(*mut ImGuiWindow window)                        { IM_ASSERT(0); IM_UNUSED(window); }                              // -> unnecessary: TempInputText() uses ImGuiInputTextFlags_MergedItem
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
 c_void      ImFontAtlasBuildInit(*mut ImFontAtlas atlas);
 c_void      ImFontAtlasBuildSetupFont(*mut ImFontAtlas atlas, *mut ImFont font, *mut ImFontConfig font_config,ascent: c_float,descent: c_float);
 c_void      ImFontAtlasBuildPackCustomRects(*mut ImFontAtlas atlas, *mut c_void stbrp_context_opaque);
 c_void      ImFontAtlasBuildFinish(*mut ImFontAtlas atlas);
 c_void      ImFontAtlasBuildRender8bppRectFromString(*mut ImFontAtlas atlas, c_int x, c_int y, c_int w, c_int h, *const char in_str, char in_marker_char, c_uchar in_marker_pixel_value);
 c_void      ImFontAtlasBuildRender32bppRectFromString(*mut ImFontAtlas atlas, c_int x, c_int y, c_int w, c_int h, *const char in_str, char in_marker_char, c_uint in_marker_pixel_value);
 c_void      ImFontAtlasBuildMultiplyCalcLookupTable(unsigned out_table: [c_char;256],in_multiply_factor: c_float);
 c_void      ImFontAtlasBuildMultiplyRectAlpha8(const unsigned table: [c_char;256], unsigned *mut char pixels, c_int x, c_int y, c_int w, c_int h, c_int stride);

//-----------------------------------------------------------------------------
// [SECTION] Test Engine specific hooks (imgui_test_engine)
//-----------------------------------------------------------------------------

// #ifdef IMGUI_ENABLE_TEST_ENGINE
extern c_void         ImGuiTestEngineHook_ItemAdd(*mut ImGuiContext ctx, bb: &ImRect, id: ImGuiID);
extern c_void         ImGuiTestEngineHook_ItemInfo(*mut ImGuiContext ctx, id: ImGuiID, *const char label, ImGuiItemStatusFlags flags);
extern c_void         ImGuiTestEngineHook_Log(*mut ImGuiContext ctx, *const char fmt, ...);
extern *const char  ImGuiTestEngine_FindItemDebugLabel(*mut ImGuiContext ctx, id: ImGuiID);

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
