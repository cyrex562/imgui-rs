use std::collections::HashSet;
use crate::context::Context;
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
    PushItemFlag(ImGuiItemFlags_NoTabStop, !allow_keyboard_focus);
}

// void ImGui::PopAllowKeyboardFocus()
pub fn pop_allow_keyboard_focus(g: &mut Context)
{
    PopItemFlag();
}