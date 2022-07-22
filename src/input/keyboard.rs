use std::collections::HashSet;
use crate::context::Context;
use crate::globals::GImGui;
use crate::input::ModFlags;

// static void ImGui::UpdateKeyboardInputs()
pub fn update_keyboard_inputs(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // ImGuiIO& io = g.io;
    let io = &mut g.io;

    // Import legacy keys or verify they are not used
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    if (io.backend_using_legacy_key_arrays == 0)
    {
        // Backend used new io.add_key_event() API: Good! Verify that old arrays are never written to externally.
        // for (int n = 0; n < ImGuiKey_LegacyNativeKey_END; n += 1) {
        // for n in 0 .. Key::LegacyNativeKey_END {
        //     // IM_ASSERT((io.keys_down[n] == false || IsKeyDown(n)) && "Backend needs to either only use io.add_key_event(), either only fill legacy io.keys_down[] + io.key_map[]. Not both!");
        // }
    }
    else
    {
        if g.frame_count == 0 {
            // for (int n = ImGuiKey_LegacyNativeKey_BEGIN; n < ImGuiKey_LegacyNativeKey_END; n += 1){
            //     IM_ASSERT(g.io.key_map[n] == -1 && "Backend is not allowed to write to io.key_map[0..511]!");
            // }
        }

        // build reverse KeyMap (Named -> Legacy)
        //for (int n = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n += 1){
        for n in Key::NamedKey_BEGIN .. Key::NamedKey_END {
            if io.key_map[n] != -1 {
                // IM_ASSERT(IsLegacyKey((ImGuiKey)io.key_map[n]));
                io.key_map[io.key_map[n]] = n;
            }
        }
        // Import legacy keys into new ones
        // for (int n = ImGuiKey_LegacyNativeKey_BEGIN; n < ImGuiKey_LegacyNativeKey_END; n += 1){
        for n in Key::LegacyNativeKey_BEGIN .. Key::LegacyNativeKey_END {
            if io.keys_down[n] || io.backend_using_legacy_key_arrays == 1 {
                // const ImGuiKey key = (ImGuiKey)(io.key_map[n] != -1? io.key_map[n]: n);
                let key = if io.key_map[n] != -1 { io.key_map[n] } else { n };
                // IM_ASSERT(io.key_map[n] == -1 || IsNamedKey(key));
                io.keys_data[key].down = io.keys_down[n];
                if key != n {
                    io.keys_down[key] = io.keys_down[n];
                } // Allow legacy code using io.keys_down[GetKeyIndex()] with old backends
                io.backend_using_legacy_key_arrays = 1;
            }
        }
        if io.backend_using_legacy_key_arrays == 1
        {
            io.keys_data[Key::ModCtrl].down = io.key_ctrl;
            io.keys_data[Key::ModShift].down = io.key_shift;
            io.keys_data[Key::ModAlt].down = io.key_alt;
            io.keys_data[Key::ModSuper].down = io.key_super;
        }
    }


    // Synchronize io.key_mods with individual modifiers io.KeyXXX bools
    io.key_mods = get_merged_mod_flags();

    // clear gamepad data if disabled
    if (io.backend_flags.contains(BackendFlags::HasGamepad)) == false {
        // for (int i = Key::Gamepad_BEGIN; i < Key::Gamepad_END; i += 1)
        for i in Key::Gamepad_BEGIN .. Key::Gamepad_END
        {
            io.keys_data[i - Key::KeysDataOffset].down = false;
            io.keys_data[i - Key::KeysDataOffset].analog_value = 0.0;
        }
    }

    // Update keys
    // for (int i = 0; i < IM_ARRAYSIZE(io.keys_data); i += 1)
    for i in 0 .. io.keys_data.len()
    {
        // ImGuiKeyData* key_data = &io.keys_data[i];
        let key_data = &mut io.keys_data[i];
        // key_data.down_durationPrev = key_data.down_duration;
        key_data.down_duration_prev = key_data.down_duration;
        key_data.down_duration = if key_data.down { (if key_data.down_duration < 0.0 { 0.0} else { key_data.down_duration + io.delta_time}) } else { -1.0};
    }
}

/// [Internal] Do not use directly (can read io.key_mods instead)
/// ImGuiModFlags ImGui::get_merged_mod_flags()
pub fn get_merged_mod_flags(g: &mut Context) -> HashSet<ModFlags>
{
    // ImGuiContext& g = *GImGui;
    let mut key_mods: HashSet<ModFlags> = HashSet::new();
    if g.io.key_ctrl { key_mods.insert(ModFlags::Ctrl); }
    if g.io.key_shift { key_mods.insert(ModFlags::Shift); }
    if g.io.key_alt { key_mods.insert(ModFlags::Alt); }
    if g.io.key_super { key_mods.insert(ModFlags::Super); }
    return key_mods;
}

// FIXME: Look into renaming this once we have settled the new Focus/Activation/TabStop system.
// void ImGui::PushAllowKeyboardFocus(bool allow_keyboard_focus)
pub fn push_allow_keyboard_focus(g: &mut Context, allow_keyboard_focus: bool)
{
    push_item_flag(ItemFlags::NoTabStop, !allow_keyboard_focus);
}

// void ImGui::PopAllowKeyboardFocus()
pub fn pop_allow_keyboard_focus(g: &mut Context)
{
    pop_item_flag();
}

// Note: this will likely be called ActivateItem() once we rework our Focus/Activation system!
// void SetKeyboardFocusHere(int offset)
pub fn set_keyboard_focus_here(g: &mut Context, offset: i32)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    // IM_ASSERT(offset >= -1);    // -1 is allowed but not below
    IMGUI_DEBUG_LOG_ACTIVEID("SetKeyboardFocusHere(%d) in window \"%s\"\n", offset, window.Name);

    // It makes sense in the vast majority of cases to never interrupt a drag and drop.
    // When we refactor this function into ActivateItem() we may want to make this an option.
    // moving_window is protected from most user inputs using SetActiveIdUsingNavAndKeys(), but
    // is also automatically dropped in the event g.active_id is stolen.
    if (g.drag_drop_active || g.moving_window != NULL)
    {
        IMGUI_DEBUG_LOG_ACTIVEID("SetKeyboardFocusHere() ignored while drag_drop_active!\n");
        return;
    }

    SetNavWindow(window);

    ImGuiScrollFlags scroll_flags = window.Appearing ? ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_AlwaysCenterY : ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_KeepVisibleEdgeY;
    NavMoveRequestSubmit(Direction::None, offset < 0 ? Direction::Up : Direction::Down, ImGuiNavMoveFlags_Tabbing | ImGuiNavMoveFlags_FocusApi, scroll_flags); // FIXME-NAV: Once we refactor tabbing, add LegacyApi flag to not activate non-inputable.
    if (offset == -1)
    {
        NavMoveRequestResolveWithLastItem(&g.NavMoveResultLocal);
    }
    else
    {
        g.NavTabbingDir = 1;
        g.NavTabbingCounter = offset + 1;
    }
}

// t0 = previous time (e.g.: g.time - g.io.delta_time)
// t1 = current time (e.g.: g.time)
// An event is triggered at:
//  t = 0.0     t = repeat_delay,    t = repeat_delay + repeat_rate*N
// int CalcTypematicRepeatAmount(float t0, float t1, float repeat_delay, float repeat_rate)
pub fn calc_typematic_repeat_amount(g: &mut Context, t0: f32, repeat_delay: f32, repeat_rate: f32) -> i32
{
    if (t1 == 0.0)
        return 1;
    if (t0 >= t1)
        return 0;
    if (repeat_rate <= 0.0)
        return (t0 < repeat_delay) && (t1 >= repeat_delay);
    const int count_t0 = (t0 < repeat_delay) ? -1 : ((t0 - repeat_delay) / repeat_rate);
    const int count_t1 = (t1 < repeat_delay) ? -1 : ((t1 - repeat_delay) / repeat_rate);
    const int count = count_t1 - count_t0;
    return count;
}

// void SetNextFrameWantCaptureKeyboard(bool want_capture_keyboard)
pub fn set_next_frame_want_capture_keyboard(g: &mut Context, want_capture_keyboard: bool)
{
    // ImGuiContext& g = *GImGui;
    g.want_capture_keyboard_next_frame = want_capture_keyboard ? 1 : 0;
}
