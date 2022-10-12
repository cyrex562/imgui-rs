use libc::c_int;

// typedef int ImGuiComboFlags;        // -> enum ImGuiComboFlags_      // Flags: for BeginCombo()
pub type ImGuiComboFlags = c_int;
