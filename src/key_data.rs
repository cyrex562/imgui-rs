#![allow(non_snake_case)]

use libc::c_float;

// [Internal] Storage used by IsKeyDown(), IsKeyPressed() etc functions.
// If prior to 1.87 you used io.KeysDownDuration[] (which was marked as internal), you should use GetKeyData(key).DownDuration and not io.KeysData[key].DownDuration.
#[derive(Default, Debug, Clone)]
pub struct ImGuiKeyData {
    pub Down: bool,
    // True for if key is down
    pub DownDuration: c_float,
    // Duration the key has been down (<0f32: not pressed, 0f32: just pressed, >0f32: time held)
    pub DownDurationPrev: c_float,
    // Last frame duration the key has been down
    pub AnalogValue: c_float,        // 0f32..1f32 for gamepad values
}
