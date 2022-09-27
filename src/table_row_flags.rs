use libc::c_int;

// typedef int ImGuiTableRowFlags;     // -> enum ImGuiTableRowFlags_   // Flags: For TableNextRow()
pub type ImGuiTableRowFlags = c_int;
