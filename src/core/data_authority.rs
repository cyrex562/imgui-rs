use libc::c_int;

pub type ImGuiDataAuthority = c_int;

pub const IM_GUI_DATA_AUTHORITY_AUTO: ImGuiDataAuthority = 0;
pub const IM_GUI_DATA_AUTHORITY_DOCK_NODE: ImGuiDataAuthority = 1;
pub const IM_GUI_DATA_AUTHORITY_WINDOW: ImGuiDataAuthority = 2;

