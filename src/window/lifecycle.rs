use std::collections::HashSet;
use crate::{Context, INVALID_ID, Viewport, window};
use crate::condition::Condition;
use crate::config::ConfigFlags;
use crate::drag_drop::DragDropFlags;
use crate::globals::GImGui;
use crate::types::Id32;
use crate::vectors::vector_2d::Vector2D;
use crate::window::{get, layer, ops, settings, state, Window, WindowFlags, WINDOWS_HOVER_PADDING};
use crate::window::settings::WindowSettings;

/// The reason this is exposed in imgui_internal.h is: on touch-based system that don't have hovering, we want to dispatch inputs to the right target (imgui vs imgui+app)
/// void ImGui::UpdateHoveredWindowAndCaptureFlags()
pub fn update_hovered_window_and_capture_flags(g: &mut Context) {
    // ImGuiContext& g = *GImGui;
    // ImGuiIO& io = g.io;
    let io = &mut g.io;
    g.windows_hover_padding = Vector2D::max(
        g.style.touch_extra_padding,
        Vector2D::new(WINDOWS_HOVER_PADDING, WINDOWS_HOVER_PADDING),
    );

    // Find the window hovered by mouse:
    // - Child windows can extend beyond the limit of their parent so we need to derive HoveredRootWindow from hovered_window.
    // - When moving a window we can skip the search, which also conveniently bypasses the fact that window->WindowRectClipped is lagging as this point of the frame.
    // - We also support the moved window toggling the NoInputs flag after moving has started in order to be able to detect windows below it, which is useful for e.g. docking mechanisms.
    let mut clear_hovered_windows = false;
    get::find_hovered_window(g);
    // IM_ASSERT(g.hovered_window == None || g.hovered_window == g.moving_window || g.hovered_window->Viewport == g.mouse_viewport);

    // Modal windows prevents mouse from hovering behind them.
    // Window* modal_window = get_top_most_popup_modal();
    let modal_window = get_top_most_popup_modal();
    let hov_win = g.window_mut(g.hovered_window_id).unwrap();
    if modal_window
        && hovered_window_id != INVALID_ID
        && !is_window_within_begin_stack_of(
        g.window_mut(g.hovered_window).unwrap().root_window_id,
        modal_window,
        )
    {
        // FIXME-MERGE: root_window_dock_tree ?
        clear_hovered_windows = true;
    }

    // Disabled mouse?
    if io.config_flags.contains(&ConfigFlags::NoMouse) {
        clear_hovered_windows = true;
    }

    // We track click ownership. When clicked outside of a window the click is owned by the application and
    // won't report hovering nor request capture even while dragging over our windows afterward.
    // const bool has_open_popup = (g.OpenPopupStack.Size > 0);
    let has_open_popup = g.open_popup_stack.size > 0;
    let has_open_modal = (modal_window != None);
    let mut mouse_earliest_down = -1;
    let mut mouse_any_down = false;
    // for (int i = 0; i < IM_ARRAYSIZE(io.mouse_down); i += 1)
    for i in 0..io.mouse_down.len() {
        if io.mouse_clicked[i] {
            io.mouse_down_owned[i] = (g.hovered_window_id != INVALID_ID) || has_open_popup;
            io.mouse_down_owned_unless_popup_close[i] =
                (g.hovered_window_id != INVALID_ID) || has_open_modal;
        }
        mouse_any_down |= io.mouse_down[i];
        if (io.mouse_down[i])
            && (mouse_earliest_down == -1
                || io.mouse_clicked_time[i] < io.mouse_clicked_time[mouse_earliest_down])
        {
            mouse_earliest_down = i;
        }
    }
    let mouse_avail = (mouse_earliest_down == -1) || io.mouse_down_owned[mouse_earliest_down];
    let mouse_avail_unless_popup_close =
        (mouse_earliest_down == -1) || io.mouse_down_owned_unless_popup_close[mouse_earliest_down];

    // If mouse was first clicked outside of ImGui bounds we also cancel out hovering.
    // FIXME: For patterns of drag and drop across OS windows, we may need to rework/remove this test (first committed 311c0ca9 on 2015/02)
    let mouse_dragging_extern_payload = g.drag_drop_active
        && (g
            .drag_drop_source_flags
            .contains(&DragDropFlags::SourceExtern));
    if !mouse_avail && !mouse_dragging_extern_payload {
        clear_hovered_windows = true;
    }

    if clear_hovered_windows {
        g.hovered_window_id = INVALID_ID;
        g.hovered_window_under_moving_window_id = INVALID_ID;
    }

    // update io.want_capture_mouse for the user application (true = dispatch mouse info to Dear ImGui only, false = dispatch mouse to Dear ImGui + underlying app)
    // update io.WantCaptureMouseAllowPopupClose (experimental) to give a chance for app to react to popup closure with a drag
    if g.want_capture_mouse_next_frame != -1 {
        io.want_capture_mouse_unless_popup_close = (g.want_capture_mouse_next_frame != 0);
        io.want_capture_mouse = io.want_capture_mouse_unless_popup_close;
    } else {
        io.want_capture_mouse = (mouse_avail
            && (g.hovered_window_id != INVALID_ID || mouse_any_down))
            || has_open_popup;
        io.want_capture_mouse_unless_popup_close = (mouse_avail_unless_popup_close
            && (g.hovered_window_id != INVALID_ID || mouse_any_down))
            || has_open_modal;
    }

    // update io.want_capture_keyboard for the user application (true = dispatch keyboard info to Dear ImGui only, false = dispatch keyboard info to Dear ImGui + underlying app)
    if g.want_capture_keyboard_next_frame != -1 {
        io.want_capture_keyboard = (g.want_capture_keyboard_next_frame != 0);
    } else {
        io.want_capture_keyboard = (g.active_id != 0) || (modal_window != None);
    }
    if io.nav_active
        && (io.config_flags.contains(&ConfigFlags::NavEnableKeyboard))
        && !(io.config_flags.contains(&ConfigFlags::NavNoCaptureKeyboard))
    {
        io.want_capture_keyboard = true;
    }

    // update io.want_text_input flag, this is to allow systems without a keyboard (e.g. mobile, hand-held) to show a software keyboard if possible
    io.want_text_input = if g.want_text_input_next_frame != -1 {
        (g.want_text_input_next_frame != 0)
    } else {
        false
    };
}

// static void UpdateWindowInFocusOrderList(Window* window, bool just_created, WindowFlags new_flags)
pub fn update_window_focus_order_list(
    g: &mut Context,
    window: &mut Window,
    just_created: bool,
    new_flags: &mut HashSet<WindowFlags>,
) {
    // ImGuiContext& g = *GImGui;

    // const bool new_is_explicit_child = (new_flags & WindowFlags::ChildWindow) != 0;
    let new_is_explicit_child = new_flags.contains(&WindowFlags::ChildWindow);
    // const bool child_flag_changed = new_is_explicit_child != window.IsExplicitChild;
    let child_flag_changed = new_is_explicit_child != window.is_explicit_child;
    if (just_created || child_flag_changed) && !new_is_explicit_child {
        // IM_ASSERT(!g.windows_focus_order.contains(window));
        g.windows_focus_order.push_back(window);
        window.focus_order = (g.windows_focus_order.size - 1);
    } else if !just_created && child_flag_changed && new_is_explicit_child {
        // IM_ASSERT(g.windows_focus_order[window.focus_order] == window);
        // for (int n = window.focus_order + 1; n < g.windows_focus_order.size; n += 1)
        for wfo in g.windows_focus_order.iter_mut() {
            // g.windows_focus_order[n] -> FocusOrder - -;
            *wfo = FocusOrder;
            FocusOrder -= 1;
        }
        g.windows_focus_order
            .erase(g.windows_focus_order.data + window.focus_order);
        window.focus_order = -1;
    }
    window.is_explicit_child = new_is_explicit_child;
}

// static Window* CreateNewWindow(const char* name, WindowFlags flags)
pub fn create_new_window(g: &mut Context, name: &str, flags: &mut HashSet<WindowFlags>) -> &mut Window
{
    // ImGuiContext& g = *GImGui;
    //IMGUI_DEBUG_LOG("CreateNewWindow '%s', flags = 0x%08X\n", name, flags);

    // Create window the first time
    // Window* window = IM_NEW(Window)(&g, name);
    let mut window = Window::new(g, name);
    window.flags = flags.clone();
    // TODO: add window to context?
    // g.windows_by_id.SetVoidPtr(window.id, window);


    // Default/arbitrary window position. Use set_next_window_pos() with the appropriate condition flag to change the initial position of a window.
    // const ImGuiViewport* main_viewport = ImGui::get_main_viewport();
   let main_viewport: &mut Viewport = get_main_viewport(g).unwrap();
    window.pos = &main_viewport.pos + Vector2D::new(60.0, 60.0);
    window.viewport_pos = main_viewport.pos.clone();

    // User can disable loading and saving of settings. Tooltip and child windows also don't store settings.
    if !(flags.contains(&WindowFlags::NoSavedSettings)) {
        let settings: Option<&mut WindowSettings> = find_window_settings(g, window.id);
        if settings.is_some(){
            // Retrieve settings from .ini file
            window.settings_offset = g.settings_windows.offset_from_ptr(settings);
            state::set_window_condition_allow_flags(&mut window, &mut HashSet::from([Condition::FirstUseEver]), false);
            settings::apply_window_settings(g, &mut window, &mut(settings.some()));
        }
    }
    window.dc.ideal_max_pos = window.pos.clone();
    window.dc.cursor_max_pos = window.pos.clone();
    window.dc.cursor_start_pos = window.pos.clone(); // So first call to CalcWindowContentSizes() doesn't return crazy values

    // if ((flags & WindowFlags::AlwaysAutoResize) != 0)
    if flags.contains(&WindowFlags::AlwaysAutoResize)
    {
        window.auto_fit_frames_y = 2;
        window.auto_fit_frames_x = 2;
        window.auto_fit_only_grows = false;
    }
    else
    {
        if window.size.x <= 0.0 {
            window.auto_fit_frames_x = 2;
        }
        if window.size.y <= 0.0 {
            window.auto_fit_frames_y = 2;
        }
        window.auto_fit_only_grows = (window.auto_fit_frames_x > 0) || (window.auto_fit_frames_y > 0);
    }

    // if (flags & WindowFlags::NoBringToFrontOnFocus) {
    if flags.contains(&WindowFlags::NoBringToFrontOnFocus) {
        g.windows.push_front(window);
    }// Quite slow but rare and only once
    else {
        g.windows.push_back(window);
    }
    // update_window_in_focus_order_list(window, true, window.flags);
    update_window_focus_order_list(g, &mut window, true, &mut window.flags);

    return &mut window;
}

// void ImGui::update_window_parent_and_root_links(Window* window, WindowFlags flags, Window* parent_window)
pub fn update_window_parent_and_root_links(
    g: &mut Context,
    window: &mut Window,
    flags: &mut HashSet<WindowFlags>,
    parent_window: Option<&mut Window>)
{
    // window.parent_window = parent_window;
    window.parent_window_id = parent_window.id;
    // window.root_window = window.root_window_popup_tree = window.root_window_dock_tree = window.root_window_for_title_bar_highlight = window.root_window_for_nav = window;
    window.root_window_id = window.id;
    window.root_window_popup_tree_id = window.id;
    window.root_window_dock_tree_id = window.id;
    window.root_window_for_title_bar_highlight_id = window.id;
    window.root_window_for_nav_id = window.id;
    // if (parent_window && (flags & WindowFlags::ChildWindow) && !(flags & WindowFlags::Tooltip))
    if parent_window.id != INVALID_ID && flags.contains(&WindowFlags::ChildWindow) && flags.contains(&WindowFlags::Tooltip) == false
    {
        // window.root_window_dock_tree = parent_window.root_window_dock_tree;
        window.root_window_dock_tree_id = parent_window.root_window_dock_tree_id;
        // if !window.dock_is_active && !(parent_window.flags & WindowFlags::DockNodeHost)
        if window.dock_is_active == false && parent.window.flags.contains(&WindowFlags::DockNodeHost) == false
        {
            window.root_window = parent_window.root_window;
        }
    }
    // if parent_window && (flags & WindowFlags::Popup)
    if parent_window.id != INVALID_ID && flags.contains(&WindowFlags::Popup)
    {
        window.root_window_popup_tree_id = parent_window.root_window_popup_tree_id;
    }
    // if (parent_window && !(flags & WindowFlags::Modal) && (flags & (WindowFlags::ChildWindow | WindowFlags::Popup))) // FIXME: simply use _NoTitleBar ?
    if parent_window.id != INVALID_ID && flags.contains(&WindowFlags::Modal) == false && (flags.contains(&WindowFlags::ChildWindow) && flags.contains(&WindowFlags::Popup))
    {
        window.root_window_for_title_bar_highlight_id = parent_window.root_window_for_title_bar_highlight_id;
    }
    // while (window.root_window_for_nav_id.flags & WindowFlags::NavFlattened)
    let mut root_window_for_nav = g.window_mut(window.root_window_for_nav_id).unwrap();
    while root_window_for_nav.flags.contains(&WindowFlags::NavFlattened)
    {
        // IM_ASSERT(window.root_window_for_nav_id.parent_window != None);
        window.root_window_for_nav_id = root_window_for_nav.parent_window_id;
        root_window_for_nav = g.window_mut(window.root_window_for_nav_id).unwrap();
    }
}


// Push a new Dear ImGui window to add widgets to.
// - A default window called "Debug" is automatically stacked at the beginning of every frame so you can use widgets without explicitly calling a Begin/End pair.
// - Begin/End can be called multiple times during the frame with the same window name to append content.
// - The window name is used as a unique identifier to preserve window information across frames (and save rudimentary information to the .ini file).
//   You can use the "##" or "###" markers to use the same label with different id, or same id with different label. See documentation at the top of this file.
// - Return false when window is collapsed, so you can early out in your code. You always need to call ImGui::End() even if false is returned.
// - Passing 'bool* p_open' displays a Close button on the upper-right corner of the window, the pointed value will be set to false when the button is pressed.
// bool ImGui::begin(const char* name, bool* p_open, WindowFlags flags)
pub fn begin(g: &mut Context, name: &str, p_open: Option<&mut bool>, flags: Option<&mut HashSet<WindowFlags>>) -> bool
{
    // ImGuiContext& g = *GImGui;
    // const ImGuiStyle& style = g.style;
    let style: &Style = &g.style;

    // IM_ASSERT(name != None && name[0] != '\0');     // window name required
    // IM_ASSERT(g.within_frame_scope);                  // Forgot to call ImGui::NewFrame()
    // IM_ASSERT(g.frame_count_ended != g.frame_count);   // Called ImGui::Render() or ImGui::EndFrame() and haven't called ImGui::NewFrame() again yet

    // Find or create
    // Window* window = FindWindowByName(name);
    // let (window, window_just_created) = find_or_create_window_by_name(g, name);
    let mut window_opt = get::find_window_by_name(g, name);
    let mut window_just_created = false;
    let mut window: &mut Window = Window::default();
    if window_opt.is_none() {
        window_just_created = true;
        window = create_window(g, name)
    } else {
        window = window_opt.unwrap();
    }
    if window_just_created == false {
        update_window_in_focus_order_list(window, window_just_created, flags);
    }


    // Automatically disable manual moving/resizing when NoInputs is set
    // if ((flags & WindowFlags::NoInputs) == WindowFlags::NoInputs)
    //     flags |= WindowFlags::NoMove | WindowFlags::NoResize;
    if flags.contains(&WindowFlags::NoInputs) {
        flags.insert(WindowFlags::NoMove);
        flags.insert(WindowFlags::NoResize);
    }

    // if (flags & WindowFlags::NavFlattened)
    //     IM_ASSERT(flags & WindowFlags::ChildWindow);

    // let current_frame = g.frame_count;
    let current_frame = g.frame_count;
    // const bool first_begin_of_the_frame = (window.LastFrameActive != current_frame);
    let first_begin_of_the_frame = window.last_frame_active != current_frame;
    // window.is_fallback_window = (g.current_window_stack.size == 0 && g.within_frame_scope_with_implicit_window);
    window.is_fallback_window = g.current_window_stack.is_empty &&
    g.within_frame_scope_with_implicit_window;

    // update the appearing flag (note: the BeginDocked() path may also set this to true later)
    // bool window_just_activated_by_user = (window.LastFrameActive < current_frame - 1); // Not using !was_active because the implicit "Debug" window would always toggle off->on
    let window_just_activated_by_user = window.last_frame_active < current_frame - 1;

    // if (flags & WindowFlags::Popup)
    if flags.contains(&WindowFlags::Popup)
    {
        PopupData& popup_ref = g.open_popup_stack[g.begin_popup_stack.size];
        window_just_activated_by_user |= (window.popup_id != popup_ref.popup_id); // We recycle popups so treat window as activated if popup id changed
        window_just_activated_by_user |= (window != popup_ref.Window);
    }

    // update flags, last_frame_active, BeginOrderXXX fields
    let window_was_appearing = window.Appearing;
    if (first_begin_of_the_frame)
    {
        window.Appearing = window_just_activated_by_user;
        if (window.Appearing)
            state::set_window_condition_allow_flags(window, ImGuiCond_Appearing, true);

        window.FlagsPreviousFrame = window.flags;
        window.flags = (WindowFlags)flags;
        window.last_frame_active = current_frame;
        window.last_time_active = g.time;
        window.BeginOrderWithinParent = 0;
        window.begin_order_within_context = (g.windows_active_count += 1);
    }
    else
    {
        flags = window.flags;
    }

    // Docking
    // (NB: during the frame dock nodes are created, it is possible that (window->dock_is_active == false) even though (window->dock_node->windows.len() > 1)
    // IM_ASSERT(window.dock_node == None || window.DockNodeAsHost == None); // Cannot be both
    if (g.next_window_data.flags & NextWindowDataFlags::HasDock)
        set_window_dock(window, g.next_window_data.dock_id, g.next_window_data.dock_cond);
    if (first_begin_of_the_frame)
    {
        bool has_dock_node = (window.dock_id != 0 || window.dock_node_id != None);
        bool new_auto_dock_node = !has_dock_node && get_window_always_want_own_tab_bar(window);
        bool dock_node_was_visible = window.dock_node_is_visible;
        bool dock_tab_was_visible = window.dock_tab_is_visible;
        if (has_dock_node || new_auto_dock_node)
        {
            BeginDocked(window, p_open);
            flags = window.flags;
            if (window.dock_is_active)
            {
                // IM_ASSERT(window.dock_node != None);
                g.next_window_data.flags &= ~NextWindowDataFlags::HasSizeConstraint; // Docking currently override constraints
            }

            // Amend the appearing flag
            if (window.dock_tab_is_visible && !dock_tab_was_visible && dock_node_was_visible && !window.Appearing && !window_was_appearing)
            {
                window.Appearing = true;
                state::set_window_condition_allow_flags(window, ImGuiCond_Appearing, true);
            }
        }
        else
        {
            window.dock_is_active = window.dock_node_is_visible = window.dock_tab_is_visible = false;
        }
    }

    // Parent window is latched only on the first call to Begin() of the frame, so further append-calls can be done from a different window stack
    Window* parent_window_in_stack = (window.dock_is_active && window.dock_node_id.host_window_id) ? window.dock_node_id.host_window_id: g.current_window_stack.empty() ? None : g.current_window_stack.back().Window;
    Window* parent_window = first_begin_of_the_frame ? ((flags & (WindowFlags::ChildWindow | WindowFlags::Popup)) ? parent_window_in_stack : None) : window.parent_window;
    // IM_ASSERT(parent_window != None || !(flags & WindowFlags::ChildWindow));

    // We allow window memory to be compacted so recreate the base stack when needed.
    if (window.id_stack.size == 0)
        window.id_stack.push_back(window.id);

    // Add to stack
    // We intentionally set g.current_window to None to prevent usage until when the viewport is set, then will call set_current_window()
    g.current_window = window;
    WindowStackData window_stack_data;
    window_stack_data.Window = window;
    window_stack_data.ParentLastItemDataBackup = g.last_item_data;
    window_stack_data.StackSizesOnBegin.set_to_current_state();
    g.current_window_stack.push_back(window_stack_data);
    g.current_window = None;
    if (flags & WindowFlags::ChildMenu)
        g.BeginMenuCount += 1;

    if (flags & WindowFlags::Popup)
    {
        PopupData& popup_ref = g.open_popup_stack[g.begin_popup_stack.size];
        popup_ref.Window = window;
        popup_ref.ParentNavLayer = parent_window_in_stack.DCnav_layer_current;
        g.begin_popup_stack.push_back(popup_ref);
        window.popup_id = popup_ref.popup_id;
    }

    // update ->RootWindow and others pointers (before any possible call to focus_window)
    if (first_begin_of_the_frame)
    {
        update_window_parent_and_root_links(window, flags, parent_window);
        window.ParentWindowInBeginStack = parent_window_in_stack;
    }

    // Process SetNextWindow***() calls
    // (FIXME: Consider splitting the HasXXX flags into x/Y components
    bool window_pos_set_by_api = false;
    bool window_size_x_set_by_api = false, window_size_y_set_by_api = false;
    if (g.next_window_data.flags & NextWindowDataFlags::HasPos)
    {
        window_pos_set_by_api = (window.set_window_pos_allow_flags & g.next_window_data.pos_cond) != 0;
        if (window_pos_set_by_api && ImLengthSqr(g.next_window_data.pos_pivot_val) > 0.00001)
        {
            // May be processed on the next frame if this is our first frame and we are measuring size
            // FIXME: Look into removing the branch so everything can go through this same code path for consistency.
            window.SetWindowPosVal = g.next_window_data.pos_val;
            window.SetWindowPosPivot = g.next_window_data.pos_pivot_val;
            window.set_window_pos_allow_flags &= ~(ImGuiCond_Once | Condition::FirstUseEver | ImGuiCond_Appearing);
        }
        else
        {
            set_window_pos(window, g.next_window_data.pos_val, g.next_window_data.pos_cond);
        }
    }
    if (g.next_window_data.flags & NextWindowDataFlags::HasSize)
    {
        window_size_x_set_by_api = (window.set_window_size_allow_flags & g.next_window_data.sizeCond) != 0 && (g.next_window_data.sizeVal.x > 0.0);
        window_size_y_set_by_api = (window.set_window_size_allow_flags & g.next_window_data.sizeCond) != 0 && (g.next_window_data.sizeVal.y > 0.0);
        set_window_size(window, g.next_window_data.sizeVal, g.next_window_data.sizeCond);
    }
    if (g.next_window_data.flags & NextWindowDataFlags::HasScroll)
    {
        if (g.next_window_data.scroll_val.x >= 0.0)
        {
            window.scroll_target.x = g.next_window_data.scroll_val.x;
            window.scroll_target_center_ratio.x = 0.0;
        }
        if (g.next_window_data.scroll_val.y >= 0.0)
        {
            window.scroll_target.y = g.next_window_data.scroll_val.y;
            window.scroll_target_center_ratio.y = 0.0;
        }
    }
    if (g.next_window_data.flags & NextWindowDataFlags::HasContentSize)
        window.content_size_explicit = g.next_window_data.content_size_val;
    else if (first_begin_of_the_frame)
        window.content_size_explicit = Vector2D::new(0.0, 0.0);
    if (g.next_window_data.flags & NextWindowDataFlags::HasWindowClass)
        window.window_class = g.next_window_data.window_class;
    if (g.next_window_data.flags & NextWindowDataFlags::HasCollapsed)
        SetWindowCollapsed(window, g.next_window_data.collapsed_val, g.next_window_data.CollapsedCond);
    if (g.next_window_data.flags & NextWindowDataFlags::HasFocus)
        layer::focus_window(window);
    if (window.Appearing)
        state::set_window_condition_allow_flags(window, ImGuiCond_Appearing, false);

    // When reusing window again multiple times a frame, just append content (don't need to setup again)
    if (first_begin_of_the_frame)
    {
        // Initialize
        let window_is_child_tooltip = (flags & WindowFlags::ChildWindow) && (flags & WindowFlags::Tooltip); // FIXME-WIP: Undocumented behavior of Child+Tooltip for pinned tooltip (#1345)
        let window_just_appearing_after_hidden_for_resize = (window.hidden_frames_cannot_skip_items > 0);
        window.active = true;
        window.has_close_button = (p_open != None);
        window.clip_rect = Vector4D(-f32::MAX, -f32::MAX, +f32::MAX, +f32::MAX);
        window.id_stack.resize(1);
        window.draw_list->_reset_for_new_frame();
        window.dc.current_tableIdx = -1;
        if (flags & WindowFlags::DockNodeHost)
        {
            window.draw_list.ChannelsSplit(2);
            window.draw_list.channels_set_current(1); // Render decorations on channel 1 as we will render the backgrounds manually later
        }

        // Restore buffer capacity when woken from a compacted state, to avoid
        if (window.memory_compacted)
            GcAwakeTransientWindowBuffers(window);

        // update stored window name when it changes (which can _only_ happen with the "###" operator, so the id would stay unchanged).
        // The title bar always display the 'name' parameter, so we only update the string storage if it needs to be visible to the end-user elsewhere.
        bool window_title_visible_elsewhere = false;
        if ((window.viewport && window.viewport.Window == window) || (window.dock_is_active))
            window_title_visible_elsewhere = true;
        else if (g.nav_windowing_list_window != None && (window.flags & WindowFlags::NoNavFocus) == 0)   // window titles visible when using CTRL+TAB
            window_title_visible_elsewhere = true;
        if (window_title_visible_elsewhere && !window_just_created && strcmp(name, window.name) != 0)
        {
            size_t buf_len = window.nameBufLen;
            window.name = ImStrdupcpy(window.name, &buf_len, name);
            window.nameBufLen = buf_len;
        }

        // UPDATE CONTENTS SIZE, UPDATE HIDDEN STATUS

        // update contents size from last frame for auto-fitting (or use explicit size)
        CalcWindowContentSizes(window, &window.ContentSize, &window.ContentSizeIdeal);

        // FIXME: These flags are decremented before they are used. This means that in order to have these fields produce their intended behaviors
        // for one frame we must set them to at least 2, which is counter-intuitive. hidden_frames_cannot_skip_items is a more complicated case because
        // it has a single usage before this code block and may be set below before it is finally checked.
        if (window..hidden_frames_can_skip_items > 0)
            window..hidden_frames_can_skip_items -= 1 ;
        if (window.hidden_frames_cannot_skip_items > 0)
            window.hidden_frames_cannot_skip_items -= 1 ;
        if (window.hiddenFramesForRenderOnly > 0)
            window.hiddenFramesForRenderOnly -= 1 ;

        // Hide new windows for one frame until they calculate their size
        if (window_just_created && (!window_size_x_set_by_api || !window_size_y_set_by_api))
            window.hidden_frames_cannot_skip_items = 1;

        // Hide popup/tooltip window when re-opening while we measure size (because we recycle the windows)
        // We reset size/content_size for reappearing popups/tooltips early in this function, so further code won't be tempted to use the old size.
        if (window_just_activated_by_user && (flags & (WindowFlags::Popup | WindowFlags::Tooltip)) != 0)
        {
            window.hidden_frames_cannot_skip_items = 1;
            if (flags & WindowFlags::AlwaysAutoResize)
            {
                if (!window_size_x_set_by_api)
                    window.size.x = window.size_full.x = 0.f;
                if (!window_size_y_set_by_api)
                    window.size.y = window.size_full.y = 0.f;
                window.ContentSize = window.ContentSizeIdeal = Vector2D::new(0.f, 0.f);
            }
        }

        // SELECT VIEWPORT
        // We need to do this before using any style/font sizes, as viewport with a different DPI may affect font sizes.

        window_select_viewport(window);
        SetCurrentViewport(window, window.viewport);
        window.FontDpiScale = (g.io.config_flags & ConfigFlags::DpiEnableScaleFonts) ? window.viewport.dpi_scale : 1.0;
        SetCurrentWindow(window);
        flags = window.flags;

        // LOCK BORDER SIZE AND PADDING FOR THE FRAME (so that altering them doesn't cause inconsistencies)
        // We read style data after the call to UpdateSelectWindowViewport() which might be swapping the style.

        if (flags & WindowFlags::ChildWindow)
            window.WindowBorderSize = style.ChildBorderSize;
        else
            window.WindowBorderSize = ((flags & (WindowFlags::Popup | WindowFlags::Tooltip)) && !(flags & WindowFlags::Modal)) ? style.PopupBorderSize : style.WindowBorderSize;
        if (!window.dock_is_active && (flags & WindowFlags::ChildWindow) && !(flags & (WindowFlags::AlwaysUseWindowPadding | WindowFlags::Popup)) && window.WindowBorderSize == 0.0)
            window.window_padding = Vector2D::new(0.0, (flags & WindowFlags::MenuBar) ? style.window_padding.y : 0.0);
        else
            window.window_padding = style.window_padding;

        // Lock menu offset so size calculation can use it as menu-bar windows need a minimum size.
        window.dc.MenuBarOffset.x = ImMax(ImMax(window.window_padding.x, style.item_spacing.x), g.next_window_data.menu_bar_offset_min_val.x);
        window.dc.MenuBarOffset.y = g.next_window_data.menu_bar_offset_min_val.y;

        // Collapse window by double-clicking on title bar
        // At this point we don't have a clipping rectangle setup yet, so we can use the title bar area for hit detection and drawing
        if (!(flags & WindowFlags::NoTitleBar) && !(flags & WindowFlags::NoCollapse) && !window.dock_is_active)
        {
            // We don't use a regular button+id to test for double-click on title bar (mostly due to legacy reason, could be fixed), so verify that we don't have items over the title bar.
            Rect title_bar_rect = window.title_bar_rect();
            if (g.hovered_window == window && g.hovered_id == 0 && g.hovered_id_previous_frame == 0 && is_mouse_hovering_rect(title_bar_rect.min, title_bar_rect.max) && g.io.mouse_clicked_count[0] == 2)
                window.WantCollapseToggle = true;
            if (window.WantCollapseToggle)
            {
                window.collapsed = !window.collapsed;
                mark_ini_settings_dirty(window);
            }
        }
        else
        {
            window.collapsed = false;
        }
        window.WantCollapseToggle = false;

        // SIZE

        // Calculate auto-fit size, handle automatic resize
        const Vector2D size_auto_fit = CalcWindowAutoFitSize(window, window.ContentSizeIdeal);
        bool use_current_size_for_scrollbar_x = window_just_created;
        bool use_current_size_for_scrollbar_y = window_just_created;
        if ((flags & WindowFlags::AlwaysAutoResize) && !window.collapsed)
        {
            // Using SetNextWindowSize() overrides WindowFlags_AlwaysAutoResize, so it can be used on tooltips/popups, etc.
            if (!window_size_x_set_by_api)
            {
                window.size_full.x = size_auto_fit.x;
                use_current_size_for_scrollbar_x = true;
            }
            if (!window_size_y_set_by_api)
            {
                window.size_full.y = size_auto_fit.y;
                use_current_size_for_scrollbar_y = true;
            }
        }
        else if (window.auto_fit_frames_x > 0 || window.auto_fit_frames_y > 0)
        {
            // Auto-fit may only grow window during the first few frames
            // We still process initial auto-fit on collapsed windows to get a window width, but otherwise don't honor WindowFlags_AlwaysAutoResize when collapsed.
            if (!window_size_x_set_by_api && window.auto_fit_frames_x > 0)
            {
                window.size_full.x = window.auto_fit_only_grows ? ImMax(window.size_full.x, size_auto_fit.x) : size_auto_fit.x;
                use_current_size_for_scrollbar_x = true;
            }
            if (!window_size_y_set_by_api && window.auto_fit_frames_y > 0)
            {
                window.size_full.y = window.auto_fit_only_grows ? ImMax(window.size_full.y, size_auto_fit.y) : size_auto_fit.y;
                use_current_size_for_scrollbar_y = true;
            }
            if (!window.collapsed)
                mark_ini_settings_dirty(window);
        }

        // Apply minimum/maximum window size constraints and final size
        window.size_full = calc_window_size_after_constraint(window, window.size_full);
        window.size = window.collapsed && !(flags & WindowFlags::ChildWindow) ? window.title_bar_rect().GetSize() : window.size_full;

        // Decoration size
        let decoration_up_height = window.title_bar_height() + window.MenuBarHeight();

        // POSITION

        // Popup latch its initial position, will position itself when it appears next frame
        if (window_just_activated_by_user)
        {
            window.auto_pos_last_direction = Direction::None;
            if ((flags & WindowFlags::Popup) != 0 && !(flags & WindowFlags::Modal) && !window_pos_set_by_api) // FIXME: begin_popup() could use set_next_window_pos()
                window.pos = g.begin_popup_stack.back().open_popup_pos;
        }

        // Position child window
        if (flags & WindowFlags::ChildWindow)
        {
            // IM_ASSERT(parent_window && parent_window.active);
            window.BeginOrderWithinParent = parent_window.dc.ChildWindows.size;
            parent_window.dc.ChildWindows.push_back(window);
            if (!(flags & WindowFlags::Popup) && !window_pos_set_by_api && !window_is_child_tooltip)
                window.pos = parent_window.dc.cursor_pos;
        }

        let window_pos_with_pivot = (window.SetWindowPosVal.x != f32::MAX && window.hidden_frames_cannot_skip_items == 0);
        if (window_pos_with_pivot)
            set_window_pos(window, window.SetWindowPosVal - window.size * window.SetWindowPosPivot, 0); // Position given a pivot (e.g. for centering)
        else if ((flags & WindowFlags::ChildMenu) != 0)
            window.pos = FindBestWindowPosForPopup(window);
        else if ((flags & WindowFlags::Popup) != 0 && !window_pos_set_by_api && window_just_appearing_after_hidden_for_resize)
            window.pos = FindBestWindowPosForPopup(window);
        else if ((flags & WindowFlags::Tooltip) != 0 && !window_pos_set_by_api && !window_is_child_tooltip)
            window.pos = FindBestWindowPosForPopup(window);

        // Late create viewport if we don't fit within our current host viewport.
        if (window.viewport_allow_platform_monitor_extend >= 0 && !window.viewport_owned && !(window.viewport.flags & ViewportFlags::Minimized))
            if (!window.viewport.get_main_rect().contains(window.rect()))
            {
                // This is based on the assumption that the DPI will be known ahead (same as the DPI of the selection done in UpdateSelectWindowViewport)
                //ImGuiViewport* old_viewport = window->viewport;
                window.viewport = add_update_viewport(window, window.id, window.pos, window.size, ViewportFlags::NoFocusOnAppearing);

                // FIXME-DPI
                //IM_ASSERT(old_viewport->dpi_scale == window->viewport->dpi_scale); // FIXME-DPI: Something went wrong
                SetCurrentViewport(window, window.viewport);
                window.FontDpiScale = (g.io.config_flags & ConfigFlags::DpiEnableScaleFonts) ? window.viewport.dpi_scale : 1.0;
                SetCurrentWindow(window);
            }

        if (window.viewport_owned)
            WindowSyncOwnedViewport(window, parent_window_in_stack);

        // Calculate the range of allowed position for that window (to be movable and visible past safe area padding)
        // When clamping to stay visible, we will enforce that window->pos stays inside of visibility_rect.
        Rect viewport_rect(window.viewport.get_main_rect());
        Rect viewport_work_rect(window.viewport.GetWorkRect());
        Vector2D visibility_padding = ImMax(style.DisplayWindowPadding, style.display_safe_area_padding);
        Rect visibility_rect(viewport_work_rect.min + visibility_padding, viewport_work_rect.max - visibility_padding);

        // Clamp position/size so window stays visible within its viewport or monitor
        // Ignore zero-sized display explicitly to avoid losing positions if a window manager reports zero-sized window when initializing or minimizing.
        // FIXME: Similar to code in GetWindowAllowedExtentRect()
        if (!window_pos_set_by_api && !(flags & WindowFlags::ChildWindow) && window.auto_fit_frames_x <= 0 && window.auto_fit_frames_y <= 0)
        {
            if (!window.viewport_owned && viewport_rect.get_width() > 0 && viewport_rect.get_height() > 0.0)
            {
                ClampWindowRect(window, visibility_rect);
            }
            else if (window.viewport_owned && g.platform_io.monitors.size > 0)
            {
                // Lost windows (e.g. a monitor disconnected) will naturally moved to the fallback/dummy monitor aka the main viewport.
                const platform_monitor* monitor = GetViewportplatform_monitor(window.viewport);
                visibility_rect.min = monitor.work_pos + visibility_padding;
                visibility_rect.max = monitor.work_pos + monitor.work_size - visibility_padding;
                ClampWindowRect(window, visibility_rect);
            }
        }
        window.pos = f32::floor(window.pos);

        // Lock window rounding for the frame (so that altering them doesn't cause inconsistencies)
        // Large values tend to lead to variety of artifacts and are not recommended.
        if (window.viewport_owned || window.dock_is_active)
            window.WindowRounding = 0.0;
        else
            window.WindowRounding = (flags & WindowFlags::ChildWindow) ? style.ChildRounding : ((flags & WindowFlags::Popup) && !(flags & WindowFlags::Modal)) ? style.PopupRounding : style.WindowRounding;

        // For windows with title bar or menu bar, we clamp to FrameHeight(font_size + FramePadding.y * 2.0) to completely hide artifacts.
        //if ((window->flags & WindowFlags_MenuBar) || !(window->flags & WindowFlags::NoTitleBar))
        //    window->window_rounding = ImMin(window->window_rounding, g.font_size + style.FramePadding.y * 2.0);

        // Apply window focus (new and reactivated windows are moved to front)
        bool want_focus = false;
        if (window_just_activated_by_user && !(flags & WindowFlags::NoFocusOnAppearing))
        {
            if (flags & WindowFlags::Popup)
                want_focus = true;
            else if ((window.dock_is_active || (flags & WindowFlags::ChildWindow) == 0) && !(flags & WindowFlags::Tooltip))
                want_focus = true;

            Window* modal = get_top_most_popup_modal();
            if (modal != None && !is_window_within_begin_stack_of(window, modal))
            {
                // Avoid focusing a window that is created outside of active modal. This will prevent active modal from being closed.
                // Since window is not focused it would reappear at the same display position like the last time it was visible.
                // In case of completely new windows it would go to the top (over current modal), but input to such window would still be blocked by modal.
                // Position window behind a modal that is not a begin-parent of this window.
                want_focus = false;
                if (window == window.root_window)
                {
                    Window* blocking_modal = FindBlockingModal(window);
                    // IM_ASSERT(blocking_modal != None);
                    BringWindowToDisplayBehind(window, blocking_modal);
                }
            }
        }

        // [Test Engine] Register whole window in the item system
#ifdef IMGUI_ENABLE_TEST_ENGINE
        if (g.TestEngineHookItems)
        {
            // IM_ASSERT(window.IDStack.size == 1);
            window.id_stack.size = 0;
            IMGUI_TEST_ENGINE_ITEM_ADD(window.rect(), window.id);
            IMGUI_TEST_ENGINE_ITEM_INFO(window.id, window.name, (g.hovered_window == window) ? ItemStatusFlags::HoveredRect : 0);
            window.id_stack.size = 1;
        }


        // Decide if we are going to handle borders and resize grips
        let handle_borders_and_resize_grips = (window.dock_node_as_host_id || !window.dock_is_active);

        // Handle manual resize: Resize Grips, Borders, Gamepad
        int border_held = -1;
        ImU32 resize_grip_col[4] = {};
        let resize_grip_count = g.io.ConfigWindowsResizeFromEdges ? 2 : 1; // Allow resize from lower-left if we have the mouse cursor feedback for it.
        let resize_grip_draw_size = f32::floor(ImMax(g.font_size * 1.10, window.WindowRounding + 1.0 + g.font_size * 0.2));
        if (handle_borders_and_resize_grips && !window.collapsed)
            if (UpdateWindowManualResize(window, size_auto_fit, &border_held, resize_grip_count, &resize_grip_col[0], visibility_rect))
                use_current_size_for_scrollbar_x = use_current_size_for_scrollbar_y = true;
        window.ResizeBorderHeld = (signed char)border_held;

        // Synchronize window --> viewport again and one last time (clamping and manual resize may have affected either)
        if (window.viewport_owned)
        {
            if (!window.viewport.platform_request_move)
                window.viewport.pos = window.pos;
            if (!window.viewport.platform_requsest_resize)
                window.viewport.size = window.size;
            window.viewport.update_work_rect();
            viewport_rect = window.viewport.get_main_rect();
        }

        // Save last known viewport position within the window itself (so it can be saved in .ini file and restored)
        window.viewport_pos = window.viewport.pos;

        // SCROLLBAR VISIBILITY

        // update scrollbar visibility (based on the size that was effective during last frame or the auto-resized size).
        if (!window.collapsed)
        {
            // When reading the current size we need to read it after size constraints have been applied.
            // When we use inner_rect here we are intentionally reading last frame size, same for scrollbar_sizes values before we set them again.
            Vector2D avail_size_from_current_frame = Vector2D::new(window.size_full.x, window.size_full.y - decoration_up_height);
            Vector2D avail_size_from_last_frame = window.inner_rect.GetSize() + window.scrollbar_sizes;
            Vector2D needed_size_from_last_frame = window_just_created ? Vector2D::new(0, 0) : window.ContentSize + window.window_padding * 2.0;
            let size_x_for_scrollbars =  use_current_size_for_scrollbar_x ? avail_size_from_current_frame.x : avail_size_from_last_frame.x;
            let size_y_for_scrollbars =  use_current_size_for_scrollbar_y ? avail_size_from_current_frame.y : avail_size_from_last_frame.y;
            //bool scrollbar_y_from_last_frame = window->scrollbar_y; // FIXME: May want to use that in the scrollbar_x expression? How many pros vs cons?
            window.scrollbar_y = (flags & WindowFlags::AlwaysVerticalScrollbar) || ((needed_size_from_last_frame.y > size_y_for_scrollbars) && !(flags & WindowFlags::NoScrollbar));
            window.scrollbar_x = (flags & WindowFlags::AlwaysHorizontalScrollbar) || ((needed_size_from_last_frame.x > size_x_for_scrollbars - (window.scrollbar_y ? style.scrollbar_size : 0.0)) && !(flags & WindowFlags::NoScrollbar) && (flags & WindowFlags::HorizontalScrollbar));
            if (window.scrollbar_x && !window.scrollbar_y)
                window.scrollbar_y = (needed_size_from_last_frame.y > size_y_for_scrollbars) && !(flags & WindowFlags::NoScrollbar);
            window.scrollbar_sizes = Vector2D::new(window.scrollbar_y ? style.scrollbar_size : 0.0, window.scrollbar_x ? style.scrollbar_size : 0.0);
        }

        // UPDATE RECTANGLES (1- THOSE NOT AFFECTED BY SCROLLING)
        // update various regions. Variables they depends on should be set above in this function.
        // We set this up after processing the resize grip so that our rectangles doesn't lag by a frame.

        // Outer rectangle
        // Not affected by window border size. Used by:
        // - FindHoveredWindow() (w/ extra padding when border resize is enabled)
        // - Begin() initial clipping rect for drawing window background and borders.
        // - Begin() clipping whole child
        const Rect host_rect = ((flags & WindowFlags::ChildWindow) && !(flags & WindowFlags::Popup) && !window_is_child_tooltip) ? parent_window.clip_rect : viewport_rect;
        const Rect outer_rect = window.rect();
        const Rect title_bar_rect = window.title_bar_rect();
        window.OuterRectClipped = outer_rect;
        if (window.dock_is_active)
            window.OuterRectClipped.min.y += window.title_bar_height();
        window.OuterRectClipped.clip_width(host_rect);

        // Inner rectangle
        // Not affected by window border size. Used by:
        // - inner_clip_rect
        // - scroll_to_rect_ex()
        // - NavUpdatePageUpPageDown()
        // - Scrollbar()
        window.inner_rect.min.x = window.pos.x;
        window.inner_rect.min.y = window.pos.y + decoration_up_height;
        window.inner_rect.max.x = window.pos.x + window.size.x - window.scrollbar_sizes.x;
        window.inner_rect.max.y = window.pos.y + window.size.y - window.scrollbar_sizes.y;

        // Inner clipping rectangle.
        // Will extend a little bit outside the normal work region.
        // This is to allow e.g. selectable or CollapsingHeader or some separators to cover that space.
        // Force round operator last to ensure that e.g. (max.x-min.x) in user's render code produce correct result.
        // Note that if our window is collapsed we will end up with an inverted (~null) clipping rectangle which is the correct behavior.
        // Affected by window/frame border size. Used by:
        // - Begin() initial clip rect
        let top_border_size =  (((flags & WindowFlags::MenuBar) || !(flags & WindowFlags::NoTitleBar)) ? style.frame_border_size : window.WindowBorderSize);
        window.InnerClipRect.min.x = f32::floor(0.5 + window.inner_rect.min.x + ImMax(f32::floor(window.window_padding.x * 0.5), window.WindowBorderSize));
        window.InnerClipRect.min.y = f32::floor(0.5 + window.inner_rect.min.y + top_border_size);
        window.InnerClipRect.max.x = f32::floor(0.5 + window.inner_rect.max.x - ImMax(f32::floor(window.window_padding.x * 0.5), window.WindowBorderSize));
        window.InnerClipRect.max.y = f32::floor(0.5 + window.inner_rect.max.y - window.WindowBorderSize);
        window.InnerClipRect.ClipWithFull(host_rect);

        // Default item width. Make it proportional to window size if window manually resizes
        if (window.size.x > 0.0 && !(flags & WindowFlags::Tooltip) && !(flags & WindowFlags::AlwaysAutoResize))
            window.item_width_default = f32::floor(window.size.x * 0.65);
        else
            window.item_width_default = f32::floor(g.font_size * 16.0);

        // SCROLLING

        // Lock down maximum scrolling
        // The value of scroll_max are ahead from scrollbar_x/scrollbar_y which is intentionally using inner_rect from previous rect in order to accommodate
        // for right/bottom aligned items without creating a scrollbar.
        window.scroll_max.x = ImMax(0.0, window.ContentSize.x + window.window_padding.x * 2.0 - window.inner_rect.width());
        window.scroll_max.y = ImMax(0.0, window.ContentSize.y + window.window_padding.y * 2.0 - window.inner_rect.height());

        // Apply scrolling
        window.scroll = calc_next_scroll_from_scroll_target_and_clamp(window);
        window.scroll_target = Vector2D::new(f32::MAX, f32::MAX);

        // DRAWING

        // Setup draw list and outer clipping rectangle
        // IM_ASSERT(window.draw_list.cmd_buffer.size == 1 && window.draw_list.cmd_buffer[0].elem_count == 0);
        window.draw_list.push_texture_id(g.font.container_atlas.TexID);
        push_clip_rect(host_rect.min, host_rect.max, false);

        // Child windows can render their decoration (bg color, border, scrollbars, etc.) within their parent to save a draw call (since 1.71)
        // When using overlapping child windows, this will break the assumption that child z-order is mapped to submission order.
        // FIXME: User code may rely on explicit sorting of overlapping child window and would need to disable this somehow. Please get in contact if you are affected (github #4493)
        let is_undocked_or_docked_visible = !window.dock_is_active || window.dock_tab_is_visible;
        if (is_undocked_or_docked_visible)
        {
            bool render_decorations_in_parent = false;
            if ((flags & WindowFlags::ChildWindow) && !(flags & WindowFlags::Popup) && !window_is_child_tooltip)
            {
                // - We test overlap with the previous child window only (testing all would end up being O(log N) not a good investment here)
                // - We disable this when the parent window has zero vertices, which is a common pattern leading to laying out multiple overlapping childs
                Window* previous_child = parent_window.dc.ChildWindows.size >= 2 ? parent_window.dc.ChildWindows[parent_window.dc.ChildWindows.size - 2] : None;
                bool previous_child_overlapping = previous_child ? previous_child.rect().Overlaps(window.rect()) : false;
                bool parent_is_empty = parent_window.draw_list.vtx_buffer.size > 0;
                if (window.draw_list.cmd_buffer.back().elem_count == 0 && parent_is_empty && !previous_child_overlapping)
                    render_decorations_in_parent = true;
            }
            if (render_decorations_in_parent)
                window.draw_list = parent_window.draw_list;

            // Handle title bar, scrollbar, resize grips and resize borders
            const Window* window_to_highlight = g.nav_windowing_target ? g.nav_windowing_target : g.nav_window;
            let title_bar_is_highlight = want_focus || (window_to_highlight && (window.root_window_for_title_bar_highlight == window_to_highlight.root_window_for_title_bar_highlight || (window.dock_node_id && window.dock_node_id == window_to_highlight.dock_node)));
            RenderWindowDecorations(window, title_bar_rect, title_bar_is_highlight, handle_borders_and_resize_grips, resize_grip_count, resize_grip_col, resize_grip_draw_size);

            if (render_decorations_in_parent)
                window.draw_list = &window.DrawListInst;
        }

        // UPDATE RECTANGLES (2- THOSE AFFECTED BY SCROLLING)

        // Work rectangle.
        // Affected by window padding and border size. Used by:
        // - columns() for right-most edge
        // - TreeNode(), CollapsingHeader() for right-most edge
        // - BeginTabBar() for right-most edge
        let allow_scrollbar_x = !(flags & WindowFlags::NoScrollbar) && (flags & WindowFlags::HorizontalScrollbar);
        let allow_scrollbar_y = !(flags & WindowFlags::NoScrollbar);
        let work_rect_size_x = (window.content_size_explicit.x != 0.0 ? window.content_size_explicit.x : ImMax(allow_scrollbar_x ? window.ContentSize.x : 0.0, window.size.x - window.window_padding.x * 2.0 - window.scrollbar_sizes.x));
        let work_rect_size_y = (window.content_size_explicit.y != 0.0 ? window.content_size_explicit.y : ImMax(allow_scrollbar_y ? window.ContentSize.y : 0.0, window.size.y - window.window_padding.y * 2.0 - decoration_up_height - window.scrollbar_sizes.y));
        window.work_rect.min.x = f32::floor(window.inner_rect.min.x - window.scroll.x + ImMax(window.window_padding.x, window.WindowBorderSize));
        window.work_rect.min.y = f32::floor(window.inner_rect.min.y - window.scroll.y + ImMax(window.window_padding.y, window.WindowBorderSize));
        window.work_rect.max.x = window.work_rect.min.x + work_rect_size_x;
        window.work_rect.max.y = window.work_rect.min.y + work_rect_size_y;
        window.ParentWorkRect = window.work_rect;

        // [LEGACY] Content Region
        // FIXME-OBSOLETE: window->content_region_rect.max is currently very misleading / partly faulty, but some BeginChild() patterns relies on it.
        // Used by:
        // - Mouse wheel scrolling + many other things
        window.content_region_rect.min.x = window.pos.x - window.scroll.x + window.window_padding.x;
        window.content_region_rect.min.y = window.pos.y - window.scroll.y + window.window_padding.y + decoration_up_height;
        window.content_region_rect.max.x = window.content_region_rect.min.x + (window.content_size_explicit.x != 0.0 ? window.content_size_explicit.x : (window.size.x - window.window_padding.x * 2.0 - window.scrollbar_sizes.x));
        window.content_region_rect.max.y = window.content_region_rect.min.y + (window.content_size_explicit.y != 0.0 ? window.content_size_explicit.y : (window.size.y - window.window_padding.y * 2.0 - decoration_up_height - window.scrollbar_sizes.y));

        // Setup drawing context
        // (NB: That term "drawing context / dc" lost its meaning a long time ago. Initially was meant to hold transient data only. Nowadays difference between window-> and window->dc-> is dubious.)
        window.dc.indent.x = 0.0 + window.window_padding.x - window.scroll.x;
        window.dc.GroupOffset.x = 0.0;
        window.dc.columns_offset.x = 0.0;

        // Record the loss of precision of CursorStartPos which can happen due to really large scrolling amount.
        // This is used by clipper to compensate and fix the most common use case of large scroll area. Easy and cheap, next best thing compared to switching everything to double or ImU64.
        double start_pos_highp_x = window.pos.x + window.window_padding.x - window.scroll.x + window.dc.columns_offset.x;
        double start_pos_highp_y = window.pos.y + window.window_padding.y - window.scroll.y + decoration_up_height;
        window.dc.cursor_start_pos  = Vector2D::new(start_pos_highp_x, start_pos_highp_y);
        window.dc.cursor_start_posLossyness = Vector2D::new((start_pos_highp_x - window.dc.cursor_start_pos.x), (start_pos_highp_y - window.dc.cursor_start_pos.y));
        window.dc.cursor_pos = window.dc.cursor_start_pos;
        window.dc.cursor_pos_prev_line = window.dc.cursor_pos;
        window.dc.cursor_max_pos = window.dc.cursor_start_pos;
        window.dc.ideal_max_pos = window.dc.cursor_start_pos;
        window.dc.curr_line_size = window.dc.prev_line_size = Vector2D::new(0.0, 0.0);
        window.dc.curr_line_text_base_offset = window.dc.PrevLineTextBaseOffset = 0.0;
        window.dc.Issame_line = false;

        window.dcnav_layer_current = NavLayer::Main;
        window.dc.nav_layers_active_mask = window.dc.nav_layers_active_mask_next;
        window.dc.nav_hide_highlight_one_frame = false;
        window.dc.nav_has_scroll = (window.scroll_max.y > 0.0);

        window.dc.menu_bar_appending = false;
        window.dc.MenuColumns.Update(style.item_spacing.x, window_just_activated_by_user);
        window.dc.TreeDepth = 0;
        window.dc.TreeJumpToParentOnPopMask = 0x00;
        window.dc.ChildWindows.resize(0);
        window.dc.StateStorage = &window.StateStorage;
        window.dc.current_columns = None;
        window.dc.layout_type = ImGuiLayoutType_Vertical;
        window.dc.ParentLayoutType = parent_window ? parent_window.dc.layout_type : ImGuiLayoutType_Vertical;

        window.dc.item_width = window.item_width_default;
        window.dc.TextWrapPos = -1.0; // disabled
        window.dc.item_width_stack.resize(0);
        window.dc.TextWrapPosStack.resize(0);

        if (window.auto_fit_frames_x > 0)
            window.auto_fit_frames_x--;
        if (window.auto_fit_frames_y > 0)
            window.auto_fit_frames_y--;

        // Apply focus (we need to call focus_window() AFTER setting dc.CursorStartPos so our initial navigation reference rectangle can start around there)
        if (want_focus)
        {
            layer::focus_window(window);
            nav_init_window(window, false); // <-- this is in the way for us to be able to defer and sort reappearing focus_window() calls
        }

        // Close requested by platform window
        if (p_open != None && window.viewport.PlatformRequestClose && window.viewport != get_main_viewport())
        {
            if (!window.dock_is_active || window.dock_tab_is_visible)
            {
                window.viewport.PlatformRequestClose = false;
                g.NavWindowingToggleLayer = false; // Assume user mapped platform_request_close on ALT-F4 so we disable ALT for menu toggle. False positive not an issue.
                IMGUI_DEBUG_LOG_VIEWPORT("[viewport] window '%s' platform_request_close\n", window.name);
                *p_open = false;
            }
        }

        // Title bar
        if (!(flags & WindowFlags::NoTitleBar) && !window.dock_is_active)
            RenderWindowTitleBarContents(window, Rect(title_bar_rect.min.x + window.WindowBorderSize, title_bar_rect.min.y, title_bar_rect.max.x - window.WindowBorderSize, title_bar_rect.max.y), name, p_open);

        // clear hit test shape every frame
        window.hit_test_hole_size.x = window.hit_test_hole_size.y = 0;

        // Pressing CTRL+C while holding on a window copy its content to the clipboard
        // This works but 1. doesn't handle multiple Begin/End pairs, 2. recursing into another Begin/End pair - so we need to work that out and add better logging scope.
        // Maybe we can support CTRL+C on every element?
        /*
        //if (g.nav_window == window && g.active_id == 0)
        if (g.active_id == window->move_id)
            if (g.io.key_ctrl && IsKeyPressedMap(ImGuiKey_C))
                LogToClipboard();
        */

        if (g.io.config_flags & ConfigFlags::DockingEnable)
        {
            // Docking: Dragging a dockable window (or any of its child) turns it into a drag and drop source.
            // We need to do this _before_ we overwrite window->dc.LastItemId below because BeginDockableDragDropSource() also overwrites it.
            if ((g.moving_window == window) && (g.io.config_docking_with_shift == g.io.key_shift))
                if ((window.root_window_dock_tree.flags & WindowFlags::NoDocking) == 0)
                    BeginDockableDragDropSource(window);

            // Docking: Any dockable window can act as a target. For dock node hosts we call begin_dockable_drag_drop_target() in dock_node_update() instead.
            if (g.drag_drop_active && !(flags & WindowFlags::NoDocking))
                if (g.moving_window == None || g.moving_window.root_window_dock_tree != window)
                    if ((window == window.root_window_dock_tree) && !(window.flags & WindowFlags::DockNodeHost))
                        begin_dockable_drag_drop_target(window);
        }

        // We fill last item data based on Title Bar/Tab, in order for IsItemHovered() and IsItemActive() to be usable after Begin().
        // This is useful to allow creating context menus on title bar only, etc.
        if (window.dock_is_active)
            SetLastItemData(window.move_id, g.current_item_flags, window.dock_tab_item_status_flags, window.dock_tab_item_rect);
        else
            SetLastItemData(window.move_id, g.current_item_flags, is_mouse_hovering_rect(title_bar_rect.min, title_bar_rect.max, false) ? ItemStatusFlags::HoveredRect : 0, title_bar_rect);

        // [Test Engine] Register title bar / tab
        if (!(window.flags & WindowFlags::NoTitleBar))
            IMGUI_TEST_ENGINE_ITEM_ADD(g.last_item_data.rect, g.last_item_data.id);
    }
    else
    {
        // Append
        SetCurrentViewport(window, window.viewport);
        SetCurrentWindow(window);
    }

    // Pull/inherit current state
    window.dc.NavFocusScopeIdCurrent = (flags & WindowFlags::ChildWindow) ? parent_window.dc.NavFocusScopeIdCurrent : window.get_id("#FOCUSSCOPE"); // Inherit from parent only // -V595

    if (!(flags & WindowFlags::DockNodeHost))
        push_clip_rect(window.InnerClipRect.min, window.InnerClipRect.max, true);

    // clear 'accessed' flag last thing (After push_clip_rect which will set the flag. We want the flag to stay false when the default "Debug" window is unused)
    window.write_accessed = false;
    window.begin_count += 1;
    g.next_window_data.ClearFlags();

    // update visibility
    if (first_begin_of_the_frame)
    {
        // When we are about to select this tab (which will only be visible on the _next frame_), flag it with a non-zero hidden_frames_cannot_skip_items.
        // This will have the important effect of actually returning true in Begin() and not setting skip_items, allowing an earlier submission of the window contents.
        // This is analogous to regular windows being hidden from one frame.
        // It is especially important as e.g. nested tab_bars would otherwise generate flicker in the form of one empty frame, or focus requests won't be processed.
        if (window.dock_is_active && !window.dock_tab_is_visible)
        {
            if (window.LastFrameJustFocused == g.frame_count)
                window.hidden_frames_cannot_skip_items = 1;
            else
                window..hidden_frames_can_skip_items = 1;
        }

        if (flags & WindowFlags::ChildWindow)
        {
            // Child window can be out of sight and have "negative" clip windows.
            // Mark them as collapsed so commands are skipped earlier (we can't manually collapse them because they have no title bar).
            // IM_ASSERT((flags& WindowFlags::NoTitleBar) != 0 || (window.dock_is_active));
            if (!(flags & WindowFlags::AlwaysAutoResize) && window.auto_fit_frames_x <= 0 && window.auto_fit_frames_y <= 0) // FIXME: Doesn't make sense for ChildWindow??
            {
                let nav_request = (flags & WindowFlags::NavFlattened) && (g.nav_any_request && g.nav_window && g.nav_window.root_window_for_nav == window.root_window_for_nav);
                if (!g.log_enabled && !nav_request)
                    if (window.OuterRectClipped.min.x >= window.OuterRectClipped.max.x || window.OuterRectClipped.min.y >= window.OuterRectClipped.max.y)
                        window..hidden_frames_can_skip_items = 1;
            }

            // Hide along with parent or if parent is collapsed
            if (parent_window && (parent_window.collapsed || parent_window..hidden_frames_can_skip_items > 0))
                window..hidden_frames_can_skip_items = 1;
            if (parent_window && (parent_window.collapsed || parent_window.hidden_frames_cannot_skip_items > 0))
                window.hidden_frames_cannot_skip_items = 1;
        }

        // Don't render if style alpha is 0.0 at the time of Begin(). This is arbitrary and inconsistent but has been there for a long while (may remove at some point)
        if (style.alpha <= 0.0)
            window..hidden_frames_can_skip_items = 1;

        // update the hidden flag
        bool hidden_regular = (window..hidden_frames_can_skip_items > 0) || (window.hidden_frames_cannot_skip_items > 0);
        window.hidden = hidden_regular || (window.hiddenFramesForRenderOnly > 0);

        // Disable inputs for requested number of frames
        if (window.DisableInputsFrames > 0)
        {
            window.DisableInputsFrames--;
            window.flags |= WindowFlags::NoInputs;
        }

        // update the skip_items flag, used to early out of all items functions (no layout required)
        bool skip_items = false;
        if (window.collapsed || !window.active || hidden_regular)
            if (window.auto_fit_frames_x <= 0 && window.auto_fit_frames_y <= 0 && window.hidden_frames_cannot_skip_items <= 0)
                skip_items = true;
        window.skip_items = skip_items;

        // Only clear nav_layers_active_mask_next when marked as visible, so a CTRL+Tab back can use a safe value.
        if (!window.skip_items)
            window.dc.nav_layers_active_mask_next = 0x00;

        // Sanity check: there are two spots which can set appearing = true
        // - when 'window_just_activated_by_user' is set -> hidden_frames_cannot_skip_items is set -> skip_items always false
        // - in BeginDocked() path when DockNodeis_visible == dock_tab_is_visible == true -> hidden _should_ be all zero // FIXME: Not formally proven, hence the assert.
        if (window.skip_items && !window.Appearing)
            // IM_ASSERT(window.Appearing == false); // Please report on GitHub if this triggers: https://github.com/ocornut/imgui/issues/4177
    }

    return !window.skip_items;
}


pub fn end(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();

    // Error checking: verify that user hasn't called End() too many times!
    if (g.current_window_stack.size <= 1 && g.within_frame_scope_with_implicit_window)
    {
        // IM_ASSERT_USER_ERROR(g.current_window_stack.size > 1, "Calling End() too many times!");
        return;
    }
    // IM_ASSERT(g.current_window_stack.size > 0);

    // Error checking: verify that user doesn't directly call End() on a child window.
    if ((window.flags & WindowFlags::ChildWindow) && !(window.flags & WindowFlags::DockNodeHost) && !window.dock_is_active)
        // IM_ASSERT_USER_ERROR(g.within_end_child, "Must call EndChild() and not End()!");

    // Close anything that is open
    if (window.dc.current_columns)
        EndColumns();
    if (!(window.flags & WindowFlags::DockNodeHost))   // Pop inner window clip rectangle
        PopClipRect();

    // Stop logging
    if (!(window.flags & WindowFlags::ChildWindow))    // FIXME: add more options for scope of logging
        LogFinish();

    // Docking: report contents sizes to parent to allow for auto-resize
    if (window.dock_node && window.dock_tab_is_visible)
        if (Window* host_window = window.dock_node.host_window)         // FIXME-DOCK
            host_window.dc.cursor_max_pos = window.dc.cursor_max_pos + window.window_padding - host_window.window_padding;

    // Pop from window stack
    g.last_item_data = g.current_window_stack.back().ParentLastItemDataBackup;
    if (window.flags & WindowFlags::ChildMenu)
        g.BeginMenuCount--;
    if (window.flags & WindowFlags::Popup)
        g.begin_popup_stack.pop_back();
    g.current_window_stack.back().StackSizesOnBegin.compare_with_current_state();
    g.current_window_stack.pop_back();
    SetCurrentWindow(g.current_window_stack.size == 0 ? None : g.current_window_stack.back().Window);
    if (g.current_window)
        SetCurrentViewport(g.current_window, g.current_window.viewport);
}

// static void AddWindowToSortBuffer(ImVector<Window*>* out_sorted_windows, Window* window)
pub fn add_window_to_sort_buffer(g: &mut Context, out_sorted_windows: &Vec<Id32>, window: Id32) {
    out_sorted_windows.push_back(window);
    let win = g.window_mut(window).unwrap();
    if window.active {
        // int count = window.dc.ChildWindows.Size;
        let count = win.dc.child_windows.len();
        // ImQsort(window.dc.ChildWindows.Data, count, sizeof(Window*), ChildWindowComparer);
        win.dc.child_windows.sort();
        for child_win_id in win.dc.child_windows.iter() {
            let child_win = g.window_mut(*child_win_id).unwrap();
            if child_win.active {
                add_window_to_sort_buffer(g, out_sorted_windows, *child_win_id);
            }
        }

        // for (int i = 0; i < count; i += 1)
        // {
        //     Window* child = window.dc.ChildWindows[i];
        //     if (child->Active)
        //         AddWindowToSortBuffer(out_sorted_windows, child);
        // }
    }
}
