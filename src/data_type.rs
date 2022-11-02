#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiDataType;          // -> enum ImGuiDataType_        // Enum: A primary data type
pub type ImGuiDataType = c_int;

// Standard Drag and Drop payload types. You can define you own payload types using short strings. Types starting with '_' are defined by Dear ImGui.
// #define IMGUI_PAYLOAD_TYPE_COLOR_3F     "_COL3F"    // float[3]: Standard type for colors, without alpha. User code may use this type.
pub type ImGuiPayloadType = i32;
pub const ImGuiPayloadType_Color3f: ImGuiPayloadType = 0;
pub const IMguiPayloadType_Color4f: ImGuiPayloadType = 1;
// #define IMGUI_PAYLOAD_TYPE_COLOR_4F     "_COL4F"    // float[4]: Standard type for colors. User code may use this type.

// A primary data type
// enum ImGuiDataType_
// {
pub const ImGuiDataType_S8: ImGuiDataType = 0;
// signed char / char (with sensible compilers)
pub const ImGuiDataType_U8: ImGuiDataType = 1;
// unsigned char
pub const ImGuiDataType_S16: ImGuiDataType = 2;
// short
pub const ImGuiDataType_U16: ImGuiDataType = 3;
// unsigned short
pub const ImGuiDataType_S32: ImGuiDataType = 4;
// int
pub const ImGuiDataType_U32: ImGuiDataType = 5;
// unsigned int
pub const ImGuiDataType_S64: ImGuiDataType = 6;
// long long / __int64
pub const ImGuiDataType_U64: ImGuiDataType = 7;
// unsigned long long / unsigned __int64
pub const ImGuiDataType_Float: ImGuiDataType = 8;
// float
pub const ImGuiDataType_Double: ImGuiDataType = 9;
// double
pub const ImGuiDataType_String: ImGuiDataType = 10;
pub const ImGuiDataType_Pointer: ImGuiDataType = 11;
pub const ImGuiDataType_ID: ImGuiDataType = 12;

pub const ImGuiDataType_COUNT: ImGuiDataType = 13;
// };
