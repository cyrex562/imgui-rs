use libc::c_int;

// typedef int ImGuiNextWindowDataFlags;   // -> enum ImGuiNextWindowDataFlags_// Flags: for SetNextWindowXXX()
pub type ImGuiNextWindowDataFlags = c_int;
