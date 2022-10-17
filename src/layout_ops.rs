#![allow(non_snake_case)]

use crate::GImGui;

// Gets back to previous line and continue with horizontal layout
//      offset_from_start_x == 0 : follow right after previous item
//      offset_from_start_x != 0 : align to specified x position (relative to window/group left)
//      spacing_w < 0            : use default spacing if pos_x == 0, no spacing if pos_x != 0
//      spacing_w >= 0           : enforce spacing amount
pub unsafe fn SameLine(offset_from_start_x: c_float, spacing_w: c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if window.SkipItems {
        return;
    }

    if offset_from_start_x != 0.0 {
        if spacing_w < 0.0 {
            spacing_w = 0.0;
        }
        window.DC.CursorPos.x = window.Pos.x - window.Scroll.x + offset_from_start_x + spacing_w + window.DC.GroupOffset.x + window.DC.ColumnsOffset.x;
        window.DC.CursorPos.y = window.DC.CursorPosPrevLine.y;
    } else {
        if spacing_w < 0.0 {
            spacing_w = g.Style.ItemSpacing.x;
        }
        window.DC.CursorPos.x = window.DC.CursorPosPrevLine.x + spacing_w;
        window.DC.CursorPos.y = window.DC.CursorPosPrevLine.y;
    }
    window.DC.CurrLineSize = window.DC.PrevLineSize;
    window.DC.CurrLineTextBaseOffset = window.DC.PrevLineTextBaseOffset;
    window.DC.IsSameLine = true;
}
