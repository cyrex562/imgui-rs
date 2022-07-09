// Simple column measurement, currently used for MenuItem() only.. This is very short-sighted/throw-away code and NOT a generic helper.
#[derive(Debug,Clone,Default)]
pub struct  ImGuiMenuColumns
{
    // ImU32       TotalWidth;
    pub TotalWidth: u32,
    // ImU32       NextTotalWidth;
    pub NextTotalWidth: u32,
    // ImU16       Spacing;
    pub Spacing: u16,
    // ImU16       OffsetIcon;         // Always zero for now
    pub OffsetIcon: u16,
    // ImU16       OffsetLabel;        // Offsets are locked in Update()
    pub OffsetLabel: u16,
    // ImU16       OffsetShortcut;
    pub OffsetShortcut: u16,
    // pImU16       OffsetMark;
    pub OffsetMark: *mut u16,
    // ImU16       Widths[4];          // width of:   Icon, Label, Shortcut, Mark  (accumulators for current frame)
    pub Widths: [u16;4],
}

impl ImGuiMenuColumns {
    // ImGuiMenuColumns() { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    // void        Update(float spacing, bool window_reappearing);
    pub fn Update(&mut self, spacing: f32, window_reappearing: bool) {
        todo!()
    }
    // float       DeclColumns(float w_icon, float w_label, float w_shortcut, float w_mark);
    pub fn DeclColumns(&mut self, w_icon: f32, w_label: f32, w_shortcut: f32, w_mark: f32) -> f32 {
        todo!()
    }
    // void        CalcNextTotalWidth(bool update_offsets);
    pub fn CalcNextTotalWidth(&mut self, update_offsets: bool) {
        todo!()
    }
}
