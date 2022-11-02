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
        if window_reappearing {
            // memset(Widths, 0, sizeof(Widths));
            self.Widths = [0, 0, 0, 0];
        }
        self.Spacing = spacing as u16;
        self.CalcNextTotalWidth(true);
        // memset(Widths, 0, sizeof(Widths));

        self.TotalWidth = self.NextTotalWidth;
        self.NextTotalWidth = 0;
    }

    // DeclColumns: c_float(w_icon: c_float,w_label: c_float,w_shortcut: c_float,w_mark: c_float);
    pub fn DeclColumns(
        &mut self,
        w_icon: c_float,
        w_label: c_float,
        w_shortcut: c_float,
        w_mark: c_float,
    ) -> c_float {
        self.Widths[0] = self.Widths[0].max(w_icon as u16);
        self.Widths[1] = self.Widths[1].max(w_label as u16);
        self.Widths[2] = self.Widths[2].max(w_shortcut as u16);
        self.Widths[3] = self.Widths[3].max(w_mark as u16);
        self.CalcNextTotalWidth(false);
        return self.TotalWidth.max(self.NextTotalWidth) as c_float;
    }

    // c_void        CalcNextTotalWidth(update_offsets: bool);
    pub fn CalcNextTotalWidth(&mut self, update_offsets: bool) {
        let mut offset = 0;
        let mut want_spacing: bool = false;
        // for (let i: c_int = 0; i < Widths.len(); i++)
        for i in 0..self.Widths.len() {
            let width = Widths[i];
            if want_spacing && width > 0 {
                offset += self.Spacing;
            }
            want_spacing |= (width > 0);
            if update_offsets {
                if i == 1 {
                    self.OffsetLabel = offset;
                }
                if i == 2 {
                    self.OffsetShortcut = offset;
                }
                if i == 3 {
                    self.OffsetMark = offset;
                }
            }
            offset += width;
        }
        self.NextTotalWidth = offset as u32;
    }
}
