use crate::{Context, INVALID_ID, window};
use crate::globals::GImGui;
use crate::rect::Rect;
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;
use crate::window::{checks, Window, WindowFlags};

// static ImGuiWindow* GetCombinedRootWindow(ImGuiWindow* window, bool popup_hierarchy, bool dock_hierarchy)
pub fn get_combined_root_window(g: &mut Context, window: &mut Window, popup_hierarchy: bool, dock_hierarchy: bool) -> &mut Window
{
    ImGuiWindow* last_window = NULL;
    while (last_window != window)
    {
        last_window = window;
        window = window.root_window;
        if (popup_hierarchy)
            window = window.root_window_popup_tree;
		if (dock_hierarchy)
			window = window.root_window_dock_tree;
	}
    return window;
}



// bool ImGui::IsWindowChildOf(ImGuiWindow* window, ImGuiWindow* potential_parent, bool popup_hierarchy, bool dock_hierarchy)
pub fn is_window_child_of(g: &mut Context, window: &mut Window, potential_parent: &mut Window, popup_hierarchy: bool, dock_hierarchy: bool) -> bool
{
    ImGuiWindow* window_root = GetCombinedRootWindow(window, popup_hierarchy, dock_hierarchy);
    if (window_root == potential_parent)
        return true;
    while (window != NULL)
    {
        if (window == potential_parent)
            return true;
        if (window == window_root) // end of chain
            return false;
        window = window.parent_window;
    }
    return false;
}

// bool ImGui::is_window_within_begin_stack_of(ImGuiWindow* window, ImGuiWindow* potential_parent)
pub fn is_window_within_begin_stack_of(g: &mut Context, window: &mut Window, potential_parent: &mut Window) -> bool
{
    if (window.root_window == potential_parent)
        return true;
    while (window != NULL)
    {
        if (window == potential_parent)
            return true;
        window = window.ParentWindowInBeginStack;
    }
    return false;
}

// ImGuiID ImGui::GetWindowDockID()
pub fn get_window_dock_id(g: &mut Context) -> Id32
{
    // ImGuiContext& g = *GImGui;
    return g.current_window.dock_id;
}

// float ImGui::GetWindowWidth()
pub fn get_window_width(g: &mut Context) -> f32
{
    ImGuiWindow* window = g.current_window_id;
    return window.size.x;
}

// float ImGui::GetWindowHeight()
pub fn get_window_height(g: &mut Context) -> f32
{
    ImGuiWindow* window = g.current_window_id;
    return window.size.y;
}


// Vector2D ImGui::GetWindowPos()
pub fn get_window_pos(g: &mut Context) -> Vector2D
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    return window.pos;
}

// Vector2D ImGui::GetWindowSize()
pub fn get_window_size(g: &mut Context) -> Vector2D
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.size;
}

/// static inline int GetWindowDisplayLayer(ImGuiWindow* window)
pub fn get_window_display_layer(window: &Window) -> i32 {
    // return (window.flags & WindowFlags::Tooltip) ? 1 : 0;
    if window.flags.contains(&WindowFlags::Tooltip) {
        1
    } else {
        0
    }
}

// static ImGuiWindow* FindFrontMostVisibleChildWindow(ImGuiWindow* window)
pub fn find_front_most_visible_child_window(ctx: &mut Context, window: &mut Window) -> &mut Window {
    // for (int n = window.dc.ChildWindows.Size - 1; n >= 0; n--){
    //     if (IsWindowActiveAndVisible(window.dc.ChildWindows[n])) {
    //         return FindFrontMostVisibleChildWindow(window.dc.ChildWindows[n]);
    //     }
    // }
    for child_win_id in window.dc.child_windows.iter() {
        let child_win = ctx.get_window(*child_win_id).unwrap();
        if checks::is_window_active_and_visible(child_win) {
            return find_front_most_visible_child_window(ctx, child_win);
        }
    }
    return window;
}

// ImGuiWindow* ImGui::FindBottomMostVisibleWindowWithinBeginStack(ImGuiWindow* parent_window)
pub fn find_bottom_most_visible_window_with_begin_stack(
    ctx: &mut Context,
    parent_window: &mut Window,
) -> &mut Window {
    // ImGuiContext& g = *GImGui;
    // ImGuiWindow* bottom_most_visible_window = parent_window;
    let mut bottom_most_visible_window: &mut Window = parent_window;
    // for (int i = FindWindowDisplayIndex(parent_window); i >= 0; i--)
    for i in find_window_display_index(parent_window)..0 {
        // ImGuiWindow* window = g.windows[i];
        let window = ctx.get_window(i).unwrap();
        if window.flags.contains(&WindowFlags::ChildWindow) {
            continue;
        }
        if !is_window_within_begin_stack_of(window, parent_window) {
            break;
        }
        if checks::is_window_active_and_visible(window)
            && get_window_display_layer(window) <= get_window_display_layer(parent_window)
        {
            bottom_most_visible_window = window;
        }
    }
    return bottom_most_visible_window;
}

// Find window given position, search front-to-back
// FIXME: Note that we have an inconsequential lag here: outer_rect_clipped is updated in Begin(), so windows moved programmatically
// with set_window_pos() and not SetNextWindowPos() will have that rectangle lagging by a frame at the time FindHoveredWindow() is
// called, aka before the next Begin(). Moving window isn't affected.
// static void find_hovered_window()
pub fn find_hovered_window(g: &mut Context) {
    // ImGuiContext& g = *GImGui;

    // Special handling for the window being moved: Ignore the mouse viewport check (because it may reset/lose its viewport during the undocking frame)
    // ImGuiViewportP* moving_window_viewport = g.moving_window ? g.moving_window->Viewport : NULL;

    let moving_window_viewport = if g.moving_window_id != INVALID_ID {
        let mw_win = g.get_window(g.moving_window_id).unwrap();
        Some(g.get_viewport(mw_win.viewport_id).unwrap())
    } else {
        None
    };
    if g.moving_window_id != INVALID_ID {
        // g.moving_window.Viewport = g.mouse_viewport;
        let mw_win = g.get_window(g.moving_window_id).unwrap();
        mw_win.viewport_id = g.mouse_viewport_id;
    }

    // ImGuiWindow* hovered_window = NULL;
    // ImGuiWindow* hovered_window_ignoring_moving_window = NULL;
    let mut hovered_window: Option<&mut Window>;
    let mut hovered_window_ignoring_moving_window: Option<&mut Window> = None;
    if g.moving_window && !(g.moving_window.flags.contains(WindowFlags::NoMouseInputs)) {
        hovered_window = g.get_window(g.moving_window_id).unwrawp();
    }

    // Vector2D padding_regular = g.style.touch_extra_padding;
    let padding_regular = g.style.touch_extra_padding.clone();
    // Vector2D padding_for_resize = g.io.ConfigWindowsResizeFromEdges ? g.windows_hover_padding : padding_regular;
    let padding_for_resize = if g.io.config_windows_resize_from_edges {
        g.windows_hover_padding.clone()
    } else {
        padding_regular
    };
    // for (int i = g.windows.Size - 1; i >= 0; i--)
    for (_, window) in g.windows.iter_mut() {
        // ImGuiWindow* window = g.windows[i];
        // IM_MSVC_WARNING_SUPPRESS(28182); // [Static Analyzer] Dereferencing NULL pointer.
        if !window.active || window.hidden {
            continue;
        }
        if window.flags.contains(&WindowFlags::NoMouseInputs) {
            continue;
        }
        // IM_ASSERT(window.viewport);
        if window.viewport_id != g.mouse_viewport {
            continue;
        }

        // Using the clipped AABB, a child window will typically be clipped by its parent (not always)
        // ImRect bb(window.OuterRectClipped);
        let bb = window.outer_rect_clipped.clone();
        // if (window.flags & (WindowFlags::ChildWindow | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_AlwaysAutoResize))
        if window.flags.contains(&WindowFlags::ChildWindow)
            && window.flags.contains(&WindowFlags::NoResize)
            && window.flags.contains(&WindowFlags::AlwaysAutoResize)
        {
            bb.expand(padding_regular);
        } else {
            bb.expand(padding_for_resize.clone());
        }
        if !bb.contains_vector(&g.io.mouse_pos) {
            continue;
        }

        // Support for one rectangular hole in any given window
        // FIXME: Consider generalizing hit-testing override (with more generic data, callback, etc.) (#1512)
        if window.hit_test_hole_size.x != 0.0 {
            // Vector2D hole_pos(window.pos.x + window.HitTestHoleOffset.x, window.pos.y + window.HitTestHoleOffset.y);
            // Vector2D hole_size((float)window.hit_test_hole_size.x, window.hit_test_hole_size.y);
            let hole_size = Vector2D::new(window.hit_test_hole_size.x, window.hit_test_hole_size.y);
            if Rect::new2(hole_pos, hole_pos + hole_size).contains_vector(&g.io.mouse_pos) {
                continue;
            }
        }

        if hovered_window.is_none() {
            hovered_window = Some(window);
        }
        // IM_MSVC_WARNING_SUPPRESS(28182); // [Static Analyzer] Dereferencing NULL pointer.

        if (hovered_window_ignoring_moving_window.is_none()
            && (g.moving_window_id == INVALID_ID
                || window.root_window_dock_tree_id
                    != g.get_window(g.moving_window_id)
                        .unwrap()
                        .root_window_dock_tree_id))
        {
            hovered_window_ignoring_moving_window = Some(window);
        }
        if hovered_window.is_some() && hovered_window_ignoring_moving_window.is_some() {
            break;
        }
    }

    g.hovered_window_id = hovered_window.unwrap().id;
    g.hovered_window_under_moving_window = hovered_window_ignoring_moving_window;

    if g.moving_window_id != INVALID_ID {
        g.get_window(g.moving_window_id).unwrap().viewport_id = moving_window_viewport.unwrap().id;
    }
}

// ImGuiWindow* ImGui::FindWindowByID(ImGuiID id)
pub fn find_window_id(g: &mut Context, id: Id32) -> &mut Window {
    // ImGuiContext& g = *GImGui;
    // return (ImGuiWindow*)g.windows_by_id.GetVoidPtr(id);
    g.windows.get_mut(&id).unwrap()
}

// static ImGuiWindow* GetWindowForTitleDisplay(ImGuiWindow* window)
pub fn get_window_for_title_display(g: &mut Context, window: &mut Window) -> &mut Window {
    // return window.DockNodeAsHost ? window.DockNodeAsHost->VisibleWindow : window;
    if window.dock_node_as_host.id != INVALID_ID {
        return g
            .get_window(window.dock_node_as_host.visible_window)
            .unwrap();
    }
    return window;
}

// static ImGuiWindow* GetWindowForTitleAndMenuHeight(ImGuiWindow* window)
pub fn get_window_for_title_and_menu_height(g: &mut Context, window: &mut Window) -> &mut Window {
    // return (window.DockNodeAsHost && window.DockNodeAsHost->VisibleWindow) ? window.DockNodeAsHost->VisibleWindow : window;
    if window.dock_node_as_host.id != INVALID_ID
        && window.dock_node_as_host.visible_window != INVALID_ID
    {
        g.get_window(window.dock_node_as_host.visible_window)
            .unwrap()
    }
    window
}

// int ImGui::FindWindowDisplayIndex(ImGuiWindow* window)
pub fn find_window_display_index(g: &mut Context, window: &mut Window) -> usize {
    ImGuiContext & g = *GImGui;
    return g.windows.index_from_ptr(g.windows.find(window));
}

// ImGuiWindow* ImGui::FindWindowByName(const char* name)
pub fn find_or_create_window_by_name(g: &mut Context, name: &str) -> (&mut Window, bool)
{
    // ImGuiID id = ImHashStr(name);
    // return FindWindowByID(id);
    for (_, win) in g.windows.iter_mut() {
        if win.name.as_str() == name {
            return (win, false);
        }
    }

    let new_win = Window::new(g, name);
    let new_win_id: Id32 = new_win.id;
    g.windows.insert(new_win.id, new_win);
    (g.windows.get_mut(&new_win_id).unwrap(), true)
}

pub fn find_window_by_name(g: &mut Context, name: &str) -> Option<&mut Window>
{
    for (_, win) in g.windows.iter_mut() {
        if win.name == String::from(name) {
            return Some(win);
        }
    }

    return None;
}
