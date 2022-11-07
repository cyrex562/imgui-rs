use std::borrow::BorrowMut;
use crate::button_flags::{
    ImGuiButtonFlags, ImGuiButtonFlags_AlignTextBaseLine, ImGuiButtonFlags_AllowItemOverlap,
    ImGuiButtonFlags_FlattenChildren, ImGuiButtonFlags_MouseButtonDefault_,
    ImGuiButtonFlags_MouseButtonLeft, ImGuiButtonFlags_MouseButtonMask_,
    ImGuiButtonFlags_MouseButtonMiddle, ImGuiButtonFlags_MouseButtonRight,
    ImGuiButtonFlags_NoHoldingActiveId, ImGuiButtonFlags_NoHoveredOnFocus,
    ImGuiButtonFlags_NoKeyModifiers, ImGuiButtonFlags_NoNavFocus, ImGuiButtonFlags_None,
    ImGuiButtonFlags_PressedOnClick, ImGuiButtonFlags_PressedOnClickRelease,
    ImGuiButtonFlags_PressedOnClickReleaseAnywhere, ImGuiButtonFlags_PressedOnDefault_,
    ImGuiButtonFlags_PressedOnDoubleClick, ImGuiButtonFlags_PressedOnDragDropHold,
    ImGuiButtonFlags_PressedOnMask_, ImGuiButtonFlags_PressedOnRelease, ImGuiButtonFlags_Repeat,
};
use crate::color::{ImGuiCol_Button, ImGuiCol_ButtonActive, ImGuiCol_ButtonHovered, ImGuiCol_Text};
use crate::direction::{ImGuiDir, ImGuiDir_Down, ImGuiDir_Right};
use crate::dock_node::ImGuiDockNode;
use crate::drag_drop_flags::{
    ImGuiDragDropFlags_SourceNoDisableHover, ImGuiDragDropFlags_SourceNoHoldToOpenOthers,
};
use crate::frame_ops::GetFrameHeight;
use crate::hovered_flags::ImGuiHoveredFlags_AllowWhenBlockedByActiveItem;
use crate::id_ops::{ClearActiveID, SetActiveID, SetHoveredID};
use crate::input_ops::{CalcTypematicRepeatAmount, GetKeyData, IsMouseClicked, IsMouseDragging};
use crate::input_source::{ImGuiInputSource_Mouse, ImGuiInputSource_Nav};
use crate::item_flags::ImGuiItemFlags_ButtonRepeat;
use crate::item_ops::{
    CalcItemSize, IsItemActive, IsItemHovered, ItemAdd, ItemHoverable, ItemSize,
};
use crate::key::{ImGuiKey_NavGamepadActivate, ImGuiKey_Space};
use crate::logging_ops::LogSetNextTextDecoration;
use crate::math_ops::ImMax;
use crate::mouse_ops::StartMouseMovingWindowOrNode;
use crate::nav_ops::SetFocusID;
use crate::rect::ImRect;
use crate::render_ops::{
    RenderArrow, RenderArrowDockMenu, RenderFrame, RenderNavHighlight, RenderTextClipped,
};
use crate::style_ops::GetColorU32;
use crate::text_ops::CalcTextSize;
use crate::type_defs::ImguiHandle;
use crate::utils::{flag_clear, flag_set};
use crate::vec2::ImVec2;
use crate::widgets::DRAGDROP_HOLD_TO_OPEN_TIMER;
use crate::window::focus::FocusWindow;
use crate::window::ops::GetCurrentWindow;
use crate::window::ImguiWindow;
use crate::GImGui;
use libc::{c_float, c_int};
use std::ptr::null;
use crate::context::ImguiContext;

pub fn ButtonBehavior(
    g: &mut ImguiContext,
    bb: &ImRect,
    id: ImguiHandle,
    out_hovered: *mut bool,
    out_held: *mut bool,
    mut flags: ImGuiButtonFlags,
) -> bool {
    let mut window = g.current_window_mut().unwrap();

    // Default only reacts to left mouse button
    if flag_clear(flags, ImGuiButtonFlags_MouseButtonMask_) {
        flags |= ImGuiButtonFlags_MouseButtonDefault_;
    }

    // Default behavior requires click + release inside bounding box
    if flag_clear(flags, ImGuiButtonFlags_PressedOnMask_) {
        flags |= ImGuiButtonFlags_PressedOnDefault_;
    }

    let mut backup_hovered_window = &g.HoveredWindow;
    let flatten_hovered_children: bool = flag_set(flags, ImGuiButtonFlags_FlattenChildren)
        && g.HoveredWindow.is_null() == false
        && g.Hoveredwindow.RootWindowDockTree == window.RootWindowDockTree;
    if flatten_hovered_children {
        g.HoveredWindow = Some(window.clone());
    }

    // #ifdef IMGUI_ENABLE_TEST_ENGINE
    if id != 0 && g.last_item_data.id != id {
        IMGUI_TEST_ENGINE_ITEM_ADD(bb, id);
    }
    // #endif

    let mut pressed: bool = false;
    let mut hovered: bool = ItemHoverable(bb, id);

    // Drag source doesn't report as hovered
    if hovered
        && g.DragDropActive
        && g.DragDropPayload.SourceId == id
        && flag_clear(
            g.DragDropSourceFlags,
            ImGuiDragDropFlags_SourceNoDisableHover,
        )
    {
        hovered = false;
    }

    // Special mode for Drag and Drop where holding button pressed for a long time while dragging another item triggers the button
    if g.DragDropActive
        && flag_set(flags, ImGuiButtonFlags_PressedOnDragDropHold)
        && flag_clear(
            g.DragDropSourceFlags,
            ImGuiDragDropFlags_SourceNoHoldToOpenOthers,
        )
    {
        if IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByActiveItem) {
            hovered = true;
            SetHoveredID(id);
            if g.HoveredIdTimer - g.IO.DeltaTime <= DRAGDROP_HOLD_TO_OPEN_TIMER
                && g.HoveredIdTimer >= DRAGDROP_HOLD_TO_OPEN_TIMER
            {
                pressed = true;
                g.DragDropHoldJustPressedId = id;
                FocusWindow(&mut window);
            }
        }
    }

    if flatten_hovered_children {
        g.HoveredWindow.replace(backup_hovered_window.unwrap().clone());
    }

    // AllowOverlap mode (rarely used) requires previous frame HoveredId to be null or to match. This allows using patterns where a later submitted widget overlaps a previous one.
    if hovered
        && flag_set(flags, ImGuiButtonFlags_AllowItemOverlap)
        && (g.HoveredIdPreviousFrame != id && g.HoveredIdPreviousFrame != 0)
    {
        hovered = false;
    }

    // Mouse handling
    if (hovered) {
        if (flag_clear(flags, ImGuiButtonFlags_NoKeyModifiers)
            || (!g.IO.KeyCtrl && !g.IO.KeyShift && !g.IO.KeyAlt))
        {
            // Poll buttons
            let mut mouse_button_clicked: c_int = -1;
            if (flag_set(flags, ImGuiButtonFlags_MouseButtonLeft) && g.IO.MouseClicked[0]) {
                mouse_button_clicked = 0;
            } else if (flag_set(flags, ImGuiButtonFlags_MouseButtonRight) && g.IO.MouseClicked[1]) {
                mouse_button_clicked = 1;
            } else if (flag_set(flags, ImGuiButtonFlags_MouseButtonMiddle) && g.IO.MouseClicked[2])
            {
                mouse_button_clicked = 2;
            }

            if (mouse_button_clicked != -1 && g.ActiveId != id) {
                if (flags
                    & (ImGuiButtonFlags_PressedOnClickRelease
                        | ImGuiButtonFlags_PressedOnClickReleaseAnywhere))
                {
                    SetActiveID(g, id, &mut window);
                    g.ActiveIdMouseButton = mouse_button_clicked;
                    if (flag_clear(flags, ImGuiButtonFlags_NoNavFocus)) {
                        SetFocusID(id, &mut window);
                    }
                    FocusWindow(&mut window);
                }
                if (flag_set(flags, ImGuiButtonFlags_PressedOnClick)
                    || (flag_set(flags, ImGuiButtonFlags_PressedOnDoubleClick)
                        && g.IO.MouseClickedCount[mouse_button_clicked] == 2))
                {
                    pressed = true;
                    if flags & ImGuiButtonFlags_NoHoldingActiveId {
                        ClearActiveID(g);
                    } else {
                        SetActiveID(g, id, &mut window);
                    } // Hold on ID
                    if (flag_clear(flags, ImGuiButtonFlags_NoNavFocus)) {
                        SetFocusID(id, &mut window);
                    }
                    g.ActiveIdMouseButton = mouse_button_clicked;
                    FocusWindow(&mut window);
                }
            }
            if (flags & ImGuiButtonFlags_PressedOnRelease) {
                let mut mouse_button_released: c_int = -1;
                if (flag_set(flags, ImGuiButtonFlags_MouseButtonLeft) && g.IO.MouseReleased[0]) {
                    mouse_button_released = 0;
                } else if (flag_set(flags, ImGuiButtonFlags_MouseButtonRight)
                    && g.IO.MouseReleased[1])
                {
                    mouse_button_released = 1;
                } else if (flag_set(flags, ImGuiButtonFlags_MouseButtonMiddle)
                    && g.IO.MouseReleased[2])
                {
                    mouse_button_released = 2;
                }
                if (mouse_button_released != -1) {
                    let has_repeated_at_least_once: bool = flag_set(flags, ImGuiButtonFlags_Repeat)
                        && g.IO.MouseDownDurationPrev[mouse_button_released] >= g.IO.KeyRepeatDelay; // Repeat mode trumps on release behavior
                    if !has_repeated_at_least_once {
                        pressed = true;
                    }
                    if (flag_clear(flags, ImGuiButtonFlags_NoNavFocus)) {
                        SetFocusID(id, &mut window);
                    }
                    ClearActiveID(g);
                }
            }

            // 'Repeat' mode acts when held regardless of _PressedOn flags (see table above).
            // Relies on repeat logic of IsMouseClicked() but we may as well do it ourselves if we end up exposing finer RepeatDelay/RepeatRate settings.
            if g.ActiveId == id && flag_set(flags, ImGuiButtonFlags_Repeat) {
                if g.IO.MouseDownDuration[g.ActiveIdMouseButton] > 0.0
                    && IsMouseClicked(g.ActiveIdMouseButton, true)
                {
                    pressed = true;
                }
            }
        }

        if pressed {
            g.NavDisableHighlight = true;
        }
    }

    // Gamepad/Keyboard navigation
    // We report navigated item as hovered but we don't set g.HoveredId to not interfere with mouse.
    if g.NavId == id
        && !g.NavDisableHighlight
        && g.NavDisableMouseHover
        && (g.ActiveId == 0 || g.ActiveId == id || g.ActiveId == window.MoveId)
    {
        if flag_clear(flags, ImGuiButtonFlags_NoHoveredOnFocus) {
            hovered = true;
        }
    }
    if g.NavActivateDownId == id {
        let mut nav_activated_by_code: bool = (g.NavActivateId == id);
        let mut nav_activated_by_inputs: bool = (g.NavActivatePressedId == id);
        if !nav_activated_by_inputs && flag_set(flags, ImGuiButtonFlags_Repeat) {
            // Avoid pressing both keys from triggering double amount of repeat events
            let key1 = GetKeyData(ImGuiKey_Space);
            let key2 = GetKeyData(ImGuiKey_NavGamepadActivate);
            let t1 = ImMax(key1.DownDuration, key2.DownDuration);
            nav_activated_by_inputs = CalcTypematicRepeatAmount(
                t1 - g.IO.DeltaTime,
                t1,
                g.IO.KeyRepeatDelay,
                g.IO.KeyRepeatRate,
            ) > 0;
        }
        if nav_activated_by_code || nav_activated_by_inputs {
            // Set active id so it can be queried by user via IsItemActive(), equivalent of holding the mouse button.
            pressed = true;
            SetActiveID(g, id, &mut window);
            g.ActiveIdSource = ImGuiInputSource_Nav;
            if flag_c {
                SetFocusID(id, &mut window);
                lear(flags, ImGuiButtonFlags_NoNavFocus);
            }
        }
    }

    // Process while held
    let mut held: bool = false;
    if (g.ActiveId == id) {
        if (g.ActiveIdSource == ImGuiInputSource_Mouse) {
            if (g.ActiveIdIsJustActivated) {
                g.ActiveIdClickOffset = g.IO.MousePos - bb.min;
            }

            let mouse_button: c_int = g.ActiveIdMouseButton;
            // IM_ASSERT(mouse_button >= 0 && mouse_button < ImGuiMouseButton_COUNT);
            if (g.IO.MouseDown[mouse_button]) {
                held = true;
            } else {
                let mut release_in: bool =
                    hovered && flag_set(flags, ImGuiButtonFlags_PressedOnClickRelease);
                let mut release_anywhere: bool =
                    flag_set(flags, ImGuiButtonFlags_PressedOnClickReleaseAnywhere);
                if ((release_in || release_anywhere) && !g.DragDropActive) {
                    // Report as pressed when releasing the mouse (this is the most common path)
                    let mut is_double_click_release: bool =
                        flag_set(flags, ImGuiButtonFlags_PressedOnDoubleClick)
                            && g.IO.MouseReleased[mouse_button]
                            && g.IO.MouseClickedLastCount[mouse_button] == 2;
                    let mut is_repeating_already: bool = flag_set(flags, ImGuiButtonFlags_Repeat)
                        && g.IO.MouseDownDurationPrev[mouse_button] >= g.IO.KeyRepeatDelay; // Repeat mode trumps <on release>
                    if !is_double_click_release && !is_repeating_already {
                        pressed = true;
                    }
                }
                ClearActiveID(g);
            }
            if (flag_clear(flags, ImGuiButtonFlags_NoNavFocus)) {
                g.NavDisableHighlight = true;
            }
        } else if (g.ActiveIdSource == ImGuiInputSource_Nav) {
            // When activated using Nav, we hold on the ActiveID until activation button is released
            if (g.NavActivateDownId != id) {
                ClearActiveID(g);
            }
        }
        if pressed {
            g.ActiveIdHasBeenPressedBefore = true;
        }
    }

    if (out_hovered) {
        *out_hovered = hovered;
    }
    if (out_held) {
        *out_held = held;
    }

    return pressed;
}

pub unsafe fn ButtonEx(label: &String, size_arg: Option<ImVec2>, mut flags: ImGuiButtonFlags) -> bool {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return false;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let setyle = &mut g.style;
    let mut id: ImguiHandle = window.id_from_str(&label, );
    let label_size: ImVec2 = CalcTextSize(, &label, true, 0.0);

    let mut pos: ImVec2 = window.dc.cursor_pos;
    if flag_set(flags, ImGuiButtonFlags_AlignTextBaseLine)
        && style.FramePadding.y < window.dc.CurrLineTextBaseOffset
    {
        // Try to vertically align buttons that are smaller/have no padding so that text baseline matches (bit hacky, since it shouldn't be a flag)
        pos.y += window.dc.CurrLineTextBaseOffset - style.FramePadding.y;
    }
    let size = CalcItemSize(
        g,
        size_arg.unwrap(),
        label_size.x + style.FramePadding.x * 2.0,
        label_size.y + style.FramePadding.y * 2.0,
    );

    let mut bb: ImRect = ImRect::new(pos, pos + size);
    ItemSize(g, &size, style.FramePadding.y);
    if !ItemAdd(g, &mut bb, id, None, 0) {
        return false;
    }

    if flag_set(g.last_item_data.in_flags, ImGuiItemFlags_ButtonRepeat) {
        flags |= ImGuiButtonFlags_Repeat;
    }

    // hovered: bool, held;
    let mut hovered = false;
    let mut held = false;
    let mut pressed = ButtonBehavior(g, &bb, id, &mut hovered, &mut held, flags);

    // Render
    col: u32 = GetColorU32(
        if (held && hovered) {
            ImGuiCol_ButtonActive
        } else {
            if hovered {
                ImGuiCol_ButtonHovered
            } else {
                ImGuiCol_Button
            }
        },
        0.0,
    );
    RenderNavHighlight(, &bb, id, 0);
    RenderFrame(bb.min, bb.max, col, true, style.FrameRounding);

    if g.LogEnabled {
        LogSetNextTextDecoration("[", "]");
    }
    RenderTextClipped(
        bb.min + style.FramePadding,
        bb.max - style.FramePadding,
        &label,
        Some(label_size.clone()),
        style.ButtonTextAlign,
        Some(bb.clone()),
    );

    // Automatically close popups
    //if (pressed && flag_clear(flags, ImGuiButtonFlags_DontClosePopups) && (window.Flags & ImGuiWindowFlags_Popup))
    //    CloseCurrentPopup();

    IMGUI_TEST_ENGINE_ITEM_INFO(id, &label, g.last_item_data.StatusFlags);
    return pressed;
}

pub unsafe fn Button(label: &String, size_arg: Option<ImVec2>) -> bool {
    return ButtonEx(label, size_arg, ImGuiButtonFlags_None);
}

// Small buttons fits within text without additional vertical spacing.
pub unsafe fn SmallButton(label: &String) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let backup_padding_y: c_float = g.style.FramePadding.y;
    g.style.FramePadding.y = 0.0;
    let mut pressed: bool = ButtonEx(
        label,
        Some(ImVec2::from_floats(0.0, 0.0)),
        ImGuiButtonFlags_AlignTextBaseLine,
    );
    g.style.FramePadding.y = backup_padding_y;
    return pressed;
}

// Tip: use PushID()/PopID() to push indices or pointers in the ID stack.
// Then you can keep 'str_id' empty or the same for all your buttons (instead of creating a string based on a non-string id)
pub unsafe fn InvisibleButton(
    str_id: &String,
    size_arg: ImVec2,
    flags: ImGuiButtonFlags,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return false;
    }

    // Cannot use zero-size for InvisibleButton(). Unlike Button() there is not way to fallback using the label size.
    // IM_ASSERT(size_arg.x != 0.0 && size_arg.y != 0.0);

    let mut id: ImguiHandle = window.id_from_str(str_id, );
    let size: ImVec2 = CalcItemSize(g, size_arg, 0.0, 0.0);
    let mut bb: ImRect = ImRect::new(window.dc.cursor_pos, window.dc.cursor_pos + size);
    ItemSize(g, &size, 0.0);
    if !ItemAdd(g, &mut bb, id, None, 0) {
        return false;
    }

    // hovered: bool, held;
    let mut hovered = false;
    let mut held = false;
    let mut pressed: bool = ButtonBehavior(g, &bb, id, &mut hovered, &mut held, flags);

    IMGUI_TEST_ENGINE_ITEM_INFO(id, str_id, g.last_item_data.StatusFlags);
    return pressed;
}

pub unsafe fn ArrowButtonEx(
    str_id: &String,
    dir: ImGuiDir,
    size: ImVec2,
    mut flags: ImGuiButtonFlags,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return false;
    }

    let mut id: ImguiHandle = window.id_from_str(str_id, );
    let mut bb: ImRect = ImRect::new(window.dc.cursor_pos, window.dc.cursor_pos + size);
    let default_size: c_float = GetFrameHeight();
    ItemSize(
        g,
        &size,
        if (size.y >= default_size) {
            g.style.FramePadding.y
        } else {
            -1.0
        },
    );
    if !ItemAdd(g, &mut bb, id, None, 0) {
        return false;
    }

    if flag_set(g.last_item_data.in_flags, ImGuiItemFlags_ButtonRepeat) {
        flags |= ImGuiButtonFlags_Repeat;
    }

    // hovered: bool, held;
    let mut hovered = false;
    let mut held = false;
    let mut pressed: bool = ButtonBehavior(g, &bb, id, &mut hovered, &mut held, flags);

    // Render
    bg_col: u32 = GetColorU32(
        if (held && hovered) {
            ImGuiCol_ButtonActive
        } else {
            if hovered {
                ImGuiCol_ButtonHovered
            } else {
                ImGuiCol_Button
            }
        },
        0.0,
    );
    text_col: u32 = GetColorU32(ImGuiCol_Text, 0.0);
    RenderNavHighlight(, &bb, id, 0);
    RenderFrame(bb.min, bb.max, bg_col, true, g.style.FrameRounding);
    RenderArrow(
        &mut window.DrawList,
        bb.min
            + ImVec2::from_floats(
                ImMax(0.0, (size.x - g.FontSize) * 0.5),
                ImMax(0.0, (size.y - g.FontSize) * 0.5),
            ),
        text_col,
        dir,
        0.0,
    );

    IMGUI_TEST_ENGINE_ITEM_INFO(id, str_id, g.last_item_data.StatusFlags);
    return pressed;
}

pub unsafe fn ArrowButton(str_id: &String, dir: ImGuiDir) -> bool {
    let sz: c_float = GetFrameHeight();
    return ArrowButtonEx(
        str_id,
        dir,
        ImVec2::from_floats(sz, sz),
        ImGuiButtonFlags_None,
    );
}

// Button to close a window
pub unsafe fn CloseButton(id: ImguiHandle, pos: &ImVec2) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();

    // Tweak 1: Shrink hit-testing area if button covers an abnormally large proportion of the visible region. That's in order to facilitate moving the window away. (#3825)
    // This may better be applied as a general hit-rect reduction mechanism for all widgets to ensure the area to move window is always accessible?
    let mut bb: ImRect = ImRect::new(
        pos,
        pos + ImVec2::from_floats(g.FontSize, g.FontSize) + g.style.FramePadding * 2.0,
    );
    let mut bb_interact: ImRect = bb;
    let area_to_visible_ratio: c_float = window.OuterRectClipped.GetArea() / bb.GetArea();
    if (area_to_visible_ratio < 1.5) {
        bb_interact.Expand(ImFloor(bb_interact.GetSize() * -0.25));
    }

    // Tweak 2: We intentionally allow interaction when clipped so that a mechanical Alt,Right,Activate sequence can always close a window.
    // (this isn't the regular behavior of buttons, but it doesn't affect the user much because navigation tends to keep items visible).
    let mut is_clipped: bool = !ItemAdd(g, &mut bb_interact, id, None, 0);

    // hovered: bool, held;
    let mut hovered = false;
    let mut held = false;
    let mut pressed: bool = ButtonBehavior(g, &bb_interact, id, &mut hovered, &mut held, 0);
    if is_clipped {
        return pressed;
    }

    // Render
    // FIXME: Clarify this mess
    col: u32 = GetColorU32(
        if held {
            ImGuiCol_ButtonActive
        } else {
            ImGuiCol_ButtonHovered
        },
        0.0,
    );
    let mut center: ImVec2 = bb.GetCenter();
    if (hovered) {
        window
            .DrawList
            .AddCircleFilled(&center, ImMax(2.0, g.FontSize * 0.5 + 1.0), col, 12);
    }

    let cross_extent: c_float = g.FontSize * 0.5 * 0.7071 - 1.0;
    cross_col: u32 = GetColorU32(ImGuiCol_Text, 0.0);
    center -= ImVec2::from_floats(0.5, 0.5);
    window.DrawList.AddLine(
        center + ImVec2::from_floats(cross_extent, cross_extent),
        center + ImVec2::from_floats(-cross_extent, -cross_extent),
        cross_col,
        1.0,
    );
    window.DrawList.AddLine(
        center + ImVec2::from_floats(cross_extent, -cross_extent),
        center + ImVec2::from_floats(-cross_extent, cross_extent),
        cross_col,
        1.0,
    );

    return pressed;
}

// The Collapse button also functions as a Dock Menu button.
pub unsafe fn CollapseButton(id: ImguiHandle, pos: &ImVec2, dock_node: *mut ImGuiDockNode) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();

    let mut bb: ImRect = ImRect::new(
        pos,
        pos + ImVec2::from_floats(g.FontSize, g.FontSize) + g.style.FramePadding * 2.0,
    );
    ItemAdd(g, &mut bb, id, None, 0);
    // hovered: bool, held;
    let mut hovered = false;
    let mut held = false;
    let mut pressed: bool = ButtonBehavior(g, &bb, id, &mut hovered, &mut held, ImGuiButtonFlags_None);

    // Render
    //is_dock_menu: bool = (window.DockNodeAsHost && !window.Collapsed);
    bg_col: u32 = GetColorU32(
        if (held && hovered) {
            ImGuiCol_ButtonActive
        } else {
            if hovered {
                ImGuiCol_ButtonHovered
            } else {
                ImGuiCol_Button
            }
        },
        0.0,
    );
    text_col: u32 = GetColorU32(ImGuiCol_Text, 0.0);
    if (hovered || held) {
        window.DrawList.AddCircleFilled(
            bb.GetCenter() + ImVec2::from_floats(0.0, -0.5),
            g.FontSize * 0.5 + 1.0,
            bg_col,
            12,
        );
    }

    if dock_node {
        RenderArrowDockMenu(
            window.DrawList,
            bb.min + g.style.FramePadding,
            g.FontSize,
            text_col,
        );
    } else {
        RenderArrow(
            window.DrawList,
            bb.min + g.style.FramePadding,
            text_col,
            if window.Collapsed {
                ImGuiDir_Right
            } else {
                ImGuiDir_Down
            },
            1.0,
        );
    }

    // Switch to moving the window after mouse is moved beyond the initial drag threshold
    if IsItemActive() && IsMouseDragging(0, 0.0) {
        StartMouseMovingWindowOrNode(window.unwrap().borrow_mut(), dock_node, true);
    }

    return pressed;
}
