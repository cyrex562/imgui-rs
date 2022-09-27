use libc::c_int;

// typedef int ImGuiInputFlags;            // -> enum ImGuiInputFlags_         // Flags: for IsKeyPressedEx()
pub type ImGuiInputFlags = c_int;
