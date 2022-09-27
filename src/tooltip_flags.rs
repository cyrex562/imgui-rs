use libc::c_int;

// typedef int ImGuiTooltipFlags;          // -> enum ImGuiTooltipFlags_       // Flags: for BeginTooltipEx()
pub type ImGuiTooltipFlags = c_int;
