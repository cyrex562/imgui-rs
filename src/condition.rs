use libc::c_int;

// typedef int ImGuiCond;              // -> enum ImGuiCond_            // Enum: A condition for many Set*() functions
pub type ImGuiCond = c_int;
