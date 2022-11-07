use crate::bit_array::ImBitArray;
use libc::{c_char, c_int, c_void, size_t};

// typedef int ImGuiDir;               // -> enum ImGuiDir_             // Enum: A cardinal direction
// pub type ImGuiDir = c_int;

// ImTexture: user data for renderer backend to identify a texture [Compile-time configurable type]
// - To use something else than an opaque void* pointer: override with e.g. '#define MyTextureType: ImTextureID*' in your imconfig.h file.
// - This can be whatever to you want it to be! read the FAQ about for: ImTextureID details.
// #ifndef ImTextureID

// typedef void* ImTextureID;          // Default: store a pointer or an integer fitting in a pointer (most renderer backends are ok with that)
pub type ImTextureID = *mut c_void;
// #endif

// ImDrawIdx: vertex index. [Compile-time configurable type]
// - To use 16-bit indices + allow large meshes: backend need to set 'io.BackendFlags |= IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VTX_OFFSET' and handle ImDrawCmd::VtxOffset (recommended).
// - To use 32-bit indices: override with '#define ImDrawIdx unsigned int' in your imconfig.h file.
// #ifndef ImDrawIdx
// typedef unsigned short ImDrawIdx;   // Default: 16-bit (for maximum compatibility with renderer backends)
// #endif
pub type ImDrawIdx = size_t;

// Scalar data types
// typedef unsigned int        ImguiHandle;// A unique ID used by widgets (typically the result of hashing a stack of string)
pub type ImguiHandle = u64;
pub const INVALID_IMGUI_HANDLE: u64 = u64::MAX;

// Callback and functions types
// typedef int     (*ImGuiInputTextCallback)(ImGuiInputTextCallbackData* data);    // Callback function for InputText()
pub type ImGuiInputTextCallback = fn(data: *mut ImGuiInpuTextCallbackData) -> c_int;

// typedef void    (*ImGuiSizeCallback)(ImGuiSizeCallbackData* data);              // Callback function for SetNextWindowSizeConstraints()
pub type ImGuisizeCallback = fn(data: *mut ImGuisizeCallbackData);

// typedef void*   (*ImGuiMemAllocFunc)(sz: size_t, void* user_data);               // Function signature for SetAllocatorFunctions()
pub type ImGuiMemAllocFunc = fn(sz: size_t, user_data: *mut c_void);

// typedef void    (*ImGuiMemFreeFunc)(void* ptr, void* user_data);                // Function signature for SetAllocatorFunctions()
pub type ImGuiMemFreeFunc = fn(ptr: *mut c_void, user_data: *mut c_void);

//  functions

// typedef void (*ImGuiErrorLogCallback)(void* user_data, const char* fmt, ...);
pub type ImGuiErrorLogCallback = fn(user_data: *mut c_void, fmt: *const c_char);

// typedef ImBitArray<ImGuiKey_NamedKey_COUNT, -ImGuiKey_NamedKey_BEGIN>    ImBitArrayForNamedKeys;
pub type ImBitArrayForNamedKeys = ImBitArray;

// typedef let mut ImPoolIdx: c_int = 0;
pub type ImPoolIdx = c_int;

// // Our current column maximum is 64 but we may raise that in the future.
// typedef i8 ImGuiTableColumnIdx;
pub type ImGuiTableColumnIdx = i8;
// typedef ImU8 ImGuiTableDrawChannelIdx;
pub type ImGuiTableDrawChannelIdx = u8;

// typedef *mut FILE ImFileHandle;
pub type ImFileHandle = *mut libc::FILE;
