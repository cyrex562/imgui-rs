#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::ffi::CString;
use libc::{c_char, c_float, c_int, c_uint};
use crate::debug_log_flags::ImGuiDebugLogFlags_EventIO;
use crate::imgui::GImGui;
use crate::input_event::ImGuiInputEvent;
use crate::input_event_type::{ImGuiInputEventType_Focus, ImGuiInputEventType_Key, ImGuiInputEventType_MouseButton, ImGuiInputEventType_MousePos, ImGuiInputEventType_MouseViewport, ImGuiInputEventType_MouseWheel, ImGuiInputEventType_Text};
use crate::input_flags::{ImGuiInputFlags, ImGuiInputFlags_None, ImGuiInputFlags_Repeat, ImGuiInputFlags_RepeatRateDefault, ImGuiInputFlags_RepeatRateMask_, ImGuiInputFlags_RepeatRateNavMove, ImGuiInputFlags_RepeatRateNavTweak};
use crate::input_source::{ImGuiInputSource, input_source_names};
use crate::io::ImGuiIO;
use crate::key::{ImGuiKey, ImGuiKey_Aliases_BEGIN, ImGuiKey_Aliases_END, ImGuiKey_Gamepad_BEGIN, ImGuiKey_Gamepad_END, ImGuiKey_KeysData_OFFSET, ImGuiKey_LegacyNativeKey_BEGIN, ImGuiKey_LegacyNativeKey_END, ImGuiKey_ModAlt, ImGuiKey_ModCtrl, ImGuiKey_ModShift, ImGuiKey_ModSuper, ImGuiKey_NamedKey_BEGIN, ImGuiKey_NamedKey_END, ImGuiKey_None};
use crate::key_data::ImGuiKeyData;
use crate::mod_flags::ImGuiModFlags;
use crate::mouse_button::ImGuiMouseButton;
use crate::mouse_cursor::ImGuiMouseCursor;
use crate::rect::ImRect;
use crate::string_ops::{ImFormatString, str_to_const_c_char_ptr};
use crate::vec2::ImVec2;


// Test if mouse cursor is hovering given rectangle
// NB- Rectangle is clipped by our current clip setting
// NB- Expand the rectangle to be generous on imprecise inputs systems (g.Style.TouchExtraPadding)
// bool IsMouseHoveringRect(const ImVec2& r_min, const ImVec2& r_max, bool clip)
pub fn IsMouseHoveringRect(r_min: &ImVec2, r_max: &ImVec2, clip: bool) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Clip
    let mut rect_clipped: ImRect = ImRect::new2(r_min, r_max);
    if clip {
        rect_clipped.ClipWith(g.Currentwindow.ClipRect);
    }

    // Expand for touch input const
    let mut rect_for_touch: ImRect = ImRect::new2(rect_clipped.Min - g.Style.TouchExtraPadding.clone(), rect_clipped.Max + g.Style.TouchExtraPadding.clone());
    if !rect_for_touch.Contains(&g.IO.MousePos) {
        return false;
    }
    if !g.MouseViewport.GetMainRect().Overlaps(&rect_clipped) {
        return false;
    }
    return true;
}

// inline bool             IsNamedKey(ImGuiKey key)
pub fn IsNamedKey(key: ImGuiKey) -> bool {
    return key >= ImGuiKey_NamedKey_BEGIN && key < ImGuiKey_NamedKey_END;
}

// inline bool             IsLegacyKey(ImGuiKey key)
pub fn IsLegacyKey(key: ImGuiKey) -> bool {
    return key >= ImGuiKey_LegacyNativeKey_BEGIN && key < ImGuiKey_LegacyNativeKey_END;
}

// inline bool             IsGamepadKey(ImGuiKey key)
pub fn IsGamepadKey(key: ImGuiKey) -> bool {
    return key >= ImGuiKey_Gamepad_BEGIN && key < ImGuiKey_Gamepad_END;
}

// inline bool             IsAliasKey(ImGuiKey key)
pub fn IsAliasKey(key: ImGuiKey) -> bool {
    return key >= ImGuiKey_Aliases_BEGIN && key < ImGuiKey_Aliases_END;
}


// ImGuiKeyData* GetKeyData(ImGuiKey key)
pub fn GetKeyData(key: ImGuiKey) -> *mut ImGuiKeyData {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut index: c_int = 0;
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//     // IM_ASSERT(key >= ImGuiKey_LegacyNativeKey_BEGIN && key < ImGuiKey_NamedKey_END);
//     if IsLegacyKey(key) {
//         index = if g.IO.KeyMap[key] != -1 {
//             g.IO.KeyMap[key]
//         } else { key };
//     } // Remap native->imgui or imgui->native
//     else {
//         index = key;
//     }
// #else
    // IM_ASSERT(IsNamedKey(key) && "Support for user key indices was dropped in favor of ImGuiKey. Please update backend & user code.");
    index = key - ImGuiKey_NamedKey_BEGIN;
// #endif
    return &mut g.IO.KeysData[index];
}

// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
// c_int GetKeyIndex(ImGuiKey key)
pub fn GetKeyIndex(key: ImGuiKey) -> c_int {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(IsNamedKey(key));
    let key_data = GetKeyData(key);
    return key_data - g.IO.KeysData;
}
// #endif

// Those names a provided for debugging purpose and are not meant to be saved persistently not compared.
pub const GKeyNames: [&'static str; 140] = [
    "Tab", "LeftArrow", "RightArrow", "UpArrow", "DownArrow", "PageUp", "PageDown",
    "Home", "End", "Insert", "Delete", "Backspace", "Space", "Enter", "Escape",
    "LeftCtrl", "LeftShift", "LeftAlt", "LeftSuper", "RightCtrl", "RightShift", "RightAlt", "RightSuper", "Menu",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H",
    "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
    "Apostrophe", "Comma", "Minus", "Period", "Slash", "Semicolon", "Equal", "LeftBracket",
    "Backslash", "RightBracket", "GraveAccent", "CapsLock", "ScrollLock", "NumLock", "PrintScreen",
    "Pause", "Keypad0", "Keypad1", "Keypad2", "Keypad3", "Keypad4", "Keypad5", "Keypad6",
    "Keypad7", "Keypad8", "Keypad9", "KeypadDecimal", "KeypadDivide", "KeypadMultiply",
    "KeypadSubtract", "KeypadAdd", "KeypadEnter", "KeypadEqual",
    "GamepadStart", "GamepadBack",
    "GamepadFaceLeft", "GamepadFaceRight", "GamepadFaceUp", "GamepadFaceDown",
    "GamepadDpadLeft", "GamepadDpadRight", "GamepadDpadUp", "GamepadDpadDown",
    "GamepadL1", "GamepadR1", "GamepadL2", "GamepadR2", "GamepadL3", "GamepadR3",
    "GamepadLStickLeft", "GamepadLStickRight", "GamepadLStickUp", "GamepadLStickDown",
    "GamepadRStickLeft", "GamepadRStickRight", "GamepadRStickUp", "GamepadRStickDown",
    "ModCtrl", "ModShift", "ModAlt", "ModSuper",
    "MouseLeft", "MouseRight", "MouseMiddle", "MouseX1", "MouseX2", "MouseWheelX", "MouseWheelY",
];
// IM_STATIC_ASSERT(ImGuiKey_NamedKey_COUNT == IM_ARRAYSIZE(GKeyNames));

// *const char GetKeyName(ImGuiKey key)
pub unsafe fn GetKeyName(mut key: ImGuiKey) -> *const c_char {
// #ifdef IMGUI_DISABLE_OBSOLETE_KEYIO
    // IM_ASSERT((IsNamedKey(key) || key == ImGuiKey_None) && "Support for user key indices was dropped in favor of ImGuiKey. Please update backend and user code.");
// #else
    if IsLegacyKey(key) {
        let mut io: *mut ImGuiIO = GetIO();
        if io.KeyMap[key.clone()] == -1 {
            return str_to_const_c_char_ptr("NA");
        }
        // IM_ASSERT(IsNamedKey((ImGuiKey)io.KeyMap[key]));
        key = io.KeyMap[key.clone()];
    }
// #endif
    if key == ImGuiKey_None {
        // return "None";
        return str_to_const_c_char_ptr("None");
    }
    if !IsNamedKey(key.clone()) {
        // return "Unknown";
        return str_to_const_c_char_ptr("Unknown");
    }

    return GKeyNames[key.clone() - ImGuiKey_NamedKey_BEGIN];
}

// c_void GetKeyChordName(ImGuiModFlags mods, ImGuiKey key, char* out_buf, c_int out_buf_size)
pub fn GetKeyChordName(mods: ImGuiModFlags, key: ImGuiKey, out_buf: *mut c_char, out_buf_size: c_int) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT((mods & ~ImGuiModFlags_All) == 0 && "Passing invalid ImGuiModFlags value!"); // A frequent mistake is to pass ImGuiKey_ModXXX instead of ImGuiModFlags_XXX
    // ImFormatString(out_buf, out_buf_size, "%s%s%s%s%s",
    //     if (mods & ImGuiModFlags_Ctrl) { "Ctrl+" } else { "" },
    //     (mods & ImGuiModFlags_Shift) ? "Shift+" : "",
    //     (mods & ImGuiModFlags_Alt) ? "Alt+" : "",
    //     (mods & ImGuiModFlags_Super) ? (g.IO.ConfigMacOSXBehaviors ? "Cmd+" : "Super+") : "",
    //     GetKeyName(key));
    todo!()
}

// t0 = previous time (e.g.: g.Time - g.IO.DeltaTime)
// t1 = current time (e.g.: g.Time)
// An event is triggered at:
//  t = 0f32     t = repeat_delay,    t = repeat_delay + repeat_rate*N
// c_int CalcTypematicRepeatAmount(c_float t0, c_float t1, c_float repeat_delay, c_float repeat_rate)
pub fn CalcTypematicRepeatAmount(t0: c_float, t1: c_float, repeat_delay: c_float, repeat_rate: c_float) -> c_int {
    if t1 == 0f32 {
        return 1;
    }
    if t0 >= t1 {
        return 0;
    }
    if repeat_rate <= 0f32 {
        return if (t0 < repeat_delay) && (t1 >= repeat_delay) {
            1
        } else {
            0
        };
    }
    let count_t0: c_int = if t0 < repeat_delay { -1 } else { ((t0 - repeat_delay) / repeat_rate) };
    let count_t1: c_int = if t1 < repeat_delay { -1 } else { ((t1 - repeat_delay) / repeat_rate) };
    let count: c_int = count_t1 - count_t0;
    return count;
}

// c_void GetTypematicRepeatRate(ImGuiInputFlags flags, c_float* repeat_delay, c_float* repeat_rate)
pub unsafe fn GetTypematicRepeatRate(flags: ImGuiInputFlags, repeat_delay: *mut c_float, repeat_rate: *mut c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (flags & ImGuiInputFlags_RepeatRateMask_) == ImGuiInputFlags_RepeatRateNavMove {
        *repeat_delay = g.IO.KeyRepeatDelay * 0.72f32;
        *repeat_rate = g.IO.KeyRepeatRate * 0.80f32;
        return;
    }
    if (flags & ImGuiInputFlags_RepeatRateMask_) == ImGuiInputFlags_RepeatRateNavMove {
        *repeat_delay = g.IO.KeyRepeatDelay * 0.72f32;
        *repeat_rate = g.IO.KeyRepeatRate * 0.3f32;
        return;
    }
    if (flags & ImGuiInputFlags_RepeatRateMask_) == ImGuiInputFlags_RepeatRateNavTweak || (flags & ImGuiInputFlags_RepeatRateMask_) == ImGuiInputFlags_RepeatRateDefault {
        *repeat_delay = g.IO.KeyRepeatDelay * 1f32;
        *repeat_rate = g.IO.KeyRepeatRate * 1f32;
        return;
    }
    *repeat_delay = g.IO.KeyRepeatDelay * 1f32;
    *repeat_rate = g.IO.KeyRepeatRate * 1f32;
    return;
}

// Return value representing the number of presses in the last time period, for the given repeat rate
// (most often returns 0 or 1. The result is generally only >1 when RepeatRate is smaller than DeltaTime, aka large DeltaTime or fast RepeatRate)
// c_int GetKeyPressedAmount(ImGuiKey key, c_float repeat_delay, c_float repeat_rate)
pub fn GetKeyPressedAmount(key: ImGuiKey, repeat_delay: c_float, repeat_rate: c_float) -> c_int {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let key_data = GetKeyData(key);
    if !key_data.Down { // In theory this should already be encoded as (DownDuration < 0f32), but testing this facilitate eating mechanism (until we finish work on input ownership)
        return 0;
    }
    let t: c_float = key_data.DownDuration;
    return CalcTypematicRepeatAmount(t - g.IO.DeltaTime, t, repeat_delay, repeat_rate);
}

// Return 2D vector representing the combination of four cardinal direction, with analog value support (for e.g. ImGuiKey_GamepadLStick* values).
// ImVec2 GetKeyVector2d(ImGuiKey key_left, ImGuiKey key_right, ImGuiKey key_up, ImGuiKey key_down)
pub fn GetKeyVector2D(key_left: ImGuiKey, key_right: ImGuiKey, key_up: ImGuiKey, key_down: ImGuiKey) -> ImVec2 {
    return ImVec2::new2(
        GetKeyData(key_right).AnalogValue - GetKeyData(key_left).AnalogValue,
        GetKeyData(key_down).AnalogValue - GetKeyData(key_up).AnalogValue);
}

// Note that Dear ImGui doesn't know the meaning/semantic of ImGuiKey from 0..511: they are legacy native keycodes.
// Consider transitioning from 'IsKeyDown(MY_ENGINE_KEY_A)' (<1.87) to IsKeyDown(ImGuiKey_A) (>= 1.87)
// bool IsKeyDown(ImGuiKey key)
pub fn IsKeyDown(key: ImGuiKey) -> bool {
    let key_data = GetKeyData(key);
    if !key_data.Down {
        return false;
    }
    return true;
}

// bool IsKeyPressed(ImGuiKey key, bool repeat)
pub unsafe fn IsKeyPressed(key: ImGuiKey, repeat: bool) -> bool {
    return IsKeyPressedEx(key, if repeat { ImGuiInputFlags_Repeat } else { ImGuiInputFlags_None });
}

// Important: unlike legacy IsKeyPressed(ImGuiKey, bool repeat=true) which DEFAULT to repeat, this requires EXPLICIT repeat.
// [Internal] 2022/07: Do not call this directly! It is a temporary entry point which we will soon replace with an overload for IsKeyPressed() when we introduce key ownership.
// bool IsKeyPressedEx(ImGuiKey key, ImGuiInputFlags flags)
pub unsafe fn IsKeyPressedEx(key: ImGuiKey, flags: ImGuiInputFlags) -> bool {
    let key_data = GetKeyData(key);
    if !key_data.Down { // In theory this should already be encoded as (DownDuration < 0f32), but testing this facilitate eating mechanism (until we finish work on input ownership)
        return false;
    }
    let t: c_float = key_data.DownDuration;
    if t < 0f32 {
        return false;
    }

    let mut pressed: bool = (t == 0f32);
    if !pressed && ((flags & ImGuiInputFlags_Repeat) != 0) {
        // c_float repeat_delay, repeat_rate;
        let mut repeat_delay: c_float = 0f32;
        let mut repeat_rate: c_float = 0f32;
        GetTypematicRepeatRate(flags, &mut repeat_delay, &mut repeat_rate);
        pressed = (t > repeat_delay) && GetKeyPressedAmount(key, repeat_delay, repeat_rate) > 0;
    }

    if !pressed {
        return false;
    }
    return true;
}

// bool IsKeyReleased(ImGuiKey key)
pub fn IsKeyRelease(key: ImGuiKey) -> bool {
    let key_data = GetKeyData(key);
    if key_data.DownDurationPrev < 0f32 || key_data.Down {
        return false;
    }
    return true;
}

// bool IsMouseDown(ImGuiMouseButton button)
pub fn IsMouseDown(button: ImGuiMouseButton) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    return g.IO.MouseDown[button];
}

// bool IsMouseClicked(ImGuiMouseButton button, bool repeat)
pub fn IsMouseClicked(button: ImGuiMouseButton, repeat: bool) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    if !g.IO.MouseDown[button] {// In theory this should already be encoded as (DownDuration < 0f32), but testing this facilitate eating mechanism (until we finish work on input ownership)
        return false;
    }
    let t: c_float = g.IO.MouseDownDuration[button];
    if t == 0f32 {
        return true;
    }
    if repeat && t > g.IO.KeyRepeatDelay {
        return CalcTypematicRepeatAmount(t - g.IO.DeltaTime, t, g.IO.KeyRepeatDelay, g.IO.KeyRepeatRate) > 0;
    }
    return false;
}

// bool IsMouseReleased(ImGuiMouseButton button)
pub fn IsMouseRelease(button: ImGuiMouseButton) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    return g.IO.MouseReleased[button];
}

// bool IsMouseDoubleClicked(ImGuiMouseButton button)
pub fn IsMouseDoubleClicked(button: ImGuiMouseButton) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    return g.IO.MouseClickedCount[button] == 2;
}

// c_int GetMouseClickedCount(ImGuiMouseButton button)
pub fn GetMouseClickedCount(button: ImGuiMouseButton) -> c_int {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    return g.IO.MouseClickedCount[button];
}

// Return if a mouse click/drag went past the given threshold. Valid to call during the MouseReleased frame.
// [Internal] This doesn't test if the button is pressed
// bool IsMouseDragPastThreshold(ImGuiMouseButton button, c_float lock_threshold)
pub fn IsMouseDragPastThreshold(button: ImGuiMouseButton, mut lock_threshold: c_float) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    if lock_threshold < 0f32 {
        lock_threshold = g.IO.MouseDragThreshold;
    }
    return g.IO.MouseDragMaxDistanceSqr[button] >= lock_threshold * lock_threshold;
}

// bool IsMouseDragging(ImGuiMouseButton button, c_float lock_threshold)
pub fn IsMouseDragging(button: ImGuiMouseButton, lock_threshold: c_float) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    if !g.IO.MouseDown[button] {
        return false;
    }
    return IsMouseDragPastThreshold(button, lock_threshold);
}

// ImVec2 GetMousePos()
pub fn GetMousePos() -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.IO.MousePos.clone();
}

// NB: prefer to call right after BeginPopup(). At the time Selectable/MenuItem is activated, the popup is already closed!
// ImVec2 GetMousePosOnOpeningCurrentPopup()
pub fn GetMousePosOnOPeningCurrentPopup() -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.BeginPopupStack.Size > 0 {
        return g.OpenPopupStack[g.BeginPopupStack.Size - 1].OpenMousePos;
    }
    return g.IO.MousePos.clone();
}

// We typically use ImVec2(-f32::MAX,-f32::MAX) to denote an invalid mouse position.
// bool IsMousePosValid(*const ImVec2 mouse_pos)
pub fn IsMousePosValid(mouse_pos: *const ImVec2) -> bool {
    // The assert is only to silence a false-positive in XCode Static Analysis.
    // Because GImGui is not dereferenced in every code path, the static analyzer assume that it may be NULL (which it doesn't for other functions).
    // IM_ASSERT(GImGui != NULL);
    let MOUSE_INVALID: c_float = -256000f32;
    let p: ImVec2 = mouse_pos? * mouse_pos: GimGui.IO.MousePos;
    return p.x >= MOUSE_INVALID && p.y >= MOUSE_INVALID;
}

// [WILL OBSOLETE] This was designed for backends, but prefer having backend maintain a mask of held mouse buttons, because upcoming input queue system will make this invalid.
// bool IsAnyMouseDown()
pub fn IsAnyMouseDown() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // for (let n: c_int = 0; n < IM_ARRAYSIZE(g.IO.MouseDown); n++)
    for n in 0..g.IO.MouseDown.len() {
        if g.IO.MouseDown[n] {
            return true;
        }
    }
    return false;
}

// Return the delta from the initial clicking position while the mouse button is clicked or was just released.
// This is locked and return 0f32 until the mouse moves past a distance threshold at least once.
// NB: This is only valid if IsMousePosValid(). backends in theory should always keep mouse position valid when dragging even outside the client window.
// ImVec2 GetMouseDragDelta(ImGuiMouseButton button, c_float lock_threshold)
pub fn GetMouseDragDelta(button: ImGuiMouseButton, mut lock_threshold: c_float) -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    if lock_threshold < 0f32 {
        lock_threshold = g.IO.MouseDragThreshold;
    }
    if g.IO.MouseDown[button] || g.IO.MouseReleased[button] {
        if g.IO.MouseDragMaxDistanceSqr[button] >= lock_threshold * lock_threshold {
            if IsMousePosValid(&g.IO.MousePos) && IsMousePosValid(&g.IO.MouseClickedPos[button]) {
                return g.IO.MousePos.clone() - g.IO.MouseClickedPos[button].clone();
            }
        }
    }
    return ImVec2::new2(0f32, 0f32);
}

// c_void ResetMouseDragDelta(ImGuiMouseButton button)
pub fn ResetMouseDragDelta(button: ImGuiMouseButton) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    // NB: We don't need to reset g.IO.MouseDragMaxDistanceSqr
    g.IO.MouseClickedPos[button] = g.IO.MousePos.clone();
}

// ImGuiMouseCursor GetMouseCursor()
pub fn GetMouseCursor() -> ImGuiMouseCursor {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.MouseCursor;
}

// c_void SetMouseCursor(ImGuiMouseCursor cursor_type)
pub fn SetMouseCursor(cursor_type: ImGuiMouseCursor) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.MouseCursor = cursor_type;
}

// c_void SetNextFrameWantCaptureKeyboard(bool want_capture_keyboard)
pub fn SetNextFrameWantCaptureKeyboard(want_capture_keyboard: bool) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.WantCaptureKeyboardNextFrame = if want_capture_keyboard { 1 } else { 0 };
}

// c_void SetNextFrameWantCaptureMouse(bool want_capture_mouse)
pub fn SetNextFrameWantCaptureMouse(want_capture_mouse: bool) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.WantCaptureMouseNextFrame = if want_capture_mouse { 1 } else { 0 };
}

// #ifndef IMGUI_DISABLE_DEBUG_TOOLS
// static *const char GetInputSourceName(ImGuiInputSource source)
pub fn GetInputSourceName(source: ImGuiInputSource) -> *const c_char {

    // IM_ASSERT(IM_ARRAYSIZE(input_source_names) == ImGuiInputSource_COUNT && source >= 0 && source < ImGuiInputSource_COUNT);
    return input_source_names[source];
}

// static c_void DebugPrintInputEvent(*const char prefix, *const ImGuiInputEvent e)
pub fn DebugPrintInputEvent(prefix: *const c_char, e: *const ImGuiInputEvent) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if e.Type == ImGuiInputEventType_MousePos {
        IMGUI_DEBUG_LOG_IO("%s: MousePos (%.1f, %.10f32)\n", prefix, e.MousePos.PosX, e.MousePos.PosY);
        return;
    }
    if e.Type == ImGuiInputEventType_MouseButton {
        // IMGUI_DEBUG_LOG_IO("%s: MouseButton %d %s\n", prefix, e.MouseButton.Button, e.MouseButton.Down ? "Down": "Up"); return; }
        if e.Type == ImGuiInputEventType_MouseWheel {
            IMGUI_DEBUG_LOG_IO("%s: MouseWheel (%.1f, %.10f32)\n", prefix, e.MouseWheel.WheelX, e.MouseWheel.WheelY);
            return;
        }
        if e.Type == ImGuiInputEventType_Key {
            // IMGUI_DEBUG_LOG_IO("%s: Key \"%s\" %s\n", prefix, GetKeyName(e.Key.Key), e.Key.Down ? "Down" : "Up"); return; }
            if e.Type == ImGuiInputEventType_Text {
                IMGUI_DEBUG_LOG_IO("%s: Text: %c (U+%08X)\n", prefix, e.Text.Char, e.Text.Char);
                return;
            }
            if e.Type == ImGuiInputEventType_Focus {
                IMGUI_DEBUG_LOG_IO("%s: AppFocused %d\n", prefix, e.AppFocused.Focused);
                return;
            }
        }
    }
}
// #endif

// Process input queue
// We always call this with the value of 'bool g.IO.ConfigInputTrickleEventQueue'.
// - trickle_fast_inputs = false : process all events, turn into flattened input state (e.g. successive down/up/down/up will be lost)
// - trickle_fast_inputs = true  : process as many events as possible (successive down/up/down/up will be trickled over several frames so nothing is lost) (new feature in 1.87)
// c_void UpdateInputEvents(bool trickle_fast_inputs)
pub fn UpdateInputEvents(trickle_fast_inputs: bool) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let io = &mut g.IO;

    // Only trickle chars<>key when working with InputText()
    // FIXME: InputText() could parse event trail?
    // FIXME: Could specialize chars<>keys trickling rules for control keys (those not typically associated to characters)
    let trickle_interleaved_keys_and_text: bool = (trickle_fast_inputs && g.WantTextInputNextFrame == 1);

    let mut mouse_moved: bool = false;
    let mut mouse_wheeled = false;
    let mut key_changed = false;
    let mut text_inputted = false;
    let mut mouse_button_changed: c_int = 0x00;
    let mut key_changed_mask: ImBitArray<ImGuiKey_KeysData_SIZE> = ImBitArray::new();

    // let event_n: c_int = 0;
    // for (; event_n < g.InputEventsQueue.Size; event_n++)
    for event_n in 0..g.InputEventsQueue.len() {
        let mut e: *mut ImGuiInputEvent = &mut g.InputEventsQueue[event_n];
        if e.Type == ImGuiInputEventType_MousePos {
            let mut event_pos = ImVec2::new2(e.MousePos.PosX, e.MousePos.PosY);
            if IsMousePosValid(&event_pos) {
                event_pos = ImVec2(ImFloorSigned(event_pos.x), ImFloorSigned(event_pos.y));
            } // Apply same flooring as UpdateMouseInputs()
            e.IgnoredAsSame = (io.MousePos.x == event_pos.x && io.MousePos.y == event_pos.y);
            if !e.IgnoredAsSame {
                // Trickling Rule: Stop processing queued events if we already handled a mouse button change
                if trickle_fast_inputs && (mouse_button_changed != 0 || mouse_wheeled || key_changed || text_inputted) {
                    break;
                }
                io.MousePos = event_pos;
                mouse_moved = true;
            }
        } else if e.Type == ImGuiInputEventType_MouseButton {
            let mut button: ImGuiMouseButton = e.MouseButton.Button;
            // IM_ASSERT(button >= 0 && button < ImGuiMouseButton_COUNT);
            e.IgnoredAsSame = (io.MouseDown[button] == e.MouseButton.Down);
            if !e.IgnoredAsSame {
                // Trickling Rule: Stop processing queued events if we got multiple action on the same button
                if trickle_fast_inputs && ((mouse_button_changed & (1 << button)) != 0 || mouse_wheeled) {
                    break;
                }
                io.MouseDown[button] = e.MouseButton.Down;
                mouse_button_changed |= (1 << button);
            }
        } else if e.Type == ImGuiInputEventType_MouseWheel {
            e.IgnoredAsSame = (e.MouseWheel.WheelX == 0f32 && e.MouseWheel.WheelY == 0f32);
            if !e.IgnoredAsSame {
                // Trickling Rule: Stop processing queued events if we got multiple action on the event
                if trickle_fast_inputs && (mouse_moved || mouse_button_changed != 0) {
                    break;
                }
                io.MouseWheelH += e.MouseWheel.WheelX;
                io.MouseWheel += e.MouseWheel.WheelY;
                mouse_wheeled = true;
            }
        } else if e.Type == ImGuiInputEventType_MouseViewport {
            io.MouseHoveredViewport = e.MouseViewport.HoveredViewportID;
        } else if e.Type == ImGuiInputEventType_Key {
            let mut key: ImGuiKey = e.Key.Key;
            // IM_ASSERT(key != ImGuiKey_None);
            let keydata_index: c_int = (key - ImGuiKey_KeysData_OFFSET);
            let mut keydata: *mut ImGuiKeyData = &mut io.KeysData[keydata_index];
            e.IgnoredAsSame = (keydata.Down == e.Key.Down && keydata.AnalogValue == e.Key.AnalogValue);
            if !e.IgnoredAsSame {
                // Trickling Rule: Stop processing queued events if we got multiple action on the same button
                if trickle_fast_inputs && keydata.Down != e.Key.Down && (key_changed_mask.TestBit(keydata_index) || text_inputted || mouse_button_changed != 0) {
                    break;
                }
                keydata.Down = e.Key.Down;
                keydata.AnalogValue = e.Key.AnalogValue;
                key_changed = true;
                key_changed_mask.SetBit(keydata_index);

                if key == ImGuiKey_ModCtrl || key == ImGuiKey_ModShift || key == ImGuiKey_ModAlt || key == ImGuiKey_ModSuper {
                    if key == ImGuiKey_ModCtrl { io.KeyCtrl = keydata.Down; }
                    if key == ImGuiKey_ModShift { io.KeyShift = keydata.Down; }
                    if key == ImGuiKey_ModAlt { io.KeyAlt = keydata.Down; }
                    if key == ImGuiKey_ModSuper { io.KeySuper = keydata.Down; }
                    io.KeyMods = GetMergedModFlags();
                }

                // Allow legacy code using io.KeysDown[GetKeyIndex()] with new backends
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//                 io.KeysDown[key] = keydata.Down;
//                 if io.KeyMap[key] != -1 {
//                     io.KeysDown[io.KeyMap[key]] = keydata.Down;
//                 }
// #endif
            }
        } else if e.Type == ImGuiInputEventType_Text {
            // Trickling Rule: Stop processing queued events if keys/mouse have been interacted with
            if trickle_fast_inputs && ((key_changed && trickle_interleaved_keys_and_text) || mouse_button_changed != 0 || mouse_moved || mouse_wheeled) {
                break;
            }
            let mut c: c_uint = e.Text.Char;
            io.InputQueueCharacters.push(if c <= IM_UNICODE_CODEPOINT_MAX { c } else { IM_UNICODE_CODEPOINT_INVALID });
            if trickle_interleaved_keys_and_text {
                text_inputted = true;
            }
        } else if e.Type == ImGuiInputEventType_Focus {
            // We intentionally overwrite this and process lower, in order to give a chance
            // to multi-viewports backends to queue AddFocusEvent(false) + AddFocusEvent(true) in same frame.
            let focus_lost: bool = !e.AppFocused.Focused;
            e.IgnoredAsSame = (io.AppFocusLost == focus_lost);
            if !e.IgnoredAsSame {
                io.AppFocusLost = focus_lost;
            }
        } else {
            // IM_ASSERT(0 && "Unknown event!");
        }
    }

    // Record trail (for domain-specific applications wanting to access a precise trail)
    //if (event_n != 0) IMGUI_DEBUG_LOG_IO("Processed: %d / Remaining: %d\n", event_n, g.InputEventsQueue.Size - event_n);
    // for (let n: c_int = 0; n < event_n; n++)
    for n in 0..event_n {
        g.InputEventsTrail.push(g.InputEventsQueue[n].clone());
    }

    // [DEBUG]
// #ifndef IMGUI_DISABLE_DEBUG_TOOLS
    if event_n != 0 && (g.DebugLogFlags.clone() & ImGuiDebugLogFlags_EventIO) != 0 {
        // for (let n: c_int = 0; n < g.InputEventsQueue.Size; n+ +)
        for n in 0..g.InputEventsQueue.len() {
            // DebugPrintInputEvent(n < event_n?(g.InputEventsQueue[n].IgnoredAsSame? "Processed (Same)": "Processed"): "Remaining", &g.InputEventsQueue[n]);
        }
    }
// #endif

    // Remaining events will be processed on the next frame
    if event_n == g.InputEventsQueue.Size {
        // g.InputEventsQueue.resize(0);
    } else {
        g.InputEventsQueue.erase(g.InputEventsQueue.Data, g.InputEventsQueue.Data + event_n);
    }

    // Clear buttons state when focus is lost
    // (this is useful so e.g. releasing Alt after focus loss on Alt-Tab doesn't trigger the Alt menu toggle)
    if g.IO.AppFocusLost {
        g.IO.ClearInputKeys();
        g.IO.AppFocusLost = false;
    }
}
