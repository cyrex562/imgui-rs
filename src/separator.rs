use libc::c_float;
use std::ptr::{null, null_mut};
use crate::color::ImGuiCol_Separator;
use crate::GImGui;
use crate::item_ops::{ItemAdd, ItemSize};
use crate::layout_type::ImGuiLayoutType_Horizontal;
use crate::old_columns::ImGuiOldColumns;
use crate::rect::ImRect;
use crate::separator_flags::{ImGuiSeparatorFlags, ImGuiSeparatorFlags_Horizontal, ImGuiSeparatorFlags_SpanAllColumns, ImGuiSeparatorFlags_Vertical};
use crate::style_ops::GetColorU32;
use crate::table::ImGuiTable;
use crate::tables::{PopColumnsBackground, PushColumnsBackground};
use crate::utils::flag_set;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;
use crate::window::ops::GetCurrentWindow;

// Horizontal/vertical separating line
pub unsafe fn SeparatorEx(flags: ImGuiSeparatorFlags)
{
    let mut window = GetCurrentWindow();
    if window.SkipItems { return ; }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(ImIsPowerOfTwo(flags & (ImGuiSeparatorFlags_Horizontal | ImGuiSeparatorFlags_Vertical)));   // Check that only 1 option is selected

    let thickness_draw: c_float =  1.0;
    let thickness_layout: c_float =  0.0;
    if flags & ImGuiSeparatorFlags_Vertical
    {
        // Vertical separator, for menu bars (use current line height). Not exposed because it is misleading and it doesn't have an effect on regular layout.
        let y1: c_float =  window.DC.CursorPos.y;
        let y2: c_float =  window.DC.CursorPos.y + window.DC.CurrLineSize.y;
        let mut bb: ImRect = ImRect::new(ImVec2::from_floats(window.DC.CursorPos.x, y1), ImVec2::from_floats(window.DC.CursorPos.x + thickness_draw, y2));
        ItemSize(&ImVec2::from_floats(thickness_layout, 0.0), 0.0);
        if !ItemAdd(&mut bb, 0, None, 0) { return ; }

        // Draw
        window.DrawList.AddLine(&ImVec2::from_floats(bb.Min.x, bb.Min.y), &ImVec2::from_floats(bb.Min.x, bb.Max.y), GetColorU32(ImGuiCol_Separator, 0.0), 0.0);
        if (g.LogEnabled) {
            // LogText(" |");
        }
    }
    else if flag_set(flags , ImGuiSeparatorFlags_Horizontal)
    {
        // Horizontal Separator
        let mut x1: c_float =  window.Pos.x;
        let mut x2: c_float =  window.Pos.x + window.Size.x;

        // FIXME-WORKRECT: old hack (#205) until we decide of consistent behavior with WorkRect/Indent and Separator
        if g.GroupStack.Size > 0 && g.GroupStack.last().unwrap().WindowID == window.ID {
            x1 += window.DC.Indent.x;
        }

        // FIXME-WORKRECT: In theory we should simply be using WorkRect.Min.x/Max.x everywhere but it isn't aesthetically what we want,
        // need to introduce a variant of WorkRect for that purpose. (#4787)
        if table: *mut ImGuiTable = g.CurrentTable
        {
            x1 = table.Columns[table.CurrentColumn].MinX;
            x2 = table.Columns[table.CurrentColumn].MaxX;
        }

        columns: *mut ImGuiOldColumns = if flags & ImGuiSeparatorFlags_SpanAllColumns { window.DC.CurrentColumns} else { None};
        if columns {
            PushColumnsBackground(); }

        // We don't provide our width to the layout so that it doesn't get feed back into AutoFit
        // FIXME: This prevents ->CursorMaxPos based bounding box evaluation from working (e.g. TableEndCell)
        let mut bb: ImRect = ImRect::new(ImVec2::from_floats(x1, window.DC.CursorPos.y), ImVec2::from_floats(x2, window.DC.CursorPos.y + thickness_draw));
        ItemSize(&ImVec2::from_floats(0.0, thickness_layout), 0.0);
        let item_visible: bool = ItemAdd(&mut bb, 0, None, 0);
        if item_visible
        {
            // Draw
            window.DrawList.AddLine(&bb.Min, &ImVec2::from_floats(bb.Max.x, bb.Min.y), GetColorU32(ImGuiCol_Separator, 0.0), 0.0);
            if g.LogEnabled {
                // LogRenderedText(&bb.Min, "--------------------------------\n");
            }

        }
        if columns
        {
            PopColumnsBackground();
            columns.LineMinY = window.DC.CursorPos.y;
        }
    }
}

pub unsafe fn Separator()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window  = &g.CurrentWindow;
    if window.SkipItems { return ; }

    // Those flags should eventually be overridable by the user
    let mut flags: ImGuiSeparatorFlags = if window.DC.LayoutType == ImGuiLayoutType_Horizontal { ImGuiSeparatorFlags_Vertical} else { ImGuiSeparatorFlags_Horizontal};
    flags |= ImGuiSeparatorFlags_SpanAllColumns; // NB: this only applies to legacy Columns() api as they relied on Separator() a lot.
    SeparatorEx(flags);
}
