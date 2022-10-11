#![allow(non_snake_case)]

use libc::c_float;
use crate::imgui::GImGui;
use crate::input_ops::GetKeyData;
use crate::key::{ImGuiKey, ImGuiKey_Gamepad_BEGIN, ImGuiKey_Gamepad_END, ImGuiKey_Keyboard_BEGIN, ImGuiKey_Keyboard_END, ImGuiKey_KeysData_OFFSET, ImGuiKey_LegacyNativeKey_BEGIN, ImGuiKey_LegacyNativeKey_END, ImGuiKey_ModAlt, ImGuiKey_ModCtrl, ImGuiKey_ModShift, ImGuiKey_ModSuper, ImGuiKey_MouseWheelX, ImGuiKey_MouseWheelY, ImGuiKey_NamedKey_BEGIN, ImGuiKey_NamedKey_END};
use crate::key_data::ImGuiKeyData;
use crate::mod_flags::{ImGuiModFlags, ImGuiModFlags_Alt, ImGuiModFlags_Ctrl, ImGuiModFlags_None, ImGuiModFlags_Shift, ImGuiModFlags_Super};
use crate::mouse_button::ImGuiMouseButton_COUNT;

// static c_void UpdateKeyboardInputs()
pub unsafe fn UpdateKeyboardInputs()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let io = &mut g.IO;

    // Import legacy keys or verify they are not used
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    if io.BackendUsingLegacyKeyArrays == 0
    {
        // Backend used new io.AddKeyEvent() API: Good! Verify that old arrays are never written to externally.
        // for (let n: c_int = 0; n < ImGuiKey_LegacyNativeKey_END; n++)
        for n in 0 .. ImGuiKey_LegacyNativeKey_END
        {}
            // IM_ASSERT((io.KeysDown[n] == false || IsKeyDown(n)) && "Backend needs to either only use io.AddKeyEvent(), either only fill legacy io.KeysDown[] + io.KeyMap[]. Not both!");
    }
    else
    {
        if g.FrameCount == 0 {
            // for (let n: c_int = ImGuiKey_LegacyNativeKey_BEGIN; n < ImGuiKey_LegacyNativeKey_END; n+ +)
            for n in ImGuiKey_LegacyNativeKey_BEGIN .. ImGuiKey_LegacyNativeKey_END
            {
                // IM_ASSERT(g.IO.KeyMap[n] == -1 && "Backend is not allowed to write to io.KeyMap[0..511]!");
                }}

        // Build reverse KeyMap (Named -> Legacy)
        // for (let n: c_int = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n++)
        for n in ImGuiKey_NamedKey_BEGIN .. ImGuiKey_NamedKey_END
        {
            if io.KeyMap[n] != -1 {
                // IM_ASSERT(IsLegacyKey((ImGuiKey)io.KeyMap[n]));
                io.KeyMap[io.KeyMap[n]] = n;
            }
        }

        // Import legacy keys into new ones
        // for (let n: c_int = ImGuiKey_LegacyNativeKey_BEGIN; n < ImGuiKey_LegacyNativeKey_END; n++)
        for n in ImGuiKey_LegacyNativeKey_BEGIN .. ImGuiKeyLegacyNativeKey_END
        {
            if io.KeysDown[n] || io.BackendUsingLegacyKeyArrays == 1 {
                let mut key: ImGuiKey = (if io.KeyMap[n] != -1 { io.KeyMap[n] }else { n });
                // IM_ASSERT(io.KeyMap[n] == -1 || IsNamedKey(key));
                io.KeysData[key].Down = io.KeysDown[n];
                if key != n {
                    io.KeysDown[key] = io.KeysDown[n];
                } // Allow legacy code using io.KeysDown[GetKeyIndex()] with old backends
                io.BackendUsingLegacyKeyArrays = 1;
            }
        }
        if io.BackendUsingLegacyKeyArrays == 1
        {
            io.KeysData[ImGuiKey_ModCtrl].Down = io.KeyCtrl;
            io.KeysData[ImGuiKey_ModShift].Down = io.KeyShift;
            io.KeysData[ImGuiKey_ModAlt].Down = io.KeyAlt;
            io.KeysData[ImGuiKey_ModSuper].Down = io.KeySuper;
        }
    }

// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//     let nav_gamepad_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (io.BackendFlags & ImGuiBackendFlags_HasGamepad) != 0;
//     if io.BackendUsingLegacyNavInputArray && nav_gamepad_active
//     {
//         #define MAP_LEGACY_NAV_INPUT_TO_KEY1(_KEY, _NAV1)           do { io.KeysData[_KEY].Down = (io.NavInputs[_NAV1] > 0f32); io.KeysData[_KEY].AnalogValue = io.NavInputs[_NAV1]; } while (0)
//         #define MAP_LEGACY_NAV_INPUT_TO_KEY2(_KEY, _NAV1, _NAV2)    do { io.KeysData[_KEY].Down = (io.NavInputs[_NAV1] > 0f32) || (io.NavInputs[_NAV2] > 0f32); io.KeysData[_KEY].AnalogValue = ImMax(io.NavInputs[_NAV1], io.NavInputs[_NAV2]); } while (0)
//         MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadFaceDown, ImGuiNavInput_Activate);
//         MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadFaceRight, ImGuiNavInput_Cancel);
//         MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadFaceLeft, ImGuiNavInput_Menu);
//         MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadFaceUp, ImGuiNavInput_Input);
//         MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadDpadLeft, ImGuiNavInput_DpadLeft);
//         MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadDpadRight, ImGuiNavInput_DpadRight);
//         MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadDpadUp, ImGuiNavInput_DpadUp);
//         MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadDpadDown, ImGuiNavInput_DpadDown);
//         MAP_LEGACY_NAV_INPUT_TO_KEY2(ImGuiKey_GamepadL1, ImGuiNavInput_FocusPrev, ImGuiNavInput_TweakSlow);
//         MAP_LEGACY_NAV_INPUT_TO_KEY2(ImGuiKey_GamepadR1, ImGuiNavInput_FocusNext, ImGuiNavInput_TweakFast);
//         MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadLStickLeft, ImGuiNavInput_LStickLeft);
//         MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadLStickRight, ImGuiNavInput_LStickRight);
//         MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadLStickUp, ImGuiNavInput_LStickUp);
//         MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadLStickDown, ImGuiNavInput_LStickDown);
//         #undef NAV_MAP_KEY
//     }
// #endif

// #endif

    // Synchronize io.KeyMods with individual modifiers io.KeyXXX bools, update aliases
    io.KeyMods = GetMergedModFlags();
    // for (let n: c_int = 0; n < ImGuiMouseButton_COUNT; n++)
    for n in 0 .. ImGuiMouseButton_COUNT
    {
        UpdateAliasKey(MouseButtonToKey(n), io.MouseDown[n], if io.MouseDown[n] { 1f32 }else { 0f32 });
    }
    UpdateAliasKey(ImGuiKey_MouseWheelX, io.MouseWheelH != 0f32, io.MouseWheelH);
    UpdateAliasKey(ImGuiKey_MouseWheelY, io.MouseWheel != 0f32, io.MouseWheel);

    // Clear gamepad data if disabled
    if (io.BackendFlags & ImGuiBackendFlags_HasGamepad) == 0 {
        // for (let i: c_int = ImGuiKey_Gamepad_BEGIN; i < ImGuiKey_Gamepad_END; i+ +)
        for i in ImGuiKey_Gamepad_BEGIN .. ImGuiKey_Gamepad_END
        {
            io.KeysData[i - ImGuiKey_KeysData_OFFSET].Down = false;
            io.KeysData[i - ImGuiKey_KeysData_OFFSET].AnalogValue = 0f32;
        }
    }

    // Update keys
    // for (let i: c_int = 0; i < IM_ARRAYSIZE(io.KeysData); i++)
    for i in 0..io.KeysData.len() {
        let mut key_data: *mut ImGuiKeyData = &mut io.KeysData[i];
        key_data.DownDurationPrev = key_data.DownDuration;
        key_data.DownDuration = if key_data.Down {
            if key_data.DownDuration < 0f32 {
                0f32
            } else { key_data.DownDuration + io.DeltaTime }
        } else { -1f32 };
    }
}



// static c_void UpdateAliasKey(ImGuiKey key, v: bool, c_float analog_value)
pub fn UpdateAliasKey(key: ImGuiKey, v: bool, analog_value: c_float) {
    // IM_ASSERT(IsAliasKey(key));
    let mut key_data: *mut ImGuiKeyData = GetKeyData(key);
    key_data.Down = v;
    key_data.AnalogValue = analog_value;
}

// [Internal] Do not use directly (can read io.KeyMods instead)
// ImGuiModFlags GetMergedModFlags()
pub unsafe fn GetMergedModFlags() -> ImGuiModFlags
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut key_mods = ImGuiModFlags_None;
    if g.IO.KeyCtrl { key_mods |= ImGuiModFlags_Ctrl; }
    if g.IO.KeyShift { key_mods |= ImGuiModFlags_Shift; }
    if g.IO.KeyAlt { key_mods |= ImGuiModFlags_Alt; }
    if g.IO.KeySuper { key_mods |= ImGuiModFlags_Super; }
    return key_mods;
}


// FIXME: Technically this also prevents use of Gamepad D-Pad, may not be an issue.
// c_void SetActiveIdUsingAllKeyboardKeys()
pub unsafe fn SetActiveIdUsingALlKeyboardKeys()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.ActiveId != 0);
    g.ActiveIdUsingNavDirMask = ~0;
    g.ActiveIdUsingKeyInputMask.SetBitRange(ImGuiKey_Keyboard_BEGIN, ImGuiKey_Keyboard_END);
    g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_ModCtrl);
    g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_ModShift);
    g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_ModAlt);
    g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_ModSuper);
    NavMoveRequestCancel();
}
