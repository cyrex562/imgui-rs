use std::ptr::{null, null_mut};
use libc::{c_char, c_float, c_int};
use crate::color::{IM_COL32_A_MASK, IM_COL32_A_SHIFT, ImGuiCol_Border, ImGuiCol_Button, ImGuiCol_ButtonActive, ImGuiCol_ButtonHovered, ImGuiCol_MenuBarBg, ImGuiCol_ModalWindowDimBg, ImGuiCol_NavWindowingDimBg, ImGuiCol_NavWindowingHighlight, ImGuiCol_SeparatorActive, ImGuiCol_Text, ImGuiCol_TitleBg, ImGuiCol_TitleBgActive, ImGuiCol_TitleBgCollapsed};
use crate::core::direction::{ImGuiDir_Left, ImGuiDir_None, ImGuiDir_Right};
use crate::{GImGui, ImguiViewport};
use crate::core::axis::{IM_GUI_AXIS_X, IM_GUI_AXIS_Y};
use crate::drawing::draw_flags::{ImDrawFlags_None, ImDrawFlags_RoundCornersBottom, ImDrawFlags_RoundCornersTop};
use crate::drawing::draw_list::ImDrawList;
use crate::draw_list_ops::GetForegroundDrawList;
use KeepAliveID;
use crate::input_ops::IsMouseDragging;
use crate::item::item_flags::{ImGuiItemFlags, ImGuiItemFlags_NoNavDefaultFocus};
use crate::core::math_ops::{ImClamp, ImFabs, ImLerp, ImMax, ImMin};
use crate::io::mouse_ops::StartMouseMovingWindowOrNode;
use crate::nav_layer::{ImGuiNavLayer_Main, ImGuiNavLayer_Menu};
use crate::window::next_window_data_flags::ImGuiNextWindowDataFlags_HasBgAlpha;
use crate::rect::ImRect;
use crate::drawing::render_ops::{RenderBullet, RenderFrame, RenderRectFilledWithHole, RenderTextClipped};
use crate::widgets::resize_border_def::resize_border_def;
use crate::widgets::resize_grip_def::resize_grip_def;
use crate::core::string_ops::str_to_const_c_char_ptr;
use crate::style::ImguiStyle;
use crate::style_ops::GetColorU32;
use crate::text_ops::CalcTextSize;
use crate::core::type_defs::ImguiHandle;
use crate::core::utils::{flag_clear, flag_set, is_not_null, is_null};
use crate::core::vec2::Vector2;
use crate::window::{find, ImguiWindow, ops};
use crate::window::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_DockNodeHost, ImGuiWindowFlags_MenuBar, ImGuiWindowFlags_Modal, ImGuiWindowFlags_NavFlattened, ImGuiWindowFlags_NoBackground, ImGuiWindowFlags_NoCollapse, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_Popup, ImGuiWindowFlags_Tooltip, ImGuiWindowFlags_UnsavedDocument};

// Render title text, collapse button, close button
// When inside a dock node, this is handled in DockNodeCalcTabBarLayout() instead.
pub unsafe fn RenderWindowTitleBarContents(window: &mut ImguiWindow, mut title_bar_rect: *mut ImRect, name: *const c_char, p_open: *mut bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = &mut g.style;
    flags: ImGuiWindowFlags = window.Flags;

    let has_close_button: bool = (p_open != null_mut());
    let has_collapse_button: bool = flag_clear(flags, ImGuiWindowFlags_NoCollapse) && (style.WindowMenuButtonPosition != ImGuiDir_None);

    // Close & Collapse button are on the Menu NavLayer and don't default focus (unless there's nothing else on that layer)
    // FIXME-NAV: Might want (or not?) to set the equivalent of ImGuiButtonFlags_NoNavFocus so that mouse clicks on standard title bar items don't necessarily set nav/keyboard ref?
    let mut item_flags_backup: ImGuiItemFlags =  g.CurrentItemFlags;
    g.CurrentItemFlags |= ImGuiItemFlags_NoNavDefaultFocus;
    window.dc.NavLayerCurrent = ImGuiNavLayer_Menu;

    // Layout buttons
    // FIXME: Would be nice to generalize the subtleties expressed here into reusable code.
    let mut pad_l: c_float =  style.FramePadding.x;
    let mut pad_r: c_float =  style.FramePadding.x;
    let button_sz: c_float =  g.FontSize;
    close_button_pos: Vector2;
    collapse_button_pos: Vector2;
    if has_close_button
    {
        pad_r += button_sz;
        close_button_pos = Vector2::from_floats(title_bar_rect.max.x - pad_r - style.FramePadding.x, title_bar_rect.min.y);
    }
    if has_collapse_button && style.WindowMenuButtonPosition == ImGuiDir_Right
    {
        pad_r += button_sz;
        collapse_button_pos = Vector2::from_floats(title_bar_rect.max.x - pad_r - style.FramePadding.x, title_bar_rect.min.y);
    }
    if has_collapse_button && style.WindowMenuButtonPosition == ImGuiDir_Left
    {
        collapse_button_pos = Vector2::from_floats(title_bar_rect.min.x + pad_l - style.FramePadding.x, title_bar_rect.min.y);
        pad_l += button_sz;
    }

    // Collapse button (submitting first so it gets priority when choosing a navigation init fallback)
    if has_collapse_button {
        if CollapseButton(window.id_by_string(null(), str_to_const_c_char_ptr("#COLLAPSE")), collapse_button_pos, null_mut()) {
            window.WantCollapseToggle = true;
        }
    } // Defer actual collapsing to next frame as we are too far in the Begin() function

    // Close button
    if has_close_button {
        if CloseButton(window.id_by_string(null(), str_to_const_c_char_ptr("#CLOSE")), close_button_pos) {
            *p_open = false;
        }
    }

    window.dc.NavLayerCurrent = ImGuiNavLayer_Main;
    g.CurrentItemFlags = item_flags_backup;

    // Title bar text (with: horizontal alignment, avoiding collapse/close button, optional "unsaved document" marker)
    // FIXME: Refactor text alignment facilities along with RenderText helpers, this is WAY too much messy code..
    let marker_size_x: c_float = if  flag_set(flags, ImGuiWindowFlags_UnsavedDocument) { button_sz * 0.80 } else { 0.0 };
    let text_size: Vector2 = CalcTextSize(name, None, true, 0.0) + Vector2::from_floats(marker_size_x, 0.0);

    // As a nice touch we try to ensure that centered title text doesn't get affected by visibility of Close/Collapse button,
    // while uncentered title text will still reach edges correctly.
    if pad_l > style.FramePadding.x {
        pad_l += g.style.ItemInnerSpacing.x;
    }
    if pad_r > style.FramePadding.x {
        pad_r += g.style.ItemInnerSpacing.x;
    }
    if style.WindowTitleAlign.x > 0.0 && style.WindowTitleAlign.x < 1.0
    {
        let centerness: c_float =  ImSaturate(1.0 - ImFabs(style.WindowTitleAlign.x - 0.5) * 2.0); // 0.0 on either edges, 1.0 on center
        let pad_extend: c_float =  ImMin(ImMax(pad_l, pad_r), title_bar_rect.GetWidth() - pad_l - pad_r - text_size.x);
        pad_l = ImMax(pad_l, pad_extend * centerness);
        pad_r = ImMax(pad_r, pad_extend * centerness);
    }

    let mut layout_r: ImRect = ImRect::new(title_bar_rect.min.x + pad_l, title_bar_rect.min.y, title_bar_rect.max.x - pad_r, title_bar_rect.max.y);
    let mut clip_r: ImRect = ImRect::new(layout_r.min.x, layout_r.min.y, ImMin(layout_r.max.x + g.style.ItemInnerSpacing.x, title_bar_rect.max.x), layout_r.max.y);
    if flags & ImGuiWindowFlags_UnsavedDocument
    {
        marker_pos: Vector2;
        marker_pos.x = ImClamp(layout_r.min.x + (layout_r.GetWidth() - text_size.x) * style.WindowTitleAlign.x + text_size.x, layout_r.min.x, layout_r.max.x);
        marker_pos.y = (layout_r.min.y + layout_r.max.y) * 0.5;
        if marker_pos.x > layout_r.min.x
        {
            RenderBullet(window.DrawList, marker_pos, GetColorU32(ImGuiCol_Text, 0.0));
            clip_r.max.x = ImMin(clip_r.max.x, marker_pos.x - (marker_size_x * 0.5));
        }
    }
    //if (g.IO.KeyShift) window.DrawList.AddRect(layout_r.Min, layout_r.Max, IM_COL32(255, 128, 0, 255)); // [DEBUG]
    //if (g.IO.KeyCtrl) window.DrawList.AddRect(clip_r.Min, clip_r.Max, IM_COL32(255, 128, 0, 255)); // [DEBUG]
    RenderTextClipped(&layout_r.min, &layout_r.max, name, None, &text_size, &style.WindowTitleAlign, &clip_r);
}


pub fn UpdateWindowParentAndRootLinks(window: &mut ImguiWindow, flags: ImGuiWindowFlags, parent_window: &mut ImguiWindow)
{
    window.ParentWindow = parent_window;
    window.RootWindow = window.RootWindowPopupTree = window.RootWindowDockTree = window.RootWindowForTitleBarHighlight = window.RootWindowForNav = window;
    if is_not_null(parent_window) && flag_set(flags, ImGuiWindowFlags_ChildWindow) && flag_clear(flags, ImGuiWindowFlags_Tooltip)
    {
        window.RootWindowDockTree = parent_window.RootWindowDockTree;
        if !window.DockIsActive && flag_clear(parent_window.Flags, ImGuiWindowFlags_DockNodeHost) {
            window.RootWindow = parent_window.RootWindow;
        }
    }
    if parent_window && (flags & ImGuiWindowFlags_Popup){
        window.RootWindowPopupTree = parent_window.RootWindowPopupTree;}
    if (parent_window && flag_clear(flags, ImGuiWindowFlags_Modal) && (flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_Popup))) // FIXME: simply use _NoTitleBar ?
        window.RootWindowForTitleBarHighlight = parent_window.RootWindowForTitleBarHighlight;
    while (window.RootWindowForNav.Flags & ImGuiWindowFlags_NavFlattened)
    {
        // IM_ASSERT(window.RootWindowForNav->ParentWindow != NULL);
        window.RootWindowForNav = window.RootWindowForNav->ParentWindow;
    }
}

// static c_void RenderDimmedBackgroundBehindWindow(window: &mut ImGuiWindow, col: u32)
pub unsafe fn RenderDimmedBackgroundBehindWindow(window: &mut ImguiWindow, col: u32) {
    if (col & IM_COL32_A_MASK) == 0 {
        return;
    }

    let mut viewport: *mut ImguiViewport = window.Viewport;
    let viewport_rect: ImRect = viewport.get_main_rect();

    // Draw behind window by moving the draw command at the FRONT of the draw list
    unsafe {
        // We've already called AddWindowToDrawData() which called DrawList.ChannelsMerge() on DockNodeHost windows,
        // and draw list have been trimmed already, hence the explicit recreation of a draw command if missing.
        // FIXME: This is creating complication, might be simpler if we could inject a drawlist in drawdata at a given position and not attempt to manipulate ImDrawCmd order.
        let mut draw_list: *mut ImDrawList = window.RootWindowDockTree.DrawList;
        if draw_list.CmdBuffer.len() == 0 {
            draw_list.AddDrawCmd();
        }
        draw_list.PushClipRect(
            viewport_rect.min - Vector2::from_floats(1.0, 1.0),
            viewport_rect.max + Vector2::from_floats(1.0, 1.0),
            false,
        ); // Ensure ImDrawCmd are not merged
        draw_list.AddRectFilled(
            &viewport_rect.min,
            &viewport_rect.max,
            col,
            0.0,
            ImDrawFlags_None,
        );
        let cmd = draw_list.CmdBuffer.last().unwrap();
        // IM_ASSERT(cmd.ElemCount == 6);
        draw_list.CmdBuffer.pop_back();
        draw_list.CmdBuffer.push_front(cmd);
        draw_list.PopClipRect();
        draw_list.AddDrawCmd(); // We need to create a command as CmdBuffer.back().IdxOffset won't be correct if we append to same command.
    }

    // Draw over sibling docking nodes in a same docking tree
    if window.Rootwindow.DockIsActive {
        let mut draw_list: *mut ImDrawList =
            find::FindFrontMostVisibleChildWindow(window.RootWindowDockTree).DrawList;
        if draw_list.CmdBuffer.len() == 0 {
            draw_list.AddDrawCmd();
        }
        draw_list.PushClipRect(&viewport_rect.min, &viewport_rect.max, false);
        RenderRectFilledWithHole(
            draw_list,
            window.RootWindowDockTree.Rect(),
            window.Rootwindow.Rect(),
            col,
            0.0,
        ); // window.RootWindowDockTree.WindowRounding);
        draw_list.PopClipRect();
    }
}

pub unsafe fn RenderDimmedBackgrounds() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut modal_window: &mut ImguiWindow = GetTopMostAndVisiblePopupModal();
    if g.DimBgRatio <= 0.0 && g.NavWindowingHighlightAlpha <= 0.0 {
        return;
    }
    let dim_bg_for_modal: bool = (modal_window != null_mut());
    let dim_bg_for_window_list: bool =
        (g.NavWindowingTargetAnim != None && g.NavWindowingTargetAnim.Active);
    if !dim_bg_for_modal && !dim_bg_for_window_list {
        return;
    }

    let mut viewports_already_dimmed: [*mut ImguiViewport; 2] = [None, None];
    if dim_bg_for_modal {
        // Draw dimming behind modal or a begin stack child, whichever comes first in draw order.
        let mut dim_behind_window: &mut ImguiWindow =
            FindBottomMostVisibleWindowWithinBeginStack(modal_window);
        RenderDimmedBackgroundBehindWindow(
            dim_behind_window,
            GetColorU32(ImGuiCol_ModalWindowDimBg, g.DimBgRatio),
        );
        viewports_already_dimmed[0] = modal_window.Viewport;
    } else if dim_bg_for_window_list {
        // Draw dimming behind CTRL+Tab target window and behind CTRL+Tab UI window
        RenderDimmedBackgroundBehindWindow(
            g.NavWindowingTargetAnim,
            GetColorU32(ImGuiCol_NavWindowingDimBg, g.DimBgRatio),
        );
        if g.NavWindowingListWindow != None
            && g.NavWindowingListwindow.Viewport
            && g.NavWindowingListwindow.Viewport != g.NavWindowingTargetAnim.Viewport
        {
            RenderDimmedBackgroundBehindWindow(
                g.NavWindowingListWindow,
                GetColorU32(ImGuiCol_NavWindowingDimBg, g.DimBgRatio),
            );
        }
        viewports_already_dimmed[0] = g.NavWindowingTargetAnim.Viewport;
        viewports_already_dimmed[1] = if g.NavWindowingListWindow {
            g.NavWindowingListwindow.Viewport
        } else {
            None
        };

        // Draw border around CTRL+Tab target window
        let mut window: &mut ImguiWindow = g.NavWindowingTargetAnim;
        ImguiViewport * viewport = window.Viewport;
        let distance: c_float = g.FontSize;
        let mut bb: ImRect = window.Rect();
        bb.Expand(distance);
        if bb.GetWidth() >= viewport.Size.x && bb.GetHeight() >= viewport.Size.y {
            bb.Expand(-distance - 1.0);
        } // If a window fits the entire viewport, adjust its highlight inward
        if window.DrawList.CmdBuffer.len() == 0 {
            window.DrawList.AddDrawCmd();
        }
        window
            .DrawList
            .PushClipRect(viewport.Pos, viewport.Pos + viewport.Size, false);
        window.DrawList.AddRect(
            &bb.min,
            &bb.max,
            GetColorU32(ImGuiCol_NavWindowingHighlight, g.NavWindowingHighlightAlpha),
            window.WindowRounding,
        );
        window.DrawList.PopClipRect();
    }

    // Draw dimming background on _other_ viewports than the ones our windows are in
    // for (let viewport_n: c_int = 0; viewport_n < g.Viewports.Size; viewport_n++)
    for viewport_n in 0..g.Viewports.len() {
        let mut viewport: *mut ImguiViewport = g.Viewports[viewport_n];
        if viewport == viewports_already_dimmed[0] || viewport == viewports_already_dimmed[1] {
            continue;
        }
        if modal_window && viewport.Window && IsWindowAbove(viewport.Window, modal_window) {
            continue;
        }
        let mut draw_list: *mut ImDrawList = GetForegroundDrawList(viewport);
        let dim_bg_col = GetColorU32(
            if dim_bg_for_modal {
                ImGuiCol_ModalWindowDimBg
            } else {
                ImGuiCol_NavWindowingDimBg
            },
            g.DimBgRatio,
        );
        draw_list.AddRectFilled(
            &viewport.Pos,
            viewport.Pos.clone() + viewport.Size.clone(),
            dim_bg_col,
            0.0,
            ImDrawFlags_None,
        );
    }
}


pub unsafe fn RenderWindowOuterBorders(window: &mut ImguiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let rounding: c_float =  window.WindowRounding;
    let border_size: c_float =  window.WindowBorderSize;
    if border_size > 0.0 && flag_clear(window.Flags, ImGuiWindowFlags_NoBackground) {
        window.DrawList.AddRect(&window.position, window.position + window.Size, GetColorU32(ImGuiCol_Border, 0.0), rounding);
    }

    let border_held = window.ResizeBorderHeld;
    if border_held != -1
    {
        let def = resize_border_def[border_held];
        let border_r: ImRect =  ops::GetResizeBorderRect(window, border_held as c_int, rounding, 0.0);
        window.DrawList.PathArcTo(ImLerp(border_r.min, border_r.max, def.SegmentN1) + Vector2::from_floats(0.5, 0.5) + def.InnerDir * rounding, rounding, def.OuterAngle - IM_PI * 0.25, def.OuterAngle, 0);
        window.DrawList.PathArcTo(ImLerp(border_r.min, border_r.max, def.SegmentN2) + Vector2::from_floats(0.5, 0.5) + def.InnerDir * rounding, rounding, def.OuterAngle, def.OuterAngle + IM_PI * 0.25, 0);
        window.DrawList.PathStroke(GetColorU32(ImGuiCol_SeparatorActive, 0.0), 0, ImMax(2.0, border_size)); // Thicker than usual
    }
    if g.style.FrameBorderSize > 0.0 && flag_clear(window.Flags, ImGuiWindowFlags_NoTitleBar) && !window.DockIsActive
    {
        let y: c_float =  window.position.y + window.TitleBarHeight() - 1;
        window.DrawList.AddLine(&Vector2::from_floats(window.position.x + border_size, y), &Vector2::from_floats(window.position.x + window.Size.x - border_size, y), GetColorU32(ImGuiCol_Border, 0.0), g.style.FrameBorderSize);
    }
}


// Draw background and borders
// Draw and handle scrollbars
pub unsafe fn RenderWindowDecorations(window: &mut ImguiWindow, title_bar_rect: &ImRect, title_bar_is_highlight: bool, handle_borders_and_resize_grips: bool, resize_grip_count: c_int, resize_grip_col: [u32;4], resize_grip_draw_size: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = &mut g.style;
    let flags: ImGuiWindowFlags = window.Flags;

    // Ensure that ScrollBar doesn't read last frame's SkipItems
    // IM_ASSERT(window.BeginCount == 0);
    window.skip_items = false;

    // Draw window + handle manual resize
    // As we highlight the title bar when want_focus is set, multiple reappearing windows will have have their title bar highlighted on their reappearing frame.
    let window_rounding: c_float =  window.WindowRounding;
    let window_border_size: c_float =  window.WindowBorderSize;
    if window.Collapsed
    {
        // Title bar only
        let backup_border_size: c_float =  style.FrameBorderSize;
        g.style.FrameBorderSize = window.WindowBorderSize;
        title_bar_col: u32 = GetColorU32(if title_bar_is_highlight && !g.NavDisableHighlight { ImGuiCol_TitleBgActive } else { ImGuiCol_TitleBgCollapsed }, 0.0);
        RenderFrame(title_bar_rect.min, title_bar_rect.max, title_bar_col, true, window_rounding);
        g.style.FrameBorderSize = backup_border_size;
    }
    else
    {
        // Window background
        if flag_clear(flags, ImGuiWindowFlags_NoBackground)
        {
            let mut is_docking_transparent_payload: bool =  false;
            if g.DragDropActive && (g.FrameCount - g.DragDropAcceptFrameCount) <= 1 && g.IO.ConfigDockingTransparentPayload {
                if g.DragDropPayload.IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) &&
                g.DragDropPayload.Data == window{
                    is_docking_transparent_payload = true;
                }
            }

            bg_col: u32 = GetColorU32(ops::GetWindowBgColorIdx(window), 0.0);
            if window.ViewportOwned
            {
                // No alpha
                bg_col = (bg_col | IM_COL32_A_MASK);
                if is_docking_transparent_payload {
                    window.Viewport.Alpha *= DOCKING_TRANSPARENT_PAYLOAD_ALPHA;
                }
            }
            else
            {
                // Adjust alpha. For docking
                let mut override_alpha: bool =  false;
                let mut alpha: c_float =  1.0;
                if g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasBgAlpha
                {
                    alpha = g.NextWindowData.BgAlphaVal;
                    override_alpha = true;
                }
                if is_docking_transparent_payload
                {
                    alpha *= DOCKING_TRANSPARENT_PAYLOAD_ALPHA; // FIXME-DOCK: Should that be an override?
                    override_alpha = true;
                }
                if override_alpha {
                    bg_col = (bg_col & !IM_COL32_A_MASK) | (IM_F32_TO_INT8_SAT(alpha) << IM_COL32_A_SHIFT);
                }
            }

            // Render, for docked windows and host windows we ensure bg goes before decorations
            if window.DockIsActive {
                window.DockNode.LastBgColor = bg_col;
            }
            let mut  bg_draw_list: *mut ImDrawList =  if window.DockIsActive { window.DockNode.Hostwindow.DrawList } else { window.DrawList };
            if window.DockIsActive || flag_set(flags, ImGuiWindowFlags_DockNodeHost) {
                bg_draw_list.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_BG);
            }
            bg_draw_list.AddRectFilled(window.position + Vector2::from_floats(0.0, window.TitleBarHeight()), window.position + window.Size, bg_col, window_rounding, if flag_set(flags, ImGuiWindowFlags_NoTitleBar) { 0 } else { ImDrawFlags_RoundCornersBottom });
            if window.DockIsActive || flag_set(flags, ImGuiWindowFlags_DockNodeHost) {
                bg_draw_list.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_FG);
            }
        }
        if window.DockIsActive {
            window.DockNode.IsBgDrawnThisFrame = true;
        }

        // Title bar
        // (when docked, DockNode are drawing their own title bar. Individual windows however do NOT set the _NoTitleBar flag,
        // in order for their pos/size to be matching their undocking state.)
        if flag_clear(flags, ImGuiWindowFlags_NoTitleBar) && !window.DockIsActive
        {
            title_bar_col: u32 = GetColorU32(if title_bar_is_highlight { ImGuiCol_TitleBgActive } else { ImGuiCol_TitleBg }, 0.0);
            window.DrawList.AddRectFilled(&title_bar_rect.min, &title_bar_rect.max, title_bar_col, window_rounding, ImDrawFlags_RoundCornersTop);
        }

        // Menu bar
        if flag_set(flags, ImGuiWindowFlags_MenuBar)
        {
            let mut menu_bar_rect: ImRect =  window.MenuBarRect();
            menu_bar_rect.ClipWith(window.Rect());  // Soft clipping, in particular child window don't have minimum size covering the menu bar so this is useful for them.
            window.DrawList.AddRectFilled(menu_bar_rect.min + Vector2::from_floats(window_border_size, 0.0), menu_bar_rect.max - Vector2::from_floats(window_border_size, 0.0), GetColorU32(ImGuiCol_MenuBarBg, 0.0), if flag_set(flags, ImGuiWindowFlags_NoTitleBar) { window_rounding }else { 0.0 }, ImDrawFlags_RoundCornersTop);
            if style.FrameBorderSize > 0.0 && menu_bar_rect.max.y < window.position.y + window.Size.y {
                window.DrawList.AddLine(&menu_bar_rect.GetBL(), &menu_bar_rect.GetBR(), GetColorU32(ImGuiCol_Border, 0.0), style.FrameBorderSize);
            }
        }

        // Docking: Unhide tab bar (small triangle in the corner), drag from small triangle to quickly undock
        node:*mut ImGuiDockNode = window.DockNode;
        if window.DockIsActive && node.IsHiddenTabBar() && !node.IsNoTabBar()
        {
            let unhide_sz_draw: c_float =  ImFloor(g.FontSize * 0.70);
            let unhide_sz_hit: c_float =  ImFloor(g.FontSize * 0.550f32);
            let p: Vector2 = node.Pos;
            let mut r: ImRect = ImRect::new(p, p + Vector2::from_floats(unhide_sz_hit, unhide_sz_hit));
            let mut unhide_id: ImguiHandle =  window.id_by_string(null(), str_to_const_c_char_ptr("#UNHIDE"));
            KeepAliveID(unhide_id);
            // hovered: bool, held;
            let mut hovered = false;
            let mut held = false;
            if ButtonBehavior(r, unhide_id, &hovered, &held, ImGuiButtonFlags_FlattenChildren) {
                node.WantHiddenTabBarToggle = true;
            }
            else if held && IsMouseDragging(0, 0.0) {
                StartMouseMovingWindowOrNode(window, node, true);
            }

            // FIXME-DOCK: Ideally we'd use ImGuiCol_TitleBgActive/ImGuiCol_TitleBg here, but neither is guaranteed to be visible enough at this sort of size..
            col: u32 = GetColorU32(if (held && hovered) || (node.IsFocused && !hovered) { ImGuiCol_ButtonActive } else {
                if hovered {
                    ImGuiCol_ButtonHovered
                } else { ImGuiCol_Button }
            }, 0.0);
            window.DrawList.AddTriangleFilled(&p, p + Vector2::from_floats(unhide_sz_draw, 0.0), p + Vector2::from_floats(0.0, unhide_sz_draw), col);
        }

        // Scrollbars
        if window.scrollbarX {
            Scrollbar(IM_GUI_AXIS_X);
        }
        if window.scrollbarY {
            Scrollbar(IM_GUI_AXIS_Y);
        }

        // Render resize grips (after their input handling so we don't have a frame of latency)
        if handle_borders_and_resize_grips && flag_clear(flags, ImGuiWindowFlags_NoResize)
        {
            // for (let resize_grip_n: c_int = 0; resize_grip_n < resize_grip_count; resize_grip_n++)
            for resize_grip_n in 0 .. resize_grip_count
            {
                let grip = resize_grip_def[resize_grip_n];
                let corner: Vector2 = ImLerp(window.position, window.position + window.Size, grip.CornerPosN);
                window.DrawList.PathLineTo(corner + grip.InnerDir * (if resize_grip_n & 1 { Vector2::from_floats(window_border_size, resize_grip_draw_size) } else { Vector2::from_floats(resize_grip_draw_size, window_border_size) }));
                window.DrawList.PathLineTo(corner + grip.InnerDir * (if resize_grip_n & 1 { Vector2::from_floats(resize_grip_draw_size, window_border_size) } else { Vector2::from_floats(window_border_size, resize_grip_draw_size) }));
                window.DrawList.PathArcToFast(&Vector2::from_floats(corner.x + grip.InnerDir.x * (window_rounding + window_border_size), corner.y + grip.InnerDir.y * (window_rounding + window_border_size)), window_rounding, grip.AngleMin12, grip.AngleMax12);
                window.DrawList.PathFillConvex(resize_grip_col[resize_grip_n]);
            }
        }

        // Borders (for dock node host they will be rendered over after the tab bar)
        if handle_borders_and_resize_grips && is_null(window.DockNodeAsHost) {
            RenderWindowOuterBorders(window);
        }
    }
}
