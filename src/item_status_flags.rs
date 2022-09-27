use libc::c_int;

// typedef int ImGuiItemStatusFlags;       // -> enum ImGuiItemStatusFlags_    // Flags: for DC.LastItemStatusFlags
pub type ImGuiItemStatusFlags = c_int;
