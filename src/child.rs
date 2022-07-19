use std::collections::HashSet;
use crate::axis::Axis;
use crate::{Context, orig_imgui_single_file};
use crate::id::set_active_id;
use crate::input::InputSource;
use crate::item::ItemStatusFlags;
use crate::rect::Rect;
use crate::style::{pop_style_color, push_style_color};
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;
use crate::window::WindowFlags;

// bool ImGui::BeginChild(const char* str_id, const Vector2D& size_arg, bool border, ImGuiWindowFlags extra_flags)
pub fn begin_child(g: &mut Context, str_id: &str, size_arg: &Vector2D, border: bool, extra_flags: &mut HashSet<WindowFlags>) -> bool
{
    // ImGuiWindow* window = GetCurrentWindow();
    let window = g.get_current_window().unwrap();
    return begin_child_ex(g, str_id, window.get_id(g, str_id), size_arg, border, extra_flags);
}

// bool ImGui::BeginChild(ImGuiID id, const Vector2D& size_arg, bool border, ImGuiWindowFlags extra_flags)
pub fn begin_child2(g: &mut Context, id: Id32, size_arg: &Vector2D, border: bool, extra_flags: &mut HashSet<WindowFlags>) -> bool
{
    // IM_ASSERT(id != 0);
    return begin_child_ex(g, "", id, size_arg, border, extra_flags);
}

// void ImGui::EndChild()
pub fn end_child(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // ImGuiWindow* window = g.current_window;
    let window = g.get_current_window().unwrap();

    // IM_ASSERT(g.within_end_child == false);
    // IM_ASSERT(window.flags & WindowFlags::ChildWindow);   // Mismatched BeginChild()/EndChild() calls

    g.within_end_child = true;
    if window.begin_count > 1
    {
        end();
    }
    else
    {
        // Vector2D sz = window.size;
        let mut sz = window.size();
        if window.auto_fit_child_axises & (1 << Axis::X) { // Arbitrary minimum zero-ish child size of 4.0 causes less trouble than a 0.0
            sz.x = f32::max(4.0, sz.x);
        }
        if window.auto_fit_child_axises & (1 << Axis::Y) {
            sz.y = f32::max(4.0, sz.y);
        }
        end();

        // ImGuiWindow* parent_window = g.current_window;
        let parent_window = g.get_current_window().unwrap();
        // ImRect bb(parent_window.dc.cursor_pos, parent_window.dc.cursor_pos + sz);
        let mut bb = Rect::new2(&parent_window.dc.cursor_pos, &(&parent_window.dc.cursor_pos + sz));
        item_size(sz);
        if (window.dc.nav_layers_active_mask != 0 || window.dc.nav_has_scroll) && !(window.flags.contains(& WindowFlags::NavFlattened))
        {
            // ItemAdd(bb, windowchild_id);
            item_add(&bb, window.child_id);
            // RenderNavHighlight(bb, windowchild_id);
            render_nav_highlight(&bb, window.child_id);

            // When browsing a window that has no activable items (scroll only) we keep a highlight on the child (pass g.nav_id to trick into always displaying)
            if window.dc.nav_layers_active_mask == 0 && window == g.nav_window {
                render_nav_highlight(Rect::new2(&(bb.min - Vector2D::new(2.0, 2.0)), bb.max + Vector2D::new(2.0, 2.0)), g.nav_id, NavHighlightingFlags::TypeThin);
            }
        }
        else
        {
            // Not navigable into
            item_add(bb, 0);
        }
        if g.get_window(g.hovered_window).unwrap() == window {
            g.last_item_data.status_flags |= ItemStatusFlags::HoveredWindow;
        }
    }
    g.within_end_child = false;
    g.log_line_pos_y = -f32::MAX; // To enforce a carriage return
}

// Helper to create a child window / scrolling region that looks like a normal widget frame.
// bool ImGui::BeginChildFrame(ImGuiID id, const Vector2D& size, ImGuiWindowFlags extra_flags)
pub fn begin_child_frame(g: &mut Context, id: Id32, size: &Vector2D, extra_flags: &mut HashSet<WindowFlags>) -> bool
{
    // ImGuiContext& g = *GImGui;
    // const ImGuiStyle& style = g.style;
    let style = &g.style;
    push_style_color(StyleColor::ChildBg, style.colors[StyleColor::FrameBg]);
    push_style_var(StyleVar::ChildRounding, style.FrameRounding);
    push_style_var(StyleVar::ChildBorderSize, style.frame_border_size);
    push_style_var(StyleVar::WindowPadding, style.frame_padding);
    let mut flags: HashSet<WindowFlags> = HashSet::from([WindowFlags::NoMove, WindowFlags::AlwaysUseWindowPadding]);
    flags.extend(extra_flags.iter());
    let ret = begin_child2(g, id, size, true, &mut flags);
    pop_style_var(3);
    pop_style_color(0);
    return ret;
}

// void ImGui::EndChildFrame()
pub fn end_child_frame(g: &mut Context)
{
    end_child(g);
}

// bool ImGui::begin_child_ex(const char* name, ImGuiID id, const Vector2D& size_arg, bool border, ImGuiWindowFlags flags)
pub fn begin_child_ex(g: &mut Context, name: &str, id: Id32, size_arg: &Vector2D, border: bool, flags: &mut HashSet<WindowFlags>) -> bool
{
    // ImGuiContext& g = *GImGui;
    // ImGuiWindow* parent_window = g.current_window;
    let parent_window = g.get_current_window().unwrap();

    // flags |= WindowFlags::NoTitleBar | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoSavedSettings | WindowFlags::ChildWindow | ImGuiWindowFlags_NoDocking;
    // flags |= (parent_window.flags & ImGuiWindowFlags_NoMove);  // Inherit the NoMove flag
    flags.insert(WindowFlags::NoTitleBar);
    flags.insert(WindowFlags::NoResize);
    flags.insert(WindowFlags::NoSavedSettings);
    flags.insert(WindowFlags::ChildWindow);
    flags.insert(WindowFlags::NoDocking);
    flags.insert(WindowFlags::NoMove);
    flags.extend(parent_window.flags.iter());

    // size
    // const Vector2D content_avail = GetContentRegionAvail();
    let content_avail = get_content_region_avail();
    // Vector2D size = f32::floor(size_arg);
    let mut size = Vector2D::floor(size_arg.clone());
    // const int auto_fit_axises = ((size.x == 0.0) ? (1 << ImGuiAxis_X) : 0x00) | ((size.y == 0.0) ? (1 << ImGuiAxis_Y) : 0x00);
    let auto_fit_axises = (if size.x == 0.0 { 1 << Axis::X} else { 0} )| if size.y == 0.0 { 1 << Axis::Y} else { 0};
    if size.x <= 0.0 {
        size.x = f32::max(content_avail.x + size.x, 4.0);
    }
    // Arbitrary minimum child size (0.0 causing too much issues)
    if size.y <= 0.0 {
        size.y = f32::max(content_avail.y + size.y, 4.0);
    }
    set_next_window_size(size);

    // build up name. If you need to append to a same child from multiple location in the id stack, use BeginChild(ImGuiID id) with a stable value.
    // const char* temp_window_name;
    let mut temp_window_name = String::from("");
    if (name) {
        // ImFormatStringToTempBuffer(&temp_window_name, NULL, "%s/%s_%08X", parent_window.Name, name, id);
        temp_window_name += format!("{}/{}_{:08x}", parent_window.name, name, id).as_str();
    }
    else {
        // ImFormatStringToTempBuffer(&temp_window_name, NULL, "%s/%08X", parent_window.Name, id);
        temp_window_name += format!("{}/{:08x}", parent_window.name, id).as_str();
    }

    // const float backup_border_size = g.style.ChildBorderSize;
    let backup_border_size = g.style.child_border_size;
    if (!border) {
        g.style.child_border_size = 0.0;
    }
    // bool ret = begin(temp_window_name, NULL, flags);
    let ret = begin(temp_window_name, None, flags);
    // g.style.ChildBorderSize = backup_border_size;
    g.style.child_border_size = backup_border_size;

    // ImGuiWindow* child_window = g.current_window;
    let child_window = g.get_current_window().unwrap();
    child_window.child_id = id;
    child_window.auto_fit_child_axises = auto_fit_axises;

    // Set the cursor to handle case where the user called SetNextWindowPos()+BeginChild() manually.
    // While this is not really documented/defined, it seems that the expected thing to do.
    if child_window.begin_count == 1 {
        parent_window.dc.cursor_pos = child_window.Pos;
    }

    // Process navigation-in immediately so NavInit can run on first frame
    if g.nav_activate_id == id && !(flags.contains(&WindowFlags::NavFlattened)) && (child_window.dc.nav_layers_active_mask != 0 || child_window.dc.nav_has_scroll)
    {
        focus_window(child_window);
        nav_init_window(child_window, false);
        set_active_id(g, id + 1, child_window); // Steal active_id with another arbitrary id so that key-press won't activate child item
        g.active_id_source = InputSource::Nav;
    }
    return ret;
}
