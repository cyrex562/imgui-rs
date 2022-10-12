use libc::c_int;

// typedef int ImGuiFocusedFlags;      // -> enum ImGuiFocusedFlags_    // Flags: for IsWindowFocused()
pub type ImGuiFocusedFlags = c_int;
