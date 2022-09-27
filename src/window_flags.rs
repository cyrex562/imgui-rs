use libc::c_int;

// typedef int ImGuiWindowFlags;       // -> enum ImGuiWindowFlags_     // Flags: for Begin(), BeginChild()
pub type ImGuiWindowFlags = c_int;
