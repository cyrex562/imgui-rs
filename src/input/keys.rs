use crate::Context;
use crate::globals::GImGui;
use crate::input::KeyInputData;

// ImGuiKeyData* GetKeyData(ImGuiKey key)
pub fn get_key_data(g: &mut Context, key: Key) -> &mut KeyInputData
{
    // ImGuiContext& g = *GImGui;
    int index;
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//     IM_ASSERT(key >= ImGuiKey_LegacyNativeKey_BEGIN && key < ImGuiKey_NamedKey_END);
//     if (IsLegacyKey(key))
//         index = (g.io.key_map[key] != -1) ? g.io.key_map[key] : key; // Remap native->imgui or imgui->native
//     else
//         index = key;
// #else
    // IM_ASSERT(IsNamedKey(key) && "Support for user key indices was dropped in favor of ImGuiKey. Please update backend & user code.");
    index = key - ImGuiKey_NamedKey_BEGIN;

    return &g.io.keys_data[index];
}

// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
// int GetKeyIndex(ImGuiKey key)
pub fn get_key_index(key: &Key) -> i32
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(IsNamedKey(key));
    const ImGuiKeyData* key_data = GetKeyData(key);
    return (key_data - g.io.keys_data);
}


// Those names a provided for debugging purpose and are not meant to be saved persistently not compared.
pub const KEY_NAMES: [&'static str;133] =
[
    "Tab", "LeftArrow", "RightArrow", "UpArrow", "DownArrow", "PageUp", "PageDown",
    "Home", "End", "Insert", "Delete", "Backspace", "Space", "Enter", "Escape",
    "LeftCtrl", "LeftShift", "LeftAlt", "LeftSuper", "RightCtrl", "RightShift", "RightAlt", "RightSuper", "Menu",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H",
    "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "x", "Y", "Z",
    "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
    "Apostrophe", "Comma", "Minus", "Period", "Slash", "Semicolon", "Equal", "LeftBracket",
    "Backslash", "RightBracket", "GraveAccent", "CapsLock", "ScrollLock", "NumLock", "PrintScreen",
    "Pause", "Keypad0", "Keypad1", "Keypad2", "Keypad3", "Keypad4", "Keypad5", "Keypad6",
    "Keypad7", "Keypad8", "Keypad9", "KeypadDecimal", "KeypadDivide", "KeypadMultiply",
    "KeypadSubtract", "KeypadAdd", "KeypadEnter", "KeypadEqual",
    "GamepadStart", "GamepadBack", "GamepadFaceUp", "GamepadFaceDown", "GamepadFaceLeft", "GamepadFaceRight",
    "GamepadDpadUp", "GamepadDpadDown", "GamepadDpadLeft", "GamepadDpadRight",
    "GamepadL1", "GamepadR1", "GamepadL2", "GamepadR2", "GamepadL3", "GamepadR3",
    "GamepadLStickUp", "GamepadLStickDown", "GamepadLStickLeft", "GamepadLStickRight",
    "GamepadRStickUp", "GamepadRStickDown", "GamepadRStickLeft", "GamepadRStickRight",
    "ModCtrl", "ModShift", "ModAlt", "ModSuper"
];

// const char* GetKeyName(ImGuiKey key)
pub fn get_key_name(g: &mut Context, key: Key) -> String
{
// #ifdef IMGUI_DISABLE_OBSOLETE_KEYIO
//     IM_ASSERT((IsNamedKey(key) || key == ImGuiKey_None) && "Support for user key indices was dropped in favor of ImGuiKey. Please update backend and user code.");
// #else
    if (IsLegacyKey(key))
    {
        ImGuiIO& io = GetIO();
        if (io.key_map[key] == -1)
            return "N/A";
        // IM_ASSERT(IsNamedKey((ImGuiKey)io.key_map[key]));
        key = (ImGuiKey)io.key_map[key];
    }

    if (key == ImGuiKey_None)
        return "None";
    if (!IsNamedKey(key))
        return "Unknown";

    return GKeyNames[key - ImGuiKey_NamedKey_BEGIN];
}

// int GetKeyPressedAmount(ImGuiKey key, float repeat_delay, float repeat_rate)
pub fn get_key_pressed_amount(g: &mut Context, key: Key, repeat_delay: f32, repeat_rate: f32) -> i32
{
    // ImGuiContext& g = *GImGui;
    const ImGuiKeyData* key_data = GetKeyData(key);
    let t = key_data.down_duration;
    return CalcTypematicRepeatAmount(t - g.io.delta_time, t, repeat_delay, repeat_rate);
}

// Note that Dear ImGui doesn't know the meaning/semantic of ImGuiKey from 0..511: they are legacy native keycodes.
// Consider transitioning from 'IsKeyDown(MY_ENGINE_KEY_A)' (<1.87) to IsKeyDown(ImGuiKey_A) (>= 1.87)
// bool IsKeyDown(ImGuiKey key)
pub fn is_key_down(g: &mut Context, key: Key) -> bool
{
    const ImGuiKeyData* key_data = GetKeyData(key);
    if (!key_data.down)
        return false;
    return true;
}

// bool IsKeyPressed(ImGuiKey key, bool repeat)
pub fn is_key_pressed(g: &mut Context, key: Key, repeat: bool) -> bool
{
    // ImGuiContext& g = *GImGui;
    const ImGuiKeyData* key_data = GetKeyData(key);
    let t = key_data.down_duration;
    if (t < 0.0)
        return false;
    const bool pressed = (t == 0.0) || (repeat && t > g.io.KeyRepeatDelay && GetKeyPressedAmount(key, g.io.KeyRepeatDelay, g.io.KeyRepeatRate) > 0);
    if (!pressed)
        return false;
    return true;
}

// bool IsKeyReleased(ImGuiKey key)
pub fn is_key_release(g: &mut Context, key: Key) -> bool
{
    const ImGuiKeyData* key_data = GetKeyData(key);
    if (key_data.down_durationPrev < 0.0 || key_data.down)
        return false;
    return true;
}
