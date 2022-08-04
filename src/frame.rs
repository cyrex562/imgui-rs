use std::collections::HashSet;
use crate::condition::Condition;
use crate::config::BackendFlags;
use crate::context::{call_context_hooks, Context, ContextHookType};
use crate::dock::context::dock_context_new_frame_update_undocking;
use crate::drag_drop::DragDropFlags;
use crate::draw::list::DrawListFlags;
use crate::gc::GcCompactTransientMiscBuffers;
use crate::input::keyboard::update_keyboard_inputs;
use crate::input::mouse::{update_mouse_inputs, update_mouse_moving_window_new_frame, update_mouse_wheel};
use crate::input::{MouseCursor, update_input_events};
use crate::input_event::InputEvent;
use crate::nav::nav_update;
use crate::popup::get_top_most_popup_modal;
use crate::rect::Rect;
use crate::settings::update_settings;
use crate::types::INVALID_ID;
use crate::vectors::vector_2d::Vector2D;
use crate::viewport::update_viewports_new_frame;
use crate::window::WindowFlags;
use crate::window::lifecycle::{add_window_to_sort_buffer, update_hovered_window_and_capture_flags};

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

// impl ImGuiOnceUponAFrame {
//     pub fn new() -> Self {
//         Self {
//             ref_frame: -1,
//         }
//     }
// }

impl Default for ImGuiOnceUponAFrame {
    fn default() -> Self {
        Self {
            ref_frame: i32::MAX,
        }
    }
}

// void ImGui::NewFrame()
pub fn new_frame(g: &mut Context)
{
    // IM_ASSERT(GImGui != None && "No current context. Did you call ImGui::CreateContext() and ImGui::SetCurrentContext() ?");
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
    update_settings(g);

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
    g.io.frame_rate = if g.frame_rate_sec_per_frame_accum > 0.0 { (1.0 / (g.frame_rate_sec_per_frame_accum / g.frame_rate_sec_per_frame_count)) } else {f32::f32::MAX};

    update_viewports_new_frame(g);

    // Setup current font and draw list shared data
    // FIXME-VIEWPORT: the concept of a single ClipRectFullscreen is not ideal!
    g.io.fonts.locked = true;
    sec_current_font(get_default_font());
    // IM_ASSERT(g.Font->IsLoaded());
    let mut virtual_space = Rect::new4(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    // for (int n = 0; n < g.viewports.Size; n += 1){
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
    if g.io.backend_flags.contains(&BackendFlags::RendererHasVxtOffset) {
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
    if g.hovered_id_previous_frame == INVALID_ID || (g.hovered_id != INVALID_ID && g.active_id == g.hovered_id) {
        g.hovered_id_not_active_timer = 0.0;
    }
    if g.hovered_id {
        g.hovered_id_timer += g.io.delta_time;
    }
    if g.hovered_id != INVALID_ID && g.active_id != g.hovered_id {
        g.hovered_id_not_active_timer += g.io.delta_time;
    }
    g.hovered_id_previous_frame = g.hovered_id;
    g.hovered_id_previous_frame_using_mouse_wheel = g.hovered_id_using_mouse_wheel;
    g.hovered_id = INVALID_ID;
    g.hovered_id_allow_overlap = false;
    g.hovered_id_using_mouse_wheel = false;
    g.hovered_id_disabled = false;

    // clear ActiveID if the item is not alive anymore.
    // In 1.87, the common most call to keep_alive_id() was moved from GetID() to ItemAdd().
    // As a result, custom widget using ButtonBehavior() _without_ ItemAdd() need to call keep_alive_id() themselves.
    if g.active_id != 0 && g.active_id_is_alive != g.active_id && g.active_id_previous_frame == g.active_id
    {
        // IMGUI_DEBUG_LOG_ACTIVEID("NewFrame(): ClearActiveID() because it isn't marked alive anymore!\n");
        clear_active_id();
    }

    // Update active_id data (clear reference to active widget if the widget isn't alive anymore)
    if g.active_id {
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
    if g.temp_input_id != 0 && g.active_id != g.temp_input_id {
        g.temp_input_id = INVALID_ID;
    }
    if g.active_id == 0
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
    g.drag_drop_hold_just_pressed_id = INVALID_ID;

    // Close popups on focus lost (currently wip/opt-in)
    //if (g.io.app_focus_lost)
    //    ClosePopupsExceptModals();

    // Process input queue (trickle as many events as possible)
    g.input_events_trail.resize(0, InputEvent::default());
    update_input_events(g, g.io.config_input_trickle_event_queue);

    // Update keyboard input state
    update_keyboard_inputs(g);

    //IM_ASSERT(g.io.key_ctrl == IsKeyDown(ImGuiKey_LeftCtrl) || IsKeyDown(ImGuiKey_RightCtrl));
    //IM_ASSERT(g.io.key_shift == IsKeyDown(ImGuiKey_LeftShift) || IsKeyDown(ImGuiKey_RightShift));
    //IM_ASSERT(g.io.key_alt == IsKeyDown(ImGuiKey_LeftAlt) || IsKeyDown(ImGuiKey_RightAlt));
    //IM_ASSERT(g.io.key_super == IsKeyDown(ImGuiKey_LeftSuper) || IsKeyDown(ImGuiKey_RightSuper));

    // Update gamepad/keyboard navigation
    nav_update(g);

    // Update mouse input state
    update_mouse_inputs(g);

    // Undocking
    // (needs to be before UpdateMouseMovingWindowNewFrame so the window is already offset and following the mouse on the detaching frame)
    dock_context_new_frame_update_undocking(g);

    // Find hovered window
    // (needs to be before UpdateMouseMovingWindowNewFrame so we fill g.hovered_window_under_moving_window on the mouse release frame)
    update_hovered_window_and_capture_flags(g);

    // Handle user moving window with mouse (at the beginning of the frame to avoid input lag or sheering)
    update_mouse_moving_window_new_frame(g);

    // Background darkening/whitening
    if get_top_most_popup_modal(g) != None || (g.nav_windowing_target_id != INVALID_ID && g.nav_windowing_highlight_alpha > 0.0) {
        g.dim_bg_ratio = f32::min(g.dim_bg_ratio + g.io.delta_time * 6.0, 1.0);
    }
    else {
        g.dim_bg_ratio = ImMax(g.dim_bg_ratio - g.io.delta_time * 10.0, 0.0);
    }

    g.mouse_cursor = MouseCursor::Arrow;
    g.want_capture_keyboard_next_frame = -1;
    g.want_text_input_next_frame = -1;
    g.want_capture_mouse_next_frame = g.want_capture_keyboard_next_frame;

    // Platform IME data: reset for the frame
    g.platform_ime_data_prev = g.platform_ime_data.clone();
    g.platform_ime_data.WantVisible = false;

    // Mouse wheel scrolling, scale
    update_mouse_wheel(g);

    // Mark all windows as not visible and compact unused memory.
    // IM_ASSERT(g.WindowsFocusOrder.Size <= g.Windows.Size);
    let memory_compact_start_time = if g.gc_compact_all || g.io.config_memory_compact_timer < 0.0 { f32::MAX} else { g.time  - g.io.config_memory_compact_timer};
    // for (int i = 0; i != g.Windows.Size; i += 1)
    for  i in 0 .. g.windows.len()
    {
        // ImGuiWindow* window = g.Windows[i];
        let mut window = g.windows[i];
        window.was_active = window.active;
        window.begin_count = 0;
        window.active = false;
        window.write_accessed = false;

        // Garbage collect transient buffers of recently unused windows
        if !window.was_active && !window.memory_compacted && window.last_time_active < memory_compact_start_time {
            gc_compact_transient_window_buffers(window);
        }
    }

    // Garbage collect transient buffers of recently unused tables
    // for (int i = 0; i < g.tables_last_time_active.size; i += 1)
    for i in 0 .. g.tables_last_time_active.len()
    {
        if g.tables_last_time_active[i] >= 0.0 && g.tables_last_time_active[i] < memory_compact_start_time {
            table_gc_compact_transient_buffers(g.tables.get_by_index(i));
        }
    }
    // for (int i = 0; i < g.tables_temp_data.size; i += 1)
    for i in 0 .. g.tables_temp_data.len()
    {
        if g.tables_temp_data[i].last_time_active >= 0.0 && g.tables_temp_data[i].last_time_active < memory_compact_start_time {
            table_gc_compact_transient_buffers(&g.tables_temp_data[i]);
        }
    }
    if g.gc_compact_all {
        GcCompactTransientMiscBuffers(g);
    }
    g.gc_compact_all = false;

    // Closing the focused window restore focus to the first active root window in descending z-order
    if g.nav_window_id && !g.nav_window_id.WasActive {
    focus_topmost_window_under_one(None, None);
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
    set_next_window_size(Vector2D::new(400, 400), Condition::FirstUseEver);
    begin("Debug##Default");
    // IM_ASSERT(g.CurrentWindow->IsFallbackWindow == true);

    call_context_hooks(g, ContextHookType::NewFramePost);
}

// This is normally called by Render(). You may want to call it directly if you want to avoid calling Render() but the gain will be very minimal.
// void ImGui::EndFrame()
pub fn end_frame(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.initialized);

    // Don't process EndFrame() multiple times.
    if g.frame_count_ended == g.frame_count {
        return;
    }
    // IM_ASSERT(g.within_frame_scope && "Forgot to call ImGui::NewFrame()?");

    call_context_hooks(g, ContextHookType::EndFramePre);

    error_check_end_frame_sanity_checks();

    // Notify Platform/OS when our Input Method Editor cursor has moved (e.g. CJK inputs using Microsoft IME)
    if g.io.set_platform_ime_data_fn.is_some() && (g.platform_ime_data == g.platform_ime_data_prev)
    {
        // ImGuiViewport* viewport = FindViewportByID(g.PlatformImeViewport);
        let viewport = g.viewport_mut(g.platform_ime_viewport);
        g.io.set_platform_ime_data_fn(if viewport.is_some() {viewport} else { get_main_viewport() }, &g.platform_ime_data);
    }

    // Hide implicit/fallback "Debug" window if it hasn't been used
    g.within_frame_scope_with_implicit_window = false;
    if g.current_window && !g.current_window.write_accessed){
        g.current_window.active = false;
    }
    end();

    // Update navigation: CTRL+Tab, wrap-around requests
    nav_end_frame();

    // Update docking
    dock_context_end_frame(g);

    set_current_viewport(None, None);

    // Drag and Drop: Elapse payload (if delivered, or if source stops being submitted)
    if g.drag_drop_active
    {
        let is_delivered = g.drag_drop_payload.delivery;
        let is_elapsed = (g.drag_drop_payload.data_frame_count + 1 < g.frame_count) && ((g.drag_drop_source_flags.contains(DragDropFlags::SourceAutoExpirePayload) ) || !is_mouse_down(g.drag_drop_mouse_button));
        if is_delivered || is_elapsed {
            clear_drag_drop();
        }
    }

    // Drag and Drop: Fallback for source tooltip. This is not ideal but better than nothing.
    if g.drag_drop_active && g.drag_drop_source_frame_count < g.frame_count && !g.drag_drop_source_flags.contains(&DragDropFlags::SourceNoPreviewTooltip)
    {
        g.drag_drop_within_source = true;
        SetTooltip("...");
        g.drag_drop_within_source = false;
    }

    // End frame
    g.within_frame_scope = false;
    g.frame_count_ended = g.frame_count;

    // Initiate moving window + handle left-click and right-click focus
    UpdateMouseMovingWindowEndFrame();

    // Update user-facing viewport list (g.viewports -> g.platform_io.viewports after filtering out some)
    UpdateViewportsEndFrame();

    // Sort the window list so that all child windows are after their parent
    // We cannot do that on focus_window() because children may not exist yet
    // g.windows_temp_sort_buffer.resize(0);
    g.windows_temp_sort_buffer.reserve(g.windows.len());
    // for (int i = 0; i != g.windows.Size; i += 1)
    for win in g.windows.iter_mut()
    {
        // ImGuiWindow* window = g.windows[i];
        if win.active && win.flags.contains(&WindowFlags::ChildWindow) {    // if a child is active its parent will add it
            continue;
        }
        add_window_to_sort_buffer(&g.windows_temp_sort_buffer, window);
    }

    // This usually assert if there is a mismatch between the ImGuiWindowFlags_ChildWindow / ParentWindow values and dc.ChildWindows[] in parents, aka we've done something wrong.
    // IM_ASSERT(g.windows.Size == g.windows_temp_sort_buffer.Size);
    g.windows.swap(&mut g.windows_temp_sort_buffer);
    g.io.metrics_active_windows = g.windows_active_count;

    // Unlock font atlas
    g.io.fonts.locked = false;

    // clear Input data for next frame
    g.io.mouse_wheel = 0.0;
    g.io.mouse_wheel_h = 0.0;
    g.io.input_queue_characters.resize(0);
    memset(g.io.NavInputs, 0, sizeof(g.io.NavInputs));

    call_context_hooks(g, ImGuiContextHookType_EndFramePost);
}

// float GetFrameHeight()
pub fn get_frame_height(g: &mut Context) -> f32
{
    // ImGuiContext& g = *GImGui;
    return g.font_size + g.style.frame_padding.y * 2.0;
}

// float GetFrameHeightWithSpacing()
pub fn get_frame_height_with_spacing(g: &mut Context) -> f32
{
    // ImGuiContext& g = *GImGui;
    return g.font_size + g.style.frame_padding.y * 2.0 + g.style.ItemSpacing.y;
}
