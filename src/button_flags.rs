use libc::c_int;

// typedef int ImGuiButtonFlags;       // -> enum ImGuiButtonFlags_     // Flags: for InvisibleButton()
pub type ImGuiButtonFlags = c_int;
