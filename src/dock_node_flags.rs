use libc::c_int;

// typedef int ImGuiDockNodeFlags;     // -> enum ImGuiDockNodeFlags_   // Flags: for DockSpace()
pub type ImGuiDockNodeFlags = c_int;
