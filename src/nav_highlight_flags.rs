use libc::c_int;

// typedef int ImGuiNavHighlightFlags;     // -> enum ImGuiNavHighlightFlags_  // Flags: for RenderNavHighlight()
pub type ImGuiNavHighlightFlags = c_int;

// enum ImGuiNavHighlightFlags_
// {
pub const ImGuiNavHighlightFlags_None: ImGuiNavHighlightFlags = 0;
pub const ImGuiNavHighlightFlags_TypeDefault: ImGuiNavHighlightFlags = 1 << 0;
pub const ImGuiNavHighlightFlags_TypeThin: ImGuiNavHighlightFlags = 1 << 1;
pub const ImGuiNavHighlightFlags_AlwaysDraw: ImGuiNavHighlightFlags = 1 << 2;
// Draw rectangular highlight if (g.NavId == id) _even_ when using the mouse.
pub const ImGuiNavHighlightFlags_NoRounding: ImGuiNavHighlightFlags = 1 << 3;
// };
