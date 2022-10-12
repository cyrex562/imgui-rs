use libc::c_int;

// typedef int ImGuiInputTextFlags;    // -> enum ImGuiInputTextFlags_  // Flags: for InputText(), InputTextMultiline()
pub type ImGuiInputTextFlags = c_int;
