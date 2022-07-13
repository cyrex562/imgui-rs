use std::collections::HashSet;
use crate::condition::Cond;
use crate::context::{call_context_hooks, Context, ContextHookType};
use crate::draw_list::DrawListFlags;
use crate::rect::Rect;
use crate::types::INVALID_ID;
use crate::vectors::Vector2D;

/// Helper: Execute a block of code at maximum once a frame. Convenient if you want to quickly create an UI within deep-nested code that runs multiple times every frame.
/// Usage: static ImGuiOnceUponAFrame oaf; if (oaf) ImGui::Text("This will be called only once per frame");
#[derive(Default,Debug,Clone,PartialEq)]
pub struct ImGuiOnceUponAFrame
{
    pub ref_frame: i32,
    // ImGuiOnceUponAFrame() { ref_frame = -1; }
    // mutable int ref_frame;
    // operator bool() const { int current_frame = ImGui::GetFrameCount(); if (ref_frame == current_frame) return false; ref_frame = current_frame; return true; }
}

impl ImGuiOnceUponAFrame {
    pub fn new() -> Self {
        Self {
            ref_frame: -1,
        }
    }
}

// void ImGui::NewFrame()
pub fn new_frame(g: &mut Context)
{
    // IM_ASSERT(GImGui != NULL && "No current context. Did you call ImGui::CreateContext() and ImGui::SetCurrentContext() ?");
    // ImGuiContext& g = *GImGui;

    // Remove pending delete hooks before frame start.
    // This deferred removal avoid issues of removal while iterating the hook vector
    // for (int n = g.Hooks.Size - 1; n >= 0; n--)
    for n in g.hooks.size - 1 .. 0
    {
        if g.hooks[n].hook_type == ContextHookType::PendingRemoval {
            g.hooks.erase(&g.hooks[n]);
        }
    }

    call_context_hooks(g, ContextHookType::NewFramePre);

    // Check and assert for various common io and Configuration mistakes
    g.config_flags_last_frame = g.config_flags_curr_frame.clone();
    error_check_new_frame_sanity_checks();
    g.config_flags_curr_frame = g.io.config_flags.clone();

    // Load settings on first frame, save settings when modified (after a delay)
    update_settings();

    g.time += g.io.delta_time;
    g.within_frame_scope = true;
    g.frame_count += 1;
    g.tool_tip_override_count = 0;
    g.windows_active_count = 0;
    g.menus_id_submitted_this_frame.clear();

    // Calculate frame-rate for the user, as a purely luxurious feature
    g.frame_rate_sec_per_frame_accum += g.io.delta_time - g.frame_rate_sec_per_frame[g.frame_rate_sec_per_frame_idx];
    g.frame_rate_sec_per_frame[g.frame_rate_sec_per_frame_idx] = g.io.delta_time;
    g.frame_rate_sec_per_frame_idx = (g.frame_rate_sec_per_frame_idx + 1) % (g.frame_rate_sec_per_frame.len());
    g.frame_rate_sec_per_frame_count = f32::min(g.frame_rate_sec_per_frame_count + 1, (g.frame_rate_sec_per_frame.len()));
    g.io.frame_rate = if(g.frame_rate_sec_per_frame_accum > 0.0) { (1.0 / (g.frame_rate_sec_per_frame_accum / g.frame_rate_sec_per_frame_count)) } else {f32::FLT_MAX};

    update_viewports_new_frame();

    // Setup current font and draw list shared data
    // FIXME-VIEWPORT: the concept of a single ClipRectFullscreen is not ideal!
    g.io.fonts.locked = true;
    sec_current_font(get_default_font());
    // IM_ASSERT(g.Font->IsLoaded());
    let mut virtual_space = Rect::new4(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    // for (int n = 0; n < g.Viewports.Size; n += 1){
    for n in 0 .. g.viewports.len() {
        virtual_space.add_rect(g.viewports[n].get_main_rect());
    }
    g.draw_list_shared_data.clip_rect_full_screen = virtual_space.to_vector_4d();
    g.draw_list_shared_data.curve_tessellation_tol = g.style.curve_tessellation_tol;
    g.draw_list_shared_data.set_circle_tessellation_max_error(g.style.circle_tessellation_max_error);
    g.draw_list_shared_data.initial_flags = HashSet::new();
    if g.style.anti_aliased_lines {
        g.draw_list_shared_data.initial_flags.insert(DrawListFlags::AntiAliasedLines);
    }
    if g.style.anti_aliased_lines_use_tex && !(g.font.container_atlas.flags.contains(FontAtlasFlags::NoBakedLines)) {
g.draw_list_shared_data.initial_flags.insert(DrawListFlags::AntiAliasedLinesUseTex);}
    if g.style.anti_aliased_fill {
        g.draw_list_shared_data.initial_flags.insert(DrawListFlags::AntiAliasedFill);
    }
    if g.io.backend_flags.contains(BackendFlags::RendererHasVxtOffset) {
        g.draw_list_shared_data.initial_flags.insert(DrawListFlags::AllowVtxOffset);
    }

    // Mark rendering data as invalid to prevent user who may have a handle on it to use it.
    // for (int n = 0; n < g.viewports.Size; n += 1)
    for i in 0 .. g.viewports.len()
    {
        // ImGuiViewportP* viewport = g.viewports[n];
        let mut viewport = g.viewports[n];
        viewport.draw_data.clear();
        // viewport->DrawDataP.Clear();
    }

    // Drag and drop keep the source id alive so even if the source disappear our state is consistent
    if g.drag_drop_active && g.drag_drop_payload.source_id == g.active_id {
        keep_alive_id(g.drag_drop_payload.source_id);
    }

    // Update hovered_id data
    if !g.hovered_id_previous_frame {
        g.hovered_id_timer = 0.0;
    }
    if (g.hovered_id_previous_frame == INVALID_ID || (g.hovered_id != INVALID_ID && g.active_id == g.hovered_id)) {
        g.hovered_id_not_active_timer = 0.0;
    }
    if g.hovered_id {
        g.hovered_id_timer += g.io.delta_time;
    }
    if (g.hovered_id != INVALID_ID && g.active_id != g.hovered_id) {
        g.hovered_id_not_active_timer += g.io.delta_time;
    }
    g.hovered_id_previous_frame = g.hovered_id;
    g.hovered_id_previous_frame_using_mouse_wheel = g.hovered_id_using_mouse_wheel;
    g.hovered_id = 0;
    g.hovered_id_allow_overlap = false;
    g.hovered_id_using_mouse_wheel = false;
    g.hovered_id_disabled = false;

    // clear ActiveID if the item is not alive anymore.
    // In 1.87, the common most call to keep_alive_id() was moved from GetID() to ItemAdd().
    // As a result, custom widget using ButtonBehavior() _without_ ItemAdd() need to call keep_alive_id() themselves.
    if (g.active_id != 0 && g.active_id_is_alive != g.active_id && g.active_id_previous_frame == g.active_id)
    {
        // IMGUI_DEBUG_LOG_ACTIVEID("NewFrame(): ClearActiveID() because it isn't marked alive anymore!\n");
        clear_active_id();
    }

    // Update active_id data (clear reference to active widget if the widget isn't alive anymore)
    if (g.active_id) {
        g.active_id_timer += g.io.delta_time;
    }
    g.last_active_id_timer += g.io.delta_time;
    g.active_id_previous_frame = g.active_id;
    g.active_id_previous_frame_window_id = g.active_id_window_id;
    g.active_id_previous_frame_has_been_edited_before = g.active_id_has_been_edited_before;
    g.active_id_is_alive = 0;
    g.active_id_has_been_edited_this_frame = false;
    g.active_id_previous_frame_is_alive = false;
    g.active_id_is_just_activated = false;
    if (g.temp_input_id != 0 && g.active_id != g.temp_input_id) {
        g.temp_input_id = 0;
    }
    if (g.active_id == 0)
    {
        g.active_id_using_nav_dir_mask = 0x00;
        g.active_id_using_nav_input_mask = 0x00;
        g.active_id_using_key_input_mask.ClearAllBits();
    }

    // Drag and drop
    g.drag_drop_accept_id_prev = g.drag_drop_accept_id_curr;
    g.drag_drop_accept_id_curr = 0;
    g.drag_drop_accept_id_curr_rect_surface = f32::MAX;
    g.drag_drop_within_source = false;
    g.drag_drop_within_target = false;
    g.drag_drop_hold_just_pressed_id = 0;

    // Close popups on focus lost (currently wip/opt-in)
    //if (g.io.app_focus_lost)
    //    ClosePopupsExceptModals();

    // Process input queue (trickle as many events as possible)
    g.input_events_trail.resize(0);
    update_input_events(g.io.config_input_trickle_event_queue);

    // Update keyboard input state
    update_keyboard_inputs();

    //IM_ASSERT(g.io.key_ctrl == IsKeyDown(ImGuiKey_LeftCtrl) || IsKeyDown(ImGuiKey_RightCtrl));
    //IM_ASSERT(g.io.key_shift == IsKeyDown(ImGuiKey_LeftShift) || IsKeyDown(ImGuiKey_RightShift));
    //IM_ASSERT(g.io.key_alt == IsKeyDown(ImGuiKey_LeftAlt) || IsKeyDown(ImGuiKey_RightAlt));
    //IM_ASSERT(g.io.key_super == IsKeyDown(ImGuiKey_LeftSuper) || IsKeyDown(ImGuiKey_RightSuper));

    // Update gamepad/keyboard navigation
    nav_update();

    // Update mouse input state
    update_mouse_inputs();

    // Undocking
    // (needs to be before UpdateMouseMovingWindowNewFrame so the window is already offset and following the mouse on the detaching frame)
    dock_context_new_frame_update_undocking(&g);

    // Find hovered window
    // (needs to be before UpdateMouseMovingWindowNewFrame so we fill g.hovered_window_under_moving_window on the mouse release frame)
    update_hovered_window_and_capture_flags();

    // Handle user moving window with mouse (at the beginning of the frame to avoid input lag or sheering)
    update_mouse_moving_window_new_frame();

    // Background darkening/whitening
    if (get_top_most_popup_modal() != NULL || (g.nav_windowing_target != NULL && g.nav_windowing_highlight_alpha > 0.0)) {
        g.dim_bg_ratio = f32::min(g.dim_bg_ratio + g.io.delta_time * 6.0, 1.0);
    }
    else {
        g.dim_bg_ratio = ImMax(g.dim_bg_ratio - g.io.delta_time * 10.0, 0.0);
    }

    g.mouse_cursor = MouseCursor::Arrow;
    g.want_capture_keyboard_next_frame = g.want_text_input_next_frame = -1;
    g.want_capture_mouse_next_frame = g.want_capture_keyboard_next_frame;

    // Platform IME data: reset for the frame
    g.platform_ime_data_prev = g.platform_ime_data;
    g.platform_ime_data.WantVisible = false;

    // Mouse wheel scrolling, scale
    update_mouse_wheel();

    // Mark all windows as not visible and compact unused memory.
    // IM_ASSERT(g.WindowsFocusOrder.Size <= g.Windows.Size);
    let memory_compact_start_time = if (g.gc_compact_all || g.io.config_memory_compact_timer < 0.0) { f32::MAX} else { g.time  - g.io.config_memory_compact_timer};
    // for (int i = 0; i != g.Windows.Size; i += 1)
    for  i in 0 .. g.windows.len()
    {
        // ImGuiWindow* window = g.Windows[i];
        let mut window = g.windows[i];
        window.was_active = window.active;
        window.begin_count = 0;
        window.active = false;
        window.WriteAccessed = false;

        // Garbage collect transient buffers of recently unused windows
        if (!window.was_active && !window.memory_compacted && window.last_time_active < memory_compact_start_time) {
            gc_compact_transient_window_buffers(window);
        }
    }

    // Garbage collect transient buffers of recently unused tables
    for (int i = 0; i < g.tables_last_time_active.Size; i += 1){
        if (g.tables_last_time_active[i] >= 0.0 && g.tables_last_time_active[i] < memory_compact_start_time) {
            table_gc_compact_transient_buffers(g.tables.get_by_index(i));
        }
    }
    for (int i = 0; i < g.tables_temp_data.Size; i += 1){
        if (g.tables_temp_data[i].last_time_active >= 0.0 && g.tables_temp_data[i].last_time_active < memory_compact_start_time) {
            table_gc_compact_transient_buffers(&g.tables_temp_data[i]);
        }
    }
    if (g.gc_compact_all) {
        GcCompactTransientMiscBuffers();
    }
    g.gc_compact_all = false;

    // Closing the focused window restore focus to the first active root window in descending z-order
    if (g.nav_window_id && !g.nav_window_id ->WasActive){
    FocusTopMostWindowUnderOne(NULL, NULL);
}

    // No window should be open at the beginning of the frame.
    // But in order to allow the user to call NewFrame() multiple times without calling Render(), we are doing an explicit clear.
    g.current_window_stack.clear();
    g.begin_popup_stack.clear();
    g.item_flags_stack.clear();
    g.item_flags_stack.push_back(GuiItemFlags::None);
    g.group_stack.clear();

    // Docking
    dock_context_new_frame_update_docking(&g);

    // [DEBUG] Update debug features
    update_debug_tool_item_picker();
    update_debug_tool_stack_queries();

    // Create implicit/fallback window - which we will only render it if the user has added something to it.
    // We don't use "Debug" to avoid colliding with user trying to create a "Debug" window with custom flags.
    // This fallback is particularly important as it avoid ImGui:: calls from crashing.
    g.within_frame_scope_with_implicit_window = true;
    set_next_window_size(Vector2D::new(400, 400), Cond::FirstUseEver);
    begin("Debug##Default");
    // IM_ASSERT(g.CurrentWindow->IsFallbackWindow == true);

    call_context_hooks(g, ContextHookType::NewFramePost);
}