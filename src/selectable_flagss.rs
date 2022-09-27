use libc::c_int;

// typedef int ImGuiSelectableFlags;   // -> enum ImGuiSelectableFlags_ // Flags: for Selectable()
pub type ImGuiSelectableFlags = c_int;
