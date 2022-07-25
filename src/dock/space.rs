use std::collections::HashSet;
use crate::{Context, Viewport};
use crate::dock::context::{dock_context_add_node, dock_context_find_node_by_id};
use crate::dock::node::DockNodeFlags;
use crate::globals::GImGui;
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;
use crate::window::class::WindowClass;
use crate::window::WindowFlags;

// Create an explicit dockspace node within an existing window. Also expose dock node flags and creates a central_node by default.
// The Central Node is always displayed even when empty and shrink/extend according to the requested size of its neighbors.
// DockSpace() needs to be submitted _before_ any window they can host. If you use a dockspace, submit it early in your app.
// ImGuiID DockSpace(ImGuiID id, const Vector2D& size_arg, ImGuiDockNodeFlags flags, const ImGuiWindowClass* window_class)
pub fn dock_space(g: &mut Context, id: Id32, size_arg: &Vector2D, flags: &mut HashSet<DockNodeFlags>, window_class: &WindowClass) -> Id32
{
    ImGuiContext* .g = GImGui;
    // ImGuiContext& g = *.g;
    ImGuiWindow* window = GetCurrentWindow();
    if (!(g.io.config_flags & ImGuiConfigFlags_DockingEnable))
        return 0;

    // Early out if parent window is hidden/collapsed
    // This is faster but also DockNodeUpdateTabBar() relies on TabBarLayout() running (which won't if skip_items=true) to set NextSelectedTabId = 0). See #2960.
    // If for whichever reason this is causing problem we would need to ensure that DockNodeUpdateTabBar() ends up clearing NextSelectedTabId even if skip_items=true.
    if (window.skip_items)
        flags |= DockNodeFlags::KeepAliveOnly;

    // IM_ASSERT((flags & ImGuiDockNodeFlags_DockSpace) == 0);
    // IM_ASSERT(id != 0);
    ImGuiDockNode* node = dock_context_find_node_by_id(.g, id);
    if (!node)
    {
        // IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x%08X created\n", id);
        node = dock_context_add_node(.g, id);
        node.set_local_flags(DockNodeFlags::CentralNode);
    }
    if (window_class && window_class.ClassId != node.window_class.ClassId)
        // IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x%08X: setup window_class 0x%08X -> 0x%08X\n", id, node.window_class.ClassId, window_class.ClassId);
    node.shared_flags = flags;
    node.window_class = window_class ? *window_class : ImGuiWindowClass();

    // When a DockSpace transitioned form implicit to explicit this may be called a second time
    // It is possible that the node has already been claimed by a docked window which appeared before the DockSpace() node, so we overwrite is_dock_space again.
    if (node.last_frame_active == g.frame_count && !(flags & DockNodeFlags::KeepAliveOnly))
    {
        // IM_ASSERT(node.is_dock_space() == false && "Cannot call DockSpace() twice a frame with the same id");
        node.set_local_flags(node.LocalFlags | DockNodeFlags::DockSpace);
        return id;
    }
    node.set_local_flags(node.LocalFlags | DockNodeFlags::DockSpace);

    // Keep alive mode, this is allow windows docked into this node so stay docked even if they are not visible
    if (flags & DockNodeFlags::KeepAliveOnly)
    {
        node.LastFrameAlive = g.frame_count;
        return id;
    }

    const Vector2D content_avail = GetContentRegionAvail();
    Vector2D size = f32::floor(size_arg);
    if (size.x <= 0.0)
        size.x = ImMax(content_avail.x + size.x, 4.0); // Arbitrary minimum child size (0.0 causing too much issues)
    if (size.y <= 0.0)
        size.y = ImMax(content_avail.y + size.y, 4.0);
    // IM_ASSERT(size.x > 0.0 && size.y > 0.0);

    node.pos = window.dc.cursor_pos;
    node.size = node.size_ref = size;
    SetNextWindowPos(node.pos);
    set_next_window_size(node.size);
    g.next_window_data.PosUndock = false;

    // FIXME-DOCK: Why do we need a child window to host a dockspace, could we host it in the existing window?
    // FIXME-DOCK: What is the reason for not simply calling BeginChild()? (OK to have a reason but should be commented)
    ImGuiWindowFlags window_flags = WindowFlags::ChildWindow | WindowFlags::DockNodeHost;
    window_flags |= WindowFlags::NoSavedSettings | WindowFlags::NoResize | WindowFlags::NoCollapse | WindowFlags::NoTitleBar;
    window_flags |= WindowFlags::NoScrollbar | WindowFlags::NoScrollWithMouse;
    window_flags |= WindowFlags::NoBackground;

    char title[256];
    ImFormatString(title, IM_ARRAYSIZE(title), "%s/DockSpace_%08X", window.Name, id);

    push_style_var(StyleVar::ChildBorderSize, 0.0);
    begin(title, NULL, window_flags);
    pop_style_var();

    ImGuiWindow* host_window = g.current_window;
    DockNodeSetupHostWindow(node, host_window);
    host_windowchild_id = window.get_id(title);
    node.only_node_with_windows = NULL;

    // IM_ASSERT(node.IsRootNode());

    // We need to handle the rare case were a central node is missing.
    // This can happen if the node was first created manually with DockBuilderAddNode() but _without_ the ImGuiDockNodeFlags_Dockspace.
    // Doing it correctly would set the _CentralNode flags, which would then propagate according to subsequent split.
    // It would also be ambiguous to attempt to assign a central node while there are split nodes, so we wait until there's a single node remaining.
    // The specific sub-property of _CentralNode we are interested in recovering here is the "Don't delete when empty" property,
    // as it doesn't make sense for an empty dockspace to not have this property.
    if (node.is_leaf_node() && !node.is_central_node())
        node.set_local_flags(node.LocalFlags | DockNodeFlags::CentralNode);

    // Update the node
    DockNodeUpdate(node);

    end();
    item_size(size);
    return id;
}

// Tips: Use with ImGuiDockNodeFlags_PassthruCentralNode!
// The limitation with this call is that your window won't have a menu bar.
// Even though we could pass window flags, it would also require the user to be able to call BeginMenuBar() somehow meaning we can't Begin/End in a single function.
// But you can also use BeginMainMenuBar(). If you really want a menu bar inside the same window as the one hosting the dockspace, you will need to copy this code somewhere and tweak it.
// ImGuiID DockSpaceOverViewport(const ImGuiViewport* viewport, ImGuiDockNodeFlags dockspace_flags, const ImGuiWindowClass* window_class)
pub fn dock_space_over_viewport(g: &mut Context, viewport: &mut Viewport, dockspace_flags: &HashSet<DockNodeFlags>, window_class: &WindowClass) -> Id32
{
    if (viewport == NULL)
        viewport = GetMainViewport();

    SetNextWindowPos(viewport.WorkPos);
    set_next_window_size(viewport.work_size);
    SetNextWindowViewport(viewport.id);

    ImGuiWindowFlags host_window_flags = 0;
    host_window_flags |= WindowFlags::NoTitleBar | WindowFlags::NoCollapse | WindowFlags::NoResize | WindowFlags::NoMove | WindowFlags::NoDocking;
    host_window_flags |= WindowFlags::NoBringToFrontOnFocus | WindowFlags::NoNavFocus;
    if (dockspace_flags & DockNodeFlags::PassthruCentralNode)
        host_window_flags |= WindowFlags::NoBackground;

    char label[32];
    ImFormatString(label, IM_ARRAYSIZE(label), "DockSpaceViewport_%08X", viewport.id);

    push_style_var(StyleVar::WindowRounding, 0.0);
    push_style_var(StyleVar::WindowBorderSize, 0.0);
    push_style_var(StyleVar::WindowPadding, Vector2D::new(0.0, 0.0));
    begin(label, NULL, host_window_flags);
    pop_style_var(3);

    ImGuiID dockspace_id = GetID("DockSpace");
    DockSpace(dockspace_id, Vector2D::new(0.0, 0.0), dockspace_flags, window_class);
    end();

    return dockspace_id;
}
