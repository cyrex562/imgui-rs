use libc::c_int;

#[derive(Default, Debug, Copy, Clone)]
pub struct ImGuiTabBarSection {
    // c_int                 TabCount;               // Number of tabs in this section.Width: c_float;                  // Sum of width of tabs in this section (after shrinking down)Spacing: c_float;                // Horizontal spacing at the end of the section.

    // ImGuiTabBarSection() { memset(this, 0, sizeof(*this)); }
    pub TabCount: c_int,
}
