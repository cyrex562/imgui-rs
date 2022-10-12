#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::{c_float, c_int};
use crate::color::{IM_COL32_DISABLE, ImGuiCol_TableRowBg, ImGuiCol_TableRowBgAlt};
use crate::cursor_ops::ErrorCheckUsingSetCursorPosToExtendParentBoundaries;
use crate::draw_flags::ImDrawFlags_None;
use crate::GImGui;
use crate::imgui_cpp::GImGui;
use crate::logging_ops::LogRenderedText;
use crate::math::{ImMax, ImMin};
use crate::nav_layer::{ImGuiNavLayer_Main, ImGuiNavLayer_Menu};
use crate::rect::ImRect;
use crate::style_ops::GetColorU32;
use crate::table::ImGuiTable;
use crate::table_column::ImGuiTableColumn;
use crate::table_flags::{ImGuiTableFlags_BordersInnerH, ImGuiTableFlags_NoClip, ImGuiTableFlags_RowBg};
use crate::table_instance_data::ImGuiTableInstanceData;
use crate::table_row_flags::ImGuiTableRowFlags_Headers;
use crate::vec2::ImVec2;
use crate::window_ops::SetWindowClipRectBeforeSetChannel;

// [Internal] Called by TableNextRow()
// c_void TableEndRow(*mut ImGuiTable table)
pub unsafe fn TableEndRow(table: *mut ImGuiTable) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
// *mut ImGuiWindow window = g.CurrentWindow;
    let window = g.CurrentWindow;
    // IM_ASSERT(window == table.InnerWindow);
// IM_ASSERT(table.IsInsideRow);

    if table.CurrentColumn != -1 {
        TableEndCell(table);
    }

// Logging
    if g.LogEnabled {
        LogRenderedText(null_mut(), "|".into(), null_mut());
    }

// Position cursor at the bottom of our row so it can be used for e.g. clipping calculation. However it is
// likely that the next call to TableBeginCell() will reposition the cursor to take account of vertical padding.
    window.DC.CursorPos.y = table.RowPosY2.clone();

// Row background fill
    let bg_y1 = table.RowPosY1.clone();
    let bg_y2 = table.RowPosY2.clone();
    let unfreeze_rows_actual = (table.CurrentRow.clone() + 1 == table.FreezeRowsCount.clone() as i32);
    let unfreeze_rows_request = (table.CurrentRow.clone() + 1 == table.FreezeRowsRequest.clone() as i32);
    if table.CurrentRow == 0 {
        TableGetInstanceData(table, table.InstanceCurrent.clone()).LastFirstRowHeight = bg_y2 - bg_y1;
    }

    let is_visible = (bg_y2 >= table.InnerClipRect.Min.y && bg_y1 <= table.InnerClipRect.Max.y);
    if is_visible {
// Decide of background color for the row
        let mut bg_col0 = 0;
        let mut bg_col1 = 0;
        if table.RowBgColor[0] != IM_COL32_DISABLE {
            bg_col0 = table.RowBgColor[0];
        } else if table.Flags.clone() & ImGuiTableFlags_RowBg {
            bg_col0 = GetColorU32(if table.RowBgColorCounter.clone() & 1 { ImGuiCol_TableRowBgAlt } else { ImGuiCol_TableRowBg }, 0.0);
        }
        if table.RowBgColor[1] != IM_COL32_DISABLE {
            bg_col1 = table.RowBgColor[1];
        }

// Decide of top border color
        let mut border_col = 0;
        let border_size = TABLE_BORDER_SIZE;
        if table.CurrentRow > 0 || table.InnerWindow == table.OuterWindow {
            if table.Flags.clone() & ImGuiTableFlags_BordersInnerH {
                border_col = if table.LastRowFlags.clone() & ImGuiTableRowFlags_Headers {
                    table.BorderColorStrong.clone()
                } else { table.BorderColorLight.clone() };
            }
        }

        let draw_cell_bg_color = table.RowCellDataCurrent >= 0;
        let draw_strong_bottom_border = unfreeze_rows_actual;
        if (bg_col0 | bg_col1 | border_col) != 0 || draw_strong_bottom_border || draw_cell_bg_color {
// In theory we could call SetWindowClipRectBeforeSetChannel() but since we know TableEndRow() is
// always followed by a change of clipping rectangle we perform the smallest overwrite possible here.
            if (table.Flags.clone() & ImGuiTableFlags_NoClip) == 0 {
                window.DrawList._CmdHeader.ClipRect = table.Bg0ClipRectForDrawCmd.ToVec4();
            }
            table.DrawSplitter.SetCurrentChannel(window.DrawList, TABLE_DRAW_CHANNEL_BG0);
        }

// Draw row background
// We soft/cpu clip this so all backgrounds and borders can share the same clipping rectangle
        if bg_col0.clone() || bg_col1.clone() {
            let mut row_rect = ImRect::new4(table.WorkRect.Min.x.clone(), bg_y1, table.WorkRect.Max.x.clone(), bg_y2);
            row_rect.ClipWith(&table.BgClipRect);
            if bg_col0 != 0 && row_rect.Min.y < row_rect.Max.y {
                window.DrawList.AddRectFilled(&row_rect.Min, &row_rect.Max, bg_col0.clone(), 0f32, ImDrawFlags_None);
            }
            if bg_col1 != 0 && row_rect.Min.y < row_rect.Max.y {
                window.DrawList.ddRectFilled(row_rect.Min, row_rect.Max, bg_col1.clone());
            }
        }

// Draw cell background color
        if draw_cell_bg_color {
            let mut cell_data_end = &mut table.RowCellData[table.RowCellDataCurrent.clone()];
            let mut cell_data = &mut table.RowCellData[0];
// for (*mut ImGuiTableCellData cell_data = &table.RowCellData[0]; cell_data <= cell_data_end; cell_data++)
            while cell_data <= cell_data_end {
// As we render the BG here we need to clip things (for layout we would not)
// FIXME: This cancels the OuterPadding addition done by TableGetCellBgRect(), need to keep it while rendering correctly while scrolling.
                let column: &mut ImGuiTableColumn = &mut table.Columns[cell_data.Column];
                let mut cell_bg_rect = TableGetCellBgRect(table, cell_data.Column);
                cell_bg_rect.ClipWith(&table.BgClipRect);
                cell_bg_rect.Min.x = ImMax(cell_bg_rect.Min.x, column.ClipRect.Min.x.clone());     // So that first column after frozen one gets clipped when scrolling
                cell_bg_rect.Max.x = ImMin(cell_bg_rect.Max.x, column.MaxX.clone());
                window.DrawList.AddRectFilled(&cell_bg_rect.Min, &cell_bg_rect.Max, cell_data.BgColor, 0f32, ImDrawFlags_None);
                cell_data += 1;
            }
        }

// Draw top border
        if border_col > 0 && bg_y1 >= table.BgClipRect.Min.y && bg_y1 < table.BgClipRect.Max.y {
            window.DrawList.AddLine(&mut ImVec2::new2(table.BorderX1.clone(), bg_y1), &mut ImVec2::new2(table.BorderX2.clone(), bg_y1), border_col.clone(), border_size);
        }

// Draw bottom border at the row unfreezing mark (always strong)
        if draw_strong_bottom_border.clone() && bg_y2 >= table.BgClipRect.Min.y && bg_y2 < table.BgClipRect.Max.y {
            window.DrawList.AddLine(&mut ImVec2::new2(table.BorderX1.clone(), bg_y2), &mut ImVec2::new2(table.BorderX2.clone(), bg_y2), table.BorderColorStrong.clone(), border_size);
        }
    }

// End frozen rows (when we are past the last frozen row line, teleport cursor and alter clipping rectangle)
// We need to do that in TableEndRow() instead of TableBeginRow() so the list clipper can mark end of row and
// get the new cursor position.
    if unfreeze_rows_request {
        // for (c_int column_n = 0; column_n < table.ColumnsCount; column_n+ +)
        for column_n in 0..table.ColumnsCount {
            let mut column: &mut ImGuiTableColumn = &mut table.Columns[column_n];
            column.NavLayerCurrent = if column_n < table.FreezeColumnsCount.clone() as c_int { ImGuiNavLayer_Menu } else { ImGuiNavLayer_Main };
        }
    }
    if unfreeze_rows_actual {
// IM_ASSERT(table.IsUnfrozenRows == false);
        table.IsUnfrozenRows = true;

// BgClipRect starts as table.InnerClipRect, reduce it now and make BgClipRectForDrawCmd == BgClipRect
        let mut y0 = ImMax(table.RowPosY2.clone()+ 1, window.InnerClipRect.Min.y.clone());
        table.BgClipRect.Min.y = ImMin(y0, window.InnerClipRect.Max.y.clone());
        ;
        table.Bg2ClipRectForDrawCmd.Min.y = ImMin(y0, window.InnerClipRect.Max.y.clone());
        table.BgClipRect.Max.y = window.InnerClipRect.Max.y.clone();
        table.Bg2ClipRectForDrawCmd.Max.y = window.InnerClipRect.Max.y.clone();
        table.Bg2DrawChannelCurrent = table.Bg2DrawChannelUnfrozen.clone();
// IM_ASSERT(table.Bg2ClipRectForDrawCmd.Min.y <= table.Bg2ClipRectForDrawCmd.Max.y);

        let mut row_height = table.RowPosY2.clone() - table.RowPosY1.clone();
        table.RowPosY2 = table.WorkRect.Min.y.clone() + table.RowPosY2.clone() - table.OuterRect.Min.y.clone();
        window.DC.CursorPos.y = table.WorkRect.Min.y.clone() + table.RowPosY2.clone() - table.OuterRect.Min.y.clone();
        table.RowPosY1 = table.RowPosY2.clone() - row_height;
// for (c_int column_n = 0; column_n < table.ColumnsCount; column_n++)
        for column_n in 0..table.ColumnsCount {
            let mut column: &mut ImGuiTableColumn = &mut table.Columns[column_n];
            column.DrawChannelCurrent = column.DrawChannelUnfrozen.clone();
            column.ClipRect.Min.y = table.Bg2ClipRectForDrawCmd.Min.y.clone();
        }

// Update cliprect ahead of TableBeginCell() so clipper can access to new ClipRect.Min.y
        SetWindowClipRectBeforeSetChannel(window, table.Columns[0].ClipRect);
        table.DrawSplitter.SetCurrentChannel(window.DrawList, table.Columns[0].DrawChannelCurrent);
    }

    if !(table.RowFlags.clone() & ImGuiTableRowFlags_Headers) {
        table.RowBgColorCounter += 1;
    }
    table.IsInsideRow = false;
}


// [Internal] Called by TableNextRow()/TableSetColumnIndex()/TableNextColumn()
// c_void TableEndCell(*mut ImGuiTable table)
pub unsafe fn TableEndCell(table: *mut ImGuiTable) {
    let mut column: &mut ImGuiTableColumn = &mut table.Columns[table.CurrentColumn.clone()];
    let mut window = table.InnerWindow;

    if window.DC.IsSetPos {
        ErrorCheckUsingSetCursorPosToExtendParentBoundaries();
    }

// Report maximum position so we can infer content size per column.
// *mut let mut p_max_pos_x: c_float = 0f32;
    let mut p_max_pos_x: *mut c_float = null_mut();
    if table.RowFlags.clone() & ImGuiTableRowFlags_Headers {
        p_max_pos_x = &mut column.ContentMaxXHeadersUsed;
    }  // Useful in case user submit contents in header row that is not a TableHeader() call
    else {
        p_max_pos_x = if table.IsUnfrozenRows { &mut column.ContentMaxXUnfrozen } else { &mut column.ContentMaxXFrozen };
    }
    *p_max_pos_x = ImMax(*p_max_pos_x.clone(), window.DC.CursorMaxPos.x.clone());
    table.RowPosY2 = ImMax(table.RowPosY2.clone(), window.DC.CursorMaxPos.y.clone() + table.CellPaddingY.clone());
    column.ItemWidth = window.DC.ItemWidth.clone();

// Propagate text baseline for the entire row
// FIXME-TABLE: Here we propagate text baseline from the last line of the cell.. instead of the first one.
    table.RowTextBaseline = ImMax(table.RowTextBaseline.clone(), window.DC.PrevLineTextBaseOffset.clone());
}


// inline *mut ImGuiTableInstanceData   TableGetInstanceData(*mut ImGuiTable table, instance_no: c_int)
pub fn TableGetInstanceData(table: *mut ImGuiTable, instance_no: c_int) -> *mut ImGuiTableInstanceData {
    if instance_no == 0 {
        return &mut table.InstanceDataFirst;
    }
    return &mut table.InstanceDataExtra[instance_no - 1];
}



// Return the cell rectangle based on currently known height.
// - Important: we generally don't know our row height until the end of the row, so Max.y will be incorrect in many situations.
//   The only case where this is correct is if we provided a min_row_height to TableNextRow() and don't go below it, or in TableEndRow() when we locked that height.
// - Important: if ImGuiTableFlags_PadOuterX is set but ImGuiTableFlags_PadInnerX is not set, the outer-most left and right
//   columns report a small offset so their CellBgRect can extend up to the outer border.
//   FIXME: But the rendering code in TableEndRow() nullifies that with clamping required for scrolling.
// ImRect TableGetCellBgRect(*const ImGuiTable table, column_n: c_int)
pub fn TableGetCellBgRect(table: *const ImGuiTable, column_n: c_int) -> ImRect {
    let column: &ImGuiTableColumn = &table.Columns[column_n];
    let mut x1 = column.MinX.clone();
    let mut x2 = column.MaxX.clone();
    //if (column.PrevEnabledColumn == -1)
    //    x1 -= table.OuterPaddingX;
    //if (column.NextEnabledColumn == -1)
    //    x2 += table.OuterPaddingX;
    x1 = ImMax(x1, table.WorkRect.Min.x.clone());
    x2 = ImMin(x2, table.WorkRect.Max.x.clone());
    return ImRect::new4(x1, table.RowPosY1.clone(), x2, table.RowPosY2.clone());
}
