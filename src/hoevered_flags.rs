use libc::c_int;

// typedef int ImGuiHoveredFlags;      // -> enum ImGuiHoveredFlags_    // Flags: for IsItemHovered(), IsWindowHovered() etc.
pub type ImGuiHoveredFlags = c_int;
