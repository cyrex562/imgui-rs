use crate::{Context, Viewport, window};
use crate::condition::Condition;
use crate::config::ConfigFlags;
use crate::dock::context::{dock_context_find_node_by_id, dock_context_gen_node_id};
use crate::dock::node;
use crate::dock::node::{dock_node_get_root_node, DockNodeFlags, DockNodeState, preview, window};
use crate::dock::node::window::dock_node_hide_window_during_host_window_creation;
use crate::drag_drop::DragDropFlags;
use crate::frame::get_frame_height;
use crate::globals::GImGui;
use crate::platform::get_viewport_platform_monitor;
use crate::rect::Rect;
use crate::types::DataAuthority::Window;
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;
use crate::window::next_window::NextWindowDataFlags;
use crate::window::WindowFlags;

// Problem:
//   Undocking a large (~full screen) window would leave it so large that the bottom right sizing corner would more
//   than likely be off the screen and the window would be hard to resize to fit on screen. This can be particularly problematic
//   with 'config_windows_move_from_title_bar_only=true' and/or with 'config_windows_resize_from_edges=false' as well (the later can be
//   due to missing ImGuiBackendFlags_HasMouseCursors backend flag).
// Solution:
//   When undocking a window we currently force its maximum size to 90% of the host viewport or monitor.
// Reevaluate this when we implement preserving docked/undocked size ("docking_wip/undocked_size" branch).
// static Vector2D FixLargeWindowsWhenUndocking(const Vector2D& size, ImGuiViewport* ref_viewport)
pub fn fix_large_windows_when_undocking(g: &mut Context, size: &Vector2D, ref_viewport: Option<&mut Viewport>) -> Vector2D
{
    // if (ref_viewport == NULL)
    //     return size;
    if ref_viewport.is_none() {
        return size.clone();
    }

    // ImGuiContext& g = *GImGui;
    // Vector2D max_size = f32::floor(ref_viewport.work_size * 0.90);
    let max_size = Vector2D::floor(ref_viewport.unwrap().work_size.clone() * 0.90);
    // if (g.config_flags_curr_frame & ConfigFlags::ViewportsEnable)
    if g.config_flags_curr_frame.contains(&ConfigFlags::ViewportsEnable)
    {
        // const ImGuiPlatformMonitor* monitor = GetViewportPlatformMonitor(ref_viewport);
        let monitor = get_viewport_platform_monitor(g, ref_viewport.unwrap());
        let max_size = Vector2D::floor(&monitor.work_size * 0.90);
    }
    return Vector2D::min(size, &max_size);
}

// Compare TabItem nodes given the last known dock_order (will persist in .ini file as hint), used to sort tabs when multiple tabs are added on the same frame.
// static int  TabItemComparerByDockOrder(const void* lhs, const void* rhs)
pub fn tab_item_comparer_by_dock_order(g: &mut Context, lhs: &Vec<u8>, rhs: &Vec<u8>) -> i32
{
    ImGuiWindow* a = ((const ImGuiTabItem*)lhs).Window;
    ImGuiWindow* b = ((const ImGuiTabItem*)rhs).Window;
    if (int d = ((a.dock_order == -1) ? INT_MAX : a.dock_order) - ((b.dock_order == -1) ? INT_MAX : b.dock_order))
        return d;
    return (a.begin_order_within_context - b.begin_order_within_context);
}

// [Internal] Called via SetNextWindowDockID()
// void SetWindowDock(ImGuiWindow* window, ImGuiID dock_id, ImGuiCond cond)
pub fn set_window_dock(g: &mut Context, window: &mut window::Window, dock_id: Id32, cond: Condition)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.set_window_dock_allow_flags & cond) == 0)
        return;
    window.set_window_dock_allow_flags &= ~(ImGuiCond_Once | Cond::FirstUseEver | ImGuiCond_Appearing);

    if (window.dock_id == dock_id)
        return;

    // If the user attempt to set a dock id that is a split node, we'll dig within to find a suitable docking spot
    ImGuiContext* g = GImGui;
    if (ImGuiDockNode* new_node = dock_context_find_node_by_id(g, dock_id))
        if (new_node.is_split_node())
        {
            // Policy: Find central node or latest focused node. We first move back to our root node.
            new_node = dock_node_get_root_node(new_node);
            if (new_node.central_node)
            {
                // IM_ASSERT(new_node.central_node.IsCentralNode());
                dock_id = new_node.central_node.id;
            }
            else
            {
                dock_id = new_node.last_focused_node_id;
            }
        }

    if (window.dock_id == dock_id)
        return;

    if (window.dock_node_id)
        window::dock_node_remove_window(window.dock_node_id, window, 0);
    window.dock_id = dock_id;
}

// bool GetWindowAlwaysWantOwnTabBar(ImGuiWindow* window)
pub fn get_window_always_want_own_tab_bar(g: &mut Context, window: &mut window::Window) -> bool
{
    // ImGuiContext& g = *GImGui;
    if (g.io.config_docking_always_tab_bar || window.window_class.docking_always_tab_bar)
        if ((window.flags & (WindowFlags::ChildWindow | WindowFlags::NoTitleBar | WindowFlags::NoDocking)) == 0)
            if (!window.IsFallbackWindow)    // We don't support AlwaysTabBar on the fallback/implicit window to avoid unused dock-node overhead/noise
                return true;
    return false;
}

// void BeginDocked(ImGuiWindow* window, bool* p_open)
pub fn begin_docked(g: &mut Context, window: &mut window::Window, p_open: &mut bool)
{
    ImGuiContext* g = GImGui;
    // ImGuiContext& g = *.g;

    // clear fields ahead so most early-out paths don't have to do it
    window.dock_is_active = window.dock_node_is_visible = window.dock_tab_is_visible = false;

    const bool auto_dock_node = GetWindowAlwaysWantOwnTabBar(window);
    if (auto_dock_node)
    {
        if (window.dock_id == 0)
        {
            // IM_ASSERT(window.dock_node == NULL);
            window.dock_id = dock_context_gen_node_id(g);
        }
    }
    else
    {
        // Calling set_next_window_pos() undock windows by default (by setting PosUndock)
        bool want_undock = false;
        want_undock |= (window.flags & WindowFlags::NoDocking) != 0;
        want_undock |= (g.next_window_data.flags & NextWindowDataFlags::HasPos) && (window.set_window_pos_allow_flags & g.next_window_data.PosCond) && g.next_window_data.PosUndock;
        if (want_undock)
        {
            dock_context_process_undock_window(g, window);
            return;
        }
    }

    // Bind to our dock node
    ImGuiDockNode* node = window.dock_node_id;
    if (node != NULL)
        // IM_ASSERT(window.DockId == node.ID);
    if (window.dock_id != 0 && node == NULL)
    {
        node = DockContextBindNodeToWindow(g, window);
        if (node == NULL)
            return;
    }

// #if0
    // Undock if the ImGuiDockNodeFlags_NoDockingInCentralNode got set
    if (node.is_central_node && (node.flags & DockNodeFlags::NoDockingInCentralNode))
    {
        dock_context_process_undock_window(g, window);
        return;
    }


    // Undock if our dockspace node disappeared
    // Note how we are testing for last_frame_alive and NOT last_frame_active. A DockSpace node can be maintained alive while being inactive with ImGuiDockNodeFlags_KeepAliveOnly.
    if (node.last_frame_alive < g.frame_count)
    {
        // If the window has been orphaned, transition the docknode to an implicit node processed in DockContextNewFrameUpdateDocking()
        ImGuiDockNode* root_node = dock_node_get_root_node(node);
        if (root_node.last_frame_alive < g.frame_count)
            dock_context_process_undock_window(g, window);
        else
            window.dock_is_active = true;
        return;
    }

    // Store style overrides
    for (int color_n = 0; color_n < WindowDockStyleColor::COUNT; color_n += 1)
        window.dock_style.colors[color_n] = ColorConvertFloat4ToU32(g.style.colors[GWindowDockStyleColors[color_n]]);

    // Fast path return. It is common for windows to hold on a persistent dock_id but be the only visible window,
    // and never create neither a host window neither a tab bar.
    // FIXME-DOCK: replace ->host_window NULL compare with something more explicit (~was initially intended as a first frame test)
    if (node.host_window == NULL)
    {
        if (node.State == DockNodeState::HostWindowHiddenBecauseWindowsAreResizing)
            window.dock_is_active = true;
        if (node.windows.len() > 1)
            dock_node_hide_window_during_host_window_creation(window);
        return;
    }

    // We can have zero-sized nodes (e.g. children of a small-size dockspace)
    // IM_ASSERT(node.host_window);
    // IM_ASSERT(node.IsLeafNode());
    // IM_ASSERT(node.size.x >= 0.0 && node.size.y >= 0.0);
    node.State = DockNodeState::HostWindowVisible;

    // Undock if we are submitted earlier than the host window
    if (!(node.merged_flags & DockNodeFlags::KeepAliveOnly) && window.begin_order_within_context < node.host_window.begin_order_within_context)
    {
        dock_context_process_undock_window(g, window);
        return;
    }

    // Position/size window
    set_next_window_pos(node.pos);
    set_next_window_size(node.size);
    g.next_window_data.PosUndock = false; // Cancel implicit undocking of set_next_window_pos()
    window.dock_is_active = true;
    window.dock_node_is_visible = true;
    window.dock_tab_is_visible = false;
    if (node.merged_flags & DockNodeFlags::KeepAliveOnly)
        return;

    // When the window is selected we mark it as visible.
    if (node.visible_window == window)
        window.dock_tab_is_visible = true;

    // Update window flag
    // IM_ASSERT((window.flags & WindowFlags::ChildWindow) == 0);
    window.flags |= WindowFlags::ChildWindow | WindowFlags::AlwaysUseWindowPadding | WindowFlags::NoResize;
    if (node.is_hidden_tab_bar() || node.is_no_tab_bar())
        window.flags |= WindowFlags::NoTitleBar;
    else
        window.flags &= ~WindowFlags::NoTitleBar;      // clear the NoTitleBar flag in case the user set it: confusingly enough we need a title bar height so we are correctly offset, but it won't be displayed!

    // Save new dock order only if the window has been visible once already
    // This allows multiple windows to be created in the same frame and have their respective dock orders preserved.
    if (node.tab_bar && window.was_active)
        window.dock_order = DockNodeGetTabOrder(window);

    if ((node.want_close_all || node.want_close_tab_id == window.tab_id) && p_open != NULL)
        *p_open = false;

    // Update child_id to allow returning from Child to Parent with Escape
    ImGuiWindow* parent_window = window.dock_node_id.host_window_id;
    windowchild_id = parent_window.get_id(window.name);
}

// void BeginDockableDragDropSource(ImGuiWindow* window)
pub fn begin_dockable_drag_drop_source(g: &mut Context, window: &mut window::Window)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.active_id == window.move_id);
    // IM_ASSERT(g.moving_window == window);
    // IM_ASSERT(g.current_window == window);

    g.last_item_data.id = window.move_id;
    window = window.root_window_dock_tree;
    // IM_ASSERT((window.flags & WindowFlags::NoDocking) == 0);
    bool is_drag_docking = (g.io.config_docking_with_shift) || Rect(0, 0, window.size_full.x, get_frame_height()).Contains(g.active_id_click_offset); // FIXME-DOCKING: Need to make this stateful and explicit
    if (is_drag_docking && BeginDragDropSource(DragDropFlags::SourceNoPreviewTooltip | ImGuiDragDropFlags_SourceNoHoldToOpenOthers | ImGuiDragDropFlags_SourceAutoExpirePayload))
    {
        SetDragDropPayload(IMGUI_PAYLOAD_TYPE_WINDOW, &window, sizeof(window));
        EndDragDropSource();

        // Store style overrides
        for (int color_n = 0; color_n < WindowDockStyleColor::COUNT; color_n += 1)
            window.dock_style.colors[color_n] = ColorConvertFloat4ToU32(g.style.colors[GWindowDockStyleColors[color_n]]);
    }
}

// void begin_dockable_drag_drop_target(ImGuiWindow* window)
pub fn begin_dockable_drag_drop_target(g: &mut Context, window: &mut window::Window)
{
    ImGuiContext* g = GImGui;
    // ImGuiContext& g = *.g;

    //IM_ASSERT(window->root_window_dock_tree == window); // May also be a DockSpace
    // IM_ASSERT((window.flags & WindowFlags::NoDocking) == 0);
    if (!g.drag_drop_active)
        return;
    //GetForegroundDrawList(window)->add_rect(window->pos, window->pos + window->size, IM_COL32(255, 255, 0, 255));
    if (!BeginDragDropTargetCustom(window.Rect(), window.id))
        return;

    // Peek into the payload before calling AcceptDragDropPayload() so we can handle overlapping dock nodes with filtering
    // (this is a little unusual pattern, normally most code would call AcceptDragDropPayload directly)
    const ImGuiPayload* payload = &g.drag_drop_payload;
    if (!payload.is_data_type(IMGUI_PAYLOAD_TYPE_WINDOW) || !dock_node_is_drop_allowed(window, *(ImGuiWindow**)payload.Data))
    {
        EndDragDropTarget();
        return;
    }

    ImGuiWindow* payload_window = *(ImGuiWindow**)payload.Data;
    if (AcceptDragDropPayload(IMGUI_PAYLOAD_TYPE_WINDOW, ImGuiDragDropFlags_AcceptBeforeDelivery | ImGuiDragDropFlags_AcceptNoDrawDefaultRect))
    {
        // Select target node
        // (Important: we cannot use g.hovered_dock_node here! Because each of our target node have filters based on payload, each candidate drop target will do its own evaluation)
        bool dock_into_floating_window = false;
        ImGuiDockNode* node = NULL;
        if (window.dock_node_as_host_id)
        {
            // Cannot assume that node will != NULL even though we passed the rectangle test: it depends on padding/spacing handled by DockNodeTreeFindVisibleNodeByPos().
            node = dock_node_tree_find_visible_node_by_pos(window.dock_node_as_host_id, g.io.mouse_pos);

            // There is an edge case when docking into a dockspace which only has _inactive_ nodes (because none of the windows are active)
            // In this case we need to fallback into any leaf mode, possibly the central node.
            // FIXME-20181220: We should not have to test for is_leaf_node() here but we have another bug to fix first.
            if (node && node.is_dock_space() && node.is_root_node())
                node = (node.central_node && node.is_leaf_node()) ? node.central_node : DockNodeTreeFindFallbackLeafNode(node);
        }
        else
        {
            if (window.dock_node_id)
                node = window.dock_node_id;
            else
                dock_into_floating_window = true; // Dock into a regular window
        }

        const Rect explicit_target_rect = (node && node.tab_bar && !node.is_hidden_tab_bar() && !node.is_no_tab_bar()) ? node.tab_bar.BarRect : Rect(window.pos, window.pos + Vector2D::new(window.size.x, get_frame_height()));
        const bool is_explicit_target = g.io.config_docking_with_shift || IsMouseHoveringRect(explicit_target_rect.min, explicit_target_rect.max);

        // preview docking request and find out split direction/ratio
        //const bool do_preview = true;     // Ignore testing for payload->is_preview() which removes one frame of delay, but breaks overlapping drop targets within the same window.
        const bool do_preview = payload.IsPreview() || payload.IsDelivery();
        if (do_preview && (node != NULL || dock_into_floating_window))
        {
            ImGuiDockPreviewData split_inner;
            ImGuiDockPreviewData split_outer;
            ImGuiDockPreviewData* split_data = &split_inner;
            if (node && (node.parent_node || node.is_central_node()))
                if (ImGuiDockNode* root_node = dock_node_get_root_node(node))
                {
                    preview::dock_node_preview_dock_setup(window, root_node, payload_window, &split_outer, is_explicit_target, true);
                    if (split_outer.is_split_dir_explicit)
                        split_data = &split_outer;
                }
            preview::dock_node_preview_dock_setup(window, node, payload_window, &split_inner, is_explicit_target, false);
            if (split_data == &split_outer)
                split_inner.is_drop_allowed = false;

            // Draw inner then outer, so that previewed tab (in inner data) will be behind the outer drop boxes
            DockNodePreviewDockRender(window, node, payload_window, &split_inner);
            DockNodePreviewDockRender(window, node, payload_window, &split_outer);

            // Queue docking request
            if (split_data.is_drop_allowed && payload.IsDelivery())
                DockContextQueueDock(g, window, split_data.split_node, payload_window, split_data.split_dir, split_data.split_ratio, split_data == &split_outer);
        }
    }
    EndDragDropTarget();
}
