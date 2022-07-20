use std::collections::HashSet;
use std::io::SeekFrom::end;
use std::io::stdout;
use std::mem::size_of;
use crate::axis::Axis;
use crate::border::{get_resize_border_rect, ResizeBorderDef};
use crate::color::{IM_COL32_A_MASK, IM_COL32_A_SHIFT, IM_COL32_BLACK, IM_COL32_WHITE, make_color_32, StyleColor};
use crate::condition::Condition;
use crate::config::{ConfigFlags, IMGUI_DEBUG_INI_SETINGS};
use crate::context::{call_context_hooks, Context, ContextHook, ContextHookType};
use crate::data_authority::DataAuthority::Window;
use crate::direction::Direction;
use crate::dock::DOCKING_TRANSPARENT_PAYLOAD_ALPHA;
use crate::dock_context::dock_context_shutdown;
use crate::dock_node::{dock_node_get_root_node, DockNode};
use crate::drag_drop::DragDropFlags;
use crate::draw_data::{add_root_window_to_draw_data, add_window_to_draw_data, DrawData};
use crate::draw_defines::DrawFlags;
use crate::draw_list::{add_draw_list_to_draw_data, DrawList, DrawListFlags, get_background_draw_list, get_foreground_draw_list, get_viewport_draw_list};
use crate::font::Font;
use crate::font_atlas::FontAtlas;
use crate::frame::end_frame;
use crate::types::{Id32, INVALID_ID};
use crate::globals::GImGui;
use crate::hash::hash_string;
use crate::id::set_active_id;
use crate::input::{DimgKeyData, InputSource, ModFlags, MouseButton, MouseCursor, NavLayer, WINDOWS_MOUSE_WHEEL_SCROLL_LOCK_TIMER};
use crate::item::{is_item_deactivated, is_item_hovered, ItemFlags, ItemStatusFlags};
use crate::kv_store::Storage;
use crate::math::{floor_vector_2d, im_f32_to_int8_sat, saturate_f32, swap_f32};
use crate::mouse::{start_lock_wheeling_window, start_mouse_moving_window, start_mouse_moving_window_or_node};
use crate::nav::NAV_RESIZE_SPEED;
use crate::rect::Rect;
use crate::render::{find_rendered_text_end, render_dimmed_background_behind_window, render_dimmed_backgrounds};
use crate::resize::{RESIZE_GRIP_DEF, ResizeGripDef};
use crate::settings::SettingsHandler;
use crate::size_callback_data::SizeCallbackData;
use crate::style::{get_color_u32, get_color_u32_no_alpha, pop_style_color, push_style_color, Style};
use crate::text::calc_text_size;
use crate::utils::{add_hash_set, remove_hash_set_val, set_hash_set, sub_hash_set};
use crate::vectors::ImLengthSqr;
use crate::vectors::two_d::Vector2D;
use crate::viewport::{setup_viewport_draw_data, Viewport, ViewportFlags};
use crate::window::{HoveredFlags, ImGuiSizeCallback, Window, WindowFlags, WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER};
use crate::window::checks::{is_window_active_and_visible, is_window_content_hoverable};
use crate::window::class::WindowClass;
use crate::window::props::get_window_bg_color_idx;
use crate::window::get::{find_bottom_most_visible_window_with_begin_stack, find_front_most_visible_child_window, find_or_create_window_by_name, find_window_by_name, get_window_display_layer, get_window_for_title_and_menu_height};
use crate::window::lifecycle::{add_window_to_sort_buffer, update_window_focus_order_list};
use crate::window::next_window::NextWindowDataFlags;
use crate::window::state::set_window_condition_allow_flags;
use crate::window::settings::{apply_window_settings, WindowSettings};
use crate::window::size::{calc_resize_pos_size_from_any_corner, calc_window_auto_fit_size, calc_window_content_sizes, calc_window_size_after_constraint};


//-----------------------------------------------------------------------------
// [SECTION] INPUTS
//-----------------------------------------------------------------------------
// IM_STATIC_ASSERT(ImGuiKey_NamedKey_COUNT == IM_ARRAYSIZE(GKeyNames));

// int GetKeyPressedAmount(ImGuiKey key, float repeat_delay, float repeat_rate)
pub fn get_key_pressed_amount(g: &mut Context, key: Key, repeat_delay: f32, repeat_rate: f32) -> i32
{
    ImGuiContext& g = *GImGui;
    const ImGuiKeyData* key_data = GetKeyData(key);
    const float t = key_data.down_duration;
    return CalcTypematicRepeatAmount(t - g.io.delta_time, t, repeat_delay, repeat_rate);
}

// Note that Dear ImGui doesn't know the meaning/semantic of ImGuiKey from 0..511: they are legacy native keycodes.
// Consider transitioning from 'IsKeyDown(MY_ENGINE_KEY_A)' (<1.87) to IsKeyDown(ImGuiKey_A) (>= 1.87)
bool IsKeyDown(ImGuiKey key)
{
    const ImGuiKeyData* key_data = GetKeyData(key);
    if (!key_data.down)
        return false;
    return true;
}

bool IsKeyPressed(ImGuiKey key, bool repeat)
{
    ImGuiContext& g = *GImGui;
    const ImGuiKeyData* key_data = GetKeyData(key);
    const float t = key_data.down_duration;
    if (t < 0.0)
        return false;
    const bool pressed = (t == 0.0) || (repeat && t > g.io.KeyRepeatDelay && GetKeyPressedAmount(key, g.io.KeyRepeatDelay, g.io.KeyRepeatRate) > 0);
    if (!pressed)
        return false;
    return true;
}

bool IsKeyReleased(ImGuiKey key)
{
    const ImGuiKeyData* key_data = GetKeyData(key);
    if (key_data.down_durationPrev < 0.0 || key_data.down)
        return false;
    return true;
}

bool IsMouseDown(ImGuiMouseButton button)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    return g.io.mouse_down[button];
}

bool IsMouseClicked(ImGuiMouseButton button, bool repeat)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    const float t = g.io.mouse_down_duration[button];
    if (t == 0.0)
        return true;
    if (repeat && t > g.io.KeyRepeatDelay)
        return CalcTypematicRepeatAmount(t - g.io.delta_time, t, g.io.KeyRepeatDelay, g.io.KeyRepeatRate) > 0;
    return false;
}

bool IsMouseReleased(ImGuiMouseButton button)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    return g.io.mouse_released[button];
}

bool IsMouseDoubleClicked(ImGuiMouseButton button)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    return g.io.mouse_clicked_count[button] == 2;
}

int GetMouseClickedCount(ImGuiMouseButton button)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    return g.io.mouse_clicked_count[button];
}

// Return if a mouse click/drag went past the given threshold. valid to call during the mouse_released frame.
// [Internal] This doesn't test if the button is pressed
bool IsMouseDragPastThreshold(ImGuiMouseButton button, float lock_threshold)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    if (lock_threshold < 0.0)
        lock_threshold = g.io.mouse_drag_threshold;
    return g.io.mouse_drag_max_distance_sqr[button] >= lock_threshold * lock_threshold;
}

bool is_mouse_dragging(ImGuiMouseButton button, float lock_threshold)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    if (!g.io.mouse_down[button])
        return false;
    return IsMouseDragPastThreshold(button, lock_threshold);
}

Vector2D GetMousePos()
{
    ImGuiContext& g = *GImGui;
    return g.io.mouse_pos;
}

// NB: prefer to call right after BeginPopup(). At the time Selectable/MenuItem is activated, the popup is already closed!
Vector2D GetMousePosOnOpeningCurrentPopup()
{
    ImGuiContext& g = *GImGui;
    if (g.begin_popup_stack.size > 0)
        return g.open_popup_stack[g.begin_popup_stack.size - 1].OpenMousePos;
    return g.io.mouse_pos;
}

// We typically use Vector2D(-FLT_MAX,-FLT_MAX) to denote an invalid mouse position.
bool is_mouse_pos_valid(const Vector2D* mouse_pos)
{
    // The assert is only to silence a false-positive in XCode Static Analysis.
    // Because GImGui is not dereferenced in every code path, the static analyzer assume that it may be NULL (which it doesn't for other functions).
    IM_ASSERT(GImGui != NULL);
    const float MOUSE_INVALID = -256000.0;
    Vector2D p = mouse_pos ? *mouse_pos : GImGui.IO.MousePos;
    return p.x >= MOUSE_INVALID && p.y >= MOUSE_INVALID;
}

// [WILL OBSOLETE] This was designed for backends, but prefer having backend maintain a mask of held mouse buttons, because upcoming input queue system will make this invalid.
bool IsAnyMouseDown()
{
    ImGuiContext& g = *GImGui;
    for (int n = 0; n < IM_ARRAYSIZE(g.io.mouse_down); n += 1)
        if (g.io.mouse_down[n])
            return true;
    return false;
}

// Return the delta from the initial clicking position while the mouse button is clicked or was just released.
// This is locked and return 0.0 until the mouse moves past a distance threshold at least once.
// NB: This is only valid if is_mouse_pos_valid(). backends in theory should always keep mouse position valid when dragging even outside the client window.
Vector2D GetMouseDragDelta(ImGuiMouseButton button, float lock_threshold)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    if (lock_threshold < 0.0)
        lock_threshold = g.io.mouse_drag_threshold;
    if (g.io.mouse_down[button] || g.io.mouse_released[button])
        if (g.io.mouse_drag_max_distance_sqr[button] >= lock_threshold * lock_threshold)
            if (is_mouse_pos_valid(&g.io.mouse_pos) && is_mouse_pos_valid(&g.io.mouse_clicked_pos[button]))
                return g.io.mouse_pos - g.io.mouse_clicked_pos[button];
    return Vector2D::new(0.0, 0.0);
}

void ResetMouseDragDelta(ImGuiMouseButton button)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    // NB: We don't need to reset g.io.mouse_drag_max_distance_sqr
    g.io.mouse_clicked_pos[button] = g.io.mouse_pos;
}

ImGuiMouseCursor GetMouseCursor()
{
    ImGuiContext& g = *GImGui;
    return g.mouse_cursor;
}

void SetMouseCursor(ImGuiMouseCursor cursor_type)
{
    ImGuiContext& g = *GImGui;
    g.mouse_cursor = cursor_type;
}

void SetNextFrameWantCaptureKeyboard(bool want_capture_keyboard)
{
    ImGuiContext& g = *GImGui;
    g.want_capture_keyboard_next_frame = want_capture_keyboard ? 1 : 0;
}

void SetNextFrameWantCaptureMouse(bool want_capture_mouse)
{
    ImGuiContext& g = *GImGui;
    g.want_capture_mouse_next_frame = want_capture_mouse ? 1 : 0;
}

#ifndef IMGUI_DISABLE_DEBUG_TOOLS
static const char* GetInputSourceName(ImGuiInputSource source)
{
    const char* input_source_names[] = { "None", "Mouse", "Keyboard", "Gamepad", "Nav", "Clipboard" };
    IM_ASSERT(IM_ARRAYSIZE(input_source_names) == InputSource::COUNT && source >= 0 && source < InputSource::COUNT);
    return input_source_names[source];
}


/*static void DebugPrintInputEvent(const char* prefix, const ImGuiInputEvent* e)
{
    if (e->Type == ImGuiInputEventType_MousePos)    { IMGUI_DEBUG_LOG_IO("%s: mouse_pos (%.1 %.1)\n", prefix, e->mouse_pos.PosX, e->mouse_pos.PosY); return; }
    if (e->Type == ImGuiInputEventType_MouseButton) { IMGUI_DEBUG_LOG_IO("%s: MouseButton %d %s\n", prefix, e->MouseButton.Button, e->MouseButton.down ? "down" : "Up"); return; }
    if (e->Type == ImGuiInputEventType_MouseWheel)  { IMGUI_DEBUG_LOG_IO("%s: mouse_wheel (%.1 %.1)\n", prefix, e->mouse_wheel.WheelX, e->mouse_wheel.WheelY); return; }
    if (e->Type == ImGuiInputEventType_Key)         { IMGUI_DEBUG_LOG_IO("%s: Key \"%s\" %s\n", prefix, GetKeyName(e->Key.Key), e->Key.down ? "down" : "Up"); return; }
    if (e->Type == ImGuiInputEventType_Text)        { IMGUI_DEBUG_LOG_IO("%s: Text: %c (U+%08X)\n", prefix, e->Text.Char, e->Text.Char); return; }
    if (e->Type == ImGuiInputEventType_Focus)       { IMGUI_DEBUG_LOG_IO("%s: AppFocused %d\n", prefix, e->AppFocused.Focused); return; }
}*/

// Process input queue
// We always call this with the value of 'bool g.io.config_input_trickle_event_queue'.
// - trickle_fast_inputs = false : process all events, turn into flattened input state (e.g. successive down/up/down/up will be lost)
// - trickle_fast_inputs = true  : process as many events as possible (successive down/up/down/up will be trickled over several frames so nothing is lost) (new feature in 1.87)
void update_input_events(bool trickle_fast_inputs)
{
    ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;

    // Only trickle chars<>key when working with InputText()
    // FIXME: InputText() could parse event trail?
    // FIXME: Could specialize chars<>keys trickling rules for control keys (those not typically associated to characters)
    const bool trickle_interleaved_keys_and_text = (trickle_fast_inputs && g.want_text_input_next_frame == 1);

    bool mouse_moved = false, mouse_wheeled = false, key_changed = false, text_inputted = false;
    int  mouse_button_changed = 0x00;
    ImBitArray<ImGuiKey_KeysData_SIZE> key_changed_mask;

    int event_n = 0;
    for (; event_n < g.InputEventsQueue.size; event_n += 1)
    {
        const ImGuiInputEvent* e = &g.InputEventsQueue[event_n];
        if (e.Type == ImGuiInputEventType_MousePos)
        {
            Vector2D event_pos(e.MousePos.PosX, e.MousePos.PosY);
            if (is_mouse_pos_valid(&event_pos))
                event_pos = Vector2D::new(f32::floor(event_pos.x), f32::floor(event_pos.y)); // Apply same flooring as UpdateMouseInputs()
            if (io.mouse_pos.x != event_pos.x || io.mouse_pos.y != event_pos.y)
            {
                // Trickling Rule: Stop processing queued events if we already handled a mouse button change
                if (trickle_fast_inputs && (mouse_button_changed != 0 || mouse_wheeled || key_changed || text_inputted))
                    break;
                io.mouse_pos = event_pos;
                mouse_moved = true;
            }
        }
        else if (e.Type == ImGuiInputEventType_MouseButton)
        {
            const ImGuiMouseButton button = e.MouseButton.Button;
            IM_ASSERT(button >= 0 && button < ImGuiMouseButton_COUNT);
            if (io.mouse_down[button] != e.MouseButton.down)
            {
                // Trickling Rule: Stop processing queued events if we got multiple action on the same button
                if (trickle_fast_inputs && ((mouse_button_changed & (1 << button)) || mouse_wheeled))
                    break;
                io.mouse_down[button] = e.MouseButton.down;
                mouse_button_changed |= (1 << button);
            }
        }
        else if (e.Type == ImGuiInputEventType_MouseWheel)
        {
            if (e.mouse_wheel.WheelX != 0.0 || e.mouse_wheel.WheelY != 0.0)
            {
                // Trickling Rule: Stop processing queued events if we got multiple action on the event
                if (trickle_fast_inputs && (mouse_moved || mouse_button_changed != 0))
                    break;
                io.mouse_wheel_h += e.mouse_wheel.WheelX;
                io.mouse_wheel += e.mouse_wheel.WheelY;
                mouse_wheeled = true;
            }
        }
        else if (e.Type == ImGuiInputEventType_MouseViewport)
        {
            io.MouseHoveredViewport = e.mouse_viewport.HoveredViewportID;
        }
        else if (e.Type == ImGuiInputEventType_Key)
        {
            ImGuiKey key = e.Key.Key;
            IM_ASSERT(key != ImGuiKey_None);
            const int keydata_index = (key - Key::KeysDataOffset);
            ImGuiKeyData* keydata = &io.keys_data[keydata_index];
            if (keydata.down != e.Key.down || keydata.analog_value != e.Key.analog_value)
            {
                // Trickling Rule: Stop processing queued events if we got multiple action on the same button
                if (trickle_fast_inputs && keydata.down != e.Key.down && (key_changed_mask.TestBit(keydata_index) || text_inputted || mouse_button_changed != 0))
                    break;
                keydata.down = e.Key.down;
                keydata.analog_value = e.Key.analog_value;
                key_changed = true;
                key_changed_mask.SetBit(keydata_index);

                if (key == Key::ModCtrl || key == Key::ModShift || key == Key::ModAlt || key == Key::ModSuper)
                {
                    if (key == Key::ModCtrl) { io.key_ctrl = keydata.down; }
                    if (key == Key::ModShift) { io.key_shift = keydata.down; }
                    if (key == Key::ModAlt) { io.key_alt = keydata.down; }
                    if (key == Key::ModSuper) { io.key_super = keydata.down; }
                    io.key_mods = get_merged_mod_flags();
                }

                // Allow legacy code using io.keys_down[GetKeyIndex()] with new backends
#ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
                io.keys_down[key] = keydata.down;
                if (io.key_map[key] != -1)
                    io.keys_down[io.key_map[key]] = keydata.down;

            }
        }
        else if (e.Type == ImGuiInputEventType_Text)
        {
            // Trickling Rule: Stop processing queued events if keys/mouse have been interacted with
            if (trickle_fast_inputs && ((key_changed && trickle_interleaved_keys_and_text) || mouse_button_changed != 0 || mouse_moved || mouse_wheeled))
                break;
            unsigned int c = e.Text.Char;
            io.input_queue_characters.push_back(c <= IM_UNICODE_CODEPOINT_MAX ? (ImWchar)c : IM_UNICODE_CODEPOINT_INVALID);
            if (trickle_interleaved_keys_and_text)
                text_inputted = true;
        }
        else if (e.Type == ImGuiInputEventType_Focus)
        {
            // We intentionally overwrite this and process lower, in order to give a chance
            // to multi-viewports backends to queue add_focus_event(false) + add_focus_event(true) in same frame.
            io.AppFocusLost = !e.AppFocused.Focused;
        }
        else
        {
            IM_ASSERT(0 && "Unknown event!");
        }
    }

    // Record trail (for domain-specific applications wanting to access a precise trail)
    //if (event_n != 0) IMGUI_DEBUG_LOG_IO("Processed: %d / Remaining: %d\n", event_n, g.input_events_queue.size - event_n);
    for (int n = 0; n < event_n; n += 1)
        g.input_events_trail.push_back(g.InputEventsQueue[n]);

    // [DEBUG]
    /*if (event_n != 0)
        for (int n = 0; n < g.input_events_queue.size; n++)
            DebugPrintInputEvent(n < event_n ? "Processed" : "Remaining", &g.input_events_queue[n]);*/

    // Remaining events will be processed on the next frame
    if (event_n == g.InputEventsQueue.size)
        g.InputEventsQueue.resize(0);
    else
        g.InputEventsQueue.erase(g.InputEventsQueue.data, g.InputEventsQueue.data + event_n);

    // clear buttons state when focus is lost
    // (this is useful so e.g. releasing Alt after focus loss on Alt-Tab doesn't trigger the Alt menu toggle)
    if (g.io.AppFocusLost)
    {
        g.io.ClearInputKeys();
        g.io.AppFocusLost = false;
    }
}


//-----------------------------------------------------------------------------
// [SECTION] ERROR CHECKING
//-----------------------------------------------------------------------------

// Helper function to verify ABI compatibility between caller code and compiled version of Dear ImGui.
// Verify that the type sizes are matching between the calling file's compilation unit and imgui.cpp's compilation unit
// If this triggers you have an issue:
// - Most commonly: mismatched headers and compiled code version.
// - Or: mismatched configuration #define, compilation settings, packing pragma etc.
//   The configuration settings mentioned in imconfig.h must be set for all compilation units involved with Dear ImGui,
//   which is way it is required you put them in your imconfig file (and not just before including imgui.h).
//   Otherwise it is possible that different compilation units would see different structure layout
bool DebugCheckVersionAndDataLayout(const char* version, size_t sz_io, size_t sz_style, size_t sz_vec2, size_t sz_vec4, size_t sz_vert, size_t sz_idx)
{
    bool error = false;
    if (strcmp(version, IMGUI_VERSION) != 0) { error = true; IM_ASSERT(strcmp(version, IMGUI_VERSION) == 0 && "Mismatched version string!"); }
    if (sz_io != sizeof(ImGuiIO)) { error = true; IM_ASSERT(sz_io == sizeof(ImGuiIO) && "Mismatched struct layout!"); }
    if (sz_style != sizeof(ImGuiStyle)) { error = true; IM_ASSERT(sz_style == sizeof(ImGuiStyle) && "Mismatched struct layout!"); }
    if (sz_vec2 != sizeof(Vector2D)) { error = true; IM_ASSERT(sz_vec2 == sizeof(Vector2D) && "Mismatched struct layout!"); }
    if (sz_vec4 != sizeof(Vector4D)) { error = true; IM_ASSERT(sz_vec4 == sizeof(Vector4D) && "Mismatched struct layout!"); }
    if (sz_vert != sizeof(ImDrawVert)) { error = true; IM_ASSERT(sz_vert == sizeof(ImDrawVert) && "Mismatched struct layout!"); }
    if (sz_idx != sizeof(ImDrawIdx)) { error = true; IM_ASSERT(sz_idx == sizeof(ImDrawIdx) && "Mismatched struct layout!"); }
    return !error;
}

static void error_check_new_frame_sanity_checks()
{
    ImGuiContext& g = *GImGui;

    // Check user IM_ASSERT macro
    // (IF YOU GET A WARNING OR COMPILE ERROR HERE: it means your assert macro is incorrectly defined!
    //  If your macro uses multiple statements, it NEEDS to be surrounded by a 'do { ... } while (0)' block.
    //  This is a common C/C++ idiom to allow multiple statements macros to be used in control flow blocks.)
    // #define IM_ASSERT(EXPR)   if (SomeCode(EXPR)) SomeMoreCode();                    // Wrong!
    // #define IM_ASSERT(EXPR)   do { if (SomeCode(EXPR)) SomeMoreCode(); } while (0)   // Correct!
    if (true) IM_ASSERT(1); else IM_ASSERT(0);

    // Check user data
    // (We pass an error message in the assert expression to make it visible to programmers who are not using a debugger, as most assert handlers display their argument)
    IM_ASSERT(g.initialized);
    IM_ASSERT((g.io.delta_time > 0.0 || g.frame_count == 0)              && "Need a positive delta_time!");
    IM_ASSERT((g.frame_count == 0 || g.frame_count_ended == g.frame_count)  && "Forgot to call Render() or EndFrame() at the end of the previous frame?");
    IM_ASSERT(g.io.display_size.x >= 0.0 && g.io.display_size.y >= 0.0  && "Invalid display_size value!");
    IM_ASSERT(g.io.fonts.IsBuilt()                                     && "font Atlas not built! Make sure you called ImGui_ImplXXXX_NewFrame() function for renderer backend, which should call io.fonts->GetTexDataAsRGBA32() / GetTexDataAsAlpha8()");
    IM_ASSERT(g.style.curve_tessellation_tol > 0.0                       && "Invalid style setting!");
    IM_ASSERT(g.style.circle_tessellation_max_error > 0.0                 && "Invalid style setting!");
    IM_ASSERT(g.style.alpha >= 0.0 && g.style.alpha <= 1.0            && "Invalid style setting!"); // Allows us to avoid a few clamps in color computations
    IM_ASSERT(g.style.window_min_size.x >= 1.0 && g.style.window_min_size.y >= 1.0 && "Invalid style setting.");
    IM_ASSERT(g.style.window_menu_button_position == Dir::None || g.style.window_menu_button_position == Dir::Left || g.style.window_menu_button_position == Dir::Right);
    IM_ASSERT(g.style.ColorButtonPosition == Dir::Left || g.style.ColorButtonPosition == Dir::Right);
#ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    for (int n = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_COUNT; n += 1)
        IM_ASSERT(g.io.key_map[n] >= -1 && g.io.key_map[n] < ImGuiKey_LegacyNativeKey_END && "io.key_map[] contains an out of bound value (need to be 0..511, or -1 for unmapped key)");

    // Check: required key mapping (we intentionally do NOT check all keys to not pressure user into setting up everything, but Space is required and was only added in 1.60 WIP)
    if ((g.io.config_flags & ConfigFlags::NavEnableKeyboard) && g.io.backend_using_legacy_key_arrays == 1)
        IM_ASSERT(g.io.key_map[ImGuiKey_Space] != -1 && "ImGuiKey_Space is not mapped, required for keyboard navigation.");


    // Check: the io.config_windows_resize_from_edges option requires backend to honor mouse cursor changes and set the ImGuiBackendFlags_HasMouseCursors flag accordingly.
    if (g.io.ConfigWindowsResizeFromEdges && !(g.io.backend_flags & ImGuiBackendFlags_HasMouseCursors))
        g.io.ConfigWindowsResizeFromEdges = false;

    // Perform simple check: error if Docking or viewport are enabled _exactly_ on frame 1 (instead of frame 0 or later), which is a common error leading to loss of .ini data.
    if (g.frame_count == 1 && (g.io.config_flags & ImGuiConfigFlags_DockingEnable) && (g.config_flags_last_frame & ImGuiConfigFlags_DockingEnable) == 0)
        IM_ASSERT(0 && "Please set DockingEnable before the first call to NewFrame()! Otherwise you will lose your .ini settings!");
    if (g.frame_count == 1 && (g.io.config_flags & ConfigFlags::ViewportsEnable) && (g.config_flags_last_frame & ConfigFlags::ViewportsEnable) == 0)
        IM_ASSERT(0 && "Please set ViewportsEnable before the first call to NewFrame()! Otherwise you will lose your .ini settings!");

    // Perform simple checks: multi-viewport and platform windows support
    if (g.io.config_flags & ConfigFlags::ViewportsEnable)
    {
        if ((g.io.backend_flags & ImGuiBackendFlags_PlatformHasViewports) && (g.io.backend_flags & ImGuiBackendFlags_RendererHasViewports))
        {
            IM_ASSERT((g.frame_count == 0 || g.frame_count == g.FrameCountPlatformEnded) && "Forgot to call UpdatePlatformWindows() in main loop after EndFrame()? Check examples/ applications for reference.");
            IM_ASSERT(g.platform_io.Platform_CreateWindow  != NULL && "Platform init didn't install handlers?");
            IM_ASSERT(g.platform_io.Platform_DestroyWindow != NULL && "Platform init didn't install handlers?");
            IM_ASSERT(g.platform_io.Platform_GetWindowPos  != NULL && "Platform init didn't install handlers?");
            IM_ASSERT(g.platform_io.Platform_SetWindowPos  != NULL && "Platform init didn't install handlers?");
            IM_ASSERT(g.platform_io.Platform_GetWindowSize != NULL && "Platform init didn't install handlers?");
            IM_ASSERT(g.platform_io.Platform_SetWindowSize != NULL && "Platform init didn't install handlers?");
            IM_ASSERT(g.platform_io.monitors.size > 0 && "Platform init didn't setup Monitors list?");
            IM_ASSERT((g.viewports[0].PlatformUserData != NULL || g.viewports[0].PlatformHandle != NULL) && "Platform init didn't setup main viewport.");
            if (g.io.config_docking_transparent_payload && (g.io.config_flags & ImGuiConfigFlags_DockingEnable))
                IM_ASSERT(g.platform_io.Platform_SetWindowAlpha != NULL && "Platform_SetWindowAlpha handler is required to use io.ConfigDockingTransparent!");
        }
        else
        {
            // Disable feature, our backends do not support it
            g.io.config_flags &= ~ConfigFlags::ViewportsEnable;
        }

        // Perform simple checks on platform monitor data + compute a total bounding box for quick early outs
        for (int monitor_n = 0; monitor_n < g.platform_io.monitors.size; monitor_n += 1)
        {
            ImGuiPlatformMonitor& mon = g.platform_io.monitors[monitor_n];
            IM_UNUSED(mon);
            IM_ASSERT(mon.MainSize.x > 0.0 && mon.MainSize.y > 0.0 && "Monitor main bounds not setup properly.");
            IM_ASSERT(Rect(mon.MainPos, mon.MainPos + mon.MainSize).Contains(Rect(mon.WorkPos, mon.WorkPos + mon.work_size)) && "Monitor work bounds not setup properly. If you don't have work area information, just copy MainPos/MainSize into them.");
            IM_ASSERT(mon.DpiScale != 0.0);
        }
    }
}

static void error_check_end_frame_sanity_checks()
{
    ImGuiContext& g = *GImGui;

    // Verify that io.KeyXXX fields haven't been tampered with. Key mods should not be modified between NewFrame() and EndFrame()
    // One possible reason leading to this assert is that your backends update inputs _AFTER_ NewFrame().
    // It is known that when some modal native windows called mid-frame takes focus away, some backends such as GLFW will
    // send key release events mid-frame. This would normally trigger this assertion and lead to sheared inputs.
    // We silently accommodate for this case by ignoring/ the case where all io.KeyXXX modifiers were released (aka key_mod_flags == 0),
    // while still correctly asserting on mid-frame key press events.
    const ImGuiModFlags key_mods = get_merged_mod_flags();
    IM_ASSERT((key_mods == 0 || g.io.key_mods == key_mods) && "Mismatching io.key_ctrl/io.key_shift/io.key_alt/io.key_super vs io.key_mods");
    IM_UNUSED(key_mods);

    // [EXPERIMENTAL] Recover from errors: You may call this yourself before EndFrame().
    //ErrorCheckEndFrameRecover();

    // Report when there is a mismatch of Begin/BeginChild vs End/EndChild calls. Important: Remember that the Begin/BeginChild API requires you
    // to always call End/EndChild even if Begin/BeginChild returns false! (this is unfortunately inconsistent with most other Begin* API).
    if (g.current_window_stack.size != 1)
    {
        if (g.current_window_stack.size > 1)
        {
            IM_ASSERT_USER_ERROR(g.current_window_stack.size == 1, "Mismatched Begin/BeginChild vs End/EndChild calls: did you forget to call End/EndChild?");
            while (g.current_window_stack.size > 1)
                end();
        }
        else
        {
            IM_ASSERT_USER_ERROR(g.current_window_stack.size == 1, "Mismatched Begin/BeginChild vs End/EndChild calls: did you call End/EndChild too much?");
        }
    }

    IM_ASSERT_USER_ERROR(g.group_stack.size == 0, "Missing EndGroup call!");
}

// Experimental recovery from incorrect usage of BeginXXX/EndXXX/PushXXX/PopXXX calls.
// Must be called during or before EndFrame().
// This is generally flawed as we are not necessarily End/Popping things in the right order.
// FIXME: Can't recover from inside BeginTabItem/EndTabItem yet.
// FIXME: Can't recover from interleaved BeginTabBar/Begin
void    ErrorCheckEndFrameRecover(ImGuiErrorLogCallback log_callback, void* user_data)
{
    // PVS-Studio V1044 is "Loop break conditions do not depend on the number of iterations"
    ImGuiContext& g = *GImGui;
    while (g.current_window_stack.size > 0) //-V1044
    {
        ErrorCheckEndWindowRecover(log_callback, user_data);
        ImGuiWindow* window = g.current_window;
        if (g.current_window_stack.size == 1)
        {
            IM_ASSERT(window.IsFallbackWindow);
            break;
        }
        if (window.flags & WindowFlags::ChildWindow)
        {
            if (log_callback) log_callback(user_data, "Recovered from missing EndChild() for '%s'", window.Name);
            end_child();
        }
        else
        {
            if (log_callback) log_callback(user_data, "Recovered from missing End() for '%s'", window.Name);
            end();
        }
    }
}

// Must be called before End()/EndChild()
void    ErrorCheckEndWindowRecover(ImGuiErrorLogCallback log_callback, void* user_data)
{
    ImGuiContext& g = *GImGui;
    while (g.CurrentTable && (g.CurrentTable.OuterWindow == g.current_window || g.CurrentTable.InnerWindow == g.current_window))
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndTable() in '%s'", g.CurrentTable.OuterWindow.Name);
        EndTable();
    }

    ImGuiWindow* window = g.current_window;
    ImGuiStackSizes* stack_sizes = &g.current_window_stack.back().StackSizesOnBegin;
    IM_ASSERT(window != NULL);
    while (g.CurrentTabBar != NULL) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndTabBar() in '%s'", window.Name);
        EndTabBar();
    }
    while (window.dc.TreeDepth > 0)
    {
        if (log_callback) log_callback(user_data, "Recovered from missing TreePop() in '%s'", window.Name);
        TreePop();
    }
    while (g.group_stack.size > stack_sizes.sizeOfGroupStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndGroup() in '%s'", window.Name);
        EndGroup();
    }
    while (window.IDStack.size > 1)
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopID() in '%s'", window.Name);
        PopID();
    }
    while (g.DisabledStackSize > stack_sizes.sizeOfDisabledStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndDisabled() in '%s'", window.Name);
        EndDisabled();
    }
    while (g.color_stack.size > stack_sizes.sizeOfColorStack)
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopStyleColor() in '%s' for ImGuiCol_%s", window.Name, GetStyleColorName(g.color_stack.back().Col));
        pop_style_color();
    }
    while (g.item_flags_stack.size > stack_sizes.sizeOfItemFlagsStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopItemFlag() in '%s'", window.Name);
        PopItemFlag();
    }
    while (g.style_var_stack.size > stack_sizes.sizeOfStyleVarStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopStyleVar() in '%s'", window.Name);
        pop_style_var();
    }
    while (g.FocusScopeStack.size > stack_sizes.sizeOfFocusScopeStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopFocusScope() in '%s'", window.Name);
        PopFocusScope();
    }
}

// Save current stack sizes for later compare
void ImGuiStackSizes::SetToCurrentState()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    SizeOfIDStack = window.IDStack.size;
    SizeOfColorStack = g.color_stack.size;
    SizeOfStyleVarStack = g.style_var_stack.size;
    SizeOfFontStack = g.font_stack.size;
    SizeOfFocusScopeStack = g.FocusScopeStack.size;
    SizeOfGroupStack = g.group_stack.size;
    SizeOfItemFlagsStack = g.item_flags_stack.size;
    SizeOfBeginPopupStack = g.begin_popup_stack.size;
    SizeOfDisabledStack = g.DisabledStackSize;
}

// Compare to detect usage errors
void ImGuiStackSizes::CompareWithCurrentState()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    IM_UNUSED(window);

    // window stacks
    // NOT checking: dc.item_width, dc.text_wrap_pos (per window) to allow user to conveniently push once and not pop (they are cleared on Begin)
    IM_ASSERT(SizeOfIDStack         == window.IDStack.size     && "PushID/PopID or TreeNode/TreePop Mismatch!");

    // Global stacks
    // For color, style and font stacks there is an incentive to use Push/Begin/Pop/.../End patterns, so we relax our checks a little to allow them.
    IM_ASSERT(SizeOfGroupStack      == g.group_stack.size        && "BeginGroup/EndGroup Mismatch!");
    IM_ASSERT(SizeOfBeginPopupStack == g.begin_popup_stack.size   && "BeginPopup/EndPopup or BeginMenu/EndMenu Mismatch!");
    IM_ASSERT(SizeOfDisabledStack   == g.DisabledStackSize      && "BeginDisabled/EndDisabled Mismatch!");
    IM_ASSERT(SizeOfItemFlagsStack  >= g.item_flags_stack.size    && "PushItemFlag/PopItemFlag Mismatch!");
    IM_ASSERT(SizeOfColorStack      >= g.color_stack.size        && "PushStyleColor/PopStyleColor Mismatch!");
    IM_ASSERT(SizeOfStyleVarStack   >= g.style_var_stack.size     && "PushStyleVar/PopStyleVar Mismatch!");
    IM_ASSERT(SizeOfFontStack       >= g.font_stack.size         && "PushFont/PopFont Mismatch!");
    IM_ASSERT(SizeOfFocusScopeStack == g.FocusScopeStack.size   && "PushFocusScope/PopFocusScope Mismatch!");
}


//-----------------------------------------------------------------------------
// [SECTION] LAYOUT
//-----------------------------------------------------------------------------
// - ItemSize()
// - ItemAdd()
// - SameLine()
// - GetCursorScreenPos()
// - SetCursorScreenPos()
// - GetCursorPos(), GetCursorPosX(), GetCursorPosY()
// - SetCursorPos(), SetCursorPosX(), SetCursorPosY()
// - GetCursorStartPos()
// - Indent()
// - Unindent()
// - SetNextItemWidth()
// - PushItemWidth()
// - PushMultiItemsWidths()
// - PopItemWidth()
// - CalcItemWidth()
// - CalcItemSize()
// - GetTextLineHeight()
// - GetTextLineHeightWithSpacing()
// - GetFrameHeight()
// - GetFrameHeightWithSpacing()
// - GetContentRegionMax()
// - GetContentRegionMaxAbs() [Internal]
// - GetContentRegionAvail(),
// - GetWindowContentRegionMin(), GetWindowContentRegionMax()
// - BeginGroup()
// - EndGroup()
// Also see in imgui_widgets: tab bars, and in imgui_tables: tables, columns.
//-----------------------------------------------------------------------------

// Advance cursor given item size for layout.
// Register minimum needed size so it can extend the bounding box used for auto-fit calculation.
// See comments in ItemAdd() about how/why the size provided to ItemSize() vs ItemAdd() may often different.
void item_size(const Vector2D& size, float text_baseline_y)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    if (window.skip_items)
        return;

    // We increase the height in this function to accommodate for baseline offset.
    // In theory we should be offsetting the starting position (window->dc.cursor_pos), that will be the topic of a larger refactor,
    // but since ItemSize() is not yet an API that moves the cursor (to handle e.g. wrapping) enlarging the height has the same effect.
    const float offset_to_match_baseline_y = (text_baseline_y >= 0) ? ImMax(0.0, window.dc.CurrLineTextBaseOffset - text_baseline_y) : 0.0;

    const float line_y1 = window.dc.IsSameLine ? window.dc.CursorPosPrevLine.y : window.dc.cursor_pos.y;
    const float line_height = ImMax(window.dc.CurrLineSize.y, /*ImMax(*/window.dc.cursor_pos.y - line_y1/*, 0.0)*/ + size.y + offset_to_match_baseline_y);

    // Always align ourselves on pixel boundaries
    //if (g.io.key_alt) window->draw_list->add_rect(window->dc.cursor_pos, window->dc.cursor_pos + Vector2D(size.x, line_height), IM_COL32(255,0,0,200)); // [DEBUG]
    window.dc.CursorPosPrevLine.x = window.dc.cursor_pos.x + size.x;
    window.dc.CursorPosPrevLine.y = line_y1;
    window.dc.cursor_pos.x = f32::floor(window.pos.x + window.dc.Indent.x + window.dc.ColumnsOffset.x);    // Next line
    window.dc.cursor_pos.y = f32::floor(line_y1 + line_height + g.style.ItemSpacing.y);                    // Next line
    window.dc.cursor_max_pos.x = ImMax(window.dc.cursor_max_pos.x, window.dc.CursorPosPrevLine.x);
    window.dc.cursor_max_pos.y = ImMax(window.dc.cursor_max_pos.y, window.dc.cursor_pos.y - g.style.ItemSpacing.y);
    //if (g.io.key_alt) window->draw_list->add_circle(window->dc.CursorMaxPos, 3.0, IM_COL32(255,0,0,255), 4); // [DEBUG]

    window.dc.PrevLineSize.y = line_height;
    window.dc.CurrLineSize.y = 0.0;
    window.dc.PrevLineTextBaseOffset = ImMax(window.dc.CurrLineTextBaseOffset, text_baseline_y);
    window.dc.CurrLineTextBaseOffset = 0.0;
    window.dc.IsSameLine = false;

    // Horizontal layout mode
    if (window.dc.LayoutType == ImGuiLayoutType_Horizontal)
        SameLine();
}

// Declare item bounding box for clipping and interaction.
// Note that the size can be different than the one provided to ItemSize(). Typically, widgets that spread over available surface
// declare their minimum size requirement to ItemSize() and provide a larger region to ItemAdd() which is used drawing/interaction.
bool item_add(const Rect& bb, ImGuiID id, const Rect* nav_bb_arg, ImGuiItemFlags extra_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;

    // Set item data
    // (display_rect is left untouched, made valid when ImGuiItemStatusFlags_HasDisplayRect is set)
    g.last_item_data.id = id;
    g.last_item_data.Rect = bb;
    g.last_item_data.NavRect = nav_bb_arg ? *nav_bb_arg : bb;
    g.last_item_data.InFlags = g.current_item_flags | extra_flags;
    g.last_item_data.status_flags = ImGuiItemStatusFlags_None;

    // Directional navigation processing
    if (id != 0)
    {
        keep_alive_id(id);

        // Runs prior to clipping early-out
        //  (a) So that nav_init_request can be honored, for newly opened windows to select a default widget
        //  (b) So that we can scroll up/down past clipped items. This adds a small O(N) cost to regular navigation requests
        //      unfortunately, but it is still limited to one window. It may not scale very well for windows with ten of
        //      thousands of item, but at least NavMoveRequest is only set on user interaction, aka maximum once a frame.
        //      We could early out with "if (is_clipped && !g.nav_init_request) return false;" but when we wouldn't be able
        //      to reach unclipped widgets. This would work if user had explicit scrolling control (e.g. mapped on a stick).
        // We intentionally don't check if g.nav_window != NULL because g.nav_any_request should only be set when it is non null.
        // If we crash on a NULL g.nav_window we need to fix the bug elsewhere.
        window.dc.NavLayersActiveMaskNext |= (1 << window.dcnav_layer_current);
        if (g.nav_id == id || g.NavAnyRequest)
            if (g.nav_window.root_window_for_nav == window.root_window_for_nav)
                if (window == g.nav_window || ((window.flags | g.nav_window.flags) & WindowFlags::NavFlattened))
                    NavProcessItem();

        // [DEBUG] People keep stumbling on this problem and using "" as identifier in the root of a window instead of "##something".
        // Empty identifier are valid and useful in a small amount of cases, but 99.9% of the time you want to use "##something".
        // READ THE FAQ: https://dearimgui.org/faq
        IM_ASSERT(id != window.id && "Cannot have an empty id at the root of a window. If you need an empty label, use ## and read the FAQ about how the id Stack works!");

        // [DEBUG] Item Picker tool, when enabling the "extended" version we perform the check in ItemAdd()
#ifdef IMGUI_DEBUG_TOOL_ITEM_PICKER_EX
        if (id == g.DebugItemPickerBreakId)
        {
            IM_DEBUG_BREAK();
            g.DebugItemPickerBreakId = 0;
        }

    }
    g.NextItemData.flags = ImGuiNextItemDataFlags_None;

#ifdef IMGUI_ENABLE_TEST_ENGINE
    if (id != 0)
        IMGUI_TEST_ENGINE_ITEM_ADD(nav_bb_arg ? *nav_bb_arg : bb, id);


    // Clipping test
    const bool is_clipped = IsClippedEx(bb, id);
    if (is_clipped)
        return false;
    //if (g.io.key_alt) window->draw_list->add_rect(bb.min, bb.max, IM_COL32(255,255,0,120)); // [DEBUG]

    // We need to calculate this now to take account of the current clipping rectangle (as items like Selectable may change them)
    if (IsMouseHoveringRect(bb.min, bb.max))
        g.last_item_data.status_flags |= ImGuiItemStatusFlags_HoveredRect;
    return true;
}

// Gets back to previous line and continue with horizontal layout
//      offset_from_start_x == 0 : follow right after previous item
//      offset_from_start_x != 0 : align to specified x position (relative to window/group left)
//      spacing_w < 0            : use default spacing if pos_x == 0, no spacing if pos_x != 0
//      spacing_w >= 0           : enforce spacing amount
void SameLine(float offset_from_start_x, float spacing_w)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    if (window.skip_items)
        return;

    if (offset_from_start_x != 0.0)
    {
        if (spacing_w < 0.0)
            spacing_w = 0.0;
        window.dc.cursor_pos.x = window.pos.x - window.scroll.x + offset_from_start_x + spacing_w + window.dc.GroupOffset.x + window.dc.ColumnsOffset.x;
        window.dc.cursor_pos.y = window.dc.CursorPosPrevLine.y;
    }
    else
    {
        if (spacing_w < 0.0)
            spacing_w = g.style.ItemSpacing.x;
        window.dc.cursor_pos.x = window.dc.CursorPosPrevLine.x + spacing_w;
        window.dc.cursor_pos.y = window.dc.CursorPosPrevLine.y;
    }
    window.dc.CurrLineSize = window.dc.PrevLineSize;
    window.dc.CurrLineTextBaseOffset = window.dc.PrevLineTextBaseOffset;
    window.dc.IsSameLine = true;
}

Vector2D GetCursorScreenPos()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.dc.cursor_pos;
}

void SetCursorScreenPos(const Vector2D& pos)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.dc.cursor_pos = pos;
    window.dc.cursor_max_pos = ImMax(window.dc.cursor_max_pos, window.dc.cursor_pos);
}

// User generally sees positions in window coordinates. Internally we store CursorPos in absolute screen coordinates because it is more convenient.
// Conversion happens as we pass the value to user, but it makes our naming convention confusing because GetCursorPos() == (dc.cursor_pos - window.pos). May want to rename 'dc.cursor_pos'.
Vector2D GetCursorPos()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.dc.cursor_pos - window.pos + window.scroll;
}

float GetCursorPosX()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.dc.cursor_pos.x - window.pos.x + window.scroll.x;
}

float GetCursorPosY()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.dc.cursor_pos.y - window.pos.y + window.scroll.y;
}

void SetCursorPos(const Vector2D& local_pos)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.dc.cursor_pos = window.pos - window.scroll + local_pos;
    window.dc.cursor_max_pos = ImMax(window.dc.cursor_max_pos, window.dc.cursor_pos);
}

void SetCursorPosX(float x)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.dc.cursor_pos.x = window.pos.x - window.scroll.x + x;
    window.dc.cursor_max_pos.x = ImMax(window.dc.cursor_max_pos.x, window.dc.cursor_pos.x);
}

void SetCursorPosY(float y)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.dc.cursor_pos.y = window.pos.y - window.scroll.y + y;
    window.dc.cursor_max_pos.y = ImMax(window.dc.cursor_max_pos.y, window.dc.cursor_pos.y);
}

Vector2D GetCursorStartPos()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.dc.cursor_start_pos - window.pos;
}

void Indent(float indent_w)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = GetCurrentWindow();
    window.dc.Indent.x += (indent_w != 0.0) ? indent_w : g.style.IndentSpacing;
    window.dc.cursor_pos.x = window.pos.x + window.dc.Indent.x + window.dc.ColumnsOffset.x;
}

void Unindent(float indent_w)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = GetCurrentWindow();
    window.dc.Indent.x -= (indent_w != 0.0) ? indent_w : g.style.IndentSpacing;
    window.dc.cursor_pos.x = window.pos.x + window.dc.Indent.x + window.dc.ColumnsOffset.x;
}

// Affect large frame+labels widgets only.
//void SetNextItemWidth(float item_width)
pub fn SetNextItemWidth(item_width: f32)
{
    // ImGuiContext& g = *GImGui;
    GImGui.NextItemData.flags |= ImGuiNextItemDataFlags::ImGuiNextItemDataFlags_HasWidth;
    GImGui.NextItemData.Width = item_width;
}

// FIXME: Remove the == 0.0 behavior?
void PushItemWidth(float item_width)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    window.dc.ItemWidthStack.push_back(window.dc.ItemWidth); // Backup current width
    window.dc.ItemWidth = (item_width == 0.0 ? window.ItemWidthDefault : item_width);
    g.NextItemData.flags &= ~ImGuiNextItemDataFlags_HasWidth;
}

void PushMultiItemsWidths(int components, float w_full)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    const ImGuiStyle& style = g.style;
    const float w_item_one  = ImMax(1.0, f32::floor((w_full - (style.item_inner_spacing.x) * (components - 1)) / components));
    const float w_item_last = ImMax(1.0, f32::floor(w_full - (w_item_one + style.item_inner_spacing.x) * (components - 1)));
    window.dc.ItemWidthStack.push_back(window.dc.ItemWidth); // Backup current width
    window.dc.ItemWidthStack.push_back(w_item_last);
    for (int i = 0; i < components - 2; i += 1)
        window.dc.ItemWidthStack.push_back(w_item_one);
    window.dc.ItemWidth = (components == 1) ? w_item_last : w_item_one;
    g.NextItemData.flags &= ~ImGuiNextItemDataFlags_HasWidth;
}

void PopItemWidth()
{
    ImGuiWindow* window = GetCurrentWindow();
    window.dc.ItemWidth = window.dc.ItemWidthStack.back();
    window.dc.ItemWidthStack.pop_back();
}

// Calculate default item width given value passed to PushItemWidth() or SetNextItemWidth().
// The SetNextItemWidth() data is generally cleared/consumed by ItemAdd() or next_item_data.ClearFlags()
float CalcItemWidth()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    float w;
    if (g.NextItemData.flags & ImGuiNextItemDataFlags_HasWidth)
        w = g.NextItemData.Width;
    else
        w = window.dc.ItemWidth;
    if (w < 0.0)
    {
        float region_max_x = GetContentRegionMaxAbs().x;
        w = ImMax(1.0, region_max_x - window.dc.cursor_pos.x + w);
    }
    w = f32::floor(w);
    return w;
}

// [Internal] Calculate full item size given user provided 'size' parameter and default width/height. Default width is often == CalcItemWidth().
// Those two functions CalcItemWidth vs CalcItemSize are awkwardly named because they are not fully symmetrical.
// Note that only CalcItemWidth() is publicly exposed.
// The 4.0 here may be changed to match CalcItemWidth() and/or BeginChild() (right now we have a mismatch which is harmless but undesirable)
Vector2D CalcItemSize(Vector2D size, float default_w, float default_h)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;

    Vector2D region_max;
    if (size.x < 0.0 || size.y < 0.0)
        region_max = GetContentRegionMaxAbs();

    if (size.x == 0.0)
        size.x = default_w;
    else if (size.x < 0.0)
        size.x = ImMax(4.0, region_max.x - window.dc.cursor_pos.x + size.x);

    if (size.y == 0.0)
        size.y = default_h;
    else if (size.y < 0.0)
        size.y = ImMax(4.0, region_max.y - window.dc.cursor_pos.y + size.y);

    return size;
}

float GetTextLineHeight()
{
    ImGuiContext& g = *GImGui;
    return g.font_size;
}

float GetTextLineHeightWithSpacing()
{
    ImGuiContext& g = *GImGui;
    return g.font_size + g.style.ItemSpacing.y;
}

float GetFrameHeight()
{
    ImGuiContext& g = *GImGui;
    return g.font_size + g.style.frame_padding.y * 2.0;
}

float GetFrameHeightWithSpacing()
{
    ImGuiContext& g = *GImGui;
    return g.font_size + g.style.frame_padding.y * 2.0 + g.style.ItemSpacing.y;
}

// FIXME: All the Contents Region function are messy or misleading. WE WILL AIM TO OBSOLETE ALL OF THEM WITH A NEW "WORK RECT" API. Thanks for your patience!

// FIXME: This is in window space (not screen space!).
Vector2D GetContentRegionMax()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    Vector2D mx = window.ContentRegionRect.max - window.pos;
    if (window.dc.CurrentColumns || g.CurrentTable)
        mx.x = window.WorkRect.max.x - window.pos.x;
    return mx;
}

// [Internal] Absolute coordinate. Saner. This is not exposed until we finishing refactoring work rect features.
Vector2D GetContentRegionMaxAbs()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    Vector2D mx = window.ContentRegionRect.max;
    if (window.dc.CurrentColumns || g.CurrentTable)
        mx.x = window.WorkRect.max.x;
    return mx;
}

Vector2D GetContentRegionAvail()
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    return GetContentRegionMaxAbs() - window.dc.cursor_pos;
}

// In window space (not screen space!)
Vector2D GetWindowContentRegionMin()
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    return window.ContentRegionRect.min - window.pos;
}

Vector2D GetWindowContentRegionMax()
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    return window.ContentRegionRect.max - window.pos;
}

// Lock horizontal starting position + capture group bounding box into one "item" (so you can use IsItemHovered() or layout primitives such as SameLine() on whole group, etc.)
// Groups are currently a mishmash of functionalities which should perhaps be clarified and separated.
// FIXME-OPT: Could we safely early out on ->skip_items?
void BeginGroup()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;

    g.group_stack.resize(g.group_stack.size + 1);
    ImGuiGroupData& group_data = g.group_stack.back();
    group_data.WindowID = window.id;
    group_data.BackupCursorPos = window.dc.cursor_pos;
    group_data.BackupCursorMaxPos = window.dc.cursor_max_pos;
    group_data.BackupIndent = window.dc.Indent;
    group_data.BackupGroupOffset = window.dc.GroupOffset;
    group_data.BackupCurrLineSize = window.dc.CurrLineSize;
    group_data.BackupCurrLineTextBaseOffset = window.dc.CurrLineTextBaseOffset;
    group_data.BackupActiveIdIsAlive = g.active_id_is_alive;
    group_data.BackupHoveredIdIsAlive = g.hovered_id != 0;
    group_data.BackupActiveIdPreviousFrameIsAlive = g.active_id_previous_frame_is_alive;
    group_data.EmitItem = true;

    window.dc.GroupOffset.x = window.dc.cursor_pos.x - window.pos.x - window.dc.ColumnsOffset.x;
    window.dc.Indent = window.dc.GroupOffset;
    window.dc.cursor_max_pos = window.dc.cursor_pos;
    window.dc.CurrLineSize = Vector2D::new(0.0, 0.0);
    if (g.LogEnabled)
        g.log_line_pos_y = -f32::MAX; // To enforce a carriage return
}

void EndGroup()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    IM_ASSERT(g.group_stack.size > 0); // Mismatched BeginGroup()/EndGroup() calls

    ImGuiGroupData& group_data = g.group_stack.back();
    IM_ASSERT(group_data.WindowID == window.id); // EndGroup() in wrong window?

    Rect group_bb(group_data.BackupCursorPos, ImMax(window.dc.cursor_max_pos, group_data.BackupCursorPos));

    window.dc.cursor_pos = group_data.BackupCursorPos;
    window.dc.cursor_max_pos = ImMax(group_data.BackupCursorMaxPos, window.dc.cursor_max_pos);
    window.dc.Indent = group_data.BackupIndent;
    window.dc.GroupOffset = group_data.BackupGroupOffset;
    window.dc.CurrLineSize = group_data.BackupCurrLineSize;
    window.dc.CurrLineTextBaseOffset = group_data.BackupCurrLineTextBaseOffset;
    if (g.LogEnabled)
        g.log_line_pos_y = -f32::MAX; // To enforce a carriage return

    if (!group_data.EmitItem)
    {
        g.group_stack.pop_back();
        return;
    }

    window.dc.CurrLineTextBaseOffset = ImMax(window.dc.PrevLineTextBaseOffset, group_data.BackupCurrLineTextBaseOffset);      // FIXME: Incorrect, we should grab the base offset from the *first line* of the group but it is hard to obtain now.
    item_size(group_bb.GetSize());
    item_add(group_bb, 0, NULL, ImGuiItemFlags_NoTabStop);

    // If the current active_id was declared within the boundary of our group, we copy it to LastItemId so IsItemActive(), is_item_deactivated() etc. will be functional on the entire group.
    // It would be be neater if we replaced window.dc.LastItemId by e.g. 'bool LastItemIsActive', but would put a little more burden on individual widgets.
    // Also if you grep for LastItemId you'll notice it is only used in that context.
    // (The two tests not the same because active_id_is_alive is an id itself, in order to be able to handle active_id being overwritten during the frame.)
    const bool group_contains_curr_active_id = (group_data.BackupActiveIdIsAlive != g.active_id) && (g.active_id_is_alive == g.active_id) && g.active_id;
    const bool group_contains_prev_active_id = (group_data.BackupActiveIdPreviousFrameIsAlive == false) && (g.active_id_previous_frame_is_alive == true);
    if (group_contains_curr_active_id)
        g.last_item_data.id = g.active_id;
    else if (group_contains_prev_active_id)
        g.last_item_data.id = g.active_id_previous_frame;
    g.last_item_data.Rect = group_bb;

    // Forward Hovered flag
    const bool group_contains_curr_hovered_id = (group_data.BackupHoveredIdIsAlive == false) && g.hovered_id != 0;
    if (group_contains_curr_hovered_id)
        g.last_item_data.status_flags |= ItemStatusFlags::HoveredWindow;

    // Forward Edited flag
    if (group_contains_curr_active_id && g.active_id_has_been_edited_this_frame)
        g.last_item_data.status_flags |= ImGuiItemStatusFlags_Edited;

    // Forward Deactivated flag
    g.last_item_data.status_flags |= ImGuiItemStatusFlags_HasDeactivated;
    if (group_contains_prev_active_id && g.active_id != g.active_id_previous_frame)
        g.last_item_data.status_flags |= ImGuiItemStatusFlags_Deactivated;

    g.group_stack.pop_back();
    //window->draw_list->add_rect(group_bb.min, group_bb.max, IM_COL32(255,0,255,255));   // [Debug]
}


//-----------------------------------------------------------------------------
// [SECTION] SCROLLING
//-----------------------------------------------------------------------------

// Helper to snap on edges when aiming at an item very close to the edge,
// So the difference between window_padding and ItemSpacing will be in the visible area after scrolling.
// When we refactor the scrolling API this may be configurable with a flag?
// Note that the effect for this won't be visible on x axis with default style settings as window_padding.x == ItemSpacing.x by default.
static float CalcScrollEdgeSnap(float target, float snap_min, float snap_max, float snap_threshold, float center_ratio)
{
    if (target <= snap_min + snap_threshold)
        return ImLerp(snap_min, target, center_ratio);
    if (target >= snap_max - snap_threshold)
        return ImLerp(target, snap_max, center_ratio);
    return target;
}

static Vector2D CalcNextScrollFromScrollTargetAndClamp(ImGuiWindow* window)
{
    Vector2D scroll = window.scroll;
    if (window.ScrollTarget.x < f32::MAX)
    {
        float decoration_total_width = window.scrollbar_sizes.x;
        float center_x_ratio = window.ScrollTargetCenterRatio.x;
        float scroll_target_x = window.ScrollTarget.x;
        if (window.ScrollTargetEdgeSnapDist.x > 0.0)
        {
            float snap_x_min = 0.0;
            float snap_x_max = window.scroll_max.x + window.size_full.x - decoration_total_width;
            scroll_target_x = CalcScrollEdgeSnap(scroll_target_x, snap_x_min, snap_x_max, window.ScrollTargetEdgeSnapDist.x, center_x_ratio);
        }
        scroll.x = scroll_target_x - center_x_ratio * (window.size_full.x - decoration_total_width);
    }
    if (window.ScrollTarget.y < f32::MAX)
    {
        float decoration_total_height = window.title_bar_height() + window.MenuBarHeight() + window.scrollbar_sizes.y;
        float center_y_ratio = window.ScrollTargetCenterRatio.y;
        float scroll_target_y = window.ScrollTarget.y;
        if (window.ScrollTargetEdgeSnapDist.y > 0.0)
        {
            float snap_y_min = 0.0;
            float snap_y_max = window.scroll_max.y + window.size_full.y - decoration_total_height;
            scroll_target_y = CalcScrollEdgeSnap(scroll_target_y, snap_y_min, snap_y_max, window.ScrollTargetEdgeSnapDist.y, center_y_ratio);
        }
        scroll.y = scroll_target_y - center_y_ratio * (window.size_full.y - decoration_total_height);
    }
    scroll.x = f32::floor(ImMax(scroll.x, 0.0));
    scroll.y = f32::floor(ImMax(scroll.y, 0.0));
    if (!window.collapsed && !window.skip_items)
    {
        scroll.x = ImMin(scroll.x, window.scroll_max.x);
        scroll.y = ImMin(scroll.y, window.scroll_max.y);
    }
    return scroll;
}

void ScrollToItem(ImGuiScrollFlags flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    ScrollToRectEx(window, g.last_item_data.NavRect, flags);
}

void ScrollToRect(ImGuiWindow* window, const Rect& item_rect, ImGuiScrollFlags flags)
{
    ScrollToRectEx(window, item_rect, flags);
}

// scroll to keep newly navigated item fully into view
Vector2D ScrollToRectEx(ImGuiWindow* window, const Rect& item_rect, ImGuiScrollFlags flags)
{
    ImGuiContext& g = *GImGui;
    Rect window_rect(window.inner_rect.min - Vector2D::new(1, 1), window.inner_rect.max + Vector2D::new(1, 1));
    //GetForegroundDrawList(window)->add_rect(window_rect.min, window_rect.max, IM_COL32_WHITE); // [DEBUG]

    // Check that only one behavior is selected per axis
    IM_ASSERT((flags & ImGuiScrollFlags_MaskX_) == 0 || ImIsPowerOfTwo(flags & ImGuiScrollFlags_MaskX_));
    IM_ASSERT((flags & ImGuiScrollFlags_MaskY_) == 0 || ImIsPowerOfTwo(flags & ImGuiScrollFlags_MaskY_));

    // Defaults
    ImGuiScrollFlags in_flags = flags;
    if ((flags & ImGuiScrollFlags_MaskX_) == 0 && window.scrollbar_x)
        flags |= ImGuiScrollFlags_KeepVisibleEdgeX;
    if ((flags & ImGuiScrollFlags_MaskY_) == 0)
        flags |= window.Appearing ? ImGuiScrollFlags_AlwaysCenterY : ImGuiScrollFlags_KeepVisibleEdgeY;

    const bool fully_visible_x = item_rect.min.x >= window_rect.min.x && item_rect.max.x <= window_rect.max.x;
    const bool fully_visible_y = item_rect.min.y >= window_rect.min.y && item_rect.max.y <= window_rect.max.y;
    const bool can_be_fully_visible_x = (item_rect.get_width() + g.style.ItemSpacing.x * 2.0) <= window_rect.get_width();
    const bool can_be_fully_visible_y = (item_rect.get_height() + g.style.ItemSpacing.y * 2.0) <= window_rect.get_height();

    if ((flags & ImGuiScrollFlags_KeepVisibleEdgeX) && !fully_visible_x)
    {
        if (item_rect.min.x < window_rect.min.x || !can_be_fully_visible_x)
            SetScrollFromPosX(window, item_rect.min.x - g.style.ItemSpacing.x - window.pos.x, 0.0);
        else if (item_rect.max.x >= window_rect.max.x)
            SetScrollFromPosX(window, item_rect.max.x + g.style.ItemSpacing.x - window.pos.x, 1.0);
    }
    else if (((flags & ImGuiScrollFlags_KeepVisibleCenterX) && !fully_visible_x) || (flags & ImGuiScrollFlags_AlwaysCenterX))
    {
        float target_x = can_be_fully_visible_x ? f32::floor((item_rect.min.x + item_rect.max.x - window.inner_rect.get_width()) * 0.5) : item_rect.min.x;
        SetScrollFromPosX(window, target_x - window.pos.x, 0.0);
    }

    if ((flags & ImGuiScrollFlags_KeepVisibleEdgeY) && !fully_visible_y)
    {
        if (item_rect.min.y < window_rect.min.y || !can_be_fully_visible_y)
            SetScrollFromPosY(window, item_rect.min.y - g.style.ItemSpacing.y - window.pos.y, 0.0);
        else if (item_rect.max.y >= window_rect.max.y)
            SetScrollFromPosY(window, item_rect.max.y + g.style.ItemSpacing.y - window.pos.y, 1.0);
    }
    else if (((flags & ImGuiScrollFlags_KeepVisibleCenterY) && !fully_visible_y) || (flags & ImGuiScrollFlags_AlwaysCenterY))
    {
        float target_y = can_be_fully_visible_y ? f32::floor((item_rect.min.y + item_rect.max.y - window.inner_rect.get_height()) * 0.5) : item_rect.min.y;
        SetScrollFromPosY(window, target_y - window.pos.y, 0.0);
    }

    Vector2D next_scroll = CalcNextScrollFromScrollTargetAndClamp(window);
    Vector2D delta_scroll = next_scroll - window.scroll;

    // Also scroll parent window to keep us into view if necessary
    if (!(flags & ImGuiScrollFlags_NoScrollParent) && (window.flags & WindowFlags::ChildWindow))
    {
        // FIXME-SCROLL: May be an option?
        if ((in_flags & (ImGuiScrollFlags_AlwaysCenterX | ImGuiScrollFlags_KeepVisibleCenterX)) != 0)
            in_flags = (in_flags & ~ImGuiScrollFlags_MaskX_) | ImGuiScrollFlags_KeepVisibleEdgeX;
        if ((in_flags & (ImGuiScrollFlags_AlwaysCenterY | ImGuiScrollFlags_KeepVisibleCenterY)) != 0)
            in_flags = (in_flags & ~ImGuiScrollFlags_MaskY_) | ImGuiScrollFlags_KeepVisibleEdgeY;
        delta_scroll += ScrollToRectEx(window.parent_window, Rect(item_rect.min - delta_scroll, item_rect.max - delta_scroll), in_flags);
    }

    return delta_scroll;
}

float GetScrollX()
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    return window.scroll.x;
}

float GetScrollY()
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    return window.scroll.y;
}

float GetScrollMaxX()
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    return window.scroll_max.x;
}

float GetScrollMaxY()
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    return window.scroll_max.y;
}

void set_scroll_x(ImGuiWindow* window, float scroll_x)
{
    window.ScrollTarget.x = scroll_x;
    window.ScrollTargetCenterRatio.x = 0.0;
    window.ScrollTargetEdgeSnapDist.x = 0.0;
}

void set_scroll_y(ImGuiWindow* window, float scroll_y)
{
    window.ScrollTarget.y = scroll_y;
    window.ScrollTargetCenterRatio.y = 0.0;
    window.ScrollTargetEdgeSnapDist.y = 0.0;
}

void set_scroll_x(float scroll_x)
{
    ImGuiContext& g = *GImGui;
    set_scroll_x(g.current_window, scroll_x);
}

void set_scroll_y(float scroll_y)
{
    ImGuiContext& g = *GImGui;
    set_scroll_y(g.current_window, scroll_y);
}

// Note that a local position will vary depending on initial scroll value,
// This is a little bit confusing so bear with us:
//  - local_pos = (absolution_pos - window->pos)
//  - So local_x/local_y are 0.0 for a position at the upper-left corner of a window,
//    and generally local_x/local_y are >(padding+decoration) && <(size-padding-decoration) when in the visible area.
//  - They mostly exists because of legacy API.
// Following the rules above, when trying to work with scrolling code, consider that:
//  - SetScrollFromPosY(0.0) == SetScrollY(0.0 + scroll.y) == has no effect!
//  - SetScrollFromPosY(-scroll.y) == SetScrollY(-scroll.y + scroll.y) == SetScrollY(0.0) == reset scroll. Of course writing SetScrollY(0.0) directly then makes more sense
// We store a target position so centering and clamping can occur on the next frame when we are guaranteed to have a known window size
void SetScrollFromPosX(ImGuiWindow* window, float local_x, float center_x_ratio)
{
    IM_ASSERT(center_x_ratio >= 0.0 && center_x_ratio <= 1.0);
    window.ScrollTarget.x = f32::floor(local_x + window.scroll.x); // Convert local position to scroll offset
    window.ScrollTargetCenterRatio.x = center_x_ratio;
    window.ScrollTargetEdgeSnapDist.x = 0.0;
}

void SetScrollFromPosY(ImGuiWindow* window, float local_y, float center_y_ratio)
{
    IM_ASSERT(center_y_ratio >= 0.0 && center_y_ratio <= 1.0);
    const float decoration_up_height = window.title_bar_height() + window.MenuBarHeight(); // FIXME: Would be nice to have a more standardized access to our scrollable/client rect;
    local_y -= decoration_up_height;
    window.ScrollTarget.y = f32::floor(local_y + window.scroll.y); // Convert local position to scroll offset
    window.ScrollTargetCenterRatio.y = center_y_ratio;
    window.ScrollTargetEdgeSnapDist.y = 0.0;
}

void SetScrollFromPosX(float local_x, float center_x_ratio)
{
    ImGuiContext& g = *GImGui;
    SetScrollFromPosX(g.current_window, local_x, center_x_ratio);
}

void SetScrollFromPosY(float local_y, float center_y_ratio)
{
    ImGuiContext& g = *GImGui;
    SetScrollFromPosY(g.current_window, local_y, center_y_ratio);
}

// center_x_ratio: 0.0 left of last item, 0.5 horizontal center of last item, 1.0 right of last item.
void SetScrollHereX(float center_x_ratio)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    float spacing_x = ImMax(window.WindowPadding.x, g.style.ItemSpacing.x);
    float target_pos_x = ImLerp(g.last_item_data.Rect.min.x - spacing_x, g.last_item_data.Rect.max.x + spacing_x, center_x_ratio);
    SetScrollFromPosX(window, target_pos_x - window.pos.x, center_x_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.ScrollTargetEdgeSnapDist.x = ImMax(0.0, window.WindowPadding.x - spacing_x);
}

// center_y_ratio: 0.0 top of last item, 0.5 vertical center of last item, 1.0 bottom of last item.
void SetScrollHereY(float center_y_ratio)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    float spacing_y = ImMax(window.WindowPadding.y, g.style.ItemSpacing.y);
    float target_pos_y = ImLerp(window.dc.CursorPosPrevLine.y - spacing_y, window.dc.CursorPosPrevLine.y + window.dc.PrevLineSize.y + spacing_y, center_y_ratio);
    SetScrollFromPosY(window, target_pos_y - window.pos.y, center_y_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.ScrollTargetEdgeSnapDist.y = ImMax(0.0, window.WindowPadding.y - spacing_y);
}

//-----------------------------------------------------------------------------
// [SECTION] TOOLTIPS
//-----------------------------------------------------------------------------

void BeginTooltip()
{
    BeginTooltipEx(ImGuiTooltipFlags_None, WindowFlags::None);
}

void BeginTooltipEx(ImGuiTooltipFlags tooltip_flags, ImGuiWindowFlags extra_window_flags)
{
    ImGuiContext& g = *GImGui;

    if (g.drag_drop_within_source || g.drag_drop_within_target)
    {
        // The default tooltip position is a little offset to give space to see the context menu (it's also clamped within the current viewport/monitor)
        // In the context of a dragging tooltip we try to reduce that offset and we enforce following the cursor.
        // Whatever we do we want to call SetNextWindowPos() to enforce a tooltip position and disable clipping the tooltip without our display area, like regular tooltip do.
        //Vector2D tooltip_pos = g.io.mouse_pos - g.ActiveIdClickOffset - g.style.window_padding;
        Vector2D tooltip_pos = g.io.mouse_pos + Vector2D::new(16 * g.style.MouseCursorScale, 8 * g.style.MouseCursorScale);
        SetNextWindowPos(tooltip_pos);
        SetNextWindowBgAlpha(g.style.colors[StyleColor::PopupBg].w * 0.60);
        //PushStyleVar(ImGuiStyleVar_Alpha, g.style.Alpha * 0.60); // This would be nice but e.g ColorButton with checkboard has issue with transparent colors :(
        tooltip_flags |= ImGuiTooltipFlags_OverridePreviousTooltip;
    }

    char window_name[16];
    ImFormatString(window_name, IM_ARRAYSIZE(window_name), "##Tooltip_%02d", g.tool_tip_override_count);
    if (tooltip_flags & ImGuiTooltipFlags_OverridePreviousTooltip)
        if (ImGuiWindow* window = FindWindowByName(window_name))
            if (window.active)
            {
                // Hide previous tooltip from being displayed. We can't easily "reset" the content of a window so we create a new one.
                window.hidden = true;
                window..hidden_frames_can_skip_items = 1; // FIXME: This may not be necessary?
                ImFormatString(window_name, IM_ARRAYSIZE(window_name), "##Tooltip_%02d", g.tool_tip_override_count += 1);
            }
    ImGuiWindowFlags flags = WindowFlags::Tooltip | WindowFlags::NoInputs | WindowFlags::NoTitleBar | WindowFlags::NoMove | WindowFlags::NoResize | WindowFlags::NoSavedSettings | WindowFlags::AlwaysAutoResize | WindowFlags::NoDocking;
    begin(window_name, NULL, flags | extra_window_flags);
}

void EndTooltip()
{
    IM_ASSERT(GetCurrentWindowRead().flags & WindowFlags::Tooltip);   // Mismatched BeginTooltip()/EndTooltip() calls
    end();
}

void SetTooltipV(const char* fmt, va_list args)
{
    BeginTooltipEx(ImGuiTooltipFlags_OverridePreviousTooltip, WindowFlags::None);
    TextV(fmt, args);
    EndTooltip();
}

void SetTooltip(const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    SetTooltipV(fmt, args);
    va_end(args);
}

//-----------------------------------------------------------------------------
// [SECTION] POPUPS
//-----------------------------------------------------------------------------

// Supported flags: ImGuiPopupFlags_AnyPopupId, ImGuiPopupFlags_AnyPopupLevel
bool IsPopupOpen(ImGuiID id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    if (popup_flags & ImGuiPopupFlags_AnyPopupId)
    {
        // Return true if any popup is open at the current BeginPopup() level of the popup stack
        // This may be used to e.g. test for another popups already opened to handle popups priorities at the same level.
        IM_ASSERT(id == 0);
        if (popup_flags & ImGuiPopupFlags_AnyPopupLevel)
            return g.open_popup_stack.size > 0;
        else
            return g.open_popup_stack.size > g.begin_popup_stack.size;
    }
    else
    {
        if (popup_flags & ImGuiPopupFlags_AnyPopupLevel)
        {
            // Return true if the popup is open anywhere in the popup stack
            for (int n = 0; n < g.open_popup_stack.size; n += 1)
                if (g.open_popup_stack[n].PopupId == id)
                    return true;
            return false;
        }
        else
        {
            // Return true if the popup is open at the current BeginPopup() level of the popup stack (this is the most-common query)
            return g.open_popup_stack.size > g.begin_popup_stack.size && g.open_popup_stack[g.begin_popup_stack.size].PopupId == id;
        }
    }
}

bool IsPopupOpen(const char* str_id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiID id = (popup_flags & ImGuiPopupFlags_AnyPopupId) ? 0 : g.current_window.get_id(str_id);
    if ((popup_flags & ImGuiPopupFlags_AnyPopupLevel) && id != 0)
        IM_ASSERT(0 && "Cannot use IsPopupOpen() with a string id and ImGuiPopupFlags_AnyPopupLevel."); // But non-string version is legal and used internally
    return IsPopupOpen(id, popup_flags);
}

ImGuiWindow* get_top_most_popup_modal()
{
    ImGuiContext& g = *GImGui;
    for (int n = g.open_popup_stack.size - 1; n >= 0; n--)
        if (ImGuiWindow* popup = g.open_popup_stack.data[n].Window)
            if (popup.flags & WindowFlags::Modal)
                return popup;
    return NULL;
}

ImGuiWindow* GetTopMostAndVisiblePopupModal()
{
    ImGuiContext& g = *GImGui;
    for (int n = g.open_popup_stack.size - 1; n >= 0; n--)
        if (ImGuiWindow* popup = g.open_popup_stack.data[n].Window)
            if ((popup.flags & WindowFlags::Modal) && is_window_active_and_visible(popup))
                return popup;
    return NULL;
}

void OpenPopup(const char* str_id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiID id = g.current_window.get_id(str_id);
    IMGUI_DEBUG_LOG_POPUP("[popup] OpenPopup(\"%s\" -> 0x%08X\n", str_id, id);
    OpenPopupEx(id, popup_flags);
}

void OpenPopup(ImGuiID id, ImGuiPopupFlags popup_flags)
{
    OpenPopupEx(id, popup_flags);
}

// Mark popup as open (toggle toward open state).
// Popups are closed when user click outside, or activate a pressable item, or CloseCurrentPopup() is called within a BeginPopup()/EndPopup() block.
// Popup identifiers are relative to the current id-stack (so OpenPopup and BeginPopup needs to be at the same level).
// One open popup per level of the popup hierarchy (NB: when assigning we reset the window member of ImGuiPopupRef to NULL)
void OpenPopupEx(ImGuiID id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* parent_window = g.current_window;
    const int current_stack_size = g.begin_popup_stack.size;

    if (popup_flags & ImGuiPopupFlags_NoOpenOverExistingPopup)
        if (IsPopupOpen(0u, ImGuiPopupFlags_AnyPopupId))
            return;

    ImGuiPopupData popup_ref; // Tagged as new ref as window will be set back to NULL if we write this into open_popup_stack.
    popup_ref.PopupId = id;
    popup_ref.Window = NULL;
    popup_ref.SourceWindow = g.nav_window;
    popup_ref.OpenFrameCount = g.frame_count;
    popup_ref.OpenParentId = parent_window.IDStack.back();
    popup_ref.OpenPopupPos = NavCalcPreferredRefPos();
    popup_ref.OpenMousePos = is_mouse_pos_valid(&g.io.mouse_pos) ? g.io.mouse_pos : popup_ref.OpenPopupPos;

    IMGUI_DEBUG_LOG_POPUP("[popup] OpenPopupEx(0x%08X)\n", id);
    if (g.open_popup_stack.size < current_stack_size + 1)
    {
        g.open_popup_stack.push_back(popup_ref);
    }
    else
    {
        // Gently handle the user mistakenly calling OpenPopup() every frame. It is a programming mistake! However, if we were to run the regular code path, the ui
        // would become completely unusable because the popup will always be in hidden-while-calculating-size state _while_ claiming focus. Which would be a very confusing
        // situation for the programmer. Instead, we silently allow the popup to proceed, it will keep reappearing and the programming error will be more obvious to understand.
        if (g.open_popup_stack[current_stack_size].PopupId == id && g.open_popup_stack[current_stack_size].OpenFrameCount == g.frame_count - 1)
        {
            g.open_popup_stack[current_stack_size].OpenFrameCount = popup_ref.OpenFrameCount;
        }
        else
        {
            // Close child popups if any, then flag popup for open/reopen
            ClosePopupToLevel(current_stack_size, false);
            g.open_popup_stack.push_back(popup_ref);
        }

        // When reopening a popup we first refocus its parent, otherwise if its parent is itself a popup it would get closed by close_popups_over_window().
        // This is equivalent to what ClosePopupToLevel() does.
        //if (g.open_popup_stack[current_stack_size].popup_id == id)
        //    focus_window(parent_window);
    }
}

// When popups are stacked, clicking on a lower level popups puts focus back to it and close popups above it.
// This function closes any popups that are over 'ref_window'.
void close_popups_over_window(ImGuiWindow* ref_window, bool restore_focus_to_window_under_popup)
{
    ImGuiContext& g = *GImGui;
    if (g.open_popup_stack.size == 0)
        return;

    // Don't close our own child popup windows.
    int popup_count_to_keep = 0;
    if (ref_window)
    {
        // Find the highest popup which is a descendant of the reference window (generally reference window = nav_window)
        for (; popup_count_to_keep < g.open_popup_stack.size; popup_count_to_keep += 1)
        {
            ImGuiPopupData& popup = g.open_popup_stack[popup_count_to_keep];
            if (!popup.Window)
                continue;
            IM_ASSERT((popup.Window.flags & WindowFlags::Popup) != 0);
            if (popup.Window.flags & WindowFlags::ChildWindow)
                continue;

            // Trim the stack unless the popup is a direct parent of the reference window (the reference window is often the nav_window)
            // - With this stack of window, clicking/focusing Popup1 will close Popup2 and Popup3:
            //     window -> Popup1 -> Popup2 -> Popup3
            // - Each popups may contain child windows, which is why we compare ->root_window_dock_tree!
            //     window -> Popup1 -> Popup1_Child -> Popup2 -> Popup2_Child
            bool ref_window_is_descendent_of_popup = false;
            for (int n = popup_count_to_keep; n < g.open_popup_stack.size; n += 1)
                if (ImGuiWindow* popup_window = g.open_popup_stack[n].Window)
                    //if (popup_window->root_window_dock_tree == ref_window->root_window_dock_tree) // FIXME-MERGE
                    if (is_window_within_begin_stack_of(ref_window, popup_window))
                    {
                        ref_window_is_descendent_of_popup = true;
                        break;
                    }
            if (!ref_window_is_descendent_of_popup)
                break;
        }
    }
    if (popup_count_to_keep < g.open_popup_stack.size) // This test is not required but it allows to set a convenient breakpoint on the statement below
    {
        IMGUI_DEBUG_LOG_POPUP("[popup] close_popups_over_window(\"%s\")\n", ref_window ? ref_window.Name : "<NULL>");
        ClosePopupToLevel(popup_count_to_keep, restore_focus_to_window_under_popup);
    }
}

void ClosePopupsExceptModals()
{
    ImGuiContext& g = *GImGui;

    int popup_count_to_keep;
    for (popup_count_to_keep = g.open_popup_stack.size; popup_count_to_keep > 0; popup_count_to_keep--)
    {
        ImGuiWindow* window = g.open_popup_stack[popup_count_to_keep - 1].Window;
        if (!window || window.flags & WindowFlags::Modal)
            break;
    }
    if (popup_count_to_keep < g.open_popup_stack.size) // This test is not required but it allows to set a convenient breakpoint on the statement below
        ClosePopupToLevel(popup_count_to_keep, true);
}

void ClosePopupToLevel(int remaining, bool restore_focus_to_window_under_popup)
{
    ImGuiContext& g = *GImGui;
    IMGUI_DEBUG_LOG_POPUP("[popup] ClosePopupToLevel(%d), restore_focus_to_window_under_popup=%d\n", remaining, restore_focus_to_window_under_popup);
    IM_ASSERT(remaining >= 0 && remaining < g.open_popup_stack.size);

    // Trim open popup stack
    ImGuiWindow* focus_window = g.open_popup_stack[remaining].SourceWindow;
    ImGuiWindow* popup_window = g.open_popup_stack[remaining].Window;
    g.open_popup_stack.resize(remaining);

    if (restore_focus_to_window_under_popup)
    {
        if (focus_window && !focus_window.was_active && popup_window)
        {
            // Fallback
            FocusTopMostWindowUnderOne(popup_window, NULL);
        }
        else
        {
            if (g.NavLayer == NavLayer::Main && focus_window)
                focus_window = NavRestoreLastChildNavWindow(focus_window);
            focus_window(focus_window);
        }
    }
}

// Close the popup we have begin-ed into.
void CloseCurrentPopup()
{
    ImGuiContext& g = *GImGui;
    int popup_idx = g.begin_popup_stack.size - 1;
    if (popup_idx < 0 || popup_idx >= g.open_popup_stack.size || g.begin_popup_stack[popup_idx].PopupId != g.open_popup_stack[popup_idx].PopupId)
        return;

    // Closing a menu closes its top-most parent popup (unless a modal)
    while (popup_idx > 0)
    {
        ImGuiWindow* popup_window = g.open_popup_stack[popup_idx].Window;
        ImGuiWindow* parent_popup_window = g.open_popup_stack[popup_idx - 1].Window;
        bool close_parent = false;
        if (popup_window && (popup_window.flags & WindowFlags::ChildMenu))
            if (parent_popup_window && !(parent_popup_window.flags & WindowFlags::MenuBar))
                close_parent = true;
        if (!close_parent)
            break;
        popup_idx--;
    }
    IMGUI_DEBUG_LOG_POPUP("[popup] CloseCurrentPopup %d -> %d\n", g.begin_popup_stack.size - 1, popup_idx);
    ClosePopupToLevel(popup_idx, true);

    // A common pattern is to close a popup when selecting a menu item/selectable that will open another window.
    // To improve this usage pattern, we avoid nav highlight for a single frame in the parent window.
    // Similarly, we could avoid mouse hover highlight in this window but it is less visually problematic.
    if (ImGuiWindow* window = g.nav_window)
        window.dc.NavHideHighlightOneFrame = true;
}

// Attention! BeginPopup() adds default flags which BeginPopupEx()!
bool BeginPopupEx(ImGuiID id, ImGuiWindowFlags flags)
{
    ImGuiContext& g = *GImGui;
    if (!IsPopupOpen(id, ImGuiPopupFlags_None))
    {
        g.next_window_data.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }

    char name[20];
    if (flags & WindowFlags::ChildMenu)
        ImFormatString(name, IM_ARRAYSIZE(name), "##Menu_%02d", g.BeginMenuCount); // Recycle windows based on depth
    else
        ImFormatString(name, IM_ARRAYSIZE(name), "##Popup_%08x", id); // Not recycling, so we can close/open during the same frame

    flags |= WindowFlags::Popup | WindowFlags::NoDocking;
    bool is_open = begin(name, NULL, flags);
    if (!is_open) // NB: Begin can return false when the popup is completely clipped (e.g. zero size display)
        EndPopup();

    return is_open;
}

bool BeginPopup(const char* str_id, ImGuiWindowFlags flags)
{
    ImGuiContext& g = *GImGui;
    if (g.open_popup_stack.size <= g.begin_popup_stack.size) // Early out for performance
    {
        g.next_window_data.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }
    flags |= WindowFlags::AlwaysAutoResize | WindowFlags::NoTitleBar | WindowFlags::NoSavedSettings;
    ImGuiID id = g.current_window.get_id(str_id);
    return BeginPopupEx(id, flags);
}

// If 'p_open' is specified for a modal popup window, the popup will have a regular close button which will close the popup.
// Note that popup visibility status is owned by Dear ImGui (and manipulated with e.g. OpenPopup) so the actual value of *p_open is meaningless here.
bool BeginPopupModal(const char* name, bool* p_open, ImGuiWindowFlags flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    const ImGuiID id = window.get_id(name);
    if (!IsPopupOpen(id, ImGuiPopupFlags_None))
    {
        g.next_window_data.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }

    // Center modal windows by default for increased visibility
    // (this won't really last as settings will kick in, and is mostly for backward compatibility. user may do the same themselves)
    // FIXME: Should test for (PosCond & window->set_window_pos_allow_flags) with the upcoming window.
    if ((g.next_window_data.flags & NextWindowDataFlags::HasPos) == 0)
    {
        const ImGuiViewport* viewport = window.was_active ? window.viewport : GetMainViewport(); // FIXME-VIEWPORT: What may be our reference viewport?
        SetNextWindowPos(viewport.GetCenter(), Cond::FirstUseEver, Vector2D::new(0.5, 0.5));
    }

    flags |= WindowFlags::Popup | WindowFlags::Modal | WindowFlags::NoCollapse | WindowFlags::NoDocking;
    const bool is_open = begin(name, p_open, flags);
    if (!is_open || (p_open && !*p_open)) // NB: is_open can be 'false' when the popup is completely clipped (e.g. zero size display)
    {
        EndPopup();
        if (is_open)
            ClosePopupToLevel(g.begin_popup_stack.size, true);
        return false;
    }
    return is_open;
}

void EndPopup()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    IM_ASSERT(window.flags & WindowFlags::Popup);  // Mismatched BeginPopup()/EndPopup() calls
    IM_ASSERT(g.begin_popup_stack.size > 0);

    // Make all menus and popups wrap around for now, may need to expose that policy (e.g. focus scope could include wrap/loop policy flags used by new move requests)
    if (g.nav_window == window)
        NavMoveRequestTryWrapping(window, ImGuiNavMoveFlags_LoopY);

    // Child-popups don't need to be laid out
    IM_ASSERT(g.within_end_child == false);
    if (window.flags & WindowFlags::ChildWindow)
        g.within_end_child = true;
    end();
    g.within_end_child = false;
}

// Helper to open a popup if mouse button is released over the item
// - This is essentially the same as BeginPopupContextItem() but without the trailing BeginPopup()
void OpenPopupOnItemClick(const char* str_id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
    {
        ImGuiID id = str_id ? window.get_id(str_id) : g.last_item_data.id;    // If user hasn't passed an id, we can use the LastItemID. Using LastItemID as a Popup id won't conflict!
        IM_ASSERT(id != 0);                                             // You cannot pass a NULL str_id if the last item has no identifier (e.g. a Text() item)
        OpenPopupEx(id, popup_flags);
    }
}

// This is a helper to handle the simplest case of associating one named popup to one given widget.
// - To create a popup associated to the last item, you generally want to pass a NULL value to str_id.
// - To create a popup with a specific identifier, pass it in str_id.
//    - This is useful when using using BeginPopupContextItem() on an item which doesn't have an identifier, e.g. a Text() call.
//    - This is useful when multiple code locations may want to manipulate/open the same popup, given an explicit id.
// - You may want to handle the whole on user side if you have specific needs (e.g. tweaking IsItemHovered() parameters).
//   This is essentially the same as:
//       id = str_id ? GetID(str_id) : GetItemID();
//       OpenPopupOnItemClick(str_id, ImGuiPopupFlags_MouseButtonRight);
//       return BeginPopup(id);
//   Which is essentially the same as:
//       id = str_id ? GetID(str_id) : GetItemID();
//       if (IsItemHovered() && IsMouseReleased(ImGuiMouseButton_Right))
//           OpenPopup(id);
//       return BeginPopup(id);
//   The main difference being that this is tweaked to avoid computing the id twice.
bool BeginPopupContextItem(const char* str_id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    if (window.skip_items)
        return false;
    ImGuiID id = str_id ? window.get_id(str_id) : g.last_item_data.id;    // If user hasn't passed an id, we can use the LastItemID. Using LastItemID as a Popup id won't conflict!
    IM_ASSERT(id != 0);                                             // You cannot pass a NULL str_id if the last item has no identifier (e.g. a Text() item)
    int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
        OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, WindowFlags::AlwaysAutoResize | WindowFlags::NoTitleBar | WindowFlags::NoSavedSettings);
}

bool BeginPopupContextWindow(const char* str_id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    if (!str_id)
        str_id = "window_context";
    ImGuiID id = window.get_id(str_id);
    int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsWindowHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
        if (!(popup_flags & ImGuiPopupFlags_NoOpenOverItems) || !IsAnyItemHovered())
            OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, WindowFlags::AlwaysAutoResize | WindowFlags::NoTitleBar | WindowFlags::NoSavedSettings);
}

bool BeginPopupContextVoid(const char* str_id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    if (!str_id)
        str_id = "void_context";
    ImGuiID id = window.get_id(str_id);
    int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && !IsWindowHovered(ImGuiHoveredFlags_AnyWindow))
        if (get_top_most_popup_modal() == NULL)
            OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, WindowFlags::AlwaysAutoResize | WindowFlags::NoTitleBar | WindowFlags::NoSavedSettings);
}

// r_avoid = the rectangle to avoid (e.g. for tooltip it is a rectangle around the mouse cursor which we want to avoid. for popups it's a small point around the cursor.)
// r_outer = the visible area rectangle, minus safe area padding. If our popup size won't fit because of safe area padding we ignore it.
// (r_outer is usually equivalent to the viewport rectangle minus padding, but when multi-viewports are enabled and monitor
//  information are available, it may represent the entire platform monitor from the frame of reference of the current viewport.
//  this allows us to have tooltips/popups displayed out of the parent viewport.)
Vector2D FindBestWindowPosForPopupEx(const Vector2D& ref_pos, const Vector2D& size, ImGuiDir* last_dir, const Rect& r_outer, const Rect& r_avoid, ImGuiPopupPositionPolicy policy)
{
    Vector2D base_pos_clamped = ImClamp(ref_pos, r_outer.min, r_outer.max - size);
    //GetForegroundDrawList()->add_rect(r_avoid.min, r_avoid.max, IM_COL32(255,0,0,255));
    //GetForegroundDrawList()->add_rect(r_outer.min, r_outer.max, IM_COL32(0,255,0,255));

    // Combo Box policy (we want a connecting edge)
    if (policy == ImGuiPopupPositionPolicy_ComboBox)
    {
        const ImGuiDir dir_prefered_order[Dir::COUNT] = { Dir::Down, Dir::Right, Dir::Left, Dir::Up };
        for (int n = (*last_dir != Dir::None) ? -1 : 0; n < Dir::COUNT; n += 1)
        {
            const ImGuiDir dir = (n == -1) ? *last_dir : dir_prefered_order[n];
            if (n != -1 && dir == *last_dir) // Already tried this direction?
                continue;
            Vector2D pos;
            if (dir == Dir::Down)  pos = Vector2D::new(r_avoid.min.x, r_avoid.max.y);          // Below, Toward Right (default)
            if (dir == Dir::Right) pos = Vector2D::new(r_avoid.min.x, r_avoid.min.y - size.y); // Above, Toward Right
            if (dir == Dir::Left)  pos = Vector2D::new(r_avoid.max.x - size.x, r_avoid.max.y); // Below, Toward Left
            if (dir == Dir::Up)    pos = Vector2D::new(r_avoid.max.x - size.x, r_avoid.min.y - size.y); // Above, Toward Left
            if (!r_outer.Contains(Rect(pos, pos + size)))
                continue;
            *last_dir = dir;
            return pos;
        }
    }

    // Tooltip and Default popup policy
    // (Always first try the direction we used on the last frame, if any)
    if (policy == ImGuiPopupPositionPolicy_Tooltip || policy == ImGuiPopupPositionPolicy_Default)
    {
        const ImGuiDir dir_prefered_order[Dir::COUNT] = { Dir::Right, Dir::Down, Dir::Up, Dir::Left };
        for (int n = (*last_dir != Dir::None) ? -1 : 0; n < Dir::COUNT; n += 1)
        {
            const ImGuiDir dir = (n == -1) ? *last_dir : dir_prefered_order[n];
            if (n != -1 && dir == *last_dir) // Already tried this direction?
                continue;

            const float avail_w = (dir == Dir::Left ? r_avoid.min.x : r_outer.max.x) - (dir == Dir::Right ? r_avoid.max.x : r_outer.min.x);
            const float avail_h = (dir == Dir::Up ? r_avoid.min.y : r_outer.max.y) - (dir == Dir::Down ? r_avoid.max.y : r_outer.min.y);

            // If there not enough room on one axis, there's no point in positioning on a side on this axis (e.g. when not enough width, use a top/bottom position to maximize available width)
            if (avail_w < size.x && (dir == Dir::Left || dir == Dir::Right))
                continue;
            if (avail_h < size.y && (dir == Dir::Up || dir == Dir::Down))
                continue;

            Vector2D pos;
            pos.x = (dir == Dir::Left) ? r_avoid.min.x - size.x : (dir == Dir::Right) ? r_avoid.max.x : base_pos_clamped.x;
            pos.y = (dir == Dir::Up) ? r_avoid.min.y - size.y : (dir == Dir::Down) ? r_avoid.max.y : base_pos_clamped.y;

            // Clamp top-left corner of popup
            pos.x = ImMax(pos.x, r_outer.min.x);
            pos.y = ImMax(pos.y, r_outer.min.y);

            *last_dir = dir;
            return pos;
        }
    }

    // Fallback when not enough room:
    *last_dir = Dir::None;

    // For tooltip we prefer avoiding the cursor at all cost even if it means that part of the tooltip won't be visible.
    if (policy == ImGuiPopupPositionPolicy_Tooltip)
        return ref_pos + Vector2D::new(2, 2);

    // Otherwise try to keep within display
    Vector2D pos = ref_pos;
    pos.x = ImMax(ImMin(pos.x + size.x, r_outer.max.x) - size.x, r_outer.min.x);
    pos.y = ImMax(ImMin(pos.y + size.y, r_outer.max.y) - size.y, r_outer.min.y);
    return pos;
}

// Note that this is used for popups, which can overlap the non work-area of individual viewports.
Rect GetPopupAllowedExtentRect(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    Rect r_screen;
    if (window.ViewportAllowPlatformMonitorExtend >= 0)
    {
        // Extent with be in the frame of reference of the given viewport (so min is likely to be negative here)
        const ImGuiPlatformMonitor& monitor = g.platform_io.monitors[window.ViewportAllowPlatformMonitorExtend];
        r_screen.min = monitor.WorkPos;
        r_screen.max = monitor.WorkPos + monitor.work_size;
    }
    else
    {
        // Use the full viewport area (not work area) for popups
        r_screen = window.viewport.get_main_rect();
    }
    Vector2D padding = g.style.DisplaySafeAreaPadding;
    r_screen.Expand(Vector2D::new((r_screen.get_width() > padding.x * 2) ? -padding.x : 0.0, (r_screen.get_height() > padding.y * 2) ? -padding.y : 0.0));
    return r_screen;
}

Vector2D FindBestWindowPosForPopup(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;

    Rect r_outer = GetPopupAllowedExtentRect(window);
    if (window.flags & WindowFlags::ChildMenu)
    {
        // Child menus typically request _any_ position within the parent menu item, and then we move the new menu outside the parent bounds.
        // This is how we end up with child menus appearing (most-commonly) on the right of the parent menu.
        ImGuiWindow* parent_window = window.parent_window;
        float horizontal_overlap = g.style.item_inner_spacing.x; // We want some overlap to convey the relative depth of each menu (currently the amount of overlap is hard-coded to style.ItemSpacing.x).
        Rect r_avoid;
        if (parent_window.dc.MenuBarAppending)
            r_avoid = Rect(-f32::MAX, parent_window.clip_rect.min.y, f32::MAX, parent_window.clip_rect.max.y); // Avoid parent menu-bar. If we wanted multi-line menu-bar, we may instead want to have the calling window setup e.g. a next_window_data.PosConstraintAvoidRect field
        else
            r_avoid = Rect(parent_window.Pos.x + horizontal_overlap, -f32::MAX, parent_window.Pos.x + parent_window.size.x - horizontal_overlap - parent_window.scrollbar_sizes.x, f32::MAX);
        return FindBestWindowPosForPopupEx(window.pos, window.size, &window.AutoPosLastDirection, r_outer, r_avoid, ImGuiPopupPositionPolicy_Default);
    }
    if (window.flags & WindowFlags::Popup)
    {
        return FindBestWindowPosForPopupEx(window.pos, window.size, &window.AutoPosLastDirection, r_outer, Rect(window.pos, window.pos), ImGuiPopupPositionPolicy_Default); // Ideally we'd disable r_avoid here
    }
    if (window.flags & WindowFlags::Tooltip)
    {
        // Position tooltip (always follows mouse)
        float sc = g.style.MouseCursorScale;
        Vector2D ref_pos = NavCalcPreferredRefPos();
        Rect r_avoid;
        if (!g.nav_disable_highlight && g.nav_disable_mouse_hover && !(g.io.config_flags & ImGuiConfigFlags_NavEnableSetMousePos))
            r_avoid = Rect(ref_pos.x - 16, ref_pos.y - 8, ref_pos.x + 16, ref_pos.y + 8);
        else
            r_avoid = Rect(ref_pos.x - 16, ref_pos.y - 8, ref_pos.x + 24 * sc, ref_pos.y + 24 * sc); // FIXME: Hard-coded based on mouse cursor shape expectation. Exact dimension not very important.
        return FindBestWindowPosForPopupEx(ref_pos, window.size, &window.AutoPosLastDirection, r_outer, r_avoid, ImGuiPopupPositionPolicy_Tooltip);
    }
    IM_ASSERT(0);
    return window.pos;
}

//-----------------------------------------------------------------------------
// [SECTION] KEYBOARD/GAMEPAD NAVIGATION
//-----------------------------------------------------------------------------

// FIXME-NAV: The existence of SetNavID vs SetFocusID vs focus_window() needs to be clarified/reworked.
// In our terminology those should be interchangeable, yet right now this is super confusing.
// Those two functions are merely a legacy artifact, so at minimum naming should be clarified.

void SetNavWindow(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    if (g.nav_window != window)
    {
        IMGUI_DEBUG_LOG_FOCUS("[focus] SetNavWindow(\"%s\")\n", window ? window.Name : "<NULL>");
        g.nav_window = window;
    }
    g.NavInitRequest = g.NavMoveSubmitted = g.NavMoveScoringItems = false;
    NavUpdateAnyRequestFlag();
}

void SetNavID(ImGuiID id, ImGuiNavLayer nav_layer, ImGuiID focus_scope_id, const Rect& rect_rel)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.nav_window != NULL);
    IM_ASSERT(nav_layer == NavLayer::Main || nav_layer == NavLayer::Menu);
    g.nav_id = id;
    g.NavLayer = nav_layer;
    g.NavFocusScopeId = focus_scope_id;
    g.nav_window.NavLastIds[nav_layer] = id;
    g.nav_window.NavRectRel[nav_layer] = rect_rel;
}

void SetFocusID(ImGuiID id, ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(id != 0);

    if (g.nav_window != window)
       SetNavWindow(window);

    // Assume that SetFocusID() is called in the context where its window->dc.NavLayerCurrent and window->dc.nav_focus_scope_id_current are valid.
    // Note that window may be != g.current_window (e.g. SetFocusID call in InputTextEx for multi-line text)
    const ImGuiNavLayer nav_layer = window.dcnav_layer_current;
    g.nav_id = id;
    g.NavLayer = nav_layer;
    g.NavFocusScopeId = window.dc.NavFocusScopeIdCurrent;
    window.NavLastIds[nav_layer] = id;
    if (g.last_item_data.id == id)
        window.NavRectRel[nav_layer] = window_rect_abs_to_rel(window, g.last_item_data.NavRect);

    if (g.active_id_source == InputSource::Nav)
        g.nav_disable_mouse_hover = true;
    else
        g.nav_disable_highlight = true;
}

ImGuiDir ImGetDirQuadrantFromDelta(float dx, float dy)
{
    if (f32::abs(dx) > f32::abs(dy))
        return (dx > 0.0) ? Dir::Right : Dir::Left;
    return (dy > 0.0) ? Dir::Down : Dir::Up;
}

static float inline NavScoreItemDistInterval(float a0, float a1, float b0, float b1)
{
    if (a1 < b0)
        return a1 - b0;
    if (b1 < a0)
        return a0 - b1;
    return 0.0;
}

static void inline NavClampRectToVisibleAreaForMoveDir(ImGuiDir move_dir, Rect& r, const Rect& clip_rect)
{
    if (move_dir == Dir::Left || move_dir == Dir::Right)
    {
        r.min.y = ImClamp(r.min.y, clip_rect.min.y, clip_rect.max.y);
        r.max.y = ImClamp(r.max.y, clip_rect.min.y, clip_rect.max.y);
    }
    else // FIXME: PageUp/PageDown are leaving move_dir == None
    {
        r.min.x = ImClamp(r.min.x, clip_rect.min.x, clip_rect.max.x);
        r.max.x = ImClamp(r.max.x, clip_rect.min.x, clip_rect.max.x);
    }
}

// Scoring function for gamepad/keyboard directional navigation. Based on https://gist.github.com/rygorous/6981057
static bool NavScoreItem(ImGuiNavItemData* result)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    if (g.NavLayer != window.dcnav_layer_current)
        return false;

    // FIXME: Those are not good variables names
    Rect cand = g.last_item_data.NavRect;   // Current item nav rectangle
    const Rect curr = g.NavScoringRect;   // Current modified source rect (NB: we've applied max.x = min.x in NavUpdate() to inhibit the effect of having varied item width)
    g.NavScoringDebugCount += 1;

    // When entering through a NavFlattened border, we consider child window items as fully clipped for scoring
    if (window.parent_window == g.nav_window)
    {
        IM_ASSERT((window.flags | g.nav_window.flags) & WindowFlags::NavFlattened);
        if (!window.clip_rect.Overlaps(cand))
            return false;
        cand.ClipWithFull(window.clip_rect); // This allows the scored item to not overlap other candidates in the parent window
    }

    // We perform scoring on items bounding box clipped by the current clipping rectangle on the other axis (clipping on our movement axis would give us equal scores for all clipped items)
    // For example, this ensure that items in one column are not reached when moving vertically from items in another column.
    NavClampRectToVisibleAreaForMoveDir(g.NavMoveClipDir, cand, window.clip_rect);

    // Compute distance between boxes
    // FIXME-NAV: Introducing biases for vertical navigation, needs to be removed.
    float dbx = NavScoreItemDistInterval(cand.min.x, cand.max.x, curr.min.x, curr.max.x);
    float dby = NavScoreItemDistInterval(ImLerp(cand.min.y, cand.max.y, 0.2), ImLerp(cand.min.y, cand.max.y, 0.8), ImLerp(curr.min.y, curr.max.y, 0.2), ImLerp(curr.min.y, curr.max.y, 0.8)); // scale down on Y to keep using box-distance for vertically touching items
    if (dby != 0.0 && dbx != 0.0)
        dbx = (dbx / 1000.0) + ((dbx > 0.0) ? +1.0 : -1.0);
    float dist_box = f32::abs(dbx) + f32::abs(dby);

    // Compute distance between centers (this is off by a factor of 2, but we only compare center distances with each other so it doesn't matter)
    float dcx = (cand.min.x + cand.max.x) - (curr.min.x + curr.max.x);
    float dcy = (cand.min.y + cand.max.y) - (curr.min.y + curr.max.y);
    float dist_center = f32::abs(dcx) + f32::abs(dcy); // L1 metric (need this for our connectedness guarantee)

    // Determine which quadrant of 'curr' our candidate item 'cand' lies in based on distance
    ImGuiDir quadrant;
    float dax = 0.0, day = 0.0, dist_axial = 0.0;
    if (dbx != 0.0 || dby != 0.0)
    {
        // For non-overlapping boxes, use distance between boxes
        dax = dbx;
        day = dby;
        dist_axial = dist_box;
        quadrant = ImGetDirQuadrantFromDelta(dbx, dby);
    }
    else if (dcx != 0.0 || dcy != 0.0)
    {
        // For overlapping boxes with different centers, use distance between centers
        dax = dcx;
        day = dcy;
        dist_axial = dist_center;
        quadrant = ImGetDirQuadrantFromDelta(dcx, dcy);
    }
    else
    {
        // Degenerate case: two overlapping buttons with same center, break ties arbitrarily (note that LastItemId here is really the _previous_ item order, but it doesn't matter)
        quadrant = (g.last_item_data.id < g.nav_id) ? Dir::Left : Dir::Right;
    }

#if IMGUI_DEBUG_NAV_SCORING
    char buf[128];
    if (IsMouseHoveringRect(cand.min, cand.max))
    {
        ImFormatString(buf, IM_ARRAYSIZE(buf), "dbox (%.2,%.2->%.4)\ndcen (%.2,%.2->%.4)\nd (%.2,%.2->%.4)\nnav %c, quadrant %c", dbx, dby, dist_box, dcx, dcy, dist_center, dax, day, dist_axial, "WENS"[g.NavMoveDir], "WENS"[quadrant]);
        ImDrawList* draw_list = get_foreground_draw_list(window);
        draw_list.AddRect(curr.min, curr.max, IM_COL32(255,200,0,100));
        draw_list.AddRect(cand.min, cand.max, IM_COL32(255,255,0,200));
        draw_list.add_rect_filled(cand.max - Vector2D::new(4, 4), cand.max + CalcTextSize(buf) + Vector2D::new(4, 4), IM_COL32(40,0,0,150));
        draw_list.AddText(cand.max, ~0U, buf);
    }
    else if (g.io.key_ctrl) // Hold to preview score in matching quadrant. Press C to rotate.
    {
        if (quadrant == g.NavMoveDir)
        {
            ImFormatString(buf, IM_ARRAYSIZE(buf), "%.0/%.0", dist_box, dist_center);
            ImDrawList* draw_list = get_foreground_draw_list(window);
            draw_list.add_rect_filled(cand.min, cand.max, IM_COL32(255, 0, 0, 200));
            draw_list.AddText(cand.min, IM_COL32(255, 255, 255, 255), buf);
        }
    }


    // Is it in the quadrant we're interesting in moving to?
    bool new_best = false;
    const ImGuiDir move_dir = g.NavMoveDir;
    if (quadrant == move_dir)
    {
        // Does it beat the current best candidate?
        if (dist_box < result.DistBox)
        {
            result.DistBox = dist_box;
            result.DistCenter = dist_center;
            return true;
        }
        if (dist_box == result.DistBox)
        {
            // Try using distance between center points to break ties
            if (dist_center < result.DistCenter)
            {
                result.DistCenter = dist_center;
                new_best = true;
            }
            else if (dist_center == result.DistCenter)
            {
                // Still tied! we need to be extra-careful to make sure everything gets linked properly. We consistently break ties by symbolically moving "later" items
                // (with higher index) to the right/downwards by an infinitesimal amount since we the current "best" button already (so it must have a lower index),
                // this is fairly easy. This rule ensures that all buttons with dx==dy==0 will end up being linked in order of appearance along the x axis.
                if (((move_dir == Dir::Up || move_dir == Dir::Down) ? dby : dbx) < 0.0) // moving bj to the right/down decreases distance
                    new_best = true;
            }
        }
    }

    // Axial check: if 'curr' has no link at all in some direction and 'cand' lies roughly in that direction, add a tentative link. This will only be kept if no "real" matches
    // are found, so it only augments the graph produced by the above method using extra links. (important, since it doesn't guarantee strong connectedness)
    // This is just to avoid buttons having no links in a particular direction when there's a suitable neighbor. you get good graphs without this too.
    // 2017/09/29: FIXME: This now currently only enabled inside menu bars, ideally we'd disable it everywhere. Menus in particular need to catch failure. For general navigation it feels awkward.
    // Disabling it may lead to disconnected graphs when nodes are very spaced out on different axis. Perhaps consider offering this as an option?
    if (result.DistBox == f32::MAX && dist_axial < result.DistAxial)  // Check axial match
        if (g.NavLayer == NavLayer::Menu && !(g.nav_window.flags & WindowFlags::ChildMenu))
            if ((move_dir == Dir::Left && dax < 0.0) || (move_dir == Dir::Right && dax > 0.0) || (move_dir == Dir::Up && day < 0.0) || (move_dir == Dir::Down && day > 0.0))
            {
                result.DistAxial = dist_axial;
                new_best = true;
            }

    return new_best;
}

static void NavApplyItemToResult(ImGuiNavItemData* result)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    result.Window = window;
    result.ID = g.last_item_data.id;
    result.FocusScopeId = window.dc.NavFocusScopeIdCurrent;
    result.InFlags = g.last_item_data.InFlags;
    result.RectRel = window_rect_abs_to_rel(window, g.last_item_data.NavRect);
}

// We get there when either nav_id == id, or when g.nav_any_request is set (which is updated by NavUpdateAnyRequestFlag above)
// This is called after last_item_data is set.
static void NavProcessItem()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    const ImGuiID id = g.last_item_data.id;
    const Rect nav_bb = g.last_item_data.NavRect;
    const ImGuiItemFlags item_flags = g.last_item_data.InFlags;

    // Process Init Request
    if (g.NavInitRequest && g.NavLayer == window.dcnav_layer_current && (item_flags & ItemFlags::Disabled) == 0)
    {
        // Even if 'ImGuiItemFlags_NoNavDefaultFocus' is on (typically collapse/close button) we record the first ResultId so they can be used as a fallback
        const bool candidate_for_nav_default_focus = (item_flags & ImGuiItemFlags_NoNavDefaultFocus) == 0;
        if (candidate_for_nav_default_focus || g.NavInitResultId == 0)
        {
            g.NavInitResultId = id;
            g.NavInitResultRectRel = window_rect_abs_to_rel(window, nav_bb);
        }
        if (candidate_for_nav_default_focus)
        {
            g.NavInitRequest = false; // Found a match, clear request
            NavUpdateAnyRequestFlag();
        }
    }

    // Process Move Request (scoring for navigation)
    // FIXME-NAV: Consider policy for double scoring (scoring from nav_scoring_rect + scoring from a rect wrapped according to current wrapping policy)
    if (g.NavMoveScoringItems)
    {
        const bool is_tab_stop = (item_flags & ImGuiItemFlags_Inputable) && (item_flags & (ImGuiItemFlags_NoTabStop | ItemFlags::Disabled)) == 0;
        const bool is_tabbing = (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing) != 0;
        if (is_tabbing)
        {
            if (is_tab_stop || (g.NavMoveFlags & ImGuiNavMoveFlags_FocusApi))
                NavProcessItemForTabbingRequest(id);
        }
        else if ((g.nav_id != id || (g.NavMoveFlags & ImGuiNavMoveFlags_AllowCurrentNavId)) && !(item_flags & (ItemFlags::Disabled | ImGuiItemFlags_NoNav)))
        {
            ImGuiNavItemData* result = (window == g.nav_window) ? &g.NavMoveResultLocal : &g.NavMoveResultOther;
            if (!is_tabbing)
            {
                if (NavScoreItem(result))
                    NavApplyItemToResult(result);

                // Features like PageUp/PageDown need to maintain a separate score for the visible set of items.
                const float VISIBLE_RATIO = 0.70;
                if ((g.NavMoveFlags & ImGuiNavMoveFlags_AlsoScoreVisibleSet) && window.clip_rect.Overlaps(nav_bb))
                    if (ImClamp(nav_bb.max.y, window.clip_rect.min.y, window.clip_rect.max.y) - ImClamp(nav_bb.min.y, window.clip_rect.min.y, window.clip_rect.max.y) >= (nav_bb.max.y - nav_bb.min.y) * VISIBLE_RATIO)
                        if (NavScoreItem(&g.NavMoveResultLocalVisible))
                            NavApplyItemToResult(&g.NavMoveResultLocalVisible);
            }
        }
    }

    // Update window-relative bounding box of navigated item
    if (g.nav_id == id)
    {
        if (g.nav_window != window)
            SetNavWindow(window); // Always refresh g.nav_window, because some operations such as FocusItem() may not have a window.
        g.NavLayer = window.dcnav_layer_current;
        g.NavFocusScopeId = window.dc.NavFocusScopeIdCurrent;
        g.NavIdIsAlive = true;
        window.NavRectRel[window.dcnav_layer_current] = window_rect_abs_to_rel(window, nav_bb);    // Store item bounding box (relative to window position)
    }
}

// Handle "scoring" of an item for a tabbing/focusing request initiated by NavUpdateCreateTabbingRequest().
// Note that SetKeyboardFocusHere() API calls are considered tabbing requests!
// - Case 1: no nav/active id:    set result to first eligible item, stop storing.
// - Case 2: tab forward:         on ref id set counter, on counter elapse store result
// - Case 3: tab forward wrap:    set result to first eligible item (preemptively), on ref id set counter, on next frame if counter hasn't elapsed store result. // FIXME-TABBING: Could be done as a next-frame forwarded request
// - Case 4: tab backward:        store all results, on ref id pick prev, stop storing
// - Case 5: tab backward wrap:   store all results, on ref id if no result keep storing until last // FIXME-TABBING: Could be done as next-frame forwarded requested
void NavProcessItemForTabbingRequest(ImGuiID id)
{
    ImGuiContext& g = *GImGui;

    // Always store in nav_move_result_local (unlike directional request which uses nav_move_result_other on sibling/flattened windows)
    ImGuiNavItemData* result = &g.NavMoveResultLocal;
    if (g.NavTabbingDir == +1)
    {
        // Tab Forward or SetKeyboardFocusHere() with >= 0
        if (g.NavTabbingResultFirst.id == 0)
            NavApplyItemToResult(&g.NavTabbingResultFirst);
        if (g.NavTabbingCounter -= 1 == 0)
            NavMoveRequestResolveWithLastItem(result);
        else if (g.nav_id == id)
            g.NavTabbingCounter = 1;
    }
    else if (g.NavTabbingDir == -1)
    {
        // Tab Backward
        if (g.nav_id == id)
        {
            if (result.ID)
            {
                g.NavMoveScoringItems = false;
                NavUpdateAnyRequestFlag();
            }
        }
        else
        {
            NavApplyItemToResult(result);
        }
    }
    else if (g.NavTabbingDir == 0)
    {
        // Tab Init
        if (g.NavTabbingResultFirst.id == 0)
            NavMoveRequestResolveWithLastItem(&g.NavTabbingResultFirst);
    }
}

bool NavMoveRequestButNoResultYet()
{
    ImGuiContext& g = *GImGui;
    return g.NavMoveScoringItems && g.NavMoveResultLocal.id == 0 && g.NavMoveResultOther.id == 0;
}

// FIXME: ScoringRect is not set
void NavMoveRequestSubmit(ImGuiDir move_dir, ImGuiDir clip_dir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.nav_window != NULL);

    if (move_flags & ImGuiNavMoveFlags_Tabbing)
        move_flags |= ImGuiNavMoveFlags_AllowCurrentNavId;

    g.NavMoveSubmitted = g.NavMoveScoringItems = true;
    g.NavMoveDir = move_dir;
    g.NavMoveDirForDebug = move_dir;
    g.NavMoveClipDir = clip_dir;
    g.NavMoveFlags = move_flags;
    g.NavMoveScrollFlags = scroll_flags;
    g.NavMoveForwardToNextFrame = false;
    g.NavMoveKeyMods = g.io.key_mods;
    g.NavMoveResultLocal.Clear();
    g.NavMoveResultLocalVisible.Clear();
    g.NavMoveResultOther.Clear();
    g.NavTabbingCounter = 0;
    g.NavTabbingResultFirst.Clear();
    NavUpdateAnyRequestFlag();
}

void NavMoveRequestResolveWithLastItem(ImGuiNavItemData* result)
{
    ImGuiContext& g = *GImGui;
    g.NavMoveScoringItems = false; // Ensure request doesn't need more processing
    NavApplyItemToResult(result);
    NavUpdateAnyRequestFlag();
}

void NavMoveRequestCancel()
{
    ImGuiContext& g = *GImGui;
    g.NavMoveSubmitted = g.NavMoveScoringItems = false;
    NavUpdateAnyRequestFlag();
}

// Forward will reuse the move request again on the next frame (generally with modifications done to it)
void NavMoveRequestForward(ImGuiDir move_dir, ImGuiDir clip_dir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.NavMoveForwardToNextFrame == false);
    NavMoveRequestCancel();
    g.NavMoveForwardToNextFrame = true;
    g.NavMoveDir = move_dir;
    g.NavMoveClipDir = clip_dir;
    g.NavMoveFlags = move_flags | ImGuiNavMoveFlags_Forwarded;
    g.NavMoveScrollFlags = scroll_flags;
}

// Navigation wrap-around logic is delayed to the end of the frame because this operation is only valid after entire
// popup is assembled and in case of appended popups it is not clear which EndPopup() call is final.
void NavMoveRequestTryWrapping(ImGuiWindow* window, ImGuiNavMoveFlags wrap_flags)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(wrap_flags != 0); // Call with _WrapX, _WrapY, _LoopX, _LoopY
    // In theory we should test for NavMoveRequestButNoResultYet() but there's no point doing it, NavEndFrame() will do the same test
    if (g.nav_window == window && g.NavMoveScoringItems && g.NavLayer == NavLayer::Main)
        g.NavMoveFlags |= wrap_flags;
}

// FIXME: This could be replaced by updating a frame number in each window when (window == nav_window) and (nav_layer == 0).
// This way we could find the last focused window among our children. It would be much less confusing this way?
static void NavSaveLastChildNavWindowIntoParent(ImGuiWindow* nav_window)
{
    ImGuiWindow* parent = nav_window;
    while (parent && parent.root_window != parent && (parent.flags & (WindowFlags::Popup | WindowFlags::ChildMenu)) == 0)
        parent = parent.parent_window;
    if (parent && parent != nav_window)
        parent.NavLastChildNavWindow = nav_window;
}

// Restore the last focused child.
// Call when we are expected to land on the Main Layer (0) after focus_window()
static ImGuiWindow* NavRestoreLastChildNavWindow(ImGuiWindow* window)
{
    if (window.NavLastChildNavWindow && window.NavLastChildNavWindow.WasActive)
        return window.NavLastChildNavWindow;
    if (window.DockNodeAsHost && window.DockNodeAsHost.TabBar)
        if (ImGuiTabItem* tab = TabBarFindMostRecentlySelectedTabForActiveWindow(window.DockNodeAsHost.TabBar))
            return tab.Window;
    return window;
}

void NavRestoreLayer(ImGuiNavLayer layer)
{
    ImGuiContext& g = *GImGui;
    if (layer == NavLayer::Main)
    {
        ImGuiWindow* prev_nav_window = g.nav_window;
        g.nav_window = NavRestoreLastChildNavWindow(g.nav_window);    // FIXME-NAV: Should clear ongoing nav requests?
        if (prev_nav_window)
            IMGUI_DEBUG_LOG_FOCUS("[focus] NavRestoreLayer: from \"%s\" to SetNavWindow(\"%s\")\n", prev_nav_window.Name, g.nav_window.Name);
    }
    ImGuiWindow* window = g.nav_window;
    if (window.NavLastIds[layer] != 0)
    {
        SetNavID(window.NavLastIds[layer], layer, 0, window.NavRectRel[layer]);
    }
    else
    {
        g.NavLayer = layer;
        nav_init_window(window, true);
    }
}

void NavRestoreHighlightAfterMove()
{
    ImGuiContext& g = *GImGui;
    g.nav_disable_highlight = false;
    g.nav_disable_mouse_hover = g.NavMousePosDirty = true;
}

static inline void NavUpdateAnyRequestFlag()
{
    ImGuiContext& g = *GImGui;
    g.NavAnyRequest = g.NavMoveScoringItems || g.NavInitRequest || (IMGUI_DEBUG_NAV_SCORING && g.nav_window != NULL);
    if (g.NavAnyRequest)
        IM_ASSERT(g.nav_window != NULL);
}

// This needs to be called before we submit any widget (aka in or before Begin)
void nav_init_window(ImGuiWindow* window, bool force_reinit)
{
    // FIXME: ChildWindow test here is wrong for docking
    ImGuiContext& g = *GImGui;
    IM_ASSERT(window == g.nav_window);

    if (window.flags & WindowFlags::NoNavInputs)
    {
        g.nav_id = g.NavFocusScopeId = 0;
        return;
    }

    bool init_for_nav = false;
    if (window == window.root_window || (window.flags & WindowFlags::Popup) || (window.NavLastIds[0] == 0) || force_reinit)
        init_for_nav = true;
    IMGUI_DEBUG_LOG_NAV("[nav] nav_init_request: from nav_init_window(), init_for_nav=%d, window=\"%s\", layer=%d\n", init_for_nav, window.Name, g.NavLayer);
    if (init_for_nav)
    {
        SetNavID(0, g.NavLayer, 0, Rect());
        g.NavInitRequest = true;
        g.NavInitRequestFromMove = false;
        g.NavInitResultId = 0;
        g.NavInitResultRectRel = Rect();
        NavUpdateAnyRequestFlag();
    }
    else
    {
        g.nav_id = window.NavLastIds[0];
        g.NavFocusScopeId = 0;
    }
}

static Vector2D NavCalcPreferredRefPos()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.nav_window;
    if (g.nav_disable_highlight || !g.nav_disable_mouse_hover || !window)
    {
        // Mouse (we need a fallback in case the mouse becomes invalid after being used)
        // The +1.0 offset when stored by OpenPopupEx() allows reopening this or another popup (same or another mouse button) while not moving the mouse, it is pretty standard.
        // In theory we could move that +1.0 offset in OpenPopupEx()
        Vector2D p = is_mouse_pos_valid(&g.io.mouse_pos) ? g.io.mouse_pos : g.mouse_last_valid_pos;
        return Vector2D::new(p.x + 1.0, p.y);
    }
    else
    {
        // When navigation is active and mouse is disabled, pick a position around the bottom left of the currently navigated item
        // Take account of upcoming scrolling (maybe set mouse pos should be done in EndFrame?)
        Rect rect_rel = WindowRectRelToAbs(window, window.NavRectRel[g.NavLayer]);
        if (window.LastFrameActive != g.frame_count && (window.ScrollTarget.x != f32::MAX || window.ScrollTarget.y != f32::MAX))
        {
            Vector2D next_scroll = CalcNextScrollFromScrollTargetAndClamp(window);
            rect_rel.Translate(window.scroll - next_scroll);
        }
        Vector2D pos = Vector2D::new(rect_rel.min.x + ImMin(g.style.frame_padding.x * 4, rect_rel.get_width()), rect_rel.max.y - ImMin(g.style.frame_padding.y, rect_rel.get_height()));
        ImGuiViewport* viewport = window.viewport;
        return f32::floor(ImClamp(pos, viewport.pos, viewport.pos + viewport.size)); // f32::floor() is important because non-integer mouse position application in backend might be lossy and result in undesirable non-zero delta.
    }
}

const char* GetNavInputName(ImGuiNavInput n)
{
    static const char* names[] =
    {
        "Activate", "Cancel", "Input", "Menu", "DpadLeft", "DpadRight", "DpadUp", "DpadDown", "LStickLeft", "LStickRight", "LStickUp", "LStickDown",
        "FocusPrev", "FocusNext", "TweakSlow", "TweakFast", "KeyLeft", "KeyRight", "KeyUp", "KeyDown"
    };
    IM_ASSERT(IM_ARRAYSIZE(names) == ImGuiNavInput_COUNT);
    IM_ASSERT(n >= 0 && n < ImGuiNavInput_COUNT);
    return names[n];
}

float GetNavInputAmount(ImGuiNavInput n, ImGuiNavReadMode mode)
{
    ImGuiContext& g = *GImGui;
    if (mode == NavReadMode::Down)
        return g.io.NavInputs[n];                         // Instant, read analog input (0.0..1.0, as provided by user)

    const float t = g.io.NavInputsDownDuration[n];
    if (t < 0.0 && mode == NavReadMode::Released)  // Return 1.0 when just released, no repeat, ignore analog input.
        return (g.io.NavInputsDownDurationPrev[n] >= 0.0 ? 1.0 : 0.0);
    if (t < 0.0)
        return 0.0;
    if (mode == NavReadMode::Pressed)               // Return 1.0 when just pressed, no repeat, ignore analog input.
        return (t == 0.0) ? 1.0 : 0.0;
    if (mode == NavReadMode::Repeat)
        return CalcTypematicRepeatAmount(t - g.io.delta_time, t, g.io.KeyRepeatDelay * 0.72, g.io.KeyRepeatRate * 0.80);
    if (mode == NavReadMode::RepeatSlow)
        return CalcTypematicRepeatAmount(t - g.io.delta_time, t, g.io.KeyRepeatDelay * 1.25, g.io.KeyRepeatRate * 2.00);
    if (mode == NavReadMode::RepeatFast)
        return CalcTypematicRepeatAmount(t - g.io.delta_time, t, g.io.KeyRepeatDelay * 0.72, g.io.KeyRepeatRate * 0.30);
    return 0.0;
}

Vector2D get_nav_input_amount_2d(ImGuiNavDirSourceFlags dir_sources, ImGuiNavReadMode mode, float slow_factor, float fast_factor)
{
    Vector2D delta(0.0, 0.0);
    if (dir_sources & NavDirSourceFlags::RawKeyboard)
        delta += Vector2D::new((float)IsKeyDown(ImGuiKey_RightArrow) - IsKeyDown(ImGuiKey_LeftArrow), IsKeyDown(ImGuiKey_DownArrow) - IsKeyDown(ImGuiKey_UpArrow));
    if (dir_sources & NavDirSourceFlags::Keyboard)
        delta += Vector2D::new(GetNavInputAmount(ImGuiNavInput_KeyRight_, mode)   - GetNavInputAmount(ImGuiNavInput_KeyLeft_,   mode), GetNavInputAmount(ImGuiNavInput_KeyDown_,   mode) - GetNavInputAmount(ImGuiNavInput_KeyUp_,   mode));
    if (dir_sources & NavDirSourceFlags::PadDPad)
        delta += Vector2D::new(GetNavInputAmount(ImGuiNavInput_DpadRight, mode)   - GetNavInputAmount(ImGuiNavInput_DpadLeft,   mode), GetNavInputAmount(ImGuiNavInput_DpadDown,   mode) - GetNavInputAmount(ImGuiNavInput_DpadUp,   mode));
    if (dir_sources & NavDirSourceFlags::PadLStick)
        delta += Vector2D::new(GetNavInputAmount(ImGuiNavInput_LStickRight, mode) - GetNavInputAmount(ImGuiNavInput_LStickLeft, mode), GetNavInputAmount(ImGuiNavInput_LStickDown, mode) - GetNavInputAmount(ImGuiNavInput_LStickUp, mode));
    if (slow_factor != 0.0 && IsNavInputDown(ImGuiNavInput_TweakSlow))
        delta *= slow_factor;
    if (fast_factor != 0.0 && IsNavInputDown(ImGuiNavInput_TweakFast))
        delta *= fast_factor;
    return delta;
}

static void nav_update()
{
    ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;

    io.WantSetMousePos = false;
    //if (g.nav_scoring_debug_count > 0) IMGUI_DEBUG_LOG_NAV("[nav] nav_scoring_debug_count %d for '%s' layer %d (Init:%d, Move:%d)\n", g.nav_scoring_debug_count, g.nav_window ? g.nav_window->name : "NULL", g.nav_layer, g.nav_init_request || g.nav_init_result_id != 0, g.NavMoveRequest);

    // Update Gamepad->Nav inputs mapping
    // Set input source as Gamepad when buttons are pressed (as some features differs when used with Gamepad vs Keyboard)
    const bool nav_gamepad_active = (io.config_flags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (io.backend_flags & BackendFlags::HasGamepad) != 0;
    if (nav_gamepad_active && g.io.BackendUsingLegacyNavInputArray == false)
    {
        for (int n = 0; n < ImGuiNavInput_COUNT; n += 1)
            IM_ASSERT(io.NavInputs[n] == 0.0 && "Backend needs to either only use io.add_key_event()/io.add_key_analog_event(), either only fill legacy io.nav_inputs[]. Not both!");
        #define NAV_MAP_KEY(_KEY, _NAV_INPUT, _ACTIVATE_NAV)  do { io.NavInputs[_NAV_INPUT] = io.keys_data[_KEY - Key::KeysDataOffset].analog_value; if (_ACTIVATE_NAV && io.NavInputs[_NAV_INPUT] > 0.0) { g.nav_input_source = InputSource::Gamepad; } } while (0)
        NAV_MAP_KEY(ImGuiKey_GamepadFaceDown, ImGuiNavInput_Activate, true);
        NAV_MAP_KEY(ImGuiKey_GamepadFaceRight, ImGuiNavInput_Cancel, true);
        NAV_MAP_KEY(ImGuiKey_GamepadFaceLeft, ImGuiNavInput_Menu, true);
        NAV_MAP_KEY(ImGuiKey_GamepadFaceUp, ImGuiNavInput_Input, true);
        NAV_MAP_KEY(ImGuiKey_GamepadDpadLeft, ImGuiNavInput_DpadLeft, true);
        NAV_MAP_KEY(ImGuiKey_GamepadDpadRight, ImGuiNavInput_DpadRight, true);
        NAV_MAP_KEY(ImGuiKey_GamepadDpadUp, ImGuiNavInput_DpadUp, true);
        NAV_MAP_KEY(ImGuiKey_GamepadDpadDown, ImGuiNavInput_DpadDown, true);
        NAV_MAP_KEY(ImGuiKey_GamepadL1, ImGuiNavInput_FocusPrev, false);
        NAV_MAP_KEY(ImGuiKey_GamepadR1, ImGuiNavInput_FocusNext, false);
        NAV_MAP_KEY(ImGuiKey_GamepadL1, ImGuiNavInput_TweakSlow, false);
        NAV_MAP_KEY(ImGuiKey_GamepadR1, ImGuiNavInput_TweakFast, false);
        NAV_MAP_KEY(ImGuiKey_GamepadLStickLeft, ImGuiNavInput_LStickLeft, false);
        NAV_MAP_KEY(ImGuiKey_GamepadLStickRight, ImGuiNavInput_LStickRight, false);
        NAV_MAP_KEY(ImGuiKey_GamepadLStickUp, ImGuiNavInput_LStickUp, false);
        NAV_MAP_KEY(ImGuiKey_GamepadLStickDown, ImGuiNavInput_LStickDown, false);
        #undef NAV_MAP_KEY
    }

    // Update Keyboard->Nav inputs mapping
    const bool nav_keyboard_active = (io.config_flags & ConfigFlags::NavEnableKeyboard) != 0;
    if (nav_keyboard_active)
    {
        #define NAV_MAP_KEY(_KEY, _NAV_INPUT)  do { if (IsKeyDown(_KEY)) { io.NavInputs[_NAV_INPUT] = 1.0; g.nav_input_source = InputSource::Keyboard; } } while (0)
        NAV_MAP_KEY(ImGuiKey_Space,     ImGuiNavInput_Activate );
        NAV_MAP_KEY(ImGuiKey_Enter,     ImGuiNavInput_Input    );
        NAV_MAP_KEY(ImGuiKey_Escape,    ImGuiNavInput_Cancel   );
        NAV_MAP_KEY(ImGuiKey_LeftArrow, ImGuiNavInput_KeyLeft_ );
        NAV_MAP_KEY(ImGuiKey_RightArrow,ImGuiNavInput_KeyRight_);
        NAV_MAP_KEY(ImGuiKey_UpArrow,   ImGuiNavInput_KeyUp_   );
        NAV_MAP_KEY(ImGuiKey_DownArrow, ImGuiNavInput_KeyDown_ );
        if (io.key_ctrl)
            io.NavInputs[ImGuiNavInput_TweakSlow] = 1.0;
        if (io.key_shift)
            io.NavInputs[ImGuiNavInput_TweakFast] = 1.0;
        #undef NAV_MAP_KEY
    }
    memcpy(io.NavInputsDownDurationPrev, io.NavInputsDownDuration, sizeof(io.NavInputsDownDuration));
    for (int i = 0; i < IM_ARRAYSIZE(io.NavInputs); i += 1)
        io.NavInputsDownDuration[i] = (io.NavInputs[i] > 0.0) ? (io.NavInputsDownDuration[i] < 0.0 ? 0.0 : io.NavInputsDownDuration[i] + io.delta_time) : -1.0;

    // Process navigation init request (select first/default focus)
    if (g.NavInitResultId != 0)
        NavInitRequestApplyResult();
    g.NavInitRequest = false;
    g.NavInitRequestFromMove = false;
    g.NavInitResultId = 0;
    g.NavJustMovedToId = 0;

    // Process navigation move request
    if (g.NavMoveSubmitted)
        NavMoveRequestApplyResult();
    g.NavTabbingCounter = 0;
    g.NavMoveSubmitted = g.NavMoveScoringItems = false;

    // Schedule mouse position update (will be done at the bottom of this function, after 1) processing all move requests and 2) updating scrolling)
    bool set_mouse_pos = false;
    if (g.NavMousePosDirty && g.NavIdIsAlive)
        if (!g.nav_disable_highlight && g.nav_disable_mouse_hover && g.nav_window)
            set_mouse_pos = true;
    g.NavMousePosDirty = false;
    IM_ASSERT(g.NavLayer == NavLayer::Main || g.NavLayer == NavLayer::Menu);

    // Store our return window (for returning from Menu Layer to Main Layer) and clear it as soon as we step back in our own Layer 0
    if (g.nav_window)
        NavSaveLastChildNavWindowIntoParent(g.nav_window);
    if (g.nav_window && g.nav_window.NavLastChildNavWindow != NULL && g.NavLayer == NavLayer::Main)
        g.nav_window.NavLastChildNavWindow = NULL;

    // Update CTRL+TAB and Windowing features (hold Square to move/resize/etc.)
    NavUpdateWindowing();

    // Set output flags for user application
    io.nav_active = (nav_keyboard_active || nav_gamepad_active) && g.nav_window && !(g.nav_window.flags & WindowFlags::NoNavInputs);
    io.NavVisible = (io.nav_active && g.nav_id != 0 && !g.nav_disable_highlight) || (g.nav_windowing_target != NULL);

    // Process NavCancel input (to close a popup, get back to parent, clear focus)
    NavUpdateCancelRequest();

    // Process manual activation request
    g.nav_activate_id = g.NavActivateDownId = g.NavActivatePressedId = g.NavActivateInputId = 0;
    g.NavActivateFlags = ImGuiActivateFlags_None;
    if (g.nav_id != 0 && !g.nav_disable_highlight && !g.nav_windowing_target && g.nav_window && !(g.nav_window.flags & WindowFlags::NoNavInputs))
    {
        bool activate_down = IsNavInputDown(ImGuiNavInput_Activate);
        bool input_down = IsNavInputDown(ImGuiNavInput_Input);
        bool activate_pressed = activate_down && IsNavInputTest(ImGuiNavInput_Activate, NavReadMode::Pressed);
        bool input_pressed = input_down && IsNavInputTest(ImGuiNavInput_Input, NavReadMode::Pressed);
        if (g.active_id == 0 && activate_pressed)
        {
            g.nav_activate_id = g.nav_id;
            g.NavActivateFlags = ImGuiActivateFlags_PreferTweak;
        }
        if ((g.active_id == 0 || g.active_id == g.nav_id) && input_pressed)
        {
            g.NavActivateInputId = g.nav_id;
            g.NavActivateFlags = ImGuiActivateFlags_PreferInput;
        }
        if ((g.active_id == 0 || g.active_id == g.nav_id) && activate_down)
            g.NavActivateDownId = g.nav_id;
        if ((g.active_id == 0 || g.active_id == g.nav_id) && activate_pressed)
            g.NavActivatePressedId = g.nav_id;
    }
    if (g.nav_window && (g.nav_window.flags & WindowFlags::NoNavInputs))
        g.nav_disable_highlight = true;
    if (g.nav_activate_id != 0)
        IM_ASSERT(g.NavActivateDownId == g.nav_activate_id);

    // Process programmatic activation request
    // FIXME-NAV: Those should eventually be queued (unlike focus they don't cancel each others)
    if (g.NavNextActivateId != 0)
    {
        if (g.NavNextActivateFlags & ImGuiActivateFlags_PreferInput)
            g.NavActivateInputId = g.NavNextActivateId;
        else
            g.nav_activate_id = g.NavActivateDownId = g.NavActivatePressedId = g.NavNextActivateId;
        g.NavActivateFlags = g.NavNextActivateFlags;
    }
    g.NavNextActivateId = 0;

    // Process move requests
    NavUpdateCreateMoveRequest();
    if (g.NavMoveDir == Dir::None)
        NavUpdateCreateTabbingRequest();
    NavUpdateAnyRequestFlag();
    g.NavIdIsAlive = false;

    // Scrolling
    if (g.nav_window && !(g.nav_window.flags & WindowFlags::NoNavInputs) && !g.nav_windowing_target)
    {
        // *Fallback* manual-scroll with Nav directional keys when window has no navigable item
        ImGuiWindow* window = g.nav_window;
        const float scroll_speed = IM_ROUND(window.CalcFontSize() * 100 * io.delta_time); // We need round the scrolling speed because sub-pixel scroll isn't reliably supported.
        const ImGuiDir move_dir = g.NavMoveDir;
        if (window.dc.nav_layers_active_mask == 0x00 && window.dc.nav_has_scroll && move_dir != Dir::None)
        {
            if (move_dir == Dir::Left || move_dir == Dir::Right)
                set_scroll_x(window, f32::floor(window.scroll.x + ((move_dir == Dir::Left) ? -1.0 : +1.0) * scroll_speed));
            if (move_dir == Dir::Up || move_dir == Dir::Down)
                set_scroll_y(window, f32::floor(window.scroll.y + ((move_dir == Dir::Up) ? -1.0 : +1.0) * scroll_speed));
        }

        // *Normal* Manual scroll with NavScrollXXX keys
        // Next movement request will clamp the nav_id reference rectangle to the visible area, so navigation will resume within those bounds.
        Vector2D scroll_dir = get_nav_input_amount_2d(NavDirSourceFlags::PadLStick, NavReadMode::Down, 1.0 / 10.0, 10.0);
        if (scroll_dir.x != 0.0 && window.scrollbar_x)
            set_scroll_x(window, f32::floor(window.scroll.x + scroll_dir.x * scroll_speed));
        if (scroll_dir.y != 0.0)
            set_scroll_y(window, f32::floor(window.scroll.y + scroll_dir.y * scroll_speed));
    }

    // Always prioritize mouse highlight if navigation is disabled
    if (!nav_keyboard_active && !nav_gamepad_active)
    {
        g.nav_disable_highlight = true;
        g.nav_disable_mouse_hover = set_mouse_pos = false;
    }

    // Update mouse position if requested
    // (This will take into account the possibility that a scroll was queued in the window to offset our absolute mouse position before scroll has been applied)
    if (set_mouse_pos && (io.config_flags & ImGuiConfigFlags_NavEnableSetMousePos) && (io.backend_flags & ImGuiBackendFlags_HasSetMousePos))
    {
        io.mouse_pos = io.mouse_pos_prev = NavCalcPreferredRefPos();
        io.WantSetMousePos = true;
        //IMGUI_DEBUG_LOG_IO("SetMousePos: (%.1,%.1)\n", io.mouse_pos.x, io.mouse_pos.y);
    }

    // [DEBUG]
    g.NavScoringDebugCount = 0;
#if IMGUI_DEBUG_NAV_RECTS
    if (g.nav_window)
    {
        ImDrawList* draw_list = get_foreground_draw_list(g.nav_window);
        if (1) { for (int layer = 0; layer < 2; layer += 1) { Rect r = WindowRectRelToAbs(g.nav_window, g.nav_window.NavRectRel[layer]); draw_list.AddRect(r.min, r.max, IM_COL32(255,200,0,255)); } } // [DEBUG]
        if (1) { ImU32 col = (!g.nav_window.Hidden) ? IM_COL32(255,0,255,255) : IM_COL32(255,0,0,255); Vector2D p = NavCalcPreferredRefPos(); char buf[32]; ImFormatString(buf, 32, "%d", g.NavLayer); draw_list.AddCircleFilled(p, 3.0, col); draw_list.AddText(NULL, 13.0, p + Vector2D::new(8,-4), col, buf); }
    }

}

void NavInitRequestApplyResult()
{
    // In very rare cases g.nav_window may be null (e.g. clearing focus after requesting an init request, which does happen when releasing Alt while clicking on void)
    ImGuiContext& g = *GImGui;
    if (!g.nav_window)
        return;

    // Apply result from previous navigation init request (will typically select the first item, unless SetItemDefaultFocus() has been called)
    // FIXME-NAV: On _NavFlattened windows, g.nav_window will only be updated during subsequent frame. Not a problem currently.
    IMGUI_DEBUG_LOG_NAV("[nav] nav_init_request: ApplyResult: NavID 0x%08X in Layer %d window \"%s\"\n", g.NavInitResultId, g.NavLayer, g.nav_window.Name);
    SetNavID(g.NavInitResultId, g.NavLayer, 0, g.NavInitResultRectRel);
    g.NavIdIsAlive = true; // Mark as alive from previous frame as we got a result
    if (g.NavInitRequestFromMove)
        NavRestoreHighlightAfterMove();
}

void NavUpdateCreateMoveRequest()
{
    ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;
    ImGuiWindow* window = g.nav_window;

    if (g.NavMoveForwardToNextFrame && window != NULL)
    {
        // Forwarding previous request (which has been modified, e.g. wrap around menus rewrite the requests with a starting rectangle at the other side of the window)
        // (preserve most state, which were already set by the NavMoveRequestForward() function)
        IM_ASSERT(g.NavMoveDir != Dir::None && g.NavMoveClipDir != Dir::None);
        IM_ASSERT(g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded);
        IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequestForward %d\n", g.NavMoveDir);
    }
    else
    {
        // Initiate directional inputs request
        g.NavMoveDir = Dir::None;
        g.NavMoveFlags = ImGuiNavMoveFlags_None;
        g.NavMoveScrollFlags = ImGuiScrollFlags_None;
        if (window && !g.nav_windowing_target && !(window.flags & WindowFlags::NoNavInputs))
        {
            const ImGuiNavReadMode read_mode = NavReadMode::Repeat;
            if (!IsActiveIdUsingNavDir(Dir::Left)  && (IsNavInputTest(ImGuiNavInput_DpadLeft,  read_mode) || IsNavInputTest(ImGuiNavInput_KeyLeft_,  read_mode))) { g.NavMoveDir = Dir::Left; }
            if (!IsActiveIdUsingNavDir(Dir::Right) && (IsNavInputTest(ImGuiNavInput_DpadRight, read_mode) || IsNavInputTest(ImGuiNavInput_KeyRight_, read_mode))) { g.NavMoveDir = Dir::Right; }
            if (!IsActiveIdUsingNavDir(Dir::Up)    && (IsNavInputTest(ImGuiNavInput_DpadUp,    read_mode) || IsNavInputTest(ImGuiNavInput_KeyUp_,    read_mode))) { g.NavMoveDir = Dir::Up; }
            if (!IsActiveIdUsingNavDir(Dir::Down)  && (IsNavInputTest(ImGuiNavInput_DpadDown,  read_mode) || IsNavInputTest(ImGuiNavInput_KeyDown_,  read_mode))) { g.NavMoveDir = Dir::Down; }
        }
        g.NavMoveClipDir = g.NavMoveDir;
        g.NavScoringNoClipRect = Rect(+f32::MAX, +f32::MAX, -f32::MAX, -f32::MAX);
    }

    // Update PageUp/PageDown/Home/End scroll
    // FIXME-NAV: Consider enabling those keys even without the master ImGuiConfigFlags_NavEnableKeyboard flag?
    const bool nav_keyboard_active = (io.config_flags & ConfigFlags::NavEnableKeyboard) != 0;
    float scoring_rect_offset_y = 0.0;
    if (window && g.NavMoveDir == Dir::None && nav_keyboard_active)
        scoring_rect_offset_y = NavUpdatePageUpPageDown();
    if (scoring_rect_offset_y != 0.0)
    {
        g.NavScoringNoClipRect = window.inner_rect;
        g.NavScoringNoClipRect.TranslateY(scoring_rect_offset_y);
    }

    // [DEBUG] Always send a request
#if IMGUI_DEBUG_NAV_SCORING
    if (io.key_ctrl && IsKeyPressed(ImGuiKey_C))
        g.NavMoveDirForDebug = (ImGuiDir)((g.NavMoveDirForDebug + 1) & 3);
    if (io.key_ctrl && g.NavMoveDir == Dir::None)
    {
        g.NavMoveDir = g.NavMoveDirForDebug;
        g.NavMoveFlags |= ImGuiNavMoveFlags_DebugNoResult;
    }


    // Submit
    g.NavMoveForwardToNextFrame = false;
    if (g.NavMoveDir != Dir::None)
        NavMoveRequestSubmit(g.NavMoveDir, g.NavMoveClipDir, g.NavMoveFlags, g.NavMoveScrollFlags);

    // Moving with no reference triggers a init request (will be used as a fallback if the direction fails to find a match)
    if (g.NavMoveSubmitted && g.nav_id == 0)
    {
        IMGUI_DEBUG_LOG_NAV("[nav] nav_init_request: from move, window \"%s\", layer=%d\n", window ? window.Name : "<NULL>", g.NavLayer);
        g.NavInitRequest = g.NavInitRequestFromMove = true;
        g.NavInitResultId = 0;
        g.nav_disable_highlight = false;
    }

    // When using gamepad, we project the reference nav bounding box into window visible area.
    // This is to allow resuming navigation inside the visible area after doing a large amount of scrolling, since with gamepad every movements are relative
    // (can't focus a visible object like we can with the mouse).
    if (g.NavMoveSubmitted && g.nav_input_source == InputSource::Gamepad && g.NavLayer == NavLayer::Main && window != NULL)// && (g.nav_move_flags & ImGuiNavMoveFlags_Forwarded))
    {
        bool clamp_x = (g.NavMoveFlags & (ImGuiNavMoveFlags_LoopX | ImGuiNavMoveFlags_WrapX)) == 0;
        bool clamp_y = (g.NavMoveFlags & (ImGuiNavMoveFlags_LoopY | ImGuiNavMoveFlags_WrapY)) == 0;
        Rect inner_rect_rel = window_rect_abs_to_rel(window, Rect(window.inner_rect.min - Vector2D::new(1, 1), window.inner_rect.max + Vector2D::new(1, 1)));
        if ((clamp_x || clamp_y) && !inner_rect_rel.Contains(window.NavRectRel[g.NavLayer]))
        {
            //IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequest: clamp nav_rect_rel for gamepad move\n");
            float pad_x = ImMin(inner_rect_rel.get_width(), window.CalcFontSize() * 0.5);
            float pad_y = ImMin(inner_rect_rel.get_height(), window.CalcFontSize() * 0.5); // Terrible approximation for the intent of starting navigation from first fully visible item
            inner_rect_rel.min.x = clamp_x ? (inner_rect_rel.min.x + pad_x) : -f32::MAX;
            inner_rect_rel.max.x = clamp_x ? (inner_rect_rel.max.x - pad_x) : +f32::MAX;
            inner_rect_rel.min.y = clamp_y ? (inner_rect_rel.min.y + pad_y) : -f32::MAX;
            inner_rect_rel.max.y = clamp_y ? (inner_rect_rel.max.y - pad_y) : +f32::MAX;
            window.NavRectRel[g.NavLayer].ClipWithFull(inner_rect_rel);
            g.nav_id = g.NavFocusScopeId = 0;
        }
    }

    // For scoring we use a single segment on the left side our current item bounding box (not touching the edge to avoid box overlap with zero-spaced items)
    Rect scoring_rect;
    if (window != NULL)
    {
        Rect nav_rect_rel = !window.NavRectRel[g.NavLayer].IsInverted() ? window.NavRectRel[g.NavLayer] : Rect(0, 0, 0, 0);
        scoring_rect = WindowRectRelToAbs(window, nav_rect_rel);
        scoring_rect.TranslateY(scoring_rect_offset_y);
        scoring_rect.min.x = ImMin(scoring_rect.min.x + 1.0, scoring_rect.max.x);
        scoring_rect.max.x = scoring_rect.min.x;
        IM_ASSERT(!scoring_rect.IsInverted()); // Ensure if we have a finite, non-inverted bounding box here will allows us to remove extraneous f32::abs() calls in NavScoreItem().
        //GetForegroundDrawList()->add_rect(scoring_rect.min, scoring_rect.max, IM_COL32(255,200,0,255)); // [DEBUG]
        //if (!g.nav_scoring_no_clip_rect.IsInverted()) { GetForegroundDrawList()->add_rect(g.nav_scoring_no_clip_rect.min, g.nav_scoring_no_clip_rect.max, IM_COL32(255, 200, 0, 255)); } // [DEBUG]
    }
    g.NavScoringRect = scoring_rect;
    g.NavScoringNoClipRect.Add(scoring_rect);
}

void NavUpdateCreateTabbingRequest()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.nav_window;
    IM_ASSERT(g.NavMoveDir == Dir::None);
    if (window == NULL || g.nav_windowing_target != NULL || (window.flags & WindowFlags::NoNavInputs))
        return;

    const bool tab_pressed = IsKeyPressed(ImGuiKey_Tab, true) && !IsActiveIdUsingKey(ImGuiKey_Tab) && !g.io.key_ctrl && !g.io.key_alt;
    if (!tab_pressed)
        return;

    // Initiate tabbing request
    // (this is ALWAYS ENABLED, regardless of ImGuiConfigFlags_NavEnableKeyboard flag!)
    // Initially this was designed to use counters and modulo arithmetic, but that could not work with unsubmitted items (list clipper). Instead we use a strategy close to other move requests.
    // See NavProcessItemForTabbingRequest() for a description of the various forward/backward tabbing cases with and without wrapping.
    //// FIXME: We use (g.active_id == 0) but (g.NavDisableHighlight == false) might be righter once we can tab through anything
    g.NavTabbingDir = g.io.key_shift ? -1 : (g.active_id == 0) ? 0 : +1;
    ImGuiScrollFlags scroll_flags = window.Appearing ? ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_AlwaysCenterY : ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_KeepVisibleEdgeY;
    ImGuiDir clip_dir = (g.NavTabbingDir < 0) ? Dir::Up : Dir::Down;
    NavMoveRequestSubmit(Dir::None, clip_dir, ImGuiNavMoveFlags_Tabbing, scroll_flags); // FIXME-NAV: Once we refactor tabbing, add LegacyApi flag to not activate non-inputable.
    g.NavTabbingCounter = -1;
}

// Apply result from previous frame navigation directional move request. Always called from NavUpdate()
void NavMoveRequestApplyResult()
{
    ImGuiContext& g = *GImGui;
#if IMGUI_DEBUG_NAV_SCORING
    if (g.NavMoveFlags & ImGuiNavMoveFlags_DebugNoResult) // [DEBUG] Scoring all items in nav_window at all times
        return;


    // Select which result to use
    ImGuiNavItemData* result = (g.NavMoveResultLocal.id != 0) ? &g.NavMoveResultLocal : (g.NavMoveResultOther.id != 0) ? &g.NavMoveResultOther : NULL;

    // Tabbing forward wrap
    if (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing)
        if ((g.NavTabbingCounter == 1 || g.NavTabbingDir == 0) && g.NavTabbingResultFirst.id)
            result = &g.NavTabbingResultFirst;

    // In a situation when there is no results but nav_id != 0, re-enable the Navigation highlight (because g.nav_id is not considered as a possible result)
    if (result == NULL)
    {
        if (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing)
            g.NavMoveFlags |= ImGuiNavMoveFlags_DontSetNavHighlight;
        if (g.nav_id != 0 && (g.NavMoveFlags & ImGuiNavMoveFlags_DontSetNavHighlight) == 0)
            NavRestoreHighlightAfterMove();
        return;
    }

    // PageUp/PageDown behavior first jumps to the bottom/top mostly visible item, _otherwise_ use the result from the previous/next page.
    if (g.NavMoveFlags & ImGuiNavMoveFlags_AlsoScoreVisibleSet)
        if (g.NavMoveResultLocalVisible.id != 0 && g.NavMoveResultLocalVisible.id != g.nav_id)
            result = &g.NavMoveResultLocalVisible;

    // Maybe entering a flattened child from the outside? In this case solve the tie using the regular scoring rules.
    if (result != &g.NavMoveResultOther && g.NavMoveResultOther.id != 0 && g.NavMoveResultOther.Window.parent_window == g.nav_window)
        if ((g.NavMoveResultOther.DistBox < result.DistBox) || (g.NavMoveResultOther.DistBox == result.DistBox && g.NavMoveResultOther.DistCenter < result.DistCenter))
            result = &g.NavMoveResultOther;
    IM_ASSERT(g.nav_window && result.Window);

    // scroll to keep newly navigated item fully into view.
    if (g.NavLayer == NavLayer::Main)
    {
        if (g.NavMoveFlags & ImGuiNavMoveFlags_ScrollToEdgeY)
        {
            // FIXME: Should remove this
            float scroll_target = (g.NavMoveDir == Dir::Up) ? result.Window->scroll_max.y : 0.0;
            set_scroll_y(result.Window, scroll_target);
        }
        else
        {
            Rect rect_abs = WindowRectRelToAbs(result.Window, result.RectRel);
            ScrollToRectEx(result.Window, rect_abs, g.NavMoveScrollFlags);
        }
    }

    if (g.nav_window != result.Window)
    {
        IMGUI_DEBUG_LOG_FOCUS("[focus] NavMoveRequest: SetNavWindow(\"%s\")\n", result.Window.Name);
        g.nav_window = result.Window;
    }
    if (g.active_id != result.ID)
        clear_active_id();
    if (g.nav_id != result.ID)
    {
        // Don't set nav_just_moved_to_id if just landed on the same spot (which may happen with ImGuiNavMoveFlags_AllowCurrentNavId)
        g.NavJustMovedToId = result.ID;
        g.NavJustMovedToFocusScopeId = result.FocusScopeId;
        g.NavJustMovedToKeyMods = g.NavMoveKeyMods;
    }

    // Focus
    IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequest: result NavID 0x%08X in Layer %d window \"%s\"\n", result.ID, g.NavLayer, g.nav_window.Name);
    SetNavID(result.ID, g.NavLayer, result.FocusScopeId, result.RectRel);

    // Tabbing: Activates Inputable or Focus non-Inputable
    if ((g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing) && (result.InFlags & ImGuiItemFlags_Inputable))
    {
        g.NavNextActivateId = result.ID;
        g.NavNextActivateFlags = ImGuiActivateFlags_PreferInput | ImGuiActivateFlags_TryToPreserveState;
        g.NavMoveFlags |= ImGuiNavMoveFlags_DontSetNavHighlight;
    }

    // Activate
    if (g.NavMoveFlags & ImGuiNavMoveFlags_Activate)
    {
        g.NavNextActivateId = result.ID;
        g.NavNextActivateFlags = ImGuiActivateFlags_None;
    }

    // Enable nav highlight
    if ((g.NavMoveFlags & ImGuiNavMoveFlags_DontSetNavHighlight) == 0)
        NavRestoreHighlightAfterMove();
}

// Process NavCancel input (to close a popup, get back to parent, clear focus)
// FIXME: In order to support e.g. Escape to clear a selection we'll need:
// - either to store the equivalent of active_id_using_key_input_mask for a FocusScope and test for it.
// - either to move most/all of those tests to the epilogue/end functions of the scope they are dealing with (e.g. exit child window in EndChild()) or in EndFrame(), to allow an earlier intercept
static void NavUpdateCancelRequest()
{
    ImGuiContext& g = *GImGui;
    if (!IsNavInputTest(ImGuiNavInput_Cancel, NavReadMode::Pressed))
        return;

    IMGUI_DEBUG_LOG_NAV("[nav] ImGuiNavInput_Cancel\n");
    if (g.active_id != 0)
    {
        if (!IsActiveIdUsingNavInput(ImGuiNavInput_Cancel))
            clear_active_id();
    }
    else if (g.NavLayer != NavLayer::Main)
    {
        // Leave the "menu" layer
        NavRestoreLayer(NavLayer::Main);
        NavRestoreHighlightAfterMove();
    }
    else if (g.nav_window && g.nav_window != g.nav_window.root_window && !(g.nav_window.flags & WindowFlags::Popup) && g.nav_window.parent_window)
    {
        // Exit child window
        ImGuiWindow* child_window = g.nav_window;
        ImGuiWindow* parent_window = g.nav_window.parent_window;
        IM_ASSERT(child_windowchild_id != 0);
        Rect child_rect = child_window.Rect();
        focus_window(parent_window);
        SetNavID(child_windowchild_id, NavLayer::Main, 0, window_rect_abs_to_rel(parent_window, child_rect));
        NavRestoreHighlightAfterMove();
    }
    else if (g.open_popup_stack.size > 0 && !(g.open_popup_stack.back().Window.flags & WindowFlags::Modal))
    {
        // Close open popup/menu
        ClosePopupToLevel(g.open_popup_stack.size - 1, true);
    }
    else
    {
        // clear NavLastId for popups but keep it for regular child window so we can leave one and come back where we were
        if (g.nav_window && ((g.nav_window.flags & WindowFlags::Popup) || !(g.nav_window.flags & WindowFlags::ChildWindow)))
            g.nav_window.NavLastIds[0] = 0;
        g.nav_id = g.NavFocusScopeId = 0;
    }
}

// Handle PageUp/PageDown/Home/End keys
// Called from NavUpdateCreateMoveRequest() which will use our output to create a move request
// FIXME-NAV: This doesn't work properly with NavFlattened siblings as we use nav_window rectangle for reference
// FIXME-NAV: how to get Home/End to aim at the beginning/end of a 2D grid?
static float NavUpdatePageUpPageDown()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.nav_window;
    if ((window.flags & WindowFlags::NoNavInputs) || g.nav_windowing_target != NULL)
        return 0.0;

    const bool page_up_held = IsKeyDown(ImGuiKey_PageUp) && !IsActiveIdUsingKey(ImGuiKey_PageUp);
    const bool page_down_held = IsKeyDown(ImGuiKey_PageDown) && !IsActiveIdUsingKey(ImGuiKey_PageDown);
    const bool home_pressed = IsKeyPressed(ImGuiKey_Home) && !IsActiveIdUsingKey(ImGuiKey_Home);
    const bool end_pressed = IsKeyPressed(ImGuiKey_End) && !IsActiveIdUsingKey(ImGuiKey_End);
    if (page_up_held == page_down_held && home_pressed == end_pressed) // Proceed if either (not both) are pressed, otherwise early out
        return 0.0;

    if (g.NavLayer != NavLayer::Main)
        NavRestoreLayer(NavLayer::Main);

    if (window.dc.nav_layers_active_mask == 0x00 && window.dc.nav_has_scroll)
    {
        // Fallback manual-scroll when window has no navigable item
        if (IsKeyPressed(ImGuiKey_PageUp, true))
            set_scroll_y(window, window.scroll.y - window.inner_rect.get_height());
        else if (IsKeyPressed(ImGuiKey_PageDown, true))
            set_scroll_y(window, window.scroll.y + window.inner_rect.get_height());
        else if (home_pressed)
            set_scroll_y(window, 0.0);
        else if (end_pressed)
            set_scroll_y(window, window.scroll_max.y);
    }
    else
    {
        Rect& nav_rect_rel = window.NavRectRel[g.NavLayer];
        const float page_offset_y = ImMax(0.0, window.inner_rect.get_height() - window.CalcFontSize() * 1.0 + nav_rect_rel.get_height());
        float nav_scoring_rect_offset_y = 0.0;
        if (IsKeyPressed(ImGuiKey_PageUp, true))
        {
            nav_scoring_rect_offset_y = -page_offset_y;
            g.NavMoveDir = Dir::Down; // Because our scoring rect is offset up, we request the down direction (so we can always land on the last item)
            g.NavMoveClipDir = Dir::Up;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_AlsoScoreVisibleSet;
        }
        else if (IsKeyPressed(ImGuiKey_PageDown, true))
        {
            nav_scoring_rect_offset_y = +page_offset_y;
            g.NavMoveDir = Dir::Up; // Because our scoring rect is offset down, we request the up direction (so we can always land on the last item)
            g.NavMoveClipDir = Dir::Down;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_AlsoScoreVisibleSet;
        }
        else if (home_pressed)
        {
            // FIXME-NAV: handling of Home/End is assuming that the top/bottom most item will be visible with scroll.y == 0/scroll_max.y
            // Scrolling will be handled via the ImGuiNavMoveFlags_ScrollToEdgeY flag, we don't scroll immediately to avoid scrolling happening before nav result.
            // Preserve current horizontal position if we have any.
            nav_rect_rel.min.y = nav_rect_rel.max.y = 0.0;
            if (nav_rect_rel.IsInverted())
                nav_rect_rel.min.x = nav_rect_rel.max.x = 0.0;
            g.NavMoveDir = Dir::Down;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_ScrollToEdgeY;
            // FIXME-NAV: MoveClipDir left to _None, intentional?
        }
        else if (end_pressed)
        {
            nav_rect_rel.min.y = nav_rect_rel.max.y = window.ContentSize.y;
            if (nav_rect_rel.IsInverted())
                nav_rect_rel.min.x = nav_rect_rel.max.x = 0.0;
            g.NavMoveDir = Dir::Up;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_ScrollToEdgeY;
            // FIXME-NAV: MoveClipDir left to _None, intentional?
        }
        return nav_scoring_rect_offset_y;
    }
    return 0.0;
}

static void NavEndFrame()
{
    ImGuiContext& g = *GImGui;

    // Show CTRL+TAB list window
    if (g.nav_windowing_target != NULL)
        NavUpdateWindowingOverlay();

    // Perform wrap-around in menus
    // FIXME-NAV: Wrap may need to apply a weight bias on the other axis. e.g. 4x4 grid with 2 last items missing on last item won't handle LoopY/WrapY correctly.
    // FIXME-NAV: Wrap (not Loop) support could be handled by the scoring function and then WrapX would function without an extra frame.
    const ImGuiNavMoveFlags wanted_flags = ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX | ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY;
    if (g.nav_window && NavMoveRequestButNoResultYet() && (g.NavMoveFlags & wanted_flags) && (g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded) == 0)
        NavUpdateCreateWrappingRequest();
}

static void NavUpdateCreateWrappingRequest()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.nav_window;

    bool do_forward = false;
    Rect bb_rel = window.NavRectRel[g.NavLayer];
    ImGuiDir clip_dir = g.NavMoveDir;
    const ImGuiNavMoveFlags move_flags = g.NavMoveFlags;
    if (g.NavMoveDir == Dir::Left && (move_flags & (ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX)))
    {
        bb_rel.min.x = bb_rel.max.x = window.ContentSize.x + window.WindowPadding.x;
        if (move_flags & ImGuiNavMoveFlags_WrapX)
        {
            bb_rel.TranslateY(-bb_rel.get_height()); // Previous row
            clip_dir = Dir::Up;
        }
        do_forward = true;
    }
    if (g.NavMoveDir == Dir::Right && (move_flags & (ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX)))
    {
        bb_rel.min.x = bb_rel.max.x = -window.WindowPadding.x;
        if (move_flags & ImGuiNavMoveFlags_WrapX)
        {
            bb_rel.TranslateY(+bb_rel.get_height()); // Next row
            clip_dir = Dir::Down;
        }
        do_forward = true;
    }
    if (g.NavMoveDir == Dir::Up && (move_flags & (ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY)))
    {
        bb_rel.min.y = bb_rel.max.y = window.ContentSize.y + window.WindowPadding.y;
        if (move_flags & ImGuiNavMoveFlags_WrapY)
        {
            bb_rel.TranslateX(-bb_rel.get_width()); // Previous column
            clip_dir = Dir::Left;
        }
        do_forward = true;
    }
    if (g.NavMoveDir == Dir::Down && (move_flags & (ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY)))
    {
        bb_rel.min.y = bb_rel.max.y = -window.WindowPadding.y;
        if (move_flags & ImGuiNavMoveFlags_WrapY)
        {
            bb_rel.TranslateX(+bb_rel.get_width()); // Next column
            clip_dir = Dir::Right;
        }
        do_forward = true;
    }
    if (!do_forward)
        return;
    window.NavRectRel[g.NavLayer] = bb_rel;
    NavMoveRequestForward(g.NavMoveDir, clip_dir, move_flags, g.NavMoveScrollFlags);
}

static int FindWindowFocusIndex(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    IM_UNUSED(g);
    int order = window.focus_order;
    IM_ASSERT(window.root_window == window); // No child window (not testing _ChildWindow because of docking)
    IM_ASSERT(g.windows_focus_order[order] == window);
    return order;
}

static ImGuiWindow* FindWindowNavFocusable(int i_start, int i_stop, int dir) // FIXME-OPT O(N)
{
    ImGuiContext& g = *GImGui;
    for (int i = i_start; i >= 0 && i < g.windows_focus_order.size && i != i_stop; i += dir)
        if (IsWindowNavFocusable(g.windows_focus_order[i]))
            return g.windows_focus_order[i];
    return NULL;
}

static void NavUpdateWindowingHighlightWindow(int focus_change_dir)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.nav_windowing_target);
    if (g.nav_windowing_target.flags & WindowFlags::Modal)
        return;

    const int i_current = FindWindowFocusIndex(g.nav_windowing_target);
    ImGuiWindow* window_target = FindWindowNavFocusable(i_current + focus_change_dir, -INT_MAX, focus_change_dir);
    if (!window_target)
        window_target = FindWindowNavFocusable((focus_change_dir < 0) ? (g.windows_focus_order.size - 1) : 0, i_current, focus_change_dir);
    if (window_target) // Don't reset windowing target if there's a single window in the list
        g.nav_windowing_target = g.NavWindowingTargetAnim = window_target;
    g.NavWindowingToggleLayer = false;
}

// Windowing management mode
// Keyboard: CTRL+Tab (change focus/move/resize), Alt (toggle menu layer)
// Gamepad:  Hold Menu/Square (change focus/move/resize), Tap Menu/Square (toggle menu layer)
static void NavUpdateWindowing()
{
    ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;

    ImGuiWindow* apply_focus_window = NULL;
    bool apply_toggle_layer = false;

    ImGuiWindow* modal_window = get_top_most_popup_modal();
    bool allow_windowing = (modal_window == NULL);
    if (!allow_windowing)
        g.nav_windowing_target = NULL;

    // Fade out
    if (g.NavWindowingTargetAnim && g.nav_windowing_target == NULL)
    {
        g.nav_windowing_highlight_alpha = ImMax(g.nav_windowing_highlight_alpha - io.delta_time * 10.0, 0.0);
        if (g.dim_bg_ration <= 0.0 && g.nav_windowing_highlight_alpha <= 0.0)
            g.NavWindowingTargetAnim = NULL;
    }

    // Start CTRL+Tab or Square+L/R window selection
    const bool start_windowing_with_gamepad = allow_windowing && !g.nav_windowing_target && IsNavInputTest(ImGuiNavInput_Menu, NavReadMode::Pressed);
    const bool start_windowing_with_keyboard = allow_windowing && !g.nav_windowing_target && io.key_ctrl && IsKeyPressed(ImGuiKey_Tab);
    if (start_windowing_with_gamepad || start_windowing_with_keyboard)
        if (ImGuiWindow* window = g.nav_window ? g.nav_window : FindWindowNavFocusable(g.windows_focus_order.size - 1, -INT_MAX, -1))
        {
            g.nav_windowing_target = g.NavWindowingTargetAnim = window.root_window;
            g.NavWindowingTimer = g.nav_windowing_highlight_alpha = 0.0;
            g.NavWindowingToggleLayer = start_windowing_with_gamepad ? true : false; // Gamepad starts toggling layer
            g.nav_input_source = start_windowing_with_keyboard ? InputSource::Keyboard : InputSource::Gamepad;
        }

    // Gamepad update
    g.NavWindowingTimer += io.delta_time;
    if (g.nav_windowing_target && g.nav_input_source == InputSource::Gamepad)
    {
        // Highlight only appears after a brief time holding the button, so that a fast tap on PadMenu (to toggle nav_layer) doesn't add visual noise
        g.nav_windowing_highlight_alpha = ImMax(g.nav_windowing_highlight_alpha, ImSaturate((g.NavWindowingTimer - NAV_WINDOWING_HIGHLIGHT_DELAY) / 0.05));

        // Select window to focus
        const int focus_change_dir = IsNavInputTest(ImGuiNavInput_FocusPrev, NavReadMode::RepeatSlow) - IsNavInputTest(ImGuiNavInput_FocusNext, NavReadMode::RepeatSlow);
        if (focus_change_dir != 0)
        {
            NavUpdateWindowingHighlightWindow(focus_change_dir);
            g.nav_windowing_highlight_alpha = 1.0;
        }

        // Single press toggles nav_layer, long press with L/R apply actual focus on release (until then the window was merely rendered top-most)
        if (!IsNavInputDown(ImGuiNavInput_Menu))
        {
            g.NavWindowingToggleLayer &= (g.nav_windowing_highlight_alpha < 1.0); // Once button was held long enough we don't consider it a tap-to-toggle-layer press anymore.
            if (g.NavWindowingToggleLayer && g.nav_window)
                apply_toggle_layer = true;
            else if (!g.NavWindowingToggleLayer)
                apply_focus_window = g.nav_windowing_target;
            g.nav_windowing_target = NULL;
        }
    }

    // Keyboard: Focus
    if (g.nav_windowing_target && g.nav_input_source == InputSource::Keyboard)
    {
        // Visuals only appears after a brief time after pressing TAB the first time, so that a fast CTRL+TAB doesn't add visual noise
        g.nav_windowing_highlight_alpha = ImMax(g.nav_windowing_highlight_alpha, ImSaturate((g.NavWindowingTimer - NAV_WINDOWING_HIGHLIGHT_DELAY) / 0.05)); // 1.0
        if (IsKeyPressed(ImGuiKey_Tab, true))
            NavUpdateWindowingHighlightWindow(io.key_shift ? +1 : -1);
        if (!io.key_ctrl)
            apply_focus_window = g.nav_windowing_target;
    }

    // Keyboard: Press and Release ALT to toggle menu layer
    // - Testing that only Alt is tested prevents Alt+Shift or AltGR from toggling menu layer.
    // - AltGR is normally Alt+Ctrl but we can't reliably detect it (not all backends/systems/layout emit it as Alt+Ctrl). But even on keyboards without AltGR we don't want Alt+Ctrl to open menu anyway.
	const bool nav_keyboard_active = (io.config_flags & ConfigFlags::NavEnableKeyboard) != 0;
    if (nav_keyboard_active && IsKeyPressed(Key::ModAlt))
    {
        g.NavWindowingToggleLayer = true;
        g.nav_input_source = InputSource::Keyboard;
    }
    if (g.NavWindowingToggleLayer && g.nav_input_source == InputSource::Keyboard)
    {
        // We cancel toggling nav layer when any text has been typed (generally while holding Alt). (See #370)
        // We cancel toggling nav layer when other modifiers are pressed. (See #4439)
        if (io.input_queue_characters.size > 0 || io.key_ctrl || io.key_shift || io.key_super)
            g.NavWindowingToggleLayer = false;

        // Apply layer toggle on release
        // Important: as before version <18314 we lacked an explicit io event for focus gain/loss, we also compare mouse validity to detect old backends clearing mouse pos on focus loss.
        if (IsKeyReleased(Key::ModAlt) && g.NavWindowingToggleLayer)
            if (g.active_id == 0 || g.ActiveIdAllowOverlap)
                if (is_mouse_pos_valid(&io.mouse_pos) == is_mouse_pos_valid(&io.mouse_pos_prev))
                    apply_toggle_layer = true;
        if (!IsKeyDown(Key::ModAlt))
            g.NavWindowingToggleLayer = false;
    }

    // Move window
    if (g.nav_windowing_target && !(g.nav_windowing_target.flags & WindowFlags::NoMove))
    {
        Vector2D move_delta;
        if (g.nav_input_source == InputSource::Keyboard && !io.key_shift)
            move_delta = get_nav_input_amount_2d(NavDirSourceFlags::RawKeyboard, NavReadMode::Down);
        if (g.nav_input_source == InputSource::Gamepad)
            move_delta = get_nav_input_amount_2d(NavDirSourceFlags::PadLStick, NavReadMode::Down);
        if (move_delta.x != 0.0 || move_delta.y != 0.0)
        {
            const float NAV_MOVE_SPEED = 800.0;
            const float move_speed = f32::floor(NAV_MOVE_SPEED * io.delta_time * ImMin(io.display_frame_buffer_scale.x, io.display_frame_buffer_scale.y)); // FIXME: Doesn't handle variable framerate very well
            ImGuiWindow* moving_window = g.nav_windowing_target.root_window_dock_tree;
            set_window_pos(moving_window, moving_window.Pos + move_delta * move_speed, Cond::Always);
            g.nav_disable_mouse_hover = true;
        }
    }

    // Apply final focus
    if (apply_focus_window && (g.nav_window == NULL || apply_focus_window != g.nav_window.root_window))
    {
        ImGuiViewport* previous_viewport = g.nav_window ? g.nav_window.Viewport : NULL;
        clear_active_id();
        NavRestoreHighlightAfterMove();
        apply_focus_window = NavRestoreLastChildNavWindow(apply_focus_window);
        close_popups_over_window(apply_focus_window, false);
        focus_window(apply_focus_window);
        if (apply_focus_window.NavLastIds[0] == 0)
            nav_init_window(apply_focus_window, false);

        // If the window has ONLY a menu layer (no main layer), select it directly
        // Use nav_layers_active_mask_next since windows didn't have a chance to be Begin()-ed on this frame,
        // so CTRL+Tab where the keys are only held for 1 frame will be able to use correct layers mask since
        // the target window as already been previewed once.
        // FIXME-NAV: This should be done in NavInit.. or in focus_window... However in both of those cases,
        // we won't have a guarantee that windows has been visible before and therefore nav_layers_active_mask*
        // won't be valid.
        if (apply_focus_window.dc.NavLayersActiveMaskNext == (1 << NavLayer::Menu))
            g.NavLayer = NavLayer::Menu;

        // Request OS level focus
        if (apply_focus_window.viewport != previous_viewport && g.platform_io.Platform_SetWindowFocus)
            g.platform_io.Platform_SetWindowFocus(apply_focus_window.viewport);
    }
    if (apply_focus_window)
        g.nav_windowing_target = NULL;

    // Apply menu/layer toggle
    if (apply_toggle_layer && g.nav_window)
    {
        clear_active_id();

        // Move to parent menu if necessary
        ImGuiWindow* new_nav_window = g.nav_window;
        while (new_nav_window.parent_window
            && (new_nav_window.dc.nav_layers_active_mask & (1 << NavLayer::Menu)) == 0
            && (new_nav_window.flags & WindowFlags::ChildWindow) != 0
            && (new_nav_window.flags & (WindowFlags::Popup | WindowFlags::ChildMenu)) == 0)
            new_nav_window = new_nav_window.parent_window;
        if (new_nav_window != g.nav_window)
        {
            ImGuiWindow* old_nav_window = g.nav_window;
            focus_window(new_nav_window);
            new_nav_window.NavLastChildNavWindow = old_nav_window;
        }

        // Toggle layer
        const ImGuiNavLayer new_nav_layer = (g.nav_window.DC.nav_layers_active_mask & (1 << NavLayer::Menu)) ? (ImGuiNavLayer)(g.NavLayer ^ 1) : NavLayer::Main;
        if (new_nav_layer != g.NavLayer)
        {
            // Reinitialize navigation when entering menu bar with the Alt key (FIXME: could be a properly of the layer?)
            const bool preserve_layer_1_nav_id = (new_nav_window.DockNodeAsHost != NULL);
            if (new_nav_layer == NavLayer::Menu && !preserve_layer_1_nav_id)
                g.nav_window.NavLastIds[new_nav_layer] = 0;
            NavRestoreLayer(new_nav_layer);
            NavRestoreHighlightAfterMove();
        }
    }
}

// window has already passed the IsWindowNavFocusable()
static const char* GetFallbackWindowNameForWindowingList(ImGuiWindow* window)
{
    if (window.flags & WindowFlags::Popup)
        return "(Popup)";
    if ((window.flags & WindowFlags::MenuBar) && strcmp(window.Name, "##MainMenuBar") == 0)
        return "(Main menu bar)";
    if (window.DockNodeAsHost)
        return "(Dock node)";
    return "(Untitled)";
}

// Overlay displayed when using CTRL+TAB. Called by EndFrame().
void NavUpdateWindowingOverlay()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.nav_windowing_target != NULL);

    if (g.NavWindowingTimer < NAV_WINDOWING_LIST_APPEAR_DELAY)
        return;

    if (g.nav_windowing_list_window == NULL)
        g.nav_windowing_list_window = FindWindowByName("###NavWindowingList");
    const ImGuiViewport* viewport = /*g.nav_window ? g.nav_window->viewport :*/ GetMainViewport();
    SetNextWindowSizeConstraints(Vector2D::new(viewport.size.x * 0.20, viewport.size.y * 0.20), Vector2D::new(f32::MAX, f32::MAX));
    SetNextWindowPos(viewport.GetCenter(), Cond::Always, Vector2D::new(0.5, 0.5));
    push_style_var(StyleVar::WindowPadding, g.style.WindowPadding * 2.0);
    begin("###NavWindowingList", NULL, WindowFlags::NoTitleBar | WindowFlags::NoFocusOnAppearing | WindowFlags::NoResize | WindowFlags::NoMove | WindowFlags::NoInputs | WindowFlags::AlwaysAutoResize | WindowFlags::NoSavedSettings);
    for (int n = g.windows_focus_order.size - 1; n >= 0; n--)
    {
        ImGuiWindow* window = g.windows_focus_order[n];
        IM_ASSERT(window != NULL); // Fix static analyzers
        if (!IsWindowNavFocusable(window))
            continue;
        const char* label = window.Name;
        if (label == FindRenderedTextEnd(label))
            label = GetFallbackWindowNameForWindowingList(window);
        Selectable(label, g.nav_windowing_target == window);
    }
    end();
    pop_style_var();
}


//-----------------------------------------------------------------------------
// [SECTION] DRAG AND DROP
//-----------------------------------------------------------------------------

bool IsDragDropActive()
{
    ImGuiContext& g = *GImGui;
    return g.drag_drop_active;
}

void clear_drag_drop()
{
    ImGuiContext& g = *GImGui;
    g.drag_drop_active = false;
    g.drag_drop_payload.Clear();
    g.DragDropAcceptFlags = ImGuiDragDropFlags_None;
    g.drag_drop_accept_id_curr = g.drag_drop_accept_id_prev = 0;
    g.drag_drop_accept_id_curr_rect_surface = f32::MAX;
    g.drag_drop_accept_fraame_count = -1;

    g.DragDropPayloadBufHeap.clear();
    memset(&g.DragDropPayloadBufLocal, 0, sizeof(g.DragDropPayloadBufLocal));
}

// When this returns true you need to: a) call SetDragDropPayload() exactly once, b) you may render the payload visual/description, c) call EndDragDropSource()
// If the item has an identifier:
// - This assume/require the item to be activated (typically via ButtonBehavior).
// - Therefore if you want to use this with a mouse button other than left mouse button, it is up to the item itself to activate with another button.
// - We then pull and use the mouse button that was used to activate the item and use it to carry on the drag.
// If the item has no identifier:
// - Currently always assume left mouse button.
bool BeginDragDropSource(ImGuiDragDropFlags flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;

    // FIXME-DRAGDROP: While in the common-most "drag from non-zero active id" case we can tell the mouse button,
    // in both SourceExtern and id==0 cases we may requires something else (explicit flags or some heuristic).
    ImGuiMouseButton mouse_button = ImGuiMouseButton_Left;

    bool source_drag_active = false;
    ImGuiID source_id = 0;
    ImGuiID source_parent_id = 0;
    if (!(flags & DragDropFlags::SourceExtern))
    {
        source_id = g.last_item_data.id;
        if (source_id != 0)
        {
            // Common path: items with id
            if (g.active_id != source_id)
                return false;
            if (g.ActiveIdMouseButton != -1)
                mouse_button = g.ActiveIdMouseButton;
            if (g.io.mouse_down[mouse_button] == false || window.skip_items)
                return false;
            g.ActiveIdAllowOverlap = false;
        }
        else
        {
            // Uncommon path: items without id
            if (g.io.mouse_down[mouse_button] == false || window.skip_items)
                return false;
            if ((g.last_item_data.status_flags & ImGuiItemStatusFlags_HoveredRect) == 0 && (g.active_id == 0 || g.active_id_window != window))
                return false;

            // If you want to use BeginDragDropSource() on an item with no unique identifier for interaction, such as Text() or Image(), you need to:
            // A) Read the explanation below, B) Use the ImGuiDragDropFlags_SourceAllowNullID flag.
            if (!(flags & ImGuiDragDropFlags_SourceAllowNullID))
            {
                IM_ASSERT(0);
                return false;
            }

            // Magic fallback to handle items with no assigned id, e.g. Text(), Image()
            // We build a throwaway id based on current id stack + relative AABB of items in window.
            // THE IDENTIFIER WON'T SURVIVE ANY REPOSITIONING/RESIZINGG OF THE WIDGET, so if your widget moves your dragging operation will be canceled.
            // We don't need to maintain/call clear_active_id() as releasing the button will early out this function and trigger !active_id_is_alive.
            // Rely on keeping other window->LastItemXXX fields intact.
            source_id = g.last_item_data.id = window.GetIDFromRectangle(g.last_item_data.Rect);
            keep_alive_id(source_id);
            bool is_hovered = ItemHoverable(g.last_item_data.Rect, source_id);
            if (is_hovered && g.io.mouse_clicked[mouse_button])
            {
                set_active_id(source_id, window);
                focus_window(window);
            }
            if (g.active_id == source_id) // Allow the underlying widget to display/return hovered during the mouse release frame, else we would get a flicker.
                g.ActiveIdAllowOverlap = is_hovered;
        }
        if (g.active_id != source_id)
            return false;
        source_parent_id = window.IDStack.back();
        source_drag_active = is_mouse_dragging(mouse_button);

        // Disable navigation and key inputs while dragging + cancel existing request if any
        SetActiveIdUsingNavAndKeys();
    }
    else
    {
        window = NULL;
        source_id = ImHashStr("#SourceExtern");
        source_drag_active = true;
    }

    if (source_drag_active)
    {
        if (!g.drag_drop_active)
        {
            IM_ASSERT(source_id != 0);
            clear_drag_drop();
            ImGuiPayload& payload = g.drag_drop_payload;
            payload.source_id = source_id;
            payload.SourceParentId = source_parent_id;
            g.drag_drop_active = true;
            g.drag_drop_source_flags = flags;
            g.DragDropMouseButton = mouse_button;
            if (payload.source_id == g.active_id)
                g.ActiveIdNoClearOnFocusLoss = true;
        }
        g.drag_drop_source_frame_count = g.frame_count;
        g.drag_drop_within_source = true;

        if (!(flags & DragDropFlags::SourceNoPreviewTooltip))
        {
            // Target can request the Source to not display its tooltip (we use a dedicated flag to make this request explicit)
            // We unfortunately can't just modify the source flags and skip the call to BeginTooltip, as caller may be emitting contents.
            BeginTooltip();
            if (g.drag_drop_accept_id_prev && (g.DragDropAcceptFlags & ImGuiDragDropFlags_AcceptNoPreviewTooltip))
            {
                ImGuiWindow* tooltip_window = g.current_window;
                tooltip_window.hidden = tooltip_window.skip_items = true;
                tooltip_window..hidden_frames_can_skip_items = 1;
            }
        }

        if (!(flags & ImGuiDragDropFlags_SourceNoDisableHover) && !(flags & DragDropFlags::SourceExtern))
            g.last_item_data.status_flags &= ~ImGuiItemStatusFlags_HoveredRect;

        return true;
    }
    return false;
}

void EndDragDropSource()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.drag_drop_active);
    IM_ASSERT(g.drag_drop_within_source && "Not after a BeginDragDropSource()?");

    if (!(g.drag_drop_source_flags & DragDropFlags::SourceNoPreviewTooltip))
        EndTooltip();

    // Discard the drag if have not called SetDragDropPayload()
    if (g.drag_drop_payload.dataFrameCount == -1)
        clear_drag_drop();
    g.drag_drop_within_source = false;
}

// Use 'cond' to choose to submit payload on drag start or every frame
bool SetDragDropPayload(const char* type, const void* data, size_t data_size, ImGuiCond cond)
{
    ImGuiContext& g = *GImGui;
    ImGuiPayload& payload = g.drag_drop_payload;
    if (cond == 0)
        cond = Cond::Always;

    IM_ASSERT(type != NULL);
    IM_ASSERT(strlen(type) < IM_ARRAYSIZE(payload.dataType) && "Payload type can be at most 32 characters long");
    IM_ASSERT((data != NULL && data_size > 0) || (data == NULL && data_size == 0));
    IM_ASSERT(cond == Cond::Always || cond == ImGuiCond_Once);
    IM_ASSERT(payload.source_id != 0);                               // Not called between BeginDragDropSource() and EndDragDropSource()

    if (cond == Cond::Always || payload.dataFrameCount == -1)
    {
        // Copy payload
        ImStrncpy(payload.dataType, type, IM_ARRAYSIZE(payload.dataType));
        g.DragDropPayloadBufHeap.resize(0);
        if (data_size > sizeof(g.DragDropPayloadBufLocal))
        {
            // Store in heap
            g.DragDropPayloadBufHeap.resize(data_size);
            payload.data = g.DragDropPayloadBufHeap.data;
            memcpy(payload.data, data, data_size);
        }
        else if (data_size > 0)
        {
            // Store locally
            memset(&g.DragDropPayloadBufLocal, 0, sizeof(g.DragDropPayloadBufLocal));
            payload.data = g.DragDropPayloadBufLocal;
            memcpy(payload.data, data, data_size);
        }
        else
        {
            payload.data = NULL;
        }
        payload.dataSize = data_size;
    }
    payload.dataFrameCount = g.frame_count;

    // Return whether the payload has been accepted
    return (g.drag_drop_accept_fraame_count == g.frame_count) || (g.drag_drop_accept_fraame_count == g.frame_count - 1);
}

bool BeginDragDropTargetCustom(const Rect& bb, ImGuiID id)
{
    ImGuiContext& g = *GImGui;
    if (!g.drag_drop_active)
        return false;

    ImGuiWindow* window = g.current_window;
    ImGuiWindow* hovered_window = g.hovered_window_under_moving_window;
    if (hovered_window == NULL || window.root_window_dock_tree != hovered_window.root_window_dock_tree)
        return false;
    IM_ASSERT(id != 0);
    if (!IsMouseHoveringRect(bb.min, bb.max) || (id == g.drag_drop_payload.source_id))
        return false;
    if (window.skip_items)
        return false;

    IM_ASSERT(g.drag_drop_within_target == false);
    g.DragDropTargetRect = bb;
    g.DragDropTargetId = id;
    g.drag_drop_within_target = true;
    return true;
}

// We don't use BeginDragDropTargetCustom() and duplicate its code because:
// 1) we use LastItemRectHoveredRect which handles items that pushes a temporarily clip rectangle in their code. Calling BeginDragDropTargetCustom(LastItemRect) would not handle them.
// 2) and it's faster. as this code may be very frequently called, we want to early out as fast as we can.
// Also note how the hovered_window test is positioned differently in both functions (in both functions we optimize for the cheapest early out case)
bool BeginDragDropTarget()
{
    ImGuiContext& g = *GImGui;
    if (!g.drag_drop_active)
        return false;

    ImGuiWindow* window = g.current_window;
    if (!(g.last_item_data.status_flags & ImGuiItemStatusFlags_HoveredRect))
        return false;
    ImGuiWindow* hovered_window = g.hovered_window_under_moving_window;
    if (hovered_window == NULL || window.root_window_dock_tree != hovered_window.root_window_dock_tree || window.skip_items)
        return false;

    const Rect& display_rect = (g.last_item_data.status_flags & ImGuiItemStatusFlags_HasDisplayRect) ? g.last_item_data.DisplayRect : g.last_item_data.Rect;
    ImGuiID id = g.last_item_data.id;
    if (id == 0)
    {
        id = window.GetIDFromRectangle(display_rect);
        keep_alive_id(id);
    }
    if (g.drag_drop_payload.source_id == id)
        return false;

    IM_ASSERT(g.drag_drop_within_target == false);
    g.DragDropTargetRect = display_rect;
    g.DragDropTargetId = id;
    g.drag_drop_within_target = true;
    return true;
}

bool is_drag_drop_payload_being_accepted()
{
    ImGuiContext& g = *GImGui;
    return g.drag_drop_active && g.drag_drop_accept_id_prev != 0;
}

const ImGuiPayload* AcceptDragDropPayload(const char* type, ImGuiDragDropFlags flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    ImGuiPayload& payload = g.drag_drop_payload;
    IM_ASSERT(g.drag_drop_active);                        // Not called between BeginDragDropTarget() and EndDragDropTarget() ?
    IM_ASSERT(payload.dataFrameCount != -1);            // Forgot to call EndDragDropTarget() ?
    if (type != NULL && !payload.IsDataType(type))
        return NULL;

    // Accept smallest drag target bounding box, this allows us to nest drag targets conveniently without ordering constraints.
    // NB: We currently accept NULL id as target. However, overlapping targets requires a unique id to function!
    const bool was_accepted_previously = (g.drag_drop_accept_id_prev == g.DragDropTargetId);
    Rect r = g.DragDropTargetRect;
    float r_surface = r.get_width() * r.get_height();
    if (r_surface <= g.drag_drop_accept_id_curr_rect_surface)
    {
        g.DragDropAcceptFlags = flags;
        g.drag_drop_accept_id_curr = g.DragDropTargetId;
        g.drag_drop_accept_id_curr_rect_surface = r_surface;
    }

    // Render default drop visuals
    // FIXME-DRAGDROP: Settle on a proper default visuals for drop target.
    payload.Preview = was_accepted_previously;
    flags |= (g.drag_drop_source_flags & ImGuiDragDropFlags_AcceptNoDrawDefaultRect); // Source can also inhibit the preview (useful for external sources that lives for 1 frame)
    if (!(flags & ImGuiDragDropFlags_AcceptNoDrawDefaultRect) && payload.Preview)
        window.draw_list.AddRect(r.min - Vector2D::new(3.5,3.5), r.max + Vector2D::new(3.5, 3.5), get_color_u32(StyleColor::DragDropTarget), 0.0, 0, 2.0);

    g.drag_drop_accept_fraame_count = g.frame_count;
    payload.Delivery = was_accepted_previously && !IsMouseDown(g.DragDropMouseButton); // For extern drag sources affecting os window focus, it's easier to just test !IsMouseDown() instead of IsMouseReleased()
    if (!payload.Delivery && !(flags & ImGuiDragDropFlags_AcceptBeforeDelivery))
        return NULL;

    return &payload;
}

const ImGuiPayload* GetDragDropPayload()
{
    ImGuiContext& g = *GImGui;
    return g.drag_drop_active ? &g.drag_drop_payload : NULL;
}

// We don't really use/need this now, but added it for the sake of consistency and because we might need it later.
void EndDragDropTarget()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.drag_drop_active);
    IM_ASSERT(g.drag_drop_within_target);
    g.drag_drop_within_target = false;
}

//-----------------------------------------------------------------------------
// [SECTION] LOGGING/CAPTURING
//-----------------------------------------------------------------------------
// All text output from the interface can be captured into tty/file/clipboard.
// By default, tree nodes are automatically opened during logging.
//-----------------------------------------------------------------------------



//-----------------------------------------------------------------------------
// [SECTION] SETTINGS
//-----------------------------------------------------------------------------
// - UpdateSettings() [Internal]
// - MarkIniSettingsDirty() [Internal]
// - CreateNewWindowSettings() [Internal]
// - FindWindowSettings() [Internal]
// - FindOrCreateWindowSettings() [Internal]
// - FindSettingsHandler() [Internal]
// - ClearIniSettings() [Internal]
// - LoadIniSettingsFromDisk()
// - LoadIniSettingsFromMemory()
// - SaveIniSettingsToDisk()
// - SaveIniSettingsToMemory()
// - WindowSettingsHandler_***() [Internal]
//-----------------------------------------------------------------------------

// Called by NewFrame()
void update_settings()
{
    // Load settings on first frame (if not explicitly loaded manually before)
    ImGuiContext& g = *GImGui;
    if (!g.settings_loaded)
    {
        IM_ASSERT(g.settings_windows.empty());
        if (g.io.ini_file_name)
            LoadIniSettingsFromDisk(g.io.ini_file_name);
        g.settings_loaded = true;
    }

    // Save settings (with a delay after the last modification, so we don't spam disk too much)
    if (g.SettingsDirtyTimer > 0.0)
    {
        g.SettingsDirtyTimer -= g.io.delta_time;
        if (g.SettingsDirtyTimer <= 0.0)
        {
            if (g.io.ini_file_name != NULL)
                save_ini_settings_to_disk(g.io.ini_file_name);
            else
                g.io.WantSaveIniSettings = true;  // Let user know they can call SaveIniSettingsToMemory(). user will need to clear io.want_save_ini_settings themselves.
            g.SettingsDirtyTimer = 0.0;
        }
    }
}

void MarkIniSettingsDirty()
{
    ImGuiContext& g = *GImGui;
    if (g.SettingsDirtyTimer <= 0.0)
        g.SettingsDirtyTimer = g.io.IniSavingRate;
}

void MarkIniSettingsDirty(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    if (!(window.flags & WindowFlags::NoSavedSettings))
        if (g.SettingsDirtyTimer <= 0.0)
            g.SettingsDirtyTimer = g.io.IniSavingRate;
}

ImGuiWindowSettings* CreateNewWindowSettings(const char* name)
{
    ImGuiContext& g = *GImGui;

#if !IMGUI_DEBUG_INI_SETTINGS
    // Skip to the "###" marker if any. We don't skip past to match the behavior of GetID()
    // Preserve the full string when IMGUI_DEBUG_INI_SETTINGS is set to make .ini inspection easier.
    if (const char* p = strstr(name, "###"))
        name = p;

    const size_t name_len = strlen(name);

    // Allocate chunk
    const size_t chunk_size = sizeof(ImGuiWindowSettings) + name_len + 1;
    ImGuiWindowSettings* settings = g.settings_windows.alloc_chunk(chunk_size);
    IM_PLACEMENT_NEW(settings) ImGuiWindowSettings();
    settings.ID = ImHashStr(name, name_len);
    memcpy(settings.GetName(), name, name_len + 1);   // Store with zero terminator

    return settings;
}

ImGuiWindowSettings* FindWindowSettings(ImGuiID id)
{
    ImGuiContext& g = *GImGui;
    for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
        if (settings.ID == id)
            return settings;
    return NULL;
}

ImGuiWindowSettings* FindOrCreateWindowSettings(const char* name)
{
    if (ImGuiWindowSettings* settings = FindWindowSettings(ImHashStr(name)))
        return settings;
    return CreateNewWindowSettings(name);
}

void add_settings_handler(const ImGuiSettingsHandler* handler)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(FindSettingsHandler(handler.TypeName) == NULL);
    g.settings_handlers.push_back(*handler);
}

void RemoveSettingsHandler(const char* type_name)
{
    ImGuiContext& g = *GImGui;
    if (ImGuiSettingsHandler* handler = FindSettingsHandler(type_name))
        g.settings_handlers.erase(handler);
}

ImGuiSettingsHandler* FindSettingsHandler(const char* type_name)
{
    ImGuiContext& g = *GImGui;
    const ImGuiID type_hash = ImHashStr(type_name);
    for (int handler_n = 0; handler_n < g.settings_handlers.size; handler_n += 1)
        if (g.settings_handlers[handler_n].type_hash == type_hash)
            return &g.settings_handlers[handler_n];
    return NULL;
}

void ClearIniSettings()
{
    ImGuiContext& g = *GImGui;
    g.SettingsIniData.clear();
    for (int handler_n = 0; handler_n < g.settings_handlers.size; handler_n += 1)
        if (g.settings_handlers[handler_n].clear_all_fn)
            g.settings_handlers[handler_n].clear_all_fn(&g, &g.settings_handlers[handler_n]);
}

void LoadIniSettingsFromDisk(const char* ini_filename)
{
    size_t file_data_size = 0;
    char* file_data = (char*)ImFileLoadToMemory(ini_filename, "rb", &file_data_size);
    if (!file_data)
        return;
    if (file_data_size > 0)
        LoadIniSettingsFromMemory(file_data, file_data_size);
    IM_FREE(file_data);
}

// Zero-tolerance, no error reporting, cheap .ini parsing
void LoadIniSettingsFromMemory(const char* ini_data, size_t ini_size)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.initialized);
    //IM_ASSERT(!g.within_frame_scope && "Cannot be called between NewFrame() and EndFrame()");
    //IM_ASSERT(g.settings_loaded == false && g.frame_count == 0);

    // For user convenience, we allow passing a non zero-terminated string (hence the ini_size parameter).
    // For our convenience and to make the code simpler, we'll also write zero-terminators within the buffer. So let's create a writable copy..
    if (ini_size == 0)
        ini_size = strlen(ini_data);
    g.SettingsIniData.Buf.resize(ini_size + 1);
    char* const buf = g.SettingsIniData.Buf.data;
    char* const buf_end = buf + ini_size;
    memcpy(buf, ini_data, ini_size);
    buf_end[0] = 0;

    // Call pre-read handlers
    // Some types will clear their data (e.g. dock information) some types will allow merge/override (window)
    for (int handler_n = 0; handler_n < g.settings_handlers.size; handler_n += 1)
        if (g.settings_handlers[handler_n].ReadInitFn)
            g.settings_handlers[handler_n].ReadInitFn(&g, &g.settings_handlers[handler_n]);

    void* entry_data = NULL;
    ImGuiSettingsHandler* entry_handler = NULL;

    char* line_end = NULL;
    for (char* line = buf; line < buf_end; line = line_end + 1)
    {
        // Skip new lines markers, then find end of the line
        while (*line == '\n' || *line == '\r')
            line += 1;
        line_end = line;
        while (line_end < buf_end && *line_end != '\n' && *line_end != '\r')
            line_end += 1;
        line_end[0] = 0;
        if (line[0] == ';')
            continue;
        if (line[0] == '[' && line_end > line && line_end[-1] == ']')
        {
            // Parse "[Type][name]". Note that 'name' can itself contains [] characters, which is acceptable with the current format and parsing code.
            line_end[-1] = 0;
            const char* name_end = line_end - 1;
            const char* type_start = line + 1;
            char* type_end = (char*)(void*)ImStrchrRange(type_start, name_end, ']');
            const char* name_start = type_end ? ImStrchrRange(type_end + 1, name_end, '[') : NULL;
            if (!type_end || !name_start)
                continue;
            *type_end = 0; // Overwrite first ']'
            name_start += 1;  // Skip second '['
            entry_handler = FindSettingsHandler(type_start);
            entry_data = entry_handler ? entry_handler.read_open_fn(&g, entry_handler, name_start) : NULL;
        }
        else if (entry_handler != NULL && entry_data != NULL)
        {
            // Let type handler parse the line
            entry_handler.read_line_fn(&g, entry_handler, entry_data, line);
        }
    }
    g.settings_loaded = true;

    // [DEBUG] Restore untouched copy so it can be browsed in Metrics (not strictly necessary)
    memcpy(buf, ini_data, ini_size);

    // Call post-read handlers
    for (int handler_n = 0; handler_n < g.settings_handlers.size; handler_n += 1)
        if (g.settings_handlers[handler_n].apply_all_fn)
            g.settings_handlers[handler_n].apply_all_fn(&g, &g.settings_handlers[handler_n]);
}

void save_ini_settings_to_disk(const char* ini_filename)
{
    ImGuiContext& g = *GImGui;
    g.SettingsDirtyTimer = 0.0;
    if (!ini_filename)
        return;

    size_t ini_data_size = 0;
    const char* ini_data = SaveIniSettingsToMemory(&ini_data_size);
    ImFileHandle f = ImFileOpen(ini_filename, "wt");
    if (!f)
        return;
    ImFileWrite(ini_data, sizeof(char), ini_data_size, f);
    ImFileClose(f);
}

// Call registered handlers (e.g. SettingsHandlerWindow_WriteAll() + custom handlers) to write their stuff into a text buffer
const char* SaveIniSettingsToMemory(size_t* out_size)
{
    ImGuiContext& g = *GImGui;
    g.SettingsDirtyTimer = 0.0;
    g.SettingsIniData.Buf.resize(0);
    g.SettingsIniData.Buf.push_back(0);
    for (int handler_n = 0; handler_n < g.settings_handlers.size; handler_n += 1)
    {
        ImGuiSettingsHandler* handler = &g.settings_handlers[handler_n];
        handler.write_all_fn(&g, handler, &g.SettingsIniData);
    }
    if (out_size)
        *out_size = g.SettingsIniData.size();
    return g.SettingsIniData.c_str();
}

static void WindowSettingsHandler_ClearAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
{
    ImGuiContext& g = *ctx;
    for (int i = 0; i != g.windows.size; i += 1)
        g.windows[i].SettingsOffset = -1;
    g.settings_windows.clear();
}

static void* WindowSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, const char* name)
{
    ImGuiWindowSettings* settings = FindOrCreateWindowSettings(name);
    ImGuiID id = settings.ID;
    *settings = ImGuiWindowSettings(); // clear existing if recycling previous entry
    settings.ID = id;
    settings.WantApply = true;
    return (void*)settings;
}

static void WindowSettingsHandler_ReadLine(ImGuiContext*, ImGuiSettingsHandler*, void* entry, const char* line)
{
    ImGuiWindowSettings* settings = (ImGuiWindowSettings*)entry;
    int x, y;
    int i;
    ImU32 u1;
    if (sscanf(line, "pos=%i,%i", &x, &y) == 2)             { settings.pos = Vector2Dih(x, y); }
    else if (sscanf(line, "size=%i,%i", &x, &y) == 2)       { settings.size = Vector2Dih(x, y); }
    else if (sscanf(line, "viewport_id=0x%08X", &u1) == 1)   { settings.viewport_id = u1; }
    else if (sscanf(line, "viewport_pos=%i,%i", &x, &y) == 2){ settings.viewport_pos = Vector2Dih(x, y); }
    else if (sscanf(line, "collapsed=%d", &i) == 1)         { settings.collapsed = (i != 0); }
    else if (sscanf(line, "dock_id=0x%x,%d", &u1, &i) == 2)  { settings.dock_id = u1; settings.dock_order = i; }
    else if (sscanf(line, "dock_id=0x%x", &u1) == 1)         { settings.dock_id = u1; settings.dock_order = -1; }
    else if (sscanf(line, "class_id=0x%x", &u1) == 1)        { settings.ClassId = u1; }
}

// Apply to existing windows (if any)
static void WindowSettingsHandler_ApplyAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
{
    ImGuiContext& g = *ctx;
    for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
        if (settings.WantApply)
        {
            if (ImGuiWindow* window = FindWindowByID(settings.ID))
                apply_window_settings(window, settings);
            settings.WantApply = false;
        }
}

static void WindowSettingsHandler_WriteAll(ImGuiContext* ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* buf)
{
    // Gather data from windows that were active during this session
    // (if a window wasn't opened in this session we preserve its settings)
    ImGuiContext& g = *ctx;
    for (int i = 0; i != g.windows.size; i += 1)
    {
        ImGuiWindow* window = g.windows[i];
        if (window.flags & WindowFlags::NoSavedSettings)
            continue;

        ImGuiWindowSettings* settings = (window.settings_offset != -1) ? g.settings_windows.ptr_from_offset(window.settings_offset) : FindWindowSettings(window.id);
        if (!settings)
        {
            settings = CreateNewWindowSettings(window.Name);
            window.settings_offset = g.settings_windows.offset_from_ptr(settings);
        }
        IM_ASSERT(settings.ID == window.id);
        settings.pos = Vector2Dih(window.pos - window.viewport_pos);
        settings.size = Vector2Dih(window.size_full);
        settings.viewport_id = window.viewport_id;
        settings.viewport_pos = Vector2Dih(window.viewport_pos);
        IM_ASSERT(window.dock_node == NULL || window.dock_node.ID == window.DockId);
        settings.dock_id = window.DockId;
        settings.ClassId = window.WindowClass.ClassId;
        settings.dock_order = window.DockOrder;
        settings.collapsed = window.collapsed;
    }

    // Write to text buffer
    buf.reserve(buf->size() + g.settings_windows.size() * 6); // ballpark reserve
    for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
    {
        const char* settings_name = settings.GetName();
        buf.appendf("[%s][%s]\n", handler.TypeName, settings_name);
        if (settings.viewport_id != 0 && settings.viewport_id != IMGUI_VIEWPORT_DEFAULT_ID)
        {
            buf.appendf("viewport_pos=%d,%d\n", settings.viewport_pos.x, settings.viewport_pos.y);
            buf.appendf("viewport_id=0x%08X\n", settings.viewport_id);
        }
        if (settings.pos.x != 0 || settings.pos.y != 0 || settings.viewport_id == IMGUI_VIEWPORT_DEFAULT_ID)
            buf.appendf("pos=%d,%d\n", settings.pos.x, settings.pos.y);
        if (settings.size.x != 0 || settings.size.y != 0)
            buf.appendf("size=%d,%d\n", settings.size.x, settings.size.y);
        buf.appendf("collapsed=%d\n", settings.collapsed);
        if (settings.dock_id != 0)
        {
            //buf->appendf("tab_id=0x%08X\n", ImHashStr("#TAB", 4, settings->id)); // window->tab_id: this is not read back but writing it makes "debugging" the .ini data easier.
            if (settings.dock_order == -1)
                buf.appendf("dock_id=0x%08X\n", settings.dock_id);
            else
                buf.appendf("dock_id=0x%08X,%d\n", settings.dock_id, settings.dock_order);
            if (settings.ClassId != 0)
                buf.appendf("class_id=0x%08X\n", settings.ClassId);
        }
        buf.append("\n");
    }
}


//-----------------------------------------------------------------------------
// [SECTION] VIEWPORTS, PLATFORM WINDOWS
//-----------------------------------------------------------------------------
// - GetMainViewport()
// - FindViewportByID()
// - FindViewportByPlatformHandle()
// - SetCurrentViewport() [Internal]
// - SetWindowViewport() [Internal]
// - GetWindowAlwaysWantOwnViewport() [Internal]
// - update_try_merge_window_into_host_viewport() [Internal]
// - UpdateTryMergeWindowIntoHostViewports() [Internal]
// - TranslateWindowsInViewport() [Internal]
// - ScaleWindowsInViewport() [Internal]
// - FindHoveredViewportFromPlatformWindowStack() [Internal]
// - UpdateViewportsNewFrame() [Internal]
// - UpdateViewportsEndFrame() [Internal]
// - AddUpdateViewport() [Internal]
// - WindowSelectViewport() [Internal]
// - WindowSyncOwnedViewport() [Internal]
// - UpdatePlatformWindows()
// - RenderPlatformWindowsDefault()
// - FindPlatformMonitorForPos() [Internal]
// - FindPlatformMonitorForRect() [Internal]
// - UpdateViewportPlatformMonitor() [Internal]
// - DestroyPlatformWindow() [Internal]
// - DestroyPlatformWindows()
//-----------------------------------------------------------------------------

ImGuiViewport* GetMainViewport()
{
    ImGuiContext& g = *GImGui;
    return g.viewports[0];
}

// FIXME: This leaks access to viewports not listed in platform_io.viewports[]. Problematic? (#4236)
ImGuiViewport* FindViewportByID(ImGuiID id)
{
    ImGuiContext& g = *GImGui;
    for (int n = 0; n < g.viewports.size; n += 1)
        if (g.viewports[n].ID == id)
            return g.viewports[n];
    return NULL;
}

ImGuiViewport* FindViewportByPlatformHandle(void* platform_handle)
{
    ImGuiContext& g = *GImGui;
    for (int i = 0; i != g.viewports.size; i += 1)
        if (g.viewports[i].PlatformHandle == platform_handle)
            return g.viewports[i];
    return NULL;
}

void SetCurrentViewport(ImGuiWindow* current_window, ImGuiViewportP* viewport)
{
    ImGuiContext& g = *GImGui;
    (void)current_window;

    if (viewport)
        viewport.LastFrameActive = g.frame_count;
    if (g.current_viewport == viewport)
        return;
    g.CurrentDpiScale = viewport ? viewport.DpiScale : 1.0;
    g.current_viewport = viewport;
    //IMGUI_DEBUG_LOG_VIEWPORT("[viewport] SetCurrentViewport changed '%s' 0x%08X\n", current_window ? current_window->name : NULL, viewport ? viewport->id : 0);

    // Notify platform layer of viewport changes
    // FIXME-DPI: This is only currently used for experimenting with handling of multiple DPI
    if (g.current_viewport && g.platform_io.Platform_OnChangedViewport)
        g.platform_io.Platform_OnChangedViewport(g.current_viewport);
}

void SetWindowViewport(ImGuiWindow* window, ImGuiViewportP* viewport)
{
    // Abandon viewport
    if (window.viewport_owned && window.viewport.Window == window)
        window.viewport.size = Vector2D::new(0.0, 0.0);

    window.viewport = viewport;
    window.viewport_id = viewport.ID;
    window.viewport_owned = (viewport.Window == window);
}

static bool GetWindowAlwaysWantOwnViewport(ImGuiWindow* window)
{
    // Tooltips and menus are not automatically forced into their own viewport when the NoMerge flag is set, however the multiplication of viewports makes them more likely to protrude and create their own.
    ImGuiContext& g = *GImGui;
    if (g.io.ConfigViewportsNoAutoMerge || (window.WindowClass.ViewportFlagsOverrideSet & ImGuiViewportFlags_NoAutoMerge))
        if (g.config_flags_curr_frame & ConfigFlags::ViewportsEnable)
            if (!window.dock_is_active)
                if ((window.flags & (WindowFlags::ChildWindow | WindowFlags::ChildMenu | WindowFlags::Tooltip)) == 0)
                    if ((window.flags & WindowFlags::Popup) == 0 || (window.flags & WindowFlags::Modal) != 0)
                        return true;
    return false;
}

static bool update_try_merge_window_into_host_viewport(ImGuiWindow* window, ImGuiViewportP* viewport)
{
    ImGuiContext& g = *GImGui;
    if (window.viewport == viewport)
        return false;
    if ((viewport.flags & ImGuiViewportFlags_CanHostOtherWindows) == 0)
        return false;
    if ((viewport.flags & ImGuiViewportFlags_Minimized) != 0)
        return false;
    if (!viewport.get_main_rect().Contains(window.Rect()))
        return false;
    if (GetWindowAlwaysWantOwnViewport(window))
        return false;

    // FIXME: Can't use g.windows_focus_order[] for root windows only as we care about Z order. If we maintained a DisplayOrder along with focus_order we could..
    for (int n = 0; n < g.windows.size; n += 1)
    {
        ImGuiWindow* window_behind = g.windows[n];
        if (window_behind == window)
            break;
        if (window_behind.WasActive && window_behind.ViewportOwned && !(window_behind.flags & WindowFlags::ChildWindow))
            if (window_behind.Viewport.get_main_rect().Overlaps(window.Rect()))
                return false;
    }

    // Move to the existing viewport, Move child/hosted windows as well (FIXME-OPT: iterate child)
    ImGuiViewportP* old_viewport = window.viewport;
    if (window.viewport_owned)
        for (int n = 0; n < g.windows.size; n += 1)
            if (g.windows[n].Viewport == old_viewport)
                SetWindowViewport(g.windows[n], viewport);
    SetWindowViewport(window, viewport);
    BringWindowToDisplayFront(window);

    return true;
}

// FIXME: handle 0 to N host viewports
static bool UpdateTryMergeWindowIntoHostViewports(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    return update_try_merge_window_into_host_viewport(window, g.viewports[0]);
}

// Translate Dear ImGui windows when a Host viewport has been moved
// (This additionally keeps windows at the same place when ConfigFlags::ViewportsEnable is toggled!)
void TranslateWindowsInViewport(ImGuiViewportP* viewport, const Vector2D& old_pos, const Vector2D& new_pos)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(viewport.Window == NULL && (viewport.flags & ImGuiViewportFlags_CanHostOtherWindows));

    // 1) We test if ConfigFlags::ViewportsEnable was just toggled, which allows us to conveniently
    // translate imgui windows from OS-window-local to absolute coordinates or vice-versa.
    // 2) If it's not going to fit into the new size, keep it at same absolute position.
    // One problem with this is that most Win32 applications doesn't update their render while dragging,
    // and so the window will appear to teleport when releasing the mouse.
    const bool translate_all_windows = (g.config_flags_curr_frame & ConfigFlags::ViewportsEnable) != (g.config_flags_last_frame & ConfigFlags::ViewportsEnable);
    Rect test_still_fit_rect(old_pos, old_pos + viewport.size);
    Vector2D delta_pos = new_pos - old_pos;
    for (int window_n = 0; window_n < g.windows.size; window_n += 1) // FIXME-OPT
        if (translate_all_windows || (g.windows[window_n].Viewport == viewport && test_still_fit_rect.Contains(g.windows[window_n].Rect())))
            TranslateWindow(g.windows[window_n], delta_pos);
}

// scale all windows (position, size). Use when e.g. changing DPI. (This is a lossy operation!)
void ScaleWindowsInViewport(ImGuiViewportP* viewport, float scale)
{
    ImGuiContext& g = *GImGui;
    if (viewport.Window)
    {
        ScaleWindow(viewport.Window, scale);
    }
    else
    {
        for (int i = 0; i != g.windows.size; i += 1)
            if (g.windows[i].Viewport == viewport)
                ScaleWindow(g.windows[i], scale);
    }
}

// If the backend doesn't set mouse_last_hovered_viewport or doesn't honor ViewportFlags::NoInputs, we do a search ourselves.
// A) It won't take account of the possibility that non-imgui windows may be in-between our dragged window and our target window.
// B) It requires Platform_GetWindowFocus to be implemented by backend.
ImGuiViewportP* FindHoveredViewportFromPlatformWindowStack(const Vector2D& mouse_platform_pos)
{
    ImGuiContext& g = *GImGui;
    ImGuiViewportP* best_candidate = NULL;
    for (int n = 0; n < g.viewports.size; n += 1)
    {
        ImGuiViewportP* viewport = g.viewports[n];
        if (!(viewport.flags & (ViewportFlags::NoInputs | ImGuiViewportFlags_Minimized)) && viewport.get_main_rect().Contains(mouse_platform_pos))
            if (best_candidate == NULL || best_candidate.LastFrontMostStampCount < viewport.LastFrontMostStampCount)
                best_candidate = viewport;
    }
    return best_candidate;
}

// Update viewports and monitor infos
// Note that this is running even if 'ConfigFlags::ViewportsEnable' is not set, in order to clear unused viewports (if any) and update monitor info.
static void update_viewports_new_frame()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.platform_io.viewports.size <= g.viewports.size);

    // Update Minimized status (we need it first in order to decide if we'll apply pos/size of the main viewport)
    const bool viewports_enabled = (g.config_flags_curr_frame & ConfigFlags::ViewportsEnable) != 0;
    if (viewports_enabled)
    {
        for (int n = 0; n < g.viewports.size; n += 1)
        {
            ImGuiViewportP* viewport = g.viewports[n];
            const bool platform_funcs_available = viewport.platform_window_created;
            if (g.platform_io.Platform_GetWindowMinimized && platform_funcs_available)
            {
                bool minimized = g.platform_io.Platform_GetWindowMinimized(viewport);
                if (minimized)
                    viewport.flags |= ImGuiViewportFlags_Minimized;
                else
                    viewport.flags &= ~ImGuiViewportFlags_Minimized;
            }
        }
    }

    // Create/update main viewport with current platform position.
    // FIXME-VIEWPORT: size is driven by backend/user code for backward-compatibility but we should aim to make this more consistent.
    ImGuiViewportP* main_viewport = g.viewports[0];
    IM_ASSERT(main_viewport.ID == IMGUI_VIEWPORT_DEFAULT_ID);
    IM_ASSERT(main_viewport.Window == NULL);
    Vector2D main_viewport_pos = viewports_enabled ? g.platform_io.Platform_GetWindowPos(main_viewport) : Vector2D::new(0.0, 0.0);
    Vector2D main_viewport_size = g.io.display_size;
    if (viewports_enabled && (main_viewport.flags & ImGuiViewportFlags_Minimized))
    {
        main_viewport_pos = main_viewport.pos;    // Preserve last pos/size when minimized (FIXME: We don't do the same for size outside of the viewport path)
        main_viewport_size = main_viewport.size;
    }
    AddUpdateViewport(NULL, IMGUI_VIEWPORT_DEFAULT_ID, main_viewport_pos, main_viewport_size, ViewportFlags::OwnedByApp | ImGuiViewportFlags_CanHostOtherWindows);

    g.CurrentDpiScale = 0.0;
    g.current_viewport = NULL;
    g.mouse_viewport = NULL;
    for (int n = 0; n < g.viewports.size; n += 1)
    {
        ImGuiViewportP* viewport = g.viewports[n];
        viewport.Idx = n;

        // Erase unused viewports
        if (n > 0 && viewport.LastFrameActive < g.frame_count - 2)
        {
            DestroyViewport(viewport);
            n--;
            continue;
        }

        const bool platform_funcs_available = viewport.platform_window_created;
        if (viewports_enabled)
        {
            // Update Position and size (from Platform window to ImGui) if requested.
            // We do it early in the frame instead of waiting for UpdatePlatformWindows() to avoid a frame of lag when moving/resizing using OS facilities.
            if (!(viewport.flags & ImGuiViewportFlags_Minimized) && platform_funcs_available)
            {
                // viewport->work_pos and work_size will be updated below
                if (viewport.PlatformRequestMove)
                    viewport.pos = viewport.LastPlatformPos = g.platform_io.Platform_GetWindowPos(viewport);
                if (viewport.PlatformRequestResize)
                    viewport.size = viewport.LastPlatformSize = g.platform_io.Platform_GetWindowSize(viewport);
            }
        }

        // Update/copy monitor info
        UpdateViewportPlatformMonitor(viewport);

        // Lock down space taken by menu bars and status bars, reset the offset for functions like BeginMainMenuBar() to alter them again.
        viewport.WorkOffsetMin = viewport.BuildWorkOffsetMin;
        viewport.WorkOffsetMax = viewport.BuildWorkOffsetMax;
        viewport.BuildWorkOffsetMin = viewport.BuildWorkOffsetMax = Vector2D::new(0.0, 0.0);
        viewport.update_work_rect();

        // Reset alpha every frame. Users of transparency (docking) needs to request a lower alpha back.
        viewport.alpha = 1.0;

        // Translate Dear ImGui windows when a Host viewport has been moved
        // (This additionally keeps windows at the same place when ConfigFlags::ViewportsEnable is toggled!)
        const Vector2D viewport_delta_pos = viewport.pos - viewport.LastPos;
        if ((viewport.flags & ImGuiViewportFlags_CanHostOtherWindows) && (viewport_delta_pos.x != 0.0 || viewport_delta_pos.y != 0.0))
            TranslateWindowsInViewport(viewport, viewport.LastPos, viewport.pos);

        // Update DPI scale
        float new_dpi_scale;
        if (g.platform_io.Platform_GetWindowDpiScale && platform_funcs_available)
            new_dpi_scale = g.platform_io.Platform_GetWindowDpiScale(viewport);
        else if (viewport.PlatformMonitor != -1)
            new_dpi_scale = g.platform_io.monitors[viewport.PlatformMonitor].DpiScale;
        else
            new_dpi_scale = (viewport.DpiScale != 0.0) ? viewport.DpiScale : 1.0;
        if (viewport.DpiScale != 0.0 && new_dpi_scale != viewport.DpiScale)
        {
            float scale_factor = new_dpi_scale / viewport.DpiScale;
            if (g.io.config_flags & ImGuiConfigFlags_DpiEnableScaleViewports)
                ScaleWindowsInViewport(viewport, scale_factor);
            //if (viewport == GetMainViewport())
            //    g.PlatformInterface.SetWindowSize(viewport, viewport->size * scale_factor);

            // scale our window moving pivot so that the window will rescale roughly around the mouse position.
            // FIXME-VIEWPORT: This currently creates a resizing feedback loop when a window is straddling a DPI transition border.
            // (Minor: since our sizes do not perfectly linearly scale, deferring the click offset scale until we know the actual window scale ratio may get us slightly more precise mouse positioning.)
            //if (g.moving_window != NULL && g.moving_window->viewport == viewport)
            //    g.ActiveIdClickOffset = f32::floor(g.ActiveIdClickOffset * scale_factor);
        }
        viewport.DpiScale = new_dpi_scale;
    }

    // Update fallback monitor
    if (g.platform_io.monitors.size == 0)
    {
        ImGuiPlatformMonitor* monitor = &g.FallbackMonitor;
        monitor.MainPos = main_viewport.pos;
        monitor.MainSize = main_viewport.size;
        monitor.WorkPos = main_viewport.WorkPos;
        monitor.work_size = main_viewport.work_size;
        monitor.DpiScale = main_viewport.DpiScale;
    }

    if (!viewports_enabled)
    {
        g.mouse_viewport = main_viewport;
        return;
    }

    // Mouse handling: decide on the actual mouse viewport for this frame between the active/focused viewport and the hovered viewport.
    // Note that 'viewport_hovered' should skip over any viewport that has the ViewportFlags::NoInputs flags set.
    ImGuiViewportP* viewport_hovered = NULL;
    if (g.io.backend_flags & ImGuiBackendFlags_HasMouseHoveredViewport)
    {
        viewport_hovered = g.io.MouseHoveredViewport ? (ImGuiViewportP*)FindViewportByID(g.io.MouseHoveredViewport) : NULL;
        if (viewport_hovered && (viewport_hovered.flags & ViewportFlags::NoInputs))
            viewport_hovered = FindHoveredViewportFromPlatformWindowStack(g.io.mouse_pos); // Backend failed to handle _NoInputs viewport: revert to our fallback.
    }
    else
    {
        // If the backend doesn't know how to honor ViewportFlags::NoInputs, we do a search ourselves. Note that this search:
        // A) won't take account of the possibility that non-imgui windows may be in-between our dragged window and our target window.
        // B) won't take account of how the backend apply parent<>child relationship to secondary viewports, which affects their Z order.
        // C) uses LastFrameAsRefViewport as a flawed replacement for the last time a window was focused (we could/should fix that by introducing Focus functions in platform_io)
        viewport_hovered = FindHoveredViewportFromPlatformWindowStack(g.io.mouse_pos);
    }
    if (viewport_hovered != NULL)
        g.mouse_last_hovered_viewport = viewport_hovered;
    else if (g.mouse_last_hovered_viewport == NULL)
        g.mouse_last_hovered_viewport = g.viewports[0];

    // Update mouse reference viewport
    // (when moving a window we aim at its viewport, but this will be overwritten below if we go in drag and drop mode)
    // (MovingViewport->viewport will be NULL in the rare situation where the window disappared while moving, set UpdateMouseMovingWindowNewFrame() for details)
    if (g.moving_window && g.moving_window.Viewport)
        g.mouse_viewport = g.moving_window.Viewport;
    else
        g.mouse_viewport = g.mouse_last_hovered_viewport;

    // When dragging something, always refer to the last hovered viewport.
    // - when releasing a moving window we will revert to aiming behind (at viewport_hovered)
    // - when we are between viewports, our dragged preview will tend to show in the last viewport _even_ if we don't have tooltips in their viewports (when lacking monitor info)
    // - consider the case of holding on a menu item to browse child menus: even thou a mouse button is held, there's no active id because menu items only react on mouse release.
    // FIXME-VIEWPORT: This is essentially broken, when ImGuiBackendFlags_HasMouseHoveredViewport is set we want to trust when viewport_hovered==NULL and use that.
    const bool is_mouse_dragging_with_an_expected_destination = g.drag_drop_active;
    if (is_mouse_dragging_with_an_expected_destination && viewport_hovered == NULL)
        viewport_hovered = g.mouse_last_hovered_viewport;
    if (is_mouse_dragging_with_an_expected_destination || g.active_id == 0 || !IsAnyMouseDown())
        if (viewport_hovered != NULL && viewport_hovered != g.mouse_viewport && !(viewport_hovered.flags & ViewportFlags::NoInputs))
            g.mouse_viewport = viewport_hovered;

    IM_ASSERT(g.mouse_viewport != NULL);
}

// Update user-facing viewport list (g.viewports -> g.platform_io.viewports after filtering out some)
static void UpdateViewportsEndFrame()
{
    ImGuiContext& g = *GImGui;
    g.platform_io.viewports.resize(0);
    for (int i = 0; i < g.viewports.size; i += 1)
    {
        ImGuiViewportP* viewport = g.viewports[i];
        viewport.LastPos = viewport.pos;
        if (viewport.LastFrameActive < g.frame_count || viewport.size.x <= 0.0 || viewport.size.y <= 0.0)
            if (i > 0) // Always include main viewport in the list
                continue;
        if (viewport.Window && !is_window_active_and_visible(viewport.Window))
            continue;
        if (i > 0)
            IM_ASSERT(viewport.Window != NULL);
        g.platform_io.viewports.push_back(viewport);
    }
    g.viewports[0].ClearRequestFlags(); // clear main viewport flags because UpdatePlatformWindows() won't do it and may not even be called
}

// FIXME: We should ideally refactor the system to call this every frame (we currently don't)
ImGuiViewportP* AddUpdateViewport(ImGuiWindow* window, ImGuiID id, const Vector2D& pos, const Vector2D& size, ImGuiViewportFlags flags)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(id != 0);

    flags |= ImGuiViewportFlags_IsPlatformWindow;
    if (window != NULL)
    {
        if (g.moving_window && g.moving_window.root_window_dock_tree == window)
            flags |= ViewportFlags::NoInputs | ImGuiViewportFlags_NoFocusOnAppearing;
        if ((window.flags & WindowFlags::NoMouseInputs) && (window.flags & WindowFlags::NoNavInputs))
            flags |= ViewportFlags::NoInputs;
        if (window.flags & WindowFlags::NoFocusOnAppearing)
            flags |= ImGuiViewportFlags_NoFocusOnAppearing;
    }

    ImGuiViewportP* viewport = (ImGuiViewportP*)FindViewportByID(id);
    if (viewport)
    {
        // Always update for main viewport as we are already pulling correct platform pos/size (see #4900)
        if (!viewport.PlatformRequestMove || viewport.ID == IMGUI_VIEWPORT_DEFAULT_ID)
            viewport.pos = pos;
        if (!viewport.PlatformRequestResize || viewport.ID == IMGUI_VIEWPORT_DEFAULT_ID)
            viewport.size = size;
        viewport.flags = flags | (viewport.flags & ImGuiViewportFlags_Minimized); // Preserve existing flags
    }
    else
    {
        // New viewport
        viewport = IM_NEW(ImGuiViewportP)();
        viewport.ID = id;
        viewport.Idx = g.viewports.size;
        viewport.pos = viewport.LastPos = pos;
        viewport.size = size;
        viewport.flags = flags;
        UpdateViewportPlatformMonitor(viewport);
        g.viewports.push_back(viewport);
        IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Add viewport %08X '%s'\n", id, window ? window.Name : "<NULL>");

        // We normally setup for all viewports in NewFrame() but here need to handle the mid-frame creation of a new viewport.
        // We need to extend the fullscreen clip rect so the OverlayDrawList clip is correct for that the first frame
        g.draw_list_shared_data.clip_rect_full_screen.x = ImMin(g.draw_list_shared_data.clip_rect_full_screen.x, viewport.pos.x);
        g.draw_list_shared_data.clip_rect_full_screen.y = ImMin(g.draw_list_shared_data.clip_rect_full_screen.y, viewport.pos.y);
        g.draw_list_shared_data.clip_rect_full_screen.z = ImMax(g.draw_list_shared_data.clip_rect_full_screen.z, viewport.pos.x + viewport.size.x);
        g.draw_list_shared_data.clip_rect_full_screen.w = ImMax(g.draw_list_shared_data.clip_rect_full_screen.w, viewport.pos.y + viewport.size.y);

        // Store initial dpi_scale before the OS platform window creation, based on expected monitor data.
        // This is so we can select an appropriate font size on the first frame of our window lifetime
        if (viewport.PlatformMonitor != -1)
            viewport.DpiScale = g.platform_io.monitors[viewport.PlatformMonitor].DpiScale;
    }

    viewport.Window = window;
    viewport.LastFrameActive = g.frame_count;
    viewport.update_work_rect();
    IM_ASSERT(window == NULL || viewport.ID == window.id);

    if (window != NULL)
        window.viewport_owned = true;

    return viewport;
}

static void DestroyViewport(ImGuiViewportP* viewport)
{
    // clear references to this viewport in windows (window->viewport_id becomes the master data)
    ImGuiContext& g = *GImGui;
    for (int window_n = 0; window_n < g.windows.size; window_n += 1)
    {
        ImGuiWindow* window = g.windows[window_n];
        if (window.viewport != viewport)
            continue;
        window.viewport = NULL;
        window.viewport_owned = false;
    }
    if (viewport == g.mouse_last_hovered_viewport)
        g.mouse_last_hovered_viewport = NULL;

    // Destroy
    IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Delete viewport %08X '%s'\n", viewport.ID, viewport.Window ? viewport.Window.Name : "n/a");
    DestroyPlatformWindow(viewport); // In most circumstances the platform window will already be destroyed here.
    IM_ASSERT(g.platform_io.viewports.contains(viewport) == false);
    IM_ASSERT(g.viewports[viewport.Idx] == viewport);
    g.viewports.erase(g.viewports.data + viewport.Idx);
    IM_DELETE(viewport);
}

// FIXME-VIEWPORT: This is all super messy and ought to be clarified or rewritten.
static void WindowSelectViewport(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindowFlags flags = window.flags;
    window.ViewportAllowPlatformMonitorExtend = -1;

    // Restore main viewport if multi-viewport is not supported by the backend
    ImGuiViewportP* main_viewport = (ImGuiViewportP*)(void*)GetMainViewport();
    if (!(g.config_flags_curr_frame & ConfigFlags::ViewportsEnable))
    {
        SetWindowViewport(window, main_viewport);
        return;
    }
    window.viewport_owned = false;

    // appearing popups reset their viewport so they can inherit again
    if ((flags & (WindowFlags::Popup | WindowFlags::Tooltip)) && window.Appearing)
    {
        window.viewport = NULL;
        window.viewport_id = 0;
    }

    if ((g.next_window_data.flags & NextWindowDataFlags::HasViewport) == 0)
    {
        // By default inherit from parent window
        if (window.viewport == NULL && window.parent_window && (!window.parent_window.IsFallbackWindow || window.parent_window.WasActive))
            window.viewport = window.parent_window.Viewport;

        // Attempt to restore saved viewport id (= window that hasn't been activated yet), try to restore the viewport based on saved 'window->viewport_pos' restored from .ini file
        if (window.viewport == NULL && window.viewport_id != 0)
        {
            window.viewport = (ImGuiViewportP*)FindViewportByID(window.viewport_id);
            if (window.viewport == NULL && window.viewport_pos.x != f32::MAX && window.viewport_pos.y != f32::MAX)
                window.viewport = AddUpdateViewport(window, window.id, window.viewport_pos, window.size, ImGuiViewportFlags_None);
        }
    }

    bool lock_viewport = false;
    if (g.next_window_data.flags & NextWindowDataFlags::HasViewport)
    {
        // Code explicitly request a viewport
        window.viewport = (ImGuiViewportP*)FindViewportByID(g.next_window_data.viewport_id);
        window.viewport_id = g.next_window_data.viewport_id; // Store id even if viewport isn't resolved yet.
        lock_viewport = true;
    }
    else if ((flags & WindowFlags::ChildWindow) || (flags & WindowFlags::ChildMenu))
    {
        // Always inherit viewport from parent window
        if (window.dock_node && window.dock_node.host_window)
            IM_ASSERT(window.dock_node.host_window.Viewport == window.parent_window.Viewport);
        window.viewport = window.parent_window.Viewport;
    }
    else if (window.dock_node && window.dock_node.host_window)
    {
        // This covers the "always inherit viewport from parent window" case for when a window reattach to a node that was just created mid-frame
        window.viewport = window.dock_node.host_window.Viewport;
    }
    else if (flags & WindowFlags::Tooltip)
    {
        window.viewport = g.mouse_viewport;
    }
    else if (GetWindowAlwaysWantOwnViewport(window))
    {
        window.viewport = AddUpdateViewport(window, window.id, window.pos, window.size, ImGuiViewportFlags_None);
    }
    else if (g.moving_window && g.moving_window.root_window_dock_tree == window && is_mouse_pos_valid())
    {
        if (window.viewport != NULL && window.viewport.Window == window)
            window.viewport = AddUpdateViewport(window, window.id, window.pos, window.size, ImGuiViewportFlags_None);
    }
    else
    {
        // merge into host viewport?
        // We cannot test window->viewport_owned as it set lower in the function.
        // Testing (g.active_id == 0 || g.active_id_allow_overlap) to avoid merging during a short-term widget interaction. Main intent was to avoid during resize (see #4212)
        bool try_to_merge_into_host_viewport = (window.viewport && window == window.viewport.Window && (g.active_id == 0 || g.ActiveIdAllowOverlap));
        if (try_to_merge_into_host_viewport)
            UpdateTryMergeWindowIntoHostViewports(window);
    }

    // Fallback: merge in default viewport if z-order matches, otherwise create a new viewport
    if (window.viewport == NULL)
        if (!update_try_merge_window_into_host_viewport(window, main_viewport))
            window.viewport = AddUpdateViewport(window, window.id, window.pos, window.size, ImGuiViewportFlags_None);

    // Mark window as allowed to protrude outside of its viewport and into the current monitor
    if (!lock_viewport)
    {
        if (flags & (WindowFlags::Tooltip | WindowFlags::Popup))
        {
            // We need to take account of the possibility that mouse may become invalid.
            // Popups/Tooltip always set viewport_allow_platform_monitor_extend so GetWindowAllowedExtentRect() will return full monitor bounds.
            Vector2D mouse_ref = (flags & WindowFlags::Tooltip) ? g.io.mouse_pos : g.begin_popup_stack.back().OpenMousePos;
            bool use_mouse_ref = (g.nav_disable_highlight || !g.nav_disable_mouse_hover || !g.nav_window);
            bool mouse_valid = is_mouse_pos_valid(&mouse_ref);
            if ((window.Appearing || (flags & (WindowFlags::Tooltip | WindowFlags::ChildMenu))) && (!use_mouse_ref || mouse_valid))
                window.ViewportAllowPlatformMonitorExtend = FindPlatformMonitorForPos((use_mouse_ref && mouse_valid) ? mouse_ref : NavCalcPreferredRefPos());
            else
                window.ViewportAllowPlatformMonitorExtend = window.viewport.PlatformMonitor;
        }
        else if (window.viewport && window != window.viewport.Window && window.viewport.Window && !(flags & WindowFlags::ChildWindow) && window.dock_node == NULL)
        {
            // When called from Begin() we don't have access to a proper version of the hidden flag yet, so we replicate this code.
            const bool will_be_visible = (window.dock_is_active && !window.DockTabIsVisible) ? false : true;
            if ((window.flags & WindowFlags::DockNodeHost) && window.viewport.LastFrameActive < g.frame_count && will_be_visible)
            {
                // Steal/transfer ownership
                IMGUI_DEBUG_LOG_VIEWPORT("[viewport] window '%s' steal viewport %08X from window '%s'\n", window.Name, window.viewport.ID, window.viewport.Window.Name);
                window.viewport.Window = window;
                window.viewport.ID = window.id;
                window.viewport.LastNameHash = 0;
            }
            else if (!UpdateTryMergeWindowIntoHostViewports(window)) // merge?
            {
                // New viewport
                window.viewport = AddUpdateViewport(window, window.id, window.pos, window.size, ImGuiViewportFlags_NoFocusOnAppearing);
            }
        }
        else if (window.ViewportAllowPlatformMonitorExtend < 0 && (flags & WindowFlags::ChildWindow) == 0)
        {
            // Regular (non-child, non-popup) windows by default are also allowed to protrude
            // Child windows are kept contained within their parent.
            window.ViewportAllowPlatformMonitorExtend = window.viewport.PlatformMonitor;
        }
    }

    // Update flags
    window.viewport_owned = (window == window.viewport.Window);
    window.viewport_id = window.viewport.ID;

    // If the OS window has a title bar, hide our imgui title bar
    //if (window->viewport_owned && !(window->viewport->flags & ImGuiViewportFlags_NoDecoration))
    //    window->flags |= WindowFlags::NoTitleBar;
}

void WindowSyncOwnedViewport(ImGuiWindow* window, ImGuiWindow* parent_window_in_stack)
{
    ImGuiContext& g = *GImGui;

    bool viewport_rect_changed = false;

    // Synchronize window --> viewport in most situations
    // Synchronize viewport -> window in case the platform window has been moved or resized from the OS/WM
    if (window.viewport.PlatformRequestMove)
    {
        window.pos = window.viewport.pos;
        MarkIniSettingsDirty(window);
    }
    else if (memcmp(&window.viewport.pos, &window.pos, sizeof(window.pos)) != 0)
    {
        viewport_rect_changed = true;
        window.viewport.pos = window.pos;
    }

    if (window.viewport.PlatformRequestResize)
    {
        window.size = window.size_full = window.viewport.size;
        MarkIniSettingsDirty(window);
    }
    else if (memcmp(&window.viewport.size, &window.size, sizeof(window.size)) != 0)
    {
        viewport_rect_changed = true;
        window.viewport.size = window.size;
    }
    window.viewport.update_work_rect();

    // The viewport may have changed monitor since the global update in UpdateViewportsNewFrame()
    // Either a SetNextWindowPos() call in the current frame or a set_window_pos() call in the previous frame may have this effect.
    if (viewport_rect_changed)
        UpdateViewportPlatformMonitor(window.viewport);

    // Update common viewport flags
    const ImGuiViewportFlags viewport_flags_to_clear = ImGuiViewportFlags_TopMost | ImGuiViewportFlags_NoTaskBarIcon | ImGuiViewportFlags_NoDecoration | ImGuiViewportFlags_NoRendererClear;
    ImGuiViewportFlags viewport_flags = window.viewport.flags & ~viewport_flags_to_clear;
    ImGuiWindowFlags window_flags = window.flags;
    const bool is_modal = (window_flags & WindowFlags::Modal) != 0;
    const bool is_short_lived_floating_window = (window_flags & (WindowFlags::ChildMenu | WindowFlags::Tooltip | WindowFlags::Popup)) != 0;
    if (window_flags & WindowFlags::Tooltip)
        viewport_flags |= ImGuiViewportFlags_TopMost;
    if ((g.io.ConfigViewportsNoTaskBarIcon || is_short_lived_floating_window) && !is_modal)
        viewport_flags |= ImGuiViewportFlags_NoTaskBarIcon;
    if (g.io.ConfigViewportsNoDecoration || is_short_lived_floating_window)
        viewport_flags |= ImGuiViewportFlags_NoDecoration;

    // Not correct to set modal as topmost because:
    // - Because other popups can be stacked above a modal (e.g. combo box in a modal)
    // - ImGuiViewportFlags_TopMost is currently handled different in backends: in Win32 it is "appear top most" whereas in GLFW and SDL it is "stay topmost"
    //if (flags & ImGuiWindowFlags_Modal)
    //    viewport_flags |= ImGuiViewportFlags_TopMost;

    // For popups and menus that may be protruding out of their parent viewport, we enable _NoFocusOnClick so that clicking on them
    // won't steal the OS focus away from their parent window (which may be reflected in OS the title bar decoration).
    // Setting _NoFocusOnClick would technically prevent us from bringing back to front in case they are being covered by an OS window from a different app,
    // but it shouldn't be much of a problem considering those are already popups that are closed when clicking elsewhere.
    if (is_short_lived_floating_window && !is_modal)
        viewport_flags |= ImGuiViewportFlags_NoFocusOnAppearing | ImGuiViewportFlags_NoFocusOnClick;

    // We can overwrite viewport flags using ImGuiWindowClass (advanced users)
    if (window.WindowClass.ViewportFlagsOverrideSet)
        viewport_flags |= window.WindowClass.ViewportFlagsOverrideSet;
    if (window.WindowClass.ViewportFlagsOverrideClear)
        viewport_flags &= ~window.WindowClass.ViewportFlagsOverrideClear;

    // We can also tell the backend that clearing the platform window won't be necessary,
    // as our window background is filling the viewport and we have disabled BgAlpha.
    // FIXME: Work on support for per-viewport transparency (#2766)
    if (!(window_flags & WindowFlags::NoBackground))
        viewport_flags |= ImGuiViewportFlags_NoRendererClear;

    window.viewport.flags = viewport_flags;

    // Update parent viewport id
    // (the !is_fallback_window test mimic the one done in WindowSelectViewport())
    if (window.WindowClass.ParentViewportId != (ImGuiID)-1)
        window.viewport.ParentViewportId = window.WindowClass.ParentViewportId;
    else if ((window_flags & (WindowFlags::Popup | WindowFlags::Tooltip)) && parent_window_in_stack && (!parent_window_in_stack.IsFallbackWindow || parent_window_in_stack.WasActive))
        window.viewport.ParentViewportId = parent_window_in_stack.Viewport.ID;
    else
        window.viewport.ParentViewportId = g.io.ConfigViewportsNoDefaultParent ? 0 : IMGUI_VIEWPORT_DEFAULT_ID;
}

// Called by user at the end of the main loop, after EndFrame()
// This will handle the creation/update of all OS windows via function defined in the ImGuiPlatformIO api.
void UpdatePlatformWindows()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.frame_count_ended == g.frame_count && "Forgot to call Render() or EndFrame() before UpdatePlatformWindows()?");
    IM_ASSERT(g.FrameCountPlatformEnded < g.frame_count);
    g.FrameCountPlatformEnded = g.frame_count;
    if (!(g.config_flags_curr_frame & ConfigFlags::ViewportsEnable))
        return;

    // Create/resize/destroy platform windows to match each active viewport.
    // Skip the main viewport (index 0), which is always fully handled by the application!
    for (int i = 1; i < g.viewports.size; i += 1)
    {
        ImGuiViewportP* viewport = g.viewports[i];

        // Destroy platform window if the viewport hasn't been submitted or if it is hosting a hidden window
        // (the implicit/fallback Debug##Default window will be registering its viewport then be disabled, causing a dummy DestroyPlatformWindow to be made each frame)
        bool destroy_platform_window = false;
        destroy_platform_window |= (viewport.LastFrameActive < g.frame_count - 1);
        destroy_platform_window |= (viewport.Window && !is_window_active_and_visible(viewport.Window));
        if (destroy_platform_window)
        {
            DestroyPlatformWindow(viewport);
            continue;
        }

        // New windows that appears directly in a new viewport won't always have a size on their first frame
        if (viewport.LastFrameActive < g.frame_count || viewport.size.x <= 0 || viewport.size.y <= 0)
            continue;

        // Create window
        bool is_new_platform_window = (viewport.platform_window_created == false);
        if (is_new_platform_window)
        {
            IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Create Platform window %08X '%s'\n", viewport.ID, viewport.Window ? viewport.Window.Name : "n/a");
            g.platform_io.Platform_CreateWindow(viewport);
            if (g.platform_io.Renderer_CreateWindow != NULL)
                g.platform_io.Renderer_CreateWindow(viewport);
            viewport.LastNameHash = 0;
            viewport.LastPlatformPos = viewport.LastPlatformSize = Vector2D::new(f32::MAX, f32::MAX); // By clearing those we'll enforce a call to Platform_SetWindowPos/size below, before Platform_ShowWindow (FIXME: Is that necessary?)
            viewport.LastRendererSize = viewport.size;                                       // We don't need to call Renderer_SetWindowSize() as it is expected Renderer_CreateWindow() already did it.
            viewport.platform_window_created = true;
        }

        // Apply Position and size (from ImGui to Platform/Renderer backends)
        if ((viewport.LastPlatformPos.x != viewport.pos.x || viewport.LastPlatformPos.y != viewport.pos.y) && !viewport.PlatformRequestMove)
            g.platform_io.Platform_SetWindowPos(viewport, viewport.pos);
        if ((viewport.LastPlatformSize.x != viewport.size.x || viewport.LastPlatformSize.y != viewport.size.y) && !viewport.PlatformRequestResize)
            g.platform_io.Platform_SetWindowSize(viewport, viewport.size);
        if ((viewport.LastRendererSize.x != viewport.size.x || viewport.LastRendererSize.y != viewport.size.y) && g.platform_io.Renderer_SetWindowSize)
            g.platform_io.Renderer_SetWindowSize(viewport, viewport.size);
        viewport.LastPlatformPos = viewport.pos;
        viewport.LastPlatformSize = viewport.LastRendererSize = viewport.size;

        // Update title bar (if it changed)
        if (ImGuiWindow* window_for_title = GetWindowForTitleDisplay(viewport.Window))
        {
            const char* title_begin = window_for_title.Name;
            char* title_end = (char*)(intptr_t)FindRenderedTextEnd(title_begin);
            const ImGuiID title_hash = ImHashStr(title_begin, title_end - title_begin);
            if (viewport.LastNameHash != title_hash)
            {
                char title_end_backup_c = *title_end;
                *title_end = 0; // Cut existing buffer short instead of doing an alloc/free, no small gain.
                g.platform_io.Platform_SetWindowTitle(viewport, title_begin);
                *title_end = title_end_backup_c;
                viewport.LastNameHash = title_hash;
            }
        }

        // Update alpha (if it changed)
        if (viewport.LastAlpha != viewport.alpha && g.platform_io.Platform_SetWindowAlpha)
            g.platform_io.Platform_SetWindowAlpha(viewport, viewport.alpha);
        viewport.LastAlpha = viewport.alpha;

        // Optional, general purpose call to allow the backend to perform general book-keeping even if things haven't changed.
        if (g.platform_io.Platform_UpdateWindow)
            g.platform_io.Platform_UpdateWindow(viewport);

        if (is_new_platform_window)
        {
            // On startup ensure new platform window don't steal focus (give it a few frames, as nested contents may lead to viewport being created a few frames late)
            if (g.frame_count < 3)
                viewport.flags |= ImGuiViewportFlags_NoFocusOnAppearing;

            // Show window
            g.platform_io.Platform_ShowWindow(viewport);

            // Even without focus, we assume the window becomes front-most.
            // This is useful for our platform z-order heuristic when io.mouse_hovered_viewport is not available.
            if (viewport.LastFrontMostStampCount != g.ViewportFrontMostStampCount)
                viewport.LastFrontMostStampCount = g.ViewportFrontMostStampCount += 1;
            }

        // clear request flags
        viewport.ClearRequestFlags();
    }

    // Update our implicit z-order knowledge of platform windows, which is used when the backend cannot provide io.mouse_hovered_viewport.
    // When setting Platform_GetWindowFocus, it is expected that the platform backend can handle calls without crashing if it doesn't have data stored.
    // FIXME-VIEWPORT: We should use this information to also set dear imgui-side focus, allowing us to handle os-level alt+tab.
    if (g.platform_io.Platform_GetWindowFocus != NULL)
    {
        ImGuiViewportP* focused_viewport = NULL;
        for (int n = 0; n < g.viewports.size && focused_viewport == NULL; n += 1)
        {
            ImGuiViewportP* viewport = g.viewports[n];
            if (viewport.platform_window_created)
                if (g.platform_io.Platform_GetWindowFocus(viewport))
                    focused_viewport = viewport;
        }

        // Store a tag so we can infer z-order easily from all our windows
        // We compare platform_last_focused_viewport_id so newly created viewports with _NoFocusOnAppearing flag
        // will keep the front most stamp instead of losing it back to their parent viewport.
        if (focused_viewport && g.PlatformLastFocusedViewportId != focused_viewport.ID)
        {
            if (focused_viewport.LastFrontMostStampCount != g.ViewportFrontMostStampCount)
                focused_viewport.LastFrontMostStampCount = g.ViewportFrontMostStampCount += 1;
            g.PlatformLastFocusedViewportId = focused_viewport.ID;
        }
    }
}

// This is a default/basic function for performing the rendering/swap of multiple Platform windows.
// Custom renderers may prefer to not call this function at all, and instead iterate the publicly exposed platform data and handle rendering/sync themselves.
// The Render/Swap functions stored in ImGuiPlatformIO are merely here to allow for this helper to exist, but you can do it yourself:
//
//    ImGuiPlatformIO& platform_io = GetPlatformIO();
//    for (int i = 1; i < platform_io.viewports.size; i++)
//        if ((platform_io.viewports[i]->flags & ImGuiViewportFlags_Minimized) == 0)
//            MyRenderFunction(platform_io.viewports[i], my_args);
//    for (int i = 1; i < platform_io.viewports.size; i++)
//        if ((platform_io.viewports[i]->flags & ImGuiViewportFlags_Minimized) == 0)
//            MySwapBufferFunction(platform_io.viewports[i], my_args);
//
void RenderPlatformWindowsDefault(void* platform_render_arg, void* renderer_render_arg)
{
    // Skip the main viewport (index 0), which is always fully handled by the application!
    ImGuiPlatformIO& platform_io = GetPlatformIO();
    for (int i = 1; i < platform_io.viewports.size; i += 1)
    {
        ImGuiViewport* viewport = platform_io.viewports[i];
        if (viewport.flags & ImGuiViewportFlags_Minimized)
            continue;
        if (platform_io.Platform_RenderWindow) platform_io.Platform_RenderWindow(viewport, platform_render_arg);
        if (platform_io.Renderer_RenderWindow) platform_io.Renderer_RenderWindow(viewport, renderer_render_arg);
    }
    for (int i = 1; i < platform_io.viewports.size; i += 1)
    {
        ImGuiViewport* viewport = platform_io.viewports[i];
        if (viewport.flags & ImGuiViewportFlags_Minimized)
            continue;
        if (platform_io.Platform_SwapBuffers) platform_io.Platform_SwapBuffers(viewport, platform_render_arg);
        if (platform_io.Renderer_SwapBuffers) platform_io.Renderer_SwapBuffers(viewport, renderer_render_arg);
    }
}

static int FindPlatformMonitorForPos(const Vector2D& pos)
{
    ImGuiContext& g = *GImGui;
    for (int monitor_n = 0; monitor_n < g.platform_io.monitors.size; monitor_n += 1)
    {
        const ImGuiPlatformMonitor& monitor = g.platform_io.monitors[monitor_n];
        if (Rect(monitor.MainPos, monitor.MainPos + monitor.MainSize).Contains(pos))
            return monitor_n;
    }
    return -1;
}

// Search for the monitor with the largest intersection area with the given rectangle
// We generally try to avoid searching loops but the monitor count should be very small here
// FIXME-OPT: We could test the last monitor used for that viewport first, and early
static int FindPlatformMonitorForRect(const Rect& rect)
{
    ImGuiContext& g = *GImGui;

    const int monitor_count = g.platform_io.monitors.size;
    if (monitor_count <= 1)
        return monitor_count - 1;

    // Use a minimum threshold of 1.0 so a zero-sized rect won't false positive, and will still find the correct monitor given its position.
    // This is necessary for tooltips which always resize down to zero at first.
    const float surface_threshold = ImMax(rect.get_width() * rect.get_height() * 0.5, 1.0);
    int best_monitor_n = -1;
    float best_monitor_surface = 0.001;

    for (int monitor_n = 0; monitor_n < g.platform_io.monitors.size && best_monitor_surface < surface_threshold; monitor_n += 1)
    {
        const ImGuiPlatformMonitor& monitor = g.platform_io.monitors[monitor_n];
        const Rect monitor_rect = Rect(monitor.MainPos, monitor.MainPos + monitor.MainSize);
        if (monitor_rect.Contains(rect))
            return monitor_n;
        Rect overlapping_rect = rect;
        overlapping_rect.ClipWithFull(monitor_rect);
        float overlapping_surface = overlapping_rect.get_width() * overlapping_rect.get_height();
        if (overlapping_surface < best_monitor_surface)
            continue;
        best_monitor_surface = overlapping_surface;
        best_monitor_n = monitor_n;
    }
    return best_monitor_n;
}

// Update monitor from viewport rectangle (we'll use this info to clamp windows and save windows lost in a removed monitor)
static void UpdateViewportPlatformMonitor(ImGuiViewportP* viewport)
{
    viewport.PlatformMonitor = FindPlatformMonitorForRect(viewport.get_main_rect());
}

// Return value is always != NULL, but don't hold on it across frames.
const ImGuiPlatformMonitor* GetViewportPlatformMonitor(ImGuiViewport* viewport_p)
{
    ImGuiContext& g = *GImGui;
    ImGuiViewportP* viewport = (ImGuiViewportP*)(void*)viewport_p;
    int monitor_idx = viewport.PlatformMonitor;
    if (monitor_idx >= 0 && monitor_idx < g.platform_io.monitors.size)
        return &g.platform_io.monitors[monitor_idx];
    return &g.FallbackMonitor;
}

void DestroyPlatformWindow(ImGuiViewportP* viewport)
{
    ImGuiContext& g = *GImGui;
    if (viewport.platform_window_created)
    {
        if (g.platform_io.Renderer_DestroyWindow)
            g.platform_io.Renderer_DestroyWindow(viewport);
        if (g.platform_io.Platform_DestroyWindow)
            g.platform_io.Platform_DestroyWindow(viewport);
        IM_ASSERT(viewport.RendererUserData == NULL && viewport.PlatformUserData == NULL);

        // Don't clear PlatformWindowCreated for the main viewport, as we initially set that up to true in Initialize()
        // The righter way may be to leave it to the backend to set this flag all-together, and made the flag public.
        if (viewport.ID != IMGUI_VIEWPORT_DEFAULT_ID)
            viewport.platform_window_created = false;
    }
    else
    {
        IM_ASSERT(viewport.RendererUserData == NULL && viewport.PlatformUserData == NULL && viewport.PlatformHandle == NULL);
    }
    viewport.RendererUserData = viewport.PlatformUserData = viewport.PlatformHandle = NULL;
    viewport.ClearRequestFlags();
}

void destroy_platform_windows()
{
    // We call the destroy window on every viewport (including the main viewport, index 0) to give a chance to the backend
    // to clear any data they may have stored in e.g. PlatformUserData, renderer_user_data.
    // It is convenient for the platform backend code to store something in the main viewport, in order for e.g. the mouse handling
    // code to operator a consistent manner.
    // It is expected that the backend can handle calls to Renderer_DestroyWindow/Platform_DestroyWindow without
    // crashing if it doesn't have data stored.
    ImGuiContext& g = *GImGui;
    for (int i = 0; i < g.viewports.size; i += 1)
        DestroyPlatformWindow(g.viewports[i]);
}


//-----------------------------------------------------------------------------
// [SECTION] DOCKING
//-----------------------------------------------------------------------------
// Docking: Internal Types
// Docking: Forward Declarations
// Docking: ImGuiDockContext
// Docking: ImGuiDockContext Docking/Undocking functions
// Docking: ImGuiDockNode
// Docking: ImGuiDockNode Tree manipulation functions
// Docking: Public Functions (SetWindowDock, DockSpace, DockSpaceOverViewport)
// Docking: Builder Functions
// Docking: Begin/End Support Functions (called from Begin/End)
// Docking: Settings
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// Typical Docking call flow: (root level is generally public API):
//-----------------------------------------------------------------------------
// - NewFrame()                               new dear imgui frame
//    | DockContextNewFrameUpdateUndocking()  - process queued undocking requests
//    | - DockContextProcessUndockWindow()    - process one window undocking request
//    | - DockContextProcessUndockNode()      - process one whole node undocking request
//    | DockContextNewFrameUpdateUndocking()  - process queue docking requests, create floating dock nodes
//    | - update g.hovered_dock_node            - [debug] update node hovered by mouse
//    | - DockContextProcessDock()            - process one docking request
//    | - DockNodeUpdate()
//    |   - DockNodeUpdateForRootNode()
//    |     - DockNodeUpdateFlagsAndCollapse()
//    |     - DockNodeFindInfo()
//    |   - destroy unused node or tab bar
//    |   - create dock node host window
//    |      - Begin() etc.
//    |   - DockNodeStartMouseMovingWindow()
//    |   - DockNodeTreeUpdatePosSize()
//    |   - DockNodeTreeUpdateSplitter()
//    |   - draw node background
//    |   - DockNodeUpdateTabBar()            - create/update tab bar for a docking node
//    |     - DockNodeAddTabBar()
//    |     - DockNodeUpdateWindowMenu()
//    |     - DockNodeCalcTabBarLayout()
//    |     - BeginTabBarEx()
//    |     - TabItemEx() calls
//    |     - EndTabBar()
//    |   - BeginDockableDragDropTarget()
//    |      - DockNodeUpdate()               - recurse into child nodes...
//-----------------------------------------------------------------------------
// - DockSpace()                              user submit a dockspace into a window
//    | Begin(Child)                          - create a child window
//    | DockNodeUpdate()                      - call main dock node update function
//    | End(Child)
//    | ItemSize()
//-----------------------------------------------------------------------------
// - Begin()
//    | BeginDocked()
//    | BeginDockableDragDropSource()
//    | BeginDockableDragDropTarget()
//    | - DockNodePreviewDockRender()
//-----------------------------------------------------------------------------
// - EndFrame()
//    | DockContextEndFrame()
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// Docking: Internal Types
//-----------------------------------------------------------------------------
// - ImGuiDockRequestType
// - ImGuiDockRequest
// - ImGuiDockPreviewData
// - ImGuiDockNodeSettings
// - ImGuiDockContext
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// Docking: Forward Declarations
//-----------------------------------------------------------------------------
//
// namespace ImGui
// {
//     // ImGuiDockContext
//     static ImGuiDockNode*   DockContextAddNode(ImGuiContext* ctx, ImGuiID id);
//     static void             DockContextRemoveNode(ImGuiContext* ctx, ImGuiDockNode* node, bool merge_sibling_into_parent_node);
//     static void             DockContextQueueNotifyRemovedNode(ImGuiContext* ctx, ImGuiDockNode* node);
//     static void             DockContextProcessDock(ImGuiContext* ctx, ImGuiDockRequest* req);
//     static void             DockContextProcessUndockWindow(ImGuiContext* ctx, ImGuiWindow* window, bool clear_persistent_docking_ref = true);
//     static void             DockContextProcessUndockNode(ImGuiContext* ctx, ImGuiDockNode* node);
//     static void             DockContextPruneUnusedSettingsNodes(ImGuiContext* ctx);
//     static ImGuiDockNode*   DockContextFindNodeByID(ImGuiContext* ctx, ImGuiID id);
//     static ImGuiDockNode*   DockContextBindNodeToWindow(ImGuiContext* ctx, ImGuiWindow* window);
//     static void             DockContextBuildNodesFromSettings(ImGuiContext* ctx, ImGuiDockNodeSettings* node_settings_array, int node_settings_count);
//     static void             DockContextBuildAddWindowsToNodes(ImGuiContext* ctx, ImGuiID root_id);                            // Use root_id==0 to add all
//
//     // ImGuiDockNode
//     static void             DockNodeAddWindow(ImGuiDockNode* node, ImGuiWindow* window, bool add_to_tab_bar);
//     static void             DockNodeMoveWindows(ImGuiDockNode* dst_node, ImGuiDockNode* src_node);
//     static void             DockNodeMoveChildNodes(ImGuiDockNode* dst_node, ImGuiDockNode* src_node);
//     static ImGuiWindow*     DockNodeFindWindowByID(ImGuiDockNode* node, ImGuiID id);
//     static void             DockNodeApplyPosSizeToWindows(ImGuiDockNode* node);
//     static void             DockNodeRemoveWindow(ImGuiDockNode* node, ImGuiWindow* window, ImGuiID save_dock_id);
//     static void             DockNodeHideHostWindow(ImGuiDockNode* node);
//     static void             DockNodeUpdate(ImGuiDockNode* node);
//     static void             DockNodeUpdateForRootNode(ImGuiDockNode* node);
//     static void             DockNodeUpdateFlagsAndCollapse(ImGuiDockNode* node);
//     static void             DockNodeUpdateHasCentralNodeChild(ImGuiDockNode* node);
//     static void             DockNodeUpdateTabBar(ImGuiDockNode* node, ImGuiWindow* host_window);
//     static void             DockNodeAddTabBar(ImGuiDockNode* node);
//     static void             DockNodeRemoveTabBar(ImGuiDockNode* node);
//     static ImGuiID          DockNodeUpdateWindowMenu(ImGuiDockNode* node, ImGuiTabBar* tab_bar);
//     static void             DockNodeUpdateVisibleFlag(ImGuiDockNode* node);
//     static void             DockNodeStartMouseMovingWindow(ImGuiDockNode* node, ImGuiWindow* window);
//     static bool             DockNodeIsDropAllowed(ImGuiWindow* host_window, ImGuiWindow* payload_window);
//     static void             DockNodePreviewDockSetup(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* payload_window, ImGuiDockPreviewData* preview_data, bool is_explicit_target, bool is_outer_docking);
//     static void             DockNodePreviewDockRender(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* payload_window, const ImGuiDockPreviewData* preview_data);
//     static void             DockNodeCalcTabBarLayout(const ImGuiDockNode* node, ImRect* out_title_rect, ImRect* out_tab_bar_rect, Vector2D* out_window_menu_button_pos, Vector2D* out_close_button_pos);
//     static void             DockNodeCalcSplitRects(Vector2D& pos_old, Vector2D& size_old, Vector2D& pos_new, Vector2D& size_new, ImGuiDir dir, Vector2D size_new_desired);
//     static bool             DockNodeCalcDropRectsAndTestMousePos(const ImRect& parent, ImGuiDir dir, ImRect& out_draw, bool outer_docking, Vector2D* test_mouse_pos);
//     static const char*      DockNodeGetHostWindowTitle(ImGuiDockNode* node, char* buf, int buf_size) { ImFormatString(buf, buf_size, "##DockNode_%02X", node->id); return buf; }
//     static int              DockNodeGetTabOrder(ImGuiWindow* window);
//
//     // ImGuiDockNode tree manipulations
//     static void             DockNodeTreeSplit(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiAxis split_axis, int split_first_child, float split_ratio, ImGuiDockNode* new_node);
//     static void             DockNodeTreeMerge(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiDockNode* merge_lead_child);
//     static void             DockNodeTreeUpdatePosSize(ImGuiDockNode* node, Vector2D pos, Vector2D size, ImGuiDockNode* only_write_to_single_node = NULL);
//     static void             DockNodeTreeUpdateSplitter(ImGuiDockNode* node);
//     static ImGuiDockNode*   DockNodeTreeFindVisibleNodeByPos(ImGuiDockNode* node, Vector2D pos);
//     static ImGuiDockNode*   DockNodeTreeFindFallbackLeafNode(ImGuiDockNode* node);
//
//     // Settings
//     static void             DockSettingsRenameNodeReferences(ImGuiID old_node_id, ImGuiID new_node_id);
//     static void             DockSettingsRemoveNodeReferences(ImGuiID* node_ids, int node_ids_count);
//     static ImGuiDockNodeSettings*   DockSettingsFindNodeSettings(ImGuiContext* ctx, ImGuiID node_id);
//     static void             DockSettingsHandler_ClearAll(ImGuiContext*, ImGuiSettingsHandler*);
//     static void             DockSettingsHandler_ApplyAll(ImGuiContext*, ImGuiSettingsHandler*);
//     static void*            DockSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, const char* name);
//     static void             DockSettingsHandler_ReadLine(ImGuiContext*, ImGuiSettingsHandler*, void* entry, const char* line);
//     static void             DockSettingsHandler_WriteAll(ImGuiContext* imgui_ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* buf);
// }

//-----------------------------------------------------------------------------
// Docking: ImGuiDockContext
//-----------------------------------------------------------------------------
// The lifetime model is different from the one of regular windows: we always create a ImGuiDockNode for each ImGuiDockNodeSettings,
// or we always hold the entire docking node tree. Nodes are frequently hidden, e.g. if the window(s) or child nodes they host are not active.
// At boot time only, we run a simple GC to remove nodes that have no references.
// Because dock node settings (which are small, contiguous structures) are always mirrored by their corresponding dock nodes (more complete structures),
// we can also very easily recreate the nodes from scratch given the settings data (this is what DockContextRebuild() does).
// This is convenient as docking reconfiguration can be implemented by mostly poking at the simpler settings data.
//-----------------------------------------------------------------------------
// - DockContextInitialize()
// - DockContextShutdown()
// - DockContextClearNodes()
// - DockContextRebuildNodes()
// - DockContextNewFrameUpdateUndocking()
// - DockContextNewFrameUpdateDocking()
// - DockContextEndFrame()
// - DockContextFindNodeByID()
// - DockContextBindNodeToWindow()
// - DockContextGenNodeID()
// - DockContextAddNode()
// - DockContextRemoveNode()
// - ImGuiDockContextPruneNodeData
// - DockContextPruneUnusedSettingsNodes()
// - DockContextBuildNodesFromSettings()
// - DockContextBuildAddWindowsToNodes()
//-----------------------------------------------------------------------------

static int IMGUI_CDECL DockNodeComparerDepthMostFirst(const void* lhs, const void* rhs)
{
    const ImGuiDockNode* a = *(const ImGuiDockNode* const*)lhs;
    const ImGuiDockNode* b = *(const ImGuiDockNode* const*)rhs;
    return DockNodeGetDepth(b) - DockNodeGetDepth(a);
}

// Pre C++0x doesn't allow us to use a function-local type (without linkage) as template parameter, so we moved this here.
struct ImGuiDockContextPruneNodeData
{
    int         CountWindows, CountChildWindows, CountChildNodes;
    ImGuiID     RootId;
    ImGuiDockContextPruneNodeData() { CountWindows = CountChildWindows = CountChildNodes = 0; RootId = 0; }
};

// Garbage collect unused nodes (run once at init time)
static void DockContextPruneUnusedSettingsNodes(ImGuiContext* ctx)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc  = &ctx.DockContext;
    IM_ASSERT(g.windows.size == 0);

    ImPool<ImGuiDockContextPruneNodeData> pool;
    pool.Reserve(dc.NodesSettings.size);

    // Count child nodes and compute RootID
    for (int settings_n = 0; settings_n < dc.NodesSettings.size; settings_n += 1)
    {
        ImGuiDockNodeSettings* settings = &dc.NodesSettings[settings_n];
        ImGuiDockContextPruneNodeData* parent_data = settings.ParentNodeId ? pool.GetByKey(settings.ParentNodeId) : 0;
        pool.GetOrAddByKey(settings.ID).RootId = parent_data ? parent_data.RootId : settings.ID;
        if (settings.ParentNodeId)
            pool.GetOrAddByKey(settings.ParentNodeId).CountChildNodes += 1;
    }

    // Count reference to dock ids from dockspaces
    // We track the 'auto-dock_node <- manual-window <- manual-DockSpace' in order to avoid 'auto-dock_node' being ditched by DockContextPruneUnusedSettingsNodes()
    for (int settings_n = 0; settings_n < dc.NodesSettings.size; settings_n += 1)
    {
        ImGuiDockNodeSettings* settings = &dc.NodesSettings[settings_n];
        if (settings.ParentWindowId != 0)
            if (ImGuiWindowSettings* window_settings = FindWindowSettings(settings.ParentWindowId))
                if (window_settings.dock_id)
                    if (ImGuiDockContextPruneNodeData* data = pool.GetByKey(window_settings.dock_id))
                        data.CountChildNodes += 1;
    }

    // Count reference to dock ids from window settings
    // We guard against the possibility of an invalid .ini file (RootID may point to a missing node)
    for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
        if (ImGuiID dock_id = settings.dock_id)
            if (ImGuiDockContextPruneNodeData* data = pool.GetByKey(dock_id))
            {
                data.CountWindows += 1;
                if (ImGuiDockContextPruneNodeData* data_root = (data.RootId == dock_id) ? data : pool.GetByKey(data.RootId))
                    data_root.CountChildWindows += 1;
            }

    // Prune
    for (int settings_n = 0; settings_n < dc.NodesSettings.size; settings_n += 1)
    {
        ImGuiDockNodeSettings* settings = &dc.NodesSettings[settings_n];
        ImGuiDockContextPruneNodeData* data = pool.GetByKey(settings.ID);
        if (data.CountWindows > 1)
            continue;
        ImGuiDockContextPruneNodeData* data_root = (data.RootId == settings.ID) ? data : pool.GetByKey(data.RootId);

        bool remove = false;
        remove |= (data.CountWindows == 1 && settings.ParentNodeId == 0 && data.CountChildNodes == 0 && !(settings.flags & ImGuiDockNodeFlags_CentralNode));  // Floating root node with only 1 window
        remove |= (data.CountWindows == 0 && settings.ParentNodeId == 0 && data.CountChildNodes == 0); // Leaf nodes with 0 window
        remove |= (data_root.CountChildWindows == 0);
        if (remove)
        {
            IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextPruneUnusedSettingsNodes: Prune 0x%08X\n", settings.ID);
            DockSettingsRemoveNodeReferences(&settings.ID, 1);
            settings.ID = 0;
        }
    }
}

static void DockContextBuildNodesFromSettings(ImGuiContext* ctx, ImGuiDockNodeSettings* node_settings_array, int node_settings_count)
{
    // build nodes
    for (int node_n = 0; node_n < node_settings_count; node_n += 1)
    {
        ImGuiDockNodeSettings* settings = &node_settings_array[node_n];
        if (settings.ID == 0)
            continue;
        ImGuiDockNode* node = DockContextAddNode(ctx, settings.ID);
        node.ParentNode = settings.ParentNodeId ? DockContextFindNodeByID(ctx, settings.ParentNodeId) : NULL;
        node.pos = Vector2D::new(settings.pos.x, settings.pos.y);
        node.size = Vector2D::new(settings.size.x, settings.size.y);
        node.sizeRef = Vector2D::new(settings.sizeRef.x, settings.sizeRef.y);
        node.AuthorityForPos = node.AuthorityForSize = node.AuthorityForViewport = ImGuiDataAuthority_DockNode;
        if (node.ParentNode && node.ParentNode.ChildNodes[0] == NULL)
            node.ParentNode.ChildNodes[0] = node;
        else if (node.ParentNode && node.ParentNode.ChildNodes[1] == NULL)
            node.ParentNode.ChildNodes[1] = node;
        node.SelectedTabId = settings.SelectedTabId;
        node.SplitAxis = (ImGuiAxis)settings.SplitAxis;
        node.SetLocalFlags(settings.flags & ImGuiDockNodeFlags_SavedFlagsMask_);

        // Bind host window immediately if it already exist (in case of a rebuild)
        // This is useful as the root_window_for_title_bar_highlight links necessary to highlight the currently focused node requires node->host_window to be set.
        char host_window_title[20];
        ImGuiDockNode* root_node = DockNodeGetRootNode(node);
        node.host_window = FindWindowByName(DockNodeGetHostWindowTitle(root_node, host_window_title, IM_ARRAYSIZE(host_window_title)));
    }
}

void DockContextBuildAddWindowsToNodes(ImGuiContext* ctx, ImGuiID root_id)
{
    // Rebind all windows to nodes (they can also lazily rebind but we'll have a visible glitch during the first frame)
    ImGuiContext& g = *ctx;
    for (int n = 0; n < g.windows.size; n += 1)
    {
        ImGuiWindow* window = g.windows[n];
        if (window.DockId == 0 || window.LastFrameActive < g.frame_count - 1)
            continue;
        if (window.dock_node != NULL)
            continue;

        ImGuiDockNode* node = DockContextFindNodeByID(ctx, window.DockId);
        IM_ASSERT(node != NULL);   // This should have been called after DockContextBuildNodesFromSettings()
        if (root_id == 0 || DockNodeGetRootNode(node).ID == root_id)
            DockNodeAddWindow(node, window, true);
    }
}

//-----------------------------------------------------------------------------
// Docking: ImGuiDockContext Docking/Undocking functions
//-----------------------------------------------------------------------------
// - DockContextQueueDock()
// - DockContextQueueUndockWindow()
// - DockContextQueueUndockNode()
// - DockContextQueueNotifyRemovedNode()
// - DockContextProcessDock()
// - DockContextProcessUndockWindow()
// - DockContextProcessUndockNode()
// - DockContextCalcDropPosForDocking()
//-----------------------------------------------------------------------------

void DockContextQueueDock(ImGuiContext* ctx, ImGuiWindow* target, ImGuiDockNode* target_node, ImGuiWindow* payload, ImGuiDir split_dir, float split_ratio, bool split_outer)
{
    IM_ASSERT(target != payload);
    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Dock;
    req.DockTargetWindow = target;
    req.DockTargetNode = target_node;
    req.DockPayload = payload;
    req.DockSplitDir = split_dir;
    req.DockSplitRatio = split_ratio;
    req.DockSplitOuter = split_outer;
    ctx.DockContext.Requests.push_back(req);
}

void DockContextQueueUndockWindow(ImGuiContext* ctx, ImGuiWindow* window)
{
    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Undock;
    req.UndockTargetWindow = window;
    ctx.DockContext.Requests.push_back(req);
}

void DockContextQueueUndockNode(ImGuiContext* ctx, ImGuiDockNode* node)
{
    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Undock;
    req.UndockTargetNode = node;
    ctx.DockContext.Requests.push_back(req);
}

void DockContextQueueNotifyRemovedNode(ImGuiContext* ctx, ImGuiDockNode* node)
{
    ImGuiDockContext* dc  = &ctx.DockContext;
    for (int n = 0; n < dc.Requests.size; n += 1)
        if (dc.Requests[n].DockTargetNode == node)
            dc.Requests[n].Type = ImGuiDockRequestType_None;
}

void DockContextProcessDock(ImGuiContext* ctx, ImGuiDockRequest* req)
{
    IM_ASSERT((req.Type == ImGuiDockRequestType_Dock && req.DockPayload != NULL) || (req.Type == ImGuiDockRequestType_Split && req.DockPayload == NULL));
    IM_ASSERT(req.DockTargetWindow != NULL || req.DockTargetNode != NULL);

    ImGuiContext& g = *ctx;
    IM_UNUSED(g);

    ImGuiWindow* payload_window = req.DockPayload;     // Optional
    ImGuiWindow* target_window = req.DockTargetWindow;
    ImGuiDockNode* node = req.DockTargetNode;
    if (payload_window)
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessDock node 0x%08X target '%s' dock window '%s', split_dir %d\n", node ? node.ID : 0, target_window ? target_window.Name : "NULL", payload_window ? payload_window.Name : "NULL", req.DockSplitDir);
    else
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessDock node 0x%08X, split_dir %d\n", node ? node.ID : 0, req.DockSplitDir);

    // Decide which Tab will be selected at the end of the operation
    ImGuiID next_selected_id = 0;
    ImGuiDockNode* payload_node = NULL;
    if (payload_window)
    {
        payload_node = payload_window.DockNodeAsHost;
        payload_window.DockNodeAsHost = NULL; // Important to clear this as the node will have its life as a child which might be merged/deleted later.
        if (payload_node && payload_node.IsLeafNode())
            next_selected_id = payload_node.TabBar.NextSelectedTabId ? payload_node.TabBar.NextSelectedTabId : payload_node.TabBar.SelectedTabId;
        if (payload_node == NULL)
            next_selected_id = payload_window.TabId;
    }

    // FIXME-DOCK: When we are trying to dock an existing single-window node into a loose window, transfer Node id as well
    // When processing an interactive split, usually last_frame_alive will be < g.frame_count. But DockBuilder operations can make it ==.
    if (node)
        IM_ASSERT(node.LastFrameAlive <= g.frame_count);
    if (node && target_window && node == target_window.DockNodeAsHost)
        IM_ASSERT(node.Windows.size > 0 || node.IsSplitNode() || node.IsCentralNode());

    // Create new node and add existing window to it
    if (node == NULL)
    {
        node = DockContextAddNode(ctx, 0);
        node.pos = target_window.Pos;
        node.size = target_window.size;
        if (target_window.DockNodeAsHost == NULL)
        {
            DockNodeAddWindow(node, target_window, true);
            node.TabBar.Tabs[0].flags &= ~ImGuiTabItemFlags_Unsorted;
            target_window.dock_is_active = true;
        }
    }

    ImGuiDir split_dir = req.DockSplitDir;
    if (split_dir != Dir::None)
    {
        // split into two, one side will be our payload node unless we are dropping a loose window
        const ImGuiAxis split_axis = (split_dir == Dir::Left || split_dir == Dir::Right) ? Axis::X : Axis::Y;
        const int split_inheritor_child_idx = (split_dir == Dir::Left || split_dir == Dir::Up) ? 1 : 0; // Current contents will be moved to the opposite side
        const float split_ratio = req.DockSplitRatio;
        DockNodeTreeSplit(ctx, node, split_axis, split_inheritor_child_idx, split_ratio, payload_node);  // payload_node may be NULL here!
        ImGuiDockNode* new_node = node.ChildNodes[split_inheritor_child_idx ^ 1];
        new_node.host_window = node.host_window;
        node = new_node;
    }
    node.SetLocalFlags(node.LocalFlags & ~ImGuiDockNodeFlags_HiddenTabBar);

    if (node != payload_node)
    {
        // Create tab bar before we call DockNodeMoveWindows (which would attempt to move the old tab-bar, which would lead us to payload tabs wrongly appearing before target tabs!)
        if (node.Windows.size > 0 && node.TabBar == NULL)
        {
            DockNodeAddTabBar(node);
            for (int n = 0; n < node.Windows.size; n += 1)
                TabBarAddTab(node.TabBar, ImGuiTabItemFlags_None, node.Windows[n]);
        }

        if (payload_node != NULL)
        {
            // Transfer full payload node (with 1+ child windows or child nodes)
            if (payload_node.IsSplitNode())
            {
                if (node.Windows.size > 0)
                {
                    // We can dock a split payload into a node that already has windows _only_ if our payload is a node tree with a single visible node.
                    // In this situation, we move the windows of the target node into the currently visible node of the payload.
                    // This allows us to preserve some of the underlying dock tree settings nicely.
                    IM_ASSERT(payload_node.OnlyNodeWithWindows != NULL); // The docking should have been blocked by DockNodePreviewDockSetup() early on and never submitted.
                    ImGuiDockNode* visible_node = payload_node.OnlyNodeWithWindows;
                    if (visible_node.TabBar)
                        IM_ASSERT(visible_node.TabBar.Tabs.size > 0);
                    DockNodeMoveWindows(node, visible_node);
                    DockNodeMoveWindows(visible_node, node);
                    DockSettingsRenameNodeReferences(node.ID, visible_node.ID);
                }
                if (node.IsCentralNode())
                {
                    // Central node property needs to be moved to a leaf node, pick the last focused one.
                    // FIXME-DOCK: If we had to transfer other flags here, what would the policy be?
                    ImGuiDockNode* last_focused_node = DockContextFindNodeByID(ctx, payload_node.LastFocusedNodeId);
                    IM_ASSERT(last_focused_node != NULL);
                    ImGuiDockNode* last_focused_root_node = DockNodeGetRootNode(last_focused_node);
                    IM_ASSERT(last_focused_root_node == DockNodeGetRootNode(payload_node));
                    last_focused_node.SetLocalFlags(last_focused_node.LocalFlags | ImGuiDockNodeFlags_CentralNode);
                    node.SetLocalFlags(node.LocalFlags & ~ImGuiDockNodeFlags_CentralNode);
                    last_focused_root_node.CentralNode = last_focused_node;
                }

                IM_ASSERT(node.Windows.size == 0);
                DockNodeMoveChildNodes(node, payload_node);
            }
            else
            {
                const ImGuiID payload_dock_id = payload_node.ID;
                DockNodeMoveWindows(node, payload_node);
                DockSettingsRenameNodeReferences(payload_dock_id, node.ID);
            }
            DockContextRemoveNode(ctx, payload_node, true);
        }
        else if (payload_window)
        {
            // Transfer single window
            const ImGuiID payload_dock_id = payload_window.DockId;
            node.VisibleWindow = payload_window;
            DockNodeAddWindow(node, payload_window, true);
            if (payload_dock_id != 0)
                DockSettingsRenameNodeReferences(payload_dock_id, node.ID);
        }
    }
    else
    {
        // When docking a floating single window node we want to reevaluate auto-hiding of the tab bar
        node.WantHiddenTabBarUpdate = true;
    }

    // Update selection immediately
    if (ImGuiTabBar* tab_bar = node.TabBar)
        tab_bar.NextSelectedTabId = next_selected_id;
    MarkIniSettingsDirty();
}

// Problem:
//   Undocking a large (~full screen) window would leave it so large that the bottom right sizing corner would more
//   than likely be off the screen and the window would be hard to resize to fit on screen. This can be particularly problematic
//   with 'config_windows_move_from_title_bar_only=true' and/or with 'config_windows_resize_from_edges=false' as well (the later can be
//   due to missing ImGuiBackendFlags_HasMouseCursors backend flag).
// Solution:
//   When undocking a window we currently force its maximum size to 90% of the host viewport or monitor.
// Reevaluate this when we implement preserving docked/undocked size ("docking_wip/undocked_size" branch).
static Vector2D FixLargeWindowsWhenUndocking(const Vector2D& size, ImGuiViewport* ref_viewport)
{
    if (ref_viewport == NULL)
        return size;

    ImGuiContext& g = *GImGui;
    Vector2D max_size = f32::floor(ref_viewport.work_size * 0.90);
    if (g.config_flags_curr_frame & ConfigFlags::ViewportsEnable)
    {
        const ImGuiPlatformMonitor* monitor = GetViewportPlatformMonitor(ref_viewport);
        max_size = f32::floor(monitor.work_size * 0.90);
    }
    return ImMin(size, max_size);
}

void DockContextProcessUndockWindow(ImGuiContext* ctx, ImGuiWindow* window, bool clear_persistent_docking_ref)
{
    ImGuiContext& g = *ctx;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessUndockWindow window '%s', clear_persistent_docking_ref = %d\n", window.Name, clear_persistent_docking_ref);
    if (window.dock_node)
        DockNodeRemoveWindow(window.dock_node, window, clear_persistent_docking_ref ? 0 : window.DockId);
    else
        window.DockId = 0;
    window.collapsed = false;
    window.dock_is_active = false;
    window.DockNodeIsVisible = window.DockTabIsVisible = false;
    window.size = window.size_full = FixLargeWindowsWhenUndocking(window.size_full, window.viewport);

    MarkIniSettingsDirty();
}

void DockContextProcessUndockNode(ImGuiContext* ctx, ImGuiDockNode* node)
{
    ImGuiContext& g = *ctx;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessUndockNode node %08X\n", node.ID);
    IM_ASSERT(node.IsLeafNode());
    IM_ASSERT(node.Windows.size >= 1);

    if (node.IsRootNode() || node.IsCentralNode())
    {
        // In the case of a root node or central node, the node will have to stay in place. Create a new node to receive the payload.
        ImGuiDockNode* new_node = DockContextAddNode(ctx, 0);
        new_node.pos = node.pos;
        new_node.size = node.size;
        new_node.sizeRef = node.sizeRef;
        DockNodeMoveWindows(new_node, node);
        DockSettingsRenameNodeReferences(node.ID, new_node.ID);
        for (int n = 0; n < new_node.Windows.size; n += 1)
        {
            ImGuiWindow* window = new_node.Windows[n];
            window.flags &= ~WindowFlags::ChildWindow;
            if (window.parent_window)
                window.parent_window.DC.ChildWindows.find_erase(window);
            UpdateWindowParentAndRootLinks(window, window.flags, NULL);
        }
        node = new_node;
    }
    else
    {
        // Otherwise extract our node and merge our sibling back into the parent node.
        IM_ASSERT(node.ParentNode.ChildNodes[0] == node || node.ParentNode.ChildNodes[1] == node);
        int index_in_parent = (node.ParentNode.ChildNodes[0] == node) ? 0 : 1;
        node.ParentNode.ChildNodes[index_in_parent] = NULL;
        DockNodeTreeMerge(ctx, node.ParentNode, node.ParentNode.ChildNodes[index_in_parent ^ 1]);
        node.ParentNode.AuthorityForViewport = ImGuiDataAuthority_Window; // The node that stays in place keeps the viewport, so our newly dragged out node will create a new viewport
        node.ParentNode = NULL;
    }
    node.AuthorityForPos = node.AuthorityForSize = ImGuiDataAuthority_DockNode;
    node.size = FixLargeWindowsWhenUndocking(node.size, node.Windows[0].Viewport);
    node.WantMouseMove = true;
    MarkIniSettingsDirty();
}

// This is mostly used for automation.
bool DockContextCalcDropPosForDocking(ImGuiWindow* target, ImGuiDockNode* target_node, ImGuiWindow* payload, ImGuiDir split_dir, bool split_outer, Vector2D* out_pos)
{
    // In DockNodePreviewDockSetup() for a root central node instead of showing both "inner" and "outer" drop rects
    // (which would be functionally identical) we only show the outer one. Reflect this here.
    if (target_node && target_node.ParentNode == NULL && target_node.IsCentralNode() && split_dir != Dir::None)
        split_outer = true;
    ImGuiDockPreviewData split_data;
    DockNodePreviewDockSetup(target, target_node, payload, &split_data, false, split_outer);
    if (split_data.DropRectsDraw[split_dir+1].IsInverted())
        return false;
    *out_pos = split_data.DropRectsDraw[split_dir+1].GetCenter();
    return true;
}

//-----------------------------------------------------------------------------
// Docking: ImGuiDockNode
//-----------------------------------------------------------------------------
// - DockNodeGetTabOrder()
// - DockNodeAddWindow()
// - DockNodeRemoveWindow()
// - DockNodeMoveChildNodes()
// - DockNodeMoveWindows()
// - DockNodeApplyPosSizeToWindows()
// - DockNodeHideHostWindow()
// - ImGuiDockNodeFindInfoResults
// - DockNodeFindInfo()
// - DockNodeFindWindowByID()
// - DockNodeUpdateFlagsAndCollapse()
// - DockNodeUpdateHasCentralNodeFlag()
// - DockNodeUpdateVisibleFlag()
// - DockNodeStartMouseMovingWindow()
// - DockNodeUpdate()
// - DockNodeUpdateWindowMenu()
// - DockNodeBeginAmendTabBar()
// - DockNodeEndAmendTabBar()
// - DockNodeUpdateTabBar()
// - DockNodeAddTabBar()
// - DockNodeRemoveTabBar()
// - DockNodeIsDropAllowedOne()
// - DockNodeIsDropAllowed()
// - DockNodeCalcTabBarLayout()
// - DockNodeCalcSplitRects()
// - DockNodeCalcDropRectsAndTestMousePos()
// - DockNodePreviewDockSetup()
// - DockNodePreviewDockRender()
//-----------------------------------------------------------------------------

ImGuiDockNode::ImGuiDockNode(ImGuiID id)
{
    ID = id;
    SharedFlags = LocalFlags = LocalFlagsInWindows = MergedFlags = ImGuiDockNodeFlags_None;
    ParentNode = ChildNodes[0] = ChildNodes[1] = NULL;
    TabBar = NULL;
    SplitAxis = ImGuiAxis_None;

    State = ImGuiDockNodeState_Unknown;
    LastBgColor = IM_COL32_WHITE;
    HostWindow = VisibleWindow = NULL;
    CentralNode = OnlyNodeWithWindows = NULL;
    CountNodeWithWindows = 0;
    LastFrameAlive = LastFrameActive = LastFrameFocused = -1;
    LastFocusedNodeId = 0;
    SelectedTabId = 0;
    WantCloseTabId = 0;
    AuthorityForPos = AuthorityForSize = ImGuiDataAuthority_DockNode;
    AuthorityForViewport = ImGuiDataAuthority_Auto;
    IsVisible = true;
    IsFocused = HasCloseButton = HasWindowMenuButton = HasCentralNodeChild = false;
    IsBgDrawnThisFrame = false;
    WantCloseAll = WantLockSizeOnce = WantMouseMove = WantHiddenTabBarUpdate = WantHiddenTabBarToggle = false;
}

ImGuiDockNode::~ImGuiDockNode()
{
    IM_DELETE(TabBar);
    TabBar = NULL;
    ChildNodes[0] = ChildNodes[1] = NULL;
}

int DockNodeGetTabOrder(ImGuiWindow* window)
{
    ImGuiTabBar* tab_bar = window.dock_node.TabBar;
    if (tab_bar == NULL)
        return -1;
    ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, window.TabId);
    return tab ? tab_bar.GetTabOrder(tab) : -1;
}

static void DockNodeHideWindowDuringHostWindowCreation(ImGuiWindow* window)
{
    window.hidden = true;
    window..hidden_frames_can_skip_items = window.active ? 1 : 2;
}

static void DockNodeAddWindow(ImGuiDockNode* node, ImGuiWindow* window, bool add_to_tab_bar)
{
    ImGuiContext& g = *GImGui; (void)g;
    if (window.dock_node)
    {
        // Can overwrite an existing window->dock_node (e.g. pointing to a disabled DockSpace node)
        IM_ASSERT(window.dock_node.ID != node.ID);
        DockNodeRemoveWindow(window.dock_node, window, 0);
    }
    IM_ASSERT(window.dock_node == NULL || window.DockNodeAsHost == NULL);
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeAddWindow node 0x%08X window '%s'\n", node.ID, window.Name);

    // If more than 2 windows appeared on the same frame leading to the creation of a new hosting window,
    // we'll hide windows until the host window is ready. Hide the 1st window after its been output (so it is not visible for one frame).
    // We will call DockNodeHideWindowDuringHostWindowCreation() on ourselves in Begin()
    if (node.host_window == NULL && node.Windows.size == 1 && node.Windows[0].WasActive == false)
        DockNodeHideWindowDuringHostWindowCreation(node.Windows[0]);

    node.Windows.push_back(window);
    node.WantHiddenTabBarUpdate = true;
    window.dock_node = node;
    window.DockId = node.ID;
    window.dock_is_active = (node.Windows.size > 1);
    window.DockTabWantClose = false;

    // When reactivating a node with one or two loose window, the window pos/size/viewport are authoritative over the node storage.
    // In particular it is important we init the viewport from the first window so we don't create two viewports and drop one.
    if (node.host_window == NULL && node.IsFloatingNode())
    {
        if (node.AuthorityForPos == ImGuiDataAuthority_Auto)
            node.AuthorityForPos = ImGuiDataAuthority_Window;
        if (node.AuthorityForSize == ImGuiDataAuthority_Auto)
            node.AuthorityForSize = ImGuiDataAuthority_Window;
        if (node.AuthorityForViewport == ImGuiDataAuthority_Auto)
            node.AuthorityForViewport = ImGuiDataAuthority_Window;
    }

    // Add to tab bar if requested
    if (add_to_tab_bar)
    {
        if (node.TabBar == NULL)
        {
            DockNodeAddTabBar(node);
            node.TabBar.SelectedTabId = node.TabBar.NextSelectedTabId = node.SelectedTabId;

            // Add existing windows
            for (int n = 0; n < node.Windows.size - 1; n += 1)
                TabBarAddTab(node.TabBar, ImGuiTabItemFlags_None, node.Windows[n]);
        }
        TabBarAddTab(node.TabBar, ImGuiTabItemFlags_Unsorted, window);
    }

    DockNodeUpdateVisibleFlag(node);

    // Update this without waiting for the next time we Begin() in the window, so our host window will have the proper title bar color on its first frame.
    if (node.host_window)
        UpdateWindowParentAndRootLinks(window, window.flags | WindowFlags::ChildWindow, node.host_window);
}

static void DockNodeRemoveWindow(ImGuiDockNode* node, ImGuiWindow* window, ImGuiID save_dock_id)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(window.dock_node == node);
    //IM_ASSERT(window->root_window_dock_tree == node->host_window);
    //IM_ASSERT(window->last_frame_active < g.frame_count);    // We may call this from Begin()
    IM_ASSERT(save_dock_id == 0 || save_dock_id == node.ID);
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeRemoveWindow node 0x%08X window '%s'\n", node.ID, window.Name);

    window.dock_node = NULL;
    window.dock_is_active = window.DockTabWantClose = false;
    window.DockId = save_dock_id;
    window.flags &= ~WindowFlags::ChildWindow;
    if (window.parent_window)
        window.parent_window.DC.ChildWindows.find_erase(window);
    UpdateWindowParentAndRootLinks(window, window.flags, NULL); // Update immediately

    // Remove window
    bool erased = false;
    for (int n = 0; n < node.Windows.size; n += 1)
        if (node.Windows[n] == window)
        {
            node.Windows.erase(node.Windows.data + n);
            erased = true;
            break;
        }
    if (!erased)
        IM_ASSERT(erased);
    if (node.VisibleWindow == window)
        node.VisibleWindow = NULL;

    // Remove tab and possibly tab bar
    node.WantHiddenTabBarUpdate = true;
    if (node.TabBar)
    {
        TabBarRemoveTab(node.TabBar, window.TabId);
        const int tab_count_threshold_for_tab_bar = node.IsCentralNode() ? 1 : 2;
        if (node.Windows.size < tab_count_threshold_for_tab_bar)
            DockNodeRemoveTabBar(node);
    }

    if (node.Windows.size == 0 && !node.IsCentralNode() && !node.IsDockSpace() && window.DockId != node.ID)
    {
        // Automatic dock node delete themselves if they are not holding at least one tab
        DockContextRemoveNode(&g, node, true);
        return;
    }

    if (node.Windows.size == 1 && !node.IsCentralNode() && node.host_window)
    {
        ImGuiWindow* remaining_window = node.Windows[0];
        if (node.host_window.ViewportOwned && node.IsRootNode())
        {
            // Transfer viewport back to the remaining loose window
            IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Node %08X transfer viewport %08X=>%08X for window '%s'\n", node.ID, node.host_window.Viewport.ID, remaining_window.id, remaining_window.Name);
            IM_ASSERT(node.host_window.Viewport.Window == node.host_window);
            node.host_window.Viewport.Window = remaining_window;
            node.host_window.Viewport.ID = remaining_window.id;
        }
        remaining_window.collapsed = node.host_window.collapsed;
    }

    // Update visibility immediately is required so the DockNodeUpdateRemoveInactiveChilds() processing can reflect changes up the tree
    DockNodeUpdateVisibleFlag(node);
}

static void DockNodeMoveChildNodes(ImGuiDockNode* dst_node, ImGuiDockNode* src_node)
{
    IM_ASSERT(dst_node.Windows.size == 0);
    dst_node.ChildNodes[0] = src_node.ChildNodes[0];
    dst_node.ChildNodes[1] = src_node.ChildNodes[1];
    if (dst_node.ChildNodes[0])
        dst_node.ChildNodes[0].ParentNode = dst_node;
    if (dst_node.ChildNodes[1])
        dst_node.ChildNodes[1].ParentNode = dst_node;
    dst_node.SplitAxis = src_node.SplitAxis;
    dst_node.sizeRef = src_node.sizeRef;
    src_node.ChildNodes[0] = src_node.ChildNodes[1] = NULL;
}

static void DockNodeMoveWindows(ImGuiDockNode* dst_node, ImGuiDockNode* src_node)
{
    // Insert tabs in the same orders as currently ordered (node->windows isn't ordered)
    IM_ASSERT(src_node && dst_node && dst_node != src_node);
    ImGuiTabBar* src_tab_bar = src_node.TabBar;
    if (src_tab_bar != NULL)
        IM_ASSERT(src_node.Windows.size <= src_node.TabBar.Tabs.size);

    // If the dst_node is empty we can just move the entire tab bar (to preserve selection, scrolling, etc.)
    bool move_tab_bar = (src_tab_bar != NULL) && (dst_node.TabBar == NULL);
    if (move_tab_bar)
    {
        dst_node.TabBar = src_node.TabBar;
        src_node.TabBar = NULL;
    }

    for (int n = 0; n < src_node.Windows.size; n += 1)
    {
        // dock_node's tab_bar may have non-window Tabs manually appended by user
        if (ImGuiWindow* window = src_tab_bar ? src_tab_bar.Tabs[n].Window : src_node.Windows[n])
        {
            window.dock_node = NULL;
            window.dock_is_active = false;
            DockNodeAddWindow(dst_node, window, move_tab_bar ? false : true);
        }
    }
    src_node.Windows.clear();

    if (!move_tab_bar && src_node.TabBar)
    {
        if (dst_node.TabBar)
            dst_node.TabBar.SelectedTabId = src_node.TabBar.SelectedTabId;
        DockNodeRemoveTabBar(src_node);
    }
}

static void DockNodeApplyPosSizeToWindows(ImGuiDockNode* node)
{
    for (int n = 0; n < node.Windows.size; n += 1)
    {
        set_window_pos(node.Windows[n], node.pos, Cond::Always); // We don't assign directly to pos because it can break the calculation of SizeContents on next frame
        SetWindowSize(node.Windows[n], node.size, Cond::Always);
    }
}

static void DockNodeHideHostWindow(ImGuiDockNode* node)
{
    if (node.host_window)
    {
        if (node.host_window.DockNodeAsHost == node)
            node.host_window.DockNodeAsHost = NULL;
        node.host_window = NULL;
    }

    if (node.Windows.size == 1)
    {
        node.VisibleWindow = node.Windows[0];
        node.Windows[0].dock_is_active = false;
    }

    if (node.TabBar)
        DockNodeRemoveTabBar(node);
}

// Search function called once by root node in DockNodeUpdate()
struct ImGuiDockNodeTreeInfo
{
    ImGuiDockNode*      CentralNode;
    ImGuiDockNode*      FirstNodeWithWindows;
    int                 CountNodesWithWindows;
    //ImGuiWindowClass  WindowClassForMerges;

    ImGuiDockNodeTreeInfo() { memset(this, 0, sizeof(*this)); }
};

static void DockNodeFindInfo(ImGuiDockNode* node, ImGuiDockNodeTreeInfo* info)
{
    if (node.Windows.size > 0)
    {
        if (info.FirstNodeWithWindows == NULL)
            info.FirstNodeWithWindows = node;
        info.CountNodesWithWindows += 1;
    }
    if (node.IsCentralNode())
    {
        IM_ASSERT(info.CentralNode == NULL); // Should be only one
        IM_ASSERT(node.IsLeafNode() && "If you get this assert: please submit .ini file + repro of actions leading to this.");
        info.CentralNode = node;
    }
    if (info.CountNodesWithWindows > 1 && info.CentralNode != NULL)
        return;
    if (node.ChildNodes[0])
        DockNodeFindInfo(node.ChildNodes[0], info);
    if (node.ChildNodes[1])
        DockNodeFindInfo(node.ChildNodes[1], info);
}

static ImGuiWindow* DockNodeFindWindowByID(ImGuiDockNode* node, ImGuiID id)
{
    IM_ASSERT(id != 0);
    for (int n = 0; n < node.Windows.size; n += 1)
        if (node.Windows[n].ID == id)
            return node.Windows[n];
    return NULL;
}

// - Remove inactive windows/nodes.
// - Update visibility flag.
static void DockNodeUpdateFlagsAndCollapse(ImGuiDockNode* node)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(node.ParentNode == NULL || node.ParentNode.ChildNodes[0] == node || node.ParentNode.ChildNodes[1] == node);

    // Inherit most flags
    if (node.ParentNode)
        node.SharedFlags = node.ParentNode.SharedFlags & ImGuiDockNodeFlags_SharedFlagsInheritMask_;

    // Recurse into children
    // There is the possibility that one of our child becoming empty will delete itself and moving its sibling contents into 'node'.
    // If 'node->ChildNode[0]' delete itself, then 'node->ChildNode[1]->windows' will be moved into 'node'
    // If 'node->ChildNode[1]' delete itself, then 'node->ChildNode[0]->windows' will be moved into 'node' and the "remove inactive windows" loop will have run twice on those windows (harmless)
    node.HasCentralNodeChild = false;
    if (node.ChildNodes[0])
        DockNodeUpdateFlagsAndCollapse(node.ChildNodes[0]);
    if (node.ChildNodes[1])
        DockNodeUpdateFlagsAndCollapse(node.ChildNodes[1]);

    // Remove inactive windows, collapse nodes
    // merge node flags overrides stored in windows
    node.LocalFlagsInWindows = ImGuiDockNodeFlags_None;
    for (int window_n = 0; window_n < node.Windows.size; window_n += 1)
    {
        ImGuiWindow* window = node.Windows[window_n];
        IM_ASSERT(window.dock_node == node);

        bool node_was_active = (node.LastFrameActive + 1 == g.frame_count);
        bool remove = false;
        remove |= node_was_active && (window.LastFrameActive + 1 < g.frame_count);
        remove |= node_was_active && (node.WantCloseAll || node.WantCloseTabId == window.TabId) && window.HasCloseButton && !(window.flags & WindowFlags::UnsavedDocument);  // Submit all _expected_ closure from last frame
        remove |= (window.DockTabWantClose);
        if (remove)
        {
            window.DockTabWantClose = false;
            if (node.Windows.size == 1 && !node.IsCentralNode())
            {
                DockNodeHideHostWindow(node);
                node.State = ImGuiDockNodeState_HostWindowHiddenBecauseSingleWindow;
                DockNodeRemoveWindow(node, window, node.ID); // Will delete the node so it'll be invalid on return
                return;
            }
            DockNodeRemoveWindow(node, window, node.ID);
            window_n--;
            continue;
        }

        // FIXME-DOCKING: Missing policies for conflict resolution, hence the "Experimental" tag on this.
        //node->LocalFlagsInWindow &= ~window->window_class.DockNodeFlagsOverrideClear;
        node.LocalFlagsInWindows |= window.WindowClass.DockNodeFlagsOverrideSet;
    }
    node.UpdateMergedFlags();

    // Auto-hide tab bar option
    ImGuiDockNodeFlags node_flags = node.MergedFlags;
    if (node.WantHiddenTabBarUpdate && node.Windows.size == 1 && (node_flags & ImGuiDockNodeFlags_AutoHideTabBar) && !node.is_hidden_tab_bar())
        node.want_hidden_tab_bar_toggle = true;
    node.WantHiddenTabBarUpdate = false;

    // Cancel toggling if we know our tab bar is enforced to be hidden at all times
    if (node.want_hidden_tab_bar_toggle && node.VisibleWindow && (node.VisibleWindow.WindowClass.DockNodeFlagsOverrideSet & ImGuiDockNodeFlags_HiddenTabBar))
        node.want_hidden_tab_bar_toggle = false;

    // Apply toggles at a single point of the frame (here!)
    if (node.Windows.size > 1)
        node.SetLocalFlags(node.LocalFlags & ~ImGuiDockNodeFlags_HiddenTabBar);
    else if (node.want_hidden_tab_bar_toggle)
        node.SetLocalFlags(node.LocalFlags ^ ImGuiDockNodeFlags_HiddenTabBar);
    node.want_hidden_tab_bar_toggle = false;

    DockNodeUpdateVisibleFlag(node);
}

// This is rarely called as DockNodeUpdateForRootNode() generally does it most frames.
static void DockNodeUpdateHasCentralNodeChild(ImGuiDockNode* node)
{
    node.HasCentralNodeChild = false;
    if (node.ChildNodes[0])
        DockNodeUpdateHasCentralNodeChild(node.ChildNodes[0]);
    if (node.ChildNodes[1])
        DockNodeUpdateHasCentralNodeChild(node.ChildNodes[1]);
    if (node.IsRootNode())
    {
        ImGuiDockNode* mark_node = node.CentralNode;
        while (mark_node)
        {
            mark_node.HasCentralNodeChild = true;
            mark_node = mark_node.ParentNode;
        }
    }
}

static void DockNodeUpdateVisibleFlag(ImGuiDockNode* node)
{
    // Update visibility flag
    bool is_visible = (node.ParentNode == NULL) ? node.IsDockSpace() : node.IsCentralNode();
    is_visible |= (node.Windows.size > 0);
    is_visible |= (node.ChildNodes[0] && node.ChildNodes[0].IsVisible);
    is_visible |= (node.ChildNodes[1] && node.ChildNodes[1].IsVisible);
    node.IsVisible = is_visible;
}

static void DockNodeStartMouseMovingWindow(ImGuiDockNode* node, ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(node.WantMouseMove == true);
    start_mouse_moving_window(window);
    g.ActiveIdClickOffset = g.io.mouse_clicked_pos[0] - node.pos;
    g.moving_window = window; // If we are docked into a non moveable root window, start_mouse_moving_window() won't set g.moving_window. Override that decision.
    node.WantMouseMove = false;
}

// Update central_node, OnlyNodeWithWindows, LastFocusedNodeID. Copy window class.
static void DockNodeUpdateForRootNode(ImGuiDockNode* node)
{
    DockNodeUpdateFlagsAndCollapse(node);

    // - Setup central node pointers
    // - Find if there's only a single visible window in the hierarchy (in which case we need to display a regular title bar -> FIXME-DOCK: that last part is not done yet!)
    // Cannot merge this with DockNodeUpdateFlagsAndCollapse() because FirstNodeWithWindows is found after window removal and child collapsing
    ImGuiDockNodeTreeInfo info;
    DockNodeFindInfo(node, &info);
    node.CentralNode = info.CentralNode;
    node.OnlyNodeWithWindows = (info.CountNodesWithWindows == 1) ? info.FirstNodeWithWindows : NULL;
    node.CountNodeWithWindows = info.CountNodesWithWindows;
    if (node.LastFocusedNodeId == 0 && info.FirstNodeWithWindows != NULL)
        node.LastFocusedNodeId = info.FirstNodeWithWindows.ID;

    // Copy the window class from of our first window so it can be used for proper dock filtering.
    // When node has mixed windows, prioritize the class with the most constraint (docking_allow_unclassed = false) as the reference to copy.
    // FIXME-DOCK: We don't recurse properly, this code could be reworked to work from DockNodeUpdateScanRec.
    if (ImGuiDockNode* first_node_with_windows = info.FirstNodeWithWindows)
    {
        node.WindowClass = first_node_with_windows.Windows[0].WindowClass;
        for (int n = 1; n < first_node_with_windows.Windows.size; n += 1)
            if (first_node_with_windows.Windows[n].WindowClass.DockingAllowUnclassed == false)
            {
                node.WindowClass = first_node_with_windows.Windows[n].WindowClass;
                break;
            }
    }

    ImGuiDockNode* mark_node = node.CentralNode;
    while (mark_node)
    {
        mark_node.HasCentralNodeChild = true;
        mark_node = mark_node.ParentNode;
    }
}

static void DockNodeSetupHostWindow(ImGuiDockNode* node, ImGuiWindow* host_window)
{
    // Remove ourselves from any previous different host window
    // This can happen if a user mistakenly does (see #4295 for details):
    //  - N+0: DockBuilderAddNode(id, 0)    // missing ImGuiDockNodeFlags_DockSpace
    //  - N+1: NewFrame()                   // will create floating host window for that node
    //  - N+1: DockSpace(id)                // requalify node as dockspace, moving host window
    if (node.host_window && node.host_window != host_window && node.host_window.DockNodeAsHost == node)
        node.host_window.DockNodeAsHost = NULL;

    host_window.DockNodeAsHost = node;
    node.host_window = host_window;
}

static void DockNodeUpdate(ImGuiDockNode* node)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(node.LastFrameActive != g.frame_count);
    node.LastFrameAlive = g.frame_count;
    node.is_bg_drawn_this_frame = false;

    node.CentralNode = node.OnlyNodeWithWindows = NULL;
    if (node.IsRootNode())
        DockNodeUpdateForRootNode(node);

    // Remove tab bar if not needed
    if (node.TabBar && node.is_no_tab_bar())
        DockNodeRemoveTabBar(node);

    // Early out for hidden root dock nodes (when all dock_id references are in inactive windows, or there is only 1 floating window holding on the dock_id)
    bool want_to_hide_host_window = false;
    if (node.IsFloatingNode())
    {
        if (node.Windows.size <= 1 && node.IsLeafNode())
            if (!g.io.ConfigDockingAlwaysTabBar && (node.Windows.size == 0 || !node.Windows[0].WindowClass.DockingAlwaysTabBar))
                want_to_hide_host_window = true;
        if (node.CountNodeWithWindows == 0)
            want_to_hide_host_window = true;
    }
    if (want_to_hide_host_window)
    {
        if (node.Windows.size == 1)
        {
            // Floating window pos/size is authoritative
            ImGuiWindow* single_window = node.Windows[0];
            node.pos = single_window.Pos;
            node.size = single_window.sizeFull;
            node.AuthorityForPos = node.AuthorityForSize = node.AuthorityForViewport = ImGuiDataAuthority_Window;

            // Transfer focus immediately so when we revert to a regular window it is immediately selected
            if (node.host_window && g.nav_window == node.host_window)
                focus_window(single_window);
            if (node.host_window)
            {
                single_window.viewport = node.host_window.Viewport;
                single_window.viewport_id = node.host_window.viewport_id;
                if (node.host_window.ViewportOwned)
                {
                    single_window.viewport.Window = single_window;
                    single_window.viewport_owned = true;
                }
            }
        }

        DockNodeHideHostWindow(node);
        node.State = ImGuiDockNodeState_HostWindowHiddenBecauseSingleWindow;
        node.WantCloseAll = false;
        node.WantCloseTabId = 0;
        node.HasCloseButton = node.HasWindowMenuButton = false;
        node.LastFrameActive = g.frame_count;

        if (node.WantMouseMove && node.Windows.size == 1)
            DockNodeStartMouseMovingWindow(node, node.Windows[0]);
        return;
    }

    // In some circumstance we will defer creating the host window (so everything will be kept hidden),
    // while the expected visible window is resizing itself.
    // This is important for first-time (no ini settings restored) single window when io.config_docking_always_tab_bar is enabled,
    // otherwise the node ends up using the minimum window size. Effectively those windows will take an extra frame to show up:
    //   N+0: Begin(): window created (with no known size), node is created
    //   N+1: DockNodeUpdate(): node skip creating host window / Begin(): window size applied, not visible
    //   N+2: DockNodeUpdate(): node can create host window / Begin(): window becomes visible
    // We could remove this frame if we could reliably calculate the expected window size during node update, before the Begin() code.
    // It would require a generalization of CalcWindowExpectedSize(), probably extracting code away from Begin().
    // In reality it isn't very important as user quickly ends up with size data in .ini file.
    if (node.IsVisible && node.host_window == NULL && node.IsFloatingNode() && node.IsLeafNode())
    {
        IM_ASSERT(node.Windows.size > 0);
        ImGuiWindow* ref_window = NULL;
        if (node.SelectedTabId != 0) // Note that we prune single-window-node settings on .ini loading, so this is generally 0 for them!
            ref_window = DockNodeFindWindowByID(node, node.SelectedTabId);
        if (ref_window == NULL)
            ref_window = node.Windows[0];
        if (ref_window.auto_fit_frames_x > 0 || ref_window.auto_fit_frames_y > 0)
        {
            node.State = ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing;
            return;
        }
    }

    const ImGuiDockNodeFlags node_flags = node.MergedFlags;

    // Decide if the node will have a close button and a window menu button
    node.HasWindowMenuButton = (node.Windows.size > 0) && (node_flags & ImGuiDockNodeFlags_NoWindowMenuButton) == 0;
    node.HasCloseButton = false;
    for (int window_n = 0; window_n < node.Windows.size; window_n += 1)
    {
        // FIXME-DOCK: Setting dock_is_active here means that for single active window in a leaf node, dock_is_active will be cleared until the next Begin() call.
        ImGuiWindow* window = node.Windows[window_n];
        node.HasCloseButton |= window.HasCloseButton;
        window.dock_is_active = (node.Windows.size > 1);
    }
    if (node_flags & ImGuiDockNodeFlags_NoCloseButton)
        node.HasCloseButton = false;

    // Bind or create host window
    ImGuiWindow* host_window = NULL;
    bool beginned_into_host_window = false;
    if (node.IsDockSpace())
    {
        // [Explicit root dockspace node]
        IM_ASSERT(node.host_window);
        host_window = node.host_window;
    }
    else
    {
        // [Automatic root or child nodes]
        if (node.IsRootNode() && node.IsVisible)
        {
            ImGuiWindow* ref_window = (node.Windows.size > 0) ? node.Windows[0] : NULL;

            // Sync pos
            if (node.AuthorityForPos == ImGuiDataAuthority_Window && ref_window)
                SetNextWindowPos(ref_window.Pos);
            else if (node.AuthorityForPos == ImGuiDataAuthority_DockNode)
                SetNextWindowPos(node.pos);

            // Sync size
            if (node.AuthorityForSize == ImGuiDataAuthority_Window && ref_window)
                set_next_window_size(ref_window.sizeFull);
            else if (node.AuthorityForSize == ImGuiDataAuthority_DockNode)
                set_next_window_size(node.size);

            // Sync collapsed
            if (node.AuthorityForSize == ImGuiDataAuthority_Window && ref_window)
                SetNextWindowCollapsed(ref_window.collapsed);

            // Sync viewport
            if (node.AuthorityForViewport == ImGuiDataAuthority_Window && ref_window)
                SetNextWindowViewport(ref_window.viewport_id);

            SetNextWindowClass(&node.WindowClass);

            // Begin into the host window
            char window_label[20];
            DockNodeGetHostWindowTitle(node, window_label, IM_ARRAYSIZE(window_label));
            ImGuiWindowFlags window_flags = WindowFlags::NoScrollbar | WindowFlags::NoScrollWithMouse | WindowFlags::DockNodeHost;
            window_flags |= WindowFlags::NoFocusOnAppearing;
            window_flags |= WindowFlags::NoSavedSettings | WindowFlags::NoNavFocus | WindowFlags::NoCollapse;
            window_flags |= WindowFlags::NoTitleBar;

            SetNextWindowBgAlpha(0.0); // Don't set ImGuiWindowFlags_NoBackground because it disables borders
            push_style_var(StyleVar::WindowPadding, Vector2D::new(0, 0));
            begin(window_label, NULL, window_flags);
            pop_style_var();
            beginned_into_host_window = true;

            host_window = g.current_window;
            DockNodeSetupHostWindow(node, host_window);
            host_window.dc.cursor_pos = host_window.Pos;
            node.pos = host_window.Pos;
            node.size = host_window.size;

            // We set ImGuiWindowFlags_NoFocusOnAppearing because we don't want the host window to take full focus (e.g. steal nav_window)
            // But we still it bring it to the front of display. There's no way to choose this precise behavior via window flags.
            // One simple case to ponder if: window A has a toggle to create windows B/C/D. Dock B/C/D together, clear the toggle and enable it again.
            // When reappearing B/C/D will request focus and be moved to the top of the display pile, but they are not linked to the dock host window
            // during the frame they appear. The dock host window would keep its old display order, and the sorting in EndFrame would move B/C/D back
            // after the dock host window, losing their top-most status.
            if (node.host_window.appearing)
                BringWindowToDisplayFront(node.host_window);

            node.AuthorityForPos = node.AuthorityForSize = node.AuthorityForViewport = ImGuiDataAuthority_Auto;
        }
        else if (node.ParentNode)
        {
            node.host_window = host_window = node.ParentNode.host_window;
            node.AuthorityForPos = node.AuthorityForSize = node.AuthorityForViewport = ImGuiDataAuthority_Auto;
        }
        if (node.WantMouseMove && node.host_window)
            DockNodeStartMouseMovingWindow(node, node.host_window);
    }

    // Update focused node (the one whose title bar is highlight) within a node tree
    if (node.IsSplitNode())
        IM_ASSERT(node.TabBar == NULL);
    if (node.IsRootNode())
        if (g.nav_window && g.nav_window.root_window.dock_node && g.nav_window.root_window.parent_window == host_window)
            node.LastFocusedNodeId = g.nav_window.root_window.dock_node.ID;

    // Register a hit-test hole in the window unless we are currently dragging a window that is compatible with our dockspace
    ImGuiDockNode* central_node = node.CentralNode;
    const bool central_node_hole = node.IsRootNode() && host_window && (node_flags & ImGuiDockNodeFlags_PassthruCentralNode) != 0 && central_node != NULL && central_node.IsEmpty();
    bool central_node_hole_register_hit_test_hole = central_node_hole;
    if (central_node_hole)
        if (const ImGuiPayload* payload = GetDragDropPayload())
            if (payload.IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) && DockNodeIsDropAllowed(host_window, *(ImGuiWindow**)payload.Data))
                central_node_hole_register_hit_test_hole = false;
    if (central_node_hole_register_hit_test_hole)
    {
        // We add a little padding to match the "resize from edges" behavior and allow grabbing the splitter easily.
        // (But we only add it if there's something else on the other side of the hole, otherwise for e.g. fullscreen
        // covering passthru node we'd have a gap on the edge not covered by the hole)
        IM_ASSERT(node.IsDockSpace()); // We cannot pass this flag without the DockSpace() api. Testing this because we also setup the hole in host_window->parent_node
        ImGuiDockNode* root_node = DockNodeGetRootNode(central_node);
        Rect root_rect(root_node.pos, root_node.pos + root_node.size);
        Rect hole_rect(central_node.pos, central_node.pos + central_node.size);
        if (hole_rect.min.x > root_rect.min.x) { hole_rect.min.x += WINDOWS_HOVER_PADDING; }
        if (hole_rect.max.x < root_rect.max.x) { hole_rect.max.x -= WINDOWS_HOVER_PADDING; }
        if (hole_rect.min.y > root_rect.min.y) { hole_rect.min.y += WINDOWS_HOVER_PADDING; }
        if (hole_rect.max.y < root_rect.max.y) { hole_rect.max.y -= WINDOWS_HOVER_PADDING; }
        //GetForegroundDrawList()->add_rect(hole_rect.min, hole_rect.max, IM_COL32(255, 0, 0, 255));
        if (central_node_hole && !hole_rect.IsInverted())
        {
            SetWindowHitTestHole(host_window, hole_rect.min, hole_rect.max - hole_rect.min);
            if (host_window.parent_window)
                SetWindowHitTestHole(host_window.parent_window, hole_rect.min, hole_rect.max - hole_rect.min);
        }
    }

    // Update position/size, process and draw resizing splitters
    if (node.IsRootNode() && host_window)
    {
        host_window.draw_list.channels_set_current(1);
        DockNodeTreeUpdatePosSize(node, host_window.Pos, host_window.size);
        DockNodeTreeUpdateSplitter(node);
    }

    // Draw empty node background (currently can only be the Central Node)
    if (host_window && node.IsEmpty() && node.IsVisible)
    {
        host_window.draw_list.channels_set_current(0);
        node.last_bg_color = (node_flags & ImGuiDockNodeFlags_PassthruCentralNode) ? 0 : get_color_u32(StyleColor::DockingEmptyBg);
        if (node.last_bg_color != 0)
            host_window.draw_list.add_rect_filled(node.pos, node.pos + node.size, node.last_bg_color);
        node.is_bg_drawn_this_frame = true;
    }

    // Draw whole dockspace background if ImGuiDockNodeFlags_PassthruCentralNode if set.
    // We need to draw a background at the root level if requested by ImGuiDockNodeFlags_PassthruCentralNode, but we will only know the correct pos/size
    // _after_ processing the resizing splitters. So we are using the draw_list channel splitting facility to submit drawing primitives out of order!
    const bool render_dockspace_bg = node.IsRootNode() && host_window && (node_flags & ImGuiDockNodeFlags_PassthruCentralNode) != 0;
    if (render_dockspace_bg && node.IsVisible)
    {
        host_window.draw_list.channels_set_current(0);
        if (central_node_hole)
            render_rect_filled_with_hole(host_window.draw_list, node.rect(), central_node.rect(), get_color_u32(StyleColor::WindowBg), 0.0);
        else
            host_window.draw_list.add_rect_filled(node.pos, node.pos + node.size, get_color_u32(StyleColor::WindowBg), 0.0);
    }

    // Draw and populate Tab Bar
    if (host_window)
        host_window.draw_list.channels_set_current(1);
    if (host_window && node.Windows.size > 0)
    {
        DockNodeUpdateTabBar(node, host_window);
    }
    else
    {
        node.WantCloseAll = false;
        node.WantCloseTabId = 0;
        node.IsFocused = false;
    }
    if (node.TabBar && node.TabBar.SelectedTabId)
        node.SelectedTabId = node.TabBar.SelectedTabId;
    else if (node.Windows.size > 0)
        node.SelectedTabId = node.Windows[0].ID;

    // Draw payload drop target
    if (host_window && node.IsVisible)
        if (node.IsRootNode() && (g.moving_window == NULL || g.moving_window.root_window_dock_tree != host_window))
            BeginDockableDragDropTarget(host_window);

    // We update this after DockNodeUpdateTabBar()
    node.LastFrameActive = g.frame_count;

    // Recurse into children
    // FIXME-DOCK FIXME-OPT: Should not need to recurse into children
    if (host_window)
    {
        if (node.ChildNodes[0])
            DockNodeUpdate(node.ChildNodes[0]);
        if (node.ChildNodes[1])
            DockNodeUpdate(node.ChildNodes[1]);

        // Render outer borders last (after the tab bar)
        if (node.IsRootNode())
        {
            host_window.draw_list.channels_set_current(1);
            RenderWindowOuterBorders(host_window);
        }

        // Further rendering (= hosted windows background) will be drawn on layer 0
        host_window.draw_list.channels_set_current(0);
    }

    // End host window
    if (beginned_into_host_window) //-V1020
        end();
}

// Compare TabItem nodes given the last known dock_order (will persist in .ini file as hint), used to sort tabs when multiple tabs are added on the same frame.
static int IMGUI_CDECL TabItemComparerByDockOrder(const void* lhs, const void* rhs)
{
    ImGuiWindow* a = ((const ImGuiTabItem*)lhs).Window;
    ImGuiWindow* b = ((const ImGuiTabItem*)rhs).Window;
    if (int d = ((a.dock_order == -1) ? INT_MAX : a.dock_order) - ((b.dock_order == -1) ? INT_MAX : b.dock_order))
        return d;
    return (a.BeginOrderWithinContext - b.BeginOrderWithinContext);
}

static ImGuiID DockNodeUpdateWindowMenu(ImGuiDockNode* node, ImGuiTabBar* tab_bar)
{
    // Try to position the menu so it is more likely to stays within the same viewport
    ImGuiContext& g = *GImGui;
    ImGuiID ret_tab_id = 0;
    if (g.style.window_menu_button_position == Dir::Left)
        SetNextWindowPos(Vector2D::new(node.pos.x, node.pos.y + GetFrameHeight()), Cond::Always, Vector2D::new(0.0, 0.0));
    else
        SetNextWindowPos(Vector2D::new(node.pos.x + node.size.x, node.pos.y + GetFrameHeight()), Cond::Always, Vector2D::new(1.0, 0.0));
    if (BeginPopup("#WindowMenu"))
    {
        node.IsFocused = true;
        if (tab_bar.Tabs.size == 1)
        {
            if (MenuItem("Hide tab bar", NULL, node.is_hidden_tab_bar()))
                node.want_hidden_tab_bar_toggle = true;
        }
        else
        {
            for (int tab_n = 0; tab_n < tab_bar.Tabs.size; tab_n += 1)
            {
                ImGuiTabItem* tab = &tab_bar.Tabs[tab_n];
                if (tab.flags & ImGuiTabItemFlags_Button)
                    continue;
                if (Selectable(tab_bar.GetTabName(tab), tab.ID == tab_bar.SelectedTabId))
                    ret_tab_id = tab.ID;
                SameLine();
                Text("   ");
            }
        }
        EndPopup();
    }
    return ret_tab_id;
}

// User helper to append/amend into a dock node tab bar. Most commonly used to add e.g. a "+" button.
bool DockNodeBeginAmendTabBar(ImGuiDockNode* node)
{
    if (node.TabBar == NULL || node.host_window == NULL)
        return false;
    if (node.MergedFlags & ImGuiDockNodeFlags_KeepAliveOnly)
        return false;
    begin(node.host_window.Name);
    PushOverrideID(node.ID);
    bool ret = BeginTabBarEx(node.TabBar, node.TabBar.BarRect, node.TabBar.flags, node);
    IM_UNUSED(ret);
    IM_ASSERT(ret);
    return true;
}

void DockNodeEndAmendTabBar()
{
    EndTabBar();
    PopID();
    end();
}

static bool IsDockNodeTitleBarHighlighted(ImGuiDockNode* node, ImGuiDockNode* root_node, ImGuiWindow* host_window)
{
    // CTRL+Tab highlight (only highlighting leaf node, not whole hierarchy)
    ImGuiContext& g = *GImGui;
    if (g.nav_windowing_target)
        return (g.nav_windowing_target.dock_node == node);

    // FIXME-DOCKING: May want alternative to treat central node void differently? e.g. if (g.nav_window == host_window)
    if (g.nav_window && g.nav_window.root_window_for_title_bar_highlight == host_window.root_window_dock_tree && root_node.LastFocusedNodeId == node.ID)
        for (ImGuiDockNode* parent_node = g.nav_window.root_window.dock_node; parent_node != NULL; parent_node = parent_node.host_window ? parent_node.host_window.root_window.dock_node : NULL)
            if ((parent_node = DockNodeGetRootNode(parent_node)) == root_node)
                return true;
    return false;
}

// Submit the tab bar corresponding to a dock node and various housekeeping details.
static void DockNodeUpdateTabBar(ImGuiDockNode* node, ImGuiWindow* host_window)
{
    ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.style;

    const bool node_was_active = (node.LastFrameActive + 1 == g.frame_count);
    const bool closed_all = node.WantCloseAll && node_was_active;
    const ImGuiID closed_one = node.WantCloseTabId && node_was_active;
    node.WantCloseAll = false;
    node.WantCloseTabId = 0;

    // Decide if we should use a focused title bar color
    bool is_focused = false;
    ImGuiDockNode* root_node = DockNodeGetRootNode(node);
    if (IsDockNodeTitleBarHighlighted(node, root_node, host_window))
        is_focused = true;

    // hidden tab bar will show a triangle on the upper-left (in Begin)
    if (node.is_hidden_tab_bar() || node.is_no_tab_bar())
    {
        node.VisibleWindow = (node.Windows.size > 0) ? node.Windows[0] : NULL;
        node.IsFocused = is_focused;
        if (is_focused)
            node.LastFrameFocused = g.frame_count;
        if (node.VisibleWindow)
        {
            // Notify root of visible window (used to display title in OS task bar)
            if (is_focused || root_node.VisibleWindow == NULL)
                root_node.VisibleWindow = node.VisibleWindow;
            if (node.TabBar)
                node.TabBar.VisibleTabId = node.VisibleWindow.TabId;
        }
        return;
    }

    // Move ourselves to the Menu layer (so we can be accessed by tapping Alt) + undo skip_items flag in order to draw over the title bar even if the window is collapsed
    bool backup_skip_item = host_window.skip_items;
    if (!node.IsDockSpace())
    {
        host_window.skip_items = false;
        host_window.dcnav_layer_current = NavLayer::Menu;
    }

    // Use PushOverrideID() instead of PushID() to use the node id _without_ the host window id.
    // This is to facilitate computing those id from the outside, and will affect more or less only the id of the collapse button, popup and tabs,
    // as docked windows themselves will override the stack with their own root id.
    PushOverrideID(node.ID);
    ImGuiTabBar* tab_bar = node.TabBar;
    bool tab_bar_is_recreated = (tab_bar == NULL); // Tab bar are automatically destroyed when a node gets hidden
    if (tab_bar == NULL)
    {
        DockNodeAddTabBar(node);
        tab_bar = node.TabBar;
    }

    ImGuiID focus_tab_id = 0;
    node.IsFocused = is_focused;

    const ImGuiDockNodeFlags node_flags = node.MergedFlags;
    const bool has_window_menu_button = (node_flags & ImGuiDockNodeFlags_NoWindowMenuButton) == 0 && (style.window_menu_button_position != Dir::None);

    // In a dock node, the Collapse Button turns into the window Menu button.
    // FIXME-DOCK FIXME-OPT: Could we recycle popups id across multiple dock nodes?
    if (has_window_menu_button && IsPopupOpen("#WindowMenu"))
    {
        if (ImGuiID tab_id = DockNodeUpdateWindowMenu(node, tab_bar))
            focus_tab_id = tab_bar.NextSelectedTabId = tab_id;
        is_focused |= node.IsFocused;
    }

    // Layout
    Rect title_bar_rect, tab_bar_rect;
    Vector2D window_menu_button_pos;
    Vector2D close_button_pos;
    DockNodeCalcTabBarLayout(node, &title_bar_rect, &tab_bar_rect, &window_menu_button_pos, &close_button_pos);

    // Submit new tabs, they will be added as Unsorted and sorted below based on relative dock_order value.
    const int tabs_count_old = tab_bar.Tabs.size;
    for (int window_n = 0; window_n < node.Windows.size; window_n += 1)
    {
        ImGuiWindow* window = node.Windows[window_n];
        if (TabBarFindTabByID(tab_bar, window.TabId) == NULL)
            TabBarAddTab(tab_bar, ImGuiTabItemFlags_Unsorted, window);
    }

    // Title bar
    if (is_focused)
        node.LastFrameFocused = g.frame_count;
    ImU32 title_bar_col = get_color_u32(host_window.collapsed ? StyleColor::TitleBgCollapsed : is_focused ? StyleColor::TitleBgActive : StyleColor::TitleBg);
    ImDrawFlags rounding_flags = CalcRoundingFlagsForRectInRect(title_bar_rect, host_window.Rect(), DOCKING_SPLITTER_SIZE);
    host_window.draw_list.add_rect_filled(title_bar_rect.min, title_bar_rect.max, title_bar_col, host_window.WindowRounding, rounding_flags);

    // Docking/Collapse button
    if (has_window_menu_button)
    {
        if (collapse_button(host_window.get_id("#COLLAPSE"), window_menu_button_pos, node)) // == DockNodeGetWindowMenuButtonId(node)
            OpenPopup("#WindowMenu");
        if (IsItemActive())
            focus_tab_id = tab_bar.SelectedTabId;
    }

    // If multiple tabs are appearing on the same frame, sort them based on their persistent dock_order value
    int tabs_unsorted_start = tab_bar.Tabs.size;
    for (int tab_n = tab_bar.Tabs.size - 1; tab_n >= 0 && (tab_bar.Tabs[tab_n].flags & ImGuiTabItemFlags_Unsorted); tab_n--)
    {
        // FIXME-DOCK: Consider only clearing the flag after the tab has been alive for a few consecutive frames, allowing late comers to not break sorting?
        tab_bar.Tabs[tab_n].flags &= ~ImGuiTabItemFlags_Unsorted;
        tabs_unsorted_start = tab_n;
    }
    if (tab_bar.Tabs.size > tabs_unsorted_start)
    {
        IMGUI_DEBUG_LOG_DOCKING("[docking] In node 0x%08X: %d new appearing tabs:%s\n", node.ID, tab_bar.Tabs.size - tabs_unsorted_start, (tab_bar.Tabs.size > tabs_unsorted_start + 1) ? " (will sort)" : "");
        for (int tab_n = tabs_unsorted_start; tab_n < tab_bar.Tabs.size; tab_n += 1)
            IMGUI_DEBUG_LOG_DOCKING("[docking] - Tab '%s' Order %d\n", tab_bar.Tabs[tab_n].Window.Name, tab_bar.Tabs[tab_n].Window.dock_order);
        if (tab_bar.Tabs.size > tabs_unsorted_start + 1)
            ImQsort(tab_bar.Tabs.data + tabs_unsorted_start, tab_bar.Tabs.size - tabs_unsorted_start, sizeof(ImGuiTabItem), TabItemComparerByDockOrder);
    }

    // Apply nav_window focus back to the tab bar
    if (g.nav_window && g.nav_window.root_window.dock_node == node)
        tab_bar.SelectedTabId = g.nav_window.root_window.ID;

    // Selected newly added tabs, or persistent tab id if the tab bar was just recreated
    if (tab_bar_is_recreated && TabBarFindTabByID(tab_bar, node.SelectedTabId) != NULL)
        tab_bar.SelectedTabId = tab_bar.NextSelectedTabId = node.SelectedTabId;
    else if (tab_bar.Tabs.size > tabs_count_old)
        tab_bar.SelectedTabId = tab_bar.NextSelectedTabId = tab_bar.Tabs.back().Window.TabId;

    // Begin tab bar
    ImGuiTabBarFlags tab_bar_flags = ImGuiTabBarFlags_Reorderable | ImGuiTabBarFlags_AutoSelectNewTabs; // | ImGuiTabBarFlags_NoTabListScrollingButtons);
    tab_bar_flags |= ImGuiTabBarFlags_SaveSettings | ImGuiTabBarFlags_DockNode;
    if (!host_window.collapsed && is_focused)
        tab_bar_flags |= ImGuiTabBarFlags_IsFocused;
    BeginTabBarEx(tab_bar, tab_bar_rect, tab_bar_flags, node);
    //host_window->draw_list->add_rect(tab_bar_rect.min, tab_bar_rect.max, IM_COL32(255,0,255,255));

    // Backup style colors
    Vector4D backup_style_cols[ImGuiWindowDockStyleCol_COUNT];
    for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
        backup_style_cols[color_n] = g.style.colors[GWindowDockStyleColors[color_n]];

    // Submit actual tabs
    node.VisibleWindow = NULL;
    for (int window_n = 0; window_n < node.Windows.size; window_n += 1)
    {
        ImGuiWindow* window = node.Windows[window_n];
        if ((closed_all || closed_one == window.TabId) && window.HasCloseButton && !(window.flags & WindowFlags::UnsavedDocument))
            continue;
        if (window.LastFrameActive + 1 >= g.frame_count || !node_was_active)
        {
            ImGuiTabItemFlags tab_item_flags = 0;
            tab_item_flags |= window.WindowClass.TabItemFlagsOverrideSet;
            if (window.flags & WindowFlags::UnsavedDocument)
                tab_item_flags |= ImGuiTabItemFlags_UnsavedDocument;
            if (tab_bar.flags & ImGuiTabBarFlags_NoCloseWithMiddleMouseButton)
                tab_item_flags |= ImGuiTabItemFlags_NoCloseWithMiddleMouseButton;

            // Apply stored style overrides for the window
            for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
                g.style.colors[GWindowDockStyleColors[color_n]] = ColorConvertU32ToFloat4(window.DockStyle.colors[color_n]);

            // Note that TabItemEx() calls TabBarCalcTabID() so our tab item id will ignore the current id stack (rightly so)
            bool tab_open = true;
            TabItemEx(tab_bar, window.Name, window.HasCloseButton ? &tab_open : NULL, tab_item_flags, window);
            if (!tab_open)
                node.WantCloseTabId = window.TabId;
            if (tab_bar.VisibleTabId == window.TabId)
                node.VisibleWindow = window;

            // Store last item data so it can be queried with IsItemXXX functions after the user Begin() call
            window.DockTabItemStatusFlags = g.last_item_data.status_flags;
            window.DockTabItemRect = g.last_item_data.Rect;

            // Update navigation id on menu layer
            if (g.nav_window && g.nav_window.root_window == window && (window.dc.nav_layers_active_mask & (1 << NavLayer::Menu)) == 0)
                host_window.NavLastIds[1] = window.TabId;
        }
    }

    // Restore style colors
    for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
        g.style.colors[GWindowDockStyleColors[color_n]] = backup_style_cols[color_n];

    // Notify root of visible window (used to display title in OS task bar)
    if (node.VisibleWindow)
        if (is_focused || root_node.VisibleWindow == NULL)
            root_node.VisibleWindow = node.VisibleWindow;

    // Close button (after visible_window was updated)
    // Note that visible_window may have been overrided by CTRL+Tabbing, so visible_window->tab_id may be != from tab_bar->selected_tab_id
    const bool close_button_is_enabled = node.HasCloseButton && node.VisibleWindow && node.VisibleWindow.HasCloseButton;
    const bool close_button_is_visible = node.HasCloseButton;
    //const bool close_button_is_visible = close_button_is_enabled; // Most people would expect this behavior of not even showing the button (leaving a hole since we can't claim that space as other windows in the tba bar have one)
    if (close_button_is_visible)
    {
        if (!close_button_is_enabled)
        {
            PushItemFlag(ItemFlags::Disabled, true);
            push_style_color(StyleColor::Text, style.colors[StyleColor::Text] * Vector4D(1.0,1.0,1.0,0.4));
        }
        if (close_button(host_window.get_id("#CLOSE"), close_button_pos))
        {
            node.WantCloseAll = true;
            for (int n = 0; n < tab_bar.Tabs.size; n += 1)
                TabBarCloseTab(tab_bar, &tab_bar.Tabs[n]);
        }
        //if (IsItemActive())
        //    focus_tab_id = tab_bar->selected_tab_id;
        if (!close_button_is_enabled)
        {
            pop_style_color();
            PopItemFlag();
        }
    }

    // When clicking on the title bar outside of tabs, we still focus the selected tab for that node
    // FIXME: TabItem use AllowItemOverlap so we manually perform a more specific test for now (hovered || held)
    ImGuiID title_bar_id = host_window.get_id("#TITLEBAR");
    if (g.hovered_id == 0 || g.hovered_id == title_bar_id || g.active_id == title_bar_id)
    {
        bool held;
        button_behavior(title_bar_rect, title_bar_id, NULL, &held, ButtonFlags::AllowItemOverlap);
        if (g.hovered_id == title_bar_id)
        {
            // ImGuiButtonFlags_AllowItemOverlap + SetItemAllowOverlap() required for appending into dock node tab bar,
            // otherwise dragging window will steal hovered_id and amended tabs cannot get them.
            g.last_item_data.id = title_bar_id;
            SetItemAllowOverlap();
        }
        if (held)
        {
            if (IsMouseClicked(0))
                focus_tab_id = tab_bar.SelectedTabId;

            // Forward moving request to selected window
            if (ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, tab_bar.SelectedTabId))
                start_mouse_moving_window_or_node(tab.Window ? tab.Window : node.host_window, node, false);
        }
    }

    // Forward focus from host node to selected window
    //if (is_focused && g.nav_window == host_window && !g.nav_windowing_target)
    //    focus_tab_id = tab_bar->selected_tab_id;

    // When clicked on a tab we requested focus to the docked child
    // This overrides the value set by "forward focus from host node to selected window".
    if (tab_bar.NextSelectedTabId)
        focus_tab_id = tab_bar.NextSelectedTabId;

    // Apply navigation focus
    if (focus_tab_id != 0)
        if (ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, focus_tab_id))
            if (tab.Window)
            {
                focus_window(tab.Window);
                nav_init_window(tab.Window, false);
            }

    EndTabBar();
    PopID();

    // Restore skip_items flag
    if (!node.IsDockSpace())
    {
        host_window.dcnav_layer_current = NavLayer::Main;
        host_window.skip_items = backup_skip_item;
    }
}

static void DockNodeAddTabBar(ImGuiDockNode* node)
{
    IM_ASSERT(node.TabBar == NULL);
    node.TabBar = IM_NEW(ImGuiTabBar);
}

static void DockNodeRemoveTabBar(ImGuiDockNode* node)
{
    if (node.TabBar == NULL)
        return;
    IM_DELETE(node.TabBar);
    node.TabBar = NULL;
}

static bool DockNodeIsDropAllowedOne(ImGuiWindow* payload, ImGuiWindow* host_window)
{
    if (host_window.DockNodeAsHost && host_window.DockNodeAsHost.IsDockSpace() && payload.BeginOrderWithinContext < host_window.BeginOrderWithinContext)
        return false;

    ImGuiWindowClass* host_class = host_window.DockNodeAsHost ? &host_window.DockNodeAsHost.WindowClass : &host_window.WindowClass;
    ImGuiWindowClass* payload_class = &payload.WindowClass;
    if (host_class.ClassId != payload_class.ClassId)
    {
        if (host_class.ClassId != 0 && host_class.DockingAllowUnclassed && payload_class.ClassId == 0)
            return true;
        if (payload_class.ClassId != 0 && payload_class.DockingAllowUnclassed && host_class.ClassId == 0)
            return true;
        return false;
    }

    // Prevent docking any window created above a popup
    // Technically we should support it (e.g. in the case of a long-lived modal window that had fancy docking features),
    // by e.g. adding a 'if (!IsWindowWithinBeginStackOf(host_window, popup_window))' test.
    // But it would requires more work on our end because the dock host windows is technically created in NewFrame()
    // and our ->ParentXXX and ->RootXXX pointers inside windows are currently mislading or lacking.
    ImGuiContext& g = *GImGui;
    for (int i = g.open_popup_stack.size - 1; i >= 0; i--)
        if (ImGuiWindow* popup_window = g.open_popup_stack[i].Window)
            if (is_window_within_begin_stack_of(payload, popup_window))   // Payload is created from within a popup begin stack.
                return false;

    return true;
}

static bool DockNodeIsDropAllowed(ImGuiWindow* host_window, ImGuiWindow* root_payload)
{
    if (root_payload.DockNodeAsHost && root_payload.DockNodeAsHost.IsSplitNode()) // FIXME-DOCK: Missing filtering
        return true;

    const int payload_count = root_payload.DockNodeAsHost ? root_payload.DockNodeAsHost.Windows.size : 1;
    for (int payload_n = 0; payload_n < payload_count; payload_n += 1)
    {
        ImGuiWindow* payload = root_payload.DockNodeAsHost ? root_payload.DockNodeAsHost.Windows[payload_n] : root_payload;
        if (DockNodeIsDropAllowedOne(payload, host_window))
            return true;
    }
    return false;
}

// window menu button == collapse button when not in a dock node.
// FIXME: This is similar to RenderWindowTitleBarContents(), may want to share code.
static void DockNodeCalcTabBarLayout(const ImGuiDockNode* node, Rect* out_title_rect, Rect* out_tab_bar_rect, Vector2D* out_window_menu_button_pos, Vector2D* out_close_button_pos)
{
    ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.style;

    Rect r = Rect(node.pos.x, node.pos.y, node.pos.x + node.size.x, node.pos.y + g.font_size + g.style.frame_padding.y * 2.0);
    if (out_title_rect) { *out_title_rect = r; }

    r.min.x += style.WindowBorderSize;
    r.max.x -= style.WindowBorderSize;

    float button_sz = g.font_size;

    Vector2D window_menu_button_pos = r.min;
    r.min.x += style.frame_padding.x;
    r.max.x -= style.frame_padding.x;
    if (node.HasCloseButton)
    {
        r.max.x -= button_sz;
        if (out_close_button_pos) *out_close_button_pos = Vector2D::new(r.max.x - style.frame_padding.x, r.min.y);
    }
    if (node.HasWindowMenuButton && style.window_menu_button_position == Dir::Left)
    {
        r.min.x += button_sz + style.item_inner_spacing.x;
    }
    else if (node.HasWindowMenuButton && style.window_menu_button_position == Dir::Right)
    {
        r.max.x -= button_sz + style.frame_padding.x;
        window_menu_button_pos = Vector2D::new(r.max.x, r.min.y);
    }
    if (out_tab_bar_rect) { *out_tab_bar_rect = r; }
    if (out_window_menu_button_pos) { *out_window_menu_button_pos = window_menu_button_pos; }
}

void DockNodeCalcSplitRects(Vector2D& pos_old, Vector2D& size_old, Vector2D& pos_new, Vector2D& size_new, ImGuiDir dir, Vector2D size_new_desired)
{
    ImGuiContext& g = *GImGui;
    const float dock_spacing = g.style.item_inner_spacing.x;
    const ImGuiAxis axis = (dir == Dir::Left || dir == Dir::Right) ? Axis::X : Axis::Y;
    pos_new[axis ^ 1] = pos_old[axis ^ 1];
    size_new[axis ^ 1] = size_old[axis ^ 1];

    // Distribute size on given axis (with a desired size or equally)
    const float w_avail = size_old[axis] - dock_spacing;
    if (size_new_desired[axis] > 0.0 && size_new_desired[axis] <= w_avail * 0.5)
    {
        size_new[axis] = size_new_desired[axis];
        size_old[axis] = f32::floor(w_avail - size_new[axis]);
    }
    else
    {
        size_new[axis] = f32::floor(w_avail * 0.5);
        size_old[axis] = f32::floor(w_avail - size_new[axis]);
    }

    // Position each node
    if (dir == Dir::Right || dir == Dir::Down)
    {
        pos_new[axis] = pos_old[axis] + size_old[axis] + dock_spacing;
    }
    else if (dir == Dir::Left || dir == Dir::Up)
    {
        pos_new[axis] = pos_old[axis];
        pos_old[axis] = pos_new[axis] + size_new[axis] + dock_spacing;
    }
}

// Retrieve the drop rectangles for a given direction or for the center + perform hit testing.
bool DockNodeCalcDropRectsAndTestMousePos(const Rect& parent, ImGuiDir dir, Rect& out_r, bool outer_docking, Vector2D* test_mouse_pos)
{
    ImGuiContext& g = *GImGui;

    const float parent_smaller_axis = ImMin(parent.get_width(), parent.get_height());
    const float hs_for_central_nodes = ImMin(g.font_size * 1.5, ImMax(g.font_size * 0.5, parent_smaller_axis / 8.0));
    float hs_w; // Half-size, longer axis
    float hs_h; // Half-size, smaller axis
    Vector2D off; // Distance from edge or center
    if (outer_docking)
    {
        //hs_w = f32::floor(ImClamp(parent_smaller_axis - hs_for_central_nodes * 4.0, g.font_size * 0.5, g.font_size * 8.0));
        //hs_h = f32::floor(hs_w * 0.15);
        //off = Vector2D(f32::floor(parent.get_width() * 0.5 - GetFrameHeightWithSpacing() * 1.4 - hs_h), f32::floor(parent.get_height() * 0.5 - GetFrameHeightWithSpacing() * 1.4 - hs_h));
        hs_w = f32::floor(hs_for_central_nodes * 1.50);
        hs_h = f32::floor(hs_for_central_nodes * 0.80);
        off = Vector2D::new(f32::floor(parent.get_width() * 0.5 - hs_h), f32::floor(parent.get_height() * 0.5 - hs_h));
    }
    else
    {
        hs_w = f32::floor(hs_for_central_nodes);
        hs_h = f32::floor(hs_for_central_nodes * 0.90);
        off = Vector2D::new(f32::floor(hs_w * 2.40), f32::floor(hs_w * 2.40));
    }

    Vector2D c = f32::floor(parent.GetCenter());
    if      (dir == Dir::None)  { out_r = Rect(c.x - hs_w, c.y - hs_w,         c.x + hs_w, c.y + hs_w);         }
    else if (dir == Dir::Up)    { out_r = Rect(c.x - hs_w, c.y - off.y - hs_h, c.x + hs_w, c.y - off.y + hs_h); }
    else if (dir == Dir::Down)  { out_r = Rect(c.x - hs_w, c.y + off.y - hs_h, c.x + hs_w, c.y + off.y + hs_h); }
    else if (dir == Dir::Left)  { out_r = Rect(c.x - off.x - hs_h, c.y - hs_w, c.x - off.x + hs_h, c.y + hs_w); }
    else if (dir == Dir::Right) { out_r = Rect(c.x + off.x - hs_h, c.y - hs_w, c.x + off.x + hs_h, c.y + hs_w); }

    if (test_mouse_pos == NULL)
        return false;

    Rect hit_r = out_r;
    if (!outer_docking)
    {
        // Custom hit testing for the 5-way selection, designed to reduce flickering when moving diagonally between sides
        hit_r.Expand(f32::floor(hs_w * 0.30));
        Vector2D mouse_delta = (*test_mouse_pos - c);
        float mouse_delta_len2 = ImLengthSqr(mouse_delta);
        float r_threshold_center = hs_w * 1.4;
        float r_threshold_sides = hs_w * (1.4 + 1.2);
        if (mouse_delta_len2 < r_threshold_center * r_threshold_center)
            return (dir == Dir::None);
        if (mouse_delta_len2 < r_threshold_sides * r_threshold_sides)
            return (dir == ImGetDirQuadrantFromDelta(mouse_delta.x, mouse_delta.y));
    }
    return hit_r.Contains(*test_mouse_pos);
}

// host_node may be NULL if the window doesn't have a dock_node already.
// FIXME-DOCK: This is misnamed since it's also doing the filtering.
static void DockNodePreviewDockSetup(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* root_payload, ImGuiDockPreviewData* data, bool is_explicit_target, bool is_outer_docking)
{
    ImGuiContext& g = *GImGui;

    // There is an edge case when docking into a dockspace which only has inactive nodes.
    // In this case DockNodeTreeFindNodeByPos() will have selected a leaf node which is inactive.
    // Because the inactive leaf node doesn't have proper pos/size yet, we'll use the root node as reference.
    ImGuiDockNode* root_payload_as_host = root_payload.DockNodeAsHost;
    ImGuiDockNode* ref_node_for_rect = (host_node && !host_node.IsVisible) ? DockNodeGetRootNode(host_node) : host_node;
    if (ref_node_for_rect)
        IM_ASSERT(ref_node_for_rect.IsVisible == true);

    // Filter, figure out where we are allowed to dock
    ImGuiDockNodeFlags src_node_flags = root_payload_as_host ? root_payload_as_host.MergedFlags : root_payload.WindowClass.DockNodeFlagsOverrideSet;
    ImGuiDockNodeFlags dst_node_flags = host_node ? host_node.MergedFlags : host_window.WindowClass.DockNodeFlagsOverrideSet;
    data.IsCenterAvailable = true;
    if (is_outer_docking)
        data.IsCenterAvailable = false;
    else if (dst_node_flags & ImGuiDockNodeFlags_NoDocking)
        data.IsCenterAvailable = false;
    else if (host_node && (dst_node_flags & ImGuiDockNodeFlags_NoDockingInCentralNode) && host_node.IsCentralNode())
        data.IsCenterAvailable = false;
    else if ((!host_node || !host_node.IsEmpty()) && root_payload_as_host && root_payload_as_host.IsSplitNode() && (root_payload_as_host.OnlyNodeWithWindows == NULL)) // Is _visibly_ split?
        data.IsCenterAvailable = false;
    else if (dst_node_flags & ImGuiDockNodeFlags_NoDockingOverMe)
        data.IsCenterAvailable = false;
    else if ((src_node_flags & ImGuiDockNodeFlags_NoDockingOverOther) && (!host_node || !host_node.IsEmpty()))
        data.IsCenterAvailable = false;
    else if ((src_node_flags & ImGuiDockNodeFlags_NoDockingOverEmpty) && host_node && host_node.IsEmpty())
        data.IsCenterAvailable = false;

    data.IsSidesAvailable = true;
    if ((dst_node_flags & ImGuiDockNodeFlags_NoSplit) || g.io.ConfigDockingNoSplit)
        data.IsSidesAvailable = false;
    else if (!is_outer_docking && host_node && host_node.ParentNode == NULL && host_node.IsCentralNode())
        data.IsSidesAvailable = false;
    else if ((dst_node_flags & ImGuiDockNodeFlags_NoDockingSplitMe) || (src_node_flags & ImGuiDockNodeFlags_NoDockingSplitOther))
        data.IsSidesAvailable = false;

    // build a tentative future node (reuse same structure because it is practical. Shape will be readjusted when previewing a split)
    data.FutureNode.HasCloseButton = (host_node ? host_node.HasCloseButton : host_window.HasCloseButton) || (root_payload.HasCloseButton);
    data.FutureNode.HasWindowMenuButton = host_node ? true : ((host_window.flags & WindowFlags::NoCollapse) == 0);
    data.FutureNode.Pos = ref_node_for_rect ? ref_node_for_rect.pos : host_window.Pos;
    data.FutureNode.size = ref_node_for_rect ? ref_node_for_rect.size : host_window.size;

    // Calculate drop shapes geometry for allowed splitting directions
    IM_ASSERT(Dir::None == -1);
    data.SplitNode = host_node;
    data.SplitDir = Dir::None;
    data.IsSplitDirExplicit = false;
    if (!host_window.collapsed)
        for (int dir = Dir::None; dir < Dir::COUNT; dir += 1)
        {
            if (dir == Dir::None && !data.IsCenterAvailable)
                continue;
            if (dir != Dir::None && !data.IsSidesAvailable)
                continue;
            if (DockNodeCalcDropRectsAndTestMousePos(data.FutureNode.Rect(), (ImGuiDir)dir, data.DropRectsDraw[dir+1], is_outer_docking, &g.io.mouse_pos))
            {
                data.SplitDir = (ImGuiDir)dir;
                data.IsSplitDirExplicit = true;
            }
        }

    // When docking without holding Shift, we only allow and preview docking when hovering over a drop rect or over the title bar
    data.IsDropAllowed = (data.SplitDir != Dir::None) || (data.IsCenterAvailable);
    if (!is_explicit_target && !data.IsSplitDirExplicit && !g.io.ConfigDockingWithShift)
        data.IsDropAllowed = false;

    // Calculate split area
    data.SplitRatio = 0.0;
    if (data.SplitDir != Dir::None)
    {
        ImGuiDir split_dir = data.SplitDir;
        ImGuiAxis split_axis = (split_dir == Dir::Left || split_dir == Dir::Right) ? Axis::X : Axis::Y;
        Vector2D pos_new, pos_old = data.FutureNode.Pos;
        Vector2D size_new, size_old = data.FutureNode.size;
        DockNodeCalcSplitRects(pos_old, size_old, pos_new, size_new, split_dir, root_payload.size);

        // Calculate split ratio so we can pass it down the docking request
        float split_ratio = ImSaturate(size_new[split_axis] / data.FutureNode.size[split_axis]);
        data.FutureNode.Pos = pos_new;
        data.FutureNode.size = size_new;
        data.SplitRatio = (split_dir == Dir::Right || split_dir == Dir::Down) ? (1.0 - split_ratio) : (split_ratio);
    }
}

static void DockNodePreviewDockRender(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* root_payload, const ImGuiDockPreviewData* data)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.current_window == host_window);   // Because we rely on font size to calculate tab sizes

    // With this option, we only display the preview on the target viewport, and the payload viewport is made transparent.
    // To compensate for the single layer obstructed by the payload, we'll increase the alpha of the preview nodes.
    const bool is_transparent_payload = g.io.config_docking_transparent_payload;

    // In case the two windows involved are on different viewports, we will draw the overlay on each of them.
    int overlay_draw_lists_count = 0;
    ImDrawList* overlay_draw_lists[2];
    overlay_draw_lists[overlay_draw_lists_count += 1] = get_foreground_draw_list(host_window.viewport);
    if (host_window.viewport != root_payload.Viewport && !is_transparent_payload)
        overlay_draw_lists[overlay_draw_lists_count += 1] = get_foreground_draw_list(root_payload.Viewport);

    // Draw main preview rectangle
    const ImU32 overlay_col_main = get_color_u32(StyleColor::DockingPreview, is_transparent_payload ? 0.60 : 0.40);
    const ImU32 overlay_col_drop = get_color_u32(StyleColor::DockingPreview, is_transparent_payload ? 0.90 : 0.70);
    const ImU32 overlay_col_drop_hovered = get_color_u32(StyleColor::DockingPreview, is_transparent_payload ? 1.20 : 1.00);
    const ImU32 overlay_col_lines = get_color_u32(StyleColor::NavWindowingHighlight, is_transparent_payload ? 0.80 : 0.60);

    // Display area preview
    const bool can_preview_tabs = (root_payload.DockNodeAsHost == NULL || root_payload.DockNodeAsHost.Windows.size > 0);
    if (data.IsDropAllowed)
    {
        Rect overlay_rect = data.FutureNode.Rect();
        if (data.SplitDir == Dir::None && can_preview_tabs)
            overlay_rect.min.y += GetFrameHeight();
        if (data.SplitDir != Dir::None || data.IsCenterAvailable)
            for (int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n += 1)
                overlay_draw_lists[overlay_n].add_rect_filled(overlay_rect.min, overlay_rect.max, overlay_col_main, host_window.WindowRounding, CalcRoundingFlagsForRectInRect(overlay_rect, host_window.Rect(), DOCKING_SPLITTER_SIZE));
    }

    // Display tab shape/label preview unless we are splitting node (it generally makes the situation harder to read)
    if (data.IsDropAllowed && can_preview_tabs && data.SplitDir == Dir::None && data.IsCenterAvailable)
    {
        // Compute target tab bar geometry so we can locate our preview tabs
        Rect tab_bar_rect;
        DockNodeCalcTabBarLayout(&data.FutureNode, NULL, &tab_bar_rect, NULL, NULL);
        Vector2D tab_pos = tab_bar_rect.min;
        if (host_node && host_node.TabBar)
        {
            if (!host_node.is_hidden_tab_bar() && !host_node.is_no_tab_bar())
                tab_pos.x += host_node.TabBar.WidthAllTabs + g.style.item_inner_spacing.x; // We don't use OffsetNewTab because when using non-persistent-order tab bar it is incremented with each Tab submission.
            else
                tab_pos.x += g.style.item_inner_spacing.x + TabItemCalcSize(host_node.Windows[0].Name, host_node.Windows[0].HasCloseButton).x;
        }
        else if (!(host_window.flags & WindowFlags::DockNodeHost))
        {
            tab_pos.x += g.style.item_inner_spacing.x + TabItemCalcSize(host_window.Name, host_window.HasCloseButton).x; // Account for slight offset which will be added when changing from title bar to tab bar
        }

        // Draw tab shape/label preview (payload may be a loose window or a host window carrying multiple tabbed windows)
        if (root_payload.DockNodeAsHost)
            IM_ASSERT(root_payload.DockNodeAsHost.Windows.size <= root_payload.DockNodeAsHost.TabBar.Tabs.size);
        ImGuiTabBar* tab_bar_with_payload = root_payload.DockNodeAsHost ? root_payload.DockNodeAsHost.TabBar : NULL;
        const int payload_count = tab_bar_with_payload ? tab_bar_with_payload.Tabs.size : 1;
        for (int payload_n = 0; payload_n < payload_count; payload_n += 1)
        {
            // dock_node's tab_bar may have non-window Tabs manually appended by user
            ImGuiWindow* payload_window = tab_bar_with_payload ? tab_bar_with_payload.Tabs[payload_n].Window : root_payload;
            if (tab_bar_with_payload && payload_window == NULL)
                continue;
            if (!DockNodeIsDropAllowedOne(payload_window, host_window))
                continue;

            // Calculate the tab bounding box for each payload window
            Vector2D tab_size = TabItemCalcSize(payload_window.Name, payload_window.HasCloseButton);
            Rect tab_bb(tab_pos.x, tab_pos.y, tab_pos.x + tab_size.x, tab_pos.y + tab_size.y);
            tab_pos.x += tab_size.x + g.style.item_inner_spacing.x;
            const ImU32 overlay_col_text = get_color_u32(payload_window.DockStyle.colors[ImGuiWindowDockStyleCol_Text]);
            const ImU32 overlay_col_tabs = get_color_u32(payload_window.DockStyle.colors[ImGuiWindowDockStyleCol_TabActive]);
            push_style_color(StyleColor::Text, overlay_col_text);
            for (int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n += 1)
            {
                ImGuiTabItemFlags tab_flags = ImGuiTabItemFlags_Preview | ((payload_window.flags & WindowFlags::UnsavedDocument) ? ImGuiTabItemFlags_UnsavedDocument : 0);
                if (!tab_bar_rect.Contains(tab_bb))
                    overlay_draw_lists[overlay_n].push_clip_rect(tab_bar_rect.min, tab_bar_rect.max);
                TabItemBackground(overlay_draw_lists[overlay_n], tab_bb, tab_flags, overlay_col_tabs);
                TabItemLabelAndCloseButton(overlay_draw_lists[overlay_n], tab_bb, tab_flags, g.style.frame_padding, payload_window.Name, 0, 0, false, NULL, NULL);
                if (!tab_bar_rect.Contains(tab_bb))
                    overlay_draw_lists[overlay_n].pop_clip_rect();
            }
            pop_style_color();
        }
    }

    // Display drop boxes
    const float overlay_rounding = ImMax(3.0, g.style.FrameRounding);
    for (int dir = Dir::None; dir < Dir::COUNT; dir += 1)
    {
        if (!data.DropRectsDraw[dir + 1].IsInverted())
        {
            Rect draw_r = data.DropRectsDraw[dir + 1];
            Rect draw_r_in = draw_r;
            draw_r_in.Expand(-2.0);
            ImU32 overlay_col = (data.SplitDir == (ImGuiDir)dir && data.IsSplitDirExplicit) ? overlay_col_drop_hovered : overlay_col_drop;
            for (int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n += 1)
            {
                Vector2D center = f32::floor(draw_r_in.GetCenter());
                overlay_draw_lists[overlay_n].add_rect_filled(draw_r.min, draw_r.max, overlay_col, overlay_rounding);
                overlay_draw_lists[overlay_n].AddRect(draw_r_in.min, draw_r_in.max, overlay_col_lines, overlay_rounding);
                if (dir == Dir::Left || dir == Dir::Right)
                    overlay_draw_lists[overlay_n].add_line(Vector2D::new(center.x, draw_r_in.min.y), Vector2D::new(center.x, draw_r_in.max.y), overlay_col_lines);
                if (dir == Dir::Up || dir == Dir::Down)
                    overlay_draw_lists[overlay_n].add_line(Vector2D::new(draw_r_in.min.x, center.y), Vector2D::new(draw_r_in.max.x, center.y), overlay_col_lines);
            }
        }

        // Stop after ImGuiDir_None
        if ((host_node && (host_node.MergedFlags & ImGuiDockNodeFlags_NoSplit)) || g.io.ConfigDockingNoSplit)
            return;
    }
}

//-----------------------------------------------------------------------------
// Docking: ImGuiDockNode Tree manipulation functions
//-----------------------------------------------------------------------------
// - DockNodeTreeSplit()
// - DockNodeTreeMerge()
// - DockNodeTreeUpdatePosSize()
// - DockNodeTreeUpdateSplitterFindTouchingNode()
// - DockNodeTreeUpdateSplitter()
// - DockNodeTreeFindFallbackLeafNode()
// - DockNodeTreeFindNodeByPos()
//-----------------------------------------------------------------------------

void DockNodeTreeSplit(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiAxis split_axis, int split_inheritor_child_idx, float split_ratio, ImGuiDockNode* new_node)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(split_axis != ImGuiAxis_None);

    ImGuiDockNode* child_0 = (new_node && split_inheritor_child_idx != 0) ? new_node : DockContextAddNode(ctx, 0);
    child_0.ParentNode = parent_node;

    ImGuiDockNode* child_1 = (new_node && split_inheritor_child_idx != 1) ? new_node : DockContextAddNode(ctx, 0);
    child_1.ParentNode = parent_node;

    ImGuiDockNode* child_inheritor = (split_inheritor_child_idx == 0) ? child_0 : child_1;
    DockNodeMoveChildNodes(child_inheritor, parent_node);
    parent_node.ChildNodes[0] = child_0;
    parent_node.ChildNodes[1] = child_1;
    parent_node.ChildNodes[split_inheritor_child_idx].VisibleWindow = parent_node.VisibleWindow;
    parent_node.SplitAxis = split_axis;
    parent_node.VisibleWindow = NULL;
    parent_node.AuthorityForPos = parent_node.AuthorityForSize = ImGuiDataAuthority_DockNode;

    float size_avail = (parent_node.size[split_axis] - DOCKING_SPLITTER_SIZE);
    size_avail = ImMax(size_avail, g.style.window_min_size[split_axis] * 2.0);
    IM_ASSERT(size_avail > 0.0); // If you created a node manually with DockBuilderAddNode(), you need to also call DockBuilderSetNodeSize() before splitting.
    child_0.sizeRef = child_1.sizeRef = parent_node.size;
    child_0.sizeRef[split_axis] = f32::floor(size_avail * split_ratio);
    child_1.sizeRef[split_axis] = f32::floor(size_avail - child_0.sizeRef[split_axis]);

    DockNodeMoveWindows(parent_node.ChildNodes[split_inheritor_child_idx], parent_node);
    DockSettingsRenameNodeReferences(parent_node.ID, parent_node.ChildNodes[split_inheritor_child_idx].ID);
    DockNodeUpdateHasCentralNodeChild(DockNodeGetRootNode(parent_node));
    DockNodeTreeUpdatePosSize(parent_node, parent_node.pos, parent_node.size);

    // flags transfer (e.g. this is where we transfer the ImGuiDockNodeFlags_CentralNode property)
    child_0.SharedFlags = parent_node.SharedFlags & ImGuiDockNodeFlags_SharedFlagsInheritMask_;
    child_1.SharedFlags = parent_node.SharedFlags & ImGuiDockNodeFlags_SharedFlagsInheritMask_;
    child_inheritor.LocalFlags = parent_node.LocalFlags & ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    parent_node.LocalFlags &= ~ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    child_0.UpdateMergedFlags();
    child_1.UpdateMergedFlags();
    parent_node.UpdateMergedFlags();
    if (child_inheritor.IsCentralNode())
        DockNodeGetRootNode(parent_node).CentralNode = child_inheritor;
}

void DockNodeTreeMerge(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiDockNode* merge_lead_child)
{
    // When called from DockContextProcessUndockNode() it is possible that one of the child is NULL.
    ImGuiContext& g = *GImGui;
    ImGuiDockNode* child_0 = parent_node.ChildNodes[0];
    ImGuiDockNode* child_1 = parent_node.ChildNodes[1];
    IM_ASSERT(child_0 || child_1);
    IM_ASSERT(merge_lead_child == child_0 || merge_lead_child == child_1);
    if ((child_0 && child_0.Windows.size > 0) || (child_1 && child_1.Windows.size > 0))
    {
        IM_ASSERT(parent_node.TabBar == NULL);
        IM_ASSERT(parent_node.Windows.size == 0);
    }
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeTreeMerge: 0x%08X + 0x%08X back into parent 0x%08X\n", child_0 ? child_0.ID : 0, child_1 ? child_1.ID : 0, parent_node.ID);

    Vector2D backup_last_explicit_size = parent_node.sizeRef;
    DockNodeMoveChildNodes(parent_node, merge_lead_child);
    if (child_0)
    {
        DockNodeMoveWindows(parent_node, child_0); // Generally only 1 of the 2 child node will have windows
        DockSettingsRenameNodeReferences(child_0.ID, parent_node.ID);
    }
    if (child_1)
    {
        DockNodeMoveWindows(parent_node, child_1);
        DockSettingsRenameNodeReferences(child_1.ID, parent_node.ID);
    }
    DockNodeApplyPosSizeToWindows(parent_node);
    parent_node.AuthorityForPos = parent_node.AuthorityForSize = parent_node.AuthorityForViewport = ImGuiDataAuthority_Auto;
    parent_node.VisibleWindow = merge_lead_child.VisibleWindow;
    parent_node.sizeRef = backup_last_explicit_size;

    // flags transfer
    parent_node.LocalFlags &= ~ImGuiDockNodeFlags_LocalFlagsTransferMask_; // Preserve Dockspace flag
    parent_node.LocalFlags |= (child_0 ? child_0.LocalFlags : 0) & ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    parent_node.LocalFlags |= (child_1 ? child_1.LocalFlags : 0) & ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    parent_node.LocalFlagsInWindows = (child_0 ? child_0.LocalFlagsInWindows : 0) | (child_1 ? child_1.LocalFlagsInWindows : 0); // FIXME: Would be more consistent to update from actual windows
    parent_node.UpdateMergedFlags();

    if (child_0)
    {
        ctx.DockContext.Nodes.SetVoidPtr(child_0.ID, NULL);
        IM_DELETE(child_0);
    }
    if (child_1)
    {
        ctx.DockContext.Nodes.SetVoidPtr(child_1.ID, NULL);
        IM_DELETE(child_1);
    }
}

// Update pos/size for a node hierarchy (don't affect child windows yet)
// (Depth-first, Pre-Order)
void DockNodeTreeUpdatePosSize(ImGuiDockNode* node, Vector2D pos, Vector2D size, ImGuiDockNode* only_write_to_single_node)
{
    // During the regular dock node update we write to all nodes.
    // 'only_write_to_single_node' is only set when turning a node visible mid-frame and we need its size right-away.
    const bool write_to_node = only_write_to_single_node == NULL || only_write_to_single_node == node;
    if (write_to_node)
    {
        node.pos = pos;
        node.size = size;
    }

    if (node.IsLeafNode())
        return;

    ImGuiDockNode* child_0 = node.ChildNodes[0];
    ImGuiDockNode* child_1 = node.ChildNodes[1];
    Vector2D child_0_pos = pos, child_1_pos = pos;
    Vector2D child_0_size = size, child_1_size = size;

    const bool child_0_is_toward_single_node = (only_write_to_single_node != NULL && DockNodeIsInHierarchyOf(only_write_to_single_node, child_0));
    const bool child_1_is_toward_single_node = (only_write_to_single_node != NULL && DockNodeIsInHierarchyOf(only_write_to_single_node, child_1));
    const bool child_0_is_or_will_be_visible = child_0.IsVisible || child_0_is_toward_single_node;
    const bool child_1_is_or_will_be_visible = child_1.IsVisible || child_1_is_toward_single_node;

    if (child_0_is_or_will_be_visible && child_1_is_or_will_be_visible)
    {
        ImGuiContext& g = *GImGui;
        const float spacing = DOCKING_SPLITTER_SIZE;
        const ImGuiAxis axis = (ImGuiAxis)node.SplitAxis;
        const float size_avail = ImMax(size[axis] - spacing, 0.0);

        // size allocation policy
        // 1) The first 0..WindowMinSize[axis]*2 are allocated evenly to both windows.
        const float size_min_each = f32::floor(ImMin(size_avail, g.style.window_min_size[axis] * 2.0) * 0.5);

        // FIXME: Blocks 2) and 3) are essentially doing nearly the same thing.
        // Difference are: write-back to size_ref; application of a minimum size; rounding before f32::floor()
        // Clarify and rework differences between size & size_ref and purpose of WantLockSizeOnce

        // 2) Process locked absolute size (during a splitter resize we preserve the child of nodes not touching the splitter edge)
        if (child_0.WantLockSizeOnce && !child_1.WantLockSizeOnce)
        {
            child_0_size[axis] = child_0.sizeRef[axis] = ImMin(size_avail - 1.0, child_0.size[axis]);
            child_1_size[axis] = child_1.sizeRef[axis] = (size_avail - child_0_size[axis]);
            IM_ASSERT(child_0.sizeRef[axis] > 0.0 && child_1.sizeRef[axis] > 0.0);
        }
        else if (child_1.WantLockSizeOnce && !child_0.WantLockSizeOnce)
        {
            child_1_size[axis] = child_1.sizeRef[axis] = ImMin(size_avail - 1.0, child_1.size[axis]);
            child_0_size[axis] = child_0.sizeRef[axis] = (size_avail - child_1_size[axis]);
            IM_ASSERT(child_0.sizeRef[axis] > 0.0 && child_1.sizeRef[axis] > 0.0);
        }
        else if (child_0.WantLockSizeOnce && child_1.WantLockSizeOnce)
        {
            // FIXME-DOCK: We cannot honor the requested size, so apply ratio.
            // Currently this path will only be taken if code programmatically sets WantLockSizeOnce
            float split_ratio = child_0_size[axis] / (child_0_size[axis] + child_1_size[axis]);
            child_0_size[axis] = child_0.sizeRef[axis] = f32::floor(size_avail * split_ratio);
            child_1_size[axis] = child_1.sizeRef[axis] = (size_avail - child_0_size[axis]);
            IM_ASSERT(child_0.sizeRef[axis] > 0.0 && child_1.sizeRef[axis] > 0.0);
        }

        // 3) If one window is the central node (~ use remaining space, should be made explicit!), use explicit size from the other, and remainder for the central node
        else if (child_0.sizeRef[axis] != 0.0 && child_1.HasCentralNodeChild)
        {
            child_0_size[axis] = ImMin(size_avail - size_min_each, child_0.sizeRef[axis]);
            child_1_size[axis] = (size_avail - child_0_size[axis]);
        }
        else if (child_1.sizeRef[axis] != 0.0 && child_0.HasCentralNodeChild)
        {
            child_1_size[axis] = ImMin(size_avail - size_min_each, child_1.sizeRef[axis]);
            child_0_size[axis] = (size_avail - child_1_size[axis]);
        }
        else
        {
            // 4) Otherwise distribute according to the relative ratio of each size_ref value
            float split_ratio = child_0.sizeRef[axis] / (child_0.sizeRef[axis] + child_1.sizeRef[axis]);
            child_0_size[axis] = ImMax(size_min_each, f32::floor(size_avail * split_ratio + 0.5));
            child_1_size[axis] = (size_avail - child_0_size[axis]);
        }

        child_1_pos[axis] += spacing + child_0_size[axis];
    }

    if (only_write_to_single_node == NULL)
        child_0.WantLockSizeOnce = child_1.WantLockSizeOnce = false;

    const bool child_0_recurse = only_write_to_single_node ? child_0_is_toward_single_node : child_0.IsVisible;
    const bool child_1_recurse = only_write_to_single_node ? child_1_is_toward_single_node : child_1.IsVisible;
    if (child_0_recurse)
        DockNodeTreeUpdatePosSize(child_0, child_0_pos, child_0_size);
    if (child_1_recurse)
        DockNodeTreeUpdatePosSize(child_1, child_1_pos, child_1_size);
}

static void DockNodeTreeUpdateSplitterFindTouchingNode(ImGuiDockNode* node, ImGuiAxis axis, int side, ImVector<ImGuiDockNode*>* touching_nodes)
{
    if (node.IsLeafNode())
    {
        touching_nodes.push_back(node);
        return;
    }
    if (node.ChildNodes[0].IsVisible)
        if (node.SplitAxis != axis || side == 0 || !node.ChildNodes[1].IsVisible)
            DockNodeTreeUpdateSplitterFindTouchingNode(node.ChildNodes[0], axis, side, touching_nodes);
    if (node.ChildNodes[1].IsVisible)
        if (node.SplitAxis != axis || side == 1 || !node.ChildNodes[0].IsVisible)
            DockNodeTreeUpdateSplitterFindTouchingNode(node.ChildNodes[1], axis, side, touching_nodes);
}

// (Depth-First, Pre-Order)
void DockNodeTreeUpdateSplitter(ImGuiDockNode* node)
{
    if (node.IsLeafNode())
        return;

    ImGuiContext& g = *GImGui;

    ImGuiDockNode* child_0 = node.ChildNodes[0];
    ImGuiDockNode* child_1 = node.ChildNodes[1];
    if (child_0.IsVisible && child_1.IsVisible)
    {
        // Bounding box of the splitter cover the space between both nodes (w = Spacing, h = size[xy^1] for when splitting horizontally)
        const ImGuiAxis axis = (ImGuiAxis)node.SplitAxis;
        IM_ASSERT(axis != ImGuiAxis_None);
        Rect bb;
        bb.min = child_0.pos;
        bb.max = child_1.pos;
        bb.min[axis] += child_0.size[axis];
        bb.max[axis ^ 1] += child_1.size[axis ^ 1];
        //if (g.io.key_ctrl) GetForegroundDrawList(g.current_window->viewport)->add_rect(bb.min, bb.max, IM_COL32(255,0,255,255));

        const ImGuiDockNodeFlags merged_flags = child_0.MergedFlags | child_1.MergedFlags; // Merged flags for BOTH childs
        const ImGuiDockNodeFlags no_resize_axis_flag = (axis == Axis::X) ? ImGuiDockNodeFlags_NoResizeX : ImGuiDockNodeFlags_NoResizeY;
        if ((merged_flags & ImGuiDockNodeFlags_NoResize) || (merged_flags & no_resize_axis_flag))
        {
            ImGuiWindow* window = g.current_window;
            window.draw_list.add_rect_filled(bb.min, bb.max, get_color_u32(StyleColor::Separator), g.style.FrameRounding);
        }
        else
        {
            //bb.min[axis] += 1; // Display a little inward so highlight doesn't connect with nearby tabs on the neighbor node.
            //bb.max[axis] -= 1;
            PushID(node.ID);

            // Find resizing limits by gathering list of nodes that are touching the splitter line.
            ImVector<ImGuiDockNode*> touching_nodes[2];
            float min_size = g.style.window_min_size[axis];
            float resize_limits[2];
            resize_limits[0] = node.ChildNodes[0].pos[axis] + min_size;
            resize_limits[1] = node.ChildNodes[1].pos[axis] + node.ChildNodes[1].size[axis] - min_size;

            ImGuiID splitter_id = GetID("##Splitter");
            if (g.active_id == splitter_id) // Only process when splitter is active
            {
                DockNodeTreeUpdateSplitterFindTouchingNode(child_0, axis, 1, &touching_nodes[0]);
                DockNodeTreeUpdateSplitterFindTouchingNode(child_1, axis, 0, &touching_nodes[1]);
                for (int touching_node_n = 0; touching_node_n < touching_nodes[0].size; touching_node_n += 1)
                    resize_limits[0] = ImMax(resize_limits[0], touching_nodes[0][touching_node_n].rect().min[axis] + min_size);
                for (int touching_node_n = 0; touching_node_n < touching_nodes[1].size; touching_node_n += 1)
                    resize_limits[1] = ImMin(resize_limits[1], touching_nodes[1][touching_node_n].rect().max[axis] - min_size);

                // [DEBUG] Render touching nodes & limits
                /*
                ImDrawList* draw_list = node->host_window ? GetForegroundDrawList(node->host_window) : GetForegroundDrawList(GetMainViewport());
                for (int n = 0; n < 2; n++)
                {
                    for (int touching_node_n = 0; touching_node_n < touching_nodes[n].size; touching_node_n++)
                        draw_list->add_rect(touching_nodes[n][touching_node_n]->pos, touching_nodes[n][touching_node_n]->pos + touching_nodes[n][touching_node_n]->size, IM_COL32(0, 255, 0, 255));
                    if (axis == ImGuiAxis_X)
                        draw_list->add_line(Vector2D(resize_limits[n], node->child_nodes[n]->pos.y), Vector2D(resize_limits[n], node->child_nodes[n]->pos.y + node->child_nodes[n]->size.y), IM_COL32(255, 0, 255, 255), 3.0);
                    else
                        draw_list->add_line(Vector2D(node->child_nodes[n]->pos.x, resize_limits[n]), Vector2D(node->child_nodes[n]->pos.x + node->child_nodes[n]->size.x, resize_limits[n]), IM_COL32(255, 0, 255, 255), 3.0);
                }
                */
            }

            // Use a short delay before highlighting the splitter (and changing the mouse cursor) in order for regular mouse movement to not highlight many splitters
            float cur_size_0 = child_0.size[axis];
            float cur_size_1 = child_1.size[axis];
            float min_size_0 = resize_limits[0] - child_0.pos[axis];
            float min_size_1 = child_1.pos[axis] + child_1.size[axis] - resize_limits[1];
            ImU32 bg_col = get_color_u32(StyleColor::WindowBg);
            if (SplitterBehavior(bb, GetID("##Splitter"), axis, &cur_size_0, &cur_size_1, min_size_0, min_size_1, WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER, bg_col))
            {
                if (touching_nodes[0].size > 0 && touching_nodes[1].size > 0)
                {
                    child_0.size[axis] = child_0.sizeRef[axis] = cur_size_0;
                    child_1.pos[axis] -= cur_size_1 - child_1.size[axis];
                    child_1.size[axis] = child_1.sizeRef[axis] = cur_size_1;

                    // Lock the size of every node that is a sibling of the node we are touching
                    // This might be less desirable if we can merge sibling of a same axis into the same parental level.
                    for (int side_n = 0; side_n < 2; side_n += 1)
                        for (int touching_node_n = 0; touching_node_n < touching_nodes[side_n].size; touching_node_n += 1)
                        {
                            ImGuiDockNode* touching_node = touching_nodes[side_n][touching_node_n];
                            //ImDrawList* draw_list = node->host_window ? GetForegroundDrawList(node->host_window) : GetForegroundDrawList(GetMainViewport());
                            //draw_list->add_rect(touching_node->pos, touching_node->pos + touching_node->size, IM_COL32(255, 128, 0, 255));
                            while (touching_node.ParentNode != node)
                            {
                                if (touching_node.ParentNode.SplitAxis == axis)
                                {
                                    // Mark other node so its size will be preserved during the upcoming call to DockNodeTreeUpdatePosSize().
                                    ImGuiDockNode* node_to_preserve = touching_node.ParentNode.ChildNodes[side_n];
                                    node_to_preserve.WantLockSizeOnce = true;
                                    //draw_list->add_rect(touching_node->pos, touching_node->rect().max, IM_COL32(255, 0, 0, 255));
                                    //draw_list->add_rect_filled(node_to_preserve->pos, node_to_preserve->rect().max, IM_COL32(0, 255, 0, 100));
                                }
                                touching_node = touching_node.ParentNode;
                            }
                        }

                    DockNodeTreeUpdatePosSize(child_0, child_0.pos, child_0.size);
                    DockNodeTreeUpdatePosSize(child_1, child_1.pos, child_1.size);
                    MarkIniSettingsDirty();
                }
            }
            PopID();
        }
    }

    if (child_0.IsVisible)
        DockNodeTreeUpdateSplitter(child_0);
    if (child_1.IsVisible)
        DockNodeTreeUpdateSplitter(child_1);
}

ImGuiDockNode* DockNodeTreeFindFallbackLeafNode(ImGuiDockNode* node)
{
    if (node.IsLeafNode())
        return node;
    if (ImGuiDockNode* leaf_node = DockNodeTreeFindFallbackLeafNode(node.ChildNodes[0]))
        return leaf_node;
    if (ImGuiDockNode* leaf_node = DockNodeTreeFindFallbackLeafNode(node.ChildNodes[1]))
        return leaf_node;
    return NULL;
}

ImGuiDockNode* DockNodeTreeFindVisibleNodeByPos(ImGuiDockNode* node, Vector2D pos)
{
    if (!node.IsVisible)
        return NULL;

    const float dock_spacing = 0.0;// g.style.ItemInnerSpacing.x; // FIXME: Relation to DOCKING_SPLITTER_SIZE?
    Rect r(node.pos, node.pos + node.size);
    r.Expand(dock_spacing * 0.5);
    bool inside = r.Contains(pos);
    if (!inside)
        return NULL;

    if (node.IsLeafNode())
        return node;
    if (ImGuiDockNode* hovered_node = DockNodeTreeFindVisibleNodeByPos(node.ChildNodes[0], pos))
        return hovered_node;
    if (ImGuiDockNode* hovered_node = DockNodeTreeFindVisibleNodeByPos(node.ChildNodes[1], pos))
        return hovered_node;

    return NULL;
}

//-----------------------------------------------------------------------------
// Docking: Public Functions (SetWindowDock, DockSpace, DockSpaceOverViewport)
//-----------------------------------------------------------------------------
// - SetWindowDock() [Internal]
// - DockSpace()
// - DockSpaceOverViewport()
//-----------------------------------------------------------------------------

// [Internal] Called via SetNextWindowDockID()
void SetWindowDock(ImGuiWindow* window, ImGuiID dock_id, ImGuiCond cond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.set_window_dock_allow_flags & cond) == 0)
        return;
    window.set_window_dock_allow_flags &= ~(ImGuiCond_Once | Cond::FirstUseEver | ImGuiCond_Appearing);

    if (window.DockId == dock_id)
        return;

    // If the user attempt to set a dock id that is a split node, we'll dig within to find a suitable docking spot
    ImGuiContext* ctx = GImGui;
    if (ImGuiDockNode* new_node = DockContextFindNodeByID(ctx, dock_id))
        if (new_node.IsSplitNode())
        {
            // Policy: Find central node or latest focused node. We first move back to our root node.
            new_node = DockNodeGetRootNode(new_node);
            if (new_node.CentralNode)
            {
                IM_ASSERT(new_node.CentralNode.IsCentralNode());
                dock_id = new_node.CentralNode.ID;
            }
            else
            {
                dock_id = new_node.LastFocusedNodeId;
            }
        }

    if (window.DockId == dock_id)
        return;

    if (window.dock_node)
        DockNodeRemoveWindow(window.dock_node, window, 0);
    window.DockId = dock_id;
}

// Create an explicit dockspace node within an existing window. Also expose dock node flags and creates a central_node by default.
// The Central Node is always displayed even when empty and shrink/extend according to the requested size of its neighbors.
// DockSpace() needs to be submitted _before_ any window they can host. If you use a dockspace, submit it early in your app.
ImGuiID DockSpace(ImGuiID id, const Vector2D& size_arg, ImGuiDockNodeFlags flags, const ImGuiWindowClass* window_class)
{
    ImGuiContext* ctx = GImGui;
    ImGuiContext& g = *ctx;
    ImGuiWindow* window = GetCurrentWindow();
    if (!(g.io.config_flags & ImGuiConfigFlags_DockingEnable))
        return 0;

    // Early out if parent window is hidden/collapsed
    // This is faster but also DockNodeUpdateTabBar() relies on TabBarLayout() running (which won't if skip_items=true) to set NextSelectedTabId = 0). See #2960.
    // If for whichever reason this is causing problem we would need to ensure that DockNodeUpdateTabBar() ends up clearing NextSelectedTabId even if skip_items=true.
    if (window.skip_items)
        flags |= ImGuiDockNodeFlags_KeepAliveOnly;

    IM_ASSERT((flags & ImGuiDockNodeFlags_DockSpace) == 0);
    IM_ASSERT(id != 0);
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, id);
    if (!node)
    {
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x%08X created\n", id);
        node = DockContextAddNode(ctx, id);
        node.SetLocalFlags(ImGuiDockNodeFlags_CentralNode);
    }
    if (window_class && window_class.ClassId != node.WindowClass.ClassId)
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x%08X: setup window_class 0x%08X -> 0x%08X\n", id, node.WindowClass.ClassId, window_class.ClassId);
    node.SharedFlags = flags;
    node.WindowClass = window_class ? *window_class : ImGuiWindowClass();

    // When a DockSpace transitioned form implicit to explicit this may be called a second time
    // It is possible that the node has already been claimed by a docked window which appeared before the DockSpace() node, so we overwrite is_dock_space again.
    if (node.LastFrameActive == g.frame_count && !(flags & ImGuiDockNodeFlags_KeepAliveOnly))
    {
        IM_ASSERT(node.IsDockSpace() == false && "Cannot call DockSpace() twice a frame with the same id");
        node.SetLocalFlags(node.LocalFlags | ImGuiDockNodeFlags_DockSpace);
        return id;
    }
    node.SetLocalFlags(node.LocalFlags | ImGuiDockNodeFlags_DockSpace);

    // Keep alive mode, this is allow windows docked into this node so stay docked even if they are not visible
    if (flags & ImGuiDockNodeFlags_KeepAliveOnly)
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
    IM_ASSERT(size.x > 0.0 && size.y > 0.0);

    node.pos = window.dc.cursor_pos;
    node.size = node.sizeRef = size;
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
    node.OnlyNodeWithWindows = NULL;

    IM_ASSERT(node.IsRootNode());

    // We need to handle the rare case were a central node is missing.
    // This can happen if the node was first created manually with DockBuilderAddNode() but _without_ the ImGuiDockNodeFlags_Dockspace.
    // Doing it correctly would set the _CentralNode flags, which would then propagate according to subsequent split.
    // It would also be ambiguous to attempt to assign a central node while there are split nodes, so we wait until there's a single node remaining.
    // The specific sub-property of _CentralNode we are interested in recovering here is the "Don't delete when empty" property,
    // as it doesn't make sense for an empty dockspace to not have this property.
    if (node.IsLeafNode() && !node.IsCentralNode())
        node.SetLocalFlags(node.LocalFlags | ImGuiDockNodeFlags_CentralNode);

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
ImGuiID DockSpaceOverViewport(const ImGuiViewport* viewport, ImGuiDockNodeFlags dockspace_flags, const ImGuiWindowClass* window_class)
{
    if (viewport == NULL)
        viewport = GetMainViewport();

    SetNextWindowPos(viewport.WorkPos);
    set_next_window_size(viewport.work_size);
    SetNextWindowViewport(viewport.ID);

    ImGuiWindowFlags host_window_flags = 0;
    host_window_flags |= WindowFlags::NoTitleBar | WindowFlags::NoCollapse | WindowFlags::NoResize | WindowFlags::NoMove | WindowFlags::NoDocking;
    host_window_flags |= WindowFlags::NoBringToFrontOnFocus | WindowFlags::NoNavFocus;
    if (dockspace_flags & ImGuiDockNodeFlags_PassthruCentralNode)
        host_window_flags |= WindowFlags::NoBackground;

    char label[32];
    ImFormatString(label, IM_ARRAYSIZE(label), "DockSpaceViewport_%08X", viewport.ID);

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

//-----------------------------------------------------------------------------
// Docking: Builder Functions
//-----------------------------------------------------------------------------
// Very early end-user API to manipulate dock nodes.
// Only available in imgui_internal.h. Expect this API to change/break!
// It is expected that those functions are all called _before_ the dockspace node submission.
//-----------------------------------------------------------------------------
// - DockBuilderDockWindow()
// - DockBuilderGetNode()
// - DockBuilderSetNodePos()
// - DockBuilderSetNodeSize()
// - DockBuilderAddNode()
// - DockBuilderRemoveNode()
// - DockBuilderRemoveNodeChildNodes()
// - DockBuilderRemoveNodeDockedWindows()
// - DockBuilderSplitNode()
// - DockBuilderCopyNodeRec()
// - DockBuilderCopyNode()
// - DockBuilderCopyWindowSettings()
// - DockBuilderCopyDockSpace()
// - DockBuilderFinish()
//-----------------------------------------------------------------------------

void DockBuilderDockWindow(const char* window_name, ImGuiID node_id)
{
    // We don't preserve relative order of multiple docked windows (by clearing dock_order back to -1)
    ImGuiID window_id = ImHashStr(window_name);
    if (ImGuiWindow* window = FindWindowByID(window_id))
    {
        // Apply to created window
        SetWindowDock(window, node_id, Cond::Always);
        window.DockOrder = -1;
    }
    else
    {
        // Apply to settings
        ImGuiWindowSettings* settings = FindWindowSettings(window_id);
        if (settings == NULL)
            settings = CreateNewWindowSettings(window_name);
        settings.dock_id = node_id;
        settings.dock_order = -1;
    }
}

ImGuiDockNode* DockBuilderGetNode(ImGuiID node_id)
{
    ImGuiContext* ctx = GImGui;
    return DockContextFindNodeByID(ctx, node_id);
}

void DockBuilderSetNodePos(ImGuiID node_id, Vector2D pos)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, node_id);
    if (node == NULL)
        return;
    node.pos = pos;
    node.AuthorityForPos = ImGuiDataAuthority_DockNode;
}

void DockBuilderSetNodeSize(ImGuiID node_id, Vector2D size)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, node_id);
    if (node == NULL)
        return;
    IM_ASSERT(size.x > 0.0 && size.y > 0.0);
    node.size = node.sizeRef = size;
    node.AuthorityForSize = ImGuiDataAuthority_DockNode;
}

// Make sure to use the ImGuiDockNodeFlags_DockSpace flag to create a dockspace node! Otherwise this will create a floating node!
// - Floating node: you can then call DockBuilderSetNodePos()/DockBuilderSetNodeSize() to position and size the floating node.
// - Dockspace node: calling DockBuilderSetNodePos() is unnecessary.
// - If you intend to split a node immediately after creation using DockBuilderSplitNode(), make sure to call DockBuilderSetNodeSize() beforehand!
//   For various reason, the splitting code currently needs a base size otherwise space may not be allocated as precisely as you would expect.
// - Use (id == 0) to let the system allocate a node identifier.
// - Existing node with a same id will be removed.
ImGuiID DockBuilderAddNode(ImGuiID id, ImGuiDockNodeFlags flags)
{
    ImGuiContext* ctx = GImGui;

    if (id != 0)
        DockBuilderRemoveNode(id);

    ImGuiDockNode* node = NULL;
    if (flags & ImGuiDockNodeFlags_DockSpace)
    {
        DockSpace(id, Vector2D::new(0, 0), (flags & ~ImGuiDockNodeFlags_DockSpace) | ImGuiDockNodeFlags_KeepAliveOnly);
        node = DockContextFindNodeByID(ctx, id);
    }
    else
    {
        node = DockContextAddNode(ctx, id);
        node.SetLocalFlags(flags);
    }
    node.LastFrameAlive = ctx.frame_count;   // Set this otherwise BeginDocked will undock during the same frame.
    return node.ID;
}

void DockBuilderRemoveNode(ImGuiID node_id)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, node_id);
    if (node == NULL)
        return;
    DockBuilderRemoveNodeDockedWindows(node_id, true);
    DockBuilderRemoveNodeChildNodes(node_id);
    // Node may have moved or deleted if e.g. any merge happened
    node = DockContextFindNodeByID(ctx, node_id);
    if (node == NULL)
        return;
    if (node.IsCentralNode() && node.ParentNode)
        node.ParentNode.SetLocalFlags(node.ParentNode.LocalFlags | ImGuiDockNodeFlags_CentralNode);
    DockContextRemoveNode(ctx, node, true);
}

// root_id = 0 to remove all, root_id != 0 to remove child of given node.
void DockBuilderRemoveNodeChildNodes(ImGuiID root_id)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockContext* dc  = &ctx.DockContext;

    ImGuiDockNode* root_node = root_id ? DockContextFindNodeByID(ctx, root_id) : NULL;
    if (root_id && root_node == NULL)
        return;
    bool has_central_node = false;

    ImGuiDataAuthority backup_root_node_authority_for_pos = root_node ? root_node.AuthorityForPos : ImGuiDataAuthority_Auto;
    ImGuiDataAuthority backup_root_node_authority_for_size = root_node ? root_node.AuthorityForSize : ImGuiDataAuthority_Auto;

    // Process active windows
    ImVector<ImGuiDockNode*> nodes_to_remove;
    for (int n = 0; n < dc.Nodes.data.size; n += 1)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc.Nodes.data[n].val_p)
        {
            bool want_removal = (root_id == 0) || (node.ID != root_id && DockNodeGetRootNode(node).ID == root_id);
            if (want_removal)
            {
                if (node.IsCentralNode())
                    has_central_node = true;
                if (root_id != 0)
                    DockContextQueueNotifyRemovedNode(ctx, node);
                if (root_node)
                {
                    DockNodeMoveWindows(root_node, node);
                    DockSettingsRenameNodeReferences(node.ID, root_node.ID);
                }
                nodes_to_remove.push_back(node);
            }
        }

    // DockNodeMoveWindows->DockNodeAddWindow will normally set those when reaching two windows (which is only adequate during interactive merge)
    // Make sure we don't lose our current pos/size. (FIXME-DOCK: Consider tidying up that code in DockNodeAddWindow instead)
    if (root_node)
    {
        root_node.AuthorityForPos = backup_root_node_authority_for_pos;
        root_node.AuthorityForSize = backup_root_node_authority_for_size;
    }

    // Apply to settings
    for (ImGuiWindowSettings* settings = ctx.SettingsWindows.begin(); settings != NULL; settings = ctx.SettingsWindows.next_chunk(settings))
        if (ImGuiID window_settings_dock_id = settings.dock_id)
            for (int n = 0; n < nodes_to_remove.size; n += 1)
                if (nodes_to_remove[n].ID == window_settings_dock_id)
                {
                    settings.dock_id = root_id;
                    break;
                }

    // Not really efficient, but easier to destroy a whole hierarchy considering DockContextRemoveNode is attempting to merge nodes
    if (nodes_to_remove.size > 1)
        ImQsort(nodes_to_remove.data, nodes_to_remove.size, sizeof(ImGuiDockNode*), DockNodeComparerDepthMostFirst);
    for (int n = 0; n < nodes_to_remove.size; n += 1)
        DockContextRemoveNode(ctx, nodes_to_remove[n], false);

    if (root_id == 0)
    {
        dc.Nodes.Clear();
        dc.Requests.clear();
    }
    else if (has_central_node)
    {
        root_node.CentralNode = root_node;
        root_node.SetLocalFlags(root_node.LocalFlags | ImGuiDockNodeFlags_CentralNode);
    }
}

void DockBuilderRemoveNodeDockedWindows(ImGuiID root_id, bool clear_settings_refs)
{
    // clear references in settings
    ImGuiContext* ctx = GImGui;
    ImGuiContext& g = *ctx;
    if (clear_settings_refs)
    {
        for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
        {
            bool want_removal = (root_id == 0) || (settings.dock_id == root_id);
            if (!want_removal && settings.dock_id != 0)
                if (ImGuiDockNode* node = DockContextFindNodeByID(ctx, settings.dock_id))
                    if (DockNodeGetRootNode(node).ID == root_id)
                        want_removal = true;
            if (want_removal)
                settings.dock_id = 0;
        }
    }

    // clear references in windows
    for (int n = 0; n < g.windows.size; n += 1)
    {
        ImGuiWindow* window = g.windows[n];
        bool want_removal = (root_id == 0) || (window.dock_node && DockNodeGetRootNode(window.dock_node).ID == root_id) || (window.DockNodeAsHost && window.DockNodeAsHost.ID == root_id);
        if (want_removal)
        {
            const ImGuiID backup_dock_id = window.DockId;
            IM_UNUSED(backup_dock_id);
            DockContextProcessUndockWindow(ctx, window, clear_settings_refs);
            if (!clear_settings_refs)
                IM_ASSERT(window.DockId == backup_dock_id);
        }
    }
}

// If 'out_id_at_dir' or 'out_id_at_opposite_dir' are non NULL, the function will write out the id of the two new nodes created.
// Return value is id of the node at the specified direction, so same as (*out_id_at_dir) if that pointer is set.
// FIXME-DOCK: We are not exposing nor using split_outer.
ImGuiID DockBuilderSplitNode(ImGuiID id, ImGuiDir split_dir, float size_ratio_for_node_at_dir, ImGuiID* out_id_at_dir, ImGuiID* out_id_at_opposite_dir)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(split_dir != Dir::None);
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockBuilderSplitNode: node 0x%08X, split_dir %d\n", id, split_dir);

    ImGuiDockNode* node = DockContextFindNodeByID(&g, id);
    if (node == NULL)
    {
        IM_ASSERT(node != NULL);
        return 0;
    }

    IM_ASSERT(!node.IsSplitNode()); // Assert if already split

    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Split;
    req.DockTargetWindow = NULL;
    req.DockTargetNode = node;
    req.DockPayload = NULL;
    req.DockSplitDir = split_dir;
    req.DockSplitRatio = ImSaturate((split_dir == Dir::Left || split_dir == Dir::Up) ? size_ratio_for_node_at_dir : 1.0 - size_ratio_for_node_at_dir);
    req.DockSplitOuter = false;
    DockContextProcessDock(&g, &req);

    ImGuiID id_at_dir = node.ChildNodes[(split_dir == Dir::Left || split_dir == Dir::Up) ? 0 : 1].ID;
    ImGuiID id_at_opposite_dir = node.ChildNodes[(split_dir == Dir::Left || split_dir == Dir::Up) ? 1 : 0].ID;
    if (out_id_at_dir)
        *out_id_at_dir = id_at_dir;
    if (out_id_at_opposite_dir)
        *out_id_at_opposite_dir = id_at_opposite_dir;
    return id_at_dir;
}

static ImGuiDockNode* DockBuilderCopyNodeRec(ImGuiDockNode* src_node, ImGuiID dst_node_id_if_known, ImVector<ImGuiID>* out_node_remap_pairs)
{
    ImGuiContext& g = *GImGui;
    ImGuiDockNode* dst_node = DockContextAddNode(&g, dst_node_id_if_known);
    dst_node.SharedFlags = src_node.SharedFlags;
    dst_node.LocalFlags = src_node.LocalFlags;
    dst_node.LocalFlagsInWindows = ImGuiDockNodeFlags_None;
    dst_node.pos = src_node.pos;
    dst_node.size = src_node.size;
    dst_node.sizeRef = src_node.sizeRef;
    dst_node.SplitAxis = src_node.SplitAxis;
    dst_node.UpdateMergedFlags();

    out_node_remap_pairs.push_back(src_node.ID);
    out_node_remap_pairs.push_back(dst_node.ID);

    for (int child_n = 0; child_n < IM_ARRAYSIZE(src_node.ChildNodes); child_n += 1)
        if (src_node.ChildNodes[child_n])
        {
            dst_node.ChildNodes[child_n] = DockBuilderCopyNodeRec(src_node.ChildNodes[child_n], 0, out_node_remap_pairs);
            dst_node.ChildNodes[child_n].ParentNode = dst_node;
        }

    IMGUI_DEBUG_LOG_DOCKING("[docking] Fork node %08X -> %08X (%d childs)\n", src_node.ID, dst_node.ID, dst_node.IsSplitNode() ? 2 : 0);
    return dst_node;
}

void DockBuilderCopyNode(ImGuiID src_node_id, ImGuiID dst_node_id, ImVector<ImGuiID>* out_node_remap_pairs)
{
    ImGuiContext* ctx = GImGui;
    IM_ASSERT(src_node_id != 0);
    IM_ASSERT(dst_node_id != 0);
    IM_ASSERT(out_node_remap_pairs != NULL);

    DockBuilderRemoveNode(dst_node_id);

    ImGuiDockNode* src_node = DockContextFindNodeByID(ctx, src_node_id);
    IM_ASSERT(src_node != NULL);

    out_node_remap_pairs.clear();
    DockBuilderCopyNodeRec(src_node, dst_node_id, out_node_remap_pairs);

    IM_ASSERT((out_node_remap_pairs.size % 2) == 0);
}

void DockBuilderCopyWindowSettings(const char* src_name, const char* dst_name)
{
    ImGuiWindow* src_window = FindWindowByName(src_name);
    if (src_window == NULL)
        return;
    if (ImGuiWindow* dst_window = FindWindowByName(dst_name))
    {
        dst_window.Pos = src_window.Pos;
        dst_window.size = src_window.size;
        dst_window.sizeFull = src_window.sizeFull;
        dst_window.collapsed = src_window.collapsed;
    }
    else if (ImGuiWindowSettings* dst_settings = FindOrCreateWindowSettings(dst_name))
    {
        Vector2Dih window_pos_2ih = Vector2Dih(src_window.Pos);
        if (src_window.viewport_id != 0 && src_window.viewport_id != IMGUI_VIEWPORT_DEFAULT_ID)
        {
            dst_settings.viewport_pos = window_pos_2ih;
            dst_settings.viewport_id = src_window.viewport_id;
            dst_settings.pos = Vector2Dih(0, 0);
        }
        else
        {
            dst_settings.pos = window_pos_2ih;
        }
        dst_settings.size = Vector2Dih(src_window.sizeFull);
        dst_settings.collapsed = src_window.collapsed;
    }
}

// FIXME: Will probably want to change this signature, in particular how the window remapping pairs are passed.
void DockBuilderCopyDockSpace(ImGuiID src_dockspace_id, ImGuiID dst_dockspace_id, ImVector<const char*>* in_window_remap_pairs)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(src_dockspace_id != 0);
    IM_ASSERT(dst_dockspace_id != 0);
    IM_ASSERT(in_window_remap_pairs != NULL);
    IM_ASSERT((in_window_remap_pairs.size % 2) == 0);

    // Duplicate entire dock
    // FIXME: When overwriting dst_dockspace_id, windows that aren't part of our dockspace window class but that are docked in a same node will be split apart,
    // whereas we could attempt to at least keep them together in a new, same floating node.
    ImVector<ImGuiID> node_remap_pairs;
    DockBuilderCopyNode(src_dockspace_id, dst_dockspace_id, &node_remap_pairs);

    // Attempt to transition all the upcoming windows associated to dst_dockspace_id into the newly created hierarchy of dock nodes
    // (The windows associated to src_dockspace_id are staying in place)
    ImVector<ImGuiID> src_windows;
    for (int remap_window_n = 0; remap_window_n < in_window_remap_pairs.size; remap_window_n += 2)
    {
        const char* src_window_name = (*in_window_remap_pairs)[remap_window_n];
        const char* dst_window_name = (*in_window_remap_pairs)[remap_window_n + 1];
        ImGuiID src_window_id = ImHashStr(src_window_name);
        src_windows.push_back(src_window_id);

        // Search in the remapping tables
        ImGuiID src_dock_id = 0;
        if (ImGuiWindow* src_window = FindWindowByID(src_window_id))
            src_dock_id = src_window.DockId;
        else if (ImGuiWindowSettings* src_window_settings = FindWindowSettings(src_window_id))
            src_dock_id = src_window_settings.dock_id;
        ImGuiID dst_dock_id = 0;
        for (int dock_remap_n = 0; dock_remap_n < node_remap_pairs.size; dock_remap_n += 2)
            if (node_remap_pairs[dock_remap_n] == src_dock_id)
            {
                dst_dock_id = node_remap_pairs[dock_remap_n + 1];
                //node_remap_pairs[dock_remap_n] = node_remap_pairs[dock_remap_n + 1] = 0; // clear
                break;
            }

        if (dst_dock_id != 0)
        {
            // Docked windows gets redocked into the new node hierarchy.
            IMGUI_DEBUG_LOG_DOCKING("[docking] Remap live window '%s' 0x%08X -> '%s' 0x%08X\n", src_window_name, src_dock_id, dst_window_name, dst_dock_id);
            DockBuilderDockWindow(dst_window_name, dst_dock_id);
        }
        else
        {
            // Floating windows gets their settings transferred (regardless of whether the new window already exist or not)
            // When this is leading to a Copy and not a Move, we would get two overlapping floating windows. Could we possibly dock them together?
            IMGUI_DEBUG_LOG_DOCKING("[docking] Remap window settings '%s' -> '%s'\n", src_window_name, dst_window_name);
            DockBuilderCopyWindowSettings(src_window_name, dst_window_name);
        }
    }

    // Anything else in the source nodes of 'node_remap_pairs' are windows that were docked in src_dockspace_id but are not owned by it (unaffiliated windows, e.g. "ImGui Demo")
    // Find those windows and move to them to the cloned dock node. This may be optional?
    for (int dock_remap_n = 0; dock_remap_n < node_remap_pairs.size; dock_remap_n += 2)
        if (ImGuiID src_dock_id = node_remap_pairs[dock_remap_n])
        {
            ImGuiID dst_dock_id = node_remap_pairs[dock_remap_n + 1];
            ImGuiDockNode* node = DockBuilderGetNode(src_dock_id);
            for (int window_n = 0; window_n < node.Windows.size; window_n += 1)
            {
                ImGuiWindow* window = node.Windows[window_n];
                if (src_windows.contains(window.id))
                    continue;

                // Docked windows gets redocked into the new node hierarchy.
                IMGUI_DEBUG_LOG_DOCKING("[docking] Remap window '%s' %08X -> %08X\n", window.Name, src_dock_id, dst_dock_id);
                DockBuilderDockWindow(window.Name, dst_dock_id);
            }
        }
}

// FIXME-DOCK: This is awkward because in series of split user is likely to loose access to its root node.
void DockBuilderFinish(ImGuiID root_id)
{
    ImGuiContext* ctx = GImGui;
    //DockContextRebuild(ctx);
    DockContextBuildAddWindowsToNodes(ctx, root_id);
}

//-----------------------------------------------------------------------------
// Docking: Begin/End Support Functions (called from Begin/End)
//-----------------------------------------------------------------------------
// - GetWindowAlwaysWantOwnTabBar()
// - DockContextBindNodeToWindow()
// - BeginDocked()
// - BeginDockableDragDropSource()
// - BeginDockableDragDropTarget()
//-----------------------------------------------------------------------------

bool GetWindowAlwaysWantOwnTabBar(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    if (g.io.ConfigDockingAlwaysTabBar || window.WindowClass.DockingAlwaysTabBar)
        if ((window.flags & (WindowFlags::ChildWindow | WindowFlags::NoTitleBar | WindowFlags::NoDocking)) == 0)
            if (!window.IsFallbackWindow)    // We don't support AlwaysTabBar on the fallback/implicit window to avoid unused dock-node overhead/noise
                return true;
    return false;
}

static ImGuiDockNode* DockContextBindNodeToWindow(ImGuiContext* ctx, ImGuiWindow* window)
{
    ImGuiContext& g = *ctx;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, window.DockId);
    IM_ASSERT(window.dock_node == NULL);

    // We should not be docking into a split node (SetWindowDock should avoid this)
    if (node && node.IsSplitNode())
    {
        DockContextProcessUndockWindow(ctx, window);
        return NULL;
    }

    // Create node
    if (node == NULL)
    {
        node = DockContextAddNode(ctx, window.DockId);
        node.AuthorityForPos = node.AuthorityForSize = node.AuthorityForViewport = ImGuiDataAuthority_Window;
        node.LastFrameAlive = g.frame_count;
    }

    // If the node just turned visible and is part of a hierarchy, it doesn't have a size assigned by DockNodeTreeUpdatePosSize() yet,
    // so we're forcing a pos/size update from the first ancestor that is already visible (often it will be the root node).
    // If we don't do this, the window will be assigned a zero-size on its first frame, which won't ideally warm up the layout.
    // This is a little wonky because we don't normally update the pos/size of visible node mid-frame.
    if (!node.IsVisible)
    {
        ImGuiDockNode* ancestor_node = node;
        while (!ancestor_node.IsVisible && ancestor_node.ParentNode)
            ancestor_node = ancestor_node.ParentNode;
        IM_ASSERT(ancestor_node.size.x > 0.0 && ancestor_node.size.y > 0.0);
        DockNodeUpdateHasCentralNodeChild(DockNodeGetRootNode(ancestor_node));
        DockNodeTreeUpdatePosSize(ancestor_node, ancestor_node.pos, ancestor_node.size, node);
    }

    // Add window to node
    bool node_was_visible = node.IsVisible;
    DockNodeAddWindow(node, window, true);
    node.IsVisible = node_was_visible; // Don't mark visible right away (so DockContextEndFrame() doesn't render it, maybe other side effects? will see)
    IM_ASSERT(node == window.dock_node);
    return node;
}

void BeginDocked(ImGuiWindow* window, bool* p_open)
{
    ImGuiContext* ctx = GImGui;
    ImGuiContext& g = *ctx;

    // clear fields ahead so most early-out paths don't have to do it
    window.dock_is_active = window.DockNodeIsVisible = window.DockTabIsVisible = false;

    const bool auto_dock_node = GetWindowAlwaysWantOwnTabBar(window);
    if (auto_dock_node)
    {
        if (window.DockId == 0)
        {
            IM_ASSERT(window.dock_node == NULL);
            window.DockId = DockContextGenNodeID(ctx);
        }
    }
    else
    {
        // Calling SetNextWindowPos() undock windows by default (by setting PosUndock)
        bool want_undock = false;
        want_undock |= (window.flags & WindowFlags::NoDocking) != 0;
        want_undock |= (g.next_window_data.flags & NextWindowDataFlags::HasPos) && (window.set_window_pos_allow_flags & g.next_window_data.PosCond) && g.next_window_data.PosUndock;
        if (want_undock)
        {
            DockContextProcessUndockWindow(ctx, window);
            return;
        }
    }

    // Bind to our dock node
    ImGuiDockNode* node = window.dock_node;
    if (node != NULL)
        IM_ASSERT(window.DockId == node.ID);
    if (window.DockId != 0 && node == NULL)
    {
        node = DockContextBindNodeToWindow(ctx, window);
        if (node == NULL)
            return;
    }

#if 0
    // Undock if the ImGuiDockNodeFlags_NoDockingInCentralNode got set
    if (node.IsCentralNode && (node.flags & ImGuiDockNodeFlags_NoDockingInCentralNode))
    {
        DockContextProcessUndockWindow(ctx, window);
        return;
    }


    // Undock if our dockspace node disappeared
    // Note how we are testing for last_frame_alive and NOT last_frame_active. A DockSpace node can be maintained alive while being inactive with ImGuiDockNodeFlags_KeepAliveOnly.
    if (node.LastFrameAlive < g.frame_count)
    {
        // If the window has been orphaned, transition the docknode to an implicit node processed in DockContextNewFrameUpdateDocking()
        ImGuiDockNode* root_node = DockNodeGetRootNode(node);
        if (root_node.LastFrameAlive < g.frame_count)
            DockContextProcessUndockWindow(ctx, window);
        else
            window.dock_is_active = true;
        return;
    }

    // Store style overrides
    for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
        window.DockStyle.colors[color_n] = ColorConvertFloat4ToU32(g.style.colors[GWindowDockStyleColors[color_n]]);

    // Fast path return. It is common for windows to hold on a persistent dock_id but be the only visible window,
    // and never create neither a host window neither a tab bar.
    // FIXME-DOCK: replace ->host_window NULL compare with something more explicit (~was initially intended as a first frame test)
    if (node.host_window == NULL)
    {
        if (node.State == ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing)
            window.dock_is_active = true;
        if (node.Windows.size > 1)
            DockNodeHideWindowDuringHostWindowCreation(window);
        return;
    }

    // We can have zero-sized nodes (e.g. children of a small-size dockspace)
    IM_ASSERT(node.host_window);
    IM_ASSERT(node.IsLeafNode());
    IM_ASSERT(node.size.x >= 0.0 && node.size.y >= 0.0);
    node.State = ImGuiDockNodeState_HostWindowVisible;

    // Undock if we are submitted earlier than the host window
    if (!(node.MergedFlags & ImGuiDockNodeFlags_KeepAliveOnly) && window.BeginOrderWithinContext < node.host_window.BeginOrderWithinContext)
    {
        DockContextProcessUndockWindow(ctx, window);
        return;
    }

    // Position/size window
    SetNextWindowPos(node.pos);
    set_next_window_size(node.size);
    g.next_window_data.PosUndock = false; // Cancel implicit undocking of SetNextWindowPos()
    window.dock_is_active = true;
    window.DockNodeIsVisible = true;
    window.DockTabIsVisible = false;
    if (node.MergedFlags & ImGuiDockNodeFlags_KeepAliveOnly)
        return;

    // When the window is selected we mark it as visible.
    if (node.VisibleWindow == window)
        window.DockTabIsVisible = true;

    // Update window flag
    IM_ASSERT((window.flags & WindowFlags::ChildWindow) == 0);
    window.flags |= WindowFlags::ChildWindow | WindowFlags::AlwaysUseWindowPadding | WindowFlags::NoResize;
    if (node.is_hidden_tab_bar() || node.is_no_tab_bar())
        window.flags |= WindowFlags::NoTitleBar;
    else
        window.flags &= ~WindowFlags::NoTitleBar;      // clear the NoTitleBar flag in case the user set it: confusingly enough we need a title bar height so we are correctly offset, but it won't be displayed!

    // Save new dock order only if the window has been visible once already
    // This allows multiple windows to be created in the same frame and have their respective dock orders preserved.
    if (node.TabBar && window.was_active)
        window.DockOrder = DockNodeGetTabOrder(window);

    if ((node.WantCloseAll || node.WantCloseTabId == window.TabId) && p_open != NULL)
        *p_open = false;

    // Update child_id to allow returning from Child to Parent with Escape
    ImGuiWindow* parent_window = window.dock_node.host_window;
    windowchild_id = parent_window.get_id(window.Name);
}

void BeginDockableDragDropSource(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.active_id == window.move_id);
    IM_ASSERT(g.moving_window == window);
    IM_ASSERT(g.current_window == window);

    g.last_item_data.id = window.move_id;
    window = window.root_window_dock_tree;
    IM_ASSERT((window.flags & WindowFlags::NoDocking) == 0);
    bool is_drag_docking = (g.io.ConfigDockingWithShift) || Rect(0, 0, window.size_full.x, GetFrameHeight()).Contains(g.ActiveIdClickOffset); // FIXME-DOCKING: Need to make this stateful and explicit
    if (is_drag_docking && BeginDragDropSource(DragDropFlags::SourceNoPreviewTooltip | ImGuiDragDropFlags_SourceNoHoldToOpenOthers | ImGuiDragDropFlags_SourceAutoExpirePayload))
    {
        SetDragDropPayload(IMGUI_PAYLOAD_TYPE_WINDOW, &window, sizeof(window));
        EndDragDropSource();

        // Store style overrides
        for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
            window.DockStyle.colors[color_n] = ColorConvertFloat4ToU32(g.style.colors[GWindowDockStyleColors[color_n]]);
    }
}

void BeginDockableDragDropTarget(ImGuiWindow* window)
{
    ImGuiContext* ctx = GImGui;
    ImGuiContext& g = *ctx;

    //IM_ASSERT(window->root_window_dock_tree == window); // May also be a DockSpace
    IM_ASSERT((window.flags & WindowFlags::NoDocking) == 0);
    if (!g.drag_drop_active)
        return;
    //GetForegroundDrawList(window)->add_rect(window->pos, window->pos + window->size, IM_COL32(255, 255, 0, 255));
    if (!BeginDragDropTargetCustom(window.Rect(), window.id))
        return;

    // Peek into the payload before calling AcceptDragDropPayload() so we can handle overlapping dock nodes with filtering
    // (this is a little unusual pattern, normally most code would call AcceptDragDropPayload directly)
    const ImGuiPayload* payload = &g.drag_drop_payload;
    if (!payload.IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) || !DockNodeIsDropAllowed(window, *(ImGuiWindow**)payload.Data))
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
        if (window.DockNodeAsHost)
        {
            // Cannot assume that node will != NULL even though we passed the rectangle test: it depends on padding/spacing handled by DockNodeTreeFindVisibleNodeByPos().
            node = DockNodeTreeFindVisibleNodeByPos(window.DockNodeAsHost, g.io.mouse_pos);

            // There is an edge case when docking into a dockspace which only has _inactive_ nodes (because none of the windows are active)
            // In this case we need to fallback into any leaf mode, possibly the central node.
            // FIXME-20181220: We should not have to test for is_leaf_node() here but we have another bug to fix first.
            if (node && node.IsDockSpace() && node.IsRootNode())
                node = (node.CentralNode && node.IsLeafNode()) ? node.CentralNode : DockNodeTreeFindFallbackLeafNode(node);
        }
        else
        {
            if (window.dock_node)
                node = window.dock_node;
            else
                dock_into_floating_window = true; // Dock into a regular window
        }

        const Rect explicit_target_rect = (node && node.TabBar && !node.is_hidden_tab_bar() && !node.is_no_tab_bar()) ? node.TabBar.BarRect : Rect(window.pos, window.pos + Vector2D::new(window.size.x, GetFrameHeight()));
        const bool is_explicit_target = g.io.ConfigDockingWithShift || IsMouseHoveringRect(explicit_target_rect.min, explicit_target_rect.max);

        // preview docking request and find out split direction/ratio
        //const bool do_preview = true;     // Ignore testing for payload->is_preview() which removes one frame of delay, but breaks overlapping drop targets within the same window.
        const bool do_preview = payload.IsPreview() || payload.IsDelivery();
        if (do_preview && (node != NULL || dock_into_floating_window))
        {
            ImGuiDockPreviewData split_inner;
            ImGuiDockPreviewData split_outer;
            ImGuiDockPreviewData* split_data = &split_inner;
            if (node && (node.ParentNode || node.IsCentralNode()))
                if (ImGuiDockNode* root_node = DockNodeGetRootNode(node))
                {
                    DockNodePreviewDockSetup(window, root_node, payload_window, &split_outer, is_explicit_target, true);
                    if (split_outer.IsSplitDirExplicit)
                        split_data = &split_outer;
                }
            DockNodePreviewDockSetup(window, node, payload_window, &split_inner, is_explicit_target, false);
            if (split_data == &split_outer)
                split_inner.IsDropAllowed = false;

            // Draw inner then outer, so that previewed tab (in inner data) will be behind the outer drop boxes
            DockNodePreviewDockRender(window, node, payload_window, &split_inner);
            DockNodePreviewDockRender(window, node, payload_window, &split_outer);

            // Queue docking request
            if (split_data.IsDropAllowed && payload.IsDelivery())
                DockContextQueueDock(ctx, window, split_data.SplitNode, payload_window, split_data.SplitDir, split_data.SplitRatio, split_data == &split_outer);
        }
    }
    EndDragDropTarget();
}

//-----------------------------------------------------------------------------
// Docking: Settings
//-----------------------------------------------------------------------------
// - DockSettingsRenameNodeReferences()
// - DockSettingsRemoveNodeReferences()
// - DockSettingsFindNodeSettings()
// - DockSettingsHandler_ApplyAll()
// - DockSettingsHandler_ReadOpen()
// - DockSettingsHandler_ReadLine()
// - DockSettingsHandler_DockNodeToSettings()
// - DockSettingsHandler_WriteAll()
//-----------------------------------------------------------------------------

static void DockSettingsRenameNodeReferences(ImGuiID old_node_id, ImGuiID new_node_id)
{
    ImGuiContext& g = *GImGui;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockSettingsRenameNodeReferences: from 0x%08X -> to 0x%08X\n", old_node_id, new_node_id);
    for (int window_n = 0; window_n < g.windows.size; window_n += 1)
    {
        ImGuiWindow* window = g.windows[window_n];
        if (window.DockId == old_node_id && window.dock_node == NULL)
            window.DockId = new_node_id;
    }
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
        if (settings.dock_id == old_node_id)
            settings.dock_id = new_node_id;
}

// Remove references stored in ImGuiWindowSettings to the given ImGuiDockNodeSettings
static void DockSettingsRemoveNodeReferences(ImGuiID* node_ids, int node_ids_count)
{
    ImGuiContext& g = *GImGui;
    int found = 0;
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
        for (int node_n = 0; node_n < node_ids_count; node_n += 1)
            if (settings.dock_id == node_ids[node_n])
            {
                settings.dock_id = 0;
                settings.dock_order = -1;
                if (found += 1 < node_ids_count)
                    break;
                return;
            }
}

static ImGuiDockNodeSettings* DockSettingsFindNodeSettings(ImGuiContext* ctx, ImGuiID id)
{
    // FIXME-OPT
    ImGuiDockContext* dc  = &ctx.DockContext;
    for (int n = 0; n < dc.NodesSettings.size; n += 1)
        if (dc.NodesSettings[n].id == id)
            return &dc.NodesSettings[n];
    return NULL;
}

// clear settings data
static void DockSettingsHandler_ClearAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
{
    ImGuiDockContext* dc  = &ctx.DockContext;
    dc.NodesSettings.clear();
    DockContextClearNodes(ctx, 0, true);
}

// Recreate nodes based on settings data
static void DockSettingsHandler_ApplyAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
{
    // Prune settings at boot time only
    ImGuiDockContext* dc  = &ctx.DockContext;
    if (ctx.Windows.size == 0)
        DockContextPruneUnusedSettingsNodes(ctx);
    DockContextBuildNodesFromSettings(ctx, dc.NodesSettings.data, dc.NodesSettings.size);
    DockContextBuildAddWindowsToNodes(ctx, 0);
}

static void* DockSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, const char* name)
{
    if (strcmp(name, "data") != 0)
        return NULL;
    return (void*)1;
}

static void DockSettingsHandler_ReadLine(ImGuiContext* ctx, ImGuiSettingsHandler*, void*, const char* line)
{
    char c = 0;
    int x = 0, y = 0;
    int r = 0;

    // Parsing, e.g.
    // " dock_node   id=0x00000001 pos=383,193 size=201,322 split=Y,0.506 "
    // "   dock_node id=0x00000002 Parent=0x00000001 "
    // Important: this code expect currently fields in a fixed order.
    ImGuiDockNodeSettings node;
    line = ImStrSkipBlank(line);
    if      (strncmp(line, "dock_node", 8) == 0)  { line = ImStrSkipBlank(line + strlen("dock_node")); }
    else if (strncmp(line, "DockSpace", 9) == 0) { line = ImStrSkipBlank(line + strlen("DockSpace")); node.flags |= ImGuiDockNodeFlags_DockSpace; }
    else return;
    if (sscanf(line, "id=0x%08X%n",      &node.id, &r) == 1)            { line += r; } else return;
    if (sscanf(line, " Parent=0x%08X%n", &node.ParentNodeId, &r) == 1)  { line += r; if (node.ParentNodeId == 0) return; }
    if (sscanf(line, " window=0x%08X%n", &node.ParentWindowId, &r) ==1) { line += r; if (node.ParentWindowId == 0) return; }
    if (node.ParentNodeId == 0)
    {
        if (sscanf(line, " pos=%i,%i%n",  &x, &y, &r) == 2)         { line += r; node.Pos = Vector2Dih(x, y); } else return;
        if (sscanf(line, " size=%i,%i%n", &x, &y, &r) == 2)         { line += r; node.size = Vector2Dih(x, y); } else return;
    }
    else
    {
        if (sscanf(line, " size_ref=%i,%i%n", &x, &y, &r) == 2)      { line += r; node.sizeRef = Vector2Dih(x, y); }
    }
    if (sscanf(line, " split=%c%n", &c, &r) == 1)                   { line += r; if (c == 'X') node.SplitAxis = Axis::X; else if (c == 'Y') node.SplitAxis = Axis::Y; }
    if (sscanf(line, " NoResize=%d%n", &x, &r) == 1)                { line += r; if (x != 0) node.flags |= ImGuiDockNodeFlags_NoResize; }
    if (sscanf(line, " central_node=%d%n", &x, &r) == 1)             { line += r; if (x != 0) node.flags |= ImGuiDockNodeFlags_CentralNode; }
    if (sscanf(line, " NoTabBar=%d%n", &x, &r) == 1)                { line += r; if (x != 0) node.flags |= ImGuiDockNodeFlags_NoTabBar; }
    if (sscanf(line, " HiddenTabBar=%d%n", &x, &r) == 1)            { line += r; if (x != 0) node.flags |= ImGuiDockNodeFlags_HiddenTabBar; }
    if (sscanf(line, " NoWindowMenuButton=%d%n", &x, &r) == 1)      { line += r; if (x != 0) node.flags |= ImGuiDockNodeFlags_NoWindowMenuButton; }
    if (sscanf(line, " NoCloseButton=%d%n", &x, &r) == 1)           { line += r; if (x != 0) node.flags |= ImGuiDockNodeFlags_NoCloseButton; }
    if (sscanf(line, " Selected=0x%08X%n", &node.SelectedTabId,&r) == 1) { line += r; }
    if (node.ParentNodeId != 0)
        if (ImGuiDockNodeSettings* parent_settings = DockSettingsFindNodeSettings(ctx, node.ParentNodeId))
            node.Depth = parent_settings.Depth + 1;
    ctx.DockContext.NodesSettings.push_back(node);
}

static void DockSettingsHandler_DockNodeToSettings(ImGuiDockContext* dc, ImGuiDockNode* node, int depth)
{
    ImGuiDockNodeSettings node_settings;
    IM_ASSERT(depth < (1 << (sizeof(node_settings.Depth) << 3)));
    node_settings.id = node.ID;
    node_settings.ParentNodeId = node.ParentNode ? node.ParentNode.ID : 0;
    node_settings.ParentWindowId = (node.IsDockSpace() && node.host_window && node.host_window.parent_window) ? node.host_window.parent_window.ID : 0;
    node_settings.SelectedTabId = node.SelectedTabId;
    node_settings.SplitAxis = (signed char)(node.IsSplitNode() ? node.SplitAxis : ImGuiAxis_None);
    node_settings.Depth = (char)depth;
    node_settings.flags = (node.LocalFlags & ImGuiDockNodeFlags_SavedFlagsMask_);
    node_settings.Pos = Vector2Dih(node.pos);
    node_settings.size = Vector2Dih(node.size);
    node_settings.sizeRef = Vector2Dih(node.sizeRef);
    dc.NodesSettings.push_back(node_settings);
    if (node.ChildNodes[0])
        DockSettingsHandler_DockNodeToSettings(dc, node.ChildNodes[0], depth + 1);
    if (node.ChildNodes[1])
        DockSettingsHandler_DockNodeToSettings(dc, node.ChildNodes[1], depth + 1);
}

static void DockSettingsHandler_WriteAll(ImGuiContext* ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* buf)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc = &ctx.DockContext;
    if (!(g.io.config_flags & ImGuiConfigFlags_DockingEnable))
        return;

    // Gather settings data
    // (unlike our windows settings, because nodes are always built we can do a full rewrite of the SettingsNode buffer)
    dc.NodesSettings.resize(0);
    dc.NodesSettings.reserve(dc.Nodes.data.size);
    for (int n = 0; n < dc.Nodes.data.size; n += 1)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc.Nodes.data[n].val_p)
            if (node.IsRootNode())
                DockSettingsHandler_DockNodeToSettings(dc, node, 0);

    int max_depth = 0;
    for (int node_n = 0; node_n < dc.NodesSettings.size; node_n += 1)
        max_depth = ImMax(dc.NodesSettings[node_n].Depth, max_depth);

    // Write to text buffer
    buf.appendf("[%s][data]\n", handler.TypeName);
    for (int node_n = 0; node_n < dc.NodesSettings.size; node_n += 1)
    {
        const int line_start_pos = buf->size(); (void)line_start_pos;
        const ImGuiDockNodeSettings* node_settings = &dc.NodesSettings[node_n];
        buf.appendf("%*s%s%*s", node_settings.Depth * 2, "", (node_settings.flags & ImGuiDockNodeFlags_DockSpace) ? "DockSpace" : "dock_node ", (max_depth - node_settings.Depth) * 2, "");  // Text align nodes to facilitate looking at .ini file
        buf.appendf(" id=0x%08X", node_settings->ID);
        if (node_settings->ParentNodeId)
        {
            buf->appendf(" Parent=0x%08X size_ref=%d,%d", node_settings->ParentNodeId, node_settings.sizeRef.x, node_settings.sizeRef.y);
        }
        else
        {
            if (node_settings->ParentWindowId)
                buf->appendf(" window=0x%08X", node_settings->ParentWindowId);
            buf->appendf(" pos=%d,%d size=%d,%d", node_settings.pos.x, node_settings.pos.y, node_settings.size.x, node_settings.size.y);
        }
        if (node_settings->SplitAxis != ImGuiAxis_None)
            buf->appendf(" split=%c", (node_settings->SplitAxis == Axis::X) ? 'X' : 'Y');
        if (node_settings.flags & ImGuiDockNodeFlags_NoResize)
            buf->appendf(" NoResize=1");
        if (node_settings.flags & ImGuiDockNodeFlags_CentralNode)
            buf->appendf(" central_node=1");
        if (node_settings.flags & ImGuiDockNodeFlags_NoTabBar)
            buf->appendf(" NoTabBar=1");
        if (node_settings.flags & ImGuiDockNodeFlags_HiddenTabBar)
            buf->appendf(" HiddenTabBar=1");
        if (node_settings.flags & ImGuiDockNodeFlags_NoWindowMenuButton)
            buf->appendf(" NoWindowMenuButton=1");
        if (node_settings.flags & ImGuiDockNodeFlags_NoCloseButton)
            buf->appendf(" NoCloseButton=1");
        if (node_settings->SelectedTabId)
            buf->appendf(" Selected=0x%08X", node_settings->SelectedTabId);

#if IMGUI_DEBUG_INI_SETTINGS
        // [DEBUG] Include comments in the .ini file to ease debugging
        if (ImGuiDockNode* node = DockContextFindNodeByID(ctx, node_settings->ID))
        {
            buf->appendf("%*s", ImMax(2, (line_start_pos + 92) - buf->size()), "");     // Align everything
            if (node->IsDockSpace() && node->HostWindow && node->HostWindow->parent_window)
                buf->appendf(" ; in '%s'", node->HostWindow->parent_window->Name);
            // Iterate settings so we can give info about windows that didn't exist during the session.
            int contains_window = 0;
            for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
                if (settings.dock_id == node_settings->ID)
                {
                    if (contains_window += 1 == 0)
                        buf->appendf(" ; contains ");
                    buf->appendf("'%s' ", settings->GetName());
                }
        }

        buf->appendf("\n");
    }
    buf->appendf("\n");
}


// Win32 API IME support (for Asian languages, etc.)
#if defined(_WIN32) && !defined(IMGUI_DISABLE_WIN32_FUNCTIONS) && !defined(IMGUI_DISABLE_WIN32_DEFAULT_IME_FUNCTIONS)

#include <imm.h>
#ifdef _MSC_VER
#pragma comment(lib, "imm32")


static void SetPlatformImeDataFn_DefaultImpl(ImGuiViewport* viewport, ImGuiPlatformImeData* data)
{
    // Notify OS Input Method Editor of text input position
    HWND hwnd = (HWND)viewport->PlatformHandleRaw;
#ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    if (hwnd == 0)
        hwnd = (HWND)GetIO().ImeWindowHandle;

    if (hwnd == 0)
        return;

    ::ImmAssociateContextEx(hwnd, NULL, data->WantVisible ? IACE_DEFAULT : 0);

    if (HIMC himc = ::ImmGetContext(hwnd))
    {
        COMPOSITIONFORM composition_form = {};
        composition_form.ptCurrentPos.x = (LONG)(data->InputPos.x - viewport.pos.x);
        composition_form.ptCurrentPos.y = (LONG)(data->InputPos.y - viewport.pos.y);
        composition_form.dwStyle = CFS_FORCE_POSITION;
        ::ImmSetCompositionWindow(himc, &composition_form);
        CANDIDATEFORM candidate_form = {};
        candidate_form.dwStyle = CFS_CANDIDATEPOS;
        candidate_form.ptCurrentPos.x = (LONG)(data->InputPos.x - viewport.pos.x);
        candidate_form.ptCurrentPos.y = (LONG)(data->InputPos.y - viewport.pos.y);
        ::ImmSetCandidateWindow(himc, &candidate_form);
        ::ImmReleaseContext(hwnd, himc);
    }
}

#else

static void SetPlatformImeDataFn_DefaultImpl(ImGuiViewport*, ImGuiPlatformImeData*) {}



//-----------------------------------------------------------------------------
// [SECTION] METRICS/DEBUGGER WINDOW
//-----------------------------------------------------------------------------
// - RenderViewportThumbnail() [Internal]
// - RenderViewportsThumbnails() [Internal]
// - DebugTextEncoding()
// - MetricsHelpMarker() [Internal]
// - ShowFontAtlas() [Internal]
// - ShowMetricsWindow()
// - DebugNodeColumns() [Internal]
// - DebugNodeDockNode() [Internal]
// - DebugNodeDrawList() [Internal]
// - DebugNodeDrawCmdShowMeshAndBoundingBox() [Internal]
// - DebugNodeFont() [Internal]
// - DebugNodeFontGlyph() [Internal]
// - DebugNodeStorage() [Internal]
// - DebugNodeTabBar() [Internal]
// - DebugNodeViewport() [Internal]
// - DebugNodeWindow() [Internal]
// - DebugNodeWindowSettings() [Internal]
// - DebugNodeWindowsList() [Internal]
// - DebugNodeWindowsListByBeginStackParent() [Internal]
//-----------------------------------------------------------------------------

#ifndef IMGUI_DISABLE_DEBUG_TOOLS

void DebugRenderViewportThumbnail(ImDrawList* draw_list, ImGuiViewportP* viewport, const Rect& bb)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;

    Vector2D scale = bb.GetSize() / viewport.size;
    Vector2D off = bb.min - viewport.pos * scale;
    float alpha_mul = (viewport.flags & ImGuiViewportFlags_Minimized) ? 0.30 : 1.00;
    window.draw_list->AddRectFilled(bb.min, bb.max, get_color_u32(StyleColor::Border, alpha_mul * 0.40));
    for (int i = 0; i != g.windows.size; i += 1)
    {
        ImGuiWindow* thumb_window = g.windows[i];
        if (!thumb_window.was_active || (thumb_window.flags & WindowFlags::ChildWindow))
            continue;
        if (thumb_window.viewport != viewport)
            continue;

        Rect thumb_r = thumb_window.Rect();
        Rect title_r = thumb_window.title_bar_rect();
        thumb_r = Rect(f32::floor(off + thumb_r.min * scale), f32::floor(off +  thumb_r.max * scale));
        title_r = Rect(f32::floor(off + title_r.min * scale), f32::floor(off +  Vector2D::new(title_r.max.x, title_r.min.y) * scale) + Vector2D::new(0,5)); // Exaggerate title bar height
        thumb_r.ClipWithFull(bb);
        title_r.ClipWithFull(bb);
        const bool window_is_focused = (g.nav_window && thumb_window.root_window_for_title_bar_highlight == g.nav_window->root_window_for_title_bar_highlight);
        window.draw_list->AddRectFilled(thumb_r.min, thumb_r.max, get_color_u32(StyleColor::WindowBg, alpha_mul));
        window.draw_list->AddRectFilled(title_r.min, title_r.max, get_color_u32(window_is_focused ? StyleColor::TitleBgActive : StyleColor::TitleBg, alpha_mul));
        window.draw_list->AddRect(thumb_r.min, thumb_r.max, get_color_u32(StyleColor::Border, alpha_mul));
        window.draw_list->AddText(g.font, g.font_size * 1.0, title_r.min, get_color_u32(StyleColor::Text, alpha_mul), thumb_window.Name, FindRenderedTextEnd(thumb_window.Name));
    }
    draw_list->AddRect(bb.min, bb.max, get_color_u32(StyleColor::Border, alpha_mul));
}

static void RenderViewportsThumbnails()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;

    // We don't display full monitor bounds (we could, but it often looks awkward), instead we display just enough to cover all of our viewports.
    float SCALE = 1.0 / 8.0;
    Rect bb_full(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    for (int n = 0; n < g.viewports.size; n += 1)
        bb_full.Add(g.viewports[n]->get_main_rect());
    Vector2D p = window.dc.cursor_pos;
    Vector2D off = p - bb_full.min * SCALE;
    for (int n = 0; n < g.viewports.size; n += 1)
    {
        ImGuiViewportP* viewport = g.viewports[n];
        Rect viewport_draw_bb(off + (viewport.pos) * SCALE, off + (viewport.pos + viewport.size) * SCALE);
        DebugRenderViewportThumbnail(window.draw_list, viewport, viewport_draw_bb);
    }
    Dummy(bb_full.GetSize() * SCALE);
}

static int IMGUI_CDECL ViewportComparerByFrontMostStampCount(const void* lhs, const void* rhs)
{
    const ImGuiViewportP* a = *(const ImGuiViewportP* const*)lhs;
    const ImGuiViewportP* b = *(const ImGuiViewportP* const*)rhs;
    return b->LastFrontMostStampCount - a->LastFrontMostStampCount;
}

// Helper tool to diagnose between text encoding issues and font loading issues. Pass your UTF-8 string and verify that there are correct.
void DebugTextEncoding(const char* str)
{
    Text("Text: \"%s\"", str);
    if (!BeginTable("list", 4, ImGuiTableFlags_Borders | ImGuiTableFlags_RowBg | ImGuiTableFlags_SizingFixedFit))
        return;
    TableSetupColumn("Offset");
    TableSetupColumn("UTF-8");
    TableSetupColumn("Glyph");
    TableSetupColumn("codepoint");
    TableHeadersRow();
    for (const char* p = str; *p != 0; )
    {
        unsigned int c;
        const int c_utf8_len = ImTextCharFromUtf8(&c, p, NULL);
        TableNextColumn();
        Text("%d", (p - str));
        TableNextColumn();
        for (int byte_index = 0; byte_index < c_utf8_len; byte_index += 1)
        {
            if (byte_index > 0)
                SameLine();
            Text("0x%02X", (unsigned char)p[byte_index]);
        }
        TableNextColumn();
        if (GetFont()->FindGlyphNoFallback((ImWchar)c))
            TextUnformatted(p, p + c_utf8_len);
        else
            TextUnformatted((c == IM_UNICODE_CODEPOINT_INVALID) ? "[invalid]" : "[missing]");
        TableNextColumn();
        Text("U+%04X", c);
        p += c_utf8_len;
    }
    EndTable();
}

// Avoid naming collision with imgui_demo.cpp's HelpMarker() for unity builds.
static void MetricsHelpMarker(const char* desc)
{
    TextDisabled("(?)");
    if (IsItemHovered())
    {
        BeginTooltip();
        PushTextWrapPos(GetFontSize() * 35.0);
        TextUnformatted(desc);
        PopTextWrapPos();
        EndTooltip();
    }
}

// [DEBUG] List fonts in a font atlas and display its texture
void ShowFontAtlas(ImFontAtlas* atlas)
{
    for (int i = 0; i < atlas->Fonts.size; i += 1)
    {
        ImFont* font = atlas->Fonts[i];
        PushID(font);
        DebugNodeFont(font);
        PopID();
    }
    if (TreeNode("Atlas texture", "Atlas texture (%dx%d pixels)", atlas->TexWidth, atlas->TexHeight))
    {
        Vector4D tint_col = Vector4D(1.0, 1.0, 1.0, 1.0);
        Vector4D border_col = Vector4D(1.0, 1.0, 1.0, 0.5);
        Image(atlas->TexID, Vector2D::new((float)atlas->TexWidth, atlas->TexHeight), Vector2D::new(0.0, 0.0), Vector2D::new(1.0, 1.0), tint_col, border_col);
        TreePop();
    }
}

void ShowMetricsWindow(bool* p_open)
{
    ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;
    ImGuiMetricsConfig* cfg = &g.DebugMetricsConfig;
    if (cfg->ShowDebugLog)
        ShowDebugLogWindow(&cfg->ShowDebugLog);
    if (cfg->ShowStackTool)
        ShowStackToolWindow(&cfg->ShowStackTool);

    if (!begin("Dear ImGui Metrics/Debugger", p_open) || GetCurrentWindow()->BeginCount > 1)
    {
        end();
        return;
    }

    // Basic info
    Text("Dear ImGui %s", GetVersion());
    Text("Application average %.3 ms/frame (%.1 FPS)", 1000.0 / io.frame_rate, io.frame_rate);
    Text("%d vertices, %d indices (%d triangles)", io.metrics_render_vertices, io.metrics_render_indices, io.metrics_render_indices / 3);
    Text("%d visible windows, %d active allocations", io.metrics_render_windows, io.MetricsActiveAllocations);
    //SameLine(); if (SmallButton("GC")) { g.gc_compact_all = true; }

    Separator();

    // Debugging enums
    enum { WRT_OuterRect, WRT_OuterRectClipped, WRT_InnerRect, WRT_InnerClipRect, WRT_WorkRect, WRT_Content, WRT_ContentIdeal, WRT_ContentRegionRect, WRT_Count }; // windows rect Type
    const char* wrt_rects_names[WRT_Count] = { "OuterRect", "outer_rect_clipped", "inner_rect", "inner_clip_rect", "work_rect", "Content", "ContentIdeal", "content_region_rect" };
    enum { TRT_OuterRect, TRT_InnerRect, TRT_WorkRect, TRT_HostClipRect, TRT_InnerClipRect, TRT_BackgroundClipRect, TRT_ColumnsRect, TRT_ColumnsWorkRect, TRT_ColumnsClipRect, TRT_ColumnsContentHeadersUsed, TRT_ColumnsContentHeadersIdeal, TRT_ColumnsContentFrozen, TRT_ColumnsContentUnfrozen, TRT_Count }; // tables rect Type
    const char* trt_rects_names[TRT_Count] = { "OuterRect", "inner_rect", "work_rect", "HostClipRect", "inner_clip_rect", "BackgroundClipRect", "ColumnsRect", "ColumnsWorkRect", "ColumnsClipRect", "ColumnsContentHeadersUsed", "ColumnsContentHeadersIdeal", "ColumnsContentFrozen", "ColumnsContentUnfrozen" };
    if (cfg->ShowWindowsRectsType < 0)
        cfg->ShowWindowsRectsType = WRT_WorkRect;
    if (cfg->ShowTablesRectsType < 0)
        cfg->ShowTablesRectsType = TRT_WorkRect;

    struct Funcs
    {
        static Rect GetTableRect(ImGuiTable* table, int rect_type, int n)
        {
            ImGuiTableInstanceData* table_instance = TableGetInstanceData(table, table->InstanceCurrent); // Always using last submitted instance
            if (rect_type == TRT_OuterRect)                     { return table->OuterRect; }
            else if (rect_type == TRT_InnerRect)                { return table->inner_rect; }
            else if (rect_type == TRT_WorkRect)                 { return table->WorkRect; }
            else if (rect_type == TRT_HostClipRect)             { return table->HostClipRect; }
            else if (rect_type == TRT_InnerClipRect)            { return table->InnerClipRect; }
            else if (rect_type == TRT_BackgroundClipRect)       { return table->BgClipRect; }
            else if (rect_type == TRT_ColumnsRect)              { ImGuiTableColumn* c = &table->Columns[n]; return Rect(c->MinX, table->InnerClipRect.min.y, c->MaxX, table->InnerClipRect.min.y + table_instance->LastOuterHeight); }
            else if (rect_type == TRT_ColumnsWorkRect)          { ImGuiTableColumn* c = &table->Columns[n]; return Rect(c->WorkMinX, table->WorkRect.min.y, c->WorkMaxX, table->WorkRect.max.y); }
            else if (rect_type == TRT_ColumnsClipRect)          { ImGuiTableColumn* c = &table->Columns[n]; return c->ClipRect; }
            else if (rect_type == TRT_ColumnsContentHeadersUsed){ ImGuiTableColumn* c = &table->Columns[n]; return Rect(c->WorkMinX, table->InnerClipRect.min.y, c->ContentMaxXHeadersUsed, table->InnerClipRect.min.y + table_instance->LastFirstRowHeight); } // Note: y1/y2 not always accurate
            else if (rect_type == TRT_ColumnsContentHeadersIdeal){ImGuiTableColumn* c = &table->Columns[n]; return Rect(c->WorkMinX, table->InnerClipRect.min.y, c->ContentMaxXHeadersIdeal, table->InnerClipRect.min.y + table_instance->LastFirstRowHeight); }
            else if (rect_type == TRT_ColumnsContentFrozen)     { ImGuiTableColumn* c = &table->Columns[n]; return Rect(c->WorkMinX, table->InnerClipRect.min.y, c->ContentMaxXFrozen, table->InnerClipRect.min.y + table_instance->LastFirstRowHeight); }
            else if (rect_type == TRT_ColumnsContentUnfrozen)   { ImGuiTableColumn* c = &table->Columns[n]; return Rect(c->WorkMinX, table->InnerClipRect.min.y + table_instance->LastFirstRowHeight, c->ContentMaxXUnfrozen, table->InnerClipRect.max.y); }
            IM_ASSERT(0);
            return Rect();
        }

        static Rect GetWindowRect(ImGuiWindow* window, int rect_type)
        {
            if (rect_type == WRT_OuterRect)                 { return window.Rect(); }
            else if (rect_type == WRT_OuterRectClipped)     { return window.OuterRectClipped; }
            else if (rect_type == WRT_InnerRect)            { return window.inner_rect; }
            else if (rect_type == WRT_InnerClipRect)        { return window.InnerClipRect; }
            else if (rect_type == WRT_WorkRect)             { return window.WorkRect; }
            else if (rect_type == WRT_Content)       { Vector2D min = window.inner_rect.min - window.scroll + window.WindowPadding; return Rect(min, min + window.ContentSize); }
            else if (rect_type == WRT_ContentIdeal)         { Vector2D min = window.inner_rect.min - window.scroll + window.WindowPadding; return Rect(min, min + window.ContentSizeIdeal); }
            else if (rect_type == WRT_ContentRegionRect)    { return window.ContentRegionRect; }
            IM_ASSERT(0);
            return Rect();
        }
    };

    // Tools
    if (TreeNode("Tools"))
    {
        bool show_encoding_viewer = TreeNode("UTF-8 Encoding viewer");
        SameLine();
        MetricsHelpMarker("You can also call DebugTextEncoding() from your code with a given string to test that your UTF-8 encoding settings are correct.");
        if (show_encoding_viewer)
        {
            static char buf[100] = "";
            SetNextItemWidth(-FLT_MIN);
            InputText("##Text", buf, IM_ARRAYSIZE(buf));
            if (buf[0] != 0)
                DebugTextEncoding(buf);
            TreePop();
        }

        // The Item Picker tool is super useful to visually select an item and break into the call-stack of where it was submitted.
        if (Checkbox("Show Item Picker", &g.DebugItemPickerActive) && g.DebugItemPickerActive)
            DebugStartItemPicker();
        SameLine();
        MetricsHelpMarker("Will call the IM_DEBUG_BREAK() macro to break in debugger.\nWarning: If you don't have a debugger attached, this will probably crash.");

        // Stack Tool is your best friend!
        Checkbox("Show Debug Log", &cfg->ShowDebugLog);
        SameLine();
        MetricsHelpMarker("You can also call ShowDebugLogWindow() from your code.");

        // Stack Tool is your best friend!
        Checkbox("Show Stack Tool", &cfg->ShowStackTool);
        SameLine();
        MetricsHelpMarker("You can also call ShowStackToolWindow() from your code.");

        Checkbox("Show windows begin order", &cfg->ShowWindowsBeginOrder);
        Checkbox("Show windows rectangles", &cfg->ShowWindowsRects);
        SameLine();
        SetNextItemWidth(GetFontSize() * 12);
        cfg->ShowWindowsRects |= Combo("##show_windows_rect_type", &cfg->ShowWindowsRectsType, wrt_rects_names, WRT_Count, WRT_Count);
        if (cfg->ShowWindowsRects && g.nav_window != NULL)
        {
            BulletText("'%s':", g.nav_window->Name);
            Indent();
            for (int rect_n = 0; rect_n < WRT_Count; rect_n += 1)
            {
                Rect r = Funcs::GetWindowRect(g.nav_window, rect_n);
                Text("(%6.1,%6.1) (%6.1,%6.1) size (%6.1,%6.1) %s", r.min.x, r.min.y, r.max.x, r.max.y, r.get_width(), r.get_height(), wrt_rects_names[rect_n]);
            }
            Unindent();
        }

        Checkbox("Show tables rectangles", &cfg->ShowTablesRects);
        SameLine();
        SetNextItemWidth(GetFontSize() * 12);
        cfg->ShowTablesRects |= Combo("##show_table_rects_type", &cfg->ShowTablesRectsType, trt_rects_names, TRT_Count, TRT_Count);
        if (cfg->ShowTablesRects && g.nav_window != NULL)
        {
            for (int table_n = 0; table_n < g.tables.GetMapSize(); table_n += 1)
            {
                ImGuiTable* table = g.tables.TryGetMapData(table_n);
                if (table == NULL || table->LastFrameActive < g.frame_count - 1 || (table->OuterWindow != g.nav_window && table->InnerWindow != g.nav_window))
                    continue;

                BulletText("Table 0x%08X (%d columns, in '%s')", table->ID, table->ColumnsCount, table->OuterWindow->Name);
                if (IsItemHovered())
                    get_foreground_draw_list()->AddRect(table->OuterRect.min - Vector2D::new(1, 1), table->OuterRect.max + Vector2D::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0, 0, 2.0);
                Indent();
                char buf[128];
                for (int rect_n = 0; rect_n < TRT_Count; rect_n += 1)
                {
                    if (rect_n >= TRT_ColumnsRect)
                    {
                        if (rect_n != TRT_ColumnsRect && rect_n != TRT_ColumnsClipRect)
                            continue;
                        for (int column_n = 0; column_n < table->ColumnsCount; column_n += 1)
                        {
                            Rect r = Funcs::GetTableRect(table, rect_n, column_n);
                            ImFormatString(buf, IM_ARRAYSIZE(buf), "(%6.1,%6.1) (%6.1,%6.1) size (%6.1,%6.1) col %d %s", r.min.x, r.min.y, r.max.x, r.max.y, r.get_width(), r.get_height(), column_n, trt_rects_names[rect_n]);
                            Selectable(buf);
                            if (IsItemHovered())
                                get_foreground_draw_list()->AddRect(r.min - Vector2D::new(1, 1), r.max + Vector2D::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0, 0, 2.0);
                        }
                    }
                    else
                    {
                        Rect r = Funcs::GetTableRect(table, rect_n, -1);
                        ImFormatString(buf, IM_ARRAYSIZE(buf), "(%6.1,%6.1) (%6.1,%6.1) size (%6.1,%6.1) %s", r.min.x, r.min.y, r.max.x, r.max.y, r.get_width(), r.get_height(), trt_rects_names[rect_n]);
                        Selectable(buf);
                        if (IsItemHovered())
                            get_foreground_draw_list()->AddRect(r.min - Vector2D::new(1, 1), r.max + Vector2D::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0, 0, 2.0);
                    }
                }
                Unindent();
            }
        }

        TreePop();
    }

    // windows
    if (TreeNode("windows", "windows (%d)", g.windows.size))
    {
        //SetNextItemOpen(true, ImGuiCond_Once);
        DebugNodeWindowsList(&g.windows, "By display order");
        DebugNodeWindowsList(&g.windows_focus_order, "By focus order (root windows)");
        if (TreeNode("By submission order (begin stack)"))
        {
            // Here we display windows in their submitted order/hierarchy, however note that the Begin stack doesn't constitute a Parent<>Child relationship!
            ImVector<ImGuiWindow*>& temp_buffer = g.windows_temp_sort_buffer;
            temp_buffer.resize(0);
            for (int i = 0; i < g.windows.size; i += 1)
                if (g.windows[i]->LastFrameActive + 1 >= g.frame_count)
                    temp_buffer.push_back(g.windows[i]);
            struct Func { static int IMGUI_CDECL WindowComparerByBeginOrder(const void* lhs, const void* rhs) { return ((*(const ImGuiWindow* const *)lhs)->BeginOrderWithinContext - (*(const ImGuiWindow* const*)rhs)->BeginOrderWithinContext); } };
            ImQsort(temp_buffer.data, temp_buffer.size, sizeof(ImGuiWindow*), Func::WindowComparerByBeginOrder);
            DebugNodeWindowsListByBeginStackParent(temp_buffer.data, temp_buffer.size, NULL);
            TreePop();
        }

        TreePop();
    }

    // DrawLists
    int drawlist_count = 0;
    for (int viewport_i = 0; viewport_i < g.viewports.size; viewport_i += 1)
        drawlist_count += g.viewports[viewport_i].draw_data_builder.GetDrawListCount();
    if (TreeNode("DrawLists", "DrawLists (%d)", drawlist_count))
    {
        Checkbox("Show ImDrawCmd mesh when hovering", &cfg->ShowDrawCmdMesh);
        Checkbox("Show ImDrawCmd bounding boxes when hovering", &cfg->ShowDrawCmdBoundingBoxes);
        for (int viewport_i = 0; viewport_i < g.viewports.size; viewport_i += 1)
        {
            ImGuiViewportP* viewport = g.viewports[viewport_i];
            bool viewport_has_drawlist = false;
            for (int layer_i = 0; layer_i < IM_ARRAYSIZE(viewport.draw_data_builder.layers); layer_i += 1)
                for (int draw_list_i = 0; draw_list_i < viewport.draw_data_builder.layers[layer_i].size; draw_list_i += 1)
                {
                    if (!viewport_has_drawlist)
                        Text("active DrawLists in viewport #%d, id: 0x%08X", viewport->Idx, viewport->ID);
                    viewport_has_drawlist = true;
                    DebugNodeDrawList(NULL, viewport, viewport.draw_data_builder.layers[layer_i][draw_list_i], "draw_list");
                }
        }
        TreePop();
    }

    // viewports
    if (TreeNode("viewports", "viewports (%d)", g.viewports.size))
    {
        Indent(GetTreeNodeToLabelSpacing());
        RenderViewportsThumbnails();
        Unindent(GetTreeNodeToLabelSpacing());

        bool open = TreeNode("Monitors", "Monitors (%d)", g.platform_io.monitors.size);
        SameLine();
        MetricsHelpMarker("Dear ImGui uses monitor data:\n- to query DPI settings on a per monitor basis\n- to position popup/tooltips so they don't straddle monitors.");
        if (open)
        {
            for (int i = 0; i < g.platform_io.monitors.size; i += 1)
            {
                const ImGuiPlatformMonitor& mon = g.platform_io.monitors[i];
                BulletText("Monitor #%d: DPI %.0%%\n MainMin (%.0,%.0), MainMax (%.0,%.0), MainSize (%.0,%.0)\n WorkMin (%.0,%.0), WorkMax (%.0,%.0), work_size (%.0,%.0)",
                    i, mon.DpiScale * 100.0,
                    mon.MainPos.x, mon.MainPos.y, mon.MainPos.x + mon.MainSize.x, mon.MainPos.y + mon.MainSize.y, mon.MainSize.x, mon.MainSize.y,
                    mon.WorkPos.x, mon.WorkPos.y, mon.WorkPos.x + mon.work_size.x, mon.WorkPos.y + mon.work_size.y, mon.work_size.x, mon.work_size.y);
            }
            TreePop();
        }

        BulletText("mouse_viewport: 0x%08X (UserHovered 0x%08X, LastHovered 0x%08X)", g.mouse_viewport ? g.mouse_viewport->ID : 0, g.io.MouseHoveredViewport, g.mouse_last_hovered_viewport ? g.mouse_last_hovered_viewport->ID : 0);
        if (TreeNode("Inferred Z order (front-to-back)"))
        {
            static ImVector<ImGuiViewportP*> viewports;
            viewports.resize(g.viewports.size);
            memcpy(viewports.data, g.viewports.data, g.viewports.size_in_bytes());
            if (viewports.size > 1)
                ImQsort(viewports.data, viewports.size, sizeof(ImGuiViewport*), ViewportComparerByFrontMostStampCount);
            for (int i = 0; i < viewports.size; i += 1)
                BulletText("viewport #%d, id: 0x%08X, FrontMostStampCount = %08d, window: \"%s\"", viewports[i]->Idx, viewports[i]->ID, viewports[i]->LastFrontMostStampCount, viewports[i]->Window ? viewports[i]->Window->Name : "N/A");
            TreePop();
        }

        for (int i = 0; i < g.viewports.size; i += 1)
            DebugNodeViewport(g.viewports[i]);
        TreePop();
    }

    // Details for Popups
    if (TreeNode("Popups", "Popups (%d)", g.open_popup_stack.size))
    {
        for (int i = 0; i < g.open_popup_stack.size; i += 1)
        {
            ImGuiWindow* window = g.open_popup_stack[i].Window;
            BulletText("PopupID: %08x, window: '%s'%s%s", g.open_popup_stack[i].PopupId, window ? window.Name : "NULL", window && (window.flags & WindowFlags::ChildWindow) ? " ChildWindow" : "", window && (window.flags & WindowFlags::ChildMenu) ? " ChildMenu" : "");
        }
        TreePop();
    }

    // Details for tab_bars
    if (TreeNode("tab_bars", "Tab Bars (%d)", g.tab_bars.GetAliveCount()))
    {
        for (int n = 0; n < g.tab_bars.GetMapSize(); n += 1)
            if (ImGuiTabBar* tab_bar = g.tab_bars.TryGetMapData(n))
            {
                PushID(tab_bar);
                DebugNodeTabBar(tab_bar, "tab_bar");
                PopID();
            }
        TreePop();
    }

    // Details for tables
    if (TreeNode("tables", "tables (%d)", g.tables.GetAliveCount()))
    {
        for (int n = 0; n < g.tables.GetMapSize(); n += 1)
            if (ImGuiTable* table = g.tables.TryGetMapData(n))
                DebugNodeTable(table);
        TreePop();
    }

    // Details for fonts
    ImFontAtlas* atlas = g.io.fonts;
    if (TreeNode("fonts", "fonts (%d)", atlas->Fonts.size))
    {
        ShowFontAtlas(atlas);
        TreePop();
    }

    // Details for InputText
    if (TreeNode("InputText"))
    {
        DebugNodeInputTextState(&g.input_text_state);
        TreePop();
    }

    // Details for Docking
#ifdef IMGUI_HAS_DOCK
    if (TreeNode("Docking"))
    {
        static bool root_nodes_only = true;
        ImGuiDockContext* dc = &g.DockContext;
        Checkbox("List root nodes", &root_nodes_only);
        Checkbox("Ctrl shows window dock info", &cfg->ShowDockingNodes);
        if (SmallButton("clear nodes")) { DockContextClearNodes(&g, 0, true); }
        SameLine();
        if (SmallButton("Rebuild all")) { dc->WantFullRebuild = true; }
        for (int n = 0; n < dc->Nodes.data.size; n += 1)
            if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.data[n].val_p)
                if (!root_nodes_only || node->IsRootNode())
                    DebugNodeDockNode(node, "Node");
        TreePop();
    }
 // #ifdef IMGUI_HAS_DOCK

    // Settings
    if (TreeNode("Settings"))
    {
        if (SmallButton("clear"))
            ClearIniSettings();
        SameLine();
        if (SmallButton("Save to memory"))
            SaveIniSettingsToMemory();
        SameLine();
        if (SmallButton("Save to disk"))
            save_ini_settings_to_disk(g.io.ini_file_name);
        SameLine();
        if (g.io.ini_file_name)
            Text("\"%s\"", g.io.ini_file_name);
        else
            TextUnformatted("<NULL>");
        Text("settings_dirty_timer %.2", g.SettingsDirtyTimer);
        if (TreeNode("settings_handlers", "Settings handlers: (%d)", g.settings_handlers.size))
        {
            for (int n = 0; n < g.settings_handlers.size; n += 1)
                BulletText("%s", g.settings_handlers[n].TypeName);
            TreePop();
        }
        if (TreeNode("settings_windows", "Settings packed data: windows: %d bytes", g.settings_windows.size()))
        {
            for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
                DebugNodeWindowSettings(settings);
            TreePop();
        }

        if (TreeNode("SettingsTables", "Settings packed data: tables: %d bytes", g.SettingsTables.size()))
        {
            for (ImGuiTableSettings* settings = g.SettingsTables.begin(); settings != NULL; settings = g.SettingsTables.next_chunk(settings))
                DebugNodeTableSettings(settings);
            TreePop();
        }

#ifdef IMGUI_HAS_DOCK
        if (TreeNode("SettingsDocking", "Settings packed data: Docking"))
        {
            ImGuiDockContext* dc = &g.DockContext;
            Text("In settings_windows:");
            for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
                if (settings.dock_id != 0)
                    BulletText("window '%s' -> dock_id %08X", settings->GetName(), settings.dock_id);
            Text("In SettingsNodes:");
            for (int n = 0; n < dc->NodesSettings.size; n += 1)
            {
                ImGuiDockNodeSettings* settings = &dc->NodesSettings[n];
                const char* selected_tab_name = NULL;
                if (settings->SelectedTabId)
                {
                    if (ImGuiWindow* window = FindWindowByID(settings->SelectedTabId))
                        selected_tab_name = window.Name;
                    else if (ImGuiWindowSettings* window_settings = FindWindowSettings(settings->SelectedTabId))
                        selected_tab_name = window_settings->GetName();
                }
                BulletText("Node %08X, Parent %08X, SelectedTab %08X ('%s')", settings->ID, settings->ParentNodeId, settings->SelectedTabId, selected_tab_name ? selected_tab_name : settings->SelectedTabId ? "N/A" : "");
            }
            TreePop();
        }
 // #ifdef IMGUI_HAS_DOCK

        if (TreeNode("settings_ini_data", "Settings unpacked data (.ini): %d bytes", g.SettingsIniData.size()))
        {
            InputTextMultiline("##Ini", (char*)(void*)g.SettingsIniData.c_str(), g.SettingsIniData.Buf.size, Vector2D::new(-FLT_MIN, GetTextLineHeight() * 20), ImGuiInputTextFlags_ReadOnly);
            TreePop();
        }
        TreePop();
    }

    // Misc Details
    if (TreeNode("Internal state"))
    {
        Text("WINDOWING");
        Indent();
        Text("hovered_window: '%s'", g.hovered_window ? g.hovered_window->Name : "NULL");
        Text("hovered_window->Root: '%s'", g.hovered_window ? g.hovered_window->root_window_dock_tree->Name : "NULL");
        Text("hovered_window_under_moving_window: '%s'", g.hovered_window_under_moving_window ? g.hovered_window_under_moving_window->Name : "NULL");
        Text("hovered_dock_node: 0x%08X", g.HoveredDockNode ? g.HoveredDockNode->ID : 0);
        Text("moving_window: '%s'", g.moving_window ? g.moving_window->Name : "NULL");
        Text("mouse_viewport: 0x%08X (UserHovered 0x%08X, LastHovered 0x%08X)", g.mouse_viewport->ID, g.io.MouseHoveredViewport, g.mouse_last_hovered_viewport ? g.mouse_last_hovered_viewport->ID : 0);
        Unindent();

        Text("ITEMS");
        Indent();
        Text("active_id: 0x%08X/0x%08X (%.2 sec), AllowOverlap: %d, Source: %s", g.active_id, g.active_id_previous_frame, g.active_id_timer, g.ActiveIdAllowOverlap, GetInputSourceName(g.active_id_source));
        Text("active_id_window: '%s'", g.active_id_window ? g.active_id_window->Name : "NULL");

        int active_id_using_key_input_count = 0;
        for (int n = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n += 1)
            active_id_using_key_input_count += g.active_id_using_key_input_mask[n] ? 1 : 0;
        Text("ActiveIdUsing: Wheel: %d, NavDirMask: %x, NavInputMask: %x, KeyInputMask: %d key(s)", g.active_id_using_mouse_wheel, g.active_id_using_nav_dir_mask, g.active_id_using_nav_input_mask, active_id_using_key_input_count);
        Text("hovered_id: 0x%08X (%.2 sec), AllowOverlap: %d", g.hovered_id_previous_frame, g.hovered_id_timer, g.hovered_id_allow_overlap); // Not displaying g.hovered_id as it is update mid-frame
        Text("DragDrop: %d, source_id = 0x%08X, Payload \"%s\" (%d bytes)", g.drag_drop_active, g.drag_drop_payload.source_id, g.drag_drop_payload.dataType, g.drag_drop_payload.dataSize);
        Unindent();

        Text("NAV,FOCUS");
        Indent();
        Text("nav_window: '%s'", g.nav_window ? g.nav_window->Name : "NULL");
        Text("nav_id: 0x%08X, nav_layer: %d", g.nav_id, g.NavLayer);
        Text("nav_input_source: %s", GetInputSourceName(g.nav_input_source));
        Text("nav_active: %d, nav_visible: %d", g.io.nav_active, g.io.NavVisible);
        Text("nav_activate_id/DownId/PressedId/InputId: %08X/%08X/%08X/%08X", g.nav_activate_id, g.NavActivateDownId, g.NavActivatePressedId, g.NavActivateInputId);
        Text("nav_activate_flags: %04X", g.NavActivateFlags);
        Text("NavDisableHighlight: %d, nav_disable_mouse_hover: %d", g.nav_disable_highlight, g.nav_disable_mouse_hover);
        Text("nav_focus_scope_id = 0x%08X", g.NavFocusScopeId);
        Text("nav_windowing_target: '%s'", g.nav_windowing_target ? g.nav_windowing_target->Name : "NULL");
        Unindent();

        TreePop();
    }

    // Overlay: Display windows Rectangles and Begin Order
    if (cfg->ShowWindowsRects || cfg->ShowWindowsBeginOrder)
    {
        for (int n = 0; n < g.windows.size; n += 1)
        {
            ImGuiWindow* window = g.windows[n];
            if (!window.was_active)
                continue;
            ImDrawList* draw_list = get_foreground_draw_list(window);
            if (cfg->ShowWindowsRects)
            {
                Rect r = Funcs::GetWindowRect(window, cfg->ShowWindowsRectsType);
                draw_list->AddRect(r.min, r.max, IM_COL32(255, 0, 128, 255));
            }
            if (cfg->ShowWindowsBeginOrder && !(window.flags & WindowFlags::ChildWindow))
            {
                char buf[32];
                ImFormatString(buf, IM_ARRAYSIZE(buf), "%d", window.BeginOrderWithinContext);
                float font_size = GetFontSize();
                draw_list->AddRectFilled(window.pos, window.pos + Vector2D::new(font_size, font_size), IM_COL32(200, 100, 100, 255));
                draw_list->AddText(window.pos, IM_COL32(255, 255, 255, 255), buf);
            }
        }
    }

    // Overlay: Display tables Rectangles
    if (cfg->ShowTablesRects)
    {
        for (int table_n = 0; table_n < g.tables.GetMapSize(); table_n += 1)
        {
            ImGuiTable* table = g.tables.TryGetMapData(table_n);
            if (table == NULL || table->LastFrameActive < g.frame_count - 1)
                continue;
            ImDrawList* draw_list = get_foreground_draw_list(table->OuterWindow);
            if (cfg->ShowTablesRectsType >= TRT_ColumnsRect)
            {
                for (int column_n = 0; column_n < table->ColumnsCount; column_n += 1)
                {
                    Rect r = Funcs::GetTableRect(table, cfg->ShowTablesRectsType, column_n);
                    ImU32 col = (table->HoveredColumnBody == column_n) ? IM_COL32(255, 255, 128, 255) : IM_COL32(255, 0, 128, 255);
                    float thickness = (table->HoveredColumnBody == column_n) ? 3.0 : 1.0;
                    draw_list->AddRect(r.min, r.max, col, 0.0, 0, thickness);
                }
            }
            else
            {
                Rect r = Funcs::GetTableRect(table, cfg->ShowTablesRectsType, -1);
                draw_list->AddRect(r.min, r.max, IM_COL32(255, 0, 128, 255));
            }
        }
    }

#ifdef IMGUI_HAS_DOCK
    // Overlay: Display Docking info
    if (cfg->ShowDockingNodes && g.io.key_ctrl && g.HoveredDockNode)
    {
        char buf[64] = "";
        char* p = buf;
        ImGuiDockNode* node = g.HoveredDockNode;
        ImDrawList* overlay_draw_list = node->HostWindow ? get_foreground_draw_list(node->HostWindow) : get_foreground_draw_list(GetMainViewport());
        p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "dock_id: %x%s\n", node->ID, node->IsCentralNode() ? " *central_node*" : "");
        p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "window_class: %08X\n", node->WindowClass.ClassId);
        p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "size: (%.0, %.0)\n", node.size.x, node.size.y);
        p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "size_ref: (%.0, %.0)\n", node.sizeRef.x, node.sizeRef.y);
        int depth = DockNodeGetDepth(node);
        overlay_draw_list->AddRect(node.pos + Vector2D::new(3, 3) * depth, node.pos + node.size - Vector2D::new(3, 3) * depth, IM_COL32(200, 100, 100, 255));
        Vector2D pos = node.pos + Vector2D::new(3, 3) * depth;
        overlay_draw_list->AddRectFilled(pos - Vector2D::new(1, 1), pos + CalcTextSize(buf) + Vector2D::new(1, 1), IM_COL32(200, 100, 100, 255));
        overlay_draw_list->AddText(NULL, 0.0, pos, IM_COL32(255, 255, 255, 255), buf);
    }
 // #ifdef IMGUI_HAS_DOCK

    end();
}

// [DEBUG] Display contents of Columns
void DebugNodeColumns(ImGuiOldColumns* columns)
{
    if (!TreeNode((void*)(uintptr_t)columns->ID, "Columns Id: 0x%08X, Count: %d, flags: 0x%04X", columns->ID, columns->Count, columns.flags))
        return;
    BulletText("width: %.1 (MinX: %.1, MaxX: %.1)", columns->OffMaxX - columns->OffMinX, columns->OffMinX, columns->OffMaxX);
    for (int column_n = 0; column_n < columns->Columns.size; column_n += 1)
        BulletText("column %02d: offset_norm %.3 (= %.1 px)", column_n, columns->Columns[column_n].OffsetNorm, GetColumnOffsetFromNorm(columns, columns->Columns[column_n].OffsetNorm));
    TreePop();
}

static void DebugNodeDockNodeFlags(ImGuiDockNodeFlags* p_flags, const char* label, bool enabled)
{
    using namespace ImGui;
    PushID(label);
    push_style_var(StyleVar::frame_padding, Vector2D::new(0.0, 0.0));
    Text("%s:", label);
    if (!enabled)
        BeginDisabled();
    CheckboxFlags("NoSplit", p_flags, ImGuiDockNodeFlags_NoSplit);
    CheckboxFlags("NoResize", p_flags, ImGuiDockNodeFlags_NoResize);
    CheckboxFlags("NoResizeX", p_flags, ImGuiDockNodeFlags_NoResizeX);
    CheckboxFlags("NoResizeY",p_flags, ImGuiDockNodeFlags_NoResizeY);
    CheckboxFlags("NoTabBar", p_flags, ImGuiDockNodeFlags_NoTabBar);
    CheckboxFlags("HiddenTabBar", p_flags, ImGuiDockNodeFlags_HiddenTabBar);
    CheckboxFlags("NoWindowMenuButton", p_flags, ImGuiDockNodeFlags_NoWindowMenuButton);
    CheckboxFlags("NoCloseButton", p_flags, ImGuiDockNodeFlags_NoCloseButton);
    CheckboxFlags("NoDocking", p_flags, ImGuiDockNodeFlags_NoDocking);
    CheckboxFlags("NoDockingSplitMe", p_flags, ImGuiDockNodeFlags_NoDockingSplitMe);
    CheckboxFlags("NoDockingSplitOther", p_flags, ImGuiDockNodeFlags_NoDockingSplitOther);
    CheckboxFlags("NoDockingOverMe", p_flags, ImGuiDockNodeFlags_NoDockingOverMe);
    CheckboxFlags("NoDockingOverOther", p_flags, ImGuiDockNodeFlags_NoDockingOverOther);
    CheckboxFlags("NoDockingOverEmpty", p_flags, ImGuiDockNodeFlags_NoDockingOverEmpty);
    if (!enabled)
        EndDisabled();
    pop_style_var();
    PopID();
}

// [DEBUG] Display contents of ImDockNode
void DebugNodeDockNode(ImGuiDockNode* node, const char* label)
{
    ImGuiContext& g = *GImGui;
    const bool is_alive = (g.frame_count - node->LastFrameAlive < 2);    // Submitted with ImGuiDockNodeFlags_KeepAliveOnly
    const bool is_active = (g.frame_count - node->LastFrameActive < 2);  // Submitted
    if (!is_alive) { push_style_color(StyleColor::Text, GetStyleColorVec4(StyleColor::TextDisabled)); }
    bool open;
    ImGuiTreeNodeFlags tree_node_flags = node->IsFocused ? ImGuiTreeNodeFlags_Selected : ImGuiTreeNodeFlags_None;
    if (node->Windows.size > 0)
        open = TreeNodeEx((void*)(intptr_t)node->ID, tree_node_flags, "%s 0x%04X%s: %d windows (vis: '%s')", label, node->ID, node->IsVisible ? "" : " (hidden)", node->Windows.size, node->VisibleWindow ? node->VisibleWindow->Name : "NULL");
    else
        open = TreeNodeEx((void*)(intptr_t)node->ID, tree_node_flags, "%s 0x%04X%s: %s split (vis: '%s')", label, node->ID, node->IsVisible ? "" : " (hidden)", (node->SplitAxis == Axis::X) ? "horizontal" : (node->SplitAxis == Axis::Y) ? "vertical" : "n/a", node->VisibleWindow ? node->VisibleWindow->Name : "NULL");
    if (!is_alive) { pop_style_color(); }
    if (is_active && IsItemHovered())
        if (ImGuiWindow* window = node->HostWindow ? node->HostWindow : node->VisibleWindow)
            get_foreground_draw_list(window)->AddRect(node.pos, node.pos + node.size, IM_COL32(255, 255, 0, 255));
    if (open)
    {
        IM_ASSERT(node->ChildNodes[0] == NULL || node->ChildNodes[0]->ParentNode == node);
        IM_ASSERT(node->ChildNodes[1] == NULL || node->ChildNodes[1]->ParentNode == node);
        BulletText("pos (%.0,%.0), size (%.0, %.0) Ref (%.0, %.0)",
            node.pos.x, node.pos.y, node.size.x, node.size.y, node.sizeRef.x, node.sizeRef.y);
        DebugNodeWindow(node->HostWindow, "host_window");
        DebugNodeWindow(node->VisibleWindow, "visible_window");
        BulletText("SelectedTabID: 0x%08X, LastFocusedNodeID: 0x%08X", node->SelectedTabId, node->LastFocusedNodeId);
        BulletText("Misc:%s%s%s%s%s%s%s",
            node->IsDockSpace() ? " is_dock_space" : "",
            node->IsCentralNode() ? " is_central_node" : "",
            is_alive ? " IsAlive" : "", is_active ? " IsActive" : "", node->IsFocused ? " is_focused" : "",
            node->WantLockSizeOnce ? " WantLockSizeOnce" : "",
            node->HasCentralNodeChild ? " has_central_node_child" : "");
        if (TreeNode("flags", "flags Merged: 0x%04X, Local: 0x%04X, InWindows: 0x%04X, Shared: 0x%04X", node->MergedFlags, node->LocalFlags, node->LocalFlagsInWindows, node->SharedFlags))
        {
            if (BeginTable("flags", 4))
            {
                TableNextColumn(); DebugNodeDockNodeFlags(&node->MergedFlags, "merged_flags", false);
                TableNextColumn(); DebugNodeDockNodeFlags(&node->LocalFlags, "local_flags", true);
                TableNextColumn(); DebugNodeDockNodeFlags(&node->LocalFlagsInWindows, "local_flags_in_windows", false);
                TableNextColumn(); DebugNodeDockNodeFlags(&node->SharedFlags, "shared_flags", true);
                EndTable();
            }
            TreePop();
        }
        if (node->ParentNode)
            DebugNodeDockNode(node->ParentNode, "parent_node");
        if (node->ChildNodes[0])
            DebugNodeDockNode(node->ChildNodes[0], "Child[0]");
        if (node->ChildNodes[1])
            DebugNodeDockNode(node->ChildNodes[1], "Child[1]");
        if (node->TabBar)
            DebugNodeTabBar(node->TabBar, "tab_bar");
        TreePop();
    }
}

// [DEBUG] Display contents of ImDrawList
// Note that both 'window' and 'viewport' may be NULL here. viewport is generally null of destroyed popups which previously owned a viewport.
void DebugNodeDrawList(ImGuiWindow* window, ImGuiViewportP* viewport, const ImDrawList* draw_list, const char* label)
{
    ImGuiContext& g = *GImGui;
    ImGuiMetricsConfig* cfg = &g.DebugMetricsConfig;
    int cmd_count = draw_list.cmd_buffer.size;
    if (cmd_count > 0 && draw_list.cmd_buffer.back().elem_count == 0 && draw_list.cmd_buffer.back().user_callback == NULL)
        cmd_count--;
    bool node_open = TreeNode(draw_list, "%s: '%s' %d vtx, %d indices, %d cmds", label, draw_list->_OwnerName ? draw_list->_OwnerName : "", draw_list->VtxBuffer.size, draw_list->IdxBuffer.size, cmd_count);
    if (draw_list == GetWindowDrawList())
    {
        SameLine();
        TextColored(Vector4D(1.0, 0.4, 0.4, 1.0), "CURRENTLY APPENDING"); // Can't display stats for active draw list! (we don't have the data double-buffered)
        if (node_open)
            TreePop();
        return;
    }

    ImDrawList* fg_draw_list = viewport ? get_foreground_draw_list(viewport) : NULL; // Render additional visuals into the top-most draw list
    if (window && IsItemHovered() && fg_draw_list)
        fg_draw_list->AddRect(window.pos, window.pos + window.size, IM_COL32(255, 255, 0, 255));
    if (!node_open)
        return;

    if (window && !window.was_active)
        TextDisabled("Warning: owning window is inactive. This draw_list is not being rendered!");

    for (const ImDrawCmd* pcmd = draw_list.cmd_buffer.data; pcmd < draw_list.cmd_buffer.data + cmd_count; pcmd += 1)
    {
        if (pcmd->UserCallback)
        {
            BulletText("Callback %p, user_data %p", pcmd->UserCallback, pcmd->UserCallbackData);
            continue;
        }

        char buf[300];
        ImFormatString(buf, IM_ARRAYSIZE(buf), "DrawCmd:%5d tris, Tex 0x%p, clip_rect (%4.0,%4.0)-(%4.0,%4.0)",
            pcmd->ElemCount / 3, (void*)(intptr_t)pcmd->TextureId,
            pcmd->ClipRect.x, pcmd->ClipRect.y, pcmd->ClipRect.z, pcmd->ClipRect.w);
        bool pcmd_node_open = TreeNode((void*)(pcmd - draw_list.cmd_buffer.begin()), "%s", buf);
        if (IsItemHovered() && (cfg->ShowDrawCmdMesh || cfg->ShowDrawCmdBoundingBoxes) && fg_draw_list)
            DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, cfg->ShowDrawCmdMesh, cfg->ShowDrawCmdBoundingBoxes);
        if (!pcmd_node_open)
            continue;

        // Calculate approximate coverage area (touched pixel count)
        // This will be in pixels squared as long there's no post-scaling happening to the renderer output.
        const ImDrawIdx* idx_buffer = (draw_list->IdxBuffer.size > 0) ? draw_list->IdxBuffer.data : NULL;
        const ImDrawVert* vtx_buffer = draw_list->VtxBuffer.data + pcmd->VtxOffset;
        float total_area = 0.0;
        for (unsigned int idx_n = pcmd->IdxOffset; idx_n < pcmd->IdxOffset + pcmd->ElemCount; )
        {
            Vector2D triangle[3];
            for (int n = 0; n < 3; n += 1, idx_n += 1)
                triangle[n] = vtx_buffer[idx_buffer ? idx_buffer[idx_n] : idx_n].pos;
            total_area += ImTriangleArea(triangle[0], triangle[1], triangle[2]);
        }

        // Display vertex information summary. Hover to get all triangles drawn in wire-frame
        ImFormatString(buf, IM_ARRAYSIZE(buf), "Mesh: elem_count: %d, vtx_offset: +%d, idx_offset: +%d, Area: ~%0.f px", pcmd->ElemCount, pcmd->VtxOffset, pcmd->IdxOffset, total_area);
        Selectable(buf);
        if (IsItemHovered() && fg_draw_list)
            DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, true, false);

        // Display individual triangles/vertices. Hover on to get the corresponding triangle highlighted.
        ImGuiListClipper clipper;
        clipper.begin(pcmd->ElemCount / 3); // Manually coarse clip our print out of individual vertices to save CPU, only items that may be visible.
        while (clipper.Step())
            for (int prim = clipper.DisplayStart, idx_i = pcmd->IdxOffset + clipper.DisplayStart * 3; prim < clipper.DisplayEnd; prim += 1)
            {
                char* buf_p = buf, * buf_end = buf + IM_ARRAYSIZE(buf);
                Vector2D triangle[3];
                for (int n = 0; n < 3; n += 1, idx_i += 1)
                {
                    const ImDrawVert& v = vtx_buffer[idx_buffer ? idx_buffer[idx_i] : idx_i];
                    triangle[n] = v.pos;
                    buf_p += ImFormatString(buf_p, buf_end - buf_p, "%s %04d: pos (%8.2,%8.2), uv (%.6,%.6), col %08X\n",
                        (n == 0) ? "Vert:" : "     ", idx_i, v.pos.x, v.pos.y, v.uv.x, v.uv.y, v.col);
                }

                Selectable(buf, false);
                if (fg_draw_list && IsItemHovered())
                {
                    ImDrawListFlags backup_flags = fg_draw_list.flags;
                    fg_draw_list.flags &= ~DrawListFlags::AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
                    fg_draw_list->AddPolyline(triangle, 3, IM_COL32(255, 255, 0, 255), DrawFlags::Closed, 1.0);
                    fg_draw_list.flags = backup_flags;
                }
            }
        TreePop();
    }
    TreePop();
}

// [DEBUG] Display mesh/aabb of a ImDrawCmd
void DebugNodeDrawCmdShowMeshAndBoundingBox(ImDrawList* out_draw_list, const ImDrawList* draw_list, const ImDrawCmd* draw_cmd, bool show_mesh, bool show_aabb)
{
    IM_ASSERT(show_mesh || show_aabb);

    // Draw wire-frame version of all triangles
    Rect clip_rect = draw_cmd->ClipRect;
    Rect vtxs_rect(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    ImDrawListFlags backup_flags = out_draw_list.flags;
    out_draw_list.flags &= ~DrawListFlags::AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
    for (unsigned int idx_n = draw_cmd->IdxOffset, idx_end = draw_cmd->IdxOffset + draw_cmd->ElemCount; idx_n < idx_end; )
    {
        ImDrawIdx* idx_buffer = (draw_list->IdxBuffer.size > 0) ? draw_list->IdxBuffer.data : NULL; // We don't hold on those pointers past iterations as ->AddPolyline() may invalidate them if out_draw_list==draw_list
        ImDrawVert* vtx_buffer = draw_list->VtxBuffer.data + draw_cmd->VtxOffset;

        Vector2D triangle[3];
        for (int n = 0; n < 3; n += 1, idx_n += 1)
            vtxs_rect.Add((triangle[n] = vtx_buffer[idx_buffer ? idx_buffer[idx_n] : idx_n].pos));
        if (show_mesh)
            out_draw_list->AddPolyline(triangle, 3, IM_COL32(255, 255, 0, 255), DrawFlags::Closed, 1.0); // In yellow: mesh triangles
    }
    // Draw bounding boxes
    if (show_aabb)
    {
        out_draw_list->AddRect(f32::floor(clip_rect.min), f32::floor(clip_rect.max), IM_COL32(255, 0, 255, 255)); // In pink: clipping rectangle submitted to GPU
        out_draw_list->AddRect(f32::floor(vtxs_rect.min), f32::floor(vtxs_rect.max), IM_COL32(0, 255, 255, 255)); // In cyan: bounding box of triangles
    }
    out_draw_list.flags = backup_flags;
}

// [DEBUG] Display details for a single font, called by ShowStyleEditor().
void DebugNodeFont(ImFont* font)
{
    bool opened = TreeNode(font, "font: \"%s\"\n%.2 px, %d glyphs, %d file(s)",
        font->ConfigData ? font->ConfigData[0].Name : "", font->FontSize, font->Glyphs.size, font->ConfigDataCount);
    SameLine();
    if (SmallButton("Set as default"))
        GetIO().FontDefault = font;
    if (!opened)
        return;

    // Display preview text
    PushFont(font);
    Text("The quick brown fox jumps over the lazy dog");
    PopFont();

    // Display details
    SetNextItemWidth(GetFontSize() * 8);
    DragFloat("font scale", &font->Scale, 0.005, 0.3, 2.0, "%.1");
    SameLine(); MetricsHelpMarker(
        "Note than the default embedded font is NOT meant to be scaled.\n\n"
        "font are currently rendered into bitmaps at a given size at the time of building the atlas. "
        "You may oversample them to get some flexibility with scaling. "
        "You can also render at multiple sizes and select which one to use at runtime.\n\n"
        "(Glimmer of hope: the atlas system will be rewritten in the future to make scaling more flexible.)");
    Text("ascent: %f, descent: %f, height: %f", font->Ascent, font->Descent, font->Ascent - font->Descent);
    char c_str[5];
    Text("Fallback character: '%s' (U+%04X)", ImTextCharToUtf8(c_str, font->FallbackChar), font->FallbackChar);
    Text("Ellipsis character: '%s' (U+%04X)", ImTextCharToUtf8(c_str, font->EllipsisChar), font->EllipsisChar);
    const int surface_sqrt = ImSqrt((float)font->MetricsTotalSurface);
    Text("Texture Area: about %d px ~%dx%d px", font->MetricsTotalSurface, surface_sqrt, surface_sqrt);
    for (int config_i = 0; config_i < font->ConfigDataCount; config_i += 1)
        if (font->ConfigData)
            if (const ImFontConfig* cfg = &font->ConfigData[config_i])
                BulletText("Input %d: \'%s\', Oversample: (%d,%d), pixel_snap_h: %d, Offset: (%.1,%.1)",
                    config_i, cfg->Name, cfg->OversampleH, cfg->OversampleV, cfg->PixelSnapH, cfg->GlyphOffset.x, cfg->GlyphOffset.y);

    // Display all glyphs of the fonts in separate pages of 256 characters
    if (TreeNode("glyphs", "glyphs (%d)", font->Glyphs.size))
    {
        ImDrawList* draw_list = GetWindowDrawList();
        const ImU32 glyph_col = get_color_u32(StyleColor::Text);
        const float cell_size = font->FontSize * 1;
        const float cell_spacing = GetStyle().ItemSpacing.y;
        for (unsigned int base = 0; base <= IM_UNICODE_CODEPOINT_MAX; base += 256)
        {
            // Skip ahead if a large bunch of glyphs are not present in the font (test in chunks of 4k)
            // This is only a small optimization to reduce the number of iterations when IM_UNICODE_MAX_CODEPOINT
            // is large // (if ImWchar==ImWchar32 we will do at least about 272 queries here)
            if (!(base & 4095) && font->IsGlyphRangeUnused(base, base + 4095))
            {
                base += 4096 - 256;
                continue;
            }

            int count = 0;
            for (unsigned int n = 0; n < 256; n += 1)
                if (font->FindGlyphNoFallback((ImWchar)(base + n)))
                    count += 1;
            if (count <= 0)
                continue;
            if (!TreeNode((void*)(intptr_t)base, "U+%04X..U+%04X (%d %s)", base, base + 255, count, count > 1 ? "glyphs" : "glyph"))
                continue;

            // Draw a 16x16 grid of glyphs
            Vector2D base_pos = GetCursorScreenPos();
            for (unsigned int n = 0; n < 256; n += 1)
            {
                // We use ImFont::render_char as a shortcut because we don't have UTF-8 conversion functions
                // available here and thus cannot easily generate a zero-terminated UTF-8 encoded string.
                Vector2D cell_p1(base_pos.x + (n % 16) * (cell_size + cell_spacing), base_pos.y + (n / 16) * (cell_size + cell_spacing));
                Vector2D cell_p2(cell_p1.x + cell_size, cell_p1.y + cell_size);
                const ImFontGlyph* glyph = font->FindGlyphNoFallback((ImWchar)(base + n));
                draw_list->AddRect(cell_p1, cell_p2, glyph ? IM_COL32(255, 255, 255, 100) : IM_COL32(255, 255, 255, 50));
                if (!glyph)
                    continue;
                font->RenderChar(draw_list, cell_size, cell_p1, glyph_col, (ImWchar)(base + n));
                if (IsMouseHoveringRect(cell_p1, cell_p2))
                {
                    BeginTooltip();
                    DebugNodeFontGlyph(font, glyph);
                    EndTooltip();
                }
            }
            Dummy(Vector2D::new((cell_size + cell_spacing) * 16, (cell_size + cell_spacing) * 16));
            TreePop();
        }
        TreePop();
    }
    TreePop();
}

void DebugNodeFontGlyph(ImFont*, const ImFontGlyph* glyph)
{
    Text("codepoint: U+%04X", glyph->Codepoint);
    Separator();
    Text("visible: %d", glyph->Visible);
    Text("advance_x: %.1", glyph->AdvanceX);
    Text("pos: (%.2,%.2)->(%.2,%.2)", glyph->X0, glyph->Y0, glyph->X1, glyph->Y1);
    Text("UV: (%.3,%.3)->(%.3,%.3)", glyph->U0, glyph->V0, glyph->U1, glyph->V1);
}

// [DEBUG] Display contents of ImGuiStorage
void DebugNodeStorage(ImGuiStorage* storage, const char* label)
{
    if (!TreeNode(label, "%s: %d entries, %d bytes", label, storage->Data.size, storage->Data.size_in_bytes()))
        return;
    for (int n = 0; n < storage->Data.size; n += 1)
    {
        const ImGuiStorage::ImGuiStoragePair& p = storage->Data[n];
        BulletText("Key 0x%08X value { i: %d }", p.key, p.val_i); // Important: we currently don't store a type, real value may not be integer.
    }
    TreePop();
}

// [DEBUG] Display contents of ImGuiTabBar
void DebugNodeTabBar(ImGuiTabBar* tab_bar, const char* label)
{
    // Standalone tab bars (not associated to docking/windows functionality) currently hold no discernible strings.
    char buf[256];
    char* p = buf;
    const char* buf_end = buf + IM_ARRAYSIZE(buf);
    const bool is_active = (tab_bar->PrevFrameVisible >= GetFrameCount() - 2);
    p += ImFormatString(p, buf_end - p, "%s 0x%08X (%d tabs)%s", label, tab_bar->ID, tab_bar->Tabs.size, is_active ? "" : " *Inactive*");
    p += ImFormatString(p, buf_end - p, "  { ");
    for (int tab_n = 0; tab_n < ImMin(tab_bar->Tabs.size, 3); tab_n += 1)
    {
        ImGuiTabItem* tab = &tab_bar->Tabs[tab_n];
        p += ImFormatString(p, buf_end - p, "%s'%s'",
            tab_n > 0 ? ", " : "", (tab->Window || tab->NameOffset != -1) ? tab_bar->GetTabName(tab) : "???");
    }
    p += ImFormatString(p, buf_end - p, (tab_bar->Tabs.size > 3) ? " ... }" : " } ");
    if (!is_active) { push_style_color(StyleColor::Text, GetStyleColorVec4(StyleColor::TextDisabled)); }
    bool open = TreeNode(label, "%s", buf);
    if (!is_active) { pop_style_color(); }
    if (is_active && IsItemHovered())
    {
        ImDrawList* draw_list = get_foreground_draw_list();
        draw_list->AddRect(tab_bar->BarRect.min, tab_bar->BarRect.max, IM_COL32(255, 255, 0, 255));
        draw_list->AddLine(Vector2D::new(tab_bar->ScrollingRectMinX, tab_bar->BarRect.min.y), Vector2D::new(tab_bar->ScrollingRectMinX, tab_bar->BarRect.max.y), IM_COL32(0, 255, 0, 255));
        draw_list->AddLine(Vector2D::new(tab_bar->ScrollingRectMaxX, tab_bar->BarRect.min.y), Vector2D::new(tab_bar->ScrollingRectMaxX, tab_bar->BarRect.max.y), IM_COL32(0, 255, 0, 255));
    }
    if (open)
    {
        for (int tab_n = 0; tab_n < tab_bar->Tabs.size; tab_n += 1)
        {
            const ImGuiTabItem* tab = &tab_bar->Tabs[tab_n];
            PushID(tab);
            if (SmallButton("<")) { TabBarQueueReorder(tab_bar, tab, -1); } SameLine(0, 2);
            if (SmallButton(">")) { TabBarQueueReorder(tab_bar, tab, +1); } SameLine();
            Text("%02d%c Tab 0x%08X '%s' Offset: %.1, width: %.1/%.1",
                tab_n, (tab->ID == tab_bar->SelectedTabId) ? '*' : ' ', tab->ID, (tab->Window || tab->NameOffset != -1) ? tab_bar->GetTabName(tab) : "???", tab->Offset, tab->Width, tab->ContentWidth);
            PopID();
        }
        TreePop();
    }
}

void DebugNodeViewport(ImGuiViewportP* viewport)
{
    SetNextItemOpen(true, ImGuiCond_Once);
    if (TreeNode((void*)(intptr_t)viewport->ID, "viewport #%d, id: 0x%08X, Parent: 0x%08X, window: \"%s\"", viewport->Idx, viewport->ID, viewport->ParentViewportId, viewport->Window ? viewport->Window->Name : "N/A"))
    {
        ImGuiWindowFlags flags = viewport.flags;
        BulletText("Main pos: (%.0,%.0), size: (%.0,%.0)\nWorkArea Offset Left: %.0 Top: %.0, Right: %.0, Bottom: %.0\nMonitor: %d, dpi_scale: %.0%%",
            viewport.pos.x, viewport.pos.y, viewport.size.x, viewport.size.y,
            viewport->WorkOffsetMin.x, viewport->WorkOffsetMin.y, viewport->WorkOffsetMax.x, viewport->WorkOffsetMax.y,
            viewport->PlatformMonitor, viewport->DpiScale * 100.0);
        if (viewport->Idx > 0) { SameLine(); if (SmallButton("Reset pos")) { viewport.pos = Vector2D::new(200, 200); viewport.update_work_rect(); if (viewport->Window) viewport->Window.pos = viewport.pos; } }
        BulletText("flags: 0x%04X =%s%s%s%s%s%s%s%s%s%s%s%s", viewport.flags,
            //(flags & ImGuiViewportFlags_IsPlatformWindow) ? " IsPlatformWindow" : "", // Omitting because it is the standard
            (flags & ImGuiViewportFlags_IsPlatformMonitor) ? " IsPlatformMonitor" : "",
            (flags & ViewportFlags::OwnedByApp) ? " OwnedByApp" : "",
            (flags & ImGuiViewportFlags_NoDecoration) ? " NoDecoration" : "",
            (flags & ImGuiViewportFlags_NoTaskBarIcon) ? " NoTaskBarIcon" : "",
            (flags & ImGuiViewportFlags_NoFocusOnAppearing) ? " NoFocusOnAppearing" : "",
            (flags & ImGuiViewportFlags_NoFocusOnClick) ? " NoFocusOnClick" : "",
            (flags & ViewportFlags::NoInputs) ? " NoInputs" : "",
            (flags & ImGuiViewportFlags_NoRendererClear) ? " NoRendererClear" : "",
            (flags & ImGuiViewportFlags_TopMost) ? " TopMost" : "",
            (flags & ImGuiViewportFlags_Minimized) ? " Minimized" : "",
            (flags & ImGuiViewportFlags_NoAutoMerge) ? " NoAutoMerge" : "",
            (flags & ImGuiViewportFlags_CanHostOtherWindows) ? " CanHostOtherWindows" : "");
        for (int layer_i = 0; layer_i < IM_ARRAYSIZE(viewport.draw_data_builder.layers); layer_i += 1)
            for (int draw_list_i = 0; draw_list_i < viewport.draw_data_builder.layers[layer_i].size; draw_list_i += 1)
                DebugNodeDrawList(NULL, viewport, viewport.draw_data_builder.layers[layer_i][draw_list_i], "draw_list");
        TreePop();
    }
}

void DebugNodeWindow(ImGuiWindow* window, const char* label)
{
    if (window == NULL)
    {
        BulletText("%s: NULL", label);
        return;
    }

    ImGuiContext& g = *GImGui;
    const bool is_active = window.was_active;
    ImGuiTreeNodeFlags tree_node_flags = (window == g.nav_window) ? ImGuiTreeNodeFlags_Selected : ImGuiTreeNodeFlags_None;
    if (!is_active) { push_style_color(StyleColor::Text, GetStyleColorVec4(StyleColor::TextDisabled)); }
    const bool open = TreeNodeEx(label, tree_node_flags, "%s '%s'%s", label, window.Name, is_active ? "" : " *Inactive*");
    if (!is_active) { pop_style_color(); }
    if (IsItemHovered() && is_active)
        get_foreground_draw_list(window)->AddRect(window.pos, window.pos + window.size, IM_COL32(255, 255, 0, 255));
    if (!open)
        return;

    if (window.memory_compacted)
        TextDisabled("Note: some memory buffers have been compacted/freed.");

    ImGuiWindowFlags flags = window.flags;
    DebugNodeDrawList(window, window.viewport, window.draw_list, "draw_list");
    BulletText("pos: (%.1,%.1), size: (%.1,%.1), content_size (%.1,%.1) Ideal (%.1,%.1)", window.pos.x, window.pos.y, window.size.x, window.size.y, window.ContentSize.x, window.ContentSize.y, window.ContentSizeIdeal.x, window.ContentSizeIdeal.y);
    BulletText("flags: 0x%08X (%s%s%s%s%s%s%s%s%s..)", flags,
        (flags & WindowFlags::ChildWindow)  ? "Child " : "",      (flags & WindowFlags::Tooltip)     ? "Tooltip "   : "",  (flags & WindowFlags::Popup) ? "Popup " : "",
        (flags & WindowFlags::Modal)        ? "Modal " : "",      (flags & WindowFlags::ChildMenu)   ? "ChildMenu " : "",  (flags & WindowFlags::NoSavedSettings) ? "NoSavedSettings " : "",
        (flags & WindowFlags::NoMouseInputs)? "NoMouseInputs":"", (flags & WindowFlags::NoNavInputs) ? "NoNavInputs" : "", (flags & WindowFlags::AlwaysAutoResize) ? "AlwaysAutoResize" : "");
    BulletText("WindowClassId: 0x%08X", window.WindowClass.ClassId);
    BulletText("scroll: (%.2/%.2,%.2/%.2) Scrollbar:%s%s", window.scroll.x, window.scroll_max.x, window.scroll.y, window.scroll_max.y, window.scrollbar_x ? "x" : "", window.scrollbar_y ? "Y" : "");
    BulletText("active: %d/%d, write_accessed: %d, begin_order_within_context: %d", window.active, window.was_active, window.write_accessed, (window.active || window.was_active) ? window.BeginOrderWithinContext : -1);
    BulletText("appearing: %d, hidden: %d (CanSkip %d Cannot %d), skip_items: %d", window.Appearing, window.hidden, window..hidden_frames_can_skip_items, window.hidden_frames_cannot_skip_items, window.skip_items);
    for (int layer = 0; layer < NavLayer::COUNT; layer += 1)
    {
        Rect r = window.NavRectRel[layer];
        if (r.min.x >= r.max.y && r.min.y >= r.max.y)
        {
            BulletText("nav_last_ids[%d]: 0x%08X", layer, window.NavLastIds[layer]);
            continue;
        }
        BulletText("nav_last_ids[%d]: 0x%08X at +(%.1,%.1)(%.1,%.1)", layer, window.NavLastIds[layer], r.min.x, r.min.y, r.max.x, r.max.y);
        if (IsItemHovered())
            get_foreground_draw_list(window)->AddRect(r.min + window.pos, r.max + window.pos, IM_COL32(255, 255, 0, 255));
    }
    BulletText("nav_layers_active_mask: %x, nav_last_child_nav_window: %s", window.dc.nav_layers_active_mask, window.NavLastChildNavWindow ? window.NavLastChildNavWindow->Name : "NULL");

    BulletText("viewport: %d%s, viewport_id: 0x%08X, viewport_pos: (%.1,%.1)", window.viewport ? window.viewport->Idx : -1, window.viewport_owned ? " (Owned)" : "", window.viewport_id, window.viewport_pos.x, window.viewport_pos.y);
    BulletText("ViewportMonitor: %d", window.viewport ? window.viewport->PlatformMonitor : -1);
    BulletText("dock_id: 0x%04X, dock_order: %d, Act: %d, Vis: %d", window.DockId, window.DockOrder, window.dock_is_active, window.DockTabIsVisible);
    if (window.dock_node || window.DockNodeAsHost)
        DebugNodeDockNode(window.DockNodeAsHost ? window.DockNodeAsHost : window.dock_node, window.DockNodeAsHost ? "dock_node_as_host" : "dock_node");

    if (window.root_window != window)       { DebugNodeWindow(window.root_window, "RootWindow"); }
    if (window.root_window_dock_tree != window.root_window) { DebugNodeWindow(window.root_window_dock_tree, "root_window_dock_tree"); }
    if (window.parent_window != NULL)       { DebugNodeWindow(window.parent_window, "ParentWindow"); }
    if (window.dc.ChildWindows.size > 0)   { DebugNodeWindowsList(&window.dc.ChildWindows, "ChildWindows"); }
    if (window.ColumnsStorage.size > 0 && TreeNode("Columns", "Columns sets (%d)", window.ColumnsStorage.size))
    {
        for (int n = 0; n < window.ColumnsStorage.size; n += 1)
            DebugNodeColumns(&window.ColumnsStorage[n]);
        TreePop();
    }
    DebugNodeStorage(&window.StateStorage, "Storage");
    TreePop();
}

void DebugNodeWindowSettings(ImGuiWindowSettings* settings)
{
    Text("0x%08X \"%s\" pos (%d,%d) size (%d,%d) collapsed=%d",
        settings->ID, settings->GetName(), settings.pos.x, settings.pos.y, settings.size.x, settings.size.y, settings.collapsed);
}

void DebugNodeWindowsList(ImVector<ImGuiWindow*>* windows, const char* label)
{
    if (!TreeNode(label, "%s (%d)", label, windows.size))
        return;
    for (int i = windows.size - 1; i >= 0; i--) // Iterate front to back
    {
        PushID((*windows)[i]);
        DebugNodeWindow((*windows)[i], "window");
        PopID();
    }
    TreePop();
}

// FIXME-OPT: This is technically suboptimal, but it is simpler this way.
void DebugNodeWindowsListByBeginStackParent(ImGuiWindow** windows, int windows_size, ImGuiWindow* parent_in_begin_stack)
{
    for (int i = 0; i < windows_size; i += 1)
    {
        ImGuiWindow* window = windows[i];
        if (window.ParentWindowInBeginStack != parent_in_begin_stack)
            continue;
        char buf[20];
        ImFormatString(buf, IM_ARRAYSIZE(buf), "[%04d] window", window.BeginOrderWithinContext);
        //BulletText("[%04d] window '%s'", window->begin_order_within_context, window->name);
        DebugNodeWindow(window, buf);
        Indent();
        DebugNodeWindowsListByBeginStackParent(windows + i + 1, windows_size - i - 1, window);
        Unindent();
    }
}

//-----------------------------------------------------------------------------
// [SECTION] DEBUG LOG
//-----------------------------------------------------------------------------

void DebugLog(const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    DebugLogV(fmt, args);
    va_end(args);
}

void DebugLogV(const char* fmt, va_list args)
{
    ImGuiContext& g = *GImGui;
    const int old_size = g.DebugLogBuf.size();
    g.DebugLogBuf.appendf("[%05d] ", g.frame_count);
    g.DebugLogBuf.appendfv(fmt, args);
    if (g.DebugLogFlags & ImGuiDebugLogFlags_OutputToTTY)
        IMGUI_DEBUG_PRINTF("%s", g.DebugLogBuf.begin() + old_size);
}

void ShowDebugLogWindow(bool* p_open)
{
    ImGuiContext& g = *GImGui;
    if (!(g.next_window_data.flags & NextWindowDataFlags::HasSize))
        set_next_window_size(Vector2D::new(0.0, GetFontSize() * 12.0), Cond::FirstUseEver);
    if (!begin("Dear ImGui Debug Log", p_open) || GetCurrentWindow()->BeginCount > 1)
    {
        end();
        return;
    }

    AlignTextToFramePadding();
    Text("Log events:");
    SameLine(); CheckboxFlags("All", &g.DebugLogFlags, ImGuiDebugLogFlags_EventMask_);
    SameLine(); CheckboxFlags("active_id", &g.DebugLogFlags, ImGuiDebugLogFlags_EventActiveId);
    SameLine(); CheckboxFlags("Focus", &g.DebugLogFlags, ImGuiDebugLogFlags_EventFocus);
    SameLine(); CheckboxFlags("Popup", &g.DebugLogFlags, ImGuiDebugLogFlags_EventPopup);
    SameLine(); CheckboxFlags("Nav", &g.DebugLogFlags, ImGuiDebugLogFlags_EventNav);
    SameLine(); CheckboxFlags("Docking", &g.DebugLogFlags, ImGuiDebugLogFlags_EventDocking);
    SameLine(); CheckboxFlags("viewport", &g.DebugLogFlags, ImGuiDebugLogFlags_EventViewport);

    if (SmallButton("clear"))
        g.DebugLogBuf.clear();
    SameLine();
    if (SmallButton("Copy"))
        SetClipboardText(g.DebugLogBuf.c_str());
    begin_child("##log", Vector2D::new(0.0, 0.0), true, WindowFlags::AlwaysVerticalScrollbar | WindowFlags::AlwaysHorizontalScrollbar);
    TextUnformatted(g.DebugLogBuf.begin(), g.DebugLogBuf.end()); // FIXME-OPT: Could use a line index, but TextUnformatted() has a semi-decent fast path for large text.
    if (GetScrollY() >= GetScrollMaxY())
        SetScrollHereY(1.0);
    end_child();

    end();
}

//-----------------------------------------------------------------------------
// [SECTION] OTHER DEBUG TOOLS (ITEM PICKER, STACK TOOL)
//-----------------------------------------------------------------------------

// [DEBUG] Item picker tool - start with DebugStartItemPicker() - useful to visually select an item and break into its call-stack.
void update_debug_tool_item_picker()
{
    ImGuiContext& g = *GImGui;
    g.DebugItemPickerBreakId = 0;
    if (!g.DebugItemPickerActive)
        return;

    const ImGuiID hovered_id = g.hovered_id_previous_frame;
    SetMouseCursor(ImGuiMouseCursor_Hand);
    if (IsKeyPressed(ImGuiKey_Escape))
        g.DebugItemPickerActive = false;
    if (IsMouseClicked(0) && hovered_id)
    {
        g.DebugItemPickerBreakId = hovered_id;
        g.DebugItemPickerActive = false;
    }
    SetNextWindowBgAlpha(0.60);
    BeginTooltip();
    Text("hovered_id: 0x%08X", hovered_id);
    Text("Press ESC to abort picking.");
    TextColored(GetStyleColorVec4(hovered_id ? StyleColor::Text : StyleColor::TextDisabled), "Click to break in debugger!");
    EndTooltip();
}

// [DEBUG] Stack Tool: update queries. Called by NewFrame()
void update_debug_tool_stack_queries()
{
    ImGuiContext& g = *GImGui;
    ImGuiStackTool* tool = &g.DebugStackTool;

    // clear hook when stack tool is not visible
    g.debug_hook_id_info = 0;
    if (g.frame_count != tool->LastActiveFrame + 1)
        return;

    // Update queries. The steps are: -1: query Stack, >= 0: query each stack item
    // We can only perform 1 id Info query every frame. This is designed so the GetID() tests are cheap and constant-time
    const ImGuiID query_id = g.hovered_id_previous_frame ? g.hovered_id_previous_frame : g.active_id;
    if (tool->QueryId != query_id)
    {
        tool->QueryId = query_id;
        tool->StackLevel = -1;
        tool->Results.resize(0);
    }
    if (query_id == 0)
        return;

    // Advance to next stack level when we got our result, or after 2 frames (in case we never get a result)
    int stack_level = tool->StackLevel;
    if (stack_level >= 0 && stack_level < tool->Results.size)
        if (tool->Results[stack_level].QuerySuccess || tool->Results[stack_level].QueryFrameCount > 2)
            tool->StackLevel += 1;

    // Update hook
    stack_level = tool->StackLevel;
    if (stack_level == -1)
        g.debug_hook_id_info = query_id;
    if (stack_level >= 0 && stack_level < tool->Results.size)
    {
        g.debug_hook_id_info = tool->Results[stack_level].id;
        tool->Results[stack_level].QueryFrameCount += 1;
    }
}

// [DEBUG] Stack tool: hooks called by GetID() family functions
void debug_hook_id_info(ImGuiID id, ImGuiDataType data_type, const void* data_id, const void* data_id_end)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    ImGuiStackTool* tool = &g.DebugStackTool;

    // Step 0: stack query
    // This assume that the id was computed with the current id stack, which tends to be the case for our widget.
    if (tool->StackLevel == -1)
    {
        tool->StackLevel += 1;
        tool->Results.resize(window.IDStack.size + 1, ImGuiStackLevelInfo());
        for (int n = 0; n < window.IDStack.size + 1; n += 1)
            tool->Results[n].id = (n < window.IDStack.size) ? window.IDStack[n] : id;
        return;
    }

    // Step 1+: query for individual level
    IM_ASSERT(tool->StackLevel >= 0);
    if (tool->StackLevel != window.IDStack.size)
        return;
    ImGuiStackLevelInfo* info = &tool->Results[tool->StackLevel];
    IM_ASSERT(info->ID == id && info->QueryFrameCount > 0);

    switch (data_type)
    {
    case ImGuiDataType_S32:
        ImFormatString(info->Desc, IM_ARRAYSIZE(info->Desc), "%d", (intptr_t)data_id);
        break;
    case DataType::String:
        ImFormatString(info->Desc, IM_ARRAYSIZE(info->Desc), "%.*s", data_id_end ? ((const char*)data_id_end - (const char*)data_id) : strlen((const char*)data_id), (const char*)data_id);
        break;
    case ImGuiDataType_Pointer:
        ImFormatString(info->Desc, IM_ARRAYSIZE(info->Desc), "(void*)0x%p", data_id);
        break;
    case ImGuiDataType_ID:
        if (info->Desc[0] != 0) // PushOverrideID() is often used to avoid hashing twice, which would lead to 2 calls to debug_hook_id_info(). We prioritize the first one.
            return;
        ImFormatString(info->Desc, IM_ARRAYSIZE(info->Desc), "0x%08X [override]", id);
        break;
    default:
        IM_ASSERT(0);
    }
    info->QuerySuccess = true;
    info->DataType = data_type;
}

static int StackToolFormatLevelInfo(ImGuiStackTool* tool, int n, bool format_for_ui, char* buf, size_t buf_size)
{
    ImGuiStackLevelInfo* info = &tool->Results[n];
    ImGuiWindow* window = (info->Desc[0] == 0 && n == 0) ? FindWindowByID(info->ID) : NULL;
    if (window)                                                                 // Source: window name (because the root id don't call GetID() and so doesn't get hooked)
        return ImFormatString(buf, buf_size, format_for_ui ? "\"%s\" [window]" : "%s", window.Name);
    if (info->QuerySuccess)                                                     // Source: GetID() hooks (prioritize over ItemInfo() because we frequently use patterns like: PushID(str), Button("") where they both have same id)
        return ImFormatString(buf, buf_size, (format_for_ui && info->DataType == DataType::String) ? "\"%s\"" : "%s", info->Desc);
    if (tool->StackLevel < tool->Results.size)                                  // Only start using fallback below when all queries are done, so during queries we don't flickering ??? markers.
        return (*buf = 0);
#ifdef IMGUI_ENABLE_TEST_ENGINE
    if (const char* label = ImGuiTestEngine_FindItemDebugLabel(GImGui, info->ID))   // Source: ImGuiTestEngine's ItemInfo()
        return ImFormatString(buf, buf_size, format_for_ui ? "??? \"%s\"" : "%s", label);

    return ImFormatString(buf, buf_size, "???");
}

// Stack Tool: Display UI
void ShowStackToolWindow(bool* p_open)
{
    ImGuiContext& g = *GImGui;
    if (!(g.next_window_data.flags & NextWindowDataFlags::HasSize))
        set_next_window_size(Vector2D::new(0.0, GetFontSize() * 8.0), Cond::FirstUseEver);
    if (!begin("Dear ImGui Stack Tool", p_open) || GetCurrentWindow()->BeginCount > 1)
    {
        end();
        return;
    }

    // Display hovered/active status
    ImGuiStackTool* tool = &g.DebugStackTool;
    const ImGuiID hovered_id = g.hovered_id_previous_frame;
    const ImGuiID active_id = g.active_id;
#ifdef IMGUI_ENABLE_TEST_ENGINE
    Text("hovered_id: 0x%08X (\"%s\"), active_id:  0x%08X (\"%s\")", hovered_id, hovered_id ? ImGuiTestEngine_FindItemDebugLabel(&g, hovered_id) : "", active_id, active_id ? ImGuiTestEngine_FindItemDebugLabel(&g, active_id) : "");
#else
    Text("hovered_id: 0x%08X, active_id:  0x%08X", hovered_id, active_id);

    SameLine();
    MetricsHelpMarker("Hover an item with the mouse to display elements of the id Stack leading to the item's final id.\nEach level of the stack correspond to a PushID() call.\nAll levels of the stack are hashed together to make the final id of a widget (id displayed at the bottom level of the stack).\nRead FAQ entry about the id stack for details.");

    // CTRL+C to copy path
    const float time_since_copy = g.time - tool->CopyToClipboardLastTime;
    Checkbox("Ctrl+C: copy path to clipboard", &tool->CopyToClipboardOnCtrlC);
    SameLine();
    TextColored((time_since_copy >= 0.0 && time_since_copy < 0.75 && f32::mod(time_since_copy, 0.25) < 0.25 * 0.5) ? Vector4D(1.f, 1.f, 0.3, 1.f) : Vector4D(), "*COPIED*");
    if (tool->CopyToClipboardOnCtrlC && IsKeyDown(Key::ModCtrl) && IsKeyPressed(ImGuiKey_C))
    {
        tool->CopyToClipboardLastTime = g.time;
        char* p = g.TempBuffer.data;
        char* p_end = p + g.TempBuffer.size;
        for (int stack_n = 0; stack_n < tool->Results.size && p + 3 < p_end; stack_n += 1)
        {
            *p += 1 = '/';
            char level_desc[256];
            StackToolFormatLevelInfo(tool, stack_n, false, level_desc, IM_ARRAYSIZE(level_desc));
            for (int n = 0; level_desc[n] && p + 2 < p_end; n += 1)
            {
                if (level_desc[n] == '/')
                    *p += 1 = '\\';
                *p += 1 = level_desc[n];
            }
        }
        *p = '\0';
        SetClipboardText(g.TempBuffer.data);
    }

    // Display decorated stack
    tool->LastActiveFrame = g.frame_count;
    if (tool->Results.size > 0 && BeginTable("##table", 3, ImGuiTableFlags_Borders))
    {
        const float id_width = CalcTextSize("0xDDDDDDDD").x;
        TableSetupColumn("Seed", ImGuiTableColumnFlags_WidthFixed, id_width);
        TableSetupColumn("PushID", ImGuiTableColumnFlags_WidthStretch);
        TableSetupColumn("Result", ImGuiTableColumnFlags_WidthFixed, id_width);
        TableHeadersRow();
        for (int n = 0; n < tool->Results.size; n += 1)
        {
            ImGuiStackLevelInfo* info = &tool->Results[n];
            TableNextColumn();
            Text("0x%08X", (n > 0) ? tool->Results[n - 1].id : 0);
            TableNextColumn();
            StackToolFormatLevelInfo(tool, n, true, g.TempBuffer.data, g.TempBuffer.size);
            TextUnformatted(g.TempBuffer.data);
            TableNextColumn();
            Text("0x%08X", info->ID);
            if (n == tool->Results.size - 1)
                TableSetBgColor(ImGuiTableBgTarget_CellBg, get_color_u32(StyleColor::Header));
        }
        EndTable();
    }
    end();
}
