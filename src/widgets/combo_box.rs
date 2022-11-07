use crate::button_ops::ButtonBehavior;
use crate::color::{
    ImGuiCol_Button, ImGuiCol_ButtonHovered, ImGuiCol_FrameBg, ImGuiCol_FrameBgHovered,
    ImGuiCol_Text,
};
use crate::combo_flags::{
    ImGuiComboFlags, ImGuiComboFlags_CustomPreview, ImGuiComboFlags_HeightLarge,
    ImGuiComboFlags_HeightMask_, ImGuiComboFlags_HeightRegular, ImGuiComboFlags_HeightSmall,
    ImGuiComboFlags_NoArrowButton, ImGuiComboFlags_NoPreview, ImGuiComboFlags_None,
    ImGuiComboFlags_PopupAlignLeft,
};
use crate::combo_preview_data::ImGuiComboPreviewData;
use crate::condition::ImGuiCond_None;
use crate::context::ImguiContext;
use crate::direction::{ImGuiDir_Down, ImGuiDir_Left};
use crate::draw_flags::{
    ImDrawFlags_RoundCornersAll, ImDrawFlags_RoundCornersLeft, ImDrawFlags_RoundCornersRight,
};
use crate::draw_list::ImDrawList;
use crate::frame_ops::GetFrameHeight;
use crate::id_ops::pop_win_id_from_stack;
use crate::item_ops::{CalcItemWidth, ItemAdd, ItemSize, MarkItemEdited};
use crate::layout_type::ImGuiLayoutType_Horizontal;
use crate::logging_ops::LogSetNextTextDecoration;
use crate::math_ops::ImMax;
use crate::next_window_data_flags::{
    ImGuiNextWindowDataFlags, ImGuiNextWindowDataFlags_HasSizeConstraint,
};
use crate::popup_flags::ImGuiPopupFlags_None;
use crate::popup_ops::{
    EndPopup, FindBestWindowPosForPopupEx, GetPopupAllowedExtentRect, IsPopupOpen, OpenPopupEx,
};
use crate::popup_position_policy::ImGuiPopupPositionPolicy_ComboBox;
use crate::rect::ImRect;
use crate::render_ops::{
    RenderArrow, RenderFrameBorder, RenderNavHighlight, RenderText, RenderTextClipped,
};
use crate::style_ops::GetColorU32;
use crate::style_var::ImGuiStyleVar_WindowPadding;
use crate::text_ops::CalcTextSize;
use crate::type_defs::ImguiHandle;
use crate::utils::{flag_clear, flag_set};
use crate::vec2::ImVec2;
use crate::widgets::Selectable;
use crate::window::find::FindWindowByName;
use crate::window::focus::SetItemDefaultFocus;
use crate::window::ops::{Begin, CalcWindowNextAutoFitSize, GetCurrentWindow};
use crate::window::props::{SetNextWindowPos, SetNextWindowSizeConstraints};
use crate::window::rect::{PopClipRect, PushClipRect};
use crate::window::window_flags::{
    ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_NoMove,
    ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoTitleBar,
    ImGuiWindowFlags_Popup,
};
use crate::window::ImguiWindow;
use crate::{button_ops, hash_string, popup_ops, GImGui};
use libc::{c_char, c_float, c_int, strlen};
use std::borrow::BorrowMut;
use std::ptr::{null, null_mut};
use crate::a_widgets::Selectable;
use crate::widgets::button_ops::ButtonBehavior;

pub fn BeginCombo(
    g: &mut ImguiContext,
    label: &String,
    preview_value: &mut String,
    flags: ImGuiComboFlags,
) -> bool {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();

    let backup_next_window_data_flags = g.NextWindowData.Flags;
    g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
    if window.skip_items {
        return false;
    }

    let style = &mut g.style;
    let mut id: ImguiHandle = window.id_by_string(g, label);
    // IM_ASSERT((flags & (ImGuiComboFlags_NoArrowButton | ImGuiComboFlags_NoPreview)) != (ImGuiComboFlags_NoArrowButton | ImGuiComboFlags_NoPreview)); // Can't use both flags together

    let arrow_size = if flag_set(flags, ImGuiComboFlags_NoArrowButton) {
        0.0
    } else {
        GetFrameHeight(g)
    };
    let label_size = CalcTextSize(g, label, true, 0.0);
    let w: c_float = if flag_set(flags, ImGuiComboFlags_NoPreview) {
        arrow_size
    } else {
        CalcItemWidth(g)
    };
    let mut bb: ImRect = ImRect::new(
        window.dc.cursor_pos,
        window.dc.cursor_pos + ImVec2::from_floats(w, label_size.y + style.FramePadding.y * 2.0),
    );
    let mut total_bb: ImRect = ImRect::new(
        bb.min,
        bb.max
            + ImVec2::from_floats(
                if label_size.x > 0.0 {
                    style.ItemInnerSpacing.x + label_size.x
                } else {
                    0.0
                },
                0.0,
            ),
    );
    ItemSize(g, &total_bb.GetSize(), style.FramePadding.y);
    if !ItemAdd(g, &mut total_bb, id, Some(&bb), 0) {
        return false;
    }

    // Open on click
    let mut hovered = false;
    let mut held = false;
    let mut pressed: bool = ButtonBehavior(g, &bb, id, &mut hovered, &mut held, 0);
    let mut popup_id: ImguiHandle = hash_string(&String::from("##ComboPopup"), id as u32);
    let mut popup_open: bool = IsPopupOpen(g, popup_id, ImGuiPopupFlags_None);
    if pressed && !popup_open {
        OpenPopupEx(g, popup_id, ImGuiPopupFlags_None);
        popup_open = true;
    }

    // Render shape
    frame_col: u32 = GetColorU32(
        if hovered {
            ImGuiCol_FrameBgHovered
        } else {
            ImGuiCol_FrameBg
        },
        0.0,
    );
    let value_x2: c_float = ImMax(bb.min.x, bb.max.x - arrow_size);
    RenderNavHighlight(g, &bb, id, 0);
    if flag_clear(flags, ImGuiComboFlags_NoPreview) {
        window.DrawList.AddRectFilled(
            &bb.min,
            &ImVec2::from_floats(value_x2, bb.max.y),
            frame_col,
            style.FrameRounding,
            if flag_set(flags, ImGuiComboFlags_NoArrowButton) {
                ImDrawFlags_RoundCornersAll
            } else {
                ImDrawFlags_RoundCornersLeft
            },
        );
    }
    if flag_clear(flags, ImGuiComboFlags_NoArrowButton) {
        bg_col: u32 = GetColorU32(
            if popup_open || hovered {
                ImGuiCol_ButtonHovered
            } else {
                ImGuiCol_Button
            },
            0.0,
        );
        text_col: u32 = GetColorU32(ImGuiCol_Text, 0.0);
        window.DrawList.AddRectFilled(
            &ImVec2::from_floats(value_x2, bb.min.y),
            &bb.max,
            bg_col,
            style.FrameRounding,
            if w <= arrow_size {
                ImDrawFlags_RoundCornersAll
            } else {
                ImDrawFlags_RoundCornersRight
            },
        );
        if value_x2 + arrow_size - style.FramePadding.x <= bb.max.x {
            RenderArrow(
                &mut window.DrawList,
                &ImVec2::from_floats(
                    value_x2 + style.FramePadding.y,
                    bb.min.y + style.FramePadding.y,
                ),
                text_col,
                ImGuiDir_Down,
                1.0,
            );
        }
    }
    RenderFrameBorder(g, bb.min, bb.max, style.FrameRounding);

    // Custom preview
    if flags & ImGuiComboFlags_CustomPreview {
        g.ComboPreviewData.PreviewRect = ImRect(bb.min.x, bb.min.y, value_x2, bb.max.y);
        // IM_ASSERT(preview_value == NULL || preview_value[0] == 0);
        preview_value.clear();
    }

    // Render preview and label
    if preview_value != None && flag_clear(flags, ImGuiComboFlags_NoPreview) {
        if g.LogEnabled {
            LogSetNextTextDecoration("{", "}");
        }
        RenderTextClipped(
            bb.min + style.FramePadding,
            ImVec2::from_floats(value_x2, bb.max.y),
            preview_value,
            None,
            None,
            None,
        );
    }
    if label_size.x > 0.0 {
        RenderText(
            ImVec2::from_floats(
                bb.max.x + style.ItemInnerSpacing.x,
                bb.min.y + style.FramePadding.y,
            ),
            label,
            false,
            g,
        );
    }

    if !popup_open {
        return false;
    }

    g.NextWindowData.Flags = backup_next_window_data_flags;
    return BeginComboPopup(g, popup_id, &mut bb, flags);
}

pub fn BeginComboPopup(
    g: &mut ImguiContext,
    popup_id: ImguiHandle,
    bb: &mut ImRect,
    mut flags: ImGuiComboFlags,
) -> bool {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    if !IsPopupOpen(g, popup_id, ImGuiPopupFlags_None) {
        g.NextWindowData.ClearFlags();
        return false;
    }

    // Set popup size
    let w: c_float = bb.GetWidth();
    if flag_set(
        g.NextWindowData.Flags,
        ImGuiNextWindowDataFlags_HasSizeConstraint,
    ) {
        g.NextWindowData.SizeConstraintRect.min.x =
            ImMax(g.NextWindowData.SizeConstraintRect.min.x, w);
    } else {
        if flag_clear(flags, ImGuiComboFlags_HeightMask_) {
            flags |= ImGuiComboFlags_HeightRegular;
        }
        // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiComboFlags_HeightMask_)); // Only one
        let mut popup_max_height_in_items: c_int = -1;
        if flags & ImGuiComboFlags_HeightRegular {
            popup_max_height_in_items = 8;
        } else if flags & ImGuiComboFlags_HeightSmall {
            popup_max_height_in_items = 4;
        } else if flags & ImGuiComboFlags_HeightLarge {
            popup_max_height_in_items = 20;
        }
        SetNextWindowSizeConstraints(
            g,
            &ImVec2::from_floats(w, 0.0),
            &ImVec2::from_floats(
                f32::MAX,
                popup_ops::CalcMaxPopupHeightFromItemCount(g, popup_max_height_in_items),
            ),
            (),
            None,
        );
    }

    // This is essentially a specialized version of BeginPopupEx()
    name: [c_char; 16];
    // ImFormatString(name, name.len(), "##Combo_{}", g.BeginPopupStack.len()); // Recycle windows based on depth

    // Set position given a custom constraint (peak into expected window size so we can position it)
    // FIXME: This might be easier to express with an hypothetical SetNextWindowPosConstraints() function?
    // FIXME: This might be moved to Begin() or at least around the same spot where Tooltips and other Popups are calling FindBestWindowPosForPopupEx()?
    let mut popup_window = FindWindowByName(g, name);
    if popup_window.is_some() {
        if popup_window.unwrap().WasActive {
            // Always override 'AutoPosLastDirection' to not leave a chance for a past value to affect us.
            let size_expected: ImVec2 =
                CalcWindowNextAutoFitSize(g, popup_window.unwrap().borrow_mut());
            popup_window.AutoPosLastDirection = if flags & ImGuiComboFlags_PopupAlignLeft {
                ImGuiDir_Left
            } else {
                ImGuiDir_Down
            }; // Left = "Below, Toward Left", Down = "Below, Toward Right (default)"
            let mut r_outer: ImRect =
                GetPopupAllowedExtentRect(g, popup_window.unwrap().borrow_mut());
            let pos: ImVec2 = FindBestWindowPosForPopupEx(
                &bb.GetBL(),
                &size_expected,
                &mut popup_window.AutoPosLastDirection,
                &mut r_outer,
                bb,
                ImGuiPopupPositionPolicy_ComboBox,
            );
            SetNextWindowPos(g, &pos, ImGuiCond_None, None);
        }
    }

    // We don't use BeginPopupEx() solely because we have a custom name string, which we could make an argument to BeginPopupEx()
    window_flags: ImGuiWindowFlags = ImGuiWindowFlags_AlwaysAutoResize
        | ImGuiWindowFlags_Popup
        | ImGuiWindowFlags_NoTitleBar
        | ImGuiWindowFlags_NoResize
        | ImGuiWindowFlags_NoSavedSettings
        | ImGuiWindowFlags_NoMove;
    PushStyleVar(
        ImGuiStyleVar_WindowPadding,
        ImVec2::from_floats(g.style.FramePadding.x, g.style.WindowPadding.y),
    ); // Horizontally align ourselves with the framed text
    let mut ret: bool = Begin(g, name, None);
    PopStyleVar();
    if !ret {
        EndPopup(g);
        // IM_ASSERT(0);   // This should never happen as we tested for IsPopupOpen() above
        return false;
    }
    return true;
}

pub fn EndCombo(g: &mut ImguiContext) {
    EndPopup(g);
}

// Call directly after the BeginCombo/EndCombo block. The preview is designed to only host non-interactive elements
// (Experimental, see GitHub issues: #1658, #4168)
pub unsafe fn BeginComboPreview(g: &mut ImguiContext) -> bool {
    let mut window_id = g.CurrentWindow;
    let mut preview_data = &mut g.ComboPreviewData;
    let window_opt = g.window_by_id_mut(window_id);
    let window = window_opt.unwrap();

    if window.skip_items || !window.ClipRect.Overlaps(g.last_item_data.rect) {
        // FIXME: Because we don't have a ImGuiItemStatusFlags_Visible flag to test last ItemAdd() result
        return false;
    }

    if !window.ClipRect.Contains(preview_data.PreviewRect) {}

    // FIXME: This could be contained in a PushWorkRect() api
    preview_data.BackupCursorPos = window.dc.cursor_pos;
    preview_data.BackupCursorMaxPos = window.dc.CursorMaxPos;
    preview_data.BackupCursorPosPrevLine = window.dc.cursor_pos_prev_line;
    preview_data.BackupPrevLineTextBaseOffset = window.dc.prev_line_text_base_offset;
    preview_data.BackupLayout = window.dc.LayoutType;
    window.dc.cursor_pos = preview_data.PreviewRect.min + g.style.FramePadding;
    window.dc.CursorMaxPos = window.dc.cursor_pos;
    window.dc.LayoutType = ImGuiLayoutType_Horizontal;
    window.dc.is_same_line = false;
    PushClipRect(
        g,
        preview_Data.PreviewRect.Min,
        preview_Data.PreviewRect.Max,
        true,
    );

    return true;
}

pub fn EndComboPreview(g: &mut ImguiContext) {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    let preview_data = &mut g.ComboPreviewData;

    // FIXME: Using CursorMaxPos approximation instead of correct AABB which we will store in ImDrawCmd in the future
    let draw_list = &mut window.DrawList;
    if window.dc.CursorMaxPos.x < preview_Data.PreviewRect.Max.x
        && window.dc.CursorMaxPos.y < preview_Data.PreviewRect.Max.y
    {
        if draw_list.CmdBuffer.len() > 1 {
            // Unlikely case that the PushClipRect() didn't create a command {
            draw_list.CmdBuffer[draw_list.CmdBuffer.len() - 1].ClipRect =
                draw_list.CmdBuffer[draw_list.CmdBuffer.len() - 2].ClipRect;
            draw_list._CmdHeader.ClipRect =
                draw_list.CmdBuffer[draw_list.CmdBuffer.len() - 1].ClipRect;
        }
    }
    draw_list._TryMergeDrawCmds();
    PopClipRect(g);
    window.dc.cursor_pos = preview_Data.BackupCursorPos;
    window.dc.CursorMaxPos = ImMax(window.dc.CursorMaxPos, preview_Data.BackupCursorMaxPos);
    window.dc.cursor_pos_prev_line = preview_Data.BackupCursorPosPrevLine;
    window.dc.prev_line_text_base_offset = preview_Data.BackupPrevLineTextBaseOffset;
    window.dc.LayoutType = preview_Data.BackupLayout;
    window.dc.is_same_line = false;
    preview_Data.PreviewRect = ImRect::default();
}

// Getter for the old Combo() API: const char*[]
pub unsafe fn Items_ArrayGetter(data: &[String], idx: usize, out_text: &mut String) -> bool {
    let items = data;
    // if (out_text) {
    //     *out_text = items[idx];
    // }
    *out_text = items[idx].clone();
    return true;
}

// Getter for the old Combo() API: "item1\0item2\0item3\0"
pub unsafe fn Items_SingleStringGetter(data: &[String], idx: usize, out_text: &mut String) -> bool {
    // FIXME-OPT: we could pre-compute the indices to fasten this. But only 1 active combo means the waste is limited.
    // let mut  items_separated_by_zeros: &str =data;
    // let mut items_count: c_int = 0;
    // let mut  p: &str = items_separated_by_zeros;
    while *p {
        if idx == items_count {
            break ();
        }
        p += strlen(p) + 1;
        items_count += 1;
    }
    if !*p {
        return false;
    }
    if out_text {
        *out_text = p;
    }
    return true;
}

// Old API, prefer using BeginCombo() nowadays if you can.
pub fn Combo(
    g: &mut ImguiContext,
    label: &String,
    current_item: &mut i32,
    items_getter: fn(&[String], usize, &mut String) -> bool,
    data: &[String],
    items_count: i32,
    popup_max_height_in_items: c_int,
) -> bool {
    // Call the getter to obtain the preview string which is a parameter to BeginCombo()
    let mut preview_value: String = String::default();
    if *current_item >= 0 && *current_item < items_count {
        items_getter(data, *current_item as usize, &mut preview_value);
    }

    // The old Combo() API exposed "popup_max_height_in_items". The new more general BeginCombo() API doesn't have/need it, but we emulate it here.
    if popup_max_height_in_items != -1
        && flag_clear(
            g.NextWindowData.Flags,
            ImGuiNextWindowDataFlags_HasSizeConstraint,
        )
    {
        SetNextWindowSizeConstraints(
            g,
            &ImVec2::from_floats(0.0, 0.0),
            &ImVec2::from_floats(
                f32::MAX,
                popup_ops::CalcMaxPopupHeightFromItemCount(g, popup_max_height_in_items),
            ),
            (),
            None,
        );
    }

    if !BeginCombo(g, label, &mut preview_value, ImGuiComboFlags_None) {
        return false;
    }

    // Display items
    // FIXME-OPT: Use clipper (but we need to disable it on the appearing frame to make sure our call to SetItemDefaultFocus() is processed)
    let mut value_changed: bool = false;
    // for (let i: c_int = 0; i < items_count; i++)
    for i in 0..items_count {
        PushID(i);
        let item_selected: bool = (i == *current_item);
        let mut item_text = String::default();
        if !items_getter(data, i as usize, &mut item_text) {
            item_text = String::from("*Unknown item*");
        }
        if Selectable(item_text, item_selected, 0, None) {
            value_changed = true;
            *current_item = i;
        }
        if item_selected {
            SetItemDefaultFocus(g);
        }
        pop_win_id_from_stack(g);
    }

    EndCombo(g);

    if value_changed {
        MarkItemEdited(g, g.last_item_data.id);
    }

    return value_changed;
}

// Combo box helper allowing to pass an array of strings.
pub unsafe fn Combo2(
    label: &String,
    current_item: &mut i32,
    items: &[String],
    items_count: usize,
    height_in_items: c_int,
) -> bool {
    let value_changed: bool = Combo(
        g,
        label,
        current_item,
        Items_ArrayGetter,
        items,
        items_count as i32,
        height_in_items,
    );
    return value_changed;
}

// Combo box helper allowing to pass all items in a single string literal holding multiple zero-terminated items "item1\0item2\0"
pub unsafe fn Combo3(
    label: &String,
    current_item: &mut i32,
    items_separated_by_zeros: &[String],
    height_in_items: c_int,
) -> bool {
    let mut items_count = items_separated_by_zeros.len();
    // let mut  p: &str = items_separated_by_zeros;       // FIXME-OPT: Avoid computing this, or at least only when combo is open
    // while (*p)
    // {
    //     p += strlen(p) + 1;
    //     items_count+= 1;
    // }
    let mut value_changed: bool = Combo(
        g,
        label,
        current_item,
        Items_SingleStringGetter,
        items_separated_by_zeros,
        items_count as i32,
        height_in_items,
    );
    return value_changed;
}
