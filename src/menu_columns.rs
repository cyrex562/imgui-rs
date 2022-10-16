use libc::c_float;

// Simple column measurement, currently used for MenuItem() only.. This is very short-sighted/throw-away code and NOT a generic helper.
#[derive(Default, Debug, Copy, Clone)]
struct ImGuiMenuColumns {
    // u32       TotalWidth;
    pub TotalWidth: u32,
    // u32       NextTotalWidth;
    pub NextTotalWidth: u32,
    // ImU16       Spacing;
    pub Spacing: u16,
    // ImU16       OffsetIcon;         // Always zero for now
    pub OffsetIcon: u16,
    // ImU16       OffsetLabel;        // Offsets are locked in Update()
    pub OffsetLabel: u16,
    // ImU16       OffsetShortcut;
    pub OffsetShortcut: u16,
    // ImU16       OffsetMark;
    pub OffsetMark: u16,
    // ImU16       Widths[4];          // Width of:   Icon, Label, Shortcut, Mark  (accumulators for current frame)
    pub Widths: [u16; 4],
}

impl ImGuiMenuColumns {
    // ImGuiMenuColumns() { memset(this, 0, sizeof(*this)); }

    // c_void        Update(spacing: c_float, window_reappearing: bool);
    pub fn Update(&mut self, spacing: c_float, window_reappearing: bool) {
        todo!()
    }

    // DeclColumns: c_float(w_icon: c_float,w_label: c_float,w_shortcut: c_float,w_mark: c_float);
    pub fn DeclColumns(
        &mut self,
        w_icon: c_float,
        w_label: c_float,
        w_shortcut: c_float,
        w_mark: c_float,
    ) -> c_float {
        todo!()
    }

    // c_void        CalcNextTotalWidth(update_offsets: bool);
    pub fn CalcNextTotalWidth(&mut self, update_offsets: bool) {
        todo!()
    }
}
