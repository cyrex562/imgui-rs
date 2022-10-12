use libc::c_int;

// typedef int ImGuiTableBgTarget;     // -> enum ImGuiTableBgTarget_   // Enum: A color target for TableSetBgColor()
pub type ImGuiTableBgTarget = c_int;
