use libc::c_int;

// typedef int ImGuiKey;               // -> enum ImGuiKey_             // Enum: A key identifier
pub type ImGuiKey = c_int;
