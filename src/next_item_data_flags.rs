use libc::c_int;

// typedef int ImGuiNextItemDataFlags;     // -> enum ImGuiNextItemDataFlags_  // Flags: for SetNextItemXXX() functions
pub type ImGuiNextItemDataFlags = c_int;
