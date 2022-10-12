use libc::c_int;

// typedef int ImGuiOldColumnFlags;        // -> enum ImGuiOldColumnFlags_     // Flags: for BeginColumns()
pub type ImGuiOldColumnFlags = c_int;
