use libc::c_int;

// typedef int ImGuiTabItemFlags;      // -> enum ImGuiTabItemFlags_    // Flags: for BeginTabItem()
pub type ImGuiTabItemFlags = c_int;
