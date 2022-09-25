use libc::{c_char, c_int, c_void, size_t};

//   With Visual Assist installed: ALT+G ("VAssistX.GoToImplementation") can also follow symbols in comments.
// typedef int ImGuiCol;               // -> enum ImGuiCol_             // Enum: A color identifier for styling
pub type ImGuiCol = c_int;

// typedef int ImGuiCond;              // -> enum ImGuiCond_            // Enum: A condition for many Set*() functions
pub type ImGuiCond = c_int;

// typedef int ImGuiDataType;          // -> enum ImGuiDataType_        // Enum: A primary data type
pub type ImGuiDataType = c_int;

// typedef int ImGuiDir;               // -> enum ImGuiDir_             // Enum: A cardinal direction
pub type ImGuiDir = c_int;

// typedef int ImGuiKey;               // -> enum ImGuiKey_             // Enum: A key identifier
pub type ImGuiKey = c_int;

// typedef int ImGuiMouseButton;       // -> enum ImGuiMouseButton_     // Enum: A mouse button identifier (0=left, 1=right, 2=middle)
pub type ImGuiMouseButton = c_int;

// typedef int ImGuiMouseCursor;       // -> enum ImGuiMouseCursor_     // Enum: A mouse cursor identifier
pub type ImGuiMouseCursor = c_int;

// typedef int ImGuiSortDirection;     // -> enum ImGuiSortDirection_   // Enum: A sorting direction (ascending or descending)
pub type ImGuiSortDirection = c_int;

// typedef int ImGuiStyleVar;          // -> enum ImGuiStyleVar_        // Enum: A variable identifier for styling
pub type ImGuiStyleVar = c_int;

// typedef int ImGuiTableBgTarget;     // -> enum ImGuiTableBgTarget_   // Enum: A color target for TableSetBgColor()
pub type ImGuiTableBgTarget = c_int;

// typedef int ImDrawFlags;            // -> enum ImDrawFlags_          // Flags: for ImDrawList functions
pub type ImDrawFlags = c_int;

// typedef int ImDrawListFlags;        // -> enum ImDrawListFlags_      // Flags: for ImDrawList instance
pub type ImDrawListFlags = c_int;

// typedef int ImFontAtlasFlags;       // -> enum ImFontAtlasFlags_     // Flags: for ImFontAtlas build
pub type ImFontAtlasFlags = c_int;

// typedef int ImGuiBackendFlags;      // -> enum ImGuiBackendFlags_    // Flags: for io.BackendFlags
pub type ImGuiBackendFlags = c_int;

// typedef int ImGuiButtonFlags;       // -> enum ImGuiButtonFlags_     // Flags: for InvisibleButton()
pub type ImGuiButtonFlags = c_int;

// typedef int ImGuiColorEditFlags;    // -> enum ImGuiColorEditFlags_  // Flags: for ColorEdit4(), ColorPicker4() etc.
pub type ImGuiColorEditFlags = c_int;

// typedef int ImGuiConfigFlags;       // -> enum ImGuiConfigFlags_     // Flags: for io.ConfigFlags
pub type ImGuiConfigFlags = c_int;

// typedef int ImGuiComboFlags;        // -> enum ImGuiComboFlags_      // Flags: for BeginCombo()
pub type ImGuiComboFlags = c_int;

// typedef int ImGuiDockNodeFlags;     // -> enum ImGuiDockNodeFlags_   // Flags: for DockSpace()
pub type ImGuiDockNodeFlags = c_int;

// typedef int ImGuiDragDropFlags;     // -> enum ImGuiDragDropFlags_   // Flags: for BeginDragDropSource(), AcceptDragDropPayload()
pub type ImGuiDragDropFlags = c_int;

// typedef int ImGuiFocusedFlags;      // -> enum ImGuiFocusedFlags_    // Flags: for IsWindowFocused()
pub type ImGuiFocusedFlags = c_int;


// typedef int ImGuiHoveredFlags;      // -> enum ImGuiHoveredFlags_    // Flags: for IsItemHovered(), IsWindowHovered() etc.
pub type ImGuiHoveredFlags = c_int;

// typedef int ImGuiInputTextFlags;    // -> enum ImGuiInputTextFlags_  // Flags: for InputText(), InputTextMultiline()
pub type ImGuiInputTextFlags = c_int;

// typedef int ImGuiModFlags;          // -> enum ImGuiModFlags_        // Flags: for io.KeyMods (Ctrl/Shift/Alt/Super)
pub type ImGuiModFlags = c_int;

// typedef int ImGuiPopupFlags;        // -> enum ImGuiPopupFlags_      // Flags: for OpenPopup*(), BeginPopupContext*(), IsPopupOpen()
pub type ImGuiPopupFlags = c_int;

// typedef int ImGuiSelectableFlags;   // -> enum ImGuiSelectableFlags_ // Flags: for Selectable()
pub type ImGuiSelectableFlags = c_int;

// typedef int ImGuiSliderFlags;       // -> enum ImGuiSliderFlags_     // Flags: for DragFloat(), DragInt(), SliderFloat(), SliderInt() etc.
pub type ImGuiSliderFlags = c_int;

// typedef int ImGuiTabBarFlags;       // -> enum ImGuiTabBarFlags_     // Flags: for BeginTabBar()
pub type ImGuiTabBatFlags = c_int;

// typedef int ImGuiTabItemFlags;      // -> enum ImGuiTabItemFlags_    // Flags: for BeginTabItem()
pub type ImGuiTabItemFlags = c_int;

// typedef int ImGuiTableFlags;        // -> enum ImGuiTableFlags_      // Flags: For BeginTable()
pub type ImGUiTableFlags = c_int;

// typedef int ImGuiTableColumnFlags;  // -> enum ImGuiTableColumnFlags_// Flags: For TableSetupColumn()
pub type ImGuiTableColumnFlags = c_int;

// typedef int ImGuiTableRowFlags;     // -> enum ImGuiTableRowFlags_   // Flags: For TableNextRow()
pub type ImGuiTableRowFlags = c_int;

// typedef int ImGuiTreeNodeFlags;     // -> enum ImGuiTreeNodeFlags_   // Flags: for TreeNode(), TreeNodeEx(), CollapsingHeader()
pub type ImGuiTreeNodeFlags = c_int;

// typedef int ImGuiViewportFlags;     // -> enum ImGuiViewportFlags_   // Flags: for ImGuiViewport
pub type ImGuiViewportFlags = c_int;

// typedef int ImGuiWindowFlags;       // -> enum ImGuiWindowFlags_     // Flags: for Begin(), BeginChild()
pub type ImGuiWindowFlags = c_int;

// ImTexture: user data for renderer backend to identify a texture [Compile-time configurable type]
// - To use something else than an opaque void* pointer: override with e.g. '#define ImTextureID MyTextureType*' in your imconfig.h file.
// - This can be whatever to you want it to be! read the FAQ about ImTextureID for details.
// #ifndef ImTextureID

// typedef void* ImTextureID;          // Default: store a pointer or an integer fitting in a pointer (most renderer backends are ok with that)
pub type ImTextureID = *mut c_void;
// #endif

// ImDrawIdx: vertex index. [Compile-time configurable type]
// - To use 16-bit indices + allow large meshes: backend need to set 'io.BackendFlags |= ImGuiBackendFlags_RendererHasVtxOffset' and handle ImDrawCmd::VtxOffset (recommended).
// - To use 32-bit indices: override with '#define ImDrawIdx unsigned int' in your imconfig.h file.
// #ifndef ImDrawIdx
// typedef unsigned short ImDrawIdx;   // Default: 16-bit (for maximum compatibility with renderer backends)
// #endif
pub type ImDrawIdx = u16;

// Scalar data types
// typedef unsigned int        ImGuiID;// A unique ID used by widgets (typically the result of hashing a stack of string)
pub type ImGuiID = c_int;

// typedef signed char         ImS8;   // 8-bit signed integer
// typedef unsigned char       ImU8;   // 8-bit unsigned integer
// typedef signed short        ImS16;  // 16-bit signed integer
// typedef unsigned short      ImU16;  // 16-bit unsigned integer
// typedef signed int          ImS32;  // 32-bit signed integer == int
// typedef unsigned int        u32;  // 32-bit unsigned integer (often used to store packed colors)
// typedef signed   long long  ImS64;  // 64-bit signed integer
// typedef unsigned long long  ImU64;  // 64-bit unsigned integer

// Character types
// (we generally use UTF-8 encoded string in the API. This is storage specifically for a decoded character used for keyboard input and display)
// typedef unsigned short ImWchar16;   // A single decoded U16 character/code point. We encode them as multi bytes UTF-8 when used in strings.
pub type ImWchar16 = u16;

// typedef unsigned int ImWchar32;     // A single decoded U32 character/code point. We encode them as multi bytes UTF-8 when used in strings.
pub type ImWchar32 = u32;

// #ifdef IMGUI_USE_WCHAR32            // ImWchar [configurable type: override in imconfig.h with '#define IMGUI_USE_WCHAR32' to support Unicode planes 1-16]
// typedef ImWchar32 ImWchar;
pub type ImWchar = ImWchar32;

// #else
// typedef ImWchar16 ImWchar;
// #endif

// Callback and functions types
// typedef int     (*ImGuiInputTextCallback)(ImGuiInputTextCallbackData* data);    // Callback function for ImGui::InputText()
pub type ImGuiInputTextCallback = fn(data: *mut ImGuiInpuTextCallbackData) -> c_int;

// typedef void    (*ImGuiSizeCallback)(ImGuiSizeCallbackData* data);              // Callback function for ImGui::SetNextWindowSizeConstraints()
pub type ImGuisizeCallback = fn(data: *mut ImGuisizeCallbackData);

// typedef void*   (*ImGuiMemAllocFunc)(size_t sz, void* user_data);               // Function signature for ImGui::SetAllocatorFunctions()
pub type ImGuiMemAllocFunc = fn(sz: size_t, user_data: *mut c_void);

// typedef void    (*ImGuiMemFreeFunc)(void* ptr, void* user_data);                // Function signature for ImGui::SetAllocatorFunctions()
pub type ImGuiMemFreeFunc = fn(ptr: *mut c_void, user_data: *mut c_void);



// Use your programming IDE "Go to definition" facility on the names of the center columns to find the actual flags/enum lists.
// typedef int ImGuiDataAuthority;         // -> enum ImGuiDataAuthority_      // Enum: for storing the source authority (dock node vs window) of a field
pub type ImGuiDataAuthority = c_int;

// typedef int ImGuiLayoutType;            // -> enum ImGuiLayoutType_         // Enum: Horizontal or vertical
pub type ImGuiLayoutType = c_int;

// typedef int ImGuiActivateFlags;         // -> enum ImGuiActivateFlags_      // Flags: for navigation/focus function (will be for ActivateItem() later)
pub type ImGuiActivateFlags = c_int;

// typedef int ImGuiDebugLogFlags;         // -> enum ImGuiDebugLogFlags_      // Flags: for ShowDebugLogWindow(), g.DebugLogFlags
pub type ImGuiDebugLogFlags = c_int;

// typedef int ImGuiInputFlags;            // -> enum ImGuiInputFlags_         // Flags: for IsKeyPressedEx()
pub type ImGuiInputFlags = c_int;

// typedef int ImGuiItemFlags;             // -> enum ImGuiItemFlags_          // Flags: for PushItemFlag()
pub type ImGuiItemFlags = c_int;

// typedef int ImGuiItemStatusFlags;       // -> enum ImGuiItemStatusFlags_    // Flags: for DC.LastItemStatusFlags
pub type ImGuiItemStatusFlags = c_int;

// typedef int ImGuiOldColumnFlags;        // -> enum ImGuiOldColumnFlags_     // Flags: for BeginColumns()
pub type ImGuiOldColumnFlags = c_int;

// typedef int ImGuiNavHighlightFlags;     // -> enum ImGuiNavHighlightFlags_  // Flags: for RenderNavHighlight()
pub type ImGuiNavHighlightFlags = c_int;

// typedef int ImGuiNavMoveFlags;          // -> enum ImGuiNavMoveFlags_       // Flags: for navigation requests
pub type ImGuiNavMoveFlags = c_int;

// typedef int ImGuiNextItemDataFlags;     // -> enum ImGuiNextItemDataFlags_  // Flags: for SetNextItemXXX() functions
pub type ImGuiNextItemDataFlags = c_int;

// typedef int ImGuiNextWindowDataFlags;   // -> enum ImGuiNextWindowDataFlags_// Flags: for SetNextWindowXXX()
pub type ImGuiNextWindowDataFlags = c_int;

//  functions

// typedef int ImGuiScrollFlags;           // -> enum ImGuiScrollFlags_        // Flags: for ScrollToItem() and navigation requests
pub type ImGuiScrollFlags = c_int;

// typedef int ImGuiSeparatorFlags;        // -> enum ImGuiSeparatorFlags_     // Flags: for SeparatorEx()
pub type ImGuiSeparatorFlags = c_int;

// typedef int ImGuiTextFlags;             // -> enum ImGuiTextFlags_          // Flags: for TextEx()
pub type ImGuiTextFlags = c_int;

// typedef int ImGuiTooltipFlags;          // -> enum ImGuiTooltipFlags_       // Flags: for BeginTooltipEx()
pub type ImGuiTooltipFlags = c_int;

// typedef void (*ImGuiErrorLogCallback)(void* user_data, const char* fmt, ...);
pub type ImGuiErrorLogCallback = fn(user_data: *mut c_void, fmt: *const c_char);