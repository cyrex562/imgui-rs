use libc::c_int;

pub type ImGuiDataType = c_int;

// Standard Drag and Drop payload types. You can define you own payload types using short strings. Types starting with '_' are defined by Dear ImGui.
pub type ImGuiPayloadType = i32;
pub const IM_GUI_PAYLOAD_TYPE_COLOR3F: ImGuiPayloadType = 0;
pub const IMGUI_PAYLOAD_TYPE_COLOR4F: ImGuiPayloadType = 1;

pub const IM_GUI_DATA_TYPE_S8: ImGuiDataType = 0;
pub const IM_GUI_DATA_TYPE_U8: ImGuiDataType = 1;
pub const IM_GUI_DATA_TYPE_S16: ImGuiDataType = 2;
pub const IM_GUI_DATA_TYPE_U16: ImGuiDataType = 3;
pub const IM_GUI_DATA_TYPE_S32: ImGuiDataType = 4;
pub const IM_GUI_DATA_TYPE_U32: ImGuiDataType = 5;
pub const IM_GUI_DATA_TYPE_S64: ImGuiDataType = 6;
pub const IM_GUI_DATA_TYPE_U64: ImGuiDataType = 7;
pub const IM_GUI_DATA_TYPE_FLOAT: ImGuiDataType = 8;
pub const IM_GUI_DATA_TYPE_DOUBLE: ImGuiDataType = 9;
pub const IM_GUI_DATA_TYPE_STRING: ImGuiDataType = 10;
pub const IM_GUI_DATA_TYPE_POINTER: ImGuiDataType = 11;
pub const IM_GUI_DATA_TYPE_ID: ImGuiDataType = 12;

pub const IM_GUI_DATA_TYPE_COUNT: ImGuiDataType = 13;
