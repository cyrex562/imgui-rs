use libc::{c_char, c_int, c_void, size_t};

// typedef int ImGuiDir;               // -> enum ImGuiDir_             // Enum: A cardinal direction
// pub type ImGuiDir = c_int;

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

//  functions

// typedef void (*ImGuiErrorLogCallback)(void* user_data, const char* fmt, ...);
pub type ImGuiErrorLogCallback = fn(user_data: *mut c_void, fmt: *const c_char);


// typedef ImBitArray<ImGuiKey_NamedKey_COUNT, -ImGuiKey_NamedKey_BEGIN>    ImBitArrayForNamedKeys;
pub type ImBitArrayForNamedKeys = ImBitArray;


// typedef c_int ImPoolIdx;
pub type ImPoolIdx = c_int;


// // Our current column maximum is 64 but we may raise that in the future.
// typedef i8 ImGuiTableColumnIdx;
pub type ImGuiTableColumnIdx = i8;
// typedef ImU8 ImGuiTableDrawChannelIdx;
pub type ImGuiTableDrawChannelIdx = u8;


// typedef *mut FILE ImFileHandle;
pub type ImFileHandle = *mut libc::FILE;
