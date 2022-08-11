use std::collections::{HashMap, HashSet};
use crate::{Context, INVALID_ID, Viewport};
use crate::condition::Condition;
use crate::config::ConfigFlags;
use crate::content::get_content_region_avail;
use crate::dock::context::{dock_context_add_node, dock_context_find_node_by_id};
use crate::dock::node::dock_node_update;
use crate::dock::node::dock_node_flags::DockNodeFlags;
use crate::dock::node::window::dock_node_setup_host_window;
use crate::globals::GImGui;
use crate::item::item_size;
use crate::nodes::pop_style_var;
use crate::types::Id32;
use crate::vectors::vector_2d::Vector2D;
use crate::viewport::get_main_viewport;
use crate::window::class::WindowClass;
use crate::window::current::get_id;
use crate::window::lifecycle::{begin, end};
use crate::window::next_window::{set_next_window_pos, set_next_window_size, set_next_window_viewport};
use crate::window::WindowFlags;

// Create an explicit dockspace node within an existing window. Also expose dock node flags and creates a central_node by default.
// The Central Node is always displayed even when empty and shrink/extend according to the requested size of its neighbors.
// DockSpace() needs to be submitted _before_ any window they can host. If you use a dockspace, submit it early in your app.
// Id32 DockSpace(Id32 id, const Vector2D& size_arg, ImGuiDockNodeFlags flags, const window_class* window_class)
pub fn dock_space(g: &mut Context, id: Id32, size_arg: &Vector2D, flags: &mut HashSet<DockNodeFlags>, window_class: &WindowClass) -> Id32
{
    // ImGuiContext* g = GImGui;
    // ImGuiContext& g = *.g;
    // Window* window = GetCurrentWindow();
    let window = g.current_window_mut();
    if !(g.io.config_flags.contains(&ConfigFlags::DockingEnable)) {
        return 0;
    }

    // Early out if parent window is hidden/collapsed
    // This is faster but also dock_node_updateTabBar() relies on TabBarLayout() running (which won't if skip_items=true) to set NextSelectedTabId = 0). See #2960.
    // If for whichever reason this is causing problem we would need to ensure that dock_node_updateTabBar() ends up clearing NextSelectedTabId even if skip_items=true.
    if window.skip_items {
        // flags |= DockNodeFlags::KeepAliveOnly;
        flags.insert(DockNodeFlags::KeepAliveOnly);
    }

    // IM_ASSERT((flags & ImGuiDockNodeFlags_DockSpace) == 0);
    // IM_ASSERT(id != 0);
    let mut node = dock_context_find_node_by_id(g, id);
    if node.is_none()
    {
        // IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x%08X created\n", id);
        node = Some(dock_context_add_node(g, id));
        node.unwrap().set_local_flags(&HashSet::from([DockNodeFlags::CentralNode]));
    }
    if window_class.class_id != INVALID_ID && window_class.class_id != node.unwrap().window_class.class_id {}
        // IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x%08X: setup window_class 0x%08X -> 0x%08X\n", id, node.window_class.ClassId, window_class.ClassId);
    node.shared_flags = flags;
    node.window_class = if window_class.class_id != INVALID_ID { window_class } else { WindowClass::default()};

    // When a DockSpace transitioned form implicit to explicit this may be called a second time
    // It is possible that the node has already been claimed by a docked window which appeared before the DockSpace() node, so we overwrite is_dock_space again.
    if node.last_frame_active == g.frame_count && !(flags.contains(&DockNodeFlags::KeepAliveOnly))
    {
        // IM_ASSERT(node.is_dock_space() == false && "Cannot call DockSpace() twice a frame with the same id");
        node.set_local_flags(node.local_flags | DockNodeFlags::DockSpace);
        return id;
    }
    node.set_local_flags(node.local_flags | DockNodeFlags::DockSpace);

    // Keep alive mode, this is allow windows docked into this node so stay docked even if they are not visible
    if flags.contains(&DockNodeFlags::KeepAliveOnly)
    {
        node.last_frame_alive = g.frame_count;
        return id;
    }

    let content_avail = get_content_region_avail(g);
    let mut size = Vector2D::floor(size_arg.clone());
    if size.x <= 0.0 {
        size.x = f32::max(content_avail.x + size.x, 4.0);
    } // Arbitrary minimum child size (0.0 causing too much issues)
    if size.y <= 0.0 {
        size.y = f32::max(content_avail.y + size.y, 4.0);
    }
    // IM_ASSERT(size.x > 0.0 && size.y > 0.0);

    node.pos = window.dc.cursor_pos.clone();
    node.size = size.clone();
    node.size_ref = size.clone();
    set_next_window_pos(g, &node.pos, Condition::None, None);
    set_next_window_size(g, &node.size, Condition::None);
    g.next_window_data.pos_undock = false;

    // FIXME-DOCK: Why do we need a child window to host a dockspace, could we host it in the existing window?
    // FIXME-DOCK: What is the reason for not simply calling BeginChild()? (OK to have a reason but should be commented)
    // WindowFlags window_flags = WindowFlags::ChildWindow | WindowFlags::DockNodeHost;
    let mut window_flags: HashSet<WindowFlags> = HashSet::from([WindowFlags::ChildWindow, WindowFlags::DockNodeHost, WindowFlags::NoSavedSettings, WindowFlags::NoResize, WindowFlags::NoCollapse, WindowFlags::NoTitleBar, WindowFlags::NoScrollbar, WindowFlags::NoScrollWithMouse, WindowFlags::NoBackground]);
    // window_flags |= WindowFlags::NoSavedSettings | WindowFlags::NoResize | WindowFlags::NoCollapse | WindowFlags::NoTitleBar;
    // window_flags |= WindowFlags::NoScrollbar | WindowFlags::NoScrollWithMouse;
    // window_flags |= WindowFlags::NoBackground;

    // char title[256];
    let mut title = String::from("");
    // ImFormatString(title, IM_ARRAYSIZE(title), "%s/DockSpace_%08X", window.name, id);
    title = format!("{}/DockSpace_{}", window.name, id);

    push_style_var(StyleVar::ChildBorderSize, 0.0);
    begin(g, &title, None, Some(&mut window_flags));
    pop_style_var(g, 0);

    // Window* host_window = g.current_window;
    let host_window = g.current_window_mut();
    dock_node_setup_host_window(g,node.unwrap(), host_window);
    host_windowchild_id = window.get_id(g, title.as_str());
    node.only_node_with_windows = None;

    // IM_ASSERT(node.IsRootNode());

    // We need to handle the rare case were a central node is missing.
    // This can happen if the node was first created manually with DockBuilderAddNode() but _without_ the ImGuiDockNodeFlags_Dockspace.
    // Doing it correctly would set the _CentralNode flags, which would then propagate according to subsequent split.
    // It would also be ambiguous to attempt to assign a central node while there are split nodes, so we wait until there's a single node remaining.
    // The specific sub-property of _CentralNode we are interested in recovering here is the "Don't delete when empty" property,
    // as it doesn't make sense for an empty dockspace to not have this property.
    if node.is_leaf_node() && !node.is_central_node() {
        node.set_local_flags(node.local_flags | DockNodeFlags::CentralNode);
    }

    // update the node
    dock_node_update(g, node.unwrap());

    end(g);
    item_size(g,&size,0.0);
    return id;
}

// Tips: Use with ImGuiDockNodeFlags_PassthruCentralNode!
// The limitation with this call is that your window won't have a menu bar.
// Even though we could pass window flags, it would also require the user to be able to call BeginMenuBar() somehow meaning we can't Begin/End in a single function.
// But you can also use BeginMainMenuBar(). If you really want a menu bar inside the same window as the one hosting the dockspace, you will need to copy this code somewhere and tweak it.
// Id32 DockSpaceOverViewport(const ImGuiViewport* viewport, ImGuiDockNodeFlags dockspace_flags, const window_class* window_class)
pub fn dock_space_over_viewport(g: &mut Context, mut viewport: Option<&mut Viewport>, dockspace_flags: &mut HashSet<DockNodeFlags>, window_class: &WindowClass) -> Id32
{
    if viewport.is_none() {
        viewport = Some(get_main_viewport(g));
    }

    set_next_window_pos(g, &viewport.unwrap().work_pos, Condition::None, None);
    set_next_window_size(g, &viewport.work_size, Condition::None);
    set_next_window_viewport(g, viewport.id);

    // WindowFlags host_window_flags = 0;
    let mut host_window_flags: HashSet<WindowFlags> = HashSet::from([
        WindowFlags::NoTitleBar, WindowFlags::NoCollapse, WindowFlags::NoResize, WindowFlags::NoMove, WindowFlags::NoDocking, WindowFlags::NoBringToFrontOnFocus, WindowFlags::NoNavFocus
    ]);
    // host_window_flags |= WindowFlags::NoTitleBar | WindowFlags::NoCollapse | WindowFlags::NoResize | WindowFlags::NoMove | WindowFlags::NoDocking;
    // host_window_flags |= WindowFlags::NoBringToFrontOnFocus | WindowFlags::NoNavFocus;
    if dockspace_flags.contains(&DockNodeFlags::PassthruCentralNode) {
        host_window_flags |= WindowFlags::NoBackground;
    }

    // char label[32];
    // ImFormatString(label, IM_ARRAYSIZE(label), "DockSpaceViewport_%08X", viewport.id);
    let label = format!("DockSpaceViewport_{}", viewport.id);

    push_style_var(StyleVar::WindowRounding, 0.0);
    push_style_var(StyleVar::WindowBorderSize, 0.0);
    push_style_var(StyleVar::WindowPadding, Vector2D::new(0.0, 0.0));
    begin(g, label.as_str(), None, Some(&mut host_window_flags));
    pop_style_var(g, 3);

    let dockspace_id = get_id(g, "DockSpace");
    dock_space(g, dockspace_id, &Vector2D::new(0.0, 0.0), dockspace_flags, window_class);
    end(g);

    return dockspace_id;
}
