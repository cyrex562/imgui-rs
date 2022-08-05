// Simple column measurement, currently used for menu_item() only.. This is very short-sighted/throw-away code and NOT a generic helper.
#[derive(Debug,Clone,Default)]
pub struct MenuColumns
{
    // ImU32       total_width;
    pub total_width: u32,
    // ImU32       next_total_width;
    pub next_total_width: u32,
    // ImU16       spacing;
    pub spacing: u16,
    // ImU16       offset_icon;         // Always zero for now
    pub offset_icon: u16,
    // ImU16       offset_label;        // Offsets are locked in update()
    pub offset_label: u16,
    // ImU16       offset_shortcut;
    pub offset_shortcut: u16,
    // pImU16       offset_mark;
    pub offset_mark: *mut u16,
    // ImU16       widths[4];          // width of:   Icon, Label, Shortcut, Mark  (accumulators for current frame)
    pub widths: [u16;4],
}

impl MenuColumns {
    // ImGuiMenuColumns() { memset(this, 0, sizeof(*this)); }
    // void        update(float spacing, bool window_reappearing);
    pub fn update(&mut self, spacing: f32, window_reappearing: bool) {
        todo!()
    }
    // float       decl_columns(float w_icon, float w_label, float w_shortcut, float w_mark);
    pub fn decl_columns(&mut self, w_icon: f32, w_label: f32, w_shortcut: f32, w_mark: f32) -> f32 {
        todo!()
    }
    // void        calc_next_total_width(bool update_offsets);
    pub fn calc_next_total_width(&mut self, update_offsets: bool) {
        todo!()
    }
}
