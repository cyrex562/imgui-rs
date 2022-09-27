use libc::c_int;

// typedef int ImGuiBackendFlags;      // -> enum ImGuiBackendFlags_    // Flags: for io.BackendFlags
pub type ImGuiBackendFlags = c_int;
