// Simple column measurement, currently used for MenuItem() only.. This is very short-sighted/throw-away code and NOT a generic helper.
#[derive(Default, Debug, Clone, Copy)]
pub struct ImGuiMenuColumns {
    pub TotalWidth: u32,
    pub NextTotalWidth: u32,
    pub Spacing: u16,
    pub OffsetIcon: u16,
    // Always zero for now
    pub OffsetLabel: u16,
    // Offsets are locked in Update()
    pub OffsetShortcut: u16,
    pub OffsetMark: u16,
    pub Widths: [u16; 4],          // Width of:   Icon, Label, Shortcut, Mark  (accumulators for current frame)
}

impl ImGuiMenuColumns {
    // ImGuiMenuColumns() { memset(this, 0, sizeof(*this)); }


    // c_void        Update(spacing: c_float, window_reappearing: bool);


    // c_float       DeclColumns(w_icon: c_float, w_label: c_float, w_shortcut: c_float, w_mark: c_float);


    // c_void        CalcNextTotalWidth(update_offsets: bool);
}
