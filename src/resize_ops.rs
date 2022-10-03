use std::ptr::null_mut;
use libc::{c_float, c_int};
use crate::direction::{ImGuiDir, ImGuiDir_Down, ImGuiDir_Left, ImGuiDir_Right, ImGuiDir_Up};
use crate::hash_ops::ImHashData;
use crate::{GImGui, ImHashStr};
use crate::axis::{ImGuiAxis_X, ImGuiAxis_Y};
use crate::color::{ImGuiCol_ResizeGrip, ImGuiCol_ResizeGripActive, ImGuiCol_ResizeGripHovered};
use crate::constants::{WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER};
use crate::id_ops::{ClearActiveID, KeepAliveID};
use crate::input_source::{ImGuiInputSource_Gamepad, ImGuiInputSource_Keyboard};
use crate::key::{ImGuiKey_DownArrow, ImGuiKey_GamepadDpadDown, ImGuiKey_GamepadDpadLeft, ImGuiKey_GamepadDpadRight, ImGuiKey_GamepadDpadUp, ImGuiKey_LeftArrow, ImGuiKey_RightArrow, ImGuiKey_UpArrow};
use crate::mouse_cursor::{ImGuiMouseCursor_ResizeEW, ImGuiMouseCursor_ResizeNESW, ImGuiMouseCursor_ResizeNS, ImGuiMouseCursor_ResizeNWSE};
use crate::nav_layer::{ImGuiNavLayer_Main, ImGuiNavLayer_Menu};
use crate::rect::ImRect;
use crate::resize_border_def::{ImGuiResizeBorderDef, resize_border_def};
use crate::resize_grip_def::resize_grip_def;
use crate::string_ops::str_to_const_c_char_ptr;
use crate::style_ops::GetColorU32;
use crate::type_defs::ImGuiID;
use crate::utils::flag_set;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;
use crate::window_flags::{ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_NoResize};
use crate::window_ops::CalcWindowSizeAfterConstraint;

// static c_void CalcResizePosSizeFromAnyCorner(window: *mut ImGuiWindow, const ImVec2& corner_target, const ImVec2& corner_norm, out_pos: *mut ImVec2, out_size: *mut ImVec2)
pub unsafe fn CalcResizePosSizeFromAnyCorner(window: *mut ImGuiWindow, corner: &ImVec2, corner_target: &ImVec2, corner_norm: &ImVec2, out_pos: *mut ImVec2, out_size: *mut ImVec2) {
    let pos_min: ImVec2 = ImLerp(corner_target, window.Pos, corner_norm);                // Expected window upper-left
    let pos_max: ImVec2 = ImLerp(window.Pos + window.Size, corner_target, corner_norm); // Expected window lower-right
    let size_expected: ImVec2 = pos_max - pos_min;
    let size_constrained: ImVec2 = CalcWindowSizeAfterConstraint(window, &size_expected);
    *out_pos = pos_min;
    if corner_norm.x == 0f32 {
        out_pos.x -= (size_constrained.x - size_expected.x);
    }
    if corner_norm.y == 0f32 {
        out_pos.y -= (size_constrained.y - size_expected.y);
    }
    *out_size = size_constrained;
}


// static ImRect GetResizeBorderRect(window: *mut ImGuiWindow, c_int border_n, c_float perp_padding, c_float thickness)
pub fn GetResizeBorderRect(window: *mut ImGuiWindow, border_n: c_int, perp_padding: c_float, thickness: c_float) -> ImRect
{
    let mut rect: ImRect =  window.Rect();
    if thickness == 0f32 {
        rect.Max -= ImVec2::new2(1, 1);
    }
    if border_n == ImGuiDir_Left { return ImRect(rect.Min.x - thickness, rect.Min.y + perp_padding, rect.Min.x + thickness, rect.Max.y - perp_padding); }
    if border_n == ImGuiDir_Right { return ImRect(rect.Max.x - thickness, rect.Min.y + perp_padding, rect.Max.x + thickness, rect.Max.y - perp_padding); }
    if border_n == ImGuiDir_Up { return ImRect(rect.Min.x + perp_padding, rect.Min.y - thickness, rect.Max.x - perp_padding, rect.Min.y + thickness);    }
    if border_n == ImGuiDir_Down { return ImRect(rect.Min.x + perp_padding, rect.Max.y - thickness, rect.Max.x - perp_padding, rect.Max.y + thickness);    }
    // IM_ASSERT(0);
    return ImRect();
}

// 0..3: corners (Lower-right, Lower-left, Unused, Unused)
// ImGuiID GetWindowResizeCornerID(window: *mut ImGuiWindow, c_int n)
pub unsafe fn GetWindowResizeCornerID(window: *mut ImGuiWindow, n: c_int) -> ImGuiID {
    // IM_ASSERT(n >= 0 && n < 4);
    let mut id: ImGuiID = if window.DockIsActive { window.DockNode.Hostwindow.ID } else { window.ID };
    id = ImHashStr(str_to_const_c_char_ptr("#RESIZE"), 0, id as u32);
    id = ImHashData(&n, sizeof, id as u32);
    return id;
}

// Borders (Left, Right, Up, Down)
// ImGuiID GetWindowResizeBorderID(window: *mut ImGuiWindow, dir: ImGuiDir)
pub unsafe fn GetWindowResizeBorderID(window: *mut ImGuiWindow, dir: ImGuiDir) -> ImGuiID
{
    // IM_ASSERT(dir >= 0 && dir < 4);
    let n: c_int = dir + 4;
    let mut id: ImGuiID =  if window.DockIsActive { window.DockNode.Hostwindow.ID } else { window.ID };
    id = ImHashStr(str_to_const_c_char_ptr("#RESIZE"), 0, id as u32);
    id = ImHashData(&n, sizeof, id as u32);
    return id;
}

// Handle resize for: Resize Grips, Borders, Gamepad
// Return true when using auto-fit (double click on resize grip)
// static bool UpdateWindowManualResize(window: *mut ImGuiWindow, const ImVec2& size_auto_fit, c_int* border_held, c_int resize_grip_count, u32 resize_grip_col[4], const ImRect& visibility_rect)
pub unsafe fn UpdateWindowManualResize(window: *mut ImGuiWindow, size_auto_fit: &ImVec2, border_held: *mut c_int, resize_grip_count: c_int, mut resize_grip_col:[u32;4], visibility_rect: &ImVec2) -> bool
{

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let flags = window.Flags;

    if flag_set(flags, ImGuiWindowFlags_NoResize) || flag_set(flags, ImGuiWindowFlags_AlwaysAutoResize) || window.AutoFitFramesX > 0 || window.AutoFitFramesY > 0 {
        return false;
    }
    if window.WasActive == false { // Early out to avoid running this code for e.g. an hidden implicit/fallback Debug window.
        return false;
    }

    let mut ret_auto_fit: bool =  false;
    let resize_border_count: c_int = if g.IO.ConfigWindowsResizeFromEdges { 4 } else { 0 };
    let grip_draw_size: c_float =  IM_FLOOR(ImMax(g.FontSize * 1.35f32, window.WindowRounding + 1f32 + g.FontSize * 0.20f32));
    let grip_hover_inner_size: c_float =  IM_FLOOR(grip_draw_size * 0.750f32);
    let grip_hover_outer_size: c_float =  if g.IO.ConfigWindowsResizeFromEdges { WINDOWS_HOVER_PADDING } else { 0f32 };

    let pos_target = ImVec2::new2(f32::MAX, f32::MAX);
    let mut size_target = ImVec2::new2(f32::MAX, f32::MAX);

    // Clip mouse interaction rectangles within the viewport rectangle (in practice the narrowing is going to happen most of the time).
    // - Not narrowing would mostly benefit the situation where OS windows _without_ decoration have a threshold for hovering when outside their limits.
    //   This is however not the case with current backends under Win32, but a custom borderless window implementation would benefit from it.
    // - When decoration are enabled we typically benefit from that distance, but then our resize elements would be conflicting with OS resize elements, so we also narrow.
    // - Note that we are unable to tell if the platform setup allows hovering with a distance threshold (on Win32, decorated window have such threshold).
    // We only clip interaction so we overwrite window.ClipRect, cannot call PushClipRect() yet as DrawList is not yet setup.
    let clip_with_viewport_rect: bool = !(g.IO.BackendFlags & ImGuiBackendFlags_HasMouseHoveredViewport) || (g.IO.MouseHoveredViewport != window.ViewportId) || !(window.Viewport.Flags & ImGuiViewportFlags_NoDecoration);
    if clip_with_viewport_rect {
        window.ClipRect = window.Viewport.GetMainRect();
    }

    // Resize grips and borders are on layer 1
    window.DC.NavLayerCurrent = ImGuiNavLayer_Menu;

    // Manual resize grips
    PushID("#RESIZE");
    // for (let resize_grip_n: c_int = 0; resize_grip_n < resize_grip_count; resize_grip_n++)
    for resize_grip_n in 0 .. resize_grip_count
    {
        let def = resize_grip_def[resize_grip_n];
        let corner: ImVec2 = ImLerp(window.Pos, window.Pos + window.Size, def.CornerPosN);

        // Using the FlattenChilds button flag we make the resize button accessible even if we are hovering over a child window
        // hovered: bool, held;
        let hovered: bool = false;
        let held: bool = false;
        let mut resize_rect = ImRect::new2(corner - def.InnerDir * grip_hover_outer_size, corner + def.InnerDir * grip_hover_inner_size);
        if resize_rect.Min.x > resize_rect.Max.x { ImSwap(resize_rect.Min.x, resize_rect.Max.x) };
        if resize_rect.Min.y > resize_rect.Max.y {
            ImSwap(resize_rect.Min.y, resize_rect.Max.y);
            let mut resize_grip_id = window.GetID3(resize_grip_n);
        } // == GetWindowResizeCornerID()
        KeepAliveID(resize_grip_id);
        ButtonBehavior(resize_rect, resize_grip_id, &hovered, &held, ImGuiButtonFlags_FlattenChildren | ImGuiButtonFlags_NoNavFocus);
        //GetForegroundDrawList(window).AddRect(resize_rect.Min, resize_rect.Max, IM_COL32(255, 255, 0, 255));
        if hovered || held {
            g.MouseCursor = if resize_grip_n & 1 {
                ImGuiMouseCursor_ResizeNESW
            } else { ImGuiMouseCursor_ResizeNWSE };
        }

        if held && g.IO.MouseClickedCount[0] == 2 && resize_grip_n == 0
        {
            // Manual auto-fit when double-clicking
            size_target = CalcWindowSizeAfterConstraint(window, size_auto_fit);
            ret_auto_fit = true;
            ClearActiveID();
        }
        else if held
        {
            // Resize from any of the four corners
            // We don't use an incremental MouseDelta but rather compute an absolute target size based on mouse position
            let clamp_min = ImVec2::new2(if def.CornerPosN.x == 1f32 { visibility_rect.Min.x } else { f32::MIN }, if def.CornerPosN.y == 1f32 { visibility_rect.Min.y } else { f32::MIN });
            let clamp_max = ImVec2::new2(if def.CornerPosN.x == 0f32 { visibility_rect.Max.x } else { f32::MAX }, if def.CornerPosN.y == 0f32 { visibility_rect.Max.y } else { f32::MAX });
            let mut corner_target: ImVec2 = g.IO.MousePos - g.ActiveIdClickOffset + ImLerp(def.InnerDir * grip_hover_outer_size, def.InnerDir * -grip_hover_inner_size, def.CornerPosN); // Corner of the window corresponding to our corner grip
            corner_target = ImClamp(corner_target, clamp_min, clamp_max);
            CalcResizePosSizeFromAnyCorner(window, &corner_target, def.CornerPosN, &pos_target, &mut size_target, null_mut());
        }

        // Only lower-left grip is visible before hovering/activating
        if resize_grip_n == 0 || held || hovered {
            resize_grip_col[resize_grip_n] = GetColorU32(if held { ImGuiCol_ResizeGripActive } else {
                            if hovered {
                                ImGuiCol_ResizeGripHovered
                            } else { ImGuiCol_ResizeGrip }
                        }, 0f32,
            );
        }
    }
    // for (let border_n: c_int = 0; border_n < resize_border_count; border_n++)
    for border_n in 0 .. resize_border_count
    {
        let def = resize_border_def[border_n];
        let axis = if (border_n == ImGuiDir_Left || border_n == ImGuiDir_Right) { ImGuiAxis_X } else { ImGuiAxis_Y };

        // hovered: bool, held;
        let mut hovered = false;
        let mut held = false;
        let border_rect =  GetResizeBorderRect(window, border_n, grip_hover_inner_size, WINDOWS_HOVER_PADDING);
        let mut border_id: ImGuiID =  window.GetID3(border_n + 4); // == GetWindowResizeBorderID()
        KeepAliveID(border_id);
        ButtonBehavior(border_rect, border_id, &hovered, &held, ImGuiButtonFlags_FlattenChildren | ImGuiButtonFlags_NoNavFocus);
        //GetForegroundDrawLists(window).AddRect(border_rect.Min, border_rect.Max, IM_COL32(255, 255, 0, 255));
        if ((hovered && g.HoveredIdTimer > WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER) || held)
        {
            g.MouseCursor = if axis == ImGuiAxis_X { ImGuiMouseCursor_ResizeEW } else { ImGuiMouseCursor_ResizeNS };
            if held {
                *border_held = border_n;
            }
        }
        if held
        {
            let mut clamp_min = ImVec2::new2(if border_n == ImGuiDir_Right { visibility_rect.Min.x } else { f32::MIN }, if border_n == ImGuiDir_Down { visibility_rect.Min.y } else { f32::MIN });
            let mut clamp_max = ImVec2::new2(if border_n == ImGuiDir_Left { visibility_rect.Max.x } else { f32::MAX }, if border_n == ImGuiDir_Up { visibility_rect.Max.y } else { f32::MAX });
            let mut border_target: ImVec2 = window.Pos;
            border_target[axis] = g.IO.MousePos[axis] - g.ActiveIdClickOffset[axis] + WINDOWS_HOVER_PADDING;
            border_target = ImClamp(border_target, clamp_min, clamp_max);
            CalcResizePosSizeFromAnyCorner(window, &border_target, ImMin(def.SegmentN1, def.SegmentN2), &pos_target, &mut size_target, null_mut());
        }
    }
    PopID();

    // Restore nav layer
    window.DC.NavLayerCurrent = ImGuiNavLayer_Main;

    // Navigation resize (keyboard/gamepad)
    // FIXME: This cannot be moved to NavUpdateWindowing() because CalcWindowSizeAfterConstraint() need to callback into user.
    // Not even sure the callback works here.
    if g.NavWindowingTarget.is_null() == false && g.NavWindowingTarget.RootWindowDockTree == window
    {
        let mut nav_resize_dir = ImVec2::new();
        if g.NavInputSource == ImGuiInputSource_Keyboard && g.IO.KeyShift {
            nav_resize_dir = GetKeyVector2d(ImGuiKey_LeftArrow, ImGuiKey_RightArrow, ImGuiKey_UpArrow, ImGuiKey_DownArrow);
        }
        if g.NavInputSource == ImGuiInputSource_Gamepad {
            nav_resize_dir = GetKeyVector2d(ImGuiKey_GamepadDpadLeft, ImGuiKey_GamepadDpadRight, ImGuiKey_GamepadDpadUp, ImGuiKey_GamepadDpadDown);
        }
        if nav_resize_dir.x != 0f32 || nav_resize_dir.y != 0f32
        {
            let NAV_RESIZE_SPEED: c_float =  600f32;
            let resize_step: c_float =  NAV_RESIZE_SPEED * g.IO.DeltaTime * ImMin(g.IO.DisplayFramebufferScale.x, g.IO.DisplayFramebufferScale.y);
            g.NavWindowingAccumDeltaSize += nav_resize_dir * resize_step;
            g.NavWindowingAccumDeltaSize = ImMax(g.NavWindowingAccumDeltaSize, visibility_rect.Min - window.Pos - window.Size); // We need Pos+Size >= visibility_rect.Min, so Size >= visibility_rect.Min - Pos, so size_delta >= visibility_rect.Min - window.Pos - window.Size
            g.NavWindowingToggleLayer = false;
            g.NavDisableMouseHover = true;
            resize_grip_col[0] = GetColorU32(ImGuiCol_ResizeGripActive, 0f32, );
            let accum_floored: ImVec2 = ImFloor(g.NavWindowingAccumDeltaSize);
            if accum_floored.x != 0f32 || accum_floored.y != 0f32
            {
                // FIXME-NAV: Should store and accumulate into a separate size buffer to handle sizing constraints properly, right now a constraint will make us stuck.
                size_target = CalcWindowSizeAfterConstraint(window, window.SizeFull + accum_floored);
                g.NavWindowingAccumDeltaSize -= accum_floored;
            }
        }
    }

    // Apply back modified position/size to window
    if (size_target.x != f32::MAX)
    {
        window.SizeFull = size_target;
        MarkIniSettingsDirty(window);
    }
    if (pos_target.x != f32::MAX)
    {
        window.Pos = ImFloor(pos_target);
        MarkIniSettingsDirty(window);
    }

    window.Size = window.SizeFull;
    return ret_auto_fit;
}
