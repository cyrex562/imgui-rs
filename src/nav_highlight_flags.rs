use libc::c_int;

// typedef int ImGuiNavHighlightFlags;     // -> enum ImGuiNavHighlightFlags_  // Flags: for RenderNavHighlight()
pub type ImGuiNavHighlightFlags = c_int;
