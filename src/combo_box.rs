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
use crate::direction::{ImGuiDir_Down, ImGuiDir_Left};
use crate::draw_flags::{
    ImDrawFlags_RoundCornersAll, ImDrawFlags_RoundCornersLeft, ImDrawFlags_RoundCornersRight,
};
use crate::draw_list::ImDrawList;
use crate::frame_ops::GetFrameHeight;
use crate::id_ops::PopID;
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
use crate::type_defs::ImGuiID;
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
use crate::window::ImGuiWindow;
use crate::{button_ops, popup_ops, GImGui, ImHashStr};
use libc::{c_char, c_float, c_int, strlen};
use std::ptr::{null, null_mut};

pub unsafe fn BeginCombo(label: String, preview_value: &mut String, flags: ImGuiComboFlags) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = GetCurrentWindow();

    let backup_next_window_data_flags: ImGuiNextWindowDataFlags = g.NextWindowData.Flags;
    g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
    if window.SkipItems {
        return false;
    }

    let setyle = &mut g.Style;
    let mut id: ImGuiID = window.id_from_str(label);
    // IM_ASSERT((flags & (ImGuiComboFlags_NoArrowButton | ImGuiComboFlags_NoPreview)) != (ImGuiComboFlags_NoArrowButton | ImGuiComboFlags_NoPreview)); // Can't use both flags together

    let arrow_size: c_float = if flag_set(flags, ImGuiComboFlags_NoArrowButton) {
        0.0
    } else {
        GetFrameHeight()
    };
    let label_size: ImVec2 = CalcTextSize(label, true, 0.0);
    let w: c_float = if flag_set(flags, ImGuiComboFlags_NoPreview) {
        arrow_size
    } else {
        CalcItemWidth()
    };
    let mut bb: ImRect = ImRect::new(
        window.DC.CursorPos,
        window.DC.CursorPos + ImVec2::from_floats(w, label_size.y + style.FramePadding.y * 2.0),
    );
    let mut total_bb: ImRect = ImRect::new(
        bb.Min,
        bb.Max
            + ImVec2::from_floats(
                if label_size.x > 0.0 {
                    style.ItemInnerSpacing.x + label_size.x
                } else {
                    0.0
                },
                0.0,
            ),
    );
    ItemSize(&total_bb.GetSize(), style.FramePadding.y);
    if !ItemAdd(&mut total_bb, id, Some(&bb), 0) {
        return false;
    }

    // Open on click
    let mut hovered = false;
    let mut held = false;
    let mut pressed: bool = button_ops::ButtonBehavior(&bb, id, &mut hovered, &mut held, 0);
    let mut popup_id: ImGuiID = ImHashStr("##ComboPopup", 0, id as u32);
    let mut popup_open: bool = IsPopupOpen(popup_id, ImGuiPopupFlags_None);
    if pressed && !popup_open {
        OpenPopupEx(popup_id, ImGuiPopupFlags_None);
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
    let value_x2: c_float = ImMax(bb.Min.x, bb.Max.x - arrow_size);
    RenderNavHighlight(&bb, id, 0);
    if flag_clear(flags, ImGuiComboFlags_NoPreview) {
        window.DrawList.AddRectFilled(
            &bb.Min,
            &ImVec2::from_floats(value_x2, bb.Max.y),
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
            &ImVec2::from_floats(value_x2, bb.Min.y),
            &bb.Max,
            bg_col,
            style.FrameRounding,
            if w <= arrow_size {
                ImDrawFlags_RoundCornersAll
            } else {
                ImDrawFlags_RoundCornersRight
            },
        );
        if value_x2 + arrow_size - style.FramePadding.x <= bb.Max.x {
            RenderArrow(
                &mut window.DrawList,
                &ImVec2::from_floats(
                    value_x2 + style.FramePadding.y,
                    bb.Min.y + style.FramePadding.y,
                ),
                text_col,
                ImGuiDir_Down,
                1.0,
            );
        }
    }
    RenderFrameBorder(bb.Min, bb.Max, style.FrameRounding);

    // Custom preview
    if flags & ImGuiComboFlags_CustomPreview {
        g.ComboPreviewData.PreviewRect = ImRect(bb.Min.x, bb.Min.y, value_x2, bb.Max.y);
        // IM_ASSERT(preview_value == NULL || preview_value[0] == 0);
        preview_value.clear();
    }

    // Render preview and label
    if preview_value != None && flag_clear(flags, ImGuiComboFlags_NoPreview) {
        if g.LogEnabled {
            LogSetNextTextDecoration("{", "}");
        }
        RenderTextClipped(
            bb.Min + style.FramePadding,
            &ImVec2::from_floats(value_x2, bb.Max.y),
            preview_value,
            None,
            None,
            None,
        );
    }
    if label_size.x > 0.0 {
        RenderText(
            ImVec2::from_floats(
                bb.Max.x + style.ItemInnerSpacing.x,
                bb.Min.y + style.FramePadding.y,
            ),
            label,
            false,
        );
    }

    if !popup_open {
        return false;
    }

    g.NextWindowData.Flags = backup_next_window_data_flags;
    return BeginComboPopup(popup_id, &mut bb, flags);
}

pub unsafe fn BeginComboPopup(
    popup_id: ImGuiID,
    bb: &mut ImRect,
    mut flags: ImGuiComboFlags,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if !IsPopupOpen(popup_id, ImGuiPopupFlags_None) {
        g.NextWindowData.ClearFlags();
        return false;
    }

    // Set popup size
    let w: c_float = bb.GetWidth();
    if flag_set(
        g.NextWindowData.Flags,
        ImGuiNextWindowDataFlags_HasSizeConstraint,
    ) {
        g.NextWindowData.SizeConstraintRect.Min.x =
            ImMax(g.NextWindowData.SizeConstraintRect.Min.x, w);
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
            &ImVec2::from_floats(w, 0.0),
            &ImVec2::from_floats(
                f32::MAX,
                popup_ops::CalcMaxPopupHeightFromItemCount(popup_max_height_in_items),
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
    if popup_window: &mut ImGuiWindow = FindWindowByName(name) {
        if popup_window.WasActive {
            // Always override 'AutoPosLastDirection' to not leave a chance for a past value to affect us.
            let size_expected: ImVec2 = CalcWindowNextAutoFitSize(popup_window);
            popup_window.AutoPosLastDirection = if flags & ImGuiComboFlags_PopupAlignLeft {
                ImGuiDir_Left
            } else {
                ImGuiDir_Down
            }; // Left = "Below, Toward Left", Down = "Below, Toward Right (default)"
            let mut r_outer: ImRect = GetPopupAllowedExtentRect(popup_window);
            let pos: ImVec2 = FindBestWindowPosForPopupEx(
                &bb.GetBL(),
                &size_expected,
                &mut popup_window.AutoPosLastDirection,
                &mut r_outer,
                bb,
                ImGuiPopupPositionPolicy_ComboBox,
            );
            SetNextWindowPos(&pos, 0, &Default::default());
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
        ImVec2::from_floats(g.Style.FramePadding.x, g.Style.WindowPadding.y),
    ); // Horizontally align ourselves with the framed text
    let mut ret: bool = Begin(name, None);
    PopStyleVar();
    if !ret {
        EndPopup();
        // IM_ASSERT(0);   // This should never happen as we tested for IsPopupOpen() above
        return false;
    }
    return true;
}

pub unsafe fn EndCombo() {
    EndPopup();
}

// Call directly after the BeginCombo/EndCombo block. The preview is designed to only host non-interactive elements
// (Experimental, see GitHub issues: #1658, #4168)
pub unsafe fn BeginComboPreview() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: &mut ImGuiWindow = g.CurrentWindow;
    preview_data: *mut ImGuiComboPreviewData = &mut g.ComboPreviewData;

    if window.SkipItems || !window.ClipRect.Overlaps(g.LastItemData.Rect) {
        // FIXME: Because we don't have a ImGuiItemStatusFlags_Visible flag to test last ItemAdd() result
        return false;
    }
    // IM_ASSERT(g.LastItemData.Rect.Min.x == preview_Data.PreviewRect.Min.x && g.LastItemData.Rect.Min.y == preview_Data.PreviewRect.Min.y); // Didn't call after BeginCombo/EndCombo block or forgot to pass ImGuiComboFlags_CustomPreview flag?
    if !window.ClipRect.Contains(preview_data.PreviewRect) {} // Narrower test (optional { return  false; }

    // FIXME: This could be contained in a PushWorkRect() api
    preview_data.BackupCursorPos = window.DC.CursorPos;
    preview_data.BackupCursorMaxPos = window.DC.CursorMaxPos;
    preview_data.BackupCursorPosPrevLine = window.DC.CursorPosPrevLine;
    preview_data.BackupPrevLineTextBaseOffset = window.DC.PrevLineTextBaseOffset;
    preview_data.BackupLayout = window.DC.LayoutType;
    window.DC.CursorPos = preview_data.PreviewRect.Min + g.Style.FramePadding;
    window.DC.CursorMaxPos = window.DC.CursorPos;
    window.DC.LayoutType = ImGuiLayoutType_Horizontal;
    window.DC.IsSameLine = false;
    PushClipRect(
        preview_Data.PreviewRect.Min,
        preview_Data.PreviewRect.Max,
        true,
    );

    return true;
}

pub unsafe fn EndComboPreview() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let preview_data: *mut ImGuiComboPreviewData = &mut g.ComboPreviewData;

    // FIXME: Using CursorMaxPos approximation instead of correct AABB which we will store in ImDrawCmd in the future
    let draw_list = &mut window.DrawList;
    if window.DC.CursorMaxPos.x < preview_Data.PreviewRect.Max.x
        && window.DC.CursorMaxPos.y < preview_Data.PreviewRect.Max.y
    {
        if draw_list.CmdBuffer.len() > 1 {
            // Unlikely case that the PushClipRect() didn't create a command {
            draw_list.CmdBuffer[draw_list.CmdBuffer.len() - 1]
                .ClipRect =
                draw_list.CmdBuffer[draw_list.CmdBuffer.len() - 2].ClipRect;
            draw_list._CmdHeader.ClipRect = draw_list.CmdBuffer[draw_list.CmdBuffer.len() - 1]
                .ClipRect;
        }
    }
    draw_list._TryMergeDrawCmds();
    PopClipRect();
    window.DC.CursorPos = preview_Data.BackupCursorPos;
    window.DC.CursorMaxPos = ImMax(window.DC.CursorMaxPos, preview_Data.BackupCursorMaxPos);
    window.DC.CursorPosPrevLine = preview_Data.BackupCursorPosPrevLine;
    window.DC.PrevLineTextBaseOffset = preview_Data.BackupPrevLineTextBaseOffset;
    window.DC.LayoutType = preview_Data.BackupLayout;
    window.DC.IsSameLine = false;
    preview_Data.PreviewRect = ImRect();
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
pub unsafe fn Combo(
    label: String,
    current_item: &mut i32,
    items_getter: fn(&[String], usize, &mut String) -> bool,
    data: &[String],
    items_count: usize,
    popup_max_height_in_items: c_int,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Call the getter to obtain the preview string which is a parameter to BeginCombo()
    let mut preview_value: String = String::default();
    if *current_item >= 0 && *current_item < items_count {
        items_getter(data, *current_item, &mut preview_value);
    }

    // The old Combo() API exposed "popup_max_height_in_items". The new more general BeginCombo() API doesn't have/need it, but we emulate it here.
    if popup_max_height_in_items != -1
        && flag_clear(
            g.NextWindowData.Flags,
            ImGuiNextWindowDataFlags_HasSizeConstraint,
        )
    {
        SetNextWindowSizeConstraints(
            &ImVec2::from_floats(0.0, 0.0),
            &ImVec2::from_floats(
                f32::MAX,
                popup_ops::CalcMaxPopupHeightFromItemCount(popup_max_height_in_items),
            ),
            (),
            None,
        );
    }

    if !BeginCombo(label, &mut preview_value, ImGuiComboFlags_None) {
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
        if !items_getter(data, i, &mut item_text) {
            item_text = String::from("*Unknown item*");
        }
        if Selectable(item_text.as_str(), item_selected, 0, None) {
            value_changed = true;
            *current_item = i;
        }
        if item_selected {
            SetItemDefaultFocus();
        }
        PopID();
    }

    EndCombo();

    if value_changed {
        MarkItemEdited(g.LastItemData.ID);
    }

    return value_changed;
}

// Combo box helper allowing to pass an array of strings.
pub unsafe fn Combo2(
    label: String,
    current_item: &mut i32,
    items: &[String],
    items_count: usize,
    height_in_items: c_int,
) -> bool {
    let value_changed: bool = Combo(
        label,
        current_item,
        Items_ArrayGetter,
        items,
        items_count,
        height_in_items,
    );
    return value_changed;
}

// Combo box helper allowing to pass all items in a single string literal holding multiple zero-terminated items "item1\0item2\0"
pub unsafe fn Combo3(
    label: String,
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
        label,
        current_item,
        Items_SingleStringGetter,
        items_separated_by_zeros,
        items_count,
        height_in_items,
    );
    return value_changed;
}
