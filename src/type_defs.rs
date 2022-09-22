use libc::c_int;

//   With Visual Assist installed: ALT+G ("VAssistX.GoToImplementation") can also follow symbols in comments.
// typedef int ImGuiCol;               // -> enum ImGuiCol_             // Enum: A color identifier for styling
type ImGuiCol = c_int;

// typedef int ImGuiCond;              // -> enum ImGuiCond_            // Enum: A condition for many Set*() functions
type ImGuiCond = c_int;

// typedef int ImGuiDataType;          // -> enum ImGuiDataType_        // Enum: A primary data type
type ImGuiDataType = c_int;

// typedef int ImGuiDir;               // -> enum ImGuiDir_             // Enum: A cardinal direction
type ImGuiDir = c_int;

// typedef int ImGuiKey;               // -> enum ImGuiKey_             // Enum: A key identifier
type ImGuiKey = c_int;

// typedef int ImGuiMouseButton;       // -> enum ImGuiMouseButton_     // Enum: A mouse button identifier (0=left, 1=right, 2=middle)
type ImGuiMouseButton = c_int;

// typedef int ImGuiMouseCursor;       // -> enum ImGuiMouseCursor_     // Enum: A mouse cursor identifier
type ImGuiMouseCursor = c_int;

// typedef int ImGuiSortDirection;     // -> enum ImGuiSortDirection_   // Enum: A sorting direction (ascending or descending)
type ImGuiSortDirection = c_int;

// typedef int ImGuiStyleVar;          // -> enum ImGuiStyleVar_        // Enum: A variable identifier for styling
type ImGuiStyleVar = c_int;

// typedef int ImGuiTableBgTarget;     // -> enum ImGuiTableBgTarget_   // Enum: A color target for TableSetBgColor()
type ImGuiTableBgTarget = c_int;

// typedef int ImDrawFlags;            // -> enum ImDrawFlags_          // Flags: for ImDrawList functions
type ImDrawFlags = c_int;

// typedef int ImDrawListFlags;        // -> enum ImDrawListFlags_      // Flags: for ImDrawList instance
type ImDrawListFlags = c_int;

// typedef int ImFontAtlasFlags;       // -> enum ImFontAtlasFlags_     // Flags: for ImFontAtlas build
type ImFontAtlasFlags = c_int;

// typedef int ImGuiBackendFlags;      // -> enum ImGuiBackendFlags_    // Flags: for io.BackendFlags
type ImGuiBackendFlags = c_int;

// typedef int ImGuiButtonFlags;       // -> enum ImGuiButtonFlags_     // Flags: for InvisibleButton()
type ImGuiButtonFlags = c_int;

// typedef int ImGuiColorEditFlags;    // -> enum ImGuiColorEditFlags_  // Flags: for ColorEdit4(), ColorPicker4() etc.
type ImGuiColorEditFlags = c_int;

// typedef int ImGuiConfigFlags;       // -> enum ImGuiConfigFlags_     // Flags: for io.ConfigFlags
type ImGuiConfigFlags = c_int;

// typedef int ImGuiComboFlags;        // -> enum ImGuiComboFlags_      // Flags: for BeginCombo()
type ImGuiComboFlags = c_int;

// typedef int ImGuiDockNodeFlags;     // -> enum ImGuiDockNodeFlags_   // Flags: for DockSpace()
type ImGuiDockNodeFlags = c_int;

// typedef int ImGuiDragDropFlags;     // -> enum ImGuiDragDropFlags_   // Flags: for BeginDragDropSource(), AcceptDragDropPayload()
type ImGuiDragDropFlags = c_int;

// typedef int ImGuiFocusedFlags;      // -> enum ImGuiFocusedFlags_    // Flags: for IsWindowFocused()
type ImGuiFocusedFlags = c_int;


// typedef int ImGuiHoveredFlags;      // -> enum ImGuiHoveredFlags_    // Flags: for IsItemHovered(), IsWindowHovered() etc.
type ImGuiHoveredFlags = c_int;

// typedef int ImGuiInputTextFlags;    // -> enum ImGuiInputTextFlags_  // Flags: for InputText(), InputTextMultiline()
type ImGuiInputTextFlags = c_int;

// typedef int ImGuiModFlags;          // -> enum ImGuiModFlags_        // Flags: for io.KeyMods (Ctrl/Shift/Alt/Super)
type ImGuiModFlags = c_int;

// typedef int ImGuiPopupFlags;        // -> enum ImGuiPopupFlags_      // Flags: for OpenPopup*(), BeginPopupContext*(), IsPopupOpen()
type ImGuiPopupFlags = c_int;

typedef int ImGuiSelectableFlags;   // -> enum ImGuiSelectableFlags_ // Flags: for Selectable()
typedef int ImGuiSliderFlags;       // -> enum ImGuiSliderFlags_     // Flags: for DragFloat(), DragInt(), SliderFloat(), SliderInt() etc.
typedef int ImGuiTabBarFlags;       // -> enum ImGuiTabBarFlags_     // Flags: for BeginTabBar()
typedef int ImGuiTabItemFlags;      // -> enum ImGuiTabItemFlags_    // Flags: for BeginTabItem()
typedef int ImGuiTableFlags;        // -> enum ImGuiTableFlags_      // Flags: For BeginTable()
typedef int ImGuiTableColumnFlags;  // -> enum ImGuiTableColumnFlags_// Flags: For TableSetupColumn()
typedef int ImGuiTableRowFlags;     // -> enum ImGuiTableRowFlags_   // Flags: For TableNextRow()
typedef int ImGuiTreeNodeFlags;     // -> enum ImGuiTreeNodeFlags_   // Flags: for TreeNode(), TreeNodeEx(), CollapsingHeader()
typedef int ImGuiViewportFlags;     // -> enum ImGuiViewportFlags_   // Flags: for ImGuiViewport
typedef int ImGuiWindowFlags;       // -> enum ImGuiWindowFlags_     // Flags: for Begin(), BeginChild()

// ImTexture: user data for renderer backend to identify a texture [Compile-time configurable type]
// - To use something else than an opaque void* pointer: override with e.g. '#define ImTextureID MyTextureType*' in your imconfig.h file.
// - This can be whatever to you want it to be! read the FAQ about ImTextureID for details.
// #ifndef ImTextureID
typedef void* ImTextureID;          // Default: store a pointer or an integer fitting in a pointer (most renderer backends are ok with that)
// #endif

// ImDrawIdx: vertex index. [Compile-time configurable type]
// - To use 16-bit indices + allow large meshes: backend need to set 'io.BackendFlags |= ImGuiBackendFlags_RendererHasVtxOffset' and handle ImDrawCmd::VtxOffset (recommended).
// - To use 32-bit indices: override with '#define ImDrawIdx unsigned int' in your imconfig.h file.
// #ifndef ImDrawIdx
typedef unsigned short ImDrawIdx;   // Default: 16-bit (for maximum compatibility with renderer backends)
// #endif

// Scalar data types
typedef unsigned int        ImGuiID;// A unique ID used by widgets (typically the result of hashing a stack of string)
typedef signed char         ImS8;   // 8-bit signed integer
typedef unsigned char       ImU8;   // 8-bit unsigned integer
typedef signed short        ImS16;  // 16-bit signed integer
typedef unsigned short      ImU16;  // 16-bit unsigned integer
typedef signed int          ImS32;  // 32-bit signed integer == int
typedef unsigned int        u32;  // 32-bit unsigned integer (often used to store packed colors)
typedef signed   long long  ImS64;  // 64-bit signed integer
typedef unsigned long long  ImU64;  // 64-bit unsigned integer

// Character types
// (we generally use UTF-8 encoded string in the API. This is storage specifically for a decoded character used for keyboard input and display)
typedef unsigned short ImWchar16;   // A single decoded U16 character/code point. We encode them as multi bytes UTF-8 when used in strings.
typedef unsigned int ImWchar32;     // A single decoded U32 character/code point. We encode them as multi bytes UTF-8 when used in strings.
// #ifdef IMGUI_USE_WCHAR32            // ImWchar [configurable type: override in imconfig.h with '#define IMGUI_USE_WCHAR32' to support Unicode planes 1-16]
typedef ImWchar32 ImWchar;
// #else
typedef ImWchar16 ImWchar;
// #endif

// Callback and functions types
typedef int     (*ImGuiInputTextCallback)(ImGuiInputTextCallbackData* data);    // Callback function for ImGui::InputText()
typedef void    (*ImGuiSizeCallback)(ImGuiSizeCallbackData* data);              // Callback function for ImGui::SetNextWindowSizeConstraints()
typedef void*   (*ImGuiMemAllocFunc)(size_t sz, void* user_data);               // Function signature for ImGui::SetAllocatorFunctions()
typedef void    (*ImGuiMemFreeFunc)(void* ptr, void* user_data);                // Function signature for ImGui::SetAllocatorFunctions()
